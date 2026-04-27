import { useWallet } from '@solana/wallet-adapter-react';
import { useWalletModal } from '@solana/wallet-adapter-react-ui';

export default function WalletButton() {
 const { connected, publicKey, disconnect } = useWallet();
 const { setVisible } = useWalletModal();

 return (
  <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
   {connected && publicKey ? (
    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
     <span style={{ fontSize: '12px', color: '#0f0', fontFamily: 'monospace' }}>
      {publicKey.toString().slice(0, 8)}...
     </span>
     <button
      onClick={() => disconnect()}
      style={{
       padding: '6px 12px',
       backgroundColor: '#222',
       color: '#0f0',
       border: '1px solid #0f0',
       borderRadius: '4px',
       cursor: 'pointer',
       fontSize: '12px',
       fontFamily: 'monospace',
      }}
     >
      Disconnect
     </button>
    </div>
   ) : (
    <button
     onClick={() => setVisible(true)}
     style={{
      padding: '8px 16px',
      backgroundColor: '#0f0',
      color: '#000',
      border: 'none',
      borderRadius: '4px',
      cursor: 'pointer',
      fontSize: '12px',
      fontWeight: 'bold',
      fontFamily: 'monospace',
     }}
    >
     Connect Wallet
    </button>
   )}
  </div>
 );
}
