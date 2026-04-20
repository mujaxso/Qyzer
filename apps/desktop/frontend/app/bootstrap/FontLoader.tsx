import { useEffect, useState } from 'react';

export function FontLoader() {
  const [fontsLoaded, setFontsLoaded] = useState(false);

  useEffect(() => {
    const checkFont = async () => {
      try {
        // Try to load the font using the FontFace API
        const font = new FontFace(
          'JetBrainsMono Nerd Font',
          'url("/fonts/JetBrainsMonoNerdFont-Regular.ttf") format("truetype")'
        );
        
        await font.load();
        document.fonts.add(font);
        
        // Check if the font is actually available
        await document.fonts.ready;
        
        setFontsLoaded(true);
        console.log('JetBrainsMono Nerd Font loaded successfully');
      } catch (error) {
        console.error('Failed to load JetBrainsMono Nerd Font:', error);
        setFontsLoaded(false);
      }
    };

    checkFont();
  }, []);

  // This component doesn't render anything visible
  return null;
}
