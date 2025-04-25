mod app;

use tracing_subscriber::{filter::Targets, prelude::*};
use tracing_web::MakeWebConsoleWriter;

use app::App;

fn main() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .without_time() // std::time is not available in browsers, see note below
        .with_writer(MakeWebConsoleWriter::new())
        .with_filter(
            Targets::new()
                .with_target("yew", tracing::Level::DEBUG) // yew trace is a menace
                .with_default(tracing::Level::TRACE),
        );
    let sub = tracing_subscriber::registry().with(fmt_layer);
    sub.init();
    yew::Renderer::<App>::new().render();
}
