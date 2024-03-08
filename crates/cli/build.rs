fn main() {
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=c");
    } else if cfg!(target_os = "windows") {
        // Does not work yet
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}
