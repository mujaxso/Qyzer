fn main() {
    // Check if font files exist, and print a warning if they don't
    let noto_sans_path = "assets/fonts/NotoSans-Regular.ttf";
    let noto_emoji_path = "assets/fonts/NotoEmoji-Regular.ttf";
    
    if !std::path::Path::new(noto_sans_path).exists() || !std::path::Path::new(noto_emoji_path).exists() {
        println!("cargo:warning=Font files not found. Run `scripts/download-fonts.sh` to download them.");
        println!("cargo:warning=Icons may not display correctly without the required fonts.");
    }
}
