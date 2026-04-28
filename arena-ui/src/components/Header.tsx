import WalletButton from './WalletButton';

interface Props {
  programId: string;
  balance: number;
  connected: boolean;
}

export default function Header({ programId, balance, connected }: Props) {
  const short = programId.slice(0, 8) + '...' + programId.slice(-6);
  const explorerUrl = `https://explorer.solana.com/address/${programId}?cluster=devnet`;

  return (
    <header style={{
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'space-between',
      padding: '14px 24px',
      borderBottom: '1px solid var(--border)',
      background: 'var(--surface)',
    }}>
      <div style={{ display: 'flex', alignItems: 'baseline', gap: 16 }}>
        <span style={{
          fontFamily: 'var(--display)',
          fontWeight: 800,
          fontSize: 22,
          color: 'var(--green)',
          letterSpacing: '-0.02em',
        }}>SWARM ARENA</span>
        <span style={{ color: 'var(--text-dim)', fontSize: 11 }}>
          on-chain agent training
        </span>
      </div>

      <div style={{ display: 'flex', alignItems: 'center', gap: 24 }}>
        <a
          href={explorerUrl}
          target="_blank"
          rel="noreferrer"
          style={{
            color: 'var(--text-dim)',
            textDecoration: 'none',
            fontSize: 11,
            fontFamily: 'var(--mono)',
            borderBottom: '1px solid var(--border)',
            paddingBottom: 1,
          }}
        >
          {short}
        </a>

        <span style={{
          fontSize: 11,
          color: 'var(--amber)',
          border: '1px solid var(--amber-dim)',
          padding: '2px 8px',
          borderRadius: 2,
        }}>DEVNET</span>

        <span style={{ color: 'var(--text-dim)', fontSize: 11 }}>
          {balance.toFixed(4)} SOL
        </span>

        <WalletButton />

        <span style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
          <span style={{
            width: 7, height: 7, borderRadius: '50%',
            background: connected ? 'var(--green)' : 'var(--red)',
            boxShadow: connected ? '0 0 6px var(--green)' : 'none',
            animation: connected ? 'pulse-green 2s infinite' : 'none',
          }} />
          <span style={{ fontSize: 11, color: 'var(--text-dim)' }}>
            {connected ? 'LIVE' : 'OFFLINE'}
          </span>
        </span>
      </div>
    </header>
  );
}
