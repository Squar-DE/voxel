use gtk4::prelude::*;
use gtk4::Application;
use crate::video_editor::VideoEditor;

pub struct VideoEditorApp;

impl VideoEditorApp {
    pub fn new(app: &Application) -> Self {
        let _editor = VideoEditor::new(app);
        Self
    }
}
