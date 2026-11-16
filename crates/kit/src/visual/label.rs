use leptos::children::Children;
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

/// Renders an accessible label associated with form controls
///
/// # Examples
///
/// ```
/// use holt_kit::visual::Label;
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let _view = view! { <Label>"Email"</Label> };
/// });
/// ```
#[component]
pub fn Label(
    #[prop(optional, into)] class: String,
    #[prop(optional_no_strip, into)] r#for: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let classes = tw_merge!(
        "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 select-none",
        &class
    );

    view! {
        <label class=classes r#for=r#for>
            {children()}
        </label>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(LabelProps);
    }
}
