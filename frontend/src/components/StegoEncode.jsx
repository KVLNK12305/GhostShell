import React, { useState } from 'react';
import { useSteganography } from '../hooks/useSteg'; // Ensure this path is correct
import { Upload, Lock, Download, AlertCircle, CheckCircle, FileImage } from 'lucide-react';

const StegoEncoder = () => {
  const [selectedFile, setSelectedFile] = useState(null);
  const [previewUrl, setPreviewUrl] = useState(null);
  const [secretText, setSecretText] = useState("");
  const [resultImage, setResultImage] = useState(null);

  const { encode, isProcessing, error } = useSteganography();

  const handleFileChange = (e) => {
    if (e.target.files && e.target.files[0]) {
      const file = e.target.files[0];
      setSelectedFile(file);
      setResultImage(null); // Reset result
      
      // Create a temporary preview URL
      const objectUrl = URL.createObjectURL(file);
      setPreviewUrl(objectUrl);
    }
  };

  const handleEncode = async () => {
    if (!selectedFile || !secretText) return;
    const encodedUrl = await encode(selectedFile, secretText);
    if (encodedUrl) {
      setResultImage(encodedUrl);
    }
  };

  return (
    <div className="flex flex-col h-full text-zinc-300 font-mono animate-in fade-in duration-300">
      
      {/* Header */}
      <div className="flex items-center gap-2 mb-6 border-b border-purple-500/20 pb-4">
        <Lock className="text-purple-400" size={20} />
        <h2 className="text-xl font-bold text-purple-100 tracking-wider">GhostShell<span className="text-purple-500 text-sm">// ENCODER PROTOCOL</span></h2>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 h-full overflow-y-auto">
        
        {/* LEFT COLUMN: Inputs */}
        <div className="space-y-6">
          
          {/* 1. Image Upload */}
          <div className="p-4 border border-zinc-700 bg-black/40 rounded-lg hover:border-purple-500/50 transition-colors">
            <label className="block text-xs font-bold text-zinc-500 mb-2 uppercase">1. Target Carrier (PNG Only)</label>
            <div className="relative group">
              <input 
                type="file" 
                accept="image/png"
                onChange={handleFileChange}
                className="absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10"
              />
              <div className="flex items-center gap-3 p-3 border border-dashed border-zinc-600 rounded bg-zinc-900/50 group-hover:bg-zinc-900 transition-colors">
                <div className="p-2 bg-zinc-800 rounded">
                  <Upload size={18} className="text-purple-400" />
                </div>
                <div className="flex-1 min-w-0">
                  <p className="text-sm font-medium truncate text-zinc-300">
                    {selectedFile ? selectedFile.name : "Select Image File..."}
                  </p>
                  <p className="text-xs text-zinc-600">
                    {selectedFile ? `${(selectedFile.size / 1024).toFixed(1)} KB` : "Drag & drop or click"}
                  </p>
                </div>
              </div>
            </div>
          </div>

          {/* 2. Secret Message */}
          <div className="p-4 border border-zinc-700 bg-black/40 rounded-lg">
             <label className="block text-xs font-bold text-zinc-500 mb-2 uppercase">2. Payload Data</label>
             <textarea 
                value={secretText}
                onChange={(e) => setSecretText(e.target.value)}
                className="w-full h-32 bg-zinc-900 border border-zinc-700 p-3 text-sm text-green-400 focus:outline-none focus:border-purple-500 rounded resize-none placeholder-zinc-700"
                placeholder="Enter secret message sequence..."
             />
             <div className="flex justify-between mt-2">
                <span className="text-xs text-zinc-600">CHARS: {secretText.length}</span>
                <span className="text-xs text-zinc-600">CAPACITY: ~{(selectedFile ? (selectedFile.size / 8).toFixed(0) : "0")} CHARS</span>
             </div>
          </div>

          {/* 3. Action Button */}
          <button
            onClick={handleEncode}
            disabled={!selectedFile || !secretText || isProcessing}
            className={`w-full py-3 px-4 rounded font-bold tracking-widest text-sm flex items-center justify-center gap-2 transition-all
              ${isProcessing 
                ? 'bg-zinc-800 text-zinc-500 cursor-not-allowed border border-zinc-700' 
                : 'bg-purple-900/20 border border-purple-500/50 text-purple-400 hover:bg-purple-500 hover:text-white shadow-[0_0_15px_rgba(168,85,247,0.2)]'
              }`}
          >
            {isProcessing ? (
              <>
                <span className="animate-pulse">ENCRYPTING PIXELS...</span>
              </>
            ) : (
              <>
                <Lock size={16} />
                INITIATE ENCODING
              </>
            )}
          </button>

          {/* Error Message */}
          {error && (
            <div className="flex items-center gap-2 p-3 bg-red-900/20 border border-red-500/50 text-red-400 text-xs rounded">
              <AlertCircle size={14} />
              <span>ERROR: {error}</span>
            </div>
          )}
        </div>


        {/* RIGHT COLUMN: Preview & Result */}
        <div className="border border-zinc-800 bg-black/20 rounded-lg p-4 flex flex-col items-center justify-center relative overflow-hidden">
          
          {!selectedFile && !resultImage && (
            <div className="text-center text-zinc-600">
              <FileImage size={48} className="mx-auto mb-2 opacity-20" />
              <p className="text-sm">Awaiting Source Material</p>
            </div>
          )}

          {/* Preview of Original */}
          {selectedFile && !resultImage && previewUrl && (
             <div className="relative max-w-full">
                <img src={previewUrl} alt="Preview" className="max-h-[400px] rounded border border-zinc-700 opacity-50 grayscale" />
                <div className="absolute top-2 right-2 bg-black/80 px-2 py-1 text-xs text-zinc-400 rounded border border-zinc-800">ORIGINAL</div>
             </div>
          )}

          {/* Result */}
          {resultImage && (
            <div className="w-full flex flex-col items-center animate-in zoom-in duration-300">
              <div className="flex items-center gap-2 text-green-500 mb-4 bg-green-900/20 px-4 py-1 rounded-full border border-green-500/30">
                <CheckCircle size={14} />
                <span className="text-xs font-bold">ENCODING SUCCESSFUL</span>
              </div>
              
              <div className="relative group">
                <img src={resultImage} alt="Stego Result" className="max-h-[350px] rounded border border-purple-500/50 shadow-[0_0_30px_rgba(168,85,247,0.15)]" />
                <div className="absolute top-2 right-2 bg-purple-900/90 px-2 py-1 text-xs text-purple-200 rounded border border-purple-500">MODIFIED</div>
              </div>

              <a 
                href={resultImage} 
                download={`stego_${selectedFile.name}`}
                className="mt-6 flex items-center gap-2 px-6 py-2 bg-zinc-100 text-black hover:bg-white rounded font-bold text-sm transition-colors"
              >
                <Download size={16} />
                DOWNLOAD RESULT
              </a>
              <p className="mt-2 text-[10px] text-zinc-500">FORMAT: PNG (LOSSLESS)</p>
            </div>
          )}
        </div>

      </div>
    </div>
  );
};

export default StegoEncoder;