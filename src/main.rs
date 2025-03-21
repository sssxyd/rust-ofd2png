use ofd2png::*;

fn test_export_ofd_to_png() {
    let mut ofd_node = read_ofd("./learning/fapiao.ofd").unwrap();
    println!("ofd: {:#?}", ofd_node);
    let ret = export_ofd_to_png(&mut ofd_node, "target/out.png", 820, 1200);
    if ret.is_err() {
        println!("error: {:?}", ret.err().unwrap());
    }
}

fn main() {
    test_export_ofd_to_png();
}