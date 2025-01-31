use dioxus::prelude::*;
mod canvas;
use canvas::canvas::Canvas;
mod components {
    pub mod dialog;
    // pub mod pulse_effect;
}
// use components::navbar::Navbar;
mod home;
use home::home::Home;
mod coffee;
use coffee::coffee::Coffee;
mod upload;
use upload::upload::Upload;
mod legal;
use legal::legal::Legal;
mod jot;
use jot::jot::Jot;
mod service;
mod user_form;
use user_form::user_form::UserForm;
mod verification;
use verification::verification::Verification;

#[derive(Debug, Clone, Routable, PartialEq)]

enum Route {
    // #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/jot")]
    Coffee {},
    #[route("/coffee")]
    Jot {},
    #[route("/canvas")]
    Canvas {},
    #[route("/upload")]
    Upload {},
    #[route("/legal")]
    Legal {},
    #[route("/user_form")]
    UserForm {},
    #[route("/verification")]
    Verification {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Clone)]
pub struct ThemeContext {
    pub current_theme: Signal<Theme>,
    // pub toggle_theme: Rc<dyn Fn()>,
}

// const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const LOTTIE_DOG: Asset = asset!("/assets/dog.json");
// const FONT_100: Asset = asset!("/assets/HelveticaNeue-Thin.woff2");
// const FONT_200: Asset = asset!("/assets/HelveticaNeue-UltraLight.woff2");
// const FONT_200_ITALIC: Asset = asset!("/assets/HelveticaNeue-UltraLightItalic.woff2");
// const FONT_300: Asset = asset!("/assets/HelveticaNeue-Light.woff2");
// const FONT_700: Asset = asset!("/assets/HelveticaNeue-Bold.woff2");
// const FONT_100: Asset = asset!("/assets/HelveticaNeue-Thin.ttf");
// const FONT_200: Asset = asset!("/assets/HelveticaNeue-UltraLight.ttf");
// const FONT_200_ITALIC: Asset = asset!("/assets/HelveticaNeue-UltraLightItalic.ttf");
// const FONT_300: Asset = asset!("/assets/HelveticaNeue-Light.ttf");
// const FONT_700: Asset = asset!("/assets/HelveticaNeue-Bold.ttf");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let theme = use_signal(|| Theme::Light);

    use_context_provider(|| ThemeContext {
        current_theme: theme,
    });

    // Dynamically set the `dark` class on the <html> element
    let root_class = match *theme.read() {
        Theme::Dark => "dark",
        Theme::Light => "",
    };

    // Inject the `dark` class into the <html> tag
    // dioxus::desktop::WebView::current()
    //     .expect("WebView not found")
    //     .document()
    //     .unwrap()
    //     .document_element()
    //     .unwrap()
    //     .set_class_name(root_class);
    let bg_class = if *theme.read() == Theme::Dark {
        "bg-[rgb(24,26,27)]"
    } else {
        "bg-[rgb(245,245,245)]"
    };

    rsx! {
        head {
               meta { name: "viewport", content: "width=device-width, initial-scale=1, viewport-fit=cover" }
           }
        div {
            class: format!(
                "{} m-0 p-0 h-screen w-full overflow-hidden flex flex-col items-center justify-start {} space-y-10",
                root_class,
                bg_class,
            ),
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }
            document::Link { rel: "stylesheet", href: LOTTIE_DOG }
            // document::Link { rel: "stylesheet", href: MAIN_CSS }
            // document::Link { rel: "stylesheet", href: FONT_100 }
            // document::Link { rel: "stylesheet", href: FONT_200 }
            // document::Link { rel: "stylesheet", href: FONT_200_ITALIC }
            // document::Link { rel: "stylesheet", href: FONT_300 }
            // document::Link { rel: "stylesheet", href: FONT_700 }
            Router::<Route> {}

        }
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

// /// Shared navbar component.
// #[component]
// fn Wrapper() -> Element {
//     rsx!(
//         div{
//             class: "wrapper-container flex flex-col  min-h-screen w-full ",
//             // style: "background-color: green;",
//             header {
//                 Header {}
//             }
//             main {
//                 class: "flex-1 flex flex-col justify-center items-center ",
//                 Router::<Route> {}
//             }
//             footer {
//                 Footer {}
//             }
//         }
//     )
// }
