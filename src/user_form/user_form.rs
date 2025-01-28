// use dioxus::logger::tracing::error;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;

#[component]
pub fn UserForm() -> Element {
    // pub fn UserForm() -> Element {
    // let upload_context = use_context::<UploadContext>();
    // let upload_files = upload_context.upload_files;

    // info!("User_form : {:?}, {}", upload_files, total_file_size);
    // let upload_files_future = use_resource(move || {
    //     let upload_files = upload_files.clone();
    //     async move {
    //         info!("Checking file size again: {}", total_file_size);
    //         upload_plans(upload_files, total_file_size).await
    //     }
    // });

    rsx! {
        div{
             class: "w-full h-screen bg-grid flex flex-col items-center justify-center m-4",
             span{
                        class:"font-helvetica font-[200] text-[45px]",
                        " Your details"
             },
             FormInput{}
        }
    }
    // match &*upload_files_future.read_unchecked() {
    // Some(Ok(response)) => rsx! {
    //    div{
    //        // class:"w-full h-screen bg-grid bg-[size:sm] md:bg-[size:md] lg:bg-[size:lg]",
    //        class: "w-full h-screen bg-grid flex flex-col items-center justify-center m-4",
    //        span{
    //            class:"font-helvetica font-[200] text-[45px]",
    //            " Your details {response :?}"
    //        }
    //        FormInput{}
    //    }
    // },
    // Some(Err(_)) => rsx! {
    //     div{
    //          class: "w-full h-screen bg-grid flex flex-col items-center justify-center m-4",
    //          span{
    //              class:"font-helvetica font-[200] text-[45px] text-red-500",
    //              "Error!"
    //          }
    //     }
    // },
    // None => rsx! { div{
    //      class: "w-full h-screen bg-grid flex flex-col items-center justify-center m-4",
    //      // PulseEffect{}
    //      span{
    //          class:"font-helvetica font-[200] text-[45px] text-blue-400",
    //          "Loading ..."
    //      }
    // }},
}

//-USER FORM TO VERIFY USER EMAIL
#[component]
pub fn FormInput() -> Element {
    rsx! {
        form{
            onsubmit: move |event| {
                info!("Submitted! {event:?}");
                info!("Submitted {:?}", event);
            },
            class:"w-[90%] md:w-[40%] flex flex-col gap-4 text-black flex  ",
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
                    "your name or your company name"
                }
                input {
                    r#type:"text",
                    class:"p-2 w-full h-[60px] bg-blue-50 border border-slate-300 font-helvetica font-[200] text-[18px]",
                    name: "name" ,
                    value:"",
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
                    class:"p-2 w-full h-[60px] bg-blue-50 border border-slate-300 font-helvetica font-[200] text-[18px]",
                    name: "email" ,
                    value:"",
                    // placeholder:"We will send measurements to provided email",
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
                    class:"p-2 w-full h-[60px] bg-blue-50 border border-slate-300 font-helvetica font-[100] text-[18px]",
                    name: "phone_number" ,
                    value:"",
                    // placeholder:"Your contact phone number",
                }
            }
            //SEND BUTTON
           div{
               class:"flex items-center justify-center",
               button {
                   onclick:  move |_| {


                       // spawn(async move
                       // {
                       //          info!("Submit button");
                       //          let res = upload_plans(upload_files(), total_file_size()).await;

                       //          match res {
                       //              Ok(_)=>{
                       //                  info!("Files have been uploaded to S3..")
                       //              }
                       //              Err(e)=>{
                       //                  error!("Error {:?}",e)
                       //              }
                       //          }

                       //          // for i in upload_files.read().iter() {
                       //          //     tracing::warn!("checking before passing");
                       //          //     tracing::info!("{:#?}", i);
                       //          // }

                       //  });
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
