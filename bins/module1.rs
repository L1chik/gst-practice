use gstreamer as gst;
use gstreamer::prelude::*;
use gstreamer::MessageView;


fn main() {
    gst::init().unwrap();

    //Building pipeline
    let uri = "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm";
    let pipeline = gst::parse_launch(&format!("playbin uri={}", uri)).unwrap();

    //Playing
    pipeline.set_state(gst::State::Playing)
        .expect("Can't change bin state");

    //Wait until error or EOS
    let bus = pipeline.bus().unwrap();

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );

                break;
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the Null state");
}