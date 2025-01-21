use std::fs;
use std::path::Path;
use std::io::{BufReader, Read};

use zip::ZipArchive;

use rofd::types::CT_PageArea;
use rofd::ofd::{Ofd, OfdNode};
use rofd::document::Document;
use rofd::document_res::Res;
use rofd::page::Page;
use rofd::render::Renderable;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    // TODO(hualet): these initializaiton really needed ?
    let mut ofd_node = OfdNode::default();
    let mut document = Document::default();
    let mut document_res = Res::default();

    let file = fs::File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let mut zip = ZipArchive::new(reader).unwrap();

    {
        // find the OFD.xml file and parse the content to ofd object.
        let mut ofd_file = zip.by_name("OFD.xml").unwrap();
        let mut content = String::new();
        let _size = ofd_file.read_to_string(&mut content).unwrap();
        ofd_node = OfdNode::from_xml(&content).unwrap();
        println!("ofd: {:#?}", ofd_node);
    }


    let mut ofd = Ofd {
        node: ofd_node,
        zip_archive: zip,
    };

    {
        // find the DocRoot file and parse the content to document object.
        let mut doc_file = ofd.zip_archive.by_name(ofd.node.doc_body.doc_root.as_str()).unwrap();
        let mut content = String::new();
        let _size = doc_file.read_to_string(&mut content).unwrap();
        document = Document::from_xml(&content).unwrap();
        println!("document: {:#?}", document);
    }

    {
        // find the DocumentRes.xml file and parse the content to document_res object.
        let path = Path::new(ofd.node.doc_body.doc_root.as_str());
        let res_path = &path.parent().unwrap().join(document.common_data.document_res.as_str());
        let mut document_res_file = ofd.zip_archive.by_name(res_path.to_str().unwrap()).unwrap();
        let mut content = String::new();
        let _size = document_res_file.read_to_string(&mut content).unwrap();
        document_res = Res::from_xml(&content).unwrap();
        println!("document_res: {:#?}", document_res);
    }
    document.res = document_res;

    let pybox = CT_PageArea::from(
        document.common_data.page_area.physical_box.clone()).toPixel();

    let mut surface = cairo::ImageSurface::create(
        cairo::Format::ARgb32,
        pybox.width as i32, pybox.height as i32).unwrap();
    let mut context = cairo::Context::new(&surface).unwrap();

    for i in 0..document.pages.page.len() {
        let page = &document.pages.page[i];
        let mut content = String::new();

        {
            // concat basename of document.doc_body.doc_root and page.base_loc
            let path = Path::new(ofd.node.doc_body.doc_root.as_str());
            let mut page_file = ofd.zip_archive.by_name(
                &path.parent().unwrap().join(
                page.base_loc.as_ref().unwrap().as_str()).to_str().unwrap()).unwrap();
            let _size = page_file.read_to_string(&mut content).unwrap();
        }

        let mut page = Page::from_xml(&content).unwrap();
        // println!("page: {:#?}", page);

        page.render(&mut context, &mut ofd, &mut document);
    }

    let mut file = fs::File::create("target/out.png").unwrap();
    surface.write_to_png(&mut file).unwrap();

    0
}