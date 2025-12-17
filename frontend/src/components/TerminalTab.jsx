import { useEffect, useRef } from 'react';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';

// Simple JS function component, receiving props directly
export default function TerminalTab({ sessionId }) {
  const divRef = useRef(null);
  const ws = useRef(null);

  useEffect(() => {
    if (!divRef.current) return;

    // 1. Setup xterm
    const term = new Terminal({
      cursorBlink: true,
      theme: { background: '#000000', foreground: '#00ff00' },
      fontFamily: 'Courier New, monospace',
      fontSize: 14,
    });
    
    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(divRef.current);
    
    // Small delay to ensure DOM is ready for measuring
    setTimeout(() => fitAddon.fit(), 0);

    // 2. Connect to Rust Backend
    // CHANGE THIS URL TO YOUR RENDER URL WHEN DEPLOYING
    // const wsUrl = `wss://stegosim-backend.onrender.com/ws?session=${sessionId}`;
    const wsUrl = `ws://localhost:3000/ws?session=${sessionId}`;
    
    ws.current = new WebSocket(wsUrl);
    ws.current.binaryType = 'arraybuffer'; // Mandatory for PTY

    ws.current.onopen = () => {
      term.write(`\r\n\x1b[32m[Connected to Session ${sessionId}]\x1b[0m\r\n$ `);
    };

    ws.current.onmessage = (event) => {
      // Convert raw binary from Rust to Uint8Array for xterm
      term.write(new Uint8Array(event.data));
    };

    ws.current.onclose = () => {
        term.write('\r\n\x1b[31m[Connection Lost]\x1b[0m');
    };

    term.onData(data => {
      if (ws.current && ws.current.readyState === WebSocket.OPEN) {
        ws.current.send(data);
      }
    });

    // Handle Window Resize
    const resizeObserver = new ResizeObserver(() => fitAddon.fit());
    resizeObserver.observe(divRef.current);

    // Cleanup
    return () => {
      if (ws.current) ws.current.close();
      term.dispose();
      resizeObserver.disconnect();
    };
  }, []); // Run once on mount

  return <div ref={divRef} className="w-full h-full" />;
}