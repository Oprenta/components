use dioxus::prelude::*;
use smol_str::SmolStr;

const INPUT_CSS: &str = "h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring/20 disabled:opacity-50";

#[component]
pub fn Input(
    #[props(into, default = SmolStr::new_static("text"))] r#type: SmolStr,
    #[props(into, default)] value: SmolStr,
    #[props(into, default)] placeholder: SmolStr,
    #[props(into)] id: Option<SmolStr>,
    #[props(default)] disabled: bool,
    #[props(default)] required: bool,
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
    #[props(into)] class: Option<SmolStr>,
) -> Element {
    let classes = format!("{} {}", INPUT_CSS, class.as_deref().unwrap_or(""));

    rsx! {
        input {
            r#type: r#type.as_str(),
            id: id.as_deref(),
            class: "{classes}",
            value: value.as_str(),
            placeholder: placeholder.as_str(),
            disabled: disabled,
            required: required,
            oninput: move |evt| {
                if let Some(handler) = &oninput {
                    handler.call(evt);
                }
            },
            onchange: move |evt| {
                if let Some(handler) = &onchange {
                    handler.call(evt);
                }
            },
            "data-slot": "input",
        }
    }
}
