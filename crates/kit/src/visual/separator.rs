use leptos::prelude::*;

/// Orientation options for the separator
#[derive(Copy, Clone, PartialEq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(SeparatorProps);
    }
}
