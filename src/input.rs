use dioxus::prelude::*;
use smol_str::SmolStr;

const INPUT_CSS: &str = "file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input h-9 w-full min-w-0 rounded-md border bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive";

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
