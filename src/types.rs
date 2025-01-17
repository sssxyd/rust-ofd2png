

pub struct CT_PageArea {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

// implement string to CT_PageArea
impl From<String> for CT_PageArea {
    fn from(value: String) -> Self {
        let parts: Vec<i32> = value.split_whitespace().map(|s| s.parse().unwrap()).collect();
        CT_PageArea {
            x: parts[0],
            y: parts[1],
            width: parts[2],
            height: parts[3],
        }
    }
}