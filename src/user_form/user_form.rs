use crate::service::service::send_email_request;
use crate::Route;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
// use dioxus_sdk::storage::{use_storage, LocalStorage};
use gloo_timers::callback::Timeout;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    UserForm,
    Timer,
    Code,
}

#[component]
pub fn UserForm() -> Element {
    let name = use_signal(|| String::new());
    let email = use_signal(|| String::new());
    let mobile = use_signal(|| String::new());
    let error = use_signal(|| String::new());
    let status = use_signal(|| Status::UserForm);
    let timer_status = use_signal(|| String::from("Verifying"));
    let code_from_server = use_signal(|| String::new());

    // let status = use_signal(|| String::from("Verifying"));
    // let is_routing = use_signal(|| false);

    // let local_code = use_storage::<LocalStorage, _>("verification_code".into(), || "".to_string());

    // info!("Verify Code: {}", verify_code);
    //NAME EMAIL INPUT
    let status_value = status.read();
    match *status_value {
        //DISPLAY USER FORM
        Status::UserForm => rsx! {
            NameEmailInput{
                name:name,
                email:email,
                mobile:mobile,
                status:status,
                code_from_server:code_from_server
            }
        },
        //DISPLAY TIMER
        Status::Timer => rsx! {
            Timer {
                timer_status,
                status
            }
        },
        Status::Code => rsx! {
            div{
                class: "w-full h-screen bg-grid flex flex-col items-center justify-center",
                VerifyCodeInput {
                    name:name,
                    email:email,
                    mobile:mobile,
                    // error:error,
                    status:status,
                    code_from_server:code_from_server
                }

            }
        },
    }
}

//-USER FORM TO VERIFY USER EMAIL
#[component]
pub fn NameEmailInput(
    name: Signal<String>,
    email: Signal<String>,
    mobile: Signal<String>,
    status: Signal<Status>,
    code_from_server: Signal<String>,
) -> Element {
    let mut error = use_signal(|| "");
    rsx! {
        div{
            class: "w-full h-screen bg-grid flex flex-col items-center justify-center m-4",
            span{
            class:"font-helvetica font-[200] text-[45px]",
            " Your details"
            }
            form{
                onsubmit: move |evt| {
                    evt.prevent_default();
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
                            error.set("");

                        },
                        onblur:move|_| {

                            if name().len() <3 {
                                error.set("Name has to be atleast 3 letters")
                            }
                            //RESET THE ERROR
                            else{
                                error.set("");
                            }
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
                        value:"{email}",
                        oninput:move |evt|
                        {
                            email.set(evt.value());
                            error.set("");
                        },
                        onblur:move|_| {
                            let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();

                            if !email_regex.is_match(&email()) {
                                error.set("Invalid email format.");
                            }
                            //RESET THE ERROR
                            else{
                                error.set("");
                            }
                        },
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
                                let mut is_error = false;
                                evt.prevent_default();
                                if name().len() <3 {
                                    error.set("Name has to be atleast 3 letters");
                                    is_error = true;
                                }
                                let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
                                if !email_regex.is_match(&email()) {
                                    error.set("Invalid email format.");
                                    is_error = true;
                                }

                                //NO ERRORS THEN SEND EMAIL
                                if !is_error {
                                    spawn(async move {
                                        let response = send_email_request(name(), email(), mobile()).await;
                                        // let mut local_code_clone = local_code.clone();
                                        match response {
                                            Ok((code, db_status)) => {
                                                info!("Code: {}", code);
                                                info!("Db Status: {:?}", db_status);
                                                code_from_server.set(code);
                                                status.set(Status::Timer);
                                                //SETTING THE LOCAL STORAGE VERIFICATION CODE
                                                // local_code.set("".T);
                                                // local_code.set(code);

                                                // info!("Verifying the code is set: {}", local_code());

                                                //ROUTE TO VERIFICATION
                                                // navigator().push(Route::Verification {  });
                                            }
                                            Err(e) => info!("Err {:?}", e),
                                        }
                                    });
                                }

                        },
                        class:" w-[20%] h-[60px] p-3 border border-black font-helvetica font-[300] text-[16px] bg-[rgb(45,45,49)] text-zinc-100
                        hover:bg-blue-700 cursor-pointer flex items-center justify-center
                        ",
                        "Send"
                    }
                }
                if !error().is_empty()
                {
                    div {
                        class:"mt-4 font-helvetica font-[300] text-[16px] text-center text-red-500 italic",
                        "{error()}"
                    }

                }
            }
        }
    }
}

