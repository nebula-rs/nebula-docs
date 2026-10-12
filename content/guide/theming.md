---
title: Temas
description: Tokens em três camadas, data-theme e data-mode.
order: 2
---

# Temas

Todo o visual do `leptos-nebula` vem de variáveis CSS (`--lx-…`), organizadas em três camadas. Trocar de tema é trocar o arquivo de tokens ativo.

## Tokens em três camadas

| Camada | Exemplos | Quem usa |
|---|---|---|
| Primitivos | `--lx-p-fjord-600`, `--lx-p-font-humanist` | Só os arquivos de tema |
| Semânticos | `--lx-color-primary`, `--lx-space-md`, `--lx-radius-md` | Seu CSS e os componentes |
| De componente | `--lx-btn-radius`, `--lx-card-shadow` | Cada componente |

Nos seus estilos, use os tokens **semânticos**: eles mudam com o tema e o modo.

```css
.minha-caixa {
  background: var(--lx-color-surface);
  color: var(--lx-color-on-surface);
  border: var(--lx-border-width) solid var(--lx-color-border);
  border-radius: var(--lx-radius-md);
  padding: var(--lx-space-md);
}
```

## Tema e modo

O tema ativo é o atributo `data-theme` do `<html>`; o modo é `data-mode`, com `light` ou `dark`. A store de tema cuida dos dois:

```rust
use leptos::prelude::*;
use leptos_nebula::{Mode, use_theme};

#[component]
fn ForcarEscuro() -> impl IntoView {
    let tema = use_theme();
    tema.mode.set(Mode::Dark);
}
```

- `Mode::Auto` segue `prefers-color-scheme` e reage quando o sistema muda.
- Sem `data-mode` (antes de a store rodar, por exemplo no HTML estático), o CSS já respeita `prefers-color-scheme`.

## Temas disponíveis

| Tema | `data-theme` | Situação |
|---|---|---|
| Scandi | `scandi` | Disponível, com modo claro e escuro |
| Material Design 3 | `material` | Planejado |
| Liquid Glass | `glass` | Planejado |

> [!NOTE]
> Um tema novo é só um arquivo de tokens que redefine os mesmos nomes. Se você precisa de um tema próprio no seu app, carregue um CSS de tokens extra **depois** do `UiStyles`.

## Realce de código

Os blocos de código desta documentação usam os tokens `--lx-code-*` (`text`, `comment`, `keyword`, `string`, `number`, `function`, `type`, `punct`), definidos por tema e por modo.
