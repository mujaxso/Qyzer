fn main() {
    // Check if font files exist, and print a warning if they don't
    let noto_sans_path = "assets/fonts/NotoSans-Regular.ttf";
    let noto_color_emoji_path = "assets/fonts/NotoColorEmoji.ttf";
    let noto_emoji_path = "assets/fonts/NotoEmoji-Regular.ttf";
    
    let sans_exists = std::path::Path::new(noto_sans_path).exists();
    let emoji_exists = std::path::Path::new(noto_color_emoji_path).exists() || 
                       std::path::Path::new(noto_emoji_path).exists();
    
    if !sans_exists || !emoji_exists {
        println!("cargo:warning=Font files not found. Run `scripts/download-fonts.sh` to download them.");
        println!("cargo:warning=Icons may not display correctly without the required fonts.");
        
        if !sans_exists {
            println!("cargo:warning=Missing: NotoSans-Regular.ttf");
        }
        if !emoji_exists {
            println!("cargo:warning=Missing: NotoColorEmoji.ttf or NotoEmoji-Regular.ttf");
        }
    } else {
        println!("cargo:warning=Font files found. Icons should display correctly.");
    }
}
