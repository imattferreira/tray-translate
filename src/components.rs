use leptos::{component, view, IntoView};

#[component]
pub fn Select(value: String) -> impl IntoView {
    println!("{}", value);

    view! {
      <select name="input_idiom" id="input_idiom">
        <option value="ptbr">Auto</option>
        <option value="ptbr">Portuguese</option>
        <option value="en">English</option>
      </select>
    }
}

#[component]
pub fn TextArea(value: String) -> impl IntoView {
    view! {
      <textarea name="" id="" cols="30" rows="10">{value}</textarea>
    }
}
