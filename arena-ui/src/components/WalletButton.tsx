import { useWallet } from '@solana/wallet-adapter-react';
import { useWalletModal } from '@solana/wallet-adapter-react-ui';

export default function WalletButton() {
  const { connected, publicKey, disconnect } = useWallet();
  const { setVisible } = useWalletModal();

  if (connected && publicKey) {
    return (
      <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
        <span style={{
          fontSize: 11,
          color: 'var(--green)',
          fontFamily: 'var(--mono)',
          border: '1px solid var(--green-dim)',
          padding: '2px 8px',
          borderRadius: 2,
        }}>
          {publicKey.toString().slice(0, 4)}...{publicKey.toString().slice(-4)}
        </span>
        <button
          onClick={() => disconnect()}
          style={{
            padding: '3px 10px',
            background: 'transparent',
            color: 'var(--text-dim)',
            border: '1px solid var(--border)',
            borderRadius: 2,
            cursor: 'pointer',
            fontSize: 10,
            fontFamily: 'var(--mono)',
            letterSpacing: '0.1em',
          }}
        >
          DISCONNECT
        </button>
      </div>
    );
  }

  return (
    <button
      onClick={() => setVisible(true)}
      style={{
        padding: '4px 14px',
        background: 'var(--green)',
        color: '#000',
        border: 'none',
        borderRadius: 2,
        cursor: 'pointer',
        fontSize: 11,
        fontWeight: 700,
        fontFamily: 'var(--mono)',
        letterSpacing: '0.1em',
      }}
    >
      CONNECT WALLET
    </button>
  );
}
