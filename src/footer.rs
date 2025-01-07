#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx!(div {
        // style: "background-color:#F1F2F5; min-width:100vh; width:100vh; height:2rem;",
        class: "bg-[#F1F2F5] w-full bg-green h-[40px] flex justify-center items-center text-white bg-grid-pattern bg-size-grid-pattern",
        // style: "background-color:green;"
    })
}
