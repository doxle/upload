use dioxus::prelude::*;

#[component]
pub fn ThankYou() -> Element {
    rsx! {
        div{
             class: "w-full h-screen bg-grid flex flex-col items-center justify-center m-4",

             div{
                 class:"md:w-[45%] flex flex-col items-center justify-center",

                img{
                    class:"h-[54px]",
                    src:"/assets/dog-blue.svg"
                }

                span{
                        class:"mt-2 font-helvetica font-[200] text-[63px] text-center text-black",
                        "Thank You"
                }
                span{
                        class:"mt-2 font-helvetica font-[200] text-[21px] text-slate-800 text-black",
                        "We will email you a link with your measurements and budget. However, please
                        note that"

                        span{
                            class:"font-helvetica font-[400] text-[21px] text-center text-blue-700",
                            " doxle.ai "
                        }
                        span{
                            class:"font-helvetica font-[200] text-[21px] text-black ",
                            "is still in the research phase and may make errors. We recommend
                            reviewing the budget with your builder or a quantity surveyor to ensure accuracy."
                        }
                }
                span{
                    class:"mt-4 font-helvetica font-[200] text-[16px] text-center text-[rgb(63,63,72)]",
                    "[ you may close this page ]"
                }

             }
        }
    }
}
