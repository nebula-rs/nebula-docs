---
title: Primeiros passos
description: Instale o leptos-nebula, carregue o CSS e use o primeiro componente.
order: 1
---

# Primeiros passos

O `leptos-nebula` é uma biblioteca de componentes para [Leptos](https://leptos.dev) 0.8. Os componentes seguem a API do [Quasar](https://quasar.dev) e todo o visual vem de **tokens CSS**, então um tema novo não exige mudar nenhum componente.

::: warning Ainda não publicado
O `leptos-nebula` ainda não está no crates.io. Por enquanto, use-o por caminho ou por git.
:::

## Instalação

No `Cargo.toml` do seu app:

```toml
[dependencies]
leptos = { version = "0.8", features = ["csr"] }
leptos_meta = "0.8"
leptos-nebula = { path = "../leptos-nebula/apps/leptos-nebula" }
```

O crate não escolhe `csr`, `hydrate` ou `ssr`: quem decide é o seu app. Todos os componentes são seguros para renderização no servidor.

## Carregue o CSS e o tema

Na raiz do app, chame `provide_meta_context()` e `provide_theme()` e coloque `UiStyles` uma vez:

```rust
use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_nebula::{Btn, UiStyles, provide_theme};

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    provide_theme();
    view! {
        <UiStyles />
        <Btn label="Olá" />
    }
}
```

`UiStyles` injeta o CSS do crate (reset, tokens, tema e estilos dos componentes) num `<style>` do `<head>`. Não há arquivo para configurar.

> [!TIP]
> `provide_theme()` guarda a escolha de tema e de modo (claro, escuro ou automático) no `localStorage` e mantém `data-theme` e `data-mode` no `<html>`. Use `ThemeSwitcher` para alternar.

## Usar as classes no seu markup (opcional)

Se você usa [UnoCSS](https://unocss.dev) e quer os mesmos atalhos (`lx-btn`…), as cores (`bg-lx-primary`…) e os ícones lucide no **seu** markup, adicione o preset ao `uno.config.ts`:

```ts
import { defineConfig, presetWind3 } from 'unocss'
import { presetUi } from 'leptos-nebula/preset'

export default defineConfig({
  presets: [presetWind3({ preflight: false }), presetUi()],
})
```

O app gera só o CSS do que ele mesmo usa. Os ícones são classes literais `i-lucide-<nome>`; o UnoCSS lê o código-fonte como texto, então escreva o nome inteiro, sem montá-lo com `format!`.

## Próximos passos

- [Temas](./theming.md): como os tokens e os temas funcionam.
- [Btn](../components/btn.md) e [Banner](../components/banner.md): os primeiros componentes documentados.
