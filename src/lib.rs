mod document;

use std::fs;
use std::io::BufReader;
use std::io::Read;

use zip::ZipArchive;

use document::{CommonData, PageArea, Document, Page};

pub struct Ofd {
    pub document: Document,
}

impl Ofd {
    pub fn from_filename(filename: &str) -> Result<Ofd, zip::result::ZipError> {
        let file = fs::File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut zip = ZipArchive::new(reader).unwrap();
        let mut ofd = Ofd {
            document: Document {
                common_data: CommonData {
                    page_area: PageArea {
                        physical_box: String::from("..."), // 填入实际值
                        application_box: String::from("..."), // 填入实际值
                    },
                    public_res: String::from("..."), // 填入实际值
                    document_res: String::from("..."), // 填入实际值
                    max_unit_id: 0, // 填入实际值
                },
                pages: Vec::new(), // 或者添加实际的 Page 对象
                annotations: None,
                custom_tags: None,
            },
        };
        for i in 0..zip.len() {
            let mut zip_file = zip.by_index(i).unwrap();
            println!("{}", zip_file.name());
            // if zip_file.is_file() {
            //     let name = zip_file.name().to_string();
            //     if name.ends_with(".xml") {
            //         ofd.document.xml = Some(String::from_utf8_lossy(&zip_file.bytes())?.to_string());
            //     } else if name.ends_with(".bin") {
            //         ofd.document.bin = Some(zip_file.bytes());
            //     }
            // }
        }
        Ok(ofd)
    }
}