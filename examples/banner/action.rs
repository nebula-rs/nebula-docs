use leptos::prelude::*;
use leptos_nebula::{Banner, BannerAction, Btn, BtnDesign, Color};

#[component]
pub fn Example() -> impl IntoView {
    let visible = RwSignal::new(true);
    view! {
        <Show when=move || visible.get()>
            <Banner color=Color::Warning rounded=true>
                "Há uma versão nova disponível."
                <BannerAction slot:action>
                    <Btn
                        label="Dispensar"
                        design=BtnDesign::Flat
                        dense=true
                        on_click=move |_| visible.set(false)
                    />
                </BannerAction>
            </Banner>
        </Show>
    }
}
