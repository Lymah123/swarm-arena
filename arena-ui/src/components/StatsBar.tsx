interface Props {
  totalEpisodes: number;
  ping: number;
  connected: boolean;
  programId: string;
}

export default function StatsBar({ totalEpisodes, ping, connected, programId }: Props) {
  return (
    <footer style={{
      display: 'flex',
      alignItems: 'center',
      gap: 32,
      padding: '8px 24px',
      borderTop: '1px solid var(--border)',
      background: 'var(--surface)',
      fontSize: 10,
      color: 'var(--text-dim)',
      fontFamily: 'var(--mono)',
    }}>
      <Stat label="PROGRAM" value={programId.slice(0, 8) + '…'} />
      <Stat label="EPISODES LOADED" value={totalEpisodes} />
      <Stat label="NETWORK" value="SOLANA DEVNET" />
      <Stat label="PING" value={`${ping}ms`} color={ping < 500 ? 'var(--green)' : 'var(--amber)'} />
      <Stat label="POLL" value="5s" />
    </footer>
  );
}

function Stat({ label, value, color }: { label: string; value: string | number; color?: string }) {
  return (
    <div style={{ display: 'flex', gap: 6 }}>
      <span>{label}:</span>
      <span style={{ color: color ?? 'var(--text)' }}>{value}</span>
    </div>
  );
}
