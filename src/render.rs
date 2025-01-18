use cairo;

use crate::page::{Page, Event, PathObject, TextObject, Color};
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

impl Renderable for Page {
    fn render(&self, context: &mut cairo::Context) {
        println!("render page");
        for event in self.content.layer.events.iter() {
            match event {
                Event::PathObject(p) => {
                    p.render(context);
                }
                Event::TextObject(t) => {
                    t.render(context);
                }
                _ => {}
            }
        }
    }
}

impl Renderable for PathObject {
    fn render(&self, context: &mut cairo::Context) {
        context.save();

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

        context.restore();
    }
}

impl Renderable for TextObject {
    fn render(&self, context: &mut cairo::Context) {
        context.save();

        let boundary = CT_Box::from(self.boundary.clone());
        let color = self.fill_color.as_ref().unwrap_or(&Color::default()).value.clone();
        let fill_color = CT_Color::from(color);

        // TODO(hualet): load the right font
        context.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
        context.set_font_size(self.size);

        context.set_source_rgb(fill_color.value[0] as f64 / 255.0,
            fill_color.value[1] as f64 / 255.0,
            fill_color.value[2] as f64 / 255.0);
        context.move_to(boundary.x as f64, boundary.y as f64);
        context.show_text(self.text_code.value.as_str());


        context.restore();

    }
}