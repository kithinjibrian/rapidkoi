# kivuli request: knowing what fits (the window's width, or a row that overflows)

From Rapidkoi, session 177 (DECISION-043). For a kivuli session in `../kivuli`, under kivuli's own CLAUDE.md and PRP process.

## What Rapidkoi needs

The header's start holds the current app's instances as pills, and the rest go behind a ⋯ menu. The owner wants it to adapt: as the window narrows, pills move into ⋯ one by one, so that:
- the address box stays centred: GTK centres a header's middle only while both sides fit;
- the window can shrink. Today the strip has a fixed width (520 px), which also sets a minimum window width.

Rapidkoi decides which pills show, so it needs to know how much room there is. kivuli 0.54 has no window size in the model and no container that hides what doesn't fit.

## What would cover it (kivuli's to choose)

1. **The window's size as a subscription:** `Sub.window_size(fun(w, h) = Msg.Resized(w, h))`, sent on GTK's `notify::default-width`/`-height` (or on the toplevel's size changes). It's generic: any app with a responsive layout wants it. Rapidkoi would work out how many pills fit from the width.
2. **Or a row that overflows:** `overflow_row(children, more: fun(hidden: i64[]) = View)`. It shows the children that fit in the room it's given, and calls `more` with the hidden ones' indices to draw a ⋯. That's closer to GTK's own layout, and the app never handles pixels.

Either is enough. Option 1 is the smaller change; option 2 keeps the layout logic in GTK, where the real widths are.

## What Rapidkoi does until then

Nothing works around it. The strip stays a fixed width with at most three pills, and a narrower window shifts the address box and can't shrink past the strip (CLAUDE.md KNOWN ISSUES).
