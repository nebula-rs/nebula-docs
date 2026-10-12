---
title: Banner
description: Faixa de aviso, equivalente ao QBanner do Quasar.
examples: banner
order: 2
---

# Banner

Faixa de aviso com cor, ícone opcional e ações. Equivale ao `QBanner` do Quasar.

```rust
use leptos_nebula::{Banner, Color};

view! { <Banner color=Color::Info rounded=true>"Olá!"</Banner> }
```

> [!NOTE]
> Implementação **parcial**: não existem `inline_actions` nem `dark`. A cor padrão é `Neutral` (o Quasar não tem cor padrão).

## Cores

<DocExample file="colors" layout="column" />

## Avatar

O slot `avatar` fica à esquerda do conteúdo.

<DocExample file="avatar" layout="column" />

## Ações

O slot `action` fica à direita; use-o para botões.

<DocExample file="action" layout="column" />

## API

<DocApi file="Banner" />
