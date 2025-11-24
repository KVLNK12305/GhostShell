import { useState } from 'react';
import TerminalTab from './components/Terminal';
import { Plus, X } from 'lucide-react';

function App() {
  // 1. State for Tabs (No types needed!)
  const [tabs, setTabs] = useState([{ id: '1', title: 'Terminal 1' }]);
  const [activeTabId, setActiveTabId] = useState('1');

  // 2. Add New Tab
  const addTab = () => {
    const newId = Date.now().toString();
    setTabs([...tabs, { id: newId, title: `Terminal ${tabs.length + 1}` }]);
    setActiveTabId(newId);
  };

  // 3. Close Tab
  const closeTab = (id, e) => {
    e.stopPropagation(); // Stop click from triggering "activate tab"
    const newTabs = tabs.filter(t => t.id !== id);
    setTabs(newTabs);
    
    // If we closed the active tab, switch to another one
    if (activeTabId === id && newTabs.length > 0) {
      setActiveTabId(newTabs[0].id);
    }
  };

  return (
    <div className="h-screen bg-black flex flex-col text-green-500 font-mono">
      {/* --- TAB BAR UI --- */}
      <div className="flex border-b border-gray-800 bg-gray-900 overflow-x-auto">
        {tabs.map(tab => (
          <div
            key={tab.id}
            onClick={() => setActiveTabId(tab.id)}
            className={`
              flex items-center px-4 py-2 cursor-pointer border-r border-gray-800 select-none min-w-[150px] justify-between
              ${activeTabId === tab.id ? 'bg-black text-white' : 'hover:bg-gray-800 text-gray-500'}
            `}
          >
            <span>{tab.title}</span>
            {tabs.length > 1 && (
              <X size={14} className="ml-2 hover:text-red-500" onClick={(e) => closeTab(tab.id, e)} />
            )}
          </div>
        ))}
        {/* Add Button */}
        <div 
          onClick={addTab}
          className="px-3 py-2 cursor-pointer hover:bg-gray-800 text-gray-500 flex items-center"
        >
          <Plus size={16} />
        </div>
      </div>

      {/* --- TERMINAL CONTENT --- */}
      <div className="flex-1 relative bg-black">
        {tabs.map(tab => (
          // We keep all tabs in the DOM (hidden) so sessions stay alive
          <div 
            key={tab.id} 
            className={`absolute inset-0 ${activeTabId === tab.id ? 'z-10' : 'z-0 invisible'}`}
          >
            <TerminalTab sessionId={tab.id} />
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;