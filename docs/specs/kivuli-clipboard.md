# kivuli request: writing text to the clipboard

From Rapidkoi, session 193 (DECISION-047, PRP-036). For a kivuli session in `../kivuli`, under kivuli's own CLAUDE.md and PRP process.

## What Rapidkoi needs

Every app window has a link: its address, its version, and the fields the app chose (`https://counter.dafifi.net/@0.3.0?count=3`). The address box shows it, and a share button beside it should copy it, so the person can paste it anywhere.

kivuli 0.54 has no way to put text on the clipboard. Rapidkoi doesn't reach past kivuli to GTK (its architecture rule 2), so share stays disabled until kivuli has one.

## What to add

A command that sets the clipboard's text, used from `update` like any other effect:

```
// Suggested API (kivuli's to decide)
Msg.Share => (m, Cmd.copy_text(link)),
```

GTK 4: `gdk_display_get_clipboard(gdk_display_get_default())`, then `gdk_clipboard_set_text(clipboard, text)`.

Reading the clipboard isn't needed: pasting into a field already works.

## How Rapidkoi will use it

`Msg.Share` copies `ui.frame.address_of(page)` for the current app, and the button is enabled.
