use leptos::prelude::*;

/// Orientation options for the separator
#[derive(Clone, Copy, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

/// A separator or divider that can be used to separate content
#[component]
pub fn Separator(
    #[prop(optional, into)] class: String,
    #[prop(optional)] orientation: Orientation,
) -> impl IntoView {
    let classes = {
        let base_class = match orientation {
            Orientation::Horizontal => "h-[1px] w-full shrink-0 bg-border",
            Orientation::Vertical => "h-full w-[1px] shrink-0 bg-border",
        };

        if class.is_empty() {
            base_class.to_string()
        } else {
            format!("{base_class} {class}")
        }
    };

    let orientation_attr = match orientation {
        Orientation::Horizontal => "horizontal",
        Orientation::Vertical => "vertical",
    };

    view! { <div class=classes role="separator" aria-orientation=orientation_attr /> }
}
