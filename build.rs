fn main() {
    let dll_dir = "win_x64";
    println!("cargo:rustc-link-search=native={}", dll_dir);
}