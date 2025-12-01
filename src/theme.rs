use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn class(&self) -> &'static str {
        match self {
            ThemeMode::Light => "",
            ThemeMode::Dark => "dark",
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        }
    }
}

/// Theme provider component that applies theme class
#[component]
pub fn ThemeProvider(mode: ThemeMode, children: Element) -> Element {
    use_context_provider(|| Signal::new(mode));

    rsx! {
        div {
            class: "{mode.class()}",
            {children}
        }
    }
}

/// Hook to access and toggle the current theme
pub fn use_theme() -> Signal<ThemeMode> {
    use_context::<Signal<ThemeMode>>()
}
