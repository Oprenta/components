use dioxus::prelude::*;

pub const COMPONENT: Asset = asset!("assets/tailwind.css");

pub fn card() -> Element {
    rsx! {
        h1 {
            class: "text-3xl font-bold underline",
            "Hello, World!"
        }
    }
}
