import { LineChart, Line, XAxis, YAxis, Tooltip, ResponsiveContainer } from 'recharts';
import { Episode } from '../App';

interface Props { episodes: Episode[] }

export default function ScoreChart({ episodes }: Props) {
  const data = [...episodes].reverse().slice(-20).map(ep => ({
    id: ep.id,
    agent0: ep.score0,
    agent1: ep.score1,
  }));

  return (
    <div className="panel" style={{ flex: 1 }}>
      <div className="panel-label">SCORE HISTORY — LAST 20 EPISODES</div>
      <ResponsiveContainer width="100%" height={140}>
        <LineChart data={data}>
          <XAxis dataKey="id" hide />
          <YAxis domain={[0, 10]} hide />
          <Tooltip
            contentStyle={{
              background: 'var(--surface)',
              border: '1px solid var(--border)',
              borderRadius: 2,
              fontSize: 11,
              fontFamily: 'var(--mono)',
            }}
            labelStyle={{ color: 'var(--text-dim)' }}
          />
          <Line
            type="monotone" dataKey="agent0"
            stroke="var(--green)" strokeWidth={1.5}
            dot={false} name="Agent 0"
          />
          <Line
            type="monotone" dataKey="agent1"
            stroke="var(--amber)" strokeWidth={1.5}
            dot={false} name="Agent 1"
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}
