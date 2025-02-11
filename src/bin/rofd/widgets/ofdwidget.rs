use rofd::*;

use gtk::{gdk, glib, graphene, gsk, prelude::*, subclass::prelude::*};

glib::wrapper! {
    pub struct OFDWidget(ObjectSubclass<OFDWidgetImpl>) @implements gdk::Paintable;
}

impl Default for OFDWidget {
    fn default() -> Self {
        glib::Object::new()
    }
}


pub struct OFDWidgetImpl;

impl Default for OFDWidgetImpl {
    fn default() -> Self {
        Self {
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for OFDWidgetImpl {
    const NAME: &'static str = "OFDWidget";
    type Type = OFDWidget;
    type Interfaces = (gdk::Paintable,);
}

impl ObjectImpl for OFDWidgetImpl {}

impl PaintableImpl for OFDWidgetImpl {
    fn snapshot(&self, snapshot: &gdk::Snapshot, width: f64, height: f64) {
        let bounds = graphene::Rect::new(0., 0., width as f32, height as f32);
        let fill_color = gdk::RGBA::RED;

        let path_builder = gsk::PathBuilder::new();
        path_builder.add_rect(&bounds);
        let fill_path = path_builder.to_path();

        snapshot.push_fill(&fill_path, gsk::FillRule::Winding);


        let mut context = snapshot.append_cairo(&bounds);
        let mut ofd_node = read_ofd("learning/test.ofd").unwrap();
        render_ofd_to_context(&mut ofd_node, &mut context).unwrap();


        snapshot.pop();
    }
}