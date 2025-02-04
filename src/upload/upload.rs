use crate::service::service::upload_plans;
use crate::service::service::UploadFile;
use crate::Route;
use crate::ThemeContext;
use dioxus::logger::tracing::error;
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use round::round;

#[component]
pub fn Upload() -> Element {
    let mut is_checked = use_signal(|| String::from("false"));
    let mut upload_files: Signal<Vec<UploadFile>> = use_signal(|| vec![]);
    let mut filenames: Signal<Vec<(String, f64)>> = use_signal(|| vec![]);
    let mut total_file_size: Signal<f64> = use_signal(|| 0f64);
    let mut submitting = use_signal(|| false);
    let context = use_context::<ThemeContext>();
    println!("current theme : {:#?}", context.current_theme.read());
    let formatted_file_size = format!("{:.2}", total_file_size);
    let mut percentage = use_signal(|| 0.0f32);
    let mut current_chunk = use_signal(|| 0usize);
    let mut total_chunk = use_signal(|| 0usize);
    let mut error = use_signal(|| "");

    //BROWSING FILES NOT SUBMITTED
    if !submitting() {
        rsx! {
            // BACKGROUND DIV
            div{
                class: "w-full h-screen bg-grid flex flex-col items-center justify-center  ",

                // UPLOAD DIV
                div {
                    // style:"background:yellow;",
                    class:"flex flex-col w-[100%] h-[80%] md:w-[60%] md:h-[70%] items-center justify-center  ",

                        //TITLE TEXT
                        span{
                            class:"font-helvetica font-[200] text-[36px]",
                                if filenames().len() <= 0{
                                    "Upload your plans"
                                }
                                else{
                                    "Submitting your plans"
                                }
                        }

                        //SELECT MULTIPLE PDFS AT ONCE
                        span{
                            class:"mt-0 font-helvetica font-[300]  text-[16px] text-center",

                            if filenames().len() <= 0{
                                "Select multiple pdfs at once using the shift key"
                            }
                            else{

                                span{
                                        class:"font-helvetica font-[400]  text-[16px] text-center text-blue-600",
                                    "{filenames.len()}"
                                }
                                span{
                                        class:"font-helvetica font-[300] text-[16px] text-center",
                                    " files selected / Total Size: "
                                }
                                span{
                                        class:"font-helvetica font-[400] text-[16px] text-center text-blue-600",
                                    // " {total_file_size}"
                                        "{formatted_file_size}"

                                }
                                span{
                                        class:"font-helvetica font-[300] text-[16px] text-center",
                                    " MB"
                                }
                            }
                        }

                        //ERROR MESSAGE TO AGREE TO TERMS AND CONDITIONS
                        if is_checked() == "false"
                        {
                            div {
                                class:"mt-4 text-[14px] md:text-[16px   ] font-helvetica font-[300] text-red-500 italic",
                                "{error}"
                            }
                        }

                        // UPLOAD - LIGHT BLUE RECT DIV
                        div{
                            // style:" background:green;",
                            class:"w-[90%] md:w-full flex flex-col flex-1 bg-blue-100 m-3 bg-opacity-60
                            items-start justify-start border-2 border-dotted border-slate-300
                            overflow-y-auto max-h-[70vh]
                            ",

                            // DISPLAY THE FILES
                            if filenames().len() > 0
                            {
                                div {
                                    class:"m-0 p-0 bg-red-0 flex flex-col items-start justify-center  overflow-y-visible w-full
                                    scroll-smooth border border-red-0
                                    ",
                                    for (index, (name, size)) in filenames.read().iter().enumerate() {

                                        div {
                                            class:"flex flex-row items-center justify-start space-x-2 py-2
                                            hover:bg-blue-200 border-b-[0.1px] border-b-[rgba(202,213,244)]       ",
                                            img {
                                                class:"",
                                                src: "/assets/blue_file.svg",
                                            }
                                            p{
                                                class:"font-helvetica font-[300] text-[14px] md:text-[16px] ",
                                                "{index+1}) {name}"
                                            }
                                            p{
                                                class:"font-helvetica font-[300] text-[14px] md:text-[16px] italic text-blue-500 ",
                                                "{size:.2} MB"
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
                                            class:"p-3 border border-black font-helvetica font-[300] text-[16px] bg-[rgb(45,45,49)] text-white
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
                                                                        info!("first filesnames: {}", file);
                                                                        let mut file_size = 0u64;
                                                                        let mut file_size_float=0f64;
                                                                        let mut file_data:Vec<u8> = vec![];
                                                                        //WRITE TO THE FILNAMES SIGNAL


                                                                        if let Some(size) = file_engine.file_size(&file).await {
                                                                            file_size = size;
                                                                            let mb = size as f64/1024.0/1024.0;
                                                                            // let rounded_mb = format!("{:.2}", mb);
                                                                            let rounded_mb = round(mb,4);
                                                                            total_file_size += rounded_mb;
                                                                            file_size_float = rounded_mb

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

                                                                        filenames.write().push((file.clone(),file_size_float));


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
                                            class:"mt-8 font-helvetica font-[300] text-[14px] md:text-[15px] italic text-blue-900",
                                            "Working Drawings, Engineering, Soil Report, Drainage etc"
                                        }
                                }
                            }
                        }


                        // BOTTOM BUTTONS OF UNDO AND SUBMIT
                        if filenames().len() > 0 {
                            div{
                                class:"h-[10%] flex flex-row space-x-4 items-center bg-amber-00 mt-[21px] ml-4 mr-4",
                                div{
                                    class:"flex items-center relative",
                                    input {
                                        class:"
                                        appearance-none rounded-full w-6 h-6 md:w-8 md:h-8
                                        border border-slate-400 checked:bg-blue-700
                                        cursor-pointer focus:ring-2 focus:ring-blue-400
                                        sm:w-5 sm:h-5 aspect-[1/1]
                                        ",
                                        r#type:"checkbox",
                                        autofocus:true,
                                        oninput:move|evt| {
                                            // println!("Checking value: {:?}", &evt.value());
                                            info!("Checking value: {:?}", &evt.value());
                                            info!("Checking data: {:?}", &evt.data());
                                            *is_checked.write() = evt.value()

                                        }

                                    }
                                    //SHOW THE TICK ICON
                                    if is_checked() == "true" {
                                        img {
                                            class:"absolute w-4 h-4 left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2",
                                            src:"/assets/check-white.svg",
                                        }
                                    }

                                }


                                span{
                                    class:"text-start font-helvetica font-[300] text-[14px] text-[rgb(120,120,120)] text-center",
                                    "By uploading your building plans to Doxle.ai, you confirm you have the necessary permissions
                                    to upload the plans and acknowledge they may be protected by copyright.
                                    For detailed information, please review our "
                                    span{
                                        class:"font-helvetica font-[300] underline text-blue-600",
                                        " Terms & Conditions "
                                    }
                                }
                            }
                            div {
                                class:"flex flex-row items-center justify-center mt-12  md:mt-6 space-x-2",
                                //UNDO BUTTON
                                button {
                                    onclick: move |_| {
                                        //RESET
                                        upload_files.write().clear();
                                        error.set("");
                                        filenames.write().clear();
                                        total_file_size.set(0.0);
                                        submitting.set(false);
                                        is_checked.set("false".to_string());

                                    },
                                    class:"w-[90px] h-[60px] p-3 border border-black font-helvetica font-[300] text-[16px]
                                    hover:font-[400] cursor-pointer items-center justify-center
                                    ",
                                    "Undo"
                                }
                                //SUBMIT BUTTON
                                button {
                                    onclick:  move |_| async move{
                                        // ONLY IF THE TERMS AND CONDITIONS ARE SELECTED
                                        if is_checked() == "true" {
                                            *submitting.write() = true;
                                            let res = upload_plans(
                                                    upload_files(),
                                                    total_file_size(),
                                                    &mut current_chunk,
                                                    &mut total_chunk,
                                                    &mut percentage
                                                ).await;

                                            match res {
                                                Ok(_)=>{
                                                    navigator().push(Route::UserForm {
                                                        // verify_code:"false".to_string()
                                                    });
                                                    info!("Files have been uploaded to S3..")

                                                }
                                                Err(e)=>{
                                                    error!("Error {:?}",e)
                                                }
                                            }
                                        }
                                        //TERMS & CONDITIONS ARE NOT CHECKED
                                        else{
                                            error.set("Select the checkbox to agree to the Terms and Conditions");
                                        }
                                    },
                                    class:"w-[90px] h-[60px] p-3 border border-black font-helvetica font-[300] text-[16px] bg-[rgb(45,45,49)] text-white
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
    //SUBMIT BUTTON IS PRESSED
    else {
        rsx! {
            div{
                class:"w-full h-screen flex flex-col justify-center items-center relative space-y-1",
                if percentage() >=75.0 {
                    div{
                        class:"w-40 h-40 bg-green-500 rounded-full animate-pulse"
                    }
                }
                else{
                    div{
                        class:"w-40 h-40 bg-blue-500 rounded-full animate-pulse"
                    }
                }
                span{
                    class:"font-helvetica font-[200] text-[30px] bg-slate-50 text-center",
                    "Uploading"
                }
                //ONLY FOR MULTI PART UPLOADS
                if total_file_size() >= 5.0 {
                    div {
                        class: "mt-4 text-center font-helvetica font-[200]",
                        h2 { "Progress: {percentage:.2}% ({current_chunk} / {total_chunk})" }
                        div {
                            class: "w-full h-2 bg-gray-200 rounded-full overflow-hidden mt-2",
                            if percentage() >=75.0 {
                                div {
                                    class: "h-full bg-green-500 font-helvetica font-[200]",
                                    style: "width: {percentage}%;"
                                }
                            }
                            else{
                                div {
                                    class: "h-full bg-blue-500 font-helvetica font-[200]",
                                    style: "width: {percentage}%;"
                                }
                            }
                        }
                    }
                }
                span{
                    class:"font-helvetica font-[300] text-[14px] text-slate-400 italic text-center",
                    "Files more than 20 MB can take several minutes"
                }

            }
        }
    }
}
