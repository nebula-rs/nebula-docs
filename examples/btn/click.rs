use leptos::prelude::*;
use leptos_nebula::Btn;

#[component]
pub fn Example() -> impl IntoView {
    let count = RwSignal::new(0);
    view! {
        <Btn
            label="Clique"
            on_click=move |_| count.update(|n| *n += 1)
        />
        <span>"Cliques: " {count}</span>
        <Btn label="Desabilitado" disable=true />
    }
}
