import { Episode } from '../App';

const RESOURCES = [
  [2,2],[5,5],[7,3],[1,8],[9,1],[4,6],[6,4],[3,7],[8,8],[0,5]
];

interface Props { episodes: Episode[] }

export default function ArenaGrid({ episodes }: Props) {
  const last = episodes[0];
  const agent0Pos = last ? [last.score0 % 10, Math.floor(last.score0 / 10)] : [2, 2];
  const agent1Pos = last ? [last.score1 % 10, Math.floor(last.score1 / 10)] : [7, 7];

  return (
    <div className="panel" style={{ flex: 0 }}>
      <div className="panel-label">ARENA — 10×10 GRID</div>
      <div style={{
        display: 'grid',
        gridTemplateColumns: 'repeat(10, 1fr)',
        gap: 3,
        aspectRatio: '1',
        maxWidth: 320,
      }}>
        {Array.from({ length: 100 }, (_, i) => {
          const x = i % 10;
          const y = Math.floor(i / 10);
          const isRes = RESOURCES.some(([rx, ry]) => rx === x && ry === y);
          const isA0  = agent0Pos[0] === x && agent0Pos[1] === y;
          const isA1  = agent1Pos[0] === x && agent1Pos[1] === y;

          return (
            <div key={i} style={{
              aspectRatio: '1',
              borderRadius: 2,
              background: isA0
                ? 'var(--green)'
                : isA1
                ? 'var(--amber)'
                : isRes
                ? 'var(--green-dim)'
                : 'var(--border)',
              boxShadow: isA0
                ? '0 0 8px var(--green)'
                : isA1
                ? '0 0 8px var(--amber)'
                : 'none',
              transition: 'all 0.3s ease',
            }} />
          );
        })}
      </div>
      <div style={{ display: 'flex', gap: 16, marginTop: 12 }}>
        <Legend color="var(--green)" label="Agent 0" />
        <Legend color="var(--amber)" label="Agent 1" />
        <Legend color="var(--green-dim)" label="Resource" />
      </div>
    </div>
  );
}

function Legend({ color, label }: { color: string; label: string }) {
  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: 6, fontSize: 10, color: 'var(--text-dim)' }}>
      <div style={{ width: 8, height: 8, borderRadius: 1, background: color }} />
      {label}
    </div>
  );
}
