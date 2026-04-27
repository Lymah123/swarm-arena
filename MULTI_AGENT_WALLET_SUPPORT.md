# Multi-Agent Support for External Wallets

This document describes the implementation of multi-agent support that allows multiple users to connect their external Solana wallets and control agents in the Swarm Arena.

## Overview

**Traction — Yes, other users!** 

The system now supports unlimited agents, each owned by a different wallet. Users can:
- Connect their Solana wallet (Phantom, Solflare, etc.)
- Automatically spawn an agent in the arena
- Track their agent's performance
- Compete with other players' agents

## Architecture

### 1. Backend (Rust/Bevy)

#### WalletRegistry Resource
Maps wallets to agents and tracks agent count:

```rust
pub struct WalletRegistry {
    pub wallet_to_agent: HashMap<String, u8>,
    pub agent_to_wallet: HashMap<u8, String>,
    pub next_agent_id: u8,
}
```

Methods:
- `register_agent(wallet: String) -> u8` - Creates or retrieves agent for wallet
- `get_agent_for_wallet(wallet: &str) -> Option<u8>` - Lookup agent by wallet
- `get_wallet_for_agent(agent_id: u8) -> Option<&str>` - Lookup wallet by agent
- `agent_count() -> u8` - Total agents in arena

#### Wallet Component
Each agent entity now includes wallet ownership:

```rust
#[derive(Component, Debug, Clone)]
pub struct Wallet(pub String);
```

#### WalletConnections System
Handles dynamic registration of new wallets:

```rust
pub struct WalletConnections {
    pub pending_wallets: Vec<String>,
    pub processed_wallets: HashSet<String>,
}
```

Key system: `process_wallet_registrations()`
- Processes pending wallet requests
- Spawns agent entities
- Emits `AgentRegistered` events
- Registers agents on-chain

#### AgentRegistered Event
Notifies when a new agent joins:

```rust
#[derive(Event, Debug)]
pub struct AgentRegistered {
    pub agent_id: u8,
    pub wallet: String,
}
```

### 2. On-Chain Program (Solana/Anchor)

#### WalletRegistry Account
Stores wallet-to-agent mapping on Solana:

```rust
#[account]
pub struct WalletRegistry {
    pub wallet: Pubkey,
    pub agent_id: u8,
    pub registered_at: i64,
    pub bump: u8,
}
```

#### register_wallet Instruction
Links a wallet to an agent:

```rust
pub fn register_wallet(
    ctx: Context<RegisterWallet>,
    agent_id: u8,
) -> Result<()>
```

This creates a PDA: `seeds: [b"wallet", signer.key().as_ref()]`

### 3. Frontend (React/TypeScript)

#### Wallet Adapter Integration
Uses Solana Wallet Adapter ecosystem:

Dependencies added:
- `@solana/wallet-adapter-react` - React hooks and components
- `@solana/wallet-adapter-react-ui` - UI components
- `@solana/wallet-adapter-wallets` - Wallet implementations

#### AppContent Component
- Wrapped with `ConnectionProvider`, `WalletProvider`, `WalletModalProvider`
- Dynamic agent tracking using `Map<string, AgentStats>`
- Displays agent count and connected wallet

#### WalletButton Component
Located: `arena-ui/src/components/WalletButton.tsx`

- Uses `WalletMultiButton` for easy connection
- Shows connected wallet address
- Supports Phantom, Solflare, and other standard wallets

## Usage Flow

### For Users

1. **Connect Wallet**
   - Click "Select Wallet" button
   - Choose Phantom, Solflare, or other wallet
   - Sign connection prompt

2. **Agent Spawns**
   - New agent appears in arena
   - Agent gets unique ID and starting position
   - Wallet address associated with agent

3. **Track Performance**
   - See agent stats (score, episodes, wins)
   - View position on grid
   - Watch in live feed

4. **Compete**
   - Agent competes with other players' agents
   - Earn scores for collecting resources
   - Track statistics on-chain

### For Developers

#### Registering a New Wallet

```rust
// In backend system
let mut wallet_connections = wallet_connections.res;
wallet_connections.request_agent("wallet_address".to_string());
```

