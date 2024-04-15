fn main() {
    if cfg!(target_os = "linux") {
        // grit-ignore no_println_in_core
        println!("cargo:rustc-link-lib=stdc++");
        // grit-ignore no_println_in_core
        println!("cargo:rustc-link-lib=dylib=stdc++");
        // grit-ignore no_println_in_core
        println!("cargo:rustc-link-lib=c");
    } else if cfg!(target_os = "windows") {
        // Does not work yet
        // grit-ignore no_println_in_core
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}
