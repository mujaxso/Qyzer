fn main() {
    // Check if font files exist from various possible locations
    let possible_paths = [
        // Relative to build.rs location
        "assets/fonts/NotoSans-Regular.ttf",
        "assets/fonts/NotoColorEmoji.ttf",
        "assets/fonts/NotoEmoji-Regular.ttf",
        // Relative to project root
        "apps/desktop/assets/fonts/NotoSans-Regular.ttf",
        "apps/desktop/assets/fonts/NotoColorEmoji.ttf",
        "apps/desktop/assets/fonts/NotoEmoji-Regular.ttf",
    ];
    
    let mut sans_found = false;
    let mut emoji_found = false;
    
    for path in &possible_paths {
        if std::path::Path::new(path).exists() {
            if path.contains("NotoSans") {
                sans_found = true;
                println!("cargo:warning=Found Noto Sans at: {}", path);
            }
            if path.contains("NotoColorEmoji") || path.contains("NotoEmoji") {
                emoji_found = true;
                println!("cargo:warning=Found emoji font at: {}", path);
            }
        }
    }
    
    if !sans_found || !emoji_found {
        println!("cargo:warning=Font files not found. Icons may not display correctly.");
        println!("cargo:warning=To download fonts, run from apps/desktop directory:");
        println!("cargo:warning=  chmod +x scripts/download-fonts.sh");
        println!("cargo:warning=  ./scripts/download-fonts.sh");
        
        if !sans_found {
            println!("cargo:warning=Missing: NotoSans-Regular.ttf");
        }
        if !emoji_found {
            println!("cargo:warning=Missing: NotoColorEmoji.ttf or NotoEmoji-Regular.ttf");
        }
    } else {
        println!("cargo:warning=Font files found. Icons should display correctly.");
    }
}
