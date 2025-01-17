use cairo;

use crate::page::{Page, Event};
use crate::document::Document;

pub trait Renderable {
    fn render(&self, context: &mut cairo::Context);
}

impl Renderable for Document {
    fn render(&self, context: &mut cairo::Context) {
        println!("render document");
        // self.pages.page.iter().for_each(|p| p.render(context));
    }
}

impl Renderable for Page {
    fn render(&self, context: &mut cairo::Context) {
        println!("render page");
        // for event in self.content.layer.events.iter() {
            // match event {
            //     Event::PathObject(p) => {
            //         context.move_to(p.x1, p.y1);
            //         context.line_to(p.x2, p.y2);
            //     }
            //     Event::TextObject(t) => {
            //         context.move_to(t.x, t.y);
            //         context.show_text(t.text.as_str());
            //     }
            // }
        // }
    }
}


// pub fn render() {
//     let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 200, 200).unwrap();
//     let context = cairo::Context::new(&surface).unwrap();

//     context.set_source_rgb(1.0, 1.0, 1.0);
//     context.paint();

//     context.set_source_rgb(0.0, 0.0, 0.0);
//     context.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
//     context.set_font_size(40.0);

//     context.move_to(50.0, 50.0);
//     context.show_text("Hello, world!");

//     let mut file = File::create("out.png").unwrap();
//     surface.write_to_png(&mut file).unwrap();
// }
