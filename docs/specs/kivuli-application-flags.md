# kivuli request: application flags (not single-instance; opening links)

From Rapidkoi, 2026-10-24 (DECISION-014). For a kivuli session in `../kivuli`, under kivuli's own CLAUDE.md and PRP process.

## What Rapidkoi needs

kivuli starts GTK with `gtk_application_new(program.id, 0)` (`src/render/live.la:109`). With no flags, GApplication is single-instance: a second `./main other.rkb` (or `./main rapidkoi://notes.dafifi.net`) finds the first one on D-Bus, raises its window, and the app it named is dropped. Two things follow from it:

1. Opening an app from outside a running Rapidkoi (a terminal, a desktop launcher, a `rapidkoi://` link clicked in another program) does nothing visible but raise the window.
2. Rapidkoi can't be run twice side by side, for development or testing.

## What to add

Let a program pass `GApplicationFlags`, and hand it the files or addresses it's asked to open:

```
// Suggested API (kivuli's to decide)
application(id, init, update, view)
    .flags([AppFlag.HandlesOpen])          // or AppFlag.NonUnique
    .on_open(fun(uris: string[]) = Msg.Open(uris))
```

GLib (`gio/gioenums.h`): `G_APPLICATION_HANDLES_OPEN` (1 << 2) delivers the arguments of a second launch to the first instance's `open` signal (`GApplication::open`, a `GFile` array, `g_file_get_uri`); `G_APPLICATION_NON_UNIQUE` (1 << 5) makes each launch its own instance.

## How Rapidkoi will use it

`HANDLES_OPEN`: a second launch's address or file opens in the running window, as if typed in the address box (`navigate`), in a new tab. Registering the `rapidkoi://` scheme with the desktop (a `.desktop` file's `MimeType=x-scheme-handler/rapidkoi`) is Rapidkoi's own packaging, later. No contract change.
