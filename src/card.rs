use dioxus::prelude::*;

const CARD_CSS: &str =
    "bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm";

const CARD_HEADER_CSS: &str = "@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-2 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6";

const CARD_TITLE_CSS: &str = "leading-none font-semibold";

const CARD_DESCRIPTION_CSS: &str = "text-muted-foreground text-sm";

const CARD_ACTION_CSS: &str = "col-start-2 row-span-2 row-start-1 self-start justify-self-end";

const CARD_CONTENT_CSS: &str = "px-6";

const CARD_FOOTER_CSS: &str = "flex items-center px-6 [.border-t]:pt-6";

#[component]
pub fn Card(children: Element, class: Option<String>) -> Element {
    let class_name = format!("{} {}", CARD_CSS, class.unwrap_or_default());

    rsx! {
        div {
            class: "{class_name}",
            "data-slot": "card",
            {children}
        }
    }
}

#[component]
pub fn CardHeader(children: Element, class: Option<String>) -> Element {
    let class_name = format!("{} {}", CARD_HEADER_CSS, class.unwrap_or_default());

    rsx! {
        div {
            class: "{class_name}",
            "data-slot": "card-header",
            {children}
        }
    }
}

#[component]
pub fn CardTitle(children: Element, class: Option<String>) -> Element {
    let class_name = format!("{} {}", CARD_TITLE_CSS, class.unwrap_or_default());

    rsx! {
        div {
            class: "{class_name}",
            "data-slot": "card-title",
            {children}
        }
    }
}

#[component]
pub fn CardDescription(children: Element, class: Option<String>) -> Element {
    let class_name = format!("{} {}", CARD_DESCRIPTION_CSS, class.unwrap_or_default());

    rsx! {
        div {
            class: "{class_name}",
            "data-slot": "card-description",
            {children}
        }
    }
}

#[component]
pub fn CardAction(children: Element, class: Option<String>) -> Element {
    let class_name = format!("{} {}", CARD_ACTION_CSS, class.unwrap_or_default());

    rsx! {
        div {
            class: "{class_name}",
            "data-slot": "card-action",
            {children}
        }
    }
}

#[component]
pub fn CardContent(children: Element, class: Option<String>) -> Element {
    let class_name = format!("{} {}", CARD_CONTENT_CSS, class.unwrap_or_default());

    rsx! {
        div {
            class: "{class_name}",
            "data-slot": "card-content",
            {children}
        }
    }
}

#[component]
pub fn CardFooter(children: Element, class: Option<String>) -> Element {
    let class_name = format!("{} {}", CARD_FOOTER_CSS, class.unwrap_or_default());

    rsx! {
        div {
            class: "{class_name}",
            "data-slot": "card-footer",
            {children}
        }
    }
}
