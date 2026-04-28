export default function Footer() {
 return (
  <footer style={{
   borderTop: '1px solid var(--border)',
   padding: '20px 24px',
   background: 'var(--surface)',
   display: 'flex',
   justifyContent: 'space-between',
   alignItems: 'center',
   fontSize: '11px',
   color: 'var(--text-dim)',
   fontFamily: 'var(--mono)',
   marginTop: 'auto',
  }}>
   <div>
    <a
     href="https://github.com/Lymah123/swarm-arena"
     target="_blank"
     rel="noreferrer"
     style={{
      color: 'var(--green)',
      textDecoration: 'none',
      borderBottom: '1px solid var(--green-dim)',
     }}
    >
     GitHub
    </a>
   </div>

   <div style={{ display: 'flex', gap: '24px' }}>
    <span>Community:</span>
    <a
     href="https://swarm.thecanteenapp.com/"
     target="_blank"
     rel="noreferrer"
     style={{
      color: 'var(--amber)',
      textDecoration: 'none',
      borderBottom: '1px solid var(--amber-dim)',
     }}
    >
     Swarm Hackathon
    </a>
    <span>·</span>
    <a
     href="https://arena.colosseum.org/refresh-session?redirectBack=%2Fhackathon"
     target="_blank"
     rel="noreferrer"
     style={{
      color: 'var(--green)',
      textDecoration: 'none',
      borderBottom: '1px solid var(--green-dim)',
     }}
    >
     Colosseum Arena
    </a>
   </div>

   <div style={{ fontSize: '10px' }}>
    SWARM ARENA © 2026 — AGENTIC HACKATHON
   </div>
  </footer>
 );
}
