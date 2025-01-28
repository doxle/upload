use dioxus::prelude::*;
use dioxus_motion::prelude::*;

#[component]
pub fn PulseEffect() -> Element {
    let mut scale = use_motion(1.0f32);

    use_effect(move || {
        scale.animate_to(
            1.8,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 100.0,
                damping: 5.0,
                mass: 0.5,
                velocity: 1.0,
            }))
            .with_loop(LoopMode::Infinite),
        );
    });

    use_drop(move || {
        scale.stop();
    });

    rsx! {
        div {
            class: "w-10 h-10 bg-blue-500 rounded-full",
            style: "transform: scale({scale.get_value()})"
        }
    }
}
