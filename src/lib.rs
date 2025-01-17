pub mod ofd;
pub mod document;
pub mod render;
pub mod page;
pub mod types;

use std::fs;
use std::path::Path;
use std::io::Read;
use std::io::BufReader;

use zip::ZipArchive;

use ofd::Ofd;
use document::{Document, PageElement};
use page::Page;
use render::Renderable;
use types::CT_PageArea;

impl Ofd {
    pub fn from_filename(filename: &str) -> Result<Ofd, zip::result::ZipError> {
        let file = fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut zip = ZipArchive::new(reader).unwrap();
        let mut ofd = Ofd::default();
        let mut document = Document::default();

        {
            // find the OFD.xml file and parse the content to ofd object.
            let mut ofd_file = zip.by_name("OFD.xml").unwrap();
            let mut content = String::new();
            let _size = ofd_file.read_to_string(&mut content).unwrap();
            ofd = Ofd::from_xml(&content).unwrap();
            println!("ofd: {:#?}", ofd);
        }

        {
            // find the DocRoot file and parse the content to document object.
            let mut doc_file = zip.by_name(ofd.doc_body.doc_root.as_str()).unwrap();
            let mut content = String::new();
            let _size = doc_file.read_to_string(&mut content).unwrap();
            document = Document::from_xml(&content).unwrap();
            println!("document: {:#?}", document);
        }

        let pybox = CT_PageArea::from(document.common_data.page_area.physical_box.clone());

        let mut surface = cairo::ImageSurface::create(cairo::Format::ARgb32, pybox.width, pybox.height).unwrap();
        let mut context = cairo::Context::new(&surface).unwrap();

        document.render(&mut context);

        for i in 0..document.pages.page.len() {
            let page = &document.pages.page[i];
            // concat basename of document.doc_body.doc_root and page.base_loc
            let path = Path::new(ofd.doc_body.doc_root.as_str());
            println!("{}", path.parent().unwrap().join(page.base_loc.as_ref().unwrap().as_str()).to_str().unwrap());
            let mut page_file = zip.by_name(&path.parent().unwrap().join(page.base_loc.as_ref().unwrap().as_str()).to_str().unwrap()).unwrap();
            let mut content = String::new();
            let _size = page_file.read_to_string(&mut content).unwrap();
            let page = Page::from_xml(&content).unwrap();
            println!("page: {:#?}", page);

            page.render(&mut context);
        }

        let mut file = fs::File::create("out.png").unwrap();
        surface.write_to_png(&mut file).unwrap();

        Ok(ofd)
    }
}