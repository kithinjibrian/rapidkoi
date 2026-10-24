## FEATURE: Apps lay themselves out for the window: no page scroll, scrolling boxes, overflow clipped (and shown in development)

## OBJECTIVE
Rapidkoi builds apps, not sites (the owner): an app's page is exactly the window's content area, and Rapidkoi never scrolls it. The developer lays the screen out for the window and marks the boxes that scroll: a list beside a note that scrolls on its own, a long settings page, a row of thumbnails. A scrolling box is as tall as its content until it runs out of room, then it scrolls; with `grow` it fills the room left. A scrolling box inside another that scrolls the same way is refused with a sentence. Content that still doesn't fit is **clipped**: silently in a published app, and, in development (a local file or a `localhost` address), with a striped band at the cut edge saying "Bottom overflowed by 42 px", as Flutter does in debug builds. Rapidkoi's own screens (loading, an error, a refusal) still scroll when they're long.

PRP-046 of three for contract 7 (DECISION-056 Q4): after the base (PRP-045), before pictures (PRP-047).

## CONTEXT
- Agreed with the owner (DECISION-056 Q4, amended at PRP review: "we shouldn't provide scroll at all. if something is overflowing the dev should take care of it… we are building apps not sites"; "do it silently then localhost can be shown explicitly"), after research: CSS's `min-height: auto` trap, Flutter's "vertical viewport was given unbounded height", Compose's "infinite maximum height", SwiftUI's content-sized `ScrollView`. The lesson: a scrolling box needs a bounded parent, and the platform should design the traps out.
- Today `ui/layout.la` wraps every page in kivuli's `scroll(…)` (lines 47, 49), so an app's own scrolling box would never scroll: the page just grows.
- kivuli 0.54 has `scroll(child)` (takes the room given, scrolls when bigger) and `scroll_fit(child)` (as big as the child until the parent can't give that much). Both are `GtkScrolledWindow`s with the automatic policy, so **they scroll whichever way the content overflows**: kivuli can't yet limit one to an axis. A scrolled window's minimum size is small, so it never pushes its parents bigger (GTK's own rule), and nothing needs a `min-height: 0`.
- Files: `src/guest/styles.la` and `nodes.la` (the `scroll` key), `src/guest/tree.la` (the nesting check), `src/ui/render.la` (drawing it), `src/ui/layout.la` (the page), `docs/specs/contract-7.md`; tests in `test/styles.la`, `test/tree.la`, `test/render.la`, `test/layout.la`.
- The built-in apps (`guest/rust/newtab`, `guest/rust/settings`) put their content in a scrolling box: they need the SDK's `Layout::scroll()` (`rapidkoi-rust` `v0.2.0-pre.2`).
- Decisions: DECISION-056 (resolved). No open ones.

## REQUIREMENTS

### Must Do
- **The style key `scroll`:** `"vertical"`, `"horizontal"` or `"both"`, on a box. A bad value stops the app with its sentence, as any style.
- **Drawing it:** a box with `scroll` is drawn inside kivuli's `scroll_fit` (as tall as its content until its parent runs out of room), or `scroll` when it also has `grow` (fills the room left, scrolls when its content is bigger).
- **The page:** a running app's view is never wrapped in a scroll: it gets exactly the content area. Rapidkoi's own screens (loading, stopped, refused, a changed key) stay in a scroll.
- **Overflow is clipped:** the app's view sits in a container that gives it the content area's size and cuts what doesn't fit, so an app can never make Rapidkoi's window bigger. GTK has one (a `GtkScrolledWindow` with `GTK_POLICY_EXTERNAL` both ways); kivuli doesn't expose it: a kivuli request (below). Until kivuli has it, an overflowing app pushes the window's minimum size: visible, the developer's to fix, never worked around.
- **In development, overflow is shown:** for an app from a local file or a `localhost` address (the same test as DECISION-045's development check), each clipped edge gets a striped band in the theme's danger red with "Bottom overflowed by 42 px" (or Right, Top, Left). Published apps never show it. It needs kivuli to say how far the child overflows (the same request); built when kivuli has it.
- **No scroll inside a scroll the same way:** a box with `scroll` inside another with `scroll` sharing an axis (`both` shares both) stops the app: "This app put a scrolling box inside another that scrolls the same way" (at its path). A horizontal one inside a vertical one is fine.
- **kivuli requests, recorded, not worked around** (`docs/specs/kivuli-requests.md`, new sections; a DECISIONS.md entry): **(a)** a clipping container (a scrolled window with `GTK_POLICY_EXTERNAL`) that reports how far its child overflows on each side (measured with `gtk_widget_measure` against its allocation), for the clip and the development band; **(b)** `scroll` with an axis (`gtk_scrolled_window_set_policy`): until then a `vertical` box also scrolls sideways if its content is wider.
- **The built-ins scroll their own content:** the new tab's and Settings' screens wrap what can grow (pins, sessions, the settings) in a scrolling box, with the SDK's `Layout::scroll()`.
- **The SDK (`../rapidkoi-rust`, at the owner's word, as in PRP-045):** `Layout::scroll()`, `Layout::scroll_x()`, `Layout::scroll_both()` writing `scroll`; tagged `v0.2.0-pre.2`.
- `docs/specs/contract-7.md`'s style table and a paragraph on scrolling; CHANGELOG.

### Must NOT Do
- No page-level scroll for apps, ever: the app lays itself out.
- No overflow band in a published app.
- No minimum-size styles for apps to learn: GTK's scrolled window already never pushes its parents bigger.
- No scrolling by messages (scroll to a node, read the position): later, if an app needs it.

## DESIGN
- `nodes.Look` gains `scroll: Option<Axis>` (`Vertical`, `Horizontal`, `Both`); `styles.read_style` reads it.
- `tree.la` carries the nearest scrolling ancestor's axis down while reading boxes and refuses an overlap.
- `render.boxed`: after building the box, `if let Some(_) = look.scroll { if look.grow { scroll(v) } else { scroll_fit(v) } }`.
- `layout.content`: `App(a)` running → `app_view(…)` with the margin, no `scroll`; anything else → `scroll(…)` as now.

## TESTS
- [ ] `scroll` reads as each axis; a bad value stops the app with its sentence
- [ ] A scrolling box inside one that scrolls the same way (`vertical` in `vertical`, anything in `both`) stops the app at its path; `horizontal` in `vertical` reads
- [ ] A box with `scroll` is drawn in a scrolled area; without, it isn't
- [ ] A running app's page isn't wrapped in a scroll; a stopped page and the loading page are
- [ ] Whether an app is in development (a file, `localhost`) decides whether its overflow band may show (pure; the band itself once kivuli reports overflow)
- [ ] The new tab and Settings still show and work (their tests), their content in a scrolling box

## ACCEPTANCE
- [ ] Every test above passes
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build src/main.la -o main` pass
- [ ] For the owner, by eye: ⚙ Settings with the window made short scrolls inside its own content; the new tab with many pins scrolls; Notes (once moved to contract 7) scrolls its list and its note apart
