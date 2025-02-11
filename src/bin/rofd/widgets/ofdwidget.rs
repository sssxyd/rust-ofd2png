use gtk::{gdk, glib, subclass::prelude::*};

// OFDWidget wrapper setup.
glib::wrapper! {
    pub struct OFDWidget(ObjectSubclass<imp::OFDWidget>) @implements gdk::Paintable;
}

impl OFDWidget {
    pub fn zoom_in(&self) {
        let imp = self.imp();
        imp.zoom(0.25);
    }

    pub fn zoom_out(&self) {
        let imp = self.imp();
        imp.zoom(-0.25);
    }

}

impl Default for OFDWidget {
    fn default() -> Self {
        glib::Object::new()
    }

}

// Implementation.
mod imp {
    use rofd::*;
    use std::cell::RefCell;

    use log::debug;
    use gtk::{gdk, glib, graphene, gsk, prelude::*, subclass::prelude::*};

    pub struct OFDWidget {
        pub(super) scale: RefCell<f64>,
    }

    impl OFDWidget {
        pub(super) fn zoom(&self, delta: f64) {
            debug!("zooming by {}", delta);
            *self.scale.borrow_mut() += delta;
        }
    }

    impl Default for OFDWidget {
        fn default() -> Self {
            Self {
                scale: RefCell::new(1.0),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for OFDWidget {
        const NAME: &'static str = "OFDWidget";
        type Type = super::OFDWidget;
        type Interfaces = (gdk::Paintable,);
    }

    impl ObjectImpl for OFDWidget {}

    impl PaintableImpl for OFDWidget {
        fn snapshot(&self, snapshot: &gdk::Snapshot, width: f64, height: f64) {
            let bounds = graphene::Rect::new(
                0.,
                0.,
                width as f32,
                height as f32);

            let path_builder = gsk::PathBuilder::new();
            path_builder.add_rect(&bounds);

            let fill_path = path_builder.to_path();
            snapshot.push_fill(&fill_path, gsk::FillRule::Winding);

            let mut context = snapshot.append_cairo(&bounds);
            context.scale(*self.scale.borrow(), *self.scale.borrow());

            let mut ofd_node = read_ofd("learning/test.ofd").unwrap();
            render_ofd_to_context(&mut ofd_node, &mut context).unwrap();

            snapshot.pop();
        }
    }

}