use crate::components::dialog::Dialog;
use crate::Route;
use dioxus::logger::tracing;
use dioxus::prelude::*;

#[component]
pub fn Legal() -> Element {
    let mut is_checked = use_signal(|| String::from("false"));
    let mut show_dialog = use_signal(|| false);
    let mut dialog_message = use_signal(|| {
        String::from("You will need to agree to the terms by selecting the checkbox")
    });

    rsx! {
        div{
            // class:"w-full h-screen bg-grid bg-[size:sm] md:bg-[size:md] lg:bg-[size:lg]",
            class: "w-full h-screen bg-grid flex flex-col items-center justify-center m-4",

            //DIALOG COMPONENT
            Dialog {
                show:show_dialog,
                message:dialog_message,
            }

            //DOG LOGO
            div{
                Link{
                    class:"hover:pointer",
                    to: Route:: Home  {},
                    img {
                        class:"w-full  h-[50px] object-contain flex items-center justify-center mb-4",
                        src:"/assets/dog-black.svg",

                    }
                }
            }

            //LEGAL TEXT
            div{
                class:"w-[85%] md:w-[45%] lg:w-[40%] p-4 items-center justify-center",
                class:"bg-[rgba(242,248,255,0.5)] ",
                // class:"bg-blue-50",
                // style:"background:yellow; padding:5rem;",
                span {
                    class:"text-[15px] font-helvetica font-[200]",
                    "By uploading building plans to Doxle.ai, you agree they will be used solely to improve our
                    machine learning models. We will not use them for construction or other commercial purposes.
                    You confirm you have the rights or permissions to upload the plans and acknowledge they
                    may be protected by copyright. Doxle.ai respects these copyrights and will not claim
                    ownership or misuse them. You agree to indemnify Doxle Pty Ltd against any claims, damages, or
                    losses resulting from improper use of the plans or lack of permissions.
                    For details, see our "
                    span {
                               class: "underline text-blue-700 cursor-pointer",
                               "Terms and Conditions"
                           }
                           "."
                }
                div{
                    class:"flex flex-row space-x-4",

                    input {
                        class:"
                        appearance-none rounded-full w-8 h-8
                        border border-slate-400 checked:bg-blue-700
                        cursor-pointer focus:ring-2 focus:ring-blue-400
                        sm:w-5 sm:h-5 aspect-[1/1] mt-4 md:mt-5
                        ",
                        r#type:"checkbox",
                        oninput:move|evt| {
                            println!("Checking value: {:?}", &evt.value());
                            tracing::info!("Checking value: {:?}", &evt.value());
                            *is_checked.write() = evt.value()
                        }

                    }
                    label{
                        class:"text-[15px] font-helvetica font-[200] italic block mt-4",
                        "\nI confirm that I have read and understood the terms above and affirm that I have the
                        necessary rights and permissions to upload this plan."

                    }
                }

            }

            // AGREE AND CANCEL BUTTON
            div{
                class:"flex flex-row items-center justify-center space-x-2",
                style:"margin-top:4rem;",
                // button{
                //     class:"p-4 border border-black font-helvetica font-[200] text-[16px] bg-black text-white
                //     hover:bg-blue-700",
                //     "Upload Plans"
                // }




                Link {
                    class:"p-3 border border-[rgb(45,45,49)] font-helvetica font-[200] text-[16px] hover:font-[400]",
                    to: Route:: Home  {},
                    "Cancel"

                }

                button{
                    onclick:move|_|{

                        if is_checked() == "true" {
                            println!("clicked");
                            navigator().push(Route::Upload  {  });

                        }
                        //NEED TO SHOW ERROR THAT CANNOT CLICK THE BUTTON
                        else {
                            show_dialog.set(true);
                        }

                        // Link {
                        //     class:"p-3 border border-black font-helvetica font-[200] text-[16px] bg-[rgb(45,45,49)] text-white
                        //     hover:bg-blue-700",
                        //     to: Route:: Upload  {},
                        //     "Agree"

                        // }

                    },
                    class:"p-3 border border-black font-helvetica font-[200] text-[16px] bg-[rgb(45,45,49)] text-white
                    hover:bg-blue-700 cursor:pointer
                    ",
                    "Agree",
                }

            }

        }
    }
}
