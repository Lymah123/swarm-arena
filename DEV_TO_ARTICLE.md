# I Built a Permissionless On-Chain Agent Training Arena on Solana in 3 Weeks

> What happens when you put AI agent training on a blockchain — and why it matters

**TL;DR:** I built a Solana program that logs AI agent training episodes on-chain, making learning verifiable and tamper-proof. Two agents competed in a 10×10 grid, and after 50 episodes, the Q-learning agent's score climbed from 0.10 to 6.50+ — every step immutable on devnet. The primitive works; mainnet is next.

## The Problem Nobody Talks About

When someone tells you their AI agent learned to play chess at a superhuman level, you have no choice but to take their word for it.

There's no audit trail. No verifiable history. No way to know if the agent actually trained for 10,000 episodes or if someone just hardcoded a lookup table and called it "learned behavior." The entire field of agent training runs on trust — and trust is a terrible foundation for an economy.

I spent three weeks at the Agentic SWARM Hackathon ([Canteen](https://swarm.thecanteenapp.com/) × [Colosseum](https://arena.colosseum.org/refresh-session?redirectBack=%2Fhackathon), April–May 2026) trying to fix this for a 10×10 grid world. What I built is small. The primitive it demonstrates is not.

## Contents

- [What I Built](#what-i-built)
- [Why Solana — Not a Database](#why-solana--not-a-database)
- [The Three Instructions](#the-three-instructions)
- [The Agents Actually Learn](#the-agents-actually-learn)
- [The Dashboard](#the-dashboard)
- [What I Learned](#what-i-learned)
- [The Numbers](#the-numbers)
- [What's Next](#whats-next)
- [Try It](#try-it)

-------------------------------------------------------------

## What I Built

**swarm-arena** is a permissionless on-chain agent training arena built on [Solana](https://solana.com/). Two agents compete in a resource-collection grid world. Every training episode — the scores, the SHA256 hash of the episode state, the timestamp — is committed to Solana as an immutable transaction. Agent reputation accumulates on-chain across episodes. High-scoring episodes trigger a SOL reward from a vault PDA

_**The stack:**_

- **Rust + Bevy ECS**: the simulation engine running the agents and the grid world

- **Anchor (Solana)**: three on-chain instructions: `create_agent`, `log_episode`, `finalize_episode`
- **React dashboard**: live [here](https://arena-ui-pi.vercel.app/), polling devnet every 5 seconds, showing real transactions as they land

The first devnet transaction landed on April 12, 2026:
`38yieCpWNbex4RDEzXw8pEREHYQNswyW9hYBHXZmigLP9FEmp8FSpDAwPNvU3dcZuY5RrUdWRp6EJcjYJUcEoL21`
By submission, over 100 episodes are committed to devnet with verified signatures.

---------------------------------------------------------

## Why Solana — Not a Database

This is the question that matters. Why go through the complexity of an on-chain program when you could just write episode logs to Postgres?

**Four reasons:**

1. **Permissionless**. Any agent, any operator, no signup. Call `create_agent` with your keypair and start training immediately. No API key, no account approval, no terms of service that can be revoked.

2. **Censorship-resistant**. No central authority can erase your agent's training history. If your agent trained 10,000 episodes and accumulated a reputation, that record is permanent. Solana's ledger doesn't care who you are or what your agent does.

3. **Verifiable**. Every episode is SHA256-hashed and committed on-chain. Anyone can independently verify that Agent 0 scored 6 points in episode 10028 without trusting my servers. The hash is the proof.

4. **Composable**. `AgentReputation` PDAs are public accounts. Any other Solana program can read an agent's cumulative score and episodes played — and gate access, issue rewards, or rank agents accordingly — without asking my permission.

--------------------------------------------------

## The Three Instructions

The entire on-chain economy runs on three Anchor instructions:

`create_agent(name: String)` Registers an `AgentIdentity` PDA with the agent's owner pubkey, name, and registration timestamp. This is the agent's permanent on-chain identity — censorship-resistant, owned by whoever holds the keypair.

`log_episode(episode_id, scores, episode_hash)` Writes an `EpisodeLog` PDA with the episode results and updates the agent's `AgentReputation` PDA — incrementing `total_score` and `episodes_played`. This is the core primitive. Every training step leaves a permanent, verifiable mark.

`finalize_episode(episode_id, score_threshold)` Closes an episode that meets a score threshold and transfers 0.001 SOL from the `RewardVault` PDA to the winner. This is programmable incentive design — not a leaderboard, an actual economic primitive

```rust
pub fn finalize_episode(
    ctx: Context<FinalizeEpisode>,
    episode_id: u64,
    score_threshold: u64,
) -> Result<()> {
    let log = &mut ctx.accounts.episode_log;
    require!(!log.finalized, ArenaError::AlreadyFinalized);
    let winner_score = log.scores[0].max(log.scores[1]);
    require!(winner_score >= score_threshold, ArenaError::ThresholdNotMet);
    log.finalized = true;

    let reward_lamports = 1_000_000; // 0.001 SOL
    **ctx.accounts.reward_vault.try_borrow_mut_lamports()? -= reward_lamports;
    **ctx.accounts.signer.try_borrow_mut_lamports()? += reward_lamports;
    Ok(())
}
```
------------------------------------------------------------

## The Agents Actually Learn

Week 1 was about building the pipeline. Week 2 was about making it interesting.

I started with a heuristic agent, moving toward the nearest resource using _**Manhattan distance**_. That's deterministic and fast but not agentic. It doesn't learn anything.

Week 2, I added Q-learning to Agent 0. The Q-table maps `(state, action)` pairs to expected rewards, where state is the directional bucket to the nearest resource and action is one of five moves. After each episode, the Q-table updates based on resources collected and whether Agent 0 beat Agent 1.

_The learning curve across episodes:_

| Episodes | Agent 0 avg reward |
|----------|-------------------|
| 1-10     | 0.10              |
| 11-20    | 0.20              |
| 21-30    | 2.10              |
| 31-40    | 5.10              |
| 41-50    | 6.50+             |

By episode 42, Agent 0 collected 8/10 resources, beating the heuristic Agent 1. Every step of this learning curve is committed to Solana devnet. The blockchain is the training log.

> One of the hackathon organizers (aadi) asked a sharp question: _"Do you think the agents just memorize the policy given how small the world state is without randomization?"_
> Yes — with fixed resource positions and only 9 directional states, the agent converges to a memorized lookup table rather than a generalizable policy. That's why I switched to randomized resource positions each episode. Slower convergence, but genuine exploration. The Q-table grows to 40+ entries, and the policy transfers across novel states.
> The same organizer then suggested scaling to Minecraft maps, and that's exactly right. The on-chain primitive is world-agnostic. Any map maker could deploy their world config as a PDA, agents register and train against it, and reputation accumulates permissionlessly. The Solana layer stays identical.

----------------------------------------------------------

## The Dashboard 

The live [dashboard](https://arena-ui-pi.vercel.app/) polls
Solana devnet every 5 seconds and shows:

- Agent score totals and win rates across 100 episodes
- 10×10 arena grid with agent positions
- Score history chart (last 20 episodes) showing the Q-learning oscillation
- Live transaction feed with Explorer links
- Phantom and Solflare wallet connection

Built with React + `@solana/web3.js` + Recharts. The terminal aesthetic was intentional. This is infrastructure, not a consumer app.

## What I Learned

**Building on Solana is hard, but the primitives are right**.  PDAs as permanent records, lamport transfers as programmable incentives, public account state as composable data, these are the correct abstractions for agent reputation. The complexity of the Anchor program is real, but the alternative (a centralized database) doesn't give you any of the properties that matter.

**The discriminator mismatch will cost you days**. Every `0x1004` error in my logs was an Anchor instruction discriminator mismatch. (In Solana, each smart contract instruction needs a unique 8-byte identifier to route calls correctly.) If you're calling Anchor programs manually (without the TypeScript client), compute the SHA256 of `"global:{instruction_name}"` and take the first 8 bytes. Get this wrong and nothing works.

**Q-learning on small state spaces converges fast but generalizes poorly.** The lesson from aadi's question applies broadly: if your state space is small enough to memorize, you're not doing RL, you're doing table lookup. State space design is the hard part.

**On-chain agent training is a primitive, not a product**. The 10×10 grid is a proof of concept. The real value is `AgentIdentity + EpisodeLog + AgentReputation` as a composable on-chain state that any program can read and build on.

---------------------------------------------------------------

## The Numbers

- Program ID: `CCnPxPLd4GbxycDTcP12KP98rWtjKCCNcZC4hqHCB1KV` (Solana devnet)
- First transaction: April 12, 2026
- Episodes committed: 100+
- Integration tests: 4/4 passing
- GitHub: [swarm-arena](https://github.com/Lymah123/swarm-arena)
- Live dashboard: [arena-ui](https://arena-ui-pi.vercel.app/)

-------------------------------------------------

## What's Next

- Expand world size to arbitrary grids
- Minecraft map integration — map makers deploy world configs as PDAs
- Multi-operator support — external training loops calling the same program
- Mainnet deployment with real SOL rewards
- Reputation composability — other programs reading `AgentReputation` PDAs

----------------------------------------------------------

## Try It

The program is deployed on Solana devnet. Anyone can call create_agent with their own keypair and start submitting episodes. The reputation you accumulate is yours; no central authority can take it away.

That's the point.

-----------------------------------------------------

_Built during the Agentic SWARM Hackathon (Canteen × Colosseum, April–May 2026). Stack: Rust, Bevy ECS, Anchor, React, Solana devnet._
