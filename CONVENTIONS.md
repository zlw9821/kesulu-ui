# `ui` crate — conventions & roadmap

A reusable, business-agnostic shadcn/ui port for Leptos v0.8. **No kesulu domain
types, no web-sys, no I/O** — deps are `leptos` + `leptos_router` only. This file
is the spec every component (and every fan-out agent) follows so the library stays
consistent as it grows.

## House rules (derive new components from `button.rs` / `card.rs`)

1. **One file per component family** under `src/`, `pub use`-flattened in `lib.rs`
   (so consumers write `use ui::Button;`). Module is private; types are `pub`.
2. **`#[component]` + `leptos::prelude::*`.** Match the shadcn class strings
   faithfully (query the shadcn MCP / `shadcn` skill before authoring).
3. **`class` passthrough is mandatory** for any component with a styleable root:
   `#[prop(optional, into)] class: String`, appended LAST so callers can override:
   `class=format!("<base classes> {}", class)` (or inside a `move ||` closure when
   the class depends on a signal). This is the project-blessed `cn()` analogue —
   do **not** introduce a separate merge helper.
4. **Variants = Rust enum + match → `&'static str`** (the `cva()` analogue).
   `#[derive(Default, Clone, Copy, PartialEq)]`, `#[default]` on the base variant,
   exposed via `#[prop(optional)] variant: XVariant`. See `ButtonVariant`.
5. **State = signals, not strings.** Inputs take `Signal<T>` / `RwSignal<T>` +
   `Callback<T>`; `data-[state=…]` becomes a signal-derived `class:`/closure.
6. **Tokens only** — colors via the `@theme` tokens in `style/ui.css`
   (`bg-primary`, `text-muted-foreground`, …). **Never hardcode hex.** If a
   component needs a new token, add it to `style/ui.css` (the CSS contract), not
   just to a consuming app's stylesheet.
7. **Runtime-agnostic / `Send`:** no `Rc`/`RefCell`/`spawn_local`. Browser-only
   bits (none today) would need `#[cfg(target_arch = "wasm32")]` guards.
8. **Empty strings over `Option`** for optional text props (`#[prop(optional,
   into)] label: String`, gate with `(!label.is_empty()).then(...)`) — matches the
   existing components.

## CSS contract (`style/ui.css`)

Self-contained: `@source "./src/**/*.rs"` (resolves against the file, so it works
wherever imported) + the `@theme` tokens every component references + the keyframes
they use (`fadeIn`, `slideUp`). A consumer wires the whole library with:

```css
@import "tailwindcss";
@import ".../crates/ui/style/ui.css";   /* tokens + component source scan */
```

kesulu's `style/main.css` does exactly this and keeps only app-specific tokens.

## Scope boundaries & dependencies (decided for the fan-out)

- **Source of truth:** translate from the real shadcn v4 source at
  `references/ui/apps/v4/registry/new-york-v4/ui/<component>.tsx` (complete, exact
  classes). The shadcn MCP is a discovery aid only — its `view` returns metadata,
  not full source, and `search` needs a project `components.json` (we have none).
- **Animations:** the CSS layer depends on **`tw-animate-css`** (installed), so
  components use shadcn's exact `animate-in` / `animate-out` / `fade-in-0` /
  `zoom-in-95` / `slide-in-from-*` classes — copy them verbatim from the source.
  **Limitation:** *enter* animations work as-is; *exit* animations
  (`data-[state=closed]:…`) need a presence/delayed-unmount helper because `<Show>`
  unmounts immediately — defer exit animations until that helper exists.
- **Icons:** inline the needed lucide SVG paths (as `checkbox`/`accordion` do).
  **No icon dependency.**
