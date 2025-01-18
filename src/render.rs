use cairo;

use crate::page::{Page, Event, PathObject};
use crate::document::Document;

use crate::types::CT_Box;
use crate::types::CT_Color;

pub trait Renderable {
    fn render(&self, context: &mut cairo::Context);
}

impl Renderable for Document {
    fn render(&self, context: &mut cairo::Context) {
        println!("render document");
        // self.pages.page.iter().for_each(|p| p.render(context));
    }
}

impl Renderable for PathObject {
    fn render(&self, context: &mut cairo::Context) {
        let boundary = CT_Box::from(self.boundary.clone());
        let color = CT_Color::from(self.stroke_color.as_ref().unwrap().value.clone());

        context.set_source_rgb(color.value[0] as f64 / 255.0,
            color.value[1] as f64 / 255.0,
            color.value[2] as f64 / 255.0);
        context.set_line_width(self.line_width);

        context.move_to(boundary.x as f64, boundary.y as f64);
        context.line_to((boundary.x + boundary.width) as f64, boundary.y as f64);
        context.line_to((boundary.x + boundary.width) as f64, (boundary.y + boundary.height) as f64);
        context.line_to(boundary.x as f64, (boundary.y + boundary.height) as f64);
        context.line_to(boundary.x as f64, boundary.y as f64);

        context.stroke();
    }
}

impl Renderable for Page {
    fn render(&self, context: &mut cairo::Context) {
        println!("render page");
        for event in self.content.layer.events.iter() {
            match event {
                Event::PathObject(p) => {
                    p.render(context);
                }
                _ => {}
            }
        }
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
