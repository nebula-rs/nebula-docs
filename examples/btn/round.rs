use leptos::prelude::*;
use leptos_nebula::{Btn, BtnDesign};

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Btn round=true icon="i-lucide-check" aria_label="Confirmar" />
        <Btn round=true dense=true icon="i-lucide-check" aria_label="Confirmar (denso)" />
        <Btn dense=true label="Denso" design=BtnDesign::Outline />
    }
}
