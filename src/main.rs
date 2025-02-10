use env_logger;

use gtk::{gdk, glib, prelude::*};

use log::debug;

mod widgets {
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
}

fn main() -> glib::ExitCode {
    env_logger::init();

    let application = gtk::Application::builder()
        .application_id("org.hualet.rofd")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    // window
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("OFD Reader"));
    window.set_default_size(400, 300);

    // header_bar
    let header_bar = gtk::HeaderBar::new();
    window.set_titlebar(Some(&header_bar));

    let zoom_in_button = gtk::Button::new();
    zoom_in_button.set_icon_name("zoom-in-symbolic");
    let zoom_out_button = gtk::Button::new();
    zoom_out_button.set_icon_name("zoom-out-symbolic");

    let zoom_combo = gtk::ComboBoxText::builder()
        .build();
    zoom_combo.append_text("25%");
    zoom_combo.append_text("50%");
    zoom_combo.append_text("75%");
    zoom_combo.append_text("100%");
    zoom_combo.append_text("125%");
    zoom_combo.append_text("150%");
    zoom_combo.append_text("175%");
    zoom_combo.append_text("200%");
    zoom_combo.set_active(Some(3));

    header_bar.pack_end(&zoom_in_button);
    header_bar.pack_end(&zoom_combo);
    header_bar.pack_end(&zoom_out_button);

    // ofd widget
    let picture = gtk::Picture::builder()
        .content_fit(gtk::ContentFit::ScaleDown)
        .build();
    let paintable = widgets::OFDWidget::default();
    picture.set_paintable(Some(&paintable));
    picture.set_focusable(true);


    let ev_ctrl = gtk::EventControllerKey::new();
    ev_ctrl.connect_key_released(move |_ctrl, key, _keycode, modifier| {
        debug!("key: {}, modifier: {:?}", key, modifier);
        match (key, modifier) {
            (gdk::Key::equal, gdk::ModifierType::CONTROL_MASK) => {
                zoom_in();
            },
            (gdk::Key::minus, gdk::ModifierType::CONTROL_MASK) => {
                zoom_out();
            },
            _ => ()
        }
    });
    picture.add_controller(ev_ctrl);

    window.set_child(Some(&picture));
    window.present();
}

fn zoom_in() {
    debug!("zoom in");
}

fn zoom_out() {
    debug!("zoom out");
}