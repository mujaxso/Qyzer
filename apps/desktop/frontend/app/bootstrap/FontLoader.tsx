import { useEffect, useState } from 'react';

export function FontLoader() {
  const [fontsLoaded, setFontsLoaded] = useState(false);

  useEffect(() => {
    const checkFont = async () => {
      try {
        // Try to load the font using the FontFace API with the correct font family name
        const font = new FontFace(
          'JetBrainsMonoNL Nerd Font Mono',
          'url("/fonts/JetBrainsMonoNerdFont-Regular.ttf") format("truetype")'
        );
        
        await font.load();
        document.fonts.add(font);
        
        // Check if the font is actually available
        await document.fonts.ready;
        
        // Add a class to the body when the font is loaded
        document.body.classList.add('fonts-loaded');
        setFontsLoaded(true);
        console.log('JetBrainsMonoNL Nerd Font Mono loaded successfully');
      } catch (error) {
        console.error('Failed to load JetBrainsMonoNL Nerd Font Mono:', error);
        setFontsLoaded(false);
      }
    };

    checkFont();
  }, []);

  // This component doesn't render anything visible
  return null;
}
