use leptos::*;
use web_sys::MouseEvent;

use crate::commands;

#[component]
pub fn App() -> impl IntoView {
    let quit = create_action(|_: &MouseEvent| async move { commands::quit().await });

    let quit_handler = move |e: MouseEvent| quit.dispatch(e);

    view! {
        <main class="container">
            <div>
                <select name="input_idiom" id="input_idiom">
                    <option value="ptbr">Auto</option>
                    <option value="ptbr">Portuguese</option>
                    <option value="en">English</option>
                </select>
                <select name="input_idiom" id="input_idiom">
                    <option value="ptbr">Auto</option>
                    <option value="ptbr">Portuguese</option>
                    <option value="en">English</option>
                </select>
            </div>
            // TODO transform in icon-only
            <button on:click=quit_handler>Quit</button>
        </main>
    }
}
