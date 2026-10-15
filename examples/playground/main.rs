#![allow(non_snake_case)]
use dioxus::prelude::*;
use shadcn_dioxus::Button;

#[cfg(all(not(target_arch = "wasm32"), feature = "native"))]
#[oxidase::main]
fn main() {
    dioxus::launch(App);
}

#[cfg(not(all(not(target_arch = "wasm32"), feature = "native")))]
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        div {
            style: "min-height: 100vh; background-color: #f8fafc; padding: 2rem; display: flex; flex-direction: column; gap: 1.5rem; align-items: flex-start;",
            h1 {
                style: "font-size: 1.5rem; font-weight: 700; color: #0f172a;",
                "shadcn-dioxus Playground"
            }
            p {
                style: "font-size: 0.875rem; color: #64748b;",
                "Tailwind v4 styled button component parity slice."
            }
            Button {
                gui_test_id: "button-primary".to_string(),
                "Primary Button"
            }
        }
    }
}
