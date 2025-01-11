use crate::ThemeContext;
use dioxus::prelude::*;

#[component]
pub fn Upload() -> Element {
    let context = use_context::<ThemeContext>();
    println!("current theme : {:#?}", context.current_theme.read());

    rsx! {
        div{
            class:"w-full md:w-[40%]",
            "doxle"
        }
    }
}
