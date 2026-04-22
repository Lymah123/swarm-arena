import { useEffect, useState, useRef } from 'react';
import { Connection, PublicKey } from '@solana/web3.js';
import Header from './components/Header';
import AgentCard from './components/AgentCard';
import LiveFeed from './components/LiveFeed';
import ScoreChart from './components/ScoreChart';
import StatsBar from './components/StatsBar';
import ArenaGrid from './components/ArenaGrid';
import './index.css';
import './App.css';

const RPC = 'https://api.devnet.solana.com';
const PROGRAM_ID = 'CCnPxPLd4GbxycDTcP12KP98rWtjKCCNcZC4hqHCB1KV';
const WALLET = 'ETVgewbsk8EKDWFheVxbyWQyVgqsGukrntXjb2VL5Umq';

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
}

export default function App() {
  const [episodes, setEpisodes]   = useState<Episode[]>([]);
  const [agent0, setAgent0]       = useState<AgentStats>({ totalScore: 0, episodes: 0, wins: 0 });
  const [agent1, setAgent1]       = useState<AgentStats>({ totalScore: 0, episodes: 0, wins: 0 });
  const [balance, setBalance]     = useState<number>(0);
  const [ping, setPing]           = useState<number>(0);
  const [connected, setConnected] = useState(false);
  const conn = useRef(new Connection(RPC, 'confirmed'));

  useEffect(() => {
    let cancelled = false;

    async function poll() {
      const t0 = Date.now();
      try {
        const [sigs, bal] = await Promise.all([
          conn.current.getSignaturesForAddress(new PublicKey(WALLET), { limit: 25 }),
          conn.current.getBalance(new PublicKey(WALLET)),
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

        setAgent0(parsed.reduce((acc, ep) => ({
          totalScore: acc.totalScore + ep.score0,
          episodes:   acc.episodes + 1,
          wins:       acc.wins + (ep.score0 > ep.score1 ? 1 : 0),
        }), { totalScore: 0, episodes: 0, wins: 0 }));

        setAgent1(parsed.reduce((acc, ep) => ({
          totalScore: acc.totalScore + ep.score1,
          episodes:   acc.episodes + 1,
          wins:       acc.wins + (ep.score1 > ep.score0 ? 1 : 0),
        }), { totalScore: 0, episodes: 0, wins: 0 }));

      } catch {
        if (!cancelled) setConnected(false);
      }
    }

    poll();
    const id = setInterval(poll, 5000);
    return () => { cancelled = true; clearInterval(id); };
  }, []);

  return (
    <div className="app">
      <Header programId={PROGRAM_ID} balance={balance} connected={connected} />

      <main className="main-grid">
        <section className="left-col">
          <div className="agents-row">
            <AgentCard id={0} stats={agent0} color="green" />
            <AgentCard id={1} stats={agent1} color="amber" />
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
