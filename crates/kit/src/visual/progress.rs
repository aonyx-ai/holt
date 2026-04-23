use leptos::prelude::*;

/// A progress bar showing completion status
#[component]
pub fn Progress(
    #[prop(optional, into)] class: String,
    #[prop(into, default = Signal::stored(0.0))] value: Signal<f64>,
) -> impl IntoView {
    let base_class = "relative h-2 w-full overflow-hidden rounded-full bg-primary/20";

    let classes = if class.is_empty() {
        base_class.to_string()
    } else {
        format!("{base_class} {class}")
    };

    let indicator_style = move || {
        let v = value.get().clamp(0.0, 100.0);
        format!("width: {v}%")
    };

    let aria_valuenow = move || value.get().clamp(0.0, 100.0) as i32;

    view! {
        <div
            class=classes
            role="progressbar"
            aria-valuenow=aria_valuenow
            aria-valuemin="0"
            aria-valuemax="100"
        >
            <div class="h-full bg-primary transition-all" style=indicator_style />
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_prop_accepts_str_and_string() {
        assert_class_prop!(ProgressProps);
    }
}
