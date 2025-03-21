use ofd2png::*;

#[test]
fn test_read_ofd() {
    let result = read_ofd("./learning/fapiao.ofd");
    if result.is_err() {
        println!("error: {:?}", result.err().unwrap());
        
    } else {
        let ofd = result.unwrap();
        println!("ofd: {:#?}", ofd);
    }
}

#[test]
fn test_export_ofd_to_png() {
    let mut ofd_node = read_ofd("./learning/test.ofd").unwrap();
    println!("ofd: {:#?}", ofd_node);
    export_ofd_to_png(&mut ofd_node, "target/out.png", 400, 400).unwrap();
}