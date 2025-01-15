use cairo;
use std::fs::File;

pub fn render() {
    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 200, 200).unwrap();
    let context = cairo::Context::new(&surface).unwrap();

    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint();

    context.set_source_rgb(0.0, 0.0, 0.0);
    context.select_font_face("Sans", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
    context.set_font_size(40.0);

    context.move_to(50.0, 50.0);
    context.show_text("Hello, world!");

    let mut file = File::create("out.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}
