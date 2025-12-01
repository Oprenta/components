use dioxus::prelude::*;
use oprenta_components::prelude::*;

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
