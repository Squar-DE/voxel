use gtk4::prelude::*;
use gtk4::{Application, Button};
use libadwaita::{ApplicationWindow as AdwApplicationWindow, ToastOverlay, Toast, HeaderBar};
use std::cell::RefCell;
use std::rc::Rc;
use std::path::PathBuf;
use crate::ui::VideoEditorUI;
use crate::gstreamer_backend::GStreamerBackend;
use libadwaita::prelude::*;

#[derive(Clone)]
pub struct VideoEditor {
    pub window: AdwApplicationWindow,
    ui: VideoEditorUI,
    backend: GStreamerBackend,
    current_file: Rc<RefCell<Option<PathBuf>>>,
    toast_overlay: ToastOverlay,
}

impl VideoEditor {
    pub fn new(app: &Application) -> Self {
        let window = AdwApplicationWindow::builder()
            .application(app)
            .title("Voxel Video Editor")
            .default_width(1000)
            .default_height(700)
            .build();


        let toast_overlay = ToastOverlay::new();
        window.set_content(Some(&toast_overlay));

        let ui = VideoEditorUI::new(&toast_overlay);
        let backend = GStreamerBackend::new(ui.video_widget.clone());

        let editor = Self {
            window,
            ui: ui.clone(),
            backend,
            current_file: Rc::new(RefCell::new(None)),
            toast_overlay,
        };

        editor.connect_signals();
        editor.window.present();
        editor
    }

    fn connect_signals(&self) {
        // Open file
        let editor_clone = self.clone();
        self.ui.open_button.connect_clicked(move |_| {
            editor_clone.open_file_dialog();
        });

        // Playback controls
        let backend = self.backend.clone();
        self.ui.play_button.connect_clicked(move |_| {
            backend.play();
        });

        let backend = self.backend.clone();
        self.ui.pause_button.connect_clicked(move |_| {
            backend.pause();
        });

        let backend = self.backend.clone();
        self.ui.stop_button.connect_clicked(move |_| {
            backend.stop();
        });

        // Export
        let editor_clone = self.clone();
        self.ui.export_button.connect_clicked(move |_| {
            editor_clone.export_video();
        });
    }

    fn open_file_dialog(&self) {
        use gtk4::{FileChooserDialog, ResponseType};

        let dialog = FileChooserDialog::new(
            Some("Open Video File"),
            Some(&self.window),
            gtk4::FileChooserAction::Open,
            &[("Cancel", ResponseType::Cancel), ("Open", ResponseType::Accept)],
        );

        let filter = gtk4::FileFilter::new();
        filter.set_name(Some("Video files"));
        filter.add_mime_type("video/*");
        dialog.add_filter(&filter);

        let editor_clone = self.clone();
        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        editor_clone.load_video(path);
                    }
                }
            }
            dialog.close();
        });

        dialog.present();
    }

    fn load_video(&self, path: PathBuf) {
        match self.backend.load_video(&path) {
            Ok(_) => {
                *self.current_file.borrow_mut() = Some(path);
                self.show_toast("Video loaded successfully");
            }
            Err(e) => {
                self.show_toast(&format!("Error loading video: {}", e));
            }
        }
    }

    fn export_video(&self) {
        if self.current_file.borrow().is_none() {
            self.show_toast("No video loaded");
            return;
        }

        use gtk4::{FileChooserDialog, ResponseType};

        let dialog = FileChooserDialog::new(
            Some("Export Video"),
            Some(&self.window),
            gtk4::FileChooserAction::Save,
            &[("Cancel", ResponseType::Cancel), ("Save", ResponseType::Accept)],
        );

        dialog.set_current_name("edited_video.mp4");

        let editor_clone = self.clone();
        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(output_path) = file.path() {
                        editor_clone.perform_export(output_path);
                    }
                }
            }
            dialog.close();
        });

        dialog.present();
    }

    fn perform_export(&self, output_path: PathBuf) {
        let input_path = match self.current_file.borrow().as_ref() {
            Some(path) => path.clone(),
            None => return,
        };

        let start_time = 0.0;
        let end_time = 60.0;

        match self.backend.export_video(&input_path, &output_path, start_time, end_time) {
            Ok(_) => {
                self.show_toast("Video exported successfully");
            }
            Err(e) => {
                self.show_toast(&format!("Export failed: {}", e));
            }
        }
    }

    fn show_toast(&self, message: &str) {
        let toast = Toast::new(message);
        self.toast_overlay.add_toast(toast);
    }
}
