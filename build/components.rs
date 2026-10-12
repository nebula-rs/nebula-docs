//! Tags do site, aceitas no Markdown: `<DocExample />` e `<DocApi />`.
//! Vivem aqui (e não no `leptos-press`) por serem específicas dos componentes documentados.

use leptos_press_build::{Attrs, Ctx, Error, MdComponent, highlight_html};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Só letras, números, `_` e `-` — o nome vira caminho de arquivo e identificador.
fn safe_name<'a>(what: &str, value: &'a str) -> Result<&'a str, Error> {
    if !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        Ok(value)
    } else {
        Err(Error::new(format!("{what} inválido: `{value}`")))
    }
}

fn ident_part(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}

/// `<DocExample file="colors" />` → `examples/<front-matter examples>/colors.rs`, compilado como
/// módulo (erro de compilação quebra o build) e mostrado com o código na base da caixa.
/// O arquivo exporta `#[component] pub fn Example()`.
///
/// Atributos: `file` (obrigatório); `title` (opcional — sem ele não há barra de título; use só
/// quando o cabeçalho acima não basta); `layout` = `row` (padrão, componentes inline como `Btn`)
/// ou `column` (componentes de bloco, como `Banner`).
pub struct DocExample;

struct Example {
    module: proc_macro2::Ident,
    abs_path: String,
    title: Option<String>,
    column: bool,
    source: String,
}

impl DocExample {
    fn load(attrs: &Attrs, ctx: &Ctx) -> Result<Example, Error> {
        let group = ctx.front.str("examples").ok_or_else(|| {
            Error::new("`<DocExample>` precisa de `examples: <pasta>` no front-matter")
        })?;
        let group = safe_name("valor de `examples`", group)?;
        let file = safe_name("atributo `file`", attrs.require("file")?)?;
        let title = attrs.get("title").map(str::to_string);
        let column = match attrs.get("layout") {
            None | Some("row") => false,
            Some("column") => true,
            Some(other) => {
                return Err(Error::new(format!(
                    "`layout` deve ser `row` ou `column`, não `{other}`"
                )));
            }
        };
        let relative = format!("examples/{group}/{file}.rs");
        let source = ctx.read_to_string(&relative)?;
        Ok(Example {
            module: format_ident!("ex_{}_{}", ident_part(group), ident_part(file)),
            abs_path: ctx.root.join(&relative).to_string_lossy().into_owned(),
            title,
            column,
            source: source.trim_end().to_string(),
        })
    }
}

