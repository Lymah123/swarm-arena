# swarm-arena

A permissionless, on-chain agent training arena built in Rust.

Agents compete in a Bevy-powered grid environment. At the end of each episode, results are committed to Solana, making agent training verifiable, tamper-proof, and composable with on-chain reputation and payment primitives.

Built for the [Agentic SWARM Hackathon](https://swarm.thecanteenapp.com/) by Canteen × Colosseum.

____________________________________________________

## Live Demo

**Dashboard:** https://arena-ui-pi.vercel.app/

**Article:** [Read on Dev.to](https://dev.to/lymah/i-built-a-permissionless-on-chain-agent-training-arena-on-solana-in-3-weeks-2on2)

**First tx:** https://explorer.solana.com/tx/38yieCpWNbex4RDEzXw8pEREHYQNswyW9hYBHXZmigLP9FEmp8FSpDAwPNvU3dcZuY5RrUdWRp6EJcjYJUcEoL21?cluster=devnet

----------------------------------------------------------

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
  └── episode ends → scores + state hashed (SHA256)

        ↓ commit on-chain

Solana program (Anchor)
  └── EpisodeLog PDA      — immutable episode record
  └── AgentReputation PDA — cumulative score per agent
  └── RewardVault PDA     — holds SOL, releases on finalization
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

First devnet transaction — episode 10000 committed and confirmed:
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

-------------------------------------------

## The Agents Actually Learn

Agent 0 uses Q-learning. Agent 1 uses a heuristic (Manhattan distance to nearest resource). The learning curve:

| Episodes | Agent 0 avg reward |
|----------|-------------------|
| 1-10     | 0.10              |
| 11-20    | 0.20              |
| 21-30    | 2.10              |
| 31-40    | 5.10              |
| 41-50    | 6.50+             |

By episode 42, Agent 0 collected 8/10 resources; beating the heuristic agent. Every step of this learning curve is committed to devnet. The blockchain is the training log.
Resource positions are randomized each episode to prevent memorization and force genuine generalization.

## Resources

**Dashboard:** https://arena-ui-pi.vercel.app/
**Demo Video:** [Watch on Loom](https://www.loom.com/share/b948ec7ef0ee4f0aa467d8b9b7699335)

## Build Status

- [x] Bevy grid environment
- [x] Episode loop (tick → score → end)
- [x] Solana program (Anchor) — 4/4 integration tests passing
- [x] On-chain episode commit
- [x] Agent reputation PDA
- [x] RewardVault PDA — permissionless SOL payouts
- [x] Devnet deployment — 100+ episodes committed
- [x] Q-learning agent — avg reward 0.10 → 6.50+ over 50 episodes
- [x] Live dashboard — arena-ui-pi.vercel.app
- [x] Wallet connect — Phantom + Solflare

## Weekly status

- **Week 1 (Apr 6–12)**: Environment setup — Rust, Bevy, Anchor. First dummy episode hash committed to local Solana validator.
- **Week 2 (Apr 13–19)**: Bevy arena built. Two-agent grid world, ECS episode loop, reward signals. Q-learning added to Agent 0.
- **Week 3 (Apr 20–26)**: On-chain integration. Episode results → Solana devnet. Agent reputation PDA live. First confirmed devnet transaction.
- **Week 4 (Apr 27–May 11)**: Live dashboard deployed, Phantom + Solflare wallet connect, article published, demo video recorded. Submitted.

------------------------------------

## Running Locally

```bash
# Clone
git clone https://github.com/Lymah123/swarm-arena.git
cd swarm-arena

# Run the Bevy arena
cargo run
```

Episodes auto-commit to **Solana devnet** every 200 ticks. View results live on the dashboard: https://arena-ui-pi.vercel.app/

**Requirements:**
- Rust toolchain
- Solana CLI (optional, for verifying transactions)
- `~/.config/solana/id.json` keypair with devnet SOL (for on-chain commits)

## Organizer Feedback

> "Perhaps if you scale up the world size, you could apply it to Minecraft maps? Would be an awesome demo, and there are Minecraft map makers out there that might love to give it a go." — aadi, Canteen hackathon organizer

## Roadmap

- **Larger world state** — expand from 10×10 to arbitrary grid sizes
- **Minecraft map integration** — map makers deploy world configs as PDAs, agents train permissionlessly across maps
- **Multi-operator support** — external training loops calling the same Anchor program with different keypairs
- **Mainnet deployment** — real SOL rewards for high-scoring agents
- **Reputation composability** — other Solana programs read `AgentReputation` PDAs to gate access or rank agents
- **Neural network policies** — replace tabular Q-learning with a small MLP, model hash stored on-chain

---

```python3 -c "
import os
path = os.path.expanduser('~/swarm-arena/README.md')
with open(path) as f:
    content = f.read()

section = '''
```

## Run Your Own Agent

Anyone can register an agent and submit episodes to the same deployed program. No permission needed.

### Prerequisites
- Rust installed
- A Solana keypair with devnet SOL

```
\`\`\`bash
# Get devnet SOL if needed
solana airdrop 2 --url devnet

# Clone the repo
git clone https://github.com/Lymah123/swarm-arena.git
cd swarm-arena
\`\`\`

### Step 1 — Register your agent

\`\`\`bash
SWARM_KEYPAIR=~/.config/solana/id.json \\\\
SWARM_AGENT_NAME=your-agent-name \\\\
cargo run --bin register-agent
\`\`\`

You will see:
\`\`\`
Agent your-agent-name registered!
  Wallet:    <your-pubkey>
  Agent PDA: <pda-address>
  Explorer:  https://explorer.solana.com/tx/...?cluster=devnet
\`\`\`

### Step 2 — Run the arena

\`\`\`bash
SWARM_KEYPAIR=~/.config/solana/id.json cargo run --bin swarm-arena
\`\`\`

```
Your agent will start training and committing episodes to the same Solana program.
Reputation accumulates in your AgentReputation PDA — permanently, across sessions.

### Environment variables

| Variable | Default | Description |
|---|---|---|
| SWARM_KEYPAIR | ~/.config/solana/id.json | Path to your Solana keypair |
| SWARM_AGENT_NAME | my-agent | Name for your agent on-chain |

Your AgentReputation PDA is derived from your keypair — only you can train under your identity.

# Insert before the Roadmap section
```content = content.replace('## Roadmap', section + '## Roadmap')
with open(path, 'w') as f:
    f.write(content)
print('README updated')
```

## Author

Built by [@Lymah123](https://github.com/Lymah123) — systems engineer focused on high-performance Rust backends and agent infrastructure.

Find me in the [Canteen Discord](https://discord.gg/canteen) if you want to run your own agent against the program.