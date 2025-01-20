use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Dialog(show: Signal<bool>, message: Signal<String>) -> Element {
    rsx! {
        div {
            class: "fixed top-0 left-0 flex w-full h-full justify-center items-center justify-center ",
            class: if !show() { "hidden" },
            // BACKGROUND DIV
            div {
                class: "fixed top-0 left-0 h-full w-full bg-gray-500 opacity-50
                items-center justify-center
                ",
                class: if !show() { "hidden" },
                // THIS DISPOSES THE DIALOG WHEN YOU CLICK OUTSIDE THE DIALOG
                onclick: move |_| *show.write() = false
            }
            //ACTUAL DIALOG
            form {
                class: "relative  w-[70%] md:w-[30%] space-y-6 bg-white p-10 rounded-2xl items-center
                 justify-center
                ",
                class: if !show() { "hidden" },
                id: "cool-form",
                style: "display: flex; flex-direction: column;",


                // You can attach a handler to the entire form
                // oninput: move |ev| {
                //     println!("Input event: {:#?}", ev);
                // },
                // onsubmit: move |ev| {
                //     println!("Submit event: {:#?}", ev);
                // },

                // Regular text inputs with handlers
                // label { class: " font-helvetica font-[200] text-xl", r#for: "message", "Message" }
                // input {
                //     class: " ring-2 rounded px-2",
                //     r#type: "text",
                //     name: "message",
                //     oninput: move |ev| *message.write() = ev.value()
                // }
                label {
                    class: " font-helvetica font-[200] text-[16px]",
                    "{message}"
                }
                // div { class: "flex flex-row space-x-5 ",
                button {
                    class: "p-3 ring-2 text-[14px] font-helvetica font-[200] hover:bg-blue-600 shadow-xl hover:shadow-none
                            bg-black text-white w-[60px] rounded-md
                    ",
                    onclick: move |_| {
                        *show.write() = false;
                        // navigator().push(Route::Legal {  });

                    },

                    // disabled: !show(),
                    "Got it"
                }
                    // button {
                    //     class: "px-2 ring-2 rounded-xl hover:bg-blue-100 shadow-xl hover:shadow-none",
                    //     onclick: move |_| *show.write() = false,
                    //     r#type: "submit",
                    //     value: "Submit",
                    //     "Submit"
                    // }
                // }
            }
        }
    }
}
