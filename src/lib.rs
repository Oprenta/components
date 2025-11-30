use dioxus::prelude::*;

pub const COMPONENT_STYLES: Asset = asset!("assets/tailwind.css");

#[component]
pub fn card() -> Element {
    rsx! {
        h1 {
            class: "text-3xl font-bold underline",
            "Hello, World!"
        }
    }
}
