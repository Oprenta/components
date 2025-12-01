use dioxus::prelude::*;
use smol_str::SmolStr;

const ITEM_BASE: &str = "flex gap-3 items-start";
const ITEM_DEFAULT: &str = "";
const ITEM_OUTLINE: &str = "border rounded-lg p-3";
const ITEM_MUTED: &str = "bg-muted/50 rounded-lg p-3";

const ITEM_SIZE_DEFAULT: &str = "p-3";
const ITEM_SIZE_SM: &str = "p-2 gap-2";

const ITEM_GROUP_BASE: &str = "flex flex-col";

const ITEM_SEPARATOR: &str = "shrink-0 bg-border/50 h-[1px] w-full my-2";

const ITEM_MEDIA_BASE: &str = "flex shrink-0 items-center justify-center";
const ITEM_MEDIA_DEFAULT: &str = "size-10";
const ITEM_MEDIA_ICON: &str = "size-5 text-muted-foreground";
const ITEM_MEDIA_IMAGE: &str = "size-10 rounded-md overflow-hidden";

const ITEM_CONTENT: &str = "flex flex-col gap-1 flex-1 min-w-0";
const ITEM_TITLE: &str = "text-sm font-medium leading-none";
const ITEM_DESCRIPTION: &str = "text-sm text-muted-foreground";
const ITEM_ACTIONS: &str = "flex items-center gap-2 ml-auto";
const ITEM_HEADER: &str = "text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-2";
const ITEM_FOOTER: &str = "text-xs text-muted-foreground mt-2";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

impl ItemVariant {
    fn classes(&self) -> &'static str {
        match self {
            ItemVariant::Default => ITEM_DEFAULT,
            ItemVariant::Outline => ITEM_OUTLINE,
            ItemVariant::Muted => ITEM_MUTED,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
}

impl ItemSize {
    fn classes(&self) -> &'static str {
        match self {
            ItemSize::Default => ITEM_SIZE_DEFAULT,
            ItemSize::Sm => ITEM_SIZE_SM,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

impl ItemMediaVariant {
    fn classes(&self) -> &'static str {
        match self {
            ItemMediaVariant::Default => ITEM_MEDIA_DEFAULT,
            ItemMediaVariant::Icon => ITEM_MEDIA_ICON,
            ItemMediaVariant::Image => ITEM_MEDIA_IMAGE,
        }
    }
}

#[component]
pub fn Item(
    children: Element,
    #[props(default)] variant: ItemVariant,
    #[props(default)] size: ItemSize,
    class: Option<SmolStr>,
) -> Element {
    let classes = format!(
        "{} {} {} {}",
        ITEM_BASE,
        variant.classes(),
        if matches!(variant, ItemVariant::Default) {
            size.classes()
        } else {
            ""
        },
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            class: "{classes}",
            "data-slot": "item",
            {children}
        }
    }
}

#[component]
pub fn ItemGroup(children: Element, class: Option<SmolStr>) -> Element {
    let classes = format!("{} {}", ITEM_GROUP_BASE, class.as_deref().unwrap_or(""));

    rsx! {
        div {
            class: "{classes}",
            "data-slot": "item-group",
            {children}
        }
    }
}

#[component]
pub fn ItemSeparator(class: Option<SmolStr>) -> Element {
    let classes = format!("{} {}", ITEM_SEPARATOR, class.as_deref().unwrap_or(""));

    rsx! {
        div {
            class: "{classes}",
            "data-slot": "item-separator",
            role: "separator"
        }
    }
}

#[component]
pub fn ItemMedia(
    children: Element,
    #[props(default)] variant: ItemMediaVariant,
    class: Option<SmolStr>,
) -> Element {
    let classes = format!(
        "{} {} {}",
        ITEM_MEDIA_BASE,
        variant.classes(),
        class.as_deref().unwrap_or("")
    );

    rsx! {
        div {
            class: "{classes}",
            "data-slot": "item-media",
            {children}
        }
    }
}

#[component]
pub fn ItemContent(children: Element, class: Option<SmolStr>) -> Element {
    let classes = format!("{} {}", ITEM_CONTENT, class.as_deref().unwrap_or(""));

    rsx! {
        div {
            class: "{classes}",
            "data-slot": "item-content",
            {children}
        }
    }
}

#[component]
pub fn ItemTitle(children: Element, class: Option<SmolStr>) -> Element {
    let classes = format!("{} {}", ITEM_TITLE, class.as_deref().unwrap_or(""));

    rsx! {
        div {
            class: "{classes}",
            "data-slot": "item-title",
            {children}
        }
    }
}

#[component]
pub fn ItemDescription(children: Element, class: Option<SmolStr>) -> Element {
    let classes = format!("{} {}", ITEM_DESCRIPTION, class.as_deref().unwrap_or(""));

    rsx! {
        div {
            class: "{classes}",
            "data-slot": "item-description",
            {children}
        }
    }
}

#[component]
pub fn ItemActions(children: Element, class: Option<SmolStr>) -> Element {
    let classes = format!("{} {}", ITEM_ACTIONS, class.as_deref().unwrap_or(""));

    rsx! {
        div {
            class: "{classes}",
            "data-slot": "item-actions",
            {children}
        }
    }
}

#[component]
pub fn ItemHeader(children: Element, class: Option<SmolStr>) -> Element {
    let classes = format!("{} {}", ITEM_HEADER, class.as_deref().unwrap_or(""));

    rsx! {
        div {
            class: "{classes}",
            "data-slot": "item-header",
            {children}
        }
    }
}

#[component]
pub fn ItemFooter(children: Element, class: Option<SmolStr>) -> Element {
    let classes = format!("{} {}", ITEM_FOOTER, class.as_deref().unwrap_or(""));

    rsx! {
        div {
            class: "{classes}",
            "data-slot": "item-footer",
            {children}
        }
    }
}
