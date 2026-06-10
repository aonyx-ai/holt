use leptos::prelude::*;

/// Displays content within a desired aspect ratio
#[component]
pub fn AspectRatio(
    #[prop(optional, into)] class: String,
    /// The desired aspect ratio (e.g. 16.0 / 9.0 for 16:9)
    #[prop(optional, default = 1.0)]
    ratio: f64,
    children: Children,
) -> impl IntoView {
    let classes = {
        let base_class = "relative w-full overflow-hidden";

        if class.is_empty() {
            base_class.to_string()
        } else {
            format!("{base_class} {class}")
        }
    };

    let style = format!("aspect-ratio: {ratio}");

    view! {
        <div class=classes style=style>
            {children()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(AspectRatioProps);
    }
}
