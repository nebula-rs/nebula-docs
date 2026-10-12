use leptos::prelude::*;
use leptos_nebula::{Btn, BtnDesign};

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Btn label="Unelevated" design=BtnDesign::Unelevated />
        <Btn label="Outline" design=BtnDesign::Outline />
        <Btn label="Flat" design=BtnDesign::Flat />
    }
}
