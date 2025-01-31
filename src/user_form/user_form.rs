use dioxus::prelude::*;
use regex::Regex;

use crate::Route;

#[component]
pub fn UserForm() -> Element {
    let name = use_signal(|| String::new());
    let email = use_signal(|| String::new());
    let mobile = use_signal(|| 0u32);
    let error = use_signal(|| String::new());

    rsx! {
        div{
             class: "w-full h-screen bg-grid flex flex-col items-center justify-center m-4",
             span{
                class:"font-helvetica font-[200] text-[45px]",
                " Your details"
             }
            FormInput{
                name:name,
                email:email,
                mobile:mobile,
                error:error
            }
            if !error().is_empty()
            {
                div {
                    class:"mt-4 font-helvetica font-[300] text-[16px] text-red-500 italic",
                    "{error()}"
                }

            }

        }
    }
}

//-USER FORM TO VERIFY USER EMAIL
#[component]
pub fn FormInput(
    name: Signal<String>,
    email: Signal<String>,
    mobile: Signal<u32>,
    error: Signal<String>,
) -> Element {
    rsx! {
        form{
            onsubmit: move |evt| {
                evt.prevent_default();
            //     info!("Submitted {:?}", evt);

            //     if name().len() <3{
            //         error.set(String::from("Name cannot be less than 3 letters"));
            //     }
            },
            class:"w-[90%] md:w-[40%] lg:w-[30%] flex flex-col gap-4 text-black flex  ",
            //NAME SECTION
            div{
                class:"flex flex-col py-2",
                div{
                    class:"flex flex-row space-x-2",
                    label{
                        class: "font-helvetica font-[300] text-[18px] text-slate-500",
                        "Name"
                    }
                    img{
                        src:"assets/red_asterik.svg",
                        class:"",
                    }
                }

                label{
                    class: "font-source_code_pro font-[300] text-[14px] text-slate-500",
                    "your name or company name - 3 letters atleast"
                }
                input {
                    r#type:"text",
                    class:"p-2 w-full h-[60px] bg-blue-50 border border-slate-300 font-helvetica font-[300] text-[18px]",
                    name: "name" ,
                     value:"{name}",
                    autofocus:true,
                    // minlength:3,
                    // maxlength:50,
                    oninput:move |evt| {
                        name.set(evt.value());


                    }

                    // placeholder:"Enter your name or company name",
                }
            }
            // EMAIL SECTION
            div{
                class:"flex flex-col ",

                div{
                    class:"flex flex-row space-x-2",
                    label{
                        class: "font-helvetica font-[300] text-[18px] text-slate-500",
                        "Email"
                    }
                    img{
                        src:"assets/red_asterik.svg",
                        class:"",
                    }
                }
                label{
                    class: "font-source_code_pro font-[300] text-[14px] text-slate-500",
                    "we will send the budget to your verified email"
                }

                input {
                    r#type:"email",
                    class:"p-2 w-full h-[60px] bg-blue-50 border border-slate-300 font-helvetica font-[300] text-[18px]",
                    name: "email" ,
                    value:"",
                   oninput:move |evt| email.set(evt.value()),
                }
            }
            // MOBILE SECTION
            div{
                class:"flex flex-col ",
                div{

                    label{
                        class: "font-helvetica font-[300] text-[18px] text-slate-500",
                        "Mobile"
                    }

                }
                label{
                    class: "font-source_code_pro font-[300] text-[14px] text-slate-500",
                    "optional - for sms verifications"
                }
                input {
                    r#type:"number",
                    class:"p-2 w-full h-[60px] bg-blue-50 border border-slate-300 font-helvetica font-[300] text-[18px]",
                    name: "phone_number" ,
                    value:"",
                     oninput:move |evt|{
                         let value = evt.value();
                         if let Ok(parsed_value) = value.parse::<u32>(){
                             mobile.set(parsed_value);
                         }

                    }
                    // placeholder:"Your contact phone number",
                }
            }
            //SEND BUTTON
           div{
               class:"mt-4 flex items-center justify-center cursor-pointer",
               button {
                   onclick:  move |evt| {
                         evt.prevent_default();
                         // Email regex pattern for validation
                        let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
                         if name().len() <3{
                             error.set(String::from("Name cannot be less than 3 letters"));
                         }
                         if !email_regex.is_match(&email()) {
                             error.set("Invalid email format.".to_string());
                        }
                        navigator().push(Route::Verification {  });

                   },
                   class:" w-[20%] h-[60px] p-3 border border-black font-helvetica font-[300] text-[16px] bg-[rgb(45,45,49)] text-zinc-100
                   hover:bg-blue-700 cursor-pointer flex items-center justify-center
                   ",
                   "Send"
               }
           }


        }
    }
}
