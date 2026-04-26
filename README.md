# swarm-arena

A permissionless, on-chain agent training arena built in Rust.

Agents compete in a Bevy-powered grid environment. At the end of each episode, results are committed to Solana — making agent training verifiable, tamper-proof, and composable with on-chain reputation and payment primitives.

Built for the [Agentic SWARM Hackathon](https://swarm.thecanteenapp.com/) by Canteen × Colosseum.

____________________________________________________

## Live Demo

**Dashboard:** https://arena-ui-pi.vercel.app  
**Program ID:** `CCnPxPLd4GbxycDTcP12KP98rWtjKCCNcZC4hqHCB1KV` (Solana devnet)  
**First tx:** https://explorer.solana.com/tx/38yieCpWNbex4RDEzXw8pEREHYQNswyW9hYBHXZmigLP9FEmp8FSpDAwPNvU3dcZuY5RrUdWRp6EJcjYJUcEoL21?cluster=devnet

## What this is?

Most agent training happens in private, centralized environments. Results are self-reported, memory is ephemeral, and there's no way to verify what an agent actually did across sessions.

**swarm-arena** makes agent training verifiable on-chain:

- A lightweight multi-agent environment runs in Rust using Bevy's ECS architecture
- Episodes compile to WASM — portable, sandboxed execution
- At the end of each episode, the result (agent IDs, scores, episode hash) is committed to a Solana program
- Agent reputation accumulates in a PDA — a verifiable, cross-session leaderboard that no single provider controls

--------------------------------------------------------------

## Architecture

```
Bevy arena (Rust/ECS)
  └── agents tick, act, collect rewards
  └── episode ends → result serialised

        ↓ compile to WASM

Episode runner
  └── scores computed
  └── episode state hashed

        ↓ commit on-chain

Solana program (Anchor)
  └── EpisodeLog account — immutable episode record
  └── AgentReputation PDA — cumulative score per agent
```

---------------------------------------------------

## Stack

- **Rust** — systems language, all environment logic
- **Bevy** — ECS game engine, handles agent simulation
- **Trunk** — WASM compilation target
- **Anchor** — Solana program framework
- **Solana devnet** — on-chain episode logging and reputation

------------------------------------------------------------

## Live on-chain evidence

First devnet transaction — episode 35 committed and finalized:
https://explorer.solana.com/tx/38yieCpWNbex4RDEzXw8pEREHYQNswyW9hYBHXZmigLP9FEmp8FSpDAwPNvU3dcZuY5RrUdWRp6EJcjYJUcEoL21?cluster=devnet

Program ID: `CCnPxPLd4GbxycDTcP12KP98rWtjKCCNcZC4hqHCB1KV`  
Network: Solana devnet

## Why Solana — not a database

Traditional agent training logs can be deleted, falsified, or gated behind
a company's API. swarm-arena puts every training episode on Solana because:

- **Permissionless** — any agent, any operator, no signup required.
  Register an AgentIdentity PDA and start training immediately.
- **Censorship-resistant** — no central authority can erase your agent's
  training history or reputation score. The ledger is permanent.
- **Verifiable** — every episode is SHA256-hashed and committed on-chain.
  Anyone can independently verify that Agent 0 scored 6 points in episode
  10028 without trusting swarm-arena's servers.
- **Composable** — AgentReputation PDAs are public accounts. Any other
  Solana program can read an agent's reputation and gate access, issue
  rewards, or rank agents — without asking permission.
- **Economic primitive** — the RewardVault PDA holds real SOL. High-scoring
  episodes trigger permissionless payouts. This is programmable incentive
  design, not a leaderboard.

A database gives you storage. Solana gives you a shared, trustless,
programmable record of who trained what, when, and how well.

## Build Status

- [x] Bevy grid environment
- [ ] Episode loop (tick -> score -> end)
- [ ] WASM compilation via Trunk
- [ ] Solana program (Anchor)
- [ ] On-chain episode commit
- [ ] Agent reputation PDA
- [ ] Devnet deployment

## Weekly status

- **Week 1 (Apr 6–12)**: Environment setup — Rust, Bevy, Anchor, Trunk. Goal: dummy episode hash committed to local Solana validator.
- **Week 2 (Apr 13–19)**: Build the Bevy arena. Two-agent grid world, ECS episode loop, reward signals.
- **Week 3 (Apr 20–26)**: On-chain integration. Episode results → Solana devnet. Agent reputation PDA live.
- **Week 4 (Apr 27–May 11)**: Polish, demo, submission.

------------------------------------

## Running Locally

```
# Clone
git clone https://github.com/Lymah123/swarm-arena.git
cd swarm-arena

# Run the Bevy arena (native)
cargo run

# Build to WASM
trunk build

# Run Solana local validator (separate terminal)
solana-test-validator

# Deploy Anchor program
anchor build && anchor deploy
```

## Author

Built by [@Lymah123](https://github.com/Lymah123) — systems engineer focused on high-performance Rust backends and agent infrastructure.
