use leptos::children::Children;
use leptos::prelude::*;
use leptos::web_sys::window;

#[derive(Clone, Copy)]
pub struct SidebarContext {
    pub is_open: ReadSignal<bool>,
    pub set_open: WriteSignal<bool>,
    pub is_mobile: ReadSignal<bool>,
}

impl SidebarContext {
    pub fn is_open(&self) -> bool {
        self.is_open.get()
    }

    pub fn toggle(&self) {
        self.set_open.update(|open| *open = !*open);
    }
}

#[component]
pub fn SidebarProvider(
    #[prop(optional)] initial_state: Option<bool>,
    #[prop(optional)] open: Option<ReadSignal<bool>>,
    #[prop(optional)] set_open_prop: Option<WriteSignal<bool>>,
    children: Children,
) -> impl IntoView {
    let initial_mobile = {
        #[cfg(target_arch = "wasm32")]
        {
            window()
                .map(|w| w.inner_width().unwrap().as_f64().unwrap_or(9999.0) < 768.0)
                .unwrap_or(false)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            false
        }
    };

    // Default: open on desktop, closed on mobile (like Shadcn)
    let initial_state = initial_state.unwrap_or(!initial_mobile);

    let (is_open_local, set_open_local) = signal(initial_state);
    let is_open = open.unwrap_or(is_open_local);
    let set_open = set_open_prop.unwrap_or(set_open_local);

    let (is_mobile, set_is_mobile) = signal(initial_mobile);
    Effect::new(move |_| {
        if let Some(win) = window() {
            let width = win.inner_width().unwrap().as_f64().unwrap_or(9999.0);
            set_is_mobile.set(width < 768.0);
        }
    });

    let context = SidebarContext {
        is_open,
        set_open,
        is_mobile,
    };

    provide_context(context);

    view! {
        <div
            class="group/sidebar-wrapper flex min-h-svh w-full has-[[data-variant=inset]]:bg-sidebar"
            style="--sidebar-width: 16rem; --sidebar-width-icon: 3rem; --sidebar-width-mobile: 18rem"
        >
            {children()}
        </div>
    }
}
