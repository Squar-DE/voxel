use gtk4::prelude::*;
use gtk4::Video;
use gstreamer::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::path::PathBuf;

#[derive(Clone)]
pub struct GStreamerBackend {
    video_widget: Video,
    pipeline: Rc<RefCell<Option<gstreamer::Pipeline>>>,
}

impl GStreamerBackend {
    pub fn new(video_widget: Video) -> Self {
        Self {
            video_widget,
            pipeline: Rc::new(RefCell::new(None)),
        }
    }

    pub fn load_video(&self, path: &PathBuf) -> anyhow::Result<f64> {
        let uri = format!("file://{}", path.display());
        
        let pipeline = gstreamer::Pipeline::new();
        let source = gstreamer::ElementFactory::make("playbin")
            .name("source")
            .build()?;
        source.set_property("uri", &uri);

        pipeline.add(&source)?;
        
        self.video_widget.set_media_stream(None::<&gtk4::MediaStream>);
        
        // Get duration
        pipeline.set_state(gstreamer::State::Paused)?;
        
        let mut duration = None;
        for _ in 0..100 {
            if let Some(d) = pipeline.query_duration::<gstreamer::ClockTime>() {
                duration = Some(d);
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        let duration_seconds = if let Some(d) = duration {
            d.seconds() as f64
        } else {
            0.0
        };

        *self.pipeline.borrow_mut() = Some(pipeline);
        Ok(duration_seconds)
    }

    pub fn play(&self) {
        if let Some(pipeline) = self.pipeline.borrow().as_ref() {
            let _ = pipeline.set_state(gstreamer::State::Playing);
        }
    }

    pub fn pause(&self) {
        if let Some(pipeline) = self.pipeline.borrow().as_ref() {
            let _ = pipeline.set_state(gstreamer::State::Paused);
        }
    }

    pub fn stop(&self) {
        if let Some(pipeline) = self.pipeline.borrow().as_ref() {
            let _ = pipeline.set_state(gstreamer::State::Ready);
        }
    }

    pub fn seek(&self, position: f64) -> anyhow::Result<()> {
        if let Some(pipeline) = self.pipeline.borrow().as_ref() {
            let position_ns = gstreamer::ClockTime::from_seconds(position as u64);
            pipeline.seek_simple(
                gstreamer::SeekFlags::FLUSH | gstreamer::SeekFlags::KEY_UNIT,
                position_ns,
            )?;
        }
        Ok(())
    }

    pub fn export_video(
        &self,
        input_path: &PathBuf,
        output_path: &PathBuf,
        start_time: f64,
        end_time: f64,
    ) -> anyhow::Result<()> {
        
        println!("Export placeholder:");
        println!("  Input: {}", input_path.display());
        println!("  Output: {}", output_path.display());
        println!("  Start: {}s", start_time);
        println!("  End: {}s", end_time);
        println!("  Duration: {}s", end_time - start_time);
        
        let pipeline = gstreamer::Pipeline::new();
        
        let filesrc = gstreamer::ElementFactory::make("filesrc").build()?;
        filesrc.set_property("location", input_path.to_str().unwrap());
        
        let decodebin = gstreamer::ElementFactory::make("decodebin").build()?;
        let videoconvert = gstreamer::ElementFactory::make("videoconvert").build()?;
        let encoder = gstreamer::ElementFactory::make("x264enc").build()?;
        let muxer = gstreamer::ElementFactory::make("mp4mux").build()?;
        let filesink = gstreamer::ElementFactory::make("filesink").build()?;
        filesink.set_property("location", output_path.to_str().unwrap());
        
        pipeline.add_many(&[&filesrc, &decodebin, &videoconvert, &encoder, &muxer, &filesink])?;
        
        // Link elements and handle seek events for trimming
        // This requires more complex GStreamer pipeline management
        
        Ok(())
    }
}
