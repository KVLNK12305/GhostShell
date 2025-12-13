import { useState, useCallback } from 'react';

export const useSteganography = () => {
  const [isProcessing, setIsProcessing] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Helper: Convert string to binary
  const textToBinary = (text: string) => {
    return text.split('').map(char =>
      char.charCodeAt(0).toString(2).padStart(8, '0')
    ).join('') + '00000000'; // Null terminator to mark end of message
  };

  // Helper: Convert binary to text
  const binaryToText = (binary: string) => {
    const bytes = binary.match(/.{1,8}/g) || [];
    let text = "";
    for (let byte of bytes) {
      const charCode = parseInt(byte, 2);
      if (charCode === 0) break; // Stop at null terminator
      text += String.fromCharCode(charCode);
    }
    return text;
  };

  const encode = useCallback(async (imageFile: File, secretText: string): Promise<string | null> => {
    setIsProcessing(true);
    setError(null);

    return new Promise((resolve) => {
      const reader = new FileReader();
      reader.onload = (event) => {
        const img = new Image();
        img.onload = () => {
          const canvas = document.createElement('canvas');
          canvas.width = img.width;
          canvas.height = img.height;
          const ctx = canvas.getContext('2d');
          
          if (!ctx) {
            setError("Canvas context failed");
            setIsProcessing(false);
            return resolve(null);
          }

          ctx.drawImage(img, 0, 0);
          const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
          const data = imageData.data;
          const binarySecret = textToBinary(secretText);

          // THE EUREKA MOMENT: LSB Replacement
          if (binarySecret.length > data.length / 4) {
            setError("Text is too long for this image!");
            setIsProcessing(false);
            return resolve(null);
          }

          for (let i = 0; i < binarySecret.length; i++) {
            // Modify the Red channel (every 4th byte is R, G, B, A)
            // We use logic to modify R, G, or B. Here we just walk through bytes.
            // data[i] is the byte (0-255).
            // We clear the last bit using bitwise AND (& 254)
            // Then we add our secret bit using bitwise OR (| bit)
            data[i * 4] = (data[i * 4] & 254) | parseInt(binarySecret[i]); 
          }

          ctx.putImageData(imageData, 0, 0);
          setIsProcessing(false);
          resolve(canvas.toDataURL('image/png')); // Must be PNG!
        };
        img.src = event.target?.result as string;
      };
      reader.readAsDataURL(imageFile);
    });
  }, []);

  const decode = useCallback(async (imageFile: File): Promise<string | null> => {
    setIsProcessing(true);
    // ... Logic to extract bits and run binaryToText ...
    // (Similar structure to encode, but reading LSBs instead of writing)
    setIsProcessing(false);
    return "Decoded Text Placeholder"; 
  }, []);

  return { encode, decode, isProcessing, error };
};