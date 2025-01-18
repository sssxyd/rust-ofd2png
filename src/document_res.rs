use serde::Deserialize;

/*
<?xml version="1.0" encoding="UTF-8"?><ofd:Res xmlns:ofd="http://www.ofdspec.org/2016" BaseLoc="Res">
  <ofd:MultiMedias>
    <ofd:MultiMedia Format="PNG" ID="36" Type="Image">
      <ofd:MediaFile>b7cdce6106634283a4c851ae1e3347717795273356109011760_ewm.png</ofd:MediaFile>
    </ofd:MultiMedia>
    <ofd:MultiMedia Format="PNG" ID="174" Type="Image">
      <ofd:MediaFile>signature.png</ofd:MediaFile>
    </ofd:MultiMedia>
  </ofd:MultiMedias>
</ofd:Res>
*/

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Res {
    pub base_loc: String,
    pub multi_medias: MultiMedias,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MultiMedias {
    pub multi_media: Vec<MultiMedia>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MultiMedia {
    #[serde(rename = "ID")]
    pub id: String,
    pub format: String,
    pub media_file: String,
}

impl Res {
    pub fn from_xml(xml: &str) -> Result<Res, serde_xml_rs::Error> {
        serde_xml_rs::from_str(xml)
    }
}
