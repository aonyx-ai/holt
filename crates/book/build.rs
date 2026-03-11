fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let css_path = std::path::Path::new(&manifest_dir).join("assets/holt-book.css");

    // Resolve target dir from OUT_DIR
    // e.g. .../target/debug/build/holt-book-xxx/out -> .../target
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = std::path::Path::new(&out_dir)
        .ancestors()
        .find(|p| p.file_name().is_some_and(|n| n == "target"))
        .unwrap()
        .to_path_buf();

    let dest = target_dir.join("css/holt-book");
    std::fs::create_dir_all(&dest).ok();

    if css_path.exists() {
        std::fs::copy(&css_path, dest.join("holt-book.css")).ok();
    } else {
        eprintln!(
            "cargo:warning=crates/book/assets/holt-book.css not found. \
             Run `just generate-book-css` to generate it."
        );
    }

    println!("cargo:rerun-if-changed=assets/holt-book.css");
}
