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
                // Header with theme toggle
                div { class: "flex justify-between items-center mb-12",
                    div {}
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        onclick: move |_| theme.set(theme().toggle()),
                        "Toggle Theme"
                    }
                }

                // Pricing page header
                div { class: "text-center mb-16",
                    h1 { class: "text-5xl font-bold mb-4", "Choose Your Plan" }
                    p { class: "text-xl text-muted-foreground max-w-2xl mx-auto",
                        "Select the perfect plan for your needs. All plans include a 14-day free trial."
                    }
                }

                // Pricing cards grid
                div { class: "grid gap-8 md:grid-cols-3 max-w-7xl mx-auto",
                    PricingCard {
                        name: "Starter",
                        price: "$9",
                        period: "per month",
                        description: "Perfect for individuals and small projects",
                        features: vec![
                            "Up to 5 projects",
                            "Basic analytics",
                            "Email support",
                            "1GB storage",
                            "Community access"
                        ],
                        is_popular: false
                    }

                    PricingCard {
                        name: "Professional",
                        price: "$29",
                        period: "per month",
                        description: "Best for growing teams and businesses",
                        features: vec![
                            "Unlimited projects",
                            "Advanced analytics",
                            "Priority support",
                            "50GB storage",
                            "Team collaboration",
                            "Custom integrations",
                            "API access"
                        ],
                        is_popular: true
                    }

                    PricingCard {
                        name: "Enterprise",
                        price: "$99",
                        period: "per month",
                        description: "For large organizations with advanced needs",
                        features: vec![
                            "Everything in Professional",
                            "Unlimited storage",
                            "24/7 phone support",
                            "Dedicated account manager",
                            "Custom contracts",
                            "SLA guarantee",
                            "Advanced security",
                            "On-premise deployment"
                        ],
                        is_popular: false
                    }
                }
            }
        }
    }
}

#[component]
fn PricingCard(
    name: String,
    price: String,
    period: String,
    description: String,
    features: Vec<&'static str>,
    #[props(default = false)] is_popular: bool,
) -> Element {
    let card_class = if is_popular {
        Smol("relative ring-2 ring-primary scale-105 shadow-xl")
    } else {
        Smol("")
    };

    rsx! {
        Card { class: card_class,
            // Popular badge
            if is_popular {
                div { class: "absolute -top-4 left-1/2 -translate-x-1/2",
                    div { class: "bg-primary text-primary-foreground px-4 py-1 rounded-full text-sm font-semibold",
                        "Most Popular"
                    }
                }
            }

            CardHeader { class: Smol("text-center"),
                CardTitle { class: Smol("text-2xl"), "{name}" }
                CardDescription { class: Smol("mt-2"), "{description}" }

                // Price display
                div { class: "mt-6",
                    div { class: "flex items-baseline justify-center gap-1",
                        span { class: "text-5xl font-bold tracking-tight", "{price}" }
                        span { class: "text-muted-foreground text-sm", "/{period}" }
                    }
                }
            }

            CardContent { class: Smol("pt-4"),
                // Features list
                ul { class: "space-y-3",
                    for feature in features {
                        li { class: "flex items-start gap-3",
                            // Checkmark icon
                            svg {
                                class: "w-5 h-5 text-primary flex-shrink-0 mt-0.5",
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 20 20",
                                fill: "currentColor",
                                path {
                                    fill_rule: "evenodd",
                                    d: "M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z",
                                    clip_rule: "evenodd"
                                }
                            }
                            span { class: "text-sm", "{feature}" }
                        }
                    }
                }
            }

            CardFooter { class: Smol("flex-col gap-4"),
                Button {
                    class: Smol("w-full"),
                    variant: if is_popular { ButtonVariant::Default } else { ButtonVariant::Outline },
                    size: ButtonSize::Lg,
                    "Get Started"
                }
                p { class: "text-xs text-muted-foreground text-center",
                    "No credit card required"
                }
            }
        }
    }
}
