use dioxus::prelude::*;
use oprenta_components::{self as oc, COMPONENT_STYLES};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: COMPONENT_STYLES}

        h1 { "Hello, Dioxus Web!" }
        oc::card {}
    }
}
