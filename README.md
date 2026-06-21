# kesulu-ui

A reusable, business-agnostic Leptos UI primitive library — a [shadcn/ui](https://ui.shadcn.com/)
port to Leptos (button, card, dialog, input, tabs, …). Depends only on `leptos`
and `leptos_router`; ships its own self-contained design tokens.

Extracted from the [kesulu](https://github.com/zlw9821/kesulu) project so it can
be shared across multiple Leptos / Tauri front-ends.

## Repo layout

A virtual Cargo workspace:

```
kesulu-ui/
├── crates/
│   ├── kesulu-ui/      ← the primitive library (the shared artifact)
│   └── showcase/       ← living gallery / dev harness (CSR, Trunk)
└── templates/          ← (future) macOS / iOS Tauri+Leptos starter templates
```

`crates/*` are workspace members (one shared lockfile). `templates/` is
**excluded** — those are copied / `cargo-generate`d to bootstrap a new app, not
built in place.

## Consuming the library (sibling path dependency)

Not published to crates.io. Consume it as a local path dependency, checked out
next to the consuming project:

```
parent/
├── kesulu-ui/         ← this repo
├── kesulu/
└── your-tauri-app/
```

### 1. Cargo

```toml
# your-app's Cargo.toml  (adjust the `..` depth to your layout)
kesulu-ui = { path = "../../../kesulu-ui/crates/kesulu-ui", features = ["csr"] }
```

Forward exactly one rendering backend: `csr` (Tauri / client-side), or `ssr` +
`hydrate` for a Leptos SSR app. In a multi-crate consumer, define the dep once in
the workspace root and use `kesulu-ui.workspace = true` in members so the path
lives in a single place.

> **Leptos version lockstep.** `kesulu-ui` floats on `leptos = "0.8"`. As a path
> dependency it compiles in the same build graph as your app, so Cargo unifies
> leptos to a single version. Keep every consumer on the same `0.8.x` line — two
> different leptos instances will not type-check.

### 2. Node link (so the CSS resolves by name)

The library is also a tiny node package, so its CSS can be imported by name
instead of a brittle relative path. Add it next to the Tailwind toolchain:

```jsonc
// your-app/package.json
"dependencies": {
  "@tailwindcss/cli": "^4.3.0",
  "tailwindcss": "^4.3.0",
  "tw-animate-css": "^1.4.0",
  "kesulu-ui": "link:../../kesulu-ui/crates/kesulu-ui"  // ← the ONLY relative path; adjust depth
}
```

`pnpm install` symlinks it into `node_modules`. This one line is the only place
the sibling path appears.

### 3. Tailwind entry CSS

Then your entry CSS is **identical in every consumer** (copy verbatim):

```css
@import "tailwindcss";
@import "tw-animate-css";
@import "kesulu-ui";                              /* tokens, resolved by package name */
@source "../node_modules/kesulu-ui/src/**/*.rs"; /* scan the lib's components */
@source "../src/**/*.rs";                         /* scan YOUR components */
```

Why this exact shape (two Tailwind v4 facts): `@import` resolves node packages
(so `kesulu-ui` works by name, via the package's `exports`), but `@source` is a
filesystem glob that ignores `node_modules` — so the lib's source is scanned via
an explicit path into *your own* `node_modules` (stable wherever the lib lives).
Build it with a Trunk `pre_build` hook running
`tailwindcss -i style/main.css -o style/output.css`.

## Usage

```rust
use kesulu_ui::*;

view! { <Button>"Click me"</Button> }
```

## Running the showcase

A standalone CSR app rendering every primitive — no server, auth, or host app.
The fastest way to develop or eyeball components:

```
cd crates/showcase
pnpm install        # once
trunk serve         # → http://localhost:8080
```

## Conventions

Component authoring rules (variants over `cva`, HSL tokens, `class:` directives,
React → Leptos translation) live in [`CONVENTIONS.md`](CONVENTIONS.md).
