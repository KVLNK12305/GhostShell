import { Skull } from 'lucide-react';

export default function Trap({ active, onReset }) {
  if (!active) return null;

  return (
    <div className="fixed inset-0 z-50 bg-black flex flex-col items-center justify-center text-red-600 animate-pulse font-mono">
      <h1 className="text-6xl md:text-9xl font-black tracking-tighter mb-4 text-center">
        TRAP TRIGGERED
      </h1>
      <Skull size={120} className="mb-8 animate-bounce" />
      <p className="text-xl md:text-2xl text-center max-w-2xl px-4 mb-8">
        ILLEGAL OPERATION DETECTED.<br/>
        IP ADDRESS LOGGED.<br/>
        SYSTEM LOCKDOWN INITIATED.
      </p>
    </div>
  );
}