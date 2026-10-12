use leptos::prelude::*;
use leptos_nebula::{Btn, BtnDesign};

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Btn label="Guia" href="/guide/getting-started" design=BtnDesign::Outline />
        <Btn
            label="Quasar"
            href="https://quasar.dev"
            target="_blank"
            icon_right="i-lucide-arrow-right"
            design=BtnDesign::Flat
        />
    }
}
