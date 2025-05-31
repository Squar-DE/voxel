mod app;
mod video_editor;
mod ui;
mod gstreamer_backend;

use gtk4::prelude::*;
use gtk4::{glib, Application};
use app::VideoEditorApp;

const APP_ID: &str = "org.squarDE.Voxel";

fn main() -> glib::ExitCode {
    // Initialize GStreamer
    gstreamer::init().expect("Failed to initialize GStreamer");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(|app| {
        libadwaita::init().expect("Failed to initialize libadwaita");
        VideoEditorApp::new(app);
    });

    app.run()
}