- **Charts:** **out of scope for `ui`** — charting needs wasm-bindgen/JS interop
  (kesulu uses lightweight-charts via `app`'s `chart_bindings.rs`) and shadcn's own
  Chart is just a Recharts theme wrapper. `ui` may provide a `Card` to frame a
  chart, never the chart engine. A separate `ui-charts` crate is a future option,
  not now.
- **Tables:** ship the shadcn **presentational primitives** only (`Table`,
  `TableHeader`, `TableBody`, `TableRow`, `TableHead`, `TableCell`, `TableCaption`)
  — zero deps. Data-grid behaviour (sort/filter/paginate/virtualize, the TanStack
  recipe) is **app-level composition**, not a `ui` component.

## Current inventory (read the source for exact props)

| Module | Components |
|---|---|
| `accordion` | `Accordion`, `AccordionItem`, `AccordionTrigger`, `AccordionContent` (single-open) |
| `alert` | `Alert` (`AlertVariant`), `AlertTitle`, `AlertDescription` |
| `button` | `Button` (`ButtonVariant`, `ButtonSize`) |
| `radio_group` | `RadioGroup`, `RadioGroupItem` |
| `card` | `Card`, `CardHeader`, `CardTitle`, `CardDescription`, `CardContent`, `CardFooter` |
| `input` | `TextInput`, `NumberInput`, `SelectInput`, `TextArea`, `Checkbox`, `Switch`, `Label` |
| `dialog` | `Dialog`, `DialogHeader`, `DialogTitle`, `DialogDescription`, `DialogFooter`, `AlertDialog` |
| `dropdown` | `DropdownMenu`, `DropdownMenuItem`, `DropdownMenuSeparator` |
| `tabs` | `Tabs`, `TabsList`, `TabsTrigger`, `TabsContent`, `RouteTabs`, `RouteTab` |
| `data_display` | `Badge` (`BadgeVariant`), `DataTable` (`Column`, `ColumnAlign`), `TableHead`, `KPICard`, `CurrencyDisplay`, `ExpandableRow`, `Progress` |
| `feedback` | `ErrorBanner`, `EmptyState`, `SkeletonBlock`, `Tooltip` |
| `layout` | `Separator`, `PageHeader`, `SectionTitle`, `ScrollArea` |
| `popover` | `Popover`, `PopoverTrigger`, `PopoverContent`, `PopoverHeader`, `PopoverTitle`, `PopoverDescription` |
| `select` | `Select`, `SelectTrigger`, `SelectValue`, `SelectContent`, `SelectLabel`, `SelectGroup`, `SelectItem`, `SelectSeparator` (distinct from `input::SelectInput`) |
| `sheet` | `Sheet`, `SheetContent`, `SheetHeader`, `SheetFooter`, `SheetTitle`, `SheetDescription` (`SheetSide`) |
| `avatar` | `Avatar`, `AvatarImage`, `AvatarFallback`, `AvatarBadge`, `AvatarGroup`, `AvatarGroupCount` |
| `breadcrumb` | `Breadcrumb`, `BreadcrumbList`, `BreadcrumbItem`, `BreadcrumbLink`, `BreadcrumbPage`, `BreadcrumbSeparator`, `BreadcrumbEllipsis` |
| `pagination` | `Pagination`, `PaginationContent`, `PaginationItem`, `PaginationLink`, `PaginationPrevious`, `PaginationNext`, `PaginationEllipsis` |
| `slider` | `Slider` (single-value, backed by a styled native `<input type=range>`) |
| `table` | `Table`, `TableHeader`, `TableBody`, `TableFooter`, `TableRow`, `TableHead` (the `<th>`), `TableCell`, `TableCaption` — shadcn primitives; the legacy `data_display::TableHead` (`<thead>` wrapper) was removed in favor of these |
| `toggle` | `Toggle`, `ToggleGroup`, `ToggleGroupItem` |
| `hover_card` | `HoverCard`, `HoverCardTrigger`, `HoverCardContent` |
| `collapsible` | `Collapsible`, `CollapsibleTrigger`, `CollapsibleContent` |
| `aspect_ratio` | `AspectRatio` |
| `spinner` | `Spinner` |
| `kbd` | `Kbd`, `KbdGroup` |

## Roadmap (M2 work-list)

**a. Close `class`-passthrough gaps** (rule 3) on existing components — **done**:
`Badge`, `TableHead`, `ErrorBanner`, `EmptyState`, `Tooltip`, `PageHeader`,
`SectionTitle` now all take `#[prop(optional, into)] class`.

**b. Add high-frequency shadcn core components** — **done** (hand-written samples
`Alert`/`RadioGroup`/`Accordion`, then a workflow fan-out for: `popover`,
`select`, `sheet`, `avatar`, `breadcrumb`, `pagination`, `slider`, `table`,
`toggle`, `hover_card`, `collapsible`, `aspect_ratio`, `spinner`, `kbd`).
Known simplifications carried by the fan-out (all per the exit-animation +
no-Radix constraints): no exit animations (`<Show>` unmounts immediately); no
roving-focus/keyboard nav (click-driven, `hover:` instead of `focus:` on menu
items); Radix Portal/anchor floating-position dropped (popovers are statically
positioned). These are the natural next polish items.

**c. CSS cleanup (done in M2 step 1):** `main.css` now `@import`s `ui.css` — no
token duplication remains.

**d. Remaining / future:** signal-driven `Tooltip` (replace hover-only),
generalized `Toast` (app has one), and the JS-backed / heavier components
deliberately deferred: `Calendar`, `Command`/`Combobox`, `Sonner`, `Carousel`,
`Chart` (out of scope — see boundaries above). A **presence/delayed-unmount
helper** would unlock exit animations across popover/select/sheet/dialog.

When fanning out across agents: each agent authors **one new `src/<name>.rs`
only** and does **not** edit `lib.rs` or `ui.css`; the orchestrator merges the
module list and any new tokens afterward to avoid conflicts.
