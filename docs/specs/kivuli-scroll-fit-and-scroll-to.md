# kivuli request: a scroll that fits its content, and scrolling a node into view

From Rapidkoi, session 148 (DECISION-037). For a kivuli session in `../kivuli`, under kivuli's own CLAUDE.md and PRP process.

## What Rapidkoi needs

The sidebar lists apps down the window's right, then a + button. The owner wants it as Chrome's tab strip:

```
 few apps            many apps
  📝                  📝 ┐
  📧                  📧 │ scrolls
  ＋  ← right after   ... ┘
                      ＋  ← stuck at the bottom
```

and, when an app opens or is selected while scrolled out of sight, the sidebar scrolls so it shows.

## 1. A scroll that's only as tall as its content

Today `scroll(child)` takes all the room its parent gives it, so a + after it always sits at the bottom of the strip, never right after the last app.

GTK has this: `gtk_scrolled_window_set_propagate_natural_height` (and `_width`). With it on, the scrolled window asks for its child's natural height and scrolls only once the parent can't give that much. Followed by + in a column (no spacer), + sits after the last app and stays visible at the bottom once the list overflows.

Suggested API (kivuli's to decide): a modifier on `scroll`, e.g. `scroll(apps).fit()` (natural height and width both), or `.fit_height()`. The column holding it must let it shrink: the scroll can't be `.expand()`ed, and GTK needs the scrolled window's minimum height small (it is, by default).

## 2. Scroll a node into view

Rapidkoi wants, after selecting or opening an app, to scroll the sidebar so that app's button shows, without moving keyboard focus (the address box or the app keeps it). `Cmd.focus(name)` scrolls as a side effect but takes focus, so it doesn't fit.

GTK 4.12+ has `gtk_viewport_scroll_to(viewport, descendant, scroll_info)` (a `GtkScrollInfo` choosing the edge or centring). Suggested API: a command, `Cmd.scroll_to(name)`, scrolling the nearest scrolled ancestor of the node named `name` (`.name(…)`) so it's fully visible, doing nothing if it already is. Same timing as `Cmd.focus` (after the next render, once the node exists).

## Notes for kivuli

- `kivuli.testing` can find `Cmd.scroll_to(name)` in a command like `Cmd.focus`, so Rapidkoi's tests check it's sent.
- Both are generic: long lists in any app want them.

## What Rapidkoi does until then

The sidebar's apps scroll, with + fixed at the bottom of the strip (PRP-023 amendment). Nothing scrolls an app into view yet.
