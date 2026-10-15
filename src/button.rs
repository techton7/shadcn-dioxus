use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default)]
    pub id: Option<String>,
    #[props(default)]
    pub class: String,
    #[props(default)]
    pub children: Element,
    #[props(default)]
    pub gui_test_id: Option<String>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let base_classes = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors bg-slate-900 text-white shadow-sm px-4 py-2";
    let combined_class = if props.class.is_empty() {
        base_classes.to_string()
    } else {
        format!("{base_classes} {}", props.class)
    };

    rsx! {
        button {
            id: props.id.as_deref().or(props.gui_test_id.as_deref()),
            r#type: "button",
            class: "{combined_class}",
            "data-gui-test-id": props.gui_test_id,
            {props.children}
        }
    }
}
