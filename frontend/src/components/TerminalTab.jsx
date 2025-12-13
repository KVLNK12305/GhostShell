import { useEffect, useRef } from 'react';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import '@xterm/xterm/css/xterm.css';
import { WebSocket } from 'vite';

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
      convertEol: true,

/*When enabled the cursor will be set to the beginning of the next line with every new line.
This is equivalent to sending '\r\n' for each '\n'. Normally the termios settings of the underlying PTY deals with the translation of '\n' to '\r\n' and this setting should not be used. 
If you deal with data from a non-PTY related source, this settings might be useful.*/
    });
    
    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(divRef.current);
    termRef.current = term;
    
    // Small delay to ensure DOM is ready for measuring
    setTimeout(() => fitAddon.fit(), 0);

    // 2. Connect to Rust Backend
    // CHANGE THIS URL TO YOUR RENDER URL WHEN DEPLOYING
    // const wsUrl = `wss://stegosim-backend.onrender.com/ws?session=${sessionId}`;
    try{
      ws.current = new WebSocket(wsUrl);
        ws.current.binaryType = 'arraybuffer';

        ws.current.onopen = () => {
            term.write(`\r\n\x1b[32m[Connected to StegoSim Core]\x1b[0m\r\n$ `);
        };

        ws.current.onmessage = (event) => {
            term.write(new Uint8Array(event.data));
        };

        ws.current.onclose = () => {
            // BACKEND IS OFFLINE -> ENABLE LOCAL TYPING
            term.write('\r\n\x1b[33m[Offline Mode Enabled - Local Echo]\x1b[0m\r\n$ ');
        };

        ws.current.onerror = (err) => {
            // Prevent crash on connection error
            console.log("WS Error (Normal if backend is down)");
        };

    }
    catch(e){
      console.log("WS Setup failed");
    }
    // 3. Data Handling: "The Router"
    let command = ""; // Buffer to store typed command locally

    term.onData(data => {
      // OPTION A: If Backend is alive, send it there
      if (ws.current && ws.current.readyState === WebSocket.OPEN) {
        ws.current.send(data);
      } 
      // OPTION B: Backend is dead? Handle it locally (So you can type!)
      else {
        const code = data.charCodeAt(0);

        // Handle Enter (Run command)
        if (code === 13) {
            term.write('\r\n');
            if (command.trim() === 'help') {
                term.write('Available commands: help, clear, whoami\r\n');
            } else if (command.trim() === 'clear') {
                term.clear();
            } else if (command.trim() !== '') {
                term.write(`Command not found: ${command}\r\n`);
            }
            term.write('$ ');
            command = "";
        } 
        // Handle Backspace
        else if (code === 127) {
            if (command.length > 0) {
                term.write('\b \b'); // Visual backspace
                command = command.slice(0, -1);
            }
        } 
        // Handle Normal Typing
        else {
            term.write(data);
            command += data;
        }
      }
    });

    const resizeObserver = new ResizeObserver(() => fitAddon.fit());
    resizeObserver.observe(divRef.current);

    return () => {
      if (ws.current) ws.current.close();
      term.dispose();
      resizeObserver.disconnect();
    };
  }, []); // Run once on mount

  return <div ref={divRef} className="w-full h-full" />
}