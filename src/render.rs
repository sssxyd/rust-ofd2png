use std::path::Path;
use std::io::Cursor;
use std::io::Read;

use cairo;
use cairo::Error;

use crate::ofd::Ofd;
use crate::document::Document;
use crate::page::{Page, Event, PathObject, TextObject, ImageObject,
    PageBlock, Color};

use crate::types::{mmtopx, ct};

pub trait Renderable {
    fn render(&self, context: &mut cairo::Context,
        ofd: &mut Ofd, document: &Document) -> Result<(), Error>;
}

impl Renderable for Document {
    fn render(&self, _context: &mut cairo::Context,
        _ofd: &mut Ofd, _document: &Document) -> Result<(), Error> {
        println!("render document");
        Ok(())
        // self.pages.page.iter().for_each(|p| p.render(context));
    }
}

impl Renderable for Page {
    fn render(&self, context: &mut cairo::Context,
        ofd: &mut Ofd, document: &Document) -> Result<(), Error> {
        println!("render page");
        _render_page_block(self.content.layer.events.clone(),
            context, ofd, document)
    }
}

impl Renderable for PathObject {
    fn render(&self, context: &mut cairo::Context,
        _ofd: &mut Ofd, _document: &Document) -> Result<(), Error> {
        context.save()?;

        let boundary = ct::Box::from(self.boundary.clone()).to_pixel();
        let color = ct::Color::from(
            self.stroke_color.as_ref().unwrap().value.clone());

        context.set_source_rgb(color.value[0] as f64 / 255.0,
            color.value[1] as f64 / 255.0,
            color.value[2] as f64 / 255.0);
        context.set_line_width(mmtopx(self.line_width));

        context.move_to(boundary.x as f64, boundary.y as f64);
        context.line_to((boundary.x + boundary.width) as f64,
            boundary.y as f64);
        context.line_to((boundary.x + boundary.width) as f64,
            (boundary.y + boundary.height) as f64);
        context.line_to(boundary.x as f64,
            (boundary.y + boundary.height) as f64);
        context.line_to(boundary.x as f64, boundary.y as f64);

        context.stroke()?;

        context.restore()
    }
}

impl Renderable for TextObject {
    fn render(&self, context: &mut cairo::Context,
        _ofd: &mut Ofd, document: &Document) -> Result<(), Error> {
        context.save()?;

        let boundary = ct::Box::from(self.boundary.clone()).to_pixel();
        let color = self.fill_color.as_ref().unwrap_or(&Color::default()).value.clone();
        let fill_color = ct::Color::from(color);

        let font_id = self.font;
        for font in document.public_res.fonts.font.iter() {
            if font.id == font_id {
                // TODO(hualet): custom font file loading.
                context.select_font_face(font.family_name.as_str(),
                    cairo::FontSlant::Normal, cairo::FontWeight::Normal);
                break;
            }
        }
        context.set_font_size(mmtopx(self.size) as f64);

        context.set_source_rgb(fill_color.value[0] as f64 / 255.0,
            fill_color.value[1] as f64 / 255.0,
            fill_color.value[2] as f64 / 255.0);
        context.move_to(boundary.x as f64, boundary.y as f64);
        context.show_text(self.text_code.value.as_str())?;


        context.restore()

    }
}

// implement Renderable for ImageObject
impl Renderable for ImageObject {
    fn render(&self, context: &mut cairo::Context,
        ofd: &mut Ofd, document: &Document) -> Result<(), Error> {
        context.save()?;

        let boundary = ct::Box::from(self.boundary.clone()).to_pixel();

        // find the image file:
        // 1) find the resource file in DocumentRes with the resource id
        // 2) construct the path of the image file
        // 3) load the image file and draw
        for resource in document.doc_res.multi_medias.multi_media.iter() {
            if resource.id == self.resource_id {
                let path = Path::new(ofd.node.doc_body.doc_root.as_str());
                let res_path = &path.parent().unwrap()
                    .join(document.doc_res.base_loc.as_str())
                    .join(resource.media_file.as_str());

                let mut file = ofd.zip_archive.by_name(res_path.to_str().unwrap()).unwrap();
                let mut content = Vec::new();
                let _size = file.read_to_end(&mut content).unwrap();

                let mut file_reader = Cursor::new(content);
                // FIXME(hualet): png is not for sure.
                let surface = cairo::ImageSurface::create_from_png(&mut file_reader).unwrap();
                context.scale(boundary.width/ surface.width() as f64,
                    boundary.height/ surface.height() as f64);
                context.set_source_surface(&surface,
                    boundary.x as f64,
                    boundary.y as f64)?;
                context.paint()?;
            }
        }


        context.restore()
    }
}

impl Renderable for PageBlock {
    fn render(&self, context: &mut cairo::Context,
        ofd: &mut Ofd, document: &Document) -> Result<(), Error> {
        println!("render pageblock");
        _render_page_block(self.events.clone(), context, ofd, document)
    }
}


fn _render_page_block(events: Vec<Event>, context: &mut cairo::Context,
    ofd: &mut Ofd, document: &Document) -> Result<(), Error> {
    for event in events.iter() {
        match event {
            Event::PathObject(p) => {
                println!("render pathobject");
                return p.render(context, ofd, document)
            }
            Event::TextObject(t) => {
                println!("render textobject");
                return t.render(context, ofd, document)
            }
            Event::ImageObject(i) => {
                println!("render imageobject");
                return i.render(context, ofd, document)
            }
            Event::PageBlock(p) => {
                println!("render pageblock");
                return p.render(context, ofd, document)
            }
        }
    }

    Ok(())
}