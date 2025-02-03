use crate::{service::service::send_email_request, Route};
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use dioxus_sdk::storage::{use_storage, LocalStorage};
use regex::Regex;

#[component]
pub fn UserForm(verify_code: String) -> Element {
    let name = use_signal(|| String::new());
    let email = use_signal(|| String::new());
    let mobile = use_signal(|| String::new());
    let error = use_signal(|| String::new());

    let local_code = use_storage::<LocalStorage, _>("verification_code".into(), || "".to_string());

    info!("Verify Code: {}", verify_code);
    //NAME EMAIL INPUT
    if verify_code == "false" {
        rsx! {
            div{
                 class: "w-full h-screen bg-grid flex flex-col items-center justify-center m-4",
                 span{
                    class:"font-helvetica font-[200] text-[45px]",
                    " Your details"
                 }
                NameEmailInput{
                    name:name,
                    email:email,
                    mobile:mobile,
                    error:error,
                    local_code: local_code
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
    // VERIFY CODE INPUT
    else {
        rsx! {
            VerifyCodeInput {}
        }
    }
}

//-USER FORM TO VERIFY USER EMAIL
#[component]
pub fn NameEmailInput(
    name: Signal<String>,
    email: Signal<String>,
    mobile: Signal<String>,
    error: Signal<String>,
    local_code: Signal<String>,
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
                        src:"/assets/red_asterik.svg",
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
                        src:"/assets/red_asterik.svg",
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
                     oninput:move |evt| mobile.set(evt.value()),

                    // placeholder:"Your contact phone number",
                }
            }
            //SEND BUTTON
           div{
               class:"mt-4 flex items-center justify-center cursor-pointer",
               button {
                   onclick:
                       move |evt| {
                                // let local_code_clone = local_code.clone();
                                evt.prevent_default();
                                // Email regex pattern for validation
                                let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
                                if name().len() < 3{
                                    error.set(String::from("Name cannot be less than 3 letters"));
                                }
                                if !email_regex.is_match(&email()) {
                                    error.set("Invalid email format.".to_string());
                                }

                                spawn(async move {
                                    let response = send_email_request(name(), email(), mobile()).await;
                                    // let mut local_code_clone = local_code.clone();
                                    match response {
                                        Ok((code, db_status)) => {
                                            info!("Code: {}", code);
                                            info!("Db Status: {:?}", db_status);
                                             //SETTING THE LOCAL STORAGE VERIFICATION CODE
                                             // local_code.set("".T);
                                             local_code.set(code);

                                            info!("Verifying the code is set: {}", local_code());

                                            //ROUTE TO VERIFICATION
                                            // navigator().push(Route::Verification {  });
                                        }
                                        Err(e) => info!("Err {:?}", e),
                                    }
                                });

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

#[component]
pub fn VerifyCodeInput() -> Element {
    rsx! {
        div{
            class: "w-full h-screen bg-grid flex flex-col items-center justify-center m-4",
            span{
                class:"font-helvetica font-[200] text-[45px]",
                " Verify Code"
            }
            div{
                class:"mt-[15px] flex flex-col md:w-[40%] items-center justify-start",
                // span{
                //     class:"font-helvetica font-[200] text-[21px] text-start",
                //     "Six digit code"
                // }
                div{
                    class:"mt-[15px] flex flex-row items-center justify-center space-x-3",
                    input {
                        r#type:"text",
                        class:"p-2 w-[50px] h-[50px] text-center bg-blue-100 border border-slate-300 font-helvetica font-[300] text-[30px]",
                        // name: "name" ,
                        // value:"{name}",
                        autofocus:true,
                        oninput:move |_| {
                            // name.set(evt.value());
                        }
                    }
                    input {
                        r#type:"text",
                        class:"p-2 w-[50px] h-[50px] text-center bg-blue-100 border border-slate-300 font-helvetica font-[300] text-[30px]",
                        // name: "name" ,
                        // value:"{name}",
                        autofocus:true,
                        oninput:move |_| {
                            // name.set(evt.value());
                        }
                    }
                    input {
                        r#type:"text",
                        class:"p-2 w-[50px] h-[50px] text-center bg-blue-100 border border-slate-300 font-helvetica font-[300] text-[30px]",
                        // name: "name" ,
                        // value:"{name}",
                        autofocus:true,
                        oninput:move |_| {
                            // name.set(evt.value());
                        }
                    }
                    input {
                        r#type:"text",
                        class:"p-2 w-[50px] h-[50px] text-center bg-blue-100 border border-slate-300 font-helvetica font-[300] text-[30px]",
                        // name: "name" ,
                        // value:"{name}",
                        autofocus:true,
                        oninput:move |_| {
                            // name.set(evt.value());
                        }
                    }
                    input {
                        r#type:"text",
                        class:"p-2 w-[50px] h-[50px] text-center bg-blue-100 border border-slate-300 font-helvetica font-[300] text-[30px]",
                        // name: "name" ,
                        // value:"{name}",
                        autofocus:true,
                        oninput:move |_| {
                            // name.set(evt.value());
                        }
                    }
                    input {
                        r#type:"text",
                        class:"p-2 w-[50px] h-[50px] text-center bg-blue-100 border border-slate-300 font-helvetica font-[300] text-[30px]",
                        // name: "name" ,
                        // value:"{name}",
                        autofocus:true,
                        oninput:move |_| {
                            // name.set(evt.value());
                        }
                    }
                }
                span{
                    class:"mt-2 font-helvetica font-[200] text-[16px]  italic",
                    "code expires in 30 minutes"
                }
                div{
                    class:"mt-[25px] w-full flex items-center justify-center space-x-1",
                    button {
                        onclick:  move |evt| {

                        },
                        class:" w-[15%] h-[55px] p-3 border border-black font-helvetica font-[300] text-[16px]
                        hover:bg-blue-700 cursor-pointer flex items-center justify-center
                        ",
                        "Resend"
                    }
                    button {
                        onclick:  move |evt| {

                        },
                        class:" w-[15%] h-[55px] p-3 border border-black font-helvetica font-[300] text-[16px] bg-[rgb(45,45,49)] text-zinc-100
                        hover:bg-blue-700 cursor-pointer flex items-center justify-center
                        ",
                        "Submit"
                    }
                }
            }
        }
    }
}
