pub mod ofd;
pub mod document;
pub mod render;
pub mod page;
pub mod types;

use std::fs;
use std::path::Path;
use std::io::Read;
use std::io::BufReader;

use ofd::Ofd;
use document::{Document, PageElement};
use page::Page;
use render::Renderable;
use types::CT_PageArea;