#### Querying Agent by Wallet

```rust
let registry = registry.res;
if let Some(agent_id) = registry.get_agent_for_wallet("wallet_address") {
    // Found agent for wallet
}
```

#### Querying All Agents

```rust
let all_agents = wallet_manager::get_all_agents(&registry, &query);
// Returns: Vec<(u8, String, i32, i32, f32)>
// (agent_id, wallet, x, y, score)
```

## Data Flow Diagram

```
┌─────────────────────┐
│  External Wallet    │
│   (Phantom, etc)    │
└──────────┬──────────┘
           │
           │ User clicks Connect
           ▼
┌─────────────────────┐
│  WalletButton       │
│  React Component    │
└──────────┬──────────┘
           │
           │ Wallet connected
           ▼
┌─────────────────────┐
│ WalletConnections   │
│ pending_wallets     │
└──────────┬──────────┘
           │
           │ process_wallet_registrations()
           ▼
┌─────────────────────┐
│  WalletRegistry     │
│  register_agent()   │
└──────────┬──────────┘
           │
           │ New agent_id assigned
           ▼
┌─────────────────────┐
│  Spawn Agent Entity │
│ + Wallet Component  │
└──────────┬──────────┘
           │
           │ AgentRegistered event
           ▼
┌─────────────────────┐
│  On-Chain Register  │
│ register_wallet()   │
└──────────┬──────────┘
           │
           │ UI updates
           ▼
┌─────────────────────┐
│ UI Shows New Agent  │
│ in Agents List      │
└─────────────────────┘
```

## Configuration

### Wallet Adapter Setup
Located in: `arena-ui/src/App.tsx`

Supported wallets:
- Phantom Wallet
- Solflare Wallet
- (Extensible to add more)

```typescript
const wallets = useMemo(
  () => [new PhantomWalletAdapter(), new SolflareWalletAdapter()],
  []
);
```

### RPC Configuration
- **Devnet**: `https://api.devnet.solana.com`
- Program ID: `CCnPxPLd4GbxycDTcP12KP98rWtjKCCNcZC4hqHCB1KV`

## Limits

- **Max Agents**: 255 (u8 limit)
- **Max Wallet Name Length**: 32 characters (on-chain)
- **Grid Size**: 10x10 (configurable)

## Future Enhancements

1. **API Endpoint**: `/register-agent` for programmatic wallet registration
2. **Agent Customization**: Names, colors, policies per wallet
3. **Leaderboard**: Global rankings by wallet
4. **Rewards**: Direct SOL payouts to winners
5. **Agent Policies**: User-defined behavior strategies
6. **Multi-Season Support**: Tournaments and seasons
7. **Token Integration**: SPL token rewards

## Testing

### Manual Testing

```bash
# Build backend
cargo build

# Build frontend
cd arena-ui
npm install
npm start
```

### Test Wallet Registration

```rust
// In a test
let mut registry = WalletRegistry::default();
let wallet1 = "wallet1".to_string();
let agent_id = registry.register_agent(wallet1.clone());
assert_eq!(registry.get_agent_for_wallet(&wallet1), Some(agent_id));
```

## Security Considerations

1. **PDA Derivation**: Wallets linked via PDA with wallet as seed
2. **Signer Verification**: Only wallet owner can register their agent
3. **Unique Mapping**: One wallet = one agent (until multi-agent per wallet)
4. **On-Chain Verification**: Registration immutable on blockchain

## Troubleshooting

### Wallet Not Connecting
- Ensure wallet extension is installed
- Check if on Devnet network
- Try refreshing page

### Agent Not Spawning
- Check if wallet is connected
- Verify backend is running
- Check WalletConnections system is active

### On-Chain Registration Failed
- Insufficient SOL for transaction fees
- Check RPC connectivity
- Verify program ID matches

## References

- [Solana Wallet Adapter](https://github.com/solana-labs/wallet-adapter)
- [Bevy ECS](https://bevyengine.org/)
- [Anchor Framework](https://www.anchor-lang.com/)
