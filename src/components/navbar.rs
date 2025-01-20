use crate::Route;
use crate::{Theme, ThemeContext};
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let context = use_context::<ThemeContext>();

    rsx! {
        div {
            id: "navbar",
            class:"fixed bottom-0 left-0 w-full p-0 my-0 md:static   md:w-[40%]  md:mt-[100px]
            h-[60px] border-t md:border-t-0 md:border-b border-[rgb(230,230,230)] dark:border-[rgb(59,63,82)]
            flex flex-row items-center ",
            // style:"background:red;",

            div{
                class:"flex-[3]  flex flex-row justify-around md:justify-start md:space-x-4",
                // style:"background:green;",
                Link {
                    class:" px-8 py-3 md:px-4 md:py-3 cursor-pointer hover:bg-[rgba(211,229,255,0.5)] dark:hover:bg-[rgb(125,147,194)]",
                    to: Route::Canvas   {},
                    img{
                        class:"w-[24px] h-[25px] ",
                        src: match *context.current_theme.read(){
                            Theme::Dark=>  "assets/home-dark.svg",
                            Theme::Light=>  "assets/home-light.svg",
                        },
                        alt:"Home Icon",
                    }
                }
                Link {
                    class:"px-4 py-3 cursor-pointer hover:bg-[rgba(211,229,255,0.5)] dark:hover:bg-[rgb(125,147,194)]",
                    to: Route:: Jot{},
                    img{
                        class:"mt-[0px] w-[26px] h-[26px]",
                        src: match *context.current_theme.read(){
                            Theme::Dark=>  "assets/todo-dark.svg",
                            Theme::Light=>  "assets/todo-light.svg",
                        },
                        alt:"Home Icon",
                    }
                }
                Link {
                    class:"px-4 py-3 cursor-pointer hover:bg-[rgba(211,229,255,0.5)] dark:hover:bg-[rgb(125,147,194)]",
                    to: Route::Jot {},
                    img{
                        class:"w-[23px] h-[25px]",
                        src: match *context.current_theme.read(){
                            Theme::Dark=>  "assets/upload-dark.svg",
                            Theme::Light=>  "assets/upload-light.svg",
                        },
                        alt:"Home Icon",
                    }
                }

            }
            div{
                class:"flex-[1] flex justify-center md:justify-end items-center",
                // style:"background:blue;",
                Link {
                    class:"px-4 py-3 cursor-pointer hover:bg-[rgba(211,229,255,0.5)] dark:hover:bg-[rgb(125,147,194)]",
                    to: Route::Jot {},
                    img{
                        class:"mt-[3.5px] w-[23px] h-[20px]",
                        src: match *context.current_theme.read(){
                            Theme::Dark=>  "assets/menu-dark.svg",
                            Theme::Light=>  "assets/menu-light.svg",
                        },
                        alt:"Home Icon",
                    }
                }
            }


            // Link {
            //     to: Route::Blog { id: 1 },
            //     "Blog"
            // }
        }

        Outlet::<Route> {}
    }
}
