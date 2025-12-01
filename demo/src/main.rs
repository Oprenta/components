use dioxus::prelude::*;
use oprenta_components::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut theme = use_signal(|| ThemeMode::Light);

    rsx! {
        document::Link { rel: "stylesheet", href: COMPONENT_STYLES}

        ThemeProvider {
            mode: theme(),

            div { class: "min-h-screen bg-background text-foreground p-8",
                div { class: "flex justify-between items-center mb-8",
                    h1 { class: "text-4xl font-bold", "Card Component Demo" }
                    Button {
                        onclick: move |_| theme.set(theme().toggle()),
                        "Toggle Theme"
                    }
                }

            div { class: "grid gap-8 md:grid-cols-2 lg:grid-cols-3",

                // Simple card
                Card {
                    CardHeader {
                        CardTitle { "Simple Card" }
                        CardDescription { "This is a basic card with title and description" }
                    }
                    CardContent {
                        p { "This is the card content area. You can put any content here." }
                    }
                }

                // Card with footer
                Card {
                    CardHeader {
                        CardTitle { "Card with Footer" }
                        CardDescription { "A card demonstrating the footer section" }
                    }
                    CardContent {
                        p { "This card includes a footer section below." }
                    }
                    CardFooter {
                        Button { "Action" }
                    }
                }

                // Card with action
                Card {
                    CardHeader {
                        CardTitle { "Card with Action" }
                        CardDescription { "This card has an action button in the header" }
                        CardAction {
                            Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "More" }
                        }
                    }
                    CardContent {
                        p { "The action button is positioned in the top-right corner of the header." }
                    }
                }

                // Full-featured card
                Card {
                    CardHeader {
                        CardTitle { "Complete Card" }
                        CardDescription { "Showcasing all card components" }
                        CardAction {
                            Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Edit" }
                        }
                    }
                    CardContent {
                        p { class: "mb-4", "This card demonstrates all available card components:" }
                        ul { class: "list-disc list-inside space-y-1",
                            li { "CardHeader with title and description" }
                            li { "CardAction in the header" }
                            li { "CardContent for the main content" }
                            li { "CardFooter with actions" }
                        }
                    }
                    CardFooter {
                        Button { class: "mr-2", "Save" }
                        Button { variant: ButtonVariant::Outline, "Cancel" }
                    }
                }
            }
            }
        }
    }
}
