import { Episode } from '../App';

interface Props { episodes: Episode[] }

export default function LiveFeed({ episodes }: Props) {
  return (
    <div className="panel" style={{
      flex: 1, overflow: 'hidden',
      display: 'flex', flexDirection: 'column'
    }}>
      <div className="panel-label" style={{ display: 'flex', justifyContent: 'space-between', flexShrink: 0 }}>
        <span>LIVE COMMITS</span>
        <span className="blink" style={{ color: 'var(--green)' }}>● REC</span>
      </div>
      <div style={{ overflowY: 'auto', flex: 1, display: 'flex', flexDirection: 'column', gap: 1 }}>
        {episodes.map((ep, i) => (
          <FeedRow key={ep.sig} ep={ep} fresh={i === 0} />
        ))}
      </div>
    </div>
  );
}

function FeedRow({ ep, fresh }: { ep: Episode; fresh: boolean }) {
  const winner = ep.score0 >= ep.score1 ? 0 : 1;
  const url = `https://explorer.solana.com/tx/${ep.sig}?cluster=devnet`;
  const time = new Date(ep.ts * 1000).toLocaleTimeString();

  return (
    <a href={url} target="_blank" rel="noreferrer" style={{
      display: 'block',
      padding: '10px 12px',
      background: fresh ? 'var(--green-dim)' : 'transparent',
      borderLeft: `2px solid ${fresh ? 'var(--green)' : 'var(--border)'}`,
      textDecoration: 'none',
      animation: fresh ? 'slide-in 0.3s ease' : 'none',
      flexShrink: 0,
    }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 5 }}>
        <span style={{ color: 'var(--green)', fontSize: 12, fontWeight: 700 }}>
          EP #{ep.id}
        </span>
        <span style={{ color: 'var(--text-dim)', fontSize: 10 }}>{time}</span>
      </div>
      <div style={{ display: 'flex', alignItems: 'center', gap: 10, marginBottom: 5 }}>
        <Score label="A0" value={ep.score0} win={winner === 0} />
        <span style={{ color: 'var(--border)', fontSize: 10 }}>vs</span>
        <Score label="A1" value={ep.score1} win={winner === 1} />
        <span style={{
          marginLeft: 'auto', fontSize: 9,
          padding: '1px 6px',
          border: `1px solid ${winner === 0 ? 'var(--green-dim)' : 'var(--amber-dim)'}`,
          color: winner === 0 ? 'var(--green)' : 'var(--amber)',
          borderRadius: 2,
        }}>
          A{winner} WIN
        </span>
      </div>
      <div style={{
        fontSize: 9, color: 'var(--text-dim)',
        overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap',
      }}>
        {ep.sig.slice(0, 44)}…
      </div>
    </a>
  );
}

function Score({ label, value, win }: { label: string; value: number; win: boolean }) {
  return (
    <span style={{ fontSize: 12 }}>
      <span style={{ color: 'var(--text-dim)' }}>{label}: </span>
      <strong style={{ color: win ? 'var(--green)' : 'var(--text)' }}>{value}</strong>
    </span>
  );
}
