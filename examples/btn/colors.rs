use leptos::prelude::*;
use leptos_nebula::{Btn, Color};

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Btn label="Primary" color=Color::Primary />
        <Btn label="Secondary" color=Color::Secondary />
        <Btn label="Positive" color=Color::Positive />
        <Btn label="Negative" color=Color::Negative />
        <Btn label="Info" color=Color::Info />
        <Btn label="Warning" color=Color::Warning />
        <Btn label="Neutral" color=Color::Neutral />
    }
}
