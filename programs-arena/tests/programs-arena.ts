import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("swarm-arena", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ProgramsArena as any;
  const signer = provider.wallet;

  let agentPda: PublicKey;
  let reputationPda: PublicKey;
  let episodePda: PublicKey;

  before(async () => {
    [agentPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("agent"), signer.publicKey.toBuffer()],
      program.programId
    );
    [reputationPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("reputation"), signer.publicKey.toBuffer()],
      program.programId
    );
    [episodePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("episode"), Buffer.from(new anchor.BN(0).toArray("le", 8))],
      program.programId
    );
  });

  it("registers an agent on-chain", async () => {
    const tx = await program.methods
      .createAgent("test-agent")
      .accounts({
        agentIdentity: agentPda,
        signer: signer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("create_agent tx:", tx);

    const agent = await program.account.agentIdentity.fetch(agentPda);
    assert.equal(agent.name, "test-agent");
    assert.ok(agent.owner.equals(signer.publicKey));
    assert.ok(agent.registeredAt.toNumber() > 0);
    console.log("Agent registered:", agent.name, "at", new Date(agent.registeredAt.toNumber() * 1000).toISOString());
  });

  it("logs an episode and updates reputation", async () => {
    const episodeId = new anchor.BN(0);
    const scores = [new anchor.BN(7), new anchor.BN(3)];
    const episodeHash = Array(32).fill(1);

    const tx = await program.methods
      .logEpisode(episodeId, scores, episodeHash)
      .accounts({
        episodeLog: episodePda,
        agentReputation: reputationPda,
        signer: signer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("log_episode tx:", tx);

    const log = await program.account.episodeLog.fetch(episodePda);
    assert.equal(log.episodeId.toNumber(), 0);
    assert.equal(log.scores[0].toNumber(), 7);
    assert.equal(log.scores[1].toNumber(), 3);

    const rep = await program.account.agentReputation.fetch(reputationPda);
    assert.equal(rep.totalScore.toNumber(), 7);
    assert.equal(rep.episodesPlayed, 1);

    console.log("Episode logged — scores:", log.scores[0].toNumber(), "/", log.scores[1].toNumber());
    console.log("Reputation — total score:", rep.totalScore.toNumber(), "episodes:", rep.episodesPlayed);
  });

  it("accumulates reputation across multiple episodes", async () => {
    const [episode1Pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("episode"), Buffer.from(new anchor.BN(1).toArray("le", 8))],
      program.programId
    );

    await program.methods
      .logEpisode(new anchor.BN(1), [new anchor.BN(5), new anchor.BN(5)], Array(32).fill(2))
      .accounts({
        episodeLog: episode1Pda,
        agentReputation: reputationPda,
        signer: signer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const rep = await program.account.agentReputation.fetch(reputationPda);
    assert.equal(rep.totalScore.toNumber(), 12); // 7 + 5
    assert.equal(rep.episodesPlayed, 2);
    console.log("After 2 episodes — total score:", rep.totalScore.toNumber());
  });
});
