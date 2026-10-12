---
title: Btn
description: Botão, equivalente ao QBtn do Quasar.
examples: btn
order: 1
---

# Btn

Botão. Equivale ao `QBtn` do Quasar. Com `href`, renderiza um `<a>`.

```rust
use leptos_nebula::{Btn, Color};

view! { <Btn label="Salvar" color=Color::Positive /> }
```

> [!NOTE]
> Esta é uma implementação **parcial**: só as props listadas em [API](#api) existem. As demais do Quasar (`size`, `loading`, `to`, `push`…) entram depois, sem quebrar o que já existe.

## Cores

Use `color` com um papel de cor do tema.

<DocExample file="colors" />

## Designs

`design` troca a aparência: cheio (`Unelevated`, o padrão), contornado (`Outline`) ou sem fundo (`Flat`).

<DocExample file="designs" />

## Ícones

`icon` e `icon_right` recebem a classe literal do ícone lucide.

<DocExample file="icons" />

## Redondo e denso

`round` faz um botão circular (use `aria_label`, já que não há texto) e `dense` o compacta.

<DocExample file="round" />

## Cliques e estado desabilitado

<DocExample file="click" />

## Link

Com `href`, o botão é um `<a>`. Links internos são tratados pelo `Router` do app; use `target` para abrir em outra aba.

<DocExample file="link" />

## API

<DocApi file="Btn" />
