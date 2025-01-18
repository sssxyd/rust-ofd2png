

pub struct CT_PageArea {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

// Boundary="10.30 30.30 0.30 22"
#[derive(Debug)]
pub struct CT_Box {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug)]
pub struct CT_Color {
    pub value: ST_Array,
    pub alpha: i32,
}

// ST_Array is actually Vector<i32>
type ST_Array = Vec<i32>;

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

// implement string to CT_Box
impl From<String> for CT_Box {
    fn from(value: String) -> Self {
        let parts: Vec<f32> = value.split_whitespace().map(|s| s.parse().unwrap()).collect();
        CT_Box {
            x: parts[0],
            y: parts[1],
            width: parts[2],
            height: parts[3],
        }
    }
}

// implement string to CT_Color
impl From<String> for CT_Color {
    fn from(value: String) -> Self {
        let parts: Vec<i32> = value.split_whitespace().map(|s| s.parse().unwrap()).collect();
        CT_Color {
            value: parts.clone(),
            alpha: 255,
        }
    }
}