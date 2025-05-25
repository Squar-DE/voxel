use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Paned, ScrolledWindow, Separator, Video};
use libadwaita::{prelude::*, ToastOverlay, Clamp, ViewStack, ActionRow, PreferencesGroup, PreferencesPage};

#[derive(Clone)]
pub struct VideoEditorUI {
    pub video_widget: Video,
    pub open_button: Button,
    pub play_button: Button,
    pub pause_button: Button,
    pub stop_button: Button,
    pub export_button: Button,
    pub timeline: Box,
}

impl VideoEditorUI {
    pub fn new(toast_overlay: &ToastOverlay) -> Self {
        // Create main paned container (sidebar + content)
        let paned = Paned::new(gtk4::Orientation::Horizontal);
        
        // Create sidebar with view stack
        let sidebar = Self::create_sidebar();
        paned.set_start_child(Some(&sidebar));
        
        // Create main content area
        let main_content = Self::create_main_content();
        paned.set_end_child(Some(&main_content));
        
        // Wrap in clamp and add to toast overlay
        let clamp = Clamp::new();
        clamp.set_maximum_size(1200);
        clamp.set_child(Some(&paned));
        toast_overlay.set_child(Some(&clamp));
        
        // Extract important widgets
        let video_widget = main_content.first_child().unwrap().downcast::<Video>().unwrap();
        let button_box = video_widget.next_sibling().unwrap().downcast::<Box>().unwrap();
        let timeline = button_box.next_sibling().unwrap().downcast::<Box>().unwrap();
        
        let open_button = button_box.first_child().unwrap().downcast::<Button>().unwrap();
        let play_button = open_button.next_sibling().unwrap().downcast::<Button>().unwrap();
        let pause_button = play_button.next_sibling().unwrap().downcast::<Button>().unwrap();
        let stop_button = pause_button.next_sibling().unwrap().downcast::<Button>().unwrap();
        
        let export_button = timeline.last_child().unwrap().downcast::<Button>().unwrap();

        Self {
            video_widget,
            open_button,
            play_button,
            pause_button,
            stop_button,
            export_button,
            timeline,
        }
    }

    fn create_sidebar() -> ScrolledWindow {
        let scroll = ScrolledWindow::new();
        scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        scroll.set_size_request(220, -1);
        
        let stack = ViewStack::new();
        
        // Project page
        let project_page = PreferencesPage::new();
        project_page.set_title("Project");
        
        let media_group = PreferencesGroup::new();
        media_group.set_title("Media");
        media_group.add(&ActionRow::builder().title("Import Media").build());
        project_page.add(&media_group);
        
        // Add project page to stack
        let project_stack_page = stack.add_titled(
            &project_page,
            Some("project"),
            "Project"
        );
        project_stack_page.set_icon_name(Some("folder-symbolic"));
        
        // Export page
        let export_page = PreferencesPage::new();
        export_page.set_title("Export Settings");
        
        let format_group = PreferencesGroup::new();
        format_group.set_title("Format");
        format_group.add(&ActionRow::builder().title("MP4").build());
        format_group.add(&ActionRow::builder().title("WebM").build());
        export_page.add(&format_group);
        
        // Add export page to stack
        let export_stack_page = stack.add_titled(
            &export_page,
            Some("export"),
            "Export"
        );
        export_stack_page.set_icon_name(Some("document-export-symbolic"));
        
        scroll.set_child(Some(&stack));
        scroll
    }

    fn create_main_content() -> Box {
        let main_box = Box::new(gtk4::Orientation::Vertical, 12);
        main_box.set_margin_top(12);
        main_box.set_margin_bottom(12);
        main_box.set_margin_start(12);
        main_box.set_margin_end(12);
        
        // Video widget
        let video_widget = Video::new();
        video_widget.set_hexpand(true);
        video_widget.set_vexpand(true);
        video_widget.set_size_request(-1, 400);
        main_box.append(&video_widget);
        
        // Control buttons
        let button_box = Box::new(gtk4::Orientation::Horizontal, 6);
        button_box.set_halign(gtk4::Align::Center);
        
        let open_button = Button::builder()
            .label("Open")
            .icon_name("document-open-symbolic")
            .build();
        
        let play_button = Button::builder()
            .label("Play")
            .icon_name("media-playback-start-symbolic")
            .build();
        
        let pause_button = Button::builder()
            .label("Pause")
            .icon_name("media-playback-pause-symbolic")
            .build();
        
        let stop_button = Button::builder()
            .label("Stop")
            .icon_name("media-playback-stop-symbolic")
            .build();
        
        button_box.append(&open_button);
        button_box.append(&play_button);
        button_box.append(&pause_button);
        button_box.append(&stop_button);
        main_box.append(&button_box);
        
        // Timeline placeholder
        let timeline = Box::new(gtk4::Orientation::Vertical, 6);
        timeline.set_size_request(-1, 120);
        timeline.add_css_class("timeline");
        
        // Empty track
        let track = Box::new(gtk4::Orientation::Horizontal, 0);
        track.set_hexpand(true);
        track.set_size_request(-1, 40);
        track.add_css_class("track");
        timeline.append(&track);
        
        // Export button
        let export_button = Button::builder()
            .label("Export Video")
            .icon_name("document-save-symbolic")
            .css_classes(["suggested-action"])
            .margin_top(12)
            .build();
        timeline.append(&export_button);
        
        main_box.append(&timeline);
        main_box
    }
}
