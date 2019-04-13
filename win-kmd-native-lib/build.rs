fn main() {
    use std::env::var;
    use std::path::Path;

    let target = var("TARGET").unwrap();

    if target.contains("x86_64") {
        let dir = var("CARGO_MANIFEST_DIR").unwrap();
        println!("cargo:rustc-link-search=native={}", Path::new(&dir).join(r#"win7\km\x64"#).display());
    }

    if target.contains("i686") {
        let dir = var("CARGO_MANIFEST_DIR").unwrap();
        println!("cargo:rustc-link-search=native={}", Path::new(&dir).join(r#"win7\km\x86"#).display());
    }
}