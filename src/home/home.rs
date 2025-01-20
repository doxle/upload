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
            // LEFT OR TOP CONTAINER
            div{
                style:"",
                class:" h-[60%] md:w-[50%] md:h-[100%] bg-[url('/assets/page_grid.svg')]
                bg-no-repeat md:bg-none bg-blue-50 bg-opacity-0 flex justify-center items-center",

                div{
                    style:"",
                    class:"ml-4 mt-24 md:m-0 md:hidden  w-full h-full flex",
                    // bg-[url('/assets/geometry.svg')]
                    // bg-no-repeat bg-cover bg-[contain,100%_100%] bg-center

                    img {
                        class:"w-full md:hidden  ",
                        src:"/assets/geometry.svg"
                    }
                }


                // FLEX FLEX-COL WITH THIRD ONE GROWING - HIDDEN FOR MOBILE SCREENS
                div{
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
                                hover:bg-blue-700",
                                to: Route:: Upload {},
                                "Upload Plans"

                            }
                            button{
                                class:"p-4 border border-[rgb(45,45,49)] font-helvetica font-[300] text-[16px]
                                hover:font-[400] ",
                                "About Us"
                            }

                        }
                    }

                }
            }


            //------------------------------------
            // RIGHT OR BOTTOM CONTAINER
            div{
                // style:format!(
                //         "
                //          margin:0px;
                //          padding:0px;
                //          background:rgba(213,215,217, 0.4);
                //          background-image: url('/assets/geometry.svg'), url('/assets/page_grid.svg');
                //          background-size: 81% auto, 100vw 100%;
                //          background-repeat: no-repeat, no-repeat;
                //          background-position: center, center;


                //          "

                //     ),
                // class:" p-[10px] h-[40%]  md:m-0 md:p-0 md:w-[50%] md:h-[100vh] relative aspect-[1/1]",
                // class: "p-[10px] h-[40%] md:m-0 md:p-0 md:w-[50%] md:h-[100vh] relative aspect-[1/1]
                //                     md:bg-[url('/assets/geometry.svg'),url('/assets/page_grid.svg')]
                //                     md:bg-[100%_auto,100%_100%] md:bg-[contain,100%_100%]
                //                     md:bg-no-repeat md:bg-center",
                //


                        class: "mt-8 h-[40%] md:m-0 md:p-0 md:w-[50%] md:h-[100vh] md:[100%]
                                md:bg-blue-50 md:bg-opacity-0  bg-[url('/assets/page_grid.svg')] bg-no-repeat bg-[auto_100%] z-0
                                items-center justify-center flex
                                ",

                        div{
                            class:" md:w-full md:h-screen  md:z-1 md:w-[100%] flex items-center justify-center",
                            // md:bg-[url('/assets/geometry.svg')] md:bg-no-repeat
                            // md:bg-cover md:bg-[contain,80%_80%] md:bg-center

                            img {
                                class:"hidden md:block md:w-full ",
                                src:"/assets/geometry.svg"
                            }
                        }

                        // Background for page_grid (it covers the full div)
                        // div {
                        //     class: " w-full h-full bg-[url('/assets/page_grid.svg')] bg-[100%_100%] bg-no-repeat bg-center z-0"
                        // },

                        // // Background for geometry.svg (it stays centered and within the 50% width)
                        // div {
                        //     class: " w-[50%] h-full bg-[url('/assets/geometry.svg')] bg-[100%_auto] bg-no-repeat bg-center z-10"
                        // },

                // RIGHT - HIDDEN FOR MD SCREENS
                div{
                    style:"margin-left:1rem;",
                    class:"w-full h-full md:hidden mt-4 md:mt-0",
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
                        class:"font-helvetica font-[200] text-left",
                        "Upload your building plans and get a personalised budget delivered straight to your inbox!
                        (We’re in beta!)"
                    }
                    // 2 BUTTONS
                    div{
                        class:"flex flex-row items-center justify-start space-x-2",
                        style:"margin-top:2rem;",
                        // button{
                        //     class:"p-3 border border-black font-helvetica font-light text-[16px] bg-black text-white
                        //     hover:bg-blue-700",
                        //     "Upload Plans"
                        // }

                        Link {
                            class:"p-3 border border-black font-helvetica font-light text-[16px] bg-black text-white
                            hover:bg-blue-700",
                            to: Route:: Upload {},
                            "Upload Plans"

                        }
                        button{
                            class:"p-3 border border-[rgb(45,45,49)] font-helvetica font-light text-[16px]
                            hover:font-[400]",
                            "About Us"
                        }
                    }

                }


            }

        }
    }
}
