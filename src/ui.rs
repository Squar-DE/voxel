use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Scale, Separator, Video};
use libadwaita::{ToastOverlay, Clamp};

#[derive(Clone)]
pub struct VideoEditorUI {
    pub video_widget: Video,
    pub open_button: Button,
    pub play_button: Button,
    pub pause_button: Button,
    pub stop_button: Button,
    pub start_scale: Scale,
    pub end_scale: Scale,
    pub export_button: Button,
}

impl VideoEditorUI {
    pub fn new(toast_overlay: &ToastOverlay) -> Self {
        let main_box = Box::new(gtk4::Orientation::Vertical, 12);
        main_box.set_margin_top(12);
        main_box.set_margin_bottom(12);
        main_box.set_margin_start(12);
        main_box.set_margin_end(12);

        let clamp = Clamp::new();
        clamp.set_maximum_size(800);
        clamp.set_child(Some(&main_box));
        toast_overlay.set_child(Some(&clamp));

        // Video widget
        let video_widget = Video::new();
        video_widget.set_height_request(300);
        main_box.append(&video_widget);

        // Control buttons
        let button_box = Self::create_control_buttons();
        main_box.append(&button_box);

        main_box.append(&Separator::new(gtk4::Orientation::Horizontal));

        // Timeline controls
        let (timeline_box, start_scale, end_scale) = Self::create_timeline_controls();
        main_box.append(&timeline_box);

        main_box.append(&Separator::new(gtk4::Orientation::Horizontal));

        // Export section
        let export_box = Self::create_export_section();
        main_box.append(&export_box);

        // Extract buttons from button_box
        let open_button = button_box.first_child().unwrap().downcast::<Button>().unwrap();
        let play_button = open_button.next_sibling().unwrap().downcast::<Button>().unwrap();
        let pause_button = play_button.next_sibling().unwrap().downcast::<Button>().unwrap();
        let stop_button = pause_button.next_sibling().unwrap().downcast::<Button>().unwrap();

        // Extract export button
        let export_button = export_box.last_child().unwrap().downcast::<Button>().unwrap();

        Self {
            video_widget,
            open_button,
            play_button,
            pause_button,
            stop_button,
            start_scale,
            end_scale,
            export_button,
        }
    }

    fn create_control_buttons() -> Box {
        let button_box = Box::new(gtk4::Orientation::Horizontal, 6);
        button_box.set_halign(gtk4::Align::Center);
        button_box.add_css_class("linked");

        let open_button = Button::with_label("Open Video");
        let play_button = Button::with_label("Play");
        let pause_button = Button::with_label("Pause");
        let stop_button = Button::with_label("Stop");

        button_box.append(&open_button);
        button_box.append(&play_button);
        button_box.append(&pause_button);
        button_box.append(&stop_button);

        button_box
    }

    fn create_timeline_controls() -> (Box, Scale, Scale) {
        let timeline_box = Box::new(gtk4::Orientation::Vertical, 6);
        
        let timeline_label = Label::new(Some("Timeline"));
        timeline_label.add_css_class("heading");
        timeline_box.append(&timeline_label);

        let start_box = Box::new(gtk4::Orientation::Horizontal, 6);
        start_box.append(&Label::new(Some("Start:")));
        let start_scale = Scale::with_range(gtk4::Orientation::Horizontal, 0.0, 100.0, 1.0);
        start_scale.set_hexpand(true);
        start_box.append(&start_scale);

        let end_box = Box::new(gtk4::Orientation::Horizontal, 6);
        end_box.append(&Label::new(Some("End:")));
        let end_scale = Scale::with_range(gtk4::Orientation::Horizontal, 0.0, 100.0, 1.0);
        end_scale.set_hexpand(true);
        end_box.append(&end_scale);

        timeline_box.append(&start_box);
        timeline_box.append(&end_box);

        (timeline_box, start_scale, end_scale)
    }

    fn create_export_section() -> Box {
        let export_box = Box::new(gtk4::Orientation::Vertical, 6);
        let export_label = Label::new(Some("Export"));
        export_label.add_css_class("heading");
        export_box.append(&export_label);

        let export_button = Button::with_label("Export Trimmed Video");
        export_button.add_css_class("suggested-action");
        export_box.append(&export_button);

        export_box
    }
}
