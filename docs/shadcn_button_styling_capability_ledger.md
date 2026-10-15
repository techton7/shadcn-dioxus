# shadcn-dioxus Button Styling Capability Ledger

**Task UID**: `01M4Z021F7KX8AJ94C88VHDVMT` (`validate-shadcn-dioxus-styling-parity`)  
**Domain**: `ui/shadcn-dioxus`  
**Reference Sources**:
- `shadcn-ui`: `reference/shadcn-ui/apps/v4/registry/new-york-v4/ui/button.tsx`
- `shadcn-svelte`: `reference/shadcn-svelte/docs/src/lib/registry/ui/button/button.svelte`

---

## 1. Executive Summary & Scope

This ledger defines the styling and component capability surface for `shadcn-dioxus::Button`, establishing an exhaustive reference model against upstream `shadcn-ui` (React) and `shadcn-svelte` implementations.

For Phase 7 parity verification, the scope is focused on the **static primary Button styling parity smoke slice**. This slice validates that Tailwind v4 styling utilities are applied consistently across Web (WASM / Chromium via Dioxus Web) and Native (Blitz Stylo CSS engine / Vello rasterizer via Dioxus Native).

---

## 2. Exhaustive Feature Taxonomy: shadcn Button

| Feature Domain | Reference Capability (`shadcn-ui` / `shadcn-svelte`) | Status (Phase 7) | Implementation / Deferral Details |
|---|---|---|---|
| **Base Element** | Native `<button type="button">` rendering with slot/children | **Adopted** | Rendered as `button { r#type: "button", class: "{combined_class}", "data-gui-test-id": props.gui_test_id, {props.children} }` |
| **Primary Variant Styling** | `bg-primary text-primary-foreground shadow-sm` | **Adopted** | Rendered with Tailwind v4 utilities: `bg-slate-900 text-white shadow-sm` |
| **Typography & Sizing** | `text-sm font-medium px-4 py-2 rounded-md` | **Adopted** | Rendered with Tailwind v4 utilities: `text-sm font-medium rounded-md px-4 py-2` |
| **Flex Layout & Alignment** | `inline-flex items-center justify-center gap-2 whitespace-nowrap` | **Adopted** | Rendered with Tailwind v4 utilities: `inline-flex items-center justify-center gap-2 whitespace-nowrap` |
| **Transitions** | `transition-colors` / `transition-all` | **Adopted** | Included in base utility string: `transition-colors` |
| **GUI Test Hook** | Selector hook for automated visual test capture | **Adopted** | `data-gui-test-id="button-primary"` attribute |
| **Custom Class Passthrough** | `class` prop concatenation | **Adopted** | Merged with `base_classes` via `props.class` |
| **Variant Matrix** | `secondary`, `destructive`, `outline`, `ghost`, `link` | **Deferred** | Deferred to subsequent component variant phases |
| **Size Matrix** | `sm`, `lg`, `icon`, `icon-xs`, `icon-sm`, `icon-lg` | **Deferred** | Deferred to subsequent component variant phases |
| **State: Disabled** | `disabled:pointer-events-none disabled:opacity-50` | **Deferred** | Deferred to interactive state phases |
| **State: Loading** | Spinner insertion, loading aria attributes | **Deferred** | Deferred to interactive state phases |
| **State: Hover / Active** | `hover:bg-primary/90`, active press transforms | **Deferred** | Static parity slice targets default resting state |
| **State: Focus-Visible** | `focus-visible:ring-[3px] focus-visible:ring-ring/50` | **Deferred** | Static parity slice targets default resting state |
| **Polymorphic / As-Child** | Radix `Slot` / `<a>` polymorphism for link buttons | **Deferred** | Deferred to navigation component integration |
| **Icon Slots** | `[&_svg]:size-4 [&_svg]:shrink-0` leading/trailing slots | **Deferred** | Deferred to icon integration phase |

---

## 3. Styling Pipeline Across Execution Hosts

The cross-host styling architecture relies on shared Tailwind v4 CSS declarations executed across two distinct rendering engines:

### 3.1 Web Pipeline (Dioxus Web / WASM)
1. **Asset Injection**: `document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }` injects compiled Tailwind v4 CSS into the document `<head>`.
2. **DOM Emission**: Dioxus VDOM mounts the button element with class string `inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors bg-slate-900 text-white shadow-sm px-4 py-2`.
3. **Engine Evaluation**: Chromium CSS parser matches utility classes against stylesheets and computes layout geometry and paint properties.

### 3.2 Native Pipeline (Dioxus Native / Blitz)
1. **Asset Injection**: Blitz DOM loads `/assets/tailwind.css` via Stylo integration.
2. **CSS Parsing**: Mozilla Stylo (servo/stylo) parses CSS rules and applies selector matching.
3. **Layout & Rasterization**: Taffy computes flexbox dimensions (`inline-flex`, padding, gap), and Vello rasterizes background color (`bg-slate-900` -> `#0f172a`), border-radius (`rounded-md` -> `6px` / `0.375rem`), and text glyphs (`text-white`).

---

## 4. Parity Proof Path & Verification Strategy

Verification of styling parity between Web and Native requires empirical evidence from both engines:

### 4.1 Web Host Proof (CDP Runtime Evaluation)
- Connect to Google Chrome via Chrome DevTools Protocol (`http://127.0.0.1:9222`).
- Query computed styles for `[data-gui-test-id='button-primary']`:
  - `backgroundColor`: `rgb(15, 23, 42)` (`bg-slate-900`)
  - `color`: `rgb(255, 255, 255)` (`text-white`)
  - `borderRadius`: `6px` (`rounded-md` / `0.375rem`)
  - `fontSize`: `14px` (`text-sm` / `0.875rem`)
  - `padding`: `8px 16px` (`py-2 px-4`)
- Confirm element differs empirically from unstyled browser default button.

### 4.2 Native Host Proof (`blitz-host` UDS RPC)
- Query `blitz-host inspect "[data-gui-test-id='button-primary']"` or `blitz-host inspect` via node resolution.
- Extract node bounding rect (`[x, y, width, height]`).
- Verify node presence, dimensions, and layout settlement.

### 4.3 Visual Cross-Host Parity Comparison (`gui-test --mode parity`)
- Execute GUI test runner across desktop viewport (`1024x768`):
  ```bash
  cargo run --manifest-path util/gui-test/Cargo.toml -- test \
    --suite tests/gui \
    --package shadcn-dioxus \
    --scenario button-variants \
    --mode parity \
    --cdp-base-url http://127.0.0.1:9222 \
    --web-target-url http://127.0.0.1:8081 \
    --native-pid "$NATIVE_PID"
  ```
- Generate and validate physical artifacts under `test-results/gui/runs/<run-id>/`:
  - `actual/shadcn-dioxus/button-variants/button-primary/web/desktop/default.png`
  - `actual/shadcn-dioxus/button-variants/button-primary/native/desktop/default.png`
  - `diff/parity/shadcn-dioxus/button-variants/button-primary/desktop/default.png` (if visual difference detected)
  - `metadata/run_result.json`
  - `report/index.html`
- Parity status: Exit code 0 (`Passed`) or 2 (`ParityFailure`) certifies valid cross-host comparison execution.
