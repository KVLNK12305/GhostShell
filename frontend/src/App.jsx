import { useState } from 'react';
import TerminalTab from './components/TerminalTab';
import Sidebar from './components/Sidebar';
import Trap from './components/Trap';
import GrimoireBot from './components/GrimoireBot'; 
import StegoEncoder from './components/StegoEncoder'; // <--- 1. Import the Tool
import { Plus, X, Monitor, Skull, Eye } from 'lucide-react'; // Added 'Eye' icon

function App() {
  // 2. Add 'type' to your initial state
  const [tabs, setTabs] = useState([
    { id: '1', title: 'Main Terminal', type: 'terminal' }
  ]);
  const [activeTabId, setActiveTabId] = useState('1');
  const [trapActive, setTrapActive] = useState(false);

  // 3. Helper to open specific tool tabs
  const openTool = (toolType) => {
    const newId = Date.now().toString();
    let title = `Session ${tabs.length + 1}`;
    
    if (toolType === 'stego') title = 'StegoSim v1.0';

    setTabs([...tabs, { id: newId, title, type: toolType }]);
    setActiveTabId(newId);
  };

  const closeTab = (id, e) => {
    e.stopPropagation();
    if (tabs.length === 1) return; 
    const newTabs = tabs.filter(t => t.id !== id);
    setTabs(newTabs);
    if (activeTabId === id) setActiveTabId(newTabs[0].id);
  };

  return (
    <div className="h-screen bg-black flex overflow-hidden text-zinc-300 select-none font-mono">
      <Trap active={trapActive} onReset={() => setTrapActive(false)} />
      <GrimoireBot />
      <Sidebar />

      <div className="flex-1 flex flex-col min-w-0 bg-black">
        {/* Top Navigation Bar */}
        <div className="h-12 border-b border-zinc-800 bg-zinc-950 flex items-center px-2 justify-between">
          
          <div className="flex items-center gap-1 overflow-x-auto no-scrollbar flex-1">
            {tabs.map(tab => (
              <div
                key={tab.id}
                onClick={() => setActiveTabId(tab.id)}
                className={`
                  group flex items-center gap-2 px-3 py-1.5 text-xs border-t-2 cursor-pointer transition-colors min-w-[140px] max-w-[200px]
                  ${activeTabId === tab.id 
                    ? 'border-green-500 bg-zinc-900 text-green-400' 
                    : 'border-transparent text-zinc-500 hover:bg-zinc-900/50 hover:text-zinc-300'}
                `}
              >
                {/* 4. Dynamic Icon based on Tab Type */}
                {tab.type === 'stego' ? <Eye size={12} /> : <Monitor size={12} />}
                
                <span className="truncate flex-1">{tab.title}</span>
                <X 
                  size={12} 
                  className="opacity-0 group-hover:opacity-100 hover:text-red-500"
                  onClick={(e) => closeTab(tab.id, e)}
                />
              </div>
            ))}
            
            {/* Standard "New Terminal" Button */}
            <button 
              onClick={() => openTool('terminal')}
              className="p-1.5 hover:bg-zinc-800 text-zinc-500 hover:text-green-500 rounded ml-1"
              title="New Terminal"
            >
              <Plus size={14} />
            </button>
          </div>

          <div className="flex gap-2">
             {/* 5. NEW BUTTON: Launch Stego Tool */}
            <button 
              onClick={() => openTool('stego')}
              className="flex items-center gap-2 px-3 py-1 text-xs bg-purple-900/10 text-purple-400 border border-purple-900/30 hover:bg-purple-900/30 rounded transition-colors"
            >
              <Eye size={12} />
              <span className="hidden sm:inline">STEGO TOOL</span>
            </button>

            <button 
              onClick={() => setTrapActive(true)}
              className="flex items-center gap-2 px-3 py-1 text-xs bg-red-900/10 text-red-500 border border-red-900/30 hover:bg-red-900/30 rounded transition-colors"
            >
              <Skull size={12} />
              <span className="hidden sm:inline">TEST TRAP</span>
            </button>
          </div>
        </div>

        {/* Content Area */}
        <div className="flex-1 relative bg-black p-1">
          {tabs.map(tab => (
            <div 
              key={tab.id} 
              className={`absolute inset-0 ${activeTabId === tab.id ? 'z-10' : 'z-0 invisible'}`}
            >
              <div className="h-full border border-zinc-800/50 rounded bg-black shadow-inner overflow-hidden relative">
                 {/* 6. CONDITIONAL RENDERING */}
                 {tab.type === 'stego' ? (
                   <div className="h-full overflow-auto bg-zinc-900/50 p-4">
                      <StegoEncoder />
                   </div>
                 ) : (
                   <TerminalTab sessionId={tab.id} />
                 )}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

export default App;