import { Episode } from '../App';

const RESOURCES = [
  [2,2],[5,5],[7,3],[1,8],[9,1],[4,6],[6,4],[3,7],[8,8],[0,5]
];

interface Props { episodes: Episode[] }

export default function ArenaGrid({ episodes }: Props) {
  const last = episodes[0];
  const a0 = last ? [last.score0 % 10, Math.floor(last.score0 * 3 / 10) % 10] : [2, 2];
  const a1 = last ? [last.score1 % 10, Math.floor(last.score1 * 7 / 10) % 10] : [7, 7];

  return (
    <div className="panel" style={{
      display: 'flex', flexDirection: 'column', overflow: 'hidden'
    }}>
      <div className="panel-label">ARENA — 10×10</div>

      <div style={{
        display: 'grid',
        gridTemplateColumns: 'repeat(10, 1fr)',
        gap: 2,
        flex: 1,
        minHeight: 0,
      }}>
        {Array.from({ length: 100 }, (_, i) => {
          const x = i % 10;
          const y = Math.floor(i / 10);
          const isRes = RESOURCES.some(([rx, ry]) => rx === x && ry === y);
          const isA0  = a0[0] === x && a0[1] === y;
          const isA1  = a1[0] === x && a1[1] === y;

          return (
            <div key={i} style={{
              borderRadius: 1,
              background: isA0
                ? 'var(--green)'
                : isA1
                ? 'var(--amber)'
                : isRes
                ? 'var(--green-dim)'
                : '#0f1c1c',
              boxShadow: isA0
                ? '0 0 6px var(--green)'
                : isA1
                ? '0 0 6px var(--amber)'
                : 'none',
              transition: 'all 0.4s ease',
              aspectRatio: '1',
            }} />
          );
        })}
      </div>

      <div style={{ display: 'flex', gap: 12, marginTop: 10, flexShrink: 0 }}>
        <Legend color="var(--green)" label="Agent 0" />
        <Legend color="var(--amber)" label="Agent 1" />
        <Legend color="var(--green-dim)" label="Resource" />
      </div>
    </div>
  );
}

function Legend({ color, label }: { color: string; label: string }) {
  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: 5, fontSize: 10, color: 'var(--text-dim)' }}>
      <div style={{ width: 7, height: 7, borderRadius: 1, background: color }} />
      {label}
    </div>
  );
}
