# kivuli request: knowing when the window comes back to the front

From Rapidkoi, 2026-10-24 (DECISION-054, PRP-044). For a kivuli session in `../kivuli`, under kivuli's own CLAUDE.md and PRP process.

## What Rapidkoi needs

Rapidkoi checks the app in front for a newer version every 15 minutes (`Sub.every`). The owner also asked for a check when Rapidkoi's window comes back to the front: someone who left it in the background for an hour sees the upgrade button as soon as they return, not up to 15 minutes later.

kivuli 0.54 has no way to hear that the window became active again.

## What to add

A subscription that sends a message when the window becomes active (and, for symmetry, inactive):

```
// Suggested API (kivuli's to decide)
Sub.window_active(fun(active: bool) = Msg.WindowActive(active))
```

GTK 4: `GtkWindow`'s `is-active` property: `g_signal_connect(window, "notify::is-active", …)`, reading `gtk_window_is_active(window)` in the handler (`gtk/gtkwindow.h`). The first value can be sent when the subscription starts.

## How Rapidkoi will use it

`shell.subscriptions` listens while a kept app is in front; `Msg.WindowActive(true)` runs `loads.check_current`, as `Msg.CheckForUpdates` does. No contract change.
