use crate::ThemeContext;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let context = use_context::<ThemeContext>();
    println!("current theme : {:#?}", context.current_theme.read());

    rsx! {
        div{
            class:"w-full h-screen flex flex-col md:flex-row",
            // LEFT OR TOP CONTAINER
            div{
                class:" md:w-[50%] h-[60%] md:h-[100%] bg-[url('/assets/page_grid.svg')]
                bg-cover bg-[length:94%] bg-no-repeat bg-center   md:bg-none  md:relative ",
                div{
                    class:" md:w-full h-full  bg-[url('/assets/geometry.svg')] bg-cover  bg-no-repeat bg-center
                    bg-[length:95%] mt-[100px] md:mt-[0px] md:relative md:bg-none",
                    // style:"z-index:1;",
                    span{
                        class:"text-[24px] sm:text-[32px] md:text-[40px] lg:text-[48px] xl:text-[54px]",
                        class:"hidden md:block font-helvetica font-thin
                        text-left md:m-0 md:p-0 md:absolute md:bottom-[30%] md:left-10 md:right-0 ",
                        "Estimate in minutes"
                    }
                    span{
                        class:"text-[24px] sm:text-[32px] md:text-[40px] lg:text-[48px] xl:text-[54px]",
                        class:"hidden md:block font-helvetica font-thin text-left md:m-0 md:p-0
                        md:absolute md:bottom-[23%] 2xl:bottom-[25%] md:left-10 md:right-0",
                        "with "
                        span {
                            class:"text-[24px] sm:text-[32px] md:text-[40px] lg:text-[48px] xl:text-[54px]",
                            class: "font-helvetica font-bold underline",
                                style:"letter-spacing:-3px;",
                                "doxle"
                            }
                    }
                    span{
                        class:"md:text-[21px]",
                        class:"hidden md:block font-helvetica font-thin  md:text-[21px] lg:text-[24px] 2xl:text-[30px]
                        text-left md:m-0 md:p-0 md:absolute md:bottom-[11%] lg:bottom-[13%] xl:bottom-[15%] 2xl:bottom-[16%] md:left-10 md:right-0",
                        "Upload your building plans and get a personalised budget delivered straight to your inbox!
                        Simplify your project planning—verify with your builder today. (We’re in beta!)"
                    }
                }
            }
            // RIGHT OR BOTTOM CONTAINER
            div{
                class:" p-[10px] md:m-0 md:p-0 md:w-[50%]  h-[40%] md:h-[100vh]  md:bg-[url('/assets/geometry.svg'),url('/assets/page_grid.svg')]
                md:bg-cover md:bg-[length:100%] md:bg-no-repeat md:bg-center relative",
                span{
                    class:"font-helvetica font-thin text-[40px] md:hidden absolute bottom-[60%] left-5 right-0",
                    "Estimate in minutes "
                }
                span{
                    class:"font-helvetica font-thin text-[40px] md:hidden absolute bottom-[50%] left-5 right-0",
                    "with "
                    span {
                            class: "font-helvetica font-bold text-[40px] underline",
                            style:"letter-spacing:-3px;",
                            "doxle"
                        }
                }
                span{
                    class:"text-[18px] ",
                    class:"md:hidden font-helvetica font-thin text-left md:m-0 md:p-0 absolute bottom-[35%] left-5 right-0",
                    "Upload your building plans and get a personalised budget delivered straight to your inbox!
                    (We’re in beta!)"
                }

            }

        }
    }
}
