use dioxus::signals::GlobalSignal;

pub static APP_READY: GlobalSignal<bool> = GlobalSignal::new(|| false);
