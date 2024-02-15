use leptos::*;
use web_sys::MouseEvent;

use crate::api;
use crate::commands;
use crate::components::*;

#[component]
pub fn App() -> impl IntoView {
    let quit = create_action(|_: &MouseEvent| async move { commands::quit().await });

    let quit_handler = move |ev: MouseEvent| quit.dispatch(ev);

    let _ = api::get_translation_of(
        "Hello World".to_string(),
        "en".to_owned(),
        "ptbr".to_owned(),
    );

    view! {
        <main class="container">
            <div>
                <Select value="12".to_owned() />
                <Select value="12".to_owned() />
            </div>
            <TextArea value="Hello World".to_owned() />
            // TODO transform in icon-only
            <button on:click=quit_handler>Quit</button>
        </main>
    }
}
