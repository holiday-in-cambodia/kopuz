use dioxus::prelude::*;
pub mod reader;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    reader::read();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img {  }
            div { id: "links",
                h1 { "Rusic" }
            }
        }
    }
}
