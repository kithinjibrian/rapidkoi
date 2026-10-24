# kivuli request: a file dialog

From Rapidkoi, 2026-10-24 (DECISION-018, DECISION-020). For a kivuli session in `../kivuli`, under kivuli's own CLAUDE.md and PRP process.

## What Rapidkoi needs

Opening a local app (`.rkb` or `.wasm`) means typing its path into the address box: one of PRP-002's placeholders (DECISION-018). A person should be able to pick the file.

## What to add

A command that opens the system's file chooser and sends the picked file's path, or nothing when cancelled:

```
// Suggested API (kivuli's to decide)
Cmd.choose_file("Open an app", [("Rapidkoi apps", ["*.rkb", "*.wasm"])], fun(path: Option<string>) = Msg.Picked(path))
```

GTK 4.10+ (`gtk/gtkfiledialog.h`, GTK 4.22 here): `gtk_file_dialog_new`, `gtk_file_dialog_set_title`, `gtk_file_dialog_set_filters` (a `GListStore` of `GtkFileFilter` with `gtk_file_filter_add_pattern`), then `gtk_file_dialog_open(dialog, window, NULL, callback, data)` and, in the callback, `gtk_file_dialog_open_finish` → a `GFile` (`g_file_get_path`), or an error when cancelled. It goes through the desktop portal when sandboxed, so it works under Flatpak too.

## How Rapidkoi will use it

An Open… item in the ☰ menu (and Ctrl+O) sends the command; the picked path opens as typing it does (`navigate.open_file`, PRP-002's limits). The address box keeps accepting typed paths. No contract change.
