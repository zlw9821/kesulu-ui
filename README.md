# kesulu-ui

A reusable, business-agnostic Leptos UI primitive library — a [shadcn/ui](https://ui.shadcn.com/)
port to Leptos (button, card, dialog, input, tabs, …). Depends only on `leptos`
and `leptos_router`; ships its own self-contained design tokens in
[`style/ui.css`](style/ui.css).

Extracted from the [kesulu](https://github.com/zlw9821/kesulu) project so it can
be shared across multiple Leptos / Tauri front-ends.

## Consuming it (sibling path dependency)

This crate is **not published to crates.io**. Consume it as a local path
dependency by checking it out next to the consuming project:

```
parent/
├── kesulu-ui/        ← this repo
├── kesulu/
└── your-tauri-app/
```

### 1. Cargo

```toml
# your-app/crates/<crate>/Cargo.toml  (adjust the `..` depth to your layout)
kesulu-ui = { path = "../../../kesulu-ui" }
```

Forward the rendering backend your app uses (exactly one is active in the final
binary):

```toml
[features]
csr = ["kesulu-ui/csr"]                                 # Tauri / client-side rendering
hydrate = ["leptos/hydrate", "kesulu-ui/hydrate"]       # Leptos SSR — wasm side
ssr = ["leptos/ssr", "kesulu-ui/ssr"]                   # Leptos SSR — server side
```

> **Leptos version lockstep.** `kesulu-ui` floats on `leptos = "0.8"`. As a path
> dependency it compiles in the same build graph as your app, so Cargo unifies
> leptos to a single version. Keep every consumer on the same `0.8.x` line — two
> different leptos instances will not type-check.

### 2. Tailwind CSS

The components are styled with Tailwind v4 utilities and `ui.css` carries the
design tokens. In your Tailwind entry file:

```css
@import "tailwindcss";
@import "tw-animate-css";                   /* node dep — see step 3 */
@import "../../kesulu-ui/style/ui.css";     /* adjust depth to your layout */
@source "../src/**/*.rs";                   /* scan YOUR app's components */
```

`ui.css` already declares `@source "./src/**/*.rs"` relative to itself, so
Tailwind scans this crate's components automatically wherever it is imported —
you only point it at your own sources.

### 3. node dependency

The components use [`tw-animate-css`](https://github.com/Wombosvideo/tw-animate-css)
utilities (shadcn's `animate-in` / `fade-in-0` / `zoom-in-95` / `slide-in-from-*`).
Because `ui.css` lives outside your project tree, it does **not** import the
package itself — a bare specifier would resolve from `ui.css`'s own directory,
which has no `node_modules`. Install it and add the `@import` to your own entry
CSS (shown in step 2):

```
pnpm add tw-animate-css
```

## Usage

```rust
use kesulu_ui::*;

view! { <Button>"Click me"</Button> }
```

## Conventions

Component authoring rules (variants over `cva`, HSL tokens, `class:` directives,
React → Leptos translation) live in [`CONVENTIONS.md`](CONVENTIONS.md).
