use std::io::Read;
use zip::ZipArchive;

#[derive(Debug, Default, Clone)]
pub struct OfdFile {
    pub xml: Option<String>,
    pub bin: Option<Vec<u8>>,
}
    
// pub fn parse_zip<R: Read>(reader: R) -> Result<OfdFile, zip::result::ZipError> {
//     let mut zip = ZipArchive::new(reader)?;
//     let mut file = OfdFile::new();
//     for i in 0..zip.len() {
//         let mut zip_file = zip.by_index(i)?;
//         if zip_file.is_file() {
//             let name = zip_file.name().to_string();
//             if name.ends_with(".xml") {
//                 file.xml = Some(String::from_utf8_lossy(&zip_file.bytes())?.to_string());
//             } else if name.ends_with(".bin") {
//                 file.bin = Some(zip_file.bytes());
//             }
//         }
//     }
//     Ok(file)
// }