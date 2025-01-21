

pub struct CT_PageArea {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

// Boundary="10.30 30.30 0.30 22"
#[derive(Debug)]
pub struct CT_Box {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug)]
pub struct CT_Color {
    pub value: ST_Array,
    pub alpha: i32,
}

// ST_Array is actually Vector<i32>
type ST_Array = Vec<i32>;

pub fn mmtopx(mm: f64) -> f64 {
    // TODO(hualet): use actual dpi?
    let dpi = 97.0;
    return mm * dpi / 25.4;
}

// implement string to CT_PageArea
impl From<String> for CT_PageArea {
    fn from(value: String) -> Self {
        let parts: Vec<f64> = value.split_whitespace().map(|s| s.parse().unwrap()).collect();
        CT_PageArea {
            x: parts[0],
            y: parts[1],
            width: parts[2],
            height: parts[3],
        }
    }
}

impl CT_PageArea {
    pub fn toPixel(&self) -> CT_PageArea {
        CT_PageArea {
            x: mmtopx(self.x),
            y: mmtopx(self.y),
            width: mmtopx(self.width),
            height: mmtopx(self.height),
        }
    }
}

// implement string to CT_Box
impl From<String> for CT_Box {
    fn from(value: String) -> Self {
        let parts: Vec<f64> = value.split_whitespace().map(|s| s.parse().unwrap()).collect();
        CT_Box {
            x: parts[0],
            y: parts[1],
            width: parts[2],
            height: parts[3],
        }
    }
}


impl CT_Box {
    pub fn toPixel(&self) -> CT_Box {
        CT_Box {
            x: mmtopx(self.x),
            y: mmtopx(self.y),
            width: mmtopx(self.width),
            height: mmtopx(self.height),
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