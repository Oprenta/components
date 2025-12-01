use dioxus::prelude::*;
use smol_str::SmolStr;

const LABEL_CSS: &str = "flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50";

#[component]
pub fn Label(
    children: Element,
    #[props(into)] r#for: Option<SmolStr>,
    class: Option<SmolStr>,
) -> Element {
    let classes = format!("{} {}", LABEL_CSS, class.as_deref().unwrap_or(""));

    rsx! {
        label {
            class: "{classes}",
            r#for: r#for.as_deref(),
            "data-slot": "label",
            {children}
        }
    }
}
