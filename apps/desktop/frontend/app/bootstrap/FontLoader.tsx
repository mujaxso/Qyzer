import { useEffect, useState } from 'react';

export function FontLoader() {
  const [fontsLoaded, setFontsLoaded] = useState(false);

  useEffect(() => {
    const loadFonts = async () => {
      try {
        // Create a FontFace for each variation using the proper Nerd Font family name
        const regular = new FontFace(
          'JetBrainsMono Nerd Font',
          'url("/fonts/JetBrainsMonoNerdFont-Regular.ttf") format("truetype")',
          { weight: '400', style: 'normal' }
        );
        const bold = new FontFace(
          'JetBrainsMono Nerd Font',
          'url("/fonts/JetBrainsMonoNerdFont-Bold.ttf") format("truetype")',
          { weight: '700', style: 'normal' }
        );
        const italic = new FontFace(
          'JetBrainsMono Nerd Font',
          'url("/fonts/JetBrainsMonoNerdFont-Italic.ttf") format("truetype")',
          { weight: '400', style: 'italic' }
        );
        const boldItalic = new FontFace(
          'JetBrainsMono Nerd Font',
          'url("/fonts/JetBrainsMonoNerdFont-BoldItalic.ttf") format("truetype")',
          { weight: '700', style: 'italic' }
        );

        // Load all fonts
        const loadedFonts = await Promise.all([
          regular.load(),
          bold.load(),
          italic.load(),
          boldItalic.load(),
        ]);

        // Add them to the document
        loadedFonts.forEach(font => document.fonts.add(font));

        // Wait for fonts to be ready
        await document.fonts.ready;

        // Mark fonts as loaded
        document.documentElement.setAttribute('data-fonts-loaded', 'true');
        document.body.classList.add('fonts-loaded');
        setFontsLoaded(true);
      } catch (error) {
        // Font loading failed silently
        setFontsLoaded(false);
      }
    };

    loadFonts();
  }, []);

  // While fonts are loading, we could show a loading indicator
  // For now, just return null
  return null;
}
