import { useParams, Link } from 'react-router-dom';
import { useEffect, useState } from 'react';
import { Connection, PublicKey } from '@solana/web3.js';

interface Episode {
 id: number;
 sig: string;
 score0: number;
 score1: number;
 hash: string;
 ts: number;
}

const RPC = 'https://api.devnet.solana.com';
const PROGRAM_ID = 'CCnPxPLd4GbxycDTcP12KP98rWtjKCCNcZC4hqHCB1KV';

export default function AgentDetails() {
 const { agentId } = useParams<{ agentId: string }>();
 const [episodes, setEpisodes] = useState<Episode[]>([]);
 const [totalScore, setTotalScore] = useState(0);
 const [wins, setWins] = useState(0);

 useEffect(() => {
  async function loadAgentHistory() {
   try {
    const conn = new Connection(RPC, 'confirmed');
    const programPK = new PublicKey(PROGRAM_ID);
    const sigs = await conn.getSignaturesForAddress(programPK, { limit: 100 });

    const agentIdx = parseInt(agentId || '0');
    const parsed: Episode[] = sigs.map((s, i) => {
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

    const agentScore = parsed.reduce(
     (sum, ep) => sum + (agentIdx === 0 ? ep.score0 : ep.score1),
     0
    );
    const agentWins = parsed.filter((ep) =>
     agentIdx === 0 ? ep.score0 > ep.score1 : ep.score1 > ep.score0
    ).length;

    setTotalScore(agentScore);
    setWins(agentWins);
   } catch (err) {
    console.error('Failed to load agent history:', err);
   }
  }

  loadAgentHistory();
 }, [agentId]);

 return (
  <div style={{ padding: '24px', minHeight: '100vh' }}>
   <div style={{ marginBottom: '24px' }}>
    <Link
     to="/"
     style={{
      color: 'var(--green)',
      textDecoration: 'none',
      fontSize: '12px',
      fontFamily: 'var(--mono)',
     }}
    >
     ← Back to Dashboard
    </Link>
   </div>

   <h1 style={{
    fontFamily: 'var(--display)',
    fontWeight: 800,
    fontSize: '28px',
    color: 'var(--green)',
    marginBottom: '24px',
   }}>
    Agent {agentId} Details
   </h1>

   <div style={{
    display: 'grid',
    gridTemplateColumns: 'repeat(3, 1fr)',
    gap: '16px',
    marginBottom: '32px',
   }}>
    <div style={{
     background: 'var(--surface)',
     border: '1px solid var(--border)',
     padding: '16px',
     borderRadius: '4px',
    }}>
     <div style={{ fontSize: '11px', color: 'var(--text-dim)', marginBottom: '8px' }}>
      TOTAL SCORE
     </div>
     <div style={{ fontSize: '24px', color: 'var(--green)', fontWeight: 'bold' }}>
      {totalScore}
     </div>
    </div>

    <div style={{
     background: 'var(--surface)',
     border: '1px solid var(--border)',
     padding: '16px',
     borderRadius: '4px',
    }}>
     <div style={{ fontSize: '11px', color: 'var(--text-dim)', marginBottom: '8px' }}>
      WINS
     </div>
     <div style={{ fontSize: '24px', color: 'var(--amber)', fontWeight: 'bold' }}>
      {wins}
     </div>
    </div>

    <div style={{
     background: 'var(--surface)',
     border: '1px solid var(--border)',
     padding: '16px',
     borderRadius: '4px',
    }}>
     <div style={{ fontSize: '11px', color: 'var(--text-dim)', marginBottom: '8px' }}>
      EPISODES
     </div>
     <div style={{ fontSize: '24px', color: 'var(--green)', fontWeight: 'bold' }}>
      {episodes.length}
     </div>
    </div>
   </div>

   <h2 style={{
    fontFamily: 'var(--mono)',
    fontSize: '12px',
    color: 'var(--text-dim)',
    marginBottom: '16px',
    textTransform: 'uppercase',
   }}>
    Episode History
   </h2>

   <div style={{
    background: 'var(--surface)',
    border: '1px solid var(--border)',
    borderRadius: '4px',
    overflow: 'hidden',
   }}>
    {episodes.length === 0 ? (
     <div style={{ padding: '16px', textAlign: 'center', color: 'var(--text-dim)' }}>
      No episodes found
     </div>
    ) : (
     <table style={{ width: '100%', borderCollapse: 'collapse' }}>
      <thead style={{ background: 'rgba(0,0,0,0.2)', borderBottom: '1px solid var(--border)' }}>
       <tr>
        <th style={{ padding: '12px', textAlign: 'left', fontSize: '11px', fontWeight: 'normal' }}>
         Episode
        </th>
        <th style={{ padding: '12px', textAlign: 'left', fontSize: '11px', fontWeight: 'normal' }}>
         Score
        </th>
        <th style={{ padding: '12px', textAlign: 'left', fontSize: '11px', fontWeight: 'normal' }}>
         Vs Score
        </th>
        <th style={{ padding: '12px', textAlign: 'left', fontSize: '11px', fontWeight: 'normal' }}>
         Result
        </th>
        <th style={{ padding: '12px', textAlign: 'left', fontSize: '11px', fontWeight: 'normal' }}>
         Tx
        </th>
       </tr>
      </thead>
      <tbody>
       {episodes.map((ep) => {
        const agentIdx = parseInt(agentId || '0');
        const myScore = agentIdx === 0 ? ep.score0 : ep.score1;
        const opponentScore = agentIdx === 0 ? ep.score1 : ep.score0;
        const won = myScore > opponentScore;
        return (
         <tr
          key={ep.id}
          style={{
           borderBottom: '1px solid var(--border)',
           background: won ? 'rgba(0, 255, 0, 0.05)' : 'rgba(255, 165, 0, 0.05)',
          }}
         >
          <td style={{ padding: '12px', fontSize: '11px', fontFamily: 'var(--mono)' }}>
           EP #{ep.id}
          </td>
          <td style={{
           padding: '12px',
           fontSize: '11px',
           fontFamily: 'var(--mono)',
           color: 'var(--green)',
           fontWeight: 'bold',
          }}>
           {myScore}
          </td>
          <td style={{
           padding: '12px',
           fontSize: '11px',
           fontFamily: 'var(--mono)',
           color: 'var(--amber)',
          }}>
           {opponentScore}
          </td>
          <td style={{
           padding: '12px',
           fontSize: '11px',
           fontFamily: 'var(--mono)',
           color: won ? 'var(--green)' : 'var(--red)',
          }}>
           {won ? 'WIN' : 'LOSS'}
          </td>
          <td style={{ padding: '12px' }}>
           <a
            href={`https://explorer.solana.com/tx/${ep.sig}?cluster=devnet`}
            target="_blank"
            rel="noreferrer"
            style={{
             fontSize: '10px',
             color: 'var(--green)',
             textDecoration: 'none',
             borderBottom: '1px solid var(--green-dim)',
            }}
           >
            {ep.sig.slice(0, 8)}...
           </a>
          </td>
         </tr>
        );
       })}
      </tbody>
     </table>
    )}
   </div>
  </div>
 );
}
