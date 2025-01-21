use crate::service::service::upload_plans;
use crate::service::service::UploadFile;
use crate::ThemeContext;
use dioxus::logger::tracing::error;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use round::round;
// ruse web_sys::window;

#[component]
pub fn Upload() -> Element {
    // let mut upload_title = use_signal(|| String::from("Upload your plans"));
    let mut upload_files: Signal<Vec<UploadFile>> = use_signal(|| vec![]);
    let mut filenames: Signal<Vec<String>> = use_signal(|| vec![]);
    let mut total_file_size: Signal<f64> = use_signal(|| 0f64);
    let context = use_context::<ThemeContext>();
    println!("current theme : {:#?}", context.current_theme.read());

    rsx! {
        // BACKGROUND DIV
        div{
            class: "w-full h-screen bg-grid flex flex-col items-center justify-center m-4 ",

            // PARENT DIV
            div {
                // style:"background:yellow;",
                class:"p-8 flex flex-col w-[90%] h-[80%] md:w-[60%] md:h-[70%] items-center justify-center  ",

                //TITLE TEXT
               span{
                   class:"font-helvetica font-[200] text-[36px]",
                    if filenames.len() <= 0{
                        "Upload your plans"
                    }
                    else{
                        "Submitting your plans"
                    }
               }

               //SELECT MULTIPLE PDFS AT ONCE
               span{
                   class:"mt-0 font-helvetica font-[300] opacity-50 text-[16px] text-center",

                   if filenames.len() <= 0{
                       "Select multiple pdfs at once using the shift key"
                   }
                   else{

                       span{
                            class:"font-helvetica font-[400] text-[16px] text-center",
                           "{filenames.len()}"
                       }
                       span{
                            class:"font-helvetica font-[300] text-[16px] text-center",
                           " files selected / Total Size: "
                       }
                       span{
                            class:"font-helvetica font-[400] text-[16px] text-center",
                           " {total_file_size}"
                           // "{format!("{:.2}", total_file_size)}"
                       }
                       span{
                            class:"font-helvetica font-[300] text-[16px] text-center",
                           " MB"
                       }
                   }
               }

               // UPLOAD - LIGHT BLUE RECT DIV
               div{
                   // style:"margin-horizontal:2rem; background:yellow;",
                   class:"p-8 w-full flex flex-col flex-1 bg-blue-100 mt-3 bg-opacity-60
                   items-start justify-start border-2 border-dotted border-slate-300

                   ",

                   // IF USER HAS ALREADY SELECTED FILES
                   if filenames().len() > 0
                   {
                       div {
                           class:"p-1  flex flex-col items-start justify-center bg--green-100 overflow-y-auto w-full max-h-[60vh]",
                           for filename in filenames.read().iter() {
                            div {
                                class:"flex flex-row items-center justify-start space-x-2
                                border-b border-b-[rgba(202,213,244)] leading-[3rem]  h-[3rem] bg-amber-0 ",
                                img {
                                    class:"",
                                    src: "/assets/file.svg",
                                }
                                p{

                                    class:"font-helvetica font-[300] text-[14px] ",
                                    "{filename}"
                                }
                            }
                        }
                       }

                       }

                    // NO FILES HAVE BEEN SELECTED -BROWSE BUTTON
                   else{
                       div {
                           class:"w-full h-full flex flex-col items-center justify-center mt-6",
                           label{
                                class:"p-3 border border-black font-helvetica font-[200] text-[16px] bg-[rgb(45,45,49)] text-white
                                hover:bg-blue-700 cursor-pointer
                                ",
                                "Browse"
                                // HIDDEN FILE INPUT
                                    input {
                                            class:"hidden",
                                            id: "file-input",
                                            // class: "hidden",
                                            r#type: "file",
                                            accept: ".pdf",
                                            multiple: true,

                                            // Asynchronous Handling of file upload
                                            onchange: move |evt| {
                                                async move {

                                                    if let Some(file_engine) = evt.files() {


                                                        //ITER OVER THE FILES
                                                        for file in file_engine.files() {
                                                            let mut file_size = 0;
                                                            let mut file_data:Vec<u8> = vec![];
                                                            //WRITE TO THE FILNAMES SIGNAL
                                                            filenames.write().push(file.clone());

                                                            if let Some(size) = file_engine.file_size(&file).await {
                                                                file_size = size;
                                                                let mb = size as f64/1024.0/1024.0;
                                                                // let rounded_mb = format!("{:.2}", mb);
                                                                let rounded_mb = round(mb,2);
                                                                total_file_size += rounded_mb;
                                                            }
                                                            else{
                                                                error!("Error getting file size");
                                                            }

                                                            if let Some(contents) = file_engine.read_file(&file).await {
                                                                file_data.extend(contents);
                                                            }
                                                            else{
                                                                error!("Error reading file");
                                                            }


                                                            let uploadFile = UploadFile {
                                                                file_name: file,
                                                                file_size: file_size,
                                                                file_contents: file_data.clone(),
                                                            };

                                                            upload_files.write().push(uploadFile);
                                                        }
                                                    }
                                                }
                                            }
                                        }


                            }
                            span{
                                class:"mt-8 font-helvetica font-[700] text-[14px] italic text-blue-900",
                                "Working Drawings, Engineering Plans, Soil Report, Drainage etc"
                            }
                       }
                   }
               }

               // SHOW THE BOTTOM BUTTONS OF UNDO AND SUBMIT
               if filenames().len() > 0 {
                   div {
                       class:"flex flex-row items-center justify-center mt-6 space-x-2",
                       button {
                           onclick: move |_| {
                               //RESET
                               upload_files.write().clear();
                               filenames.write().clear();
                               total_file_size.set(0.0);

                           },
                           class:"p-3 border border-black font-helvetica font-[300] text-[16px]
                           hover:font-[300] cursor-pointer items-center justify-center
                           ",
                           "Undo"
                       }
                       //SUBMIT BUTTON
                       button {
                           onclick:  move |_| {
                               async move
                               {
                                        info!("Submit button");
                                        let res = upload_plans(upload_files(), total_file_size()).await;

                                        match res {
                                            Ok(_)=>{
                                                info!("Files have been uploaded to S3..")
                                            }
                                            Err(e)=>{
                                                error!("Error {:?}",e)
                                            }
                                        }

                                        // for i in upload_files.read().iter() {
                                        //     tracing::warn!("checking before passing");
                                        //     tracing::info!("{:#?}", i);
                                        // }

                                }
                           },
                           class:"p-3 border border-black font-helvetica font-[300] text-[16px] bg-[rgb(45,45,49)] text-white
                           hover:bg-blue-700 cursor-pointer items-center justify-center
                           ",
                           "Submit"
                       }
                   }
               }

            }
        }
    }
}