//TIMER COMPONENT TO BUY SOME TIME FOR THE EMAIL TO BE SENT OUT
#[component]
pub fn Timer(timer_status: Signal<String>, status: Signal<Status>) -> Element {
    use_effect(move || {
        let start_timer = move || {
            let mut timer_status = timer_status.clone();

            Timeout::new(2000, move || {
                timer_status.set("Redirecting".to_string());

                Timeout::new(2000, move || {
                    status.set(Status::Code);
                })
                .forget();
            })
            .forget();
        };
        start_timer();
    });

    rsx! {
        div{
             class: "w-full h-screen bg-grid flex flex-col items-center justify-center m-4",
                if timer_status() == "Verifying" {
                    div{
                         class:"w-40 h-40 bg-blue-500 rounded-full animate-pulse"
                     }
                }
                else{
                    div{
                         class:"w-40 h-40 bg-green-500 rounded-full animate-pulse"
                     }
                }

                span{
                     class:"mt-4 font-helvetica font-[200] text-[30px] text-center text-black",
                     "{timer_status}"
                 }
        }
    }
}

//ENTERING 6 DIGIT CODE
#[component]
pub fn VerifyCodeInput(
    name: Signal<String>,
    email: Signal<String>,
    mobile: Signal<String>,
    status: Signal<Status>,
    code_from_server: Signal<String>,
) -> Element {
    let mut usr_code1 = use_signal(|| String::with_capacity(1));
    let mut usr_code2 = use_signal(|| String::with_capacity(1));
    let mut usr_code3 = use_signal(|| String::with_capacity(1));
    let mut usr_code4 = use_signal(|| String::with_capacity(1));
    let mut usr_code5 = use_signal(|| String::with_capacity(1));
    let mut usr_code6 = use_signal(|| String::with_capacity(1));
    let mut error = use_signal(|| "");

    rsx! {

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
                    value:"{usr_code1}",
                    minlength:1,
                    maxlength:1,
                    autofocus:true,

                    oninput:move |evt|
                    {
                        error.set("");
                        let value = evt.value();
                        if value.len() == 1 && value.chars().all(|c| c.is_numeric()) {
                               usr_code1.set(value);
                        }
                        else{
                            error.set("Only numbers allowed");
                        }

                    }
                }
                input {
                    r#type:"text",
                    class:"p-2 w-[50px] h-[50px] text-center bg-blue-100 border border-slate-300 font-helvetica font-[300] text-[30px]",
                    minlength:1,
                    maxlength:1,
                    // name: "name" ,
                    value:"{usr_code2}",
                    // autofocus:true,
                    oninput:move |evt| {
                        error.set("");
                        let value = evt.value();
                        if value.len() == 1 && value.chars().all(|c| c.is_numeric()) {
                               usr_code2.set(value);
                        }
                        else{
                            error.set("Only numbers allowed");
                        }
                    }
                }
                input {
                    r#type:"text",
                    class:"p-2 w-[50px] h-[50px] text-center bg-blue-100 border border-slate-300 font-helvetica font-[300] text-[30px]",
                    minlength:1,
                    maxlength:1,
                    // name: "name" ,
                    value:"{usr_code3}",
                    // autofocus:true,
                    oninput:move |evt| {
                        error.set("");
                        let value = evt.value();
                        if value.len() == 1 && value.chars().all(|c| c.is_numeric()) {
                               usr_code3.set(value);
                        }
                        else{
                            error.set("Only numbers allowed");
                        }
                    }
                }
                input {
                    r#type:"text",
                    class:"p-2 w-[50px] h-[50px] text-center bg-blue-100 border border-slate-300 font-helvetica font-[300] text-[30px]",
                    minlength:1,
                    maxlength:1,
                    // name: "name" ,
                    value:"{usr_code4}",
                    // autofocus:true,
                    oninput:move |evt| {
                        error.set("");
                        let value = evt.value();
                        if value.len() == 1 && value.chars().all(|c| c.is_numeric()) {
                               usr_code4.set(value);
                        }
                        else{
                            error.set("Only numbers allowed");
                        }
                    }
                }
                input {
                    r#type:"text",
                    class:"p-2 w-[50px] h-[50px] text-center bg-blue-100 border border-slate-300 font-helvetica font-[300] text-[30px]",
                    minlength:1,
                    maxlength:1,
                    // name: "name" ,
                    value:"{usr_code5}",
                    // autofocus:true,
                    oninput:move |evt| {
                        error.set("");
                        let value = evt.value();
                        if value.len() == 1 && value.chars().all(|c| c.is_numeric()) {
                               usr_code5.set(value);
                        }
                        else{
                            error.set("Only numbers allowed");
                        }
                    }
                }
                input {
                    r#type:"text",
                    class:"p-2 w-[50px] h-[50px] text-center bg-blue-100 border border-slate-300 font-helvetica font-[300] text-[30px]",
                    minlength:1,
                    maxlength:1,
                    // name: "name" ,
                    value:"{usr_code6}",
                    // autofocus:true,
                    oninput:move |evt| {
                        error.set("");
                        let value = evt.value();
                        if value.len() == 1 && value.chars().all(|c| c.is_numeric()) {
                               usr_code6.set(value);
                        }
                        else{
                            error.set("Only numbers allowed");
                        }
                    }
                }
            }
            span{
                class:"mt-2 font-helvetica font-[200] text-[16px]  italic",
                "code expires in 30 minutes"
            }
            //2 BUTTONS
            div{
                class:"mt-[25px] w-full flex items-center justify-center space-x-1",
                button {
                    //EMAIL NEEDS TO BE RESENT
                    onclick:  move |_| {
                        spawn(async move {
                            call_email_request(name, email, mobile, &mut code_from_server, &mut status).await;
                        });

                    },
                    class:" w-[90px] h-[55px] p-3 border border-black font-helvetica font-[300] text-[16px]
                        hover:bg-blue-100 cursor-pointer flex items-center justify-center
                        ",
                    "Resend"
                }
                button {
                    //VALIDATE THE CODE
                    onclick:  move |_| {
                        let usr_code = usr_code1() + &usr_code2() + &usr_code3() + &usr_code4() + &usr_code5() + &usr_code6();
                        info!("Entered code: {}", usr_code);
                        info!("Server code: {}", code_from_server());
                        if usr_code == code_from_server(){
                            info!("Code has matched");
                            navigator().push(Route::ThankYou {  });
                        }
                        else{
                             info!("Code does not match");
                             // error.set("Code does not match".to_string());
                        }
                    },
                    class:" w-[90px] h-[55px] p-3 border border-black font-helvetica font-[300] text-[16px] bg-[rgb(45,45,49)] text-zinc-100
                        hover:bg-blue-700 cursor-pointer flex items-center justify-center
                        ",
                    "Submit"
                }
            }

            //ERROR VALIDATION
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

pub async fn call_email_request(
    name: Signal<String>,
    email: Signal<String>,
    mobile: Signal<String>,
    code_from_server: &mut Signal<String>,
    status: &mut Signal<Status>,
) {
    let response = send_email_request(name(), email(), mobile()).await;
    // let mut local_code_clone = local_code.clone();
    match response {
        Ok((code, db_status)) => {
            info!("Code: {}", code);
            info!("Db Status: {:?}", db_status);
            code_from_server.set(code);
            status.set(Status::Timer);
        }
        Err(e) => info!("Err {:?}", e),
    }
}
