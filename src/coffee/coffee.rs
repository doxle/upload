use dioxus::logger::tracing::info;
use dioxus::prelude::*;

#[component]
pub fn Coffee() -> Element {
    let status = use_signal(|| String::from("Thank you"));
    let mut selected_value = use_signal(|| String::new());
    let mut another_amount = use_signal(|| 0);

    rsx! {
        div{
             class: "w-full h-screen flex flex-col items-center justify-start overflow-y-auto",

               div{
                    class:"w-[90%] md:w-[50%] h-[150vh]  bg-red-0 items-center",
                    //THANK YOU
                    div{
                        class:"mt-[5%]  flex flex-row items-center justify-center bg-cyan-00 space-x-4",
                        img{
                            class:"flex items-center",
                            src:"/assets/dog-blue.svg"
                        }
                        span{
                             class:"font-helvetica font-[200] text-[54px] text-blue-600 text-center ",
                             "{status}"
                         }
                    }
                    //LARGE TEXT
                    div{
                        class:"mt-[2%] w-[100%] flex items-center justify-start ",
                        span{
                             class:"mt-4 font-helvetica font-[300] text-[14px] text-black",
                             "We are Doxle—a dedicated and passionate team with a bold vision: to create a fully automated estimating software for the
                             building and construction industry.

                             Our journey is deeply rooted in this industry; we’ve lived and breathed every part of it. Over the years,
                             we’ve witnessed firsthand the challenges that builders face, from navigating complex projects to managing
                             soaring costs, especially in recent times due to the pandemic. Driven by a genuine desire to make a
                             difference, we have been working relentlessly to simplify and streamline the estimating process.
                             Through extensive research and the application of advanced machine learning algorithms, we aim to reduce,
                             costs and the effort required to build accurately and efficiently.

                             Currently, Doxle is in its beta phase, and we’re actively collaborating with builders to refine and test our platform,
                             ensuring it delivers the impact we envision."

                             span{
                                 class:"mt-1 font-helvetica font-[300] text-[14px] block text-start",
                                 "As dog lovers ourselves, we believe in loyalty, dedication, and the joy of doing what we love—traits that fuel both our lives
                                 and our work at Doxle. After all, building a better world is something you can really sink your paws into! 🐾"
                             }
                         }

                    }
                   //ONLY DONATE
                   div{
                        class:"mt-4 flex flex-col md: w-[95%] h-[10%] font-[400] ",

                        span{
                            class:"mt-1 font-helvetica font-[300] text-[14px] block text-start text-newutral-950 italic",
                        "Only donate if you can afford it and would like us to keep innovating. Either way, we appreciate your support and thanks for
                        using Doxle :)"
                        }

                        // $10
                        div{
                           class:"mt-4 pl-0 flex flex-row bg-red-0 h-[100px] items-center justify-start space-x-4",
                           input {
                               class:"
                               appearance-none rounded-full w-6 h-6
                               border border-slate-600 checked:bg-blue-700
                               cursor-pointer focus:ring-2 focus:ring-blue-400
                               aspect-[1/1]
                               ",
                               r#type:"radio",
                               name:"amount",
                               value:"$10",
                               oninput:move|evt| {
                                   // println!("Checking value: {:?}", &evt.value());

                                  info!("radio1 clicked {:?}", evt.value());
                                  selected_value.set("$10".to_string());
                               }

                           }
                           if selected_value() != "$10" {
                               span{
                                   class:"font-helvetica font-[300] text-[24px] block text-start text-blue-600",
                               "$10"
                               }
                           }
                           else{
                               span{
                                   class:"font-helvetica font-[400] text-[24px] block text-start text-blue-600",
                               "$10"
                               }
                           }

                        }
                        // $25
                        div{
                           class:"mt-4 pl-0 flex flex-row bg-red-0 h-[100px] items-center justify-start space-x-4",
                           input {
                               class:"
                               appearance-none rounded-full w-6 h-6
                               border border-slate-600 checked:bg-blue-700
                               cursor-pointer focus:ring-2 focus:ring-blue-400
                               aspect-[1/1]
                               ",
                               r#type:"radio",
                               name:"amount",
                               value:"$25",
                               oninput:move|evt| {
                                   // println!("Checking value: {:?}", &evt.value());

                                   info!("radio1 clicked {:?}", evt);
                                   selected_value.set("$25".to_string());
                               }

                           }
                           if selected_value() != "$25" {
                               span{
                                   class:"font-helvetica font-[300] text-[24px] block text-start text-blue-600",
                               "$25"
                               }
                           }
                           else{
                               span{
                                   class:"font-helvetica font-[400] text-[24px] block text-start text-blue-600",
                               "$25"
                               }
                           }

                        }

                        // $50
                        div{
                           class:"mt-4 pl-0 flex flex-row bg-red-0 h-[100px] items-center justify-start space-x-4",
                           input {
                               class:"
                               appearance-none rounded-full w-6 h-6
                               border border-slate-600 checked:bg-blue-700
                               cursor-pointer focus:ring-2 focus:ring-blue-400
                               aspect-[1/1]
                               ",
                               r#type:"radio",
                               name:"amount",
                               value:"$50",
                               oninput:move|evt| {
                                   println!("Checking value: {:?}", &evt.value());
                                   selected_value.set("$50".to_string());
                               }

                           }
                           if selected_value() != "$50" {
                               span{
                                   class:"font-helvetica font-[300] text-[24px] block text-start text-blue-600",
                               "$50"
                               }
                           }
                           else{
                               span{
                                   class:"font-helvetica font-[400] text-[24px] block text-start text-blue-600",
                               "$50"
                               }
                           }

                        }

                        // $100
                        div{
                           class:"mt-4 pl-0 flex flex-row bg-red-0 h-[100px] items-center justify-start space-x-4",
                           input {
                               class:"
                               appearance-none rounded-full w-6 h-6
                               border border-slate-600 checked:bg-blue-700
                               cursor-pointer focus:ring-2 focus:ring-blue-400
                               aspect-[1/1]
                               ",
                               r#type:"radio",
                               name:"amount",
                               value:"$100",
                               oninput:move|evt| {
                                   println!("Checking value: {:?}", &evt.value());
                                   selected_value.set("$100".to_string());

                                   // *is_checked.write() = evt.value()
                               }

                           }
                           if selected_value() != "$100" {
                               span{
                                   class:"font-helvetica font-[300] text-[24px] block text-start text-blue-600",
                               "$100"
                               }
                           }
                           else{
                               span{
                                   class:"font-helvetica font-[400] text-[24px] block text-start text-blue-600",
                               "$100"
                               }
                           }

                        }

                        // CUSTOM
                        div{
                           class:"mt-4 pl-0 flex flex-row bg-red-0 h-[100px] items-center justify-start space-x-4",
                           input {
                               class:"
                               appearance-none rounded-full w-6 h-6
                               border border-slate-600 checked:bg-blue-700
                               cursor-pointer focus:ring-2 focus:ring-blue-400
                                aspect-[1/1]
                               ",
                               r#type:"radio",
                               name:"amount",
                               value:"custom",
                               oninput:move|evt| {
                                   println!("Checking value: {:?}", &evt.value());
                                   selected_value.set("custom".to_string());
                               }

                           }
                           if selected_value() != "custom" {
                               span{
                                   class:"font-helvetica font-[300] text-[24px] block text-start text-blue-600",
                               "Custom"
                               }
                           }
                           else{
                               span{
                                   class:"font-helvetica font-[400] text-[24px] block text-start text-blue-600",
                               "Custom"
                               }
                           }


                       }

                       if selected_value() == "custom" {

                           div{
                               input {
                                   r#type:"number",
                                   class:"p-4 w-[40%] h-[60px] bg-blue-50 border border-slate-300 font-helvetica font-[300] text-[18px]",
                                   name: "custom" ,
                                   value:"{another_amount}",
                                   placeholder:"$0",
                                   autofocus:true,
                                   // minlength:3,
                                   // maxlength:10,
                                   oninput:move |evt| {
                                       another_amount.set(evt.value().parse::<i32>().unwrap_or(0) );
                                   }
                               }
                           }
                       }
                       //BACK AND NEXT BUTTONS
                       div{
                           class:"mt-8 flex flex-row items-center justify-center space-x-2",
                           button {
                               onclick:  move |_evt| {
                                   navigator().go_back();

                               },
                               class:" w-[90px] h-[60px] p-3 border border-black font-helvetica font-[300] text-[16px]
                               hover:bg-blue-100 cursor-pointer flex items-center justify-center
                               ",
                               "Back"
                           }
                           button {
                               onclick:  move |evt| {
                                   evt.prevent_default();

                               },
                               class:" w-[90px] h-[60px] p-3 border border-black font-helvetica font-[300] text-[16px] bg-[rgb(45,45,49)] text-white
                               hover:bg-blue-700 cursor-pointer flex items-center justify-center
                               ",
                               "Next"
                           }
                       }


                   }

               }



        }
    }
}
