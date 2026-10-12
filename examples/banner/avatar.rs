use leptos::prelude::*;
use leptos_nebula::{Banner, BannerAvatar, Color, Icon};

#[component]
pub fn Example() -> impl IntoView {
    view! {
        <Banner color=Color::Info rounded=true>
            <BannerAvatar slot:avatar>
                <Icon name="i-lucide-info" />
            </BannerAvatar>
            "Você está offline. As alterações serão enviadas quando a conexão voltar."
        </Banner>
    }
}
