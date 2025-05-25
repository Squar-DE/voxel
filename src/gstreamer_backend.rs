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
            d.seconds() as f64 // idfk
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
        let pipeline_str = format!(
            "filesrc location={} ! decodebin name=dec ! queue ! videoconvert ! \
            videotrim start={} stop={} ! queue ! x264enc ! mp4mux ! filesink location={}",
            input_path.display(),
            start_time * 1_000_000_000.0, // Convert to nanoseconds
            end_time * 1_000_000_000.0,
            output_path.display()
        );

        let pipeline = gstreamer::parse_launch(&pipeline_str)?;
        let pipeline = pipeline.downcast::<gstreamer::Pipeline>().unwrap();
        
        pipeline.set_state(gstreamer::State::Playing)?;
        
        // Wait for export to complete
        let bus = pipeline.bus().unwrap();
        while let Some(msg) = bus.timed_pop(gstreamer::ClockTime::NONE) {
            use gstreamer::MessageView;
            
            match msg.view() {
                MessageView::Eos(..) => break,
                MessageView::Error(err) => {
                    return Err(anyhow::anyhow!(
                        "Error from {}: {} ({})",
                        msg.src().map(|s| s.path_string()).unwrap_or_default(),
                        err.error(),
                        err.debug().unwrap_or_default()
                    ));
                }
                _ => (),
            }
        }
        
        pipeline.set_state(gstreamer::State::Null)?;
        Ok(())
    }
}
