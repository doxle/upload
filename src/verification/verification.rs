use crate::{service::service::send_email_request, Route};
use dioxus::logger::tracing::info;
use dioxus::prelude::*;
// use dioxus_router::prelude::FromQuery;
use gloo_timers::callback::Timeout;

// #[derive(Default, Props, Debug, Clone, PartialEq)]
// pub struct VerificationProps {
//     pub name: String,
//     pub email: String,
//     pub mobile: String,
// }

// impl Display for VerificationProps {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "name={}&email={}&mobile={}",
//             self.name, self.email, self.mobile
//         )
//     }
// }

/// The query segment is anything that implements https://docs.rs/dioxus-router/latest/dioxus_router/routable/trait.FromQuery.html. You can implement that trait for a struct if you want to parse multiple query parameters.
// impl FromStr for VerificationProps {
//     type Err = String;
//     fn from_str(query: &str) -> Result<Self, Self::Err> {
//         let mut name = None;
//         let mut email = None;
//         let mut mobile: Option<String> = None;

//         let pairs = form_urlencoded::parse(query.as_bytes());

//         for (key, value) in pairs {
//             match key.as_ref() {
//                 "name" => name = Some(value.into_owned()),
//                 "email" => email = Some(value.into_owned()),
//                 "mobile" => mobile = Some(value.into_owned()),
//                 _ => {}
//             }
//         }

//         Ok(Self {
//             name: name.unwrap_or_default(),
//             email: email.unwrap_or_default(),
//             mobile: mobile.unwrap_or_default(),
//         })
//     }
// }

#[component]
pub fn Verification(name: String, email: String, mobile: String) -> Element {
    let status = use_signal(|| String::from("Verifying"));
    let is_routing = use_signal(|| false);
    spawn(async move {
        let response = send_email_request(name, email, mobile).await;
        match response {
            Ok(json) => info!("Json response {:?}", json),
            Err(e) => info!("Err {:?}", e),
        }
    });

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
            navigator().push(Route::Home {});
        }
    });

    rsx! {
        div{
             class: "w-full h-screen bg-grid flex flex-col items-center justify-center m-4",
                if (status() == "Verifying") {
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
