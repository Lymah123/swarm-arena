import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

export default function WalletButton() {
 const { connected, publicKey } = useWallet();

 return (
  <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
   {connected && publicKey && (
    <span style={{ fontSize: '12px', color: '#0f0', fontFamily: 'monospace' }}>
     {publicKey.toString().slice(0, 8)}...
    </span>
   )}
   <WalletMultiButton />
  </div>
 );
}
