use leptos::prelude::*;

/// Orientation options for the scroll area
#[derive(Copy, Clone, PartialEq, Default)]
pub enum ScrollOrientation {
    Vertical,
    Horizontal,
    #[default]
    Both,
}

/// A scrollable container with custom scrollbar styling
///
/// Use the `class` prop to set dimensions (e.g., `max-h-72`, `h-96`, `w-full`).
#[component]
pub fn ScrollArea(
    #[prop(optional, into)] class: String,
    #[prop(optional)] orientation: ScrollOrientation,
    children: Children,
) -> impl IntoView {
    let overflow_class = match orientation {
        ScrollOrientation::Vertical => "overflow-y-auto overflow-x-hidden",
        ScrollOrientation::Horizontal => "overflow-x-auto overflow-y-hidden",
        ScrollOrientation::Both => "overflow-auto",
    };

    let base_class = format!(
        "relative {overflow_class} \
         [&::-webkit-scrollbar]:w-2.5 \
         [&::-webkit-scrollbar]:h-2.5 \
         [&::-webkit-scrollbar-track]:bg-transparent \
         [&::-webkit-scrollbar-thumb]:rounded-full \
         [&::-webkit-scrollbar-thumb]:bg-border \
         [&::-webkit-scrollbar-corner]:bg-transparent"
    );

    let classes = if class.is_empty() {
        base_class
    } else {
        format!("{base_class} {class}")
    };

    view! { <div class=classes>{children()}</div> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(ScrollAreaProps);
    }
}
