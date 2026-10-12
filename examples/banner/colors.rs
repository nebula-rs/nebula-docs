use leptos::prelude::*;
use leptos_nebula::{Banner, Color};

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Banner color=Color::Info rounded=true>"Uma informação para quem lê."</Banner>
        <Banner color=Color::Positive rounded=true>"Tudo certo."</Banner>
        <Banner color=Color::Warning rounded=true>"Atenção a este detalhe."</Banner>
        <Banner color=Color::Negative rounded=true>"Algo deu errado."</Banner>
    }
}
