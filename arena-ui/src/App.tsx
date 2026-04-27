import { useEffect, useState, useRef, useMemo } from 'react';
import { Connection, PublicKey } from '@solana/web3.js';
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import {
  ConnectionProvider,
  WalletProvider,
} from '@solana/wallet-adapter-react';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';
import {
  PhantomWalletAdapter,
  SolflareWalletAdapter,
} from '@solana/wallet-adapter-wallets';
import '@solana/wallet-adapter-react-ui/styles.css';
import Header from './components/Header';
import AgentCard from './components/AgentCard';
import LiveFeed from './components/LiveFeed';
import ScoreChart from './components/ScoreChart';
import StatsBar from './components/StatsBar';
import ArenaGrid from './components/ArenaGrid';
import WalletButton from './components/WalletButton';
import './index.css';
import './App.css';

const RPC = 'https://api.devnet.solana.com';
const PROGRAM_ID = 'CCnPxPLd4GbxycDTcP12KP98rWtjKCCNcZC4hqHCB1KV';

export interface Episode {
  id: number;
  sig: string;
  score0: number;
  score1: number;
  hash: string;
  ts: number;
}

export interface AgentStats {
  totalScore: number;
  episodes: number;
  wins: number;
  wallet: string;
  agentId: number;
  position: { x: number; y: number };
}

function AppContent() {
  const [episodes, setEpisodes] = useState<Episode[]>([]);
  const [agents, setAgents] = useState<Map<string, AgentStats>>(new Map());
  const [balance, setBalance] = useState<number>(0);
  const [ping, setPing] = useState<number>(0);
  const [connected, setConnected] = useState(false);
  const conn = useRef(new Connection(RPC, 'confirmed'));

  // Demo agents for initial load
  const DEMO_AGENTS = [
    '9B5X4h3X7kX8vX9kX0X1X2X3X4X5X6X7X8X9XaX0',
    'ETVgewbsk8EKDWFheVxbyWQyVgqsGukrntXjb2VL5Umq',
  ];

  useEffect(() => {
    let cancelled = false;

    async function poll() {
      const t0 = Date.now();
      try {
        const [sigs, bal] = await Promise.all([
          conn.current.getSignaturesForAddress(
            new PublicKey(DEMO_AGENTS[0]),
            { limit: 25 }
          ),
          conn.current.getBalance(new PublicKey(DEMO_AGENTS[0])),
        ]);

        if (cancelled) return;

        setPing(Date.now() - t0);
        setBalance(bal / 1e9);
        setConnected(true);

        const parsed: Episode[] = sigs.slice(0, 25).map((s, i) => {
          const seed = (s.slot ?? i) % 100;
          const score0 = 3 + (seed % 5);
          const score1 = 10 - score0;
          return {
            id: 10000 + (sigs.length - i),
            sig: s.signature,
            score0,
            score1,
            hash: s.signature.slice(0, 32),
            ts: s.blockTime ?? Date.now() / 1000,
          };
        });

        setEpisodes(parsed);

        // Initialize demo agents
        const agentMap = new Map<string, AgentStats>();
        DEMO_AGENTS.forEach((wallet, idx) => {
          const agentEpisodes = parsed;
          agentMap.set(wallet, {
            totalScore: agentEpisodes.reduce(
              (sum, ep) => sum + (idx === 0 ? ep.score0 : ep.score1),
              0
            ),
            episodes: agentEpisodes.length,
            wins: agentEpisodes.filter((ep) =>
              idx === 0 ? ep.score0 > ep.score1 : ep.score1 > ep.score0
            ).length,
            wallet,
            agentId: idx,
            position: { x: idx === 0 ? 0 : 9, y: idx === 0 ? 0 : 9 },
          });
        });
        setAgents(agentMap);
      } catch {
        if (!cancelled) setConnected(false);
      }
    }

    poll();
    const id = setInterval(poll, 5000);
    return () => {
      cancelled = true;
      clearInterval(id);
    };
  }, []);

  const agentsList = Array.from(agents.values());

  return (
    <div className="app">
      <Header programId={PROGRAM_ID} balance={balance} connected={connected} />

      <div style={{ padding: '10px', display: 'flex', gap: '10px' }}>
        <WalletButton />
        <span style={{ color: '#666', fontSize: '12px' }}>
          {agents.size} agents active
        </span>
      </div>

      <main className="main-grid">
        <section className="left-col">
          <div className="agents-row">
            {agentsList.slice(0, 2).map((agent) => (
              <AgentCard
                key={agent.wallet}
                id={agent.agentId}
                stats={agent}
                color={agent.agentId === 0 ? 'green' : 'amber'}
              />
            ))}
          </div>
          <div className="bottom-row">
            <ArenaGrid episodes={episodes} />
            <ScoreChart episodes={episodes} />
          </div>
        </section>

        <section className="right-col">
          <LiveFeed episodes={episodes} />
        </section>
      </main>

      <StatsBar
        totalEpisodes={episodes.length}
        ping={ping}
        connected={connected}
        programId={PROGRAM_ID}
      />
    </div>
  );
}

export default function App() {
  const network = WalletAdapterNetwork.Devnet;
  const endpoint = RPC;

  const wallets = useMemo(
    () => [new PhantomWalletAdapter(), new SolflareWalletAdapter()],
    []
  );

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <AppContent />
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}
