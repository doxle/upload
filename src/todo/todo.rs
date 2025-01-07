use crate::{Theme, ThemeContext};
use dioxus::logger::*;
use dioxus::prelude::*;

use anyhow::{Context, Error};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TodoItem {
    pub id: i32,
    pub content: String,
    pub checked: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TodoResponse {
    pub status: u16,
    pub message: String,
    pub data: Vec<TodoItem>,
}

static TODOS: GlobalSignal<Vec<TodoItem>> = Signal::global(|| vec![]);
#[component]
pub fn Todo() -> Element {
    let context = use_context::<ThemeContext>();
    println!("current theme : {:#?}", context.current_theme.read());

    rsx! {
        div{
            class:"w-full md:w-[40%]",
            span{
                class:"mt-[70px] md:mt-[45px] md:mb-[45px] flex justify-center font-helvetica font-thin text-[100px] md:text-[63px] text-[rgba(169,200,242,1)] dark:text-[rgba(122,156,208,0.7)]",
                "jot"
            }
            TodoHeader{}
            TodoList{}
        }

    }
}

#[component]
pub fn TodoHeader() -> Element {
    rsx! {
        div{
            class:"mt-[60px] md:mt-[0px] p-0 m-0  h-[60px] text-black dark:text-white bg-[rgb(250,250,250)] dark:bg-[rgb(33,35,36)]
            border-b border-b-[rgb(230,230,230)] dark:border-b-[rgb(67,71,91)] flex",
            input {
                // style:"background:red; margin:0; padding:0p;",
                class:"pl-5 w-full leading-[1.2] font-helvetica font-thin text-[24px] italic text-[rgb(240,240,240)] dark:text-[rgb(227,232,255)] bg-[rgb(250,250,250)] dark:bg-[rgb(33,35,36)] ",
                placeholder:"What needs to be done?",
                value:"",
            }
        }
    }
}

#[component]
pub fn TodoList() -> Element {
    let url = "http://localhost:9000/lambda-url/lambda_todo/";
    let future = use_resource(move || async move {
        let api_response = reqwest::get(url)
            .await
            .context("Failed to fetch data from the API")?;

        let json_response = api_response
            .json::<TodoResponse>()
            .await
            .context("Failed to parse response JSON into TodoResponse");

        //USE RESOURCE HAS THE FOLLOWING RETURN TYPE
        //- 1 OUTER SOME AND NONE
        //- 2 INNER OK AND ERR
        //ALTERNATIVELY SINCE SOME HAS OK AND ERR INSIDE IT WE CAN AVOID BY IMPLEMENTING LIKE DIOXUS DOCS
        //- SOME(OK()), SOME(ERR()), NONE
        match json_response {
            Ok(todo_response) => {
                let data = todo_response.data.clone();
                *TODOS.write() = data.clone();
                println!("TODOS have been written");
                println!("{:#?}", TODOS);
                Ok(data)
            }
            Err(e) => {
                println!("Matchin Json Response Error {:#?}", &e);
                Err(e)
            }
        }
        // json_response
    });

    match &*future.read_unchecked() {
        //SUCCESS VIEW
        Some(Ok(todos)) => rsx! {
            div{
                    class:"md:m-0 md:p-0 w-full h-screen w-text-black dark:text-white ",
                ul{
                    class:"p-0 m-0 flex flex-col items-start space-y-0 ",

                    for todo in TODOS.iter()
                    {
                        li {
                                class: " w-full flex items-center h-[60px] bg-[rgb(250,250,250)] dark:bg-[rgb(33,35,36)] border-b
                                border-b-[rgb(230,230,230)] dark:border-b-[rgb(67,71,91)] ",
                                input {
                                    class: "ml-5 appearance-none w-[29px] h-[30px] rounded-full bg-[rgba(169,200,242,0.5)] dark:bg-[rgba(122,156,208,0.7)]
                                            checked:bg-blue-300 dark:checked:bg-blue-400 focus:outline-none  focus:ring-blue-100 cursor-pointer",
                                    r#type: "checkbox",
                                    onchange:move|evt|{

                                        //FOR MOBILE
                                        println!("Event IOS {:?}", evt.data.value());
                                        //FOR WEB
                                        tracing::info!("Event WEB {:?}", evt);

                                        // UPDATE THE SIGNAL
                                        let mut todo_clone = todo.clone();
                                        todo_clone.checked = !todo_clone.checked; // Toggle the `checked` value
                                        // TODOS.write() =
                                        println!("Updated todo: {:?}", todo);
                                    },
                                    // onclick:move|evt|{
                                    //     println!("Event WEB {:?}", evt);
                                    //     tracing::info!("Event WEB {:?}", evt);
                                    // }
                                }
                                span {
                                    class: "ml-5 leading-[1.2] text-[rgb(77,77,77)] dark:text-[rgb(227,232,255)] font-helvetica font-light text-[24px] md:text-[24px]",
                                    "{todo.content}"
                                }
                            }
                    }
                }

            }
        },

        //ERROR VIEW
        Some(Err(e)) => {
            rsx! {
                div{
                    class:"dark:text-white font-lexend font-extralight",
                    class:"md:m-5 md:p-0 md:mt-[40vh] text-[14px] text-black dark:text-[rgb(213,107,120)] ",
                    "Error: {e}"
                }
            }
        }

        //LOADING VIEW
        None => rsx! {
            div {
                class:"dark:text-white font-lexend font-extralight",
                class:"md:m-5 md:p-0 md:mt-[40vh] text-[14px] text-black dark:text-[rgb(227,232,255)] ",
                "Loading..."
            }
        },
    }
}
