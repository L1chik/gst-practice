use gstreamer as gst;
use gstreamer::MessageView;
use gstreamer::prelude::*;

fn main() {
    gst::init().unwrap();

    let uri =
        "https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm";

    let source = gst::ElementFactory::make("videotestsrc", Some("source"))
        .unwrap();
    let filter = gst::ElementFactory::make("vertigotv", Some("filter"))
        .expect("No such element");
    let converter = gst::ElementFactory::make("videoconvert", Some("converter"))
        .unwrap();
    let sink = gst::ElementFactory::make("autovideosink", Some("sink"))
        .unwrap();

    let pipeline = gst::Pipeline::new(Some("test-pipe"));

    pipeline.add_many(&[&source, &filter, &converter, &sink]).unwrap();

    source.link(&filter).expect("cant add filter");
    filter.link(&converter).expect("Elements could not be linked");
    converter.link(&sink).expect("Elements could not be linked");

    source.set_property_from_str("pattern", "smpte");

    // Start playing
    pipeline.set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to the `Playing` state");

    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {

        match msg.view() {
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                break;
            }
            MessageView::Eos(..) => break,
            _ => (),
        }
    }

    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}