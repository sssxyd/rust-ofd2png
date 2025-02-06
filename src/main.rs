use rofd::*;

use env_logger;

fn main() {
    env_logger::init();
    let mut ofd_node = read_ofd("learning/test.ofd").unwrap();
    export_ofd_to_png(&mut ofd_node, "target/out.png").unwrap();
}