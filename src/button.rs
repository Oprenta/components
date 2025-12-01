use dioxus::prelude::*;
use smol_str::SmolStr;

// Base button styles
const BUTTON_BASE: &str = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]";

// Variant styles
const BUTTON_DEFAULT: &str = "bg-primary text-primary-foreground hover:bg-primary/90";
const BUTTON_DESTRUCTIVE: &str = "bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60";
const BUTTON_OUTLINE: &str = "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50";
const BUTTON_SECONDARY: &str = "bg-secondary text-secondary-foreground hover:bg-secondary/80";
const BUTTON_GHOST: &str = "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50";
const BUTTON_LINK: &str = "text-primary underline-offset-4 hover:underline";

// Size styles
const BUTTON_SIZE_DEFAULT: &str = "h-9 px-4 py-2";
const BUTTON_SIZE_SM: &str = "h-8 rounded-md gap-1.5 px-3";
const BUTTON_SIZE_LG: &str = "h-10 rounded-md px-6";
const BUTTON_SIZE_ICON: &str = "size-9";
const BUTTON_SIZE_ICON_SM: &str = "size-8";
const BUTTON_SIZE_ICON_LG: &str = "size-10";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

impl ButtonVariant {
    fn classes(&self) -> &'static str {
        match self {
            ButtonVariant::Default => BUTTON_DEFAULT,
            ButtonVariant::Destructive => BUTTON_DESTRUCTIVE,
            ButtonVariant::Outline => BUTTON_OUTLINE,
            ButtonVariant::Secondary => BUTTON_SECONDARY,
            ButtonVariant::Ghost => BUTTON_GHOST,
            ButtonVariant::Link => BUTTON_LINK,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
    IconSm,
    IconLg,
}

impl ButtonSize {
    fn classes(&self) -> &'static str {
        match self {
            ButtonSize::Default => BUTTON_SIZE_DEFAULT,
            ButtonSize::Sm => BUTTON_SIZE_SM,
            ButtonSize::Lg => BUTTON_SIZE_LG,
            ButtonSize::Icon => BUTTON_SIZE_ICON,
            ButtonSize::IconSm => BUTTON_SIZE_ICON_SM,
            ButtonSize::IconLg => BUTTON_SIZE_ICON_LG,
        }
    }
}

#[component]
pub fn Button(
    children: Element,
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    class: Option<SmolStr>,
) -> Element {
    let classes = format!(
        "{} {} {} {}",
        BUTTON_BASE,
        variant.classes(),
        size.classes(),
        class.as_deref().unwrap_or("")
    );

    rsx! {
        button {
            class: "{classes}",
            disabled: disabled,
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            "data-slot": "button",
            {children}
        }
    }
}
