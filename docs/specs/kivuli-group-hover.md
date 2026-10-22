# kivuli request: a parent's hover that reaches its children (group hover)

From Rapidkoi, session 145 (DECISION-036). For a kivuli session in `../kivuli`, under kivuli's own CLAUDE.md and PRP process.

## What Rapidkoi needs

The sidebar shows each open app as a button. The owner wants a small × to appear on an app **whenever the pointer is anywhere over that app's button**, and to stay hidden otherwise:

```
 ┌─────────┐
 │  📝   × │   ← the app's button; the × a separate button inside or over it
 └─────────┘
```

Today a node can only be styled by its **own** state (`When(State.Hover, …)` → `:hover` on that node). The × can't react to the pointer being over the app's button around it. Hidden at rest and shown on its own hover, it would only appear once the pointer is already exactly on it.

## What to add

A way to mark a node as a **group**, and a **state** that applies to a node while an ancestor group is in a state. As Tailwind's `group` / `group-hover`, and GTK's CSS allows it (`.parent:hover .child`).

Suggested API (kivuli's to decide):

```
// The app's button is the group.
button_with(row([face, spacer(), close]), Msg.Select(id)).group()

// The ×: invisible at rest, shown while its group is hovered.
close.style([
    Effects.opacity(0.0),
    When(State.GroupHover, [Effects.opacity(1.0)]),
])
```

- `.group()` on any node: adds a CSS class (say `kv-group`) to its widget.
- `State.GroupHover`: in `src/styles/sheets.la`'s `state_rules`, a rule whose selector is the group's hover **before** the node, e.g. `.kv-group:hover .kv-ghover-<class>` (a descendant selector), ranked with `Hover`. Possibly `GroupFocusWithin` the same way, so keyboard users see the × too (`.kv-group:focus-within …`).
- Works inside `When` as other states do, and with `Inside(Part…)`.
- Nested groups: the nearest group is enough for Rapidkoi; Tailwind's named groups (`group/name`) are not needed.

## Notes for kivuli

- **Overlays:** the × may sit in an `overlay` layer over the app's button rather than inside it. A layer isn't a GTK descendant of the base, so the group would have to be the overlay itself (`overlay(base, [close]).group()`). Worth checking the selector reaches layers.
- **Hit-testing:** an invisible (opacity 0) × still takes clicks. Fine for Rapidkoi (clicking the corner closes), but kivuli may want `When(State.GroupHover, …)` usable with `.inert`-like behaviour later; not needed now.
- **Tests:** kivuli's own; Rapidkoi only needs the styles to build. `kivuli.testing` finds the × by its tooltip either way.
- **Other uses:** row actions shown on hover (site settings' rows, tiles on the new tab, list items).

## What Rapidkoi does until then

The × shows on the **active** app only; any app closes from its right-click menu, and Ctrl+W closes the current one (PRP-023). With group hover, the × moves to "any hovered app" and nothing else changes.