impl MdComponent for DocExample {
    fn tag(&self) -> &'static str {
        "DocExample"
    }

    fn codegen(&self, attrs: &Attrs, ctx: &Ctx) -> Result<TokenStream, Error> {
        let Example {
            module,
            title,
            column,
            source,
            ..
        } = Self::load(attrs, ctx)?;
        let highlighted = highlight_html("rust", &source);
        let title = match title {
            Some(t) => quote! { ::core::option::Option::Some(#t) },
            None => quote! { ::core::option::Option::None },
        };
        Ok(quote! {
            crate::doc_example(
                #title,
                #column,
                || ::leptos::prelude::IntoAny::into_any(#module::Example()),
                ::leptos_press::code_block_attached("rust", #source, #highlighted),
            )
        })
    }

    fn items(&self, attrs: &Attrs, ctx: &Ctx) -> Result<TokenStream, Error> {
        let Example {
            module, abs_path, ..
        } = Self::load(attrs, ctx)?;
        Ok(quote! {
            #[path = #abs_path]
            mod #module;
        })
    }

    fn llms(&self, attrs: &Attrs, ctx: &Ctx) -> Result<String, Error> {
        let Example { title, source, .. } = Self::load(attrs, ctx)?;
        let code = format!("```rust\n{source}\n```");
        Ok(match title {
            Some(title) => format!("**{title}**\n\n{code}"),
            None => code,
        })
    }
}

/// `<DocApi file="Btn" />` → tabelas de props, slots e eventos de `api/Btn.json`.
///
/// Formato (subconjunto dos `Q*.json` do Quasar; só o que está **implementado**):
/// `props.<nome> { type, desc, default?, values? }`, `slots.<nome> { desc }`,
/// `events.<nome> { desc }`.
pub struct DocApi;

struct Section {
    id: String,
    title: &'static str,
    head: Vec<&'static str>,
    rows: Vec<Vec<String>>,
}

impl DocApi {
    fn load(attrs: &Attrs, ctx: &Ctx) -> Result<Vec<Section>, Error> {
        let file = safe_name("atributo `file`", attrs.require("file")?)?;
        let relative = format!("api/{file}.json");
        let text = ctx.read_to_string(&relative)?;
        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| Error::new(format!("{relative} inválido: {e}")))?;
        let id = ident_part(file);
        let text_of = |v: &serde_json::Value, key: &str| {
            v.get(key)
                .and_then(serde_json::Value::as_str)
                .unwrap_or("")
                .to_string()
        };
        let mut sections = Vec::new();

        if let Some(props) = json.get("props").and_then(serde_json::Value::as_object) {
            let rows = props
                .iter()
                .map(|(name, p)| {
                    let mut desc = text_of(p, "desc");
                    if let Some(values) = p.get("values").and_then(serde_json::Value::as_array) {
                        let list: Vec<String> = values
                            .iter()
                            .filter_map(serde_json::Value::as_str)
                            .map(|v| format!("`{v}`"))
                            .collect();
                        desc = format!("{desc} Valores: {}.", list.join(", "));
                    }
                    let default = p
                        .get("default")
                        .map(|d| d.as_str().map_or_else(|| d.to_string(), str::to_string))
                        .map_or(String::new(), |d| format!("`{d}`"));
                    vec![
                        format!("`{name}`"),
                        format!("`{}`", text_of(p, "type")),
                        default,
                        desc,
                    ]
                })
                .collect();
            sections.push(Section {
                id: format!("api-{id}-props"),
                title: "Props",
                head: vec!["Prop", "Tipo", "Padrão", "Descrição"],
                rows,
            });
        }
        for (key, title, head) in [("slots", "Slots", "Slot"), ("events", "Eventos", "Evento")] {
            if let Some(items) = json.get(key).and_then(serde_json::Value::as_object) {
                let rows = items
                    .iter()
                    .map(|(name, v)| vec![format!("`{name}`"), text_of(v, "desc")])
                    .collect();
                sections.push(Section {
                    id: format!("api-{id}-{key}"),
                    title,
                    head: vec![head, "Descrição"],
                    rows,
                });
            }
        }
        Ok(sections)
    }
}

/// Texto simples com `código` entre crases → HTML escapado.
fn inline_html(text: &str) -> String {
    let mut out = String::new();
    for (i, part) in text.split('`').enumerate() {
        let escaped = part
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;");
        if i % 2 == 1 {
            out.push_str(&format!("<code>{escaped}</code>"));
        } else {
            out.push_str(&escaped);
        }
    }
    out
}

impl MdComponent for DocApi {
    fn tag(&self) -> &'static str {
        "DocApi"
    }

    fn codegen(&self, attrs: &Attrs, ctx: &Ctx) -> Result<TokenStream, Error> {
        let mut views: Vec<TokenStream> = Vec::new();
        for s in Self::load(attrs, ctx)? {
            let heading = format!("<h4 id=\"{}\">{}</h4>\n", s.id, s.title);
            let mut table = String::from("<thead>\n<tr>");
            for h in &s.head {
                table.push_str(&format!("<th>{h}</th>"));
            }
            table.push_str("</tr>\n</thead>\n<tbody>\n");
            for row in &s.rows {
                table.push_str("<tr>");
                for cell in row {
                    table.push_str(&format!("<td>{}</td>", inline_html(cell)));
                }
                table.push_str("</tr>\n");
            }
            table.push_str("</tbody>\n");
            views.push(quote! { ::leptos_press::html(#heading) });
            views.push(quote! { ::leptos_press::table(#table) });
        }
        Ok(quote! { ::std::vec![ #( #views ),* ] })
    }

    fn llms(&self, attrs: &Attrs, ctx: &Ctx) -> Result<String, Error> {
        let mut out = String::new();
        for s in Self::load(attrs, ctx)? {
            out.push_str(&format!("#### {}\n\n", s.title));
            out.push_str(&format!("| {} |\n", s.head.join(" | ")));
            out.push_str(&format!("|{}\n", "---|".repeat(s.head.len())));
            for row in &s.rows {
                let cells: Vec<String> = row.iter().map(|c| c.replace('|', "\\|")).collect();
                out.push_str(&format!("| {} |\n", cells.join(" | ")));
            }
            out.push('\n');
        }
        Ok(out)
    }
}
