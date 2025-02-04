use crate::{Route, ThemeContext};
use dioxus::prelude::*;

pub const GEOMETRY_SVG: Asset = asset!(
    "/assets/geometry.svg",
    ImageAssetOptions::new()
        .with_size(ImageSize::Manual {
            width: 500,
            height: 500
        })
        .with_format(ImageFormat::Unknown)
);

#[component]
pub fn Home() -> Element {
    let context = use_context::<ThemeContext>();
    println!("current theme : {:#?}", context.current_theme.read());

    rsx! {
        document::Link{
            rel:"preload",
            as: "image",
            href: "{GEOMETRY_SVG}"
        }
        div{
            class:"w-full h-screen flex flex-col md:flex-row",

            // MD=LEFT OR MOBILE=TOP
            div{
                // style:"background:yellow; margin:0; padding:0;",
                class:" h-[60%] md:w-[50%] md:h-[100%]
                bg-no-repeat md:bg-none flex
                 bg-[url('/assets/page_grid.svg')] bg-no-repeat
                  bg-[cover,auto_100%]
                 ",

                div{
                    style:" margin:0; padding:0;",
                    class:"md:m-0 md:hidden  w-full h-full flex
                     bg-no-repeat bg-[contain,100%_100%]
                    ",
                    // bg-[url('/assets/geometry.svg')]
                    // bg-no-repeat bg-cover bg-[contain,100%_100%] bg-center
                    // bg-[url('/assets/page_grid.svg')]


                    img {
                        class:"w-full md:hidden z-0 ",
                        src:"/assets/geometry.svg"
                    }

                }


                // FLEX FLEX-COL WITH THIRD ONE GROWING - HIDDEN FOR MOBILE SCREENS
                div{
                    // style:"background:blue; opacity:0.1;",
                    class:" md:w-full h-full md:flex md:flex-col ",
                    // bg-[length:90%] mt-[100px] md:mt-[0px] md:bg-none flex flex-col",
                    // class:"w-full h-screen flex items-center justify-center md:bg-[url('/assets/geometry.svg')] md:bg-cover md:bg-no-repeat md:bg-center
                    //        md:bg-[length:50%]",

                    // style:"z-index:1; background:yellow;",
                    div{
                        class: "flex-1 ",
                        // "Row 1"
                    }
                    div {
                           class: "flex-1 ",
                           // "Row 2"
                    }
                    div {
                        class: "flex-1 ",
                        // "Row 3"
                    }
                    //LEFT - HIDDEN FOR MOBILE SCREENS
                    div{
                        class: "hidden md:p-3 md:flex-auto md:flex-col md:block ",
                        // style:"background:blue; opacity:0.1;",
                        span{
                            class:"h-14 text-[24px] sm:text-[32px] md:text-[40px] lg:text-[48px] xl:text-[54px]",
                            class:"hidden md:block font-helvetica font-thin
                            text-left",
                            "Estimate in minutes"
                        }
                        span{
                            // style:"background:yellow; ",
                            class:"h-16 text-[24px] sm:text-[32px] md:text-[40px] lg:text-[48px] xl:text-[54px]",
                            class:"hidden md:block font-helvetica font-thin text-left md:m-0 md:p-0 ",
                            "with ",


                            span {
                                class:"h-16 text-[24px] sm:text-[32px] md:text-[40px] lg:text-[48px] xl:text-[54px]",
                                class: "font-helvetica font-bold underline",
                                    style:"letter-spacing:-3px;",
                                    "doxle"
                                }
                        }
                        span{
                            class:"md:text-[16px] font-[300] opacity-100 ",
                            class:"hidden md:block font-helvetica text-left",
                            style:"margin-top:2rem;",
                            "Upload your building plans and get a personalised budget delivered straight to your inbox!
                            Simplify your project planning & verify with your builder today. (We’re in beta!)"
                        }
                        // LEFT - 2 BUTTONS
                        div{
                            class:"hidden md:flex md:flex-row items-center justify-start space-x-2",
                            style:"margin-top:2rem;",
                            // button{
                            //     class:"p-4 border border-black font-helvetica font-[200] text-[16px] bg-black text-white
                            //     hover:bg-blue-700",
                            //     "Upload Plans"
                            // }

                            Link {
                                class:"p-4 border border-black font-helvetica font-[300]  text-[16px] bg-black text-white
                                hover:bg-blue-700 cursor-pointer",
                                to: Route:: Upload {},
                                "Upload Plans"

                            }
                            div{
                                class:"m-0 p-0 flex flex-row bg-red-0 border border-[rgb(45,45,49)] cursor-pointer
                                justify-start items-center space-x-2 p-4
                                ",
                                img {
                                    class:"h-5 w-5",
                                    src:"/assets/coffee.svg",
                                }
                                Link{
                                    style:"cursor:pointer;",
                                    class:"font-helvetica font-[300] text-[16px]
                                    hover:font-[400] ",
                                    to: Route:: Coffee {},
                                    "Buy us a coffee"
                                }
                            }


                        }
                    }

                }
            }


            //------------------------------------
            // RIGHT OR BOTTOM CONTAINER
            div{
                // style:"background:yellow; opacity:0.1;",

                class: "mt-0 h-[40%] md:m-0 md:p-0 md:w-[50%] md:h-[100vh] md:h-screen
                        md:bg-[url('/assets/page_grid.svg')] md:bg-cover md:bg-no-repeat
                        items-center justify-center flex border-t-[1px] border-[rgb(230,230,235)]
                        md:border-[0px]
                        ",
                div{
                    class:" md:w-full md:h-screen  md:z-1 md:w-[100%] flex items-center justify-center ",
                    // md:bg-[url('/assets/geometry.svg')] md:bg-no-repeat
                    // md:bg-cover md:bg-[contain,80%_80%] md:bg-center

                    img {
                        class:"hidden md:block md:w-full md:z-20",
                        src:"/assets/geometry.svg"
                    }

                }


                // MOBILE=BOTTOM; MD=HIDDEN
                div{
                    style:"margin-left:0.1rem;",
                    class:"w-full h-full md:hidden mt-4 md:mt-0  ",
                    span{
                    class:"font-helvetica font-thin text-[40px]",
                    "Estimate in minutes "
                    }
                    span{
                        style:"margin-top:-1rem;",
                        class:"font-helvetica font-thin text-[40px] block",
                        "with "
                        span {
                                style:"margin-top:-1rem;letter-spacing:-3px;",
                                class: "font-helvetica font-bold text-[40px] underline",
                                "doxle"
                            }
                    }
                    span{
                        class:"text-[18px] ",
                        class:"font-helvetica font-[300] text-left ",
                        "Upload your building plans and get a personalised budget delivered straight to your inbox!
                        (We’re in beta!)"
                    }
                    // 2 BUTTONS MOBILE
                    div{
                        class:"flex flex-row items-center justify-start space-x-2",
                        style:"margin-top:2rem;",

                        Link {
                            class:"p-4 border border-black font-helvetica font-light text-[16px] bg-black text-white
                            hover:bg-blue-700 cursor-pointer",
                            to: Route:: Upload {},
                            "Upload Plans"

                        }
                        div{
                            class:"m-0 p-0 flex flex-row bg-red-0 border border-[rgb(45,45,49)] cursor-pointer
                            justify-start items-center space-x-2 p-4
                            ",
                            img {
                                class:"h-5 w-5",
                                src:"/assets/coffee.svg",
                            }
                            Link{
                                style:"cursor:pointer;",
                                class:"font-helvetica font-[300] text-[16px]
                                hover:font-[400] ",
                                to: Route:: Coffee {},
                                "Buy us a coffee"
                            }
                        }
                    }

                }


            }

        }
    }
}
