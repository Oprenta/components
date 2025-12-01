use dioxus::prelude::*;

#[component]
pub fn Card(children: Element, class: Option<String>) -> Element {
    let class_name = format!(
        "bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm {}",
        class.unwrap_or_default()
    );

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
    let class_name = format!(
        "@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-2 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6 {}",
        class.unwrap_or_default()
    );

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
    let class_name = format!("leading-none font-semibold {}", class.unwrap_or_default());

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
    let class_name = format!(
        "text-muted-foreground text-sm {}",
        class.unwrap_or_default()
    );

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
    let class_name = format!(
        "col-start-2 row-span-2 row-start-1 self-start justify-self-end {}",
        class.unwrap_or_default()
    );

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
    let class_name = format!("px-6 {}", class.unwrap_or_default());

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
    let class_name = format!(
        "flex items-center px-6 [.border-t]:pt-6 {}",
        class.unwrap_or_default()
    );

    rsx! {
        div {
            class: "{class_name}",
            "data-slot": "card-footer",
            {children}
        }
    }
}
