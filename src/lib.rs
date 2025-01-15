pub mod ofd;
mod document;

use std::fs;
use std::io::Read;
use std::io::BufReader;

use zip::ZipArchive;

use ofd::Ofd;
use document::{CommonData, PageArea, Document, Page};

impl Ofd {
    pub fn from_filename(filename: &str) -> Result<Ofd, zip::result::ZipError> {
        let file = fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut zip = ZipArchive::new(reader).unwrap();
        let mut ofd = Ofd::default();
        for i in 0..zip.len() {
            let mut zip_file = zip.by_index(i).unwrap();
            println!("{}", zip_file.name());
            // find the OFD.xml file and parse the content to ofd object.
            if zip_file.name() == "OFD.xml" {
                let mut content = String::new();
                let _size = zip_file.read_to_string(&mut content).unwrap();
                println!("{}", content);
                ofd = Ofd::from_xml(&content).unwrap();
                println!("ofd: {:#?}", ofd);
            }
        }
        Ok(ofd)
    }
}