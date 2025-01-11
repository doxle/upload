use std::collections::BTreeMap;

use crate::{Theme, ThemeContext};
use anyhow::{Context, Error};
use chrono::{DateTime, Utc};
use dioxus::logger::*;
use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct JotItem {
    pub id: u32,
    pub content: String,
    pub checked: bool,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
}

impl Default for JotItem {
    fn default() -> Self {
        Self {
            id: 0,
            content: String::new(),
            checked: false,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct JotResponse {
    pub status: u16,
    pub message: String,
    pub data: Vec<JotItem>,
}

static JOTS: GlobalSignal<BTreeMap<u32, JotItem>> =
    Signal::global(|| BTreeMap::<u32, JotItem>::new());

#[component]
pub fn Jot() -> Element {
    let context = use_context::<ThemeContext>();
    println!("current theme : {:#?}", context.current_theme.read());

    rsx! {
        div{
            class:"w-full md:w-[40%]",

            span{
                class:"mt-[100px] md:mt-[45px] md:mb-[45px] flex justify-center font-helvetica font-thin text-[45px] md:text-[63px] text-[rgba(169,200,242,1)] dark:text-[rgba(122,156,208,0.7)]",
                "doxle"
            }
            // JotHeader{}
            JotList{}
        }

    }
}

#[component]
pub fn JotHeader() -> Element {
    let mut draft = use_signal(|| "".to_string());
    let mut jotid: Signal<u32> = use_signal(|| 0);

    let onkeydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter {
            let jot_item = JotItem {
                id: jotid(),
                content: draft(),
                checked: false,
                ..Default::default()
            };
            JOTS.write().insert(jotid() as u32, jot_item);
            jotid += 1;
        }
    };

    rsx! {
        div{
            class:"mt-[60px] md:mt-[0px] p-0 m-0  h-[60px] text-black dark:text-white bg-[rgb(250,250,250)] dark:bg-[rgb(33,35,36)]
            border-b border-b-[rgb(230,230,230)] dark:border-b-[rgb(67,71,91)] flex",
            input {
                // style:"background:red; margin:0; padding:0p;",
                class:"pl-5 w-full leading-[1.2] font-helvetica font-thin text-[24px] italic text-[rgb(240,240,240)] dark:text-[rgb(227,232,255)] bg-[rgb(250,250,250)] dark:bg-[rgb(33,35,36)] ",
                placeholder:"What needs to be done?",
                value:"{draft}",
                autofocus:"true",
                oninput:move |evt| draft.set(evt.value()),
                onkeydown:onkeydown
            }
        }
    }
}

#[component]
pub fn JotList() -> Element {
    rsx! {
            div{
                class:"md:m-0 md:p-0 w-full h-screen w-text-black dark:text-white ",

                ul{
                    class:"p-0 m-0 flex flex-col items-start space-y-0 ",
                     style: "background:green; margin-left: 10px; margin-right: 10px;",


                        li
                        {
                                class: " w-full flex flex-row justify-content items-center h-[60px] bg-[rgb(250,250,250)] dark:bg-[rgb(33,35,36)] border-b
                                border-b-[rgb(230,230,230)] dark:border-b-[rgb(67,71,91)] overflow-visible",

                                input
                                {
                                    class: "ml-5 flex appearance-none rounded-full bg-[rgba(169,200,242,0.5)] dark:bg-[rgba(122,156,208,0.7)]
                                            checked:bg-blue-300 dark:checked:bg-blue-400 focus:outline-none  focus:ring-blue-100 cursor-pointer",
                                    style:"width:30px; height:30px;",
                                    r#type: "checkbox",
                                    onchange:move|evt|
                                    {

                                    }

                                }
                                div{
                                    class:"flex flex-1 flex-row items-center justify-between bg-red-100 ml-5 mr-[-15px]",
                                    style:"margin-right: 15px;",
                                    span
                                    {
                                        class: "text-[rgb(77,77,77)] dark:text-[rgb(227,232,255)] font-helvetica font-light text-[24px] md:text-[24px]",
                                        "Preliminaries"
                                    }
                                    span
                                    {
                                        class: " text-[rgb(77,77,77)] dark:text-[rgb(227,232,255)] font-helvetica font-light text-[24px] md:text-[24px]",
                                        "$5000"
                                    }
                                }

                        },
                        li
                        {
                                class: " w-full flex flex-row justify-content items-center h-[60px] bg-[rgb(250,250,250)] dark:bg-[rgb(33,35,36)] border-b
                                border-b-[rgb(230,230,230)] dark:border-b-[rgb(67,71,91)] overflow-visible",
                                input
                                {
                                    class: "ml-5 flex appearance-none rounded-full bg-[rgba(169,200,242,0.5)] dark:bg-[rgba(122,156,208,0.7)]
                                            checked:bg-blue-300 dark:checked:bg-blue-400 focus:outline-none  focus:ring-blue-100 cursor-pointer",
                                    style:"width:30px; height:30px;",
                                    r#type: "checkbox",
                                    onchange:move|evt|
                                    {

                                    }

                                }
                                div{
                                    class:"flex flex-1 flex-row items-center justify-between bg-red-100 ml-5 mr-[-15px]",
                                    style:"margin-right: 15px;",
                                    span
                                    {
                                        class: "text-[rgb(77,77,77)] dark:text-[rgb(227,232,255)] font-helvetica font-light text-[24px] md:text-[24px]",
                                        "Telstra"
                                    }
                                    span
                                    {
                                        class: " text-[rgb(77,77,77)] dark:text-[rgb(227,232,255)] font-helvetica font-light text-[24px] md:text-[24px]",
                                        "$2,500"
                                    }
                                }

                        },
                        li
                        {
                                class: " w-full flex flex-row justify-content items-center h-[60px] bg-[rgb(250,250,250)] dark:bg-[rgb(33,35,36)] border-b
                                border-b-[rgb(230,230,230)] dark:border-b-[rgb(67,71,91)] overflow-visible",
                                input
                                {
                                    class: "ml-5 flex appearance-none rounded-full bg-[rgba(169,200,242,0.5)] dark:bg-[rgba(122,156,208,0.7)]
                                            checked:bg-blue-300 dark:checked:bg-blue-400 focus:outline-none  focus:ring-blue-100 cursor-pointer",
                                    style:"width:30px; height:30px;",
                                    r#type: "checkbox",
                                    onchange:move|evt|
                                    {

                                    }

                                }
                                div{
                                    class:"flex flex-1 flex-row items-center justify-between bg-red-100 ml-5 mr-[-15px]",
                                    style:"margin-right: 15px;",
                                    span
                                    {
                                        class: "text-[rgb(77,77,77)] dark:text-[rgb(227,232,255)] font-helvetica font-light text-[24px] md:text-[24px]",
                                        "Demolition"
                                    }
                                    span
                                    {
                                        class: " text-[rgb(77,77,77)] dark:text-[rgb(227,232,255)] font-helvetica font-light text-[24px] md:text-[24px]",
                                        "$15,500"
                                    }
                                }

                        }//// l//// li

                } //ul
            }//div
    }
}
