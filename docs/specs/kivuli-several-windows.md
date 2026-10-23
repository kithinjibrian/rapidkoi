# kivuli request: more than one window of one app

From Rapidkoi, session 177 (DECISION-042). For a kivuli session in `../kivuli`, under kivuli's own CLAUDE.md and PRP process.

## What Rapidkoi needs

Rapidkoi shows one app instance at a time (PRP-033, named instances). To see two side by side, the owner's answer is a second Rapidkoi window. Today that doesn't work: kivuli registers the GTK application as single-instance, so running `./main` again only raises the first window (Rapidkoi's DECISION-014).

## What would cover it (kivuli's to choose)

1. **Several windows in one kivuli program:** one model, each window drawing its own view, and a command to open another (like `Cmd.open_window(id)`). Messages from each window carry which window they came from. Rapidkoi would then decide what each window shows.
2. **Or no single-instance:** an option on `App` to let a second launch start its own process and window. That's simpler, but each window would have its own model, so it gives no shared state between windows.

Option 1 fits Rapidkoi better: instances, pins and trust stay one model.

## What Rapidkoi does until then

Nothing works around it. One window; a second `./main` raises the first (CLAUDE.md KNOWN ISSUES).
