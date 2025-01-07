use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let mut dropdown_open = use_signal(|| false); // State to toggle dropdown visibility

    rsx!(
        div {
            class: "flex header-container bg-white text-black w-full h-[40px] relative",

            // Header with dropdown toggle
            div {
                class: "flex items-center justify-between px-4 w-full h-full",

                // // Logo or Title
                img {
                    class: "h-full max-h-[24px] w-auto cursor-pointer",
                    src: "dog.svg",
                     onclick: move |_| {

                    // Use intermediate variable to avoid simultaneous borrow
                        let current_state = *dropdown_open.read();
                        dropdown_open.set(!current_state); // Toggle dropdown
                    }, // T
                    // height: "10px"
                }

                // Dropdown Toggle Button
                button {
                    class: "text-[12px] text-slate-100 hover:text-blue-300",
                    onclick: move |_| {

                    // Use intermediate variable to avoid simultaneous borrow
                        let current_state = *dropdown_open.read();
                        dropdown_open.set(!current_state); // Toggle dropdown
                    }, // Toggle dropdown
                    "Doxle"
                }
            }

            // Dropdown Menu (conditionally rendered)
            {
                if *dropdown_open.read() {
                    rsx!(
                        div {
                            class: "dropdown-menu absolute top-full left-4 bg-slate-950 bg-opacity-100 text-white mt-2 shadow-lg rounded w-48 flex flex-col",
                            // style:"opacity: 0.75;",
                            // Link { class: "text-[12px] text-slate-100 hover:text-white px-4 py-2", to: Route:: {}, "Draw" }
                            // Link { class: "text-[12px] text-slate-400 hover:text-white px-4 py-2", to: Route::Home {}, "Home" }
                            // Link { class: "text-[12px] text-slate-400 hover:text-white px-4 py-2", to: Route::Table { id: 2 }, "Table" }
                            // Link { class: "text-[12px] text-slate-400 hover:text-white px-4 py-2", to: Route::Canvas {}, "Canvas" }
                            // Link { class: "text-[12px] text-slate-400 hover:text-white px-4 py-2", to: Route::Events {}, "Events" }
                            // Link { class: "text-[12px] text-slate-400 hover:text-white px-4 py-2", to: Route::Hooks {}, "Hooks" }
                            // Link { class: "text-[12px] text-slate-400 hover:text-white px-4 py-2", to: Route::Input {}, "Input" }
                            // Link { class: "text-[12px] text-slate-400 hover:text-white px-4 py-2", to: Route::Context {}, "Context" }
                            // Link { class: "text-[12px] text-slate-400 hover:text-white px-4 py-2", to: Route::DynamicRendering {}, "Dynamic Rendering" }

                        }
                    )
                } else {
                    rsx!() // Empty node if dropdown is not open
                }
            }

            // Border positioned at the bottom of the header
            div {
                class: "menu-border bg-[rgb(230,230,230)] w-full h-[1px] absolute bottom-0",
            }
        }
    )
}
