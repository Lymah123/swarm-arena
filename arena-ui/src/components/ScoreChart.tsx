import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer, ReferenceLine } from 'recharts';
import { Episode } from '../App';

interface Props { episodes: Episode[] }

export default function ScoreChart({ episodes }: Props) {
  const data = [...episodes].reverse().slice(-20).map(ep => ({
    id: `#${ep.id}`,
    a0: ep.score0,
    a1: ep.score1,
  }));

  return (
    <div className="panel" style={{ display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
      <div className="panel-label">SCORE HISTORY — LAST 20 EPISODES</div>

      <div style={{ flex: 1, minHeight: 0 }}>
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={data} margin={{ top: 4, right: 8, left: -20, bottom: 0 }}>
            <XAxis
              dataKey="id"
              tick={{ fill: 'var(--text-dim)', fontSize: 9, fontFamily: 'var(--mono)' }}
              tickLine={false}
              axisLine={{ stroke: 'var(--border)' }}
              interval={6}
            />
            <YAxis
              domain={[0, 10]}
              tick={{ fill: 'var(--text-dim)', fontSize: 9, fontFamily: 'var(--mono)' }}
              tickLine={false}
              axisLine={false}
            />
            <ReferenceLine y={5} stroke="var(--border)" strokeDasharray="3 3" />
            <Tooltip
              contentStyle={{
                background: '#0d1317',
                border: '1px solid var(--border)',
                borderRadius: 2,
                fontSize: 11,
                fontFamily: 'var(--mono)',
              }}
              labelStyle={{ color: 'var(--text-dim)', marginBottom: 4 }}
              itemStyle={{ color: 'var(--text)' }}
            />
            <Line
              type="monotone" dataKey="a0"
              stroke="var(--green)" strokeWidth={1.5}
              dot={false} name="Agent 0"
              activeDot={{ r: 3, fill: 'var(--green)' }}
            />
            <Line
              type="monotone" dataKey="a1"
              stroke="var(--amber)" strokeWidth={1.5}
              dot={false} name="Agent 1"
              activeDot={{ r: 3, fill: 'var(--amber)' }}
            />
          </LineChart>
        </ResponsiveContainer>
      </div>

      <div style={{ display: 'flex', gap: 20, marginTop: 8, flexShrink: 0 }}>
        <Legend color="var(--green)" label="Agent 0" />
        <Legend color="var(--amber)" label="Agent 1" />
      </div>
    </div>
  );
}

function Legend({ color, label }: { color: string; label: string }) {
  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: 6, fontSize: 10, color: 'var(--text-dim)' }}>
      <div style={{ width: 24, height: 2, background: color }} />
      {label}
    </div>
  );
}
