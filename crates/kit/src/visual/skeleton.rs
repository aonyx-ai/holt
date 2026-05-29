use leptos::prelude::*;

/// A placeholder animation to indicate loading content.
///
/// Control the size and shape via the `class` prop (e.g. `class="h-4 w-[250px]"`
/// or `class="size-12 rounded-full"` for a circle).
#[component]
pub fn Skeleton(#[prop(optional, into)] class: String) -> impl IntoView {
    let classes = {
        let base_class = "animate-pulse rounded-md bg-primary/10";

        if class.is_empty() {
            base_class.to_string()
        } else {
            format!("{base_class} {class}")
        }
    };

    view! { <div class=classes /> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(SkeletonProps);
    }
}
