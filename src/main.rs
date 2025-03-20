// main.rs
use std::env;
use std::path::PathBuf;
use rofd::{export_ofd_to_png, read_ofd};

fn main() {
    let dll_path = PathBuf::from(env::current_dir().unwrap()).join("win_x64");
    env::set_var("PATH", format!("{};{}", dll_path.display(), env::var("PATH").unwrap()));
    // 后续代码...

    let ofd_node = read_ofd("./learning/test.ofd").unwrap();
    println!("ofd: {:#?}", ofd_node);

    let mut ofd_node = read_ofd("./learning/test.ofd").unwrap();
    println!("ofd: {:#?}", ofd_node);
    export_ofd_to_png(&mut ofd_node, "target/out.png", 400, 400).unwrap();
}