use dioxus::prelude::*;
use smol_str::SmolStr;

const DIVIDER_BASE: &str = "shrink-0 bg-border/100";
const DIVIDER_HORIZONTAL: &str = "h-[1px] w-full";
const DIVIDER_VERTICAL: &str = "w-[1px] h-full";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl DividerOrientation {
    fn classes(&self) -> &'static str {
        match self {
            DividerOrientation::Horizontal => DIVIDER_HORIZONTAL,
            DividerOrientation::Vertical => DIVIDER_VERTICAL,
        }
    }
}

#[component]
pub fn Divider(
    #[props(default)] orientation: DividerOrientation,
    class: Option<SmolStr>,
) -> Element {
    let classes = format!(
        "{} {} {}",
        DIVIDER_BASE,
        orientation.classes(),
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            class: "{classes}",
            "data-slot": "divider",
            role: "separator",
            "aria-orientation": if matches!(orientation, DividerOrientation::Vertical) { "vertical" } else { "horizontal" }
        }
    }
}
