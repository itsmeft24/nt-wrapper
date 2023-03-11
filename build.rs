use std::env;

fn main() {
    println!("cargo:rustc-link-lib=dylib=ntcore");
    let path = env::current_dir().unwrap();
    println!("cargo:rustc-link-search={}/lib", path.display());
}
