use dioxus::prelude::*;
use smol_str::SmolStr;

const CHECKBOX_WRAPPER: &str =
    "relative inline-flex h-4 w-4 items-center justify-center align-middle";
const CHECKBOX_INPUT: &str = "peer absolute inset-0 z-10 h-full w-full cursor-pointer appearance-none rounded-[4px] opacity-0 focus-visible:outline-none disabled:cursor-not-allowed";
const CHECKBOX_BOX: &str = "pointer-events-none inline-flex h-4 w-4 items-center justify-center border-input dark:bg-input/30 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground data-[state=checked]:border-primary shrink-0 rounded-[4px] border shadow-xs transition-shadow outline-none peer-focus-visible:border-ring peer-focus-visible:ring-ring/50 peer-focus-visible:ring-[3px] aria-[invalid=true]:ring-destructive/20 dark:aria-[invalid=true]:ring-destructive/40 aria-[invalid=true]:border-destructive peer-disabled:cursor-not-allowed peer-disabled:opacity-50";
const CHECKBOX_INDICATOR: &str = "pointer-events-none hidden";

#[component]
pub fn Checkbox(
    #[props(into)] id: Option<SmolStr>,
    #[props(default)] checked: bool,
    #[props(default)] disabled: bool,
    onchange: Option<EventHandler<FormEvent>>,
    class: Option<SmolStr>,
) -> Element {
    let state = if checked { "checked" } else { "unchecked" };
    let aria_checked = if checked { "true" } else { "false" };
    let classes = format!("{} {}", CHECKBOX_BOX, class.as_deref().unwrap_or(""));

    rsx! {
        span { class: CHECKBOX_WRAPPER,
            input {
                id: id.as_deref(),
                r#type: "checkbox",
                class: CHECKBOX_INPUT,
                checked: checked,
                disabled: disabled,
                onchange: move |evt| {
                    if let Some(handler) = &onchange {
                        handler.call(evt);
                    }
                },
                "aria-checked": aria_checked,
                "data-state": state
            }

            span {
                class: "{classes}",
                "data-slot": "checkbox",
                "data-state": state,

                span {
                    class: CHECKBOX_INDICATOR,
                    "data-slot": "checkbox-indicator",
                    "data-state": state
                }
            }
        }
    }
}
