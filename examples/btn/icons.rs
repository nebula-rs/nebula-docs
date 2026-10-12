use leptos::prelude::*;
use leptos_nebula::{Btn, BtnDesign};

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Btn label="Copiar" icon="i-lucide-copy" />
        <Btn label="Próximo" icon_right="i-lucide-arrow-right" design=BtnDesign::Outline />
        <Btn label="Pronto" icon="i-lucide-check" icon_right="i-lucide-info" design=BtnDesign::Flat />
    }
}
