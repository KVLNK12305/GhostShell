import { ShieldAlert, Terminal, Lock, Database } from 'lucide-react';

export default function Sidebar() {
  const missions = [
    { id: 1, title: "0x01: Basic Shell", icon: <Terminal size={18} />, status: "active" }
  ];

  return (
    <div className="w-64 bg-zinc-950 border-r border-zinc-800 flex flex-col h-full font-mono hidden md:flex">
      {/* Header */}
      <div className="p-4 border-b border-zinc-800">
        <h1 className="text-green-500 font-bold text-xl tracking-wider">GhostShell<span className="animate-pulse">_</span></h1>
        <p className="text-xs text-zinc-500 mt-1">v1.0.4 [UNCLASSIFIED]</p>
      </div>

      {/* Mission List */}
      <div className="flex-1 overflow-y-auto py-4">
        <p className="px-4 text-xs font-bold text-zinc-600 mb-2 uppercase">Active Operations</p>
        {missions.map((m) => (
          <div 
            key={m.id}
            className={`
              px-4 py-3 flex items-center gap-3 cursor-pointer border-l-2 transition-all
              ${m.status === 'active' 
                ? 'bg-zinc-900 border-green-500 text-green-400' 
                : 'border-transparent text-zinc-500 hover:text-zinc-300 hover:bg-zinc-900/50'}
            `}
          >
            {m.icon}
            <span className="text-sm">{m.title}</span>
            {m.status === 'locked' && <Lock size={12} className="ml-auto opacity-50" />}
          </div>
        ))}
      </div>

      {/* Footer / User Info */}
      <div className="p-4 border-t border-zinc-800">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded bg-green-900/20 border border-green-500/30 flex items-center justify-center text-green-500 text-xs font-bold">
            OP
          </div>
          <div>
            <div className="text-sm text-zinc-200">Student</div>
            <div className="text-xs text-zinc-500">Level 1 // Rookie</div>
          </div>
        </div>
      </div>
    </div>
  );
}