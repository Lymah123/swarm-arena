import { AgentStats } from '../App';

interface Props {
  id: number;
  stats: AgentStats;
  color: 'green' | 'amber';
}

export default function AgentCard({ id, stats, color }: Props) {
  const c = color === 'green' ? 'var(--green)' : 'var(--amber)';
  const winRate = stats.episodes > 0
    ? ((stats.wins / stats.episodes) * 100).toFixed(1)
    : '0.0';

  return (
    <div style={{
      padding: '20px 24px',
      background: 'var(--surface)',
      borderTop: `2px solid ${c}`,
      position: 'relative',
      overflow: 'hidden',
    }}>
      <div style={{
        position: 'absolute', top: 0, right: 0,
        width: 80, height: 80,
        background: c,
        opacity: 0.03,
        borderRadius: '0 0 0 80px',
      }} />

      <div style={{
        fontFamily: 'var(--display)',
        fontSize: 10, fontWeight: 700,
        letterSpacing: '0.2em',
        color: 'var(--text-dim)',
        marginBottom: 8,
      }}>AGENT {id}</div>

      <div style={{
        fontFamily: 'var(--display)',
        fontSize: 42, fontWeight: 800,
        color: c, lineHeight: 1,
        marginBottom: 16,
      }}>
        {stats.totalScore}
        <span style={{ fontSize: 14, fontWeight: 400, color: 'var(--text-dim)', marginLeft: 6 }}>
          pts
        </span>
      </div>

      <div style={{ display: 'flex', gap: 24 }}>
        <Stat label="EPISODES" value={stats.episodes} />
        <Stat label="WINS" value={stats.wins} />
        <Stat label="WIN RATE" value={`${winRate}%`} />
      </div>
    </div>
  );
}

function Stat({ label, value }: { label: string; value: string | number }) {
  return (
    <div>
      <div style={{ fontSize: 9, color: 'var(--text-dim)', letterSpacing: '0.15em', marginBottom: 2 }}>
        {label}
      </div>
      <div style={{ fontSize: 16, color: 'var(--text)' }}>{value}</div>
    </div>
  );
}
