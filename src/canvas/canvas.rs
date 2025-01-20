use dioxus::logger::*;
use dioxus::prelude::*;
use document::document;

// const JS_CANVAS: Asset = asset!("/src/canvas/canvas.rs");

#[component]
pub fn Canvas() -> Element {
    // let js_code = r#"
    //     const canvas = document.getElementById("drawingCanvas");
    //     const ctx = canvas.getContext("2d");

    //     let isDrawing = false;
    //     let startX = 0;
    //     let startY = 0;

    //     function getTouchPosition(canvas, touchEvent) {
    //         const rect = canvas.getBoundingClientRect();
    //         const touch = touchEvent.touches[0] || touchEvent.changedTouches[0];
    //         return {
    //           x: touch.clientX - rect.left,
    //           y: touch.clientY - rect.top
    //         };
    //     }

    //     //WEB
    //     canvas.addEventListener('mousedown', (event) => {
    //       isDrawing = true;
    //       startX = event.offsetX;
    //       startY = event.offsetY;
    //       console.log("mousedown");
    //     });

    //     //MOBILE
    //     canvas.addEventListener('touchstart', (event) => {
    //       isDrawing = true;
    //       const touchPos = getTouchPosition(canvas, event)
    //       startX = touchPos.offsetX;
    //       startY = touchPos.offsetY;
    //       // TO PREVENT ANY SCROLLING
    //       event.preventDefault();
    //     });

    //     canvas.addEventListener('mousemove', event => {
    //       if (isDrawing) {
    //         const endX = event.offsetX;
    //         const endY = event.offsetY;
    //         ctx.beginPath();
    //         ctx.moveTo(startX, startY);
    //         ctx.lineTo(endX, endY);
    //         ctx.stroke();
    //         startX = endX;
    //         startY = endY;
    //       }
    //     });

    //     canvas.addEventListener('touchmove', event => {
    //       if (isDrawing) {
    //         const touchPos = getTouchPosition(canvas, event);
    //         const endX = touchPos.x;
    //         const endY = touchPos.y;
    //         ctx.beginPath();
    //         ctx.moveTo(startX, startY);
    //         ctx.lineTo(endX, endY);
    //         ctx.stroke();
    //         startX = endX;
    //         startY = endY;

    //         event.preventDefault();
    //       }
    //     });

    //     canvas.addEventListener('mouseup', () => {
    //       isDrawing = false;
    //     });

    //     canvas.addEventListener('mouseleave', () => {
    //       isDrawing = false;
    //     });

    //     canvas.addEventListener('touchend', () => {
    //       isDrawing = false;
    //     });

    //     canvas.addEventListener('touchcancel', () => {
    //       isDrawing = false;
    //     });

    //     "#;

    use_effect(move || {
        // document::eval(&js_code);
    });
    rsx! {

        // body {
        div { class: "md:w-full md:h-screen  ml-[1px] mr-[1px] bg-blue-50",
            div { class: "font-helvetica font-thin text-[50px]",
                "Canvas"

                dialog {

                }

                canvas {
                    class: "md:w-full md:h-screen bg-blue-100",
                    id: "drawingCanvas",
                    onmousemove: move |evt| {
                        println!("pmouse move {:#?}", evt);
                        tracing::info!("mouse move: {:#?}", evt);
                    },
                    onmousedown: move |evt| {
                        println!("pmouse down {:#?}", evt);
                        tracing::info!("mouse down: {:#?}", evt);
                        let js_code = include_str!("./canvas1.js");
                        document::eval(js_code);
                    },
                                // class:"bg-blue-50  w-full"
                }
            }
                // }
        // document::Link { rel: "stylesheet", href: JS_CANVAS }

        }
    }
}

#[component]
pub fn ReadDomain() -> Element {
    let mut domain_name = use_signal(|| String::new());
    rsx! {
        head {

            script { src: "./canvas.js" }
        }
        div { class: "mt-20 flex flex-col w-full h-screen ",
            p { class: "mt-10",
                "Domain name: "
                span { "{domain_name}" }
            }
            button {
                class: "bg-blue-500 w-[100px]",
                onclick: move |_| async move {
                    domain_name
                        .set(document::eval("return document.domain").await.unwrap().to_string());
                },
                "Read Domain"
            }
        }
    }
}
