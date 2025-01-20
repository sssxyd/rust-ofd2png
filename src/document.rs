use serde::{Deserialize};

use crate::document_res::Res;

/*
<?xml version="1.0" encoding="UTF-8"?><ofd:Document xmlns:ofd="http://www.ofdspec.org/2016">
  <ofd:CommonData>
    <ofd:PageArea>
      <ofd:PhysicalBox>0 0 210 297</ofd:PhysicalBox>
      <ofd:ApplicationBox>0 0 210 297</ofd:ApplicationBox>
    </ofd:PageArea>
    <ofd:PublicRes>PublicRes.xml</ofd:PublicRes>
    <ofd:DocumentRes>DocumentRes.xml</ofd:DocumentRes>
    <ofd:MaxUnitID>176</ofd:MaxUnitID>
  </ofd:CommonData>
  <ofd:Pages>
    <ofd:Page BaseLoc="Pages/Page_0/Content.xml" ID="1"/>
  </ofd:Pages>
  <ofd:Annotations>Annots/Annotations.xml</ofd:Annotations>
  <ofd:CustomTags>Tags/CustomTags.xml</ofd:CustomTags>
</ofd:Document>
*/

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Document {
    pub common_data: CommonData,
    pub pages: PageList,
    pub annotations: Option<String>,
    pub custom_tags: Option<String>,

    #[serde(skip)]
    pub res: Res,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CommonData {
    pub page_area: PageArea,
    pub public_res: String,
    pub document_res: String,
    #[serde(rename = "MaxUnitID")]
    pub max_unit_id: u32,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PageArea {
    pub physical_box: String,
    pub application_box: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PageList {
    pub page: Vec<PageElement>, // TODO(hualet): don't know why, same in ofd.rs CustomData:custom_data.
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PageElement {
    #[serde(rename = "BaseLoc")]
    pub base_loc: Option<String>, // TODO(hualet): don't know why the field should be Option.
    #[serde(rename = "ID")]
    pub id: Option<u32>,
}

impl Document {
    pub fn from_xml(xml: &str) -> Result<Document, serde_xml_rs::Error> {
        serde_xml_rs::from_str(xml)
    }
}
