use crate::Route;
use dioxus::prelude::*;
use gloo_timers::callback::Timeout;

#[component]
pub fn Verification() -> Element {
    let status = use_signal(|| String::from("Verifying"));
    let is_routing = use_signal(|| false);

    let start_timer = move || {
        let mut status = status.clone();
        let mut is_routing = is_routing.clone();

        // let navigator = use_navigator();

        Timeout::new(2000, move || {
            status.set("Redirecting".to_string());

            Timeout::new(2000, move || {
                // navigator().push(Route::Home {});
                is_routing.set(true);
            })
            .forget();
        })
        .forget();
    };

    use_effect(move || {
        start_timer();
        if is_routing() {
            navigator().push(Route::UserForm {
                verify_code: "true".to_string(),
            });
        }
    });

    rsx! {
        div{
             class: "w-full h-screen bg-grid flex flex-col items-center justify-center m-4",
                if status() == "Verifying" {
                    div{
                         class:"w-40 h-40 bg-blue-500 rounded-full animate-pulse"
                     }
                }
                else{
                    div{
                         class:"w-40 h-40 bg-green-500 rounded-full animate-pulse"
                     }
                }

                span{
                     class:"mt-4 font-helvetica font-[200] text-[30px] text-center text-black",
                     "{status}"
                 }
        }
    }
}
