fn main() {
    let root_dir = std::path::Path::new(".");
    let php_dir = root_dir.join("php").join("src");
    let php_only_dir = root_dir.join("php_only").join("src");

    let mut c_config = cc::Build::new();
    c_config.include(&php_dir);
    c_config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable")
        .flag_if_supported("-Wno-trigraphs");

    for path in &[
        php_dir.join("parser.c"),
        php_dir.join("scanner.c"),
        php_only_dir.join("parser.c"),
        php_only_dir.join("scanner.c"),
    ] {
        c_config.file(&path);
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    println!(
        "cargo:rerun-if-changed={}",
        root_dir.join("common").join("scanner.h").to_str().unwrap()
    );

    c_config.compile("parser-scanner");
}
