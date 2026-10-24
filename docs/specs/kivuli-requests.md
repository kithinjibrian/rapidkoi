# kivuli requests from Rapidkoi

Everything Rapidkoi needs from kivuli and doesn't have yet, in one place, most useful first. For kivuli sessions in `../kivuli`, under kivuli's own CLAUDE.md and PRP process: each section is a request (or a bug), its suggested API is kivuli's to decide, and Rapidkoi works around none of them. Each names its Rapidkoi decision in `DECISIONS.md`. When one is done, Rapidkoi moves its decision to resolved and removes its section here.

Fixed and kept for the record: `kivuli-group-hover.md` (0.52), `kivuli-scroll-fit-and-scroll-to.md` (0.53, 0.54).

1. [Accessible roles, states and labels on any node](#1-accessible-roles-states-and-labels-on-any-node) (DECISION-050)
2. [Writing text to the clipboard](#2-writing-text-to-the-clipboard) (DECISION-047)
3. [Application flags: opening from outside, not single-instance](#3-application-flags-opening-from-outside-not-single-instance) (DECISION-014)
4. [A file dialog](#4-a-file-dialog) (DECISION-020)
5. [More than one window of one app](#5-more-than-one-window-of-one-app) (DECISION-042)
6. [Knowing what fits: the window's width, or a row that overflows](#6-knowing-what-fits-the-windows-width-or-a-row-that-overflows) (DECISION-043)
7. [Bug: a drop-down inside a popover breaks the popover's dismissal](#7-bug-a-drop-down-inside-a-popover-breaks-the-popovers-dismissal) (DECISION-040)
8. [Bug: a picture's `.size(n)` doesn't cap it](#8-bug-a-pictures-sizen-doesnt-cap-it) (DECISION-034)
9. [Pictures: WebP, larger pictures, and a decode that fails](#9-pictures-webp-larger-pictures-and-a-decode-that-fails) (DECISION-058)

---

## 1. Accessible roles, states and labels on any node

DECISION-050 (session 198, PRP-037).

**What Rapidkoi needs.** Rapidkoi's apps build widgets from styled boxes: a checkbox is a small styled square and a text inside a pressable box. Rapidkoi knows what it is, because the app says so (`role: checkbox, checked: true`). A screen reader doesn't: kivuli gives GTK no way to hear it, so it announces "button" instead of "Buy milk, checkbox, ticked". Typing boxes need a label too ("Search notes").

**What to add.** Modifiers that set a node's accessible role, state and label, mapped onto GTK 4's `GtkAccessible`:

```
// Suggested API (kivuli's to decide)
button_with(row([square, text("Buy milk")]), Msg.Toggle(3))
    .role(Role.Checkbox)        // GTK_ACCESSIBLE_ROLE_CHECKBOX (at construction)
    .checked(true)              // gtk_accessible_update_state(GTK_ACCESSIBLE_STATE_CHECKED)
    .label("Buy milk")          // gtk_accessible_update_property(GTK_ACCESSIBLE_PROPERTY_LABEL)
```

Roles first: button, checkbox, switch, tab, tab list, heading, list, list item, group, separator. States: checked, selected, disabled. A label on text fields and text areas as well as boxes. A role is fixed when a widget is made (`accessible-role` is construct-only in GTK 4), so a node whose role changes is rebuilt.

**How Rapidkoi will use it.** `ui.render` passes each contract 5 node's `role`, `checked`, `selected` and `label` straight through; typing boxes gain `label` in the contract then (a version bump, Rapidkoi's own).

---

## 2. Writing text to the clipboard

DECISION-047 (session 193, PRP-036).

**What Rapidkoi needs.** Every app window has a link (`https://counter.dafifi.net/@0.3.0?count=3`). The address box shows it, and a share button beside it should copy it. kivuli 0.54 can't put text on the clipboard, and Rapidkoi doesn't reach past kivuli to GTK, so share stays disabled.

**What to add.** A command that sets the clipboard's text, used from `update` like any other effect:

```
// Suggested API (kivuli's to decide)
Msg.Share => (m, Cmd.copy_text(link)),
```

GTK 4: `gdk_display_get_clipboard(gdk_display_get_default())`, then `gdk_clipboard_set_text(clipboard, text)`. Reading the clipboard isn't needed: pasting into a field already works.

**How Rapidkoi will use it.** `Msg.Share` copies `ui.frame.address_of(page)`, and the button is enabled.

---

## 3. Application flags: opening from outside, not single-instance

DECISION-014.

**What Rapidkoi needs.** kivuli starts GTK with `gtk_application_new(program.id, 0)` (`src/render/live.la:109`). With no flags, GApplication is single-instance: a second `./main other.rkb` (or `./main rapidkoi://notes.dafifi.net`) finds the first one on D-Bus, raises its window, and the app it named is dropped. So opening an app from outside a running Rapidkoi (a terminal, a launcher, a `rapidkoi://` link in another program) only raises the window, and Rapidkoi can't run twice side by side.

**What to add.** Let a program pass `GApplicationFlags`, and hand it what it's asked to open:

```
// Suggested API (kivuli's to decide)
application(id, init, update, view)
    .flags([AppFlag.HandlesOpen])          // or AppFlag.NonUnique
    .on_open(fun(uris: string[]) = Msg.Open(uris))
```

GLib (`gio/gioenums.h`): `G_APPLICATION_HANDLES_OPEN` (1 << 2) delivers a second launch's arguments to the first instance's `open` signal (`GApplication::open`, a `GFile` array, `g_file_get_uri`); `G_APPLICATION_NON_UNIQUE` (1 << 5) makes each launch its own instance.

**How Rapidkoi will use it.** `HANDLES_OPEN`: a second launch's address or file opens in the running window, in a new tab, as if typed in the address box. Registering the `rapidkoi://` scheme with the desktop (`MimeType=x-scheme-handler/rapidkoi`) is Rapidkoi's packaging, later. No contract change.

---

## 4. A file dialog

DECISION-020 (and DECISION-018's placeholders).

**What Rapidkoi needs.** Opening a local app (`.rkb` or `.wasm`) means typing its path into the address box. A person should be able to pick the file.

**What to add.** A command that opens the system's file chooser and sends the picked file's path, or nothing when cancelled:

```
// Suggested API (kivuli's to decide)
Cmd.choose_file("Open an app", [("Rapidkoi apps", ["*.rkb", "*.wasm"])], fun(path: Option<string>) = Msg.Picked(path))
```

GTK 4.10+ (`gtk/gtkfiledialog.h`; GTK 4.22 here): `gtk_file_dialog_new`, `gtk_file_dialog_set_title`, `gtk_file_dialog_set_filters` (a `GListStore` of `GtkFileFilter` with `gtk_file_filter_add_pattern`), then `gtk_file_dialog_open(dialog, window, NULL, callback, data)` and, in the callback, `gtk_file_dialog_open_finish` → a `GFile` (`g_file_get_path`), or an error when cancelled. It goes through the desktop portal when sandboxed.

**How Rapidkoi will use it.** An Open… item in the ☰ menu (and Ctrl+O); the picked path opens as typing it does (`navigate.open_file`). Typed paths keep working. No contract change.

---

## 5. More than one window of one app

DECISION-042 (session 177).

**What Rapidkoi needs.** Rapidkoi shows one app instance at a time (PRP-033). To see two side by side, the owner's answer is a second Rapidkoi window, which doesn't work today (section 5).

**What would cover it (kivuli's to choose).**
1. **Several windows in one kivuli program:** one model, each window drawing its own view, and a command to open another (`Cmd.open_window(id)`). Messages from each window carry which window they came from.
2. **Or no single-instance** (section 5's `NON_UNIQUE`): each launch its own process and window, with its own model, so no state shared between windows.

Option 1 fits Rapidkoi better: instances, pins and trust stay one model.

---

## 6. Knowing what fits: the window's width, or a row that overflows

DECISION-043 (session 177).

**What Rapidkoi needs.** The header's start holds the current app's instances as pills, the rest behind ⋯. As the window narrows, pills should move into ⋯ one by one, so the address box stays centred (GTK centres a header's middle only while both sides fit) and the window can shrink. Today the strip is a fixed 520 px, which also sets a minimum window width. kivuli 0.54 has no window size in the model and no container that hides what doesn't fit.

**What would cover it (kivuli's to choose).**
1. **The window's size as a subscription:** `Sub.window_size(fun(w, h) = Msg.Resized(w, h))`, on GTK's `notify::default-width`/`-height`. Generic: any responsive layout wants it.
2. **Or a row that overflows:** `overflow_row(children, more: fun(hidden: i64[]) = View)`, showing the children that fit and calling `more` with the hidden ones' indices to draw a ⋯. The layout stays in GTK, where the real widths are.

Either is enough.

---

## 7. Bug: a drop-down inside a popover breaks the popover's dismissal

DECISION-040 (session 172).

**What happens.** Rapidkoi's settings card is a `popover` holding `ui.dropdown`s (its Permissions page). After a pick in one, the popover can't be dismissed: a click elsewhere in the window doesn't close it, nor does Escape. Only a click on another surface does (the desktop's clock, a panel button). The model and the widget drift apart: the repro showed `open: true · close messages: 2` with the popover still on screen. A dialog opened from inside the same popover works; it's the drop-down's own list closing that matters.

**Repro (kivuli 0.54, no Rapidkoi code).** A package depending on `kivuli = { git = "https://github.com/kithinjibrian/kivuli", version = "0.54" }`, `src/main.la`:

```
// Repro: a ui.dropdown inside a kivuli popover. Open the popover, pick a
// choice, then try to dismiss the popover (click outside, Escape).
import kivuli { App, View, Cmd, Sub, application, column, text, popover };
import kivuli.ui { button, dropdown };

struct Model {
    open: bool,
    picked: string,
    closes: i64,
}

enum Msg {
    Toggle,
    Close,
    Pick(string),
}

fun update(m: Model, msg: Msg): (Model, Cmd<Msg>) = match msg {
    Toggle => (Model { open: !m.open, ..m }, Cmd.none()),
    Close => (Model { open: false, closes: m.closes + 1, ..m }, Cmd.none()),
    Pick(s) => (Model { picked: s, ..m }, Cmd.none()),
};

fun view(m: Model): View<Msg> {
    let choices = ["Always allowed", "Needs approval", "Blocked"].map(fun(s: string) = (s, Msg.Pick(s)));
    column([
        popover(
            button("Settings", Msg.Toggle),
            m.open,
            column([text("Reading your pins"), dropdown(choices, m.picked, "")]),
            Msg.Close,
        ),
        text("open: {m.open} · picked: {m.picked} · close messages: {m.closes}"),
    ])
}

fun main(): i32 = application(App {
    id: "app.rapidkoi.repro",
    init: (Model { open: false, picked: "Always allowed", closes: 0 }, Cmd.none()),
    update: update,
    view: view,
    subscriptions: fun(_: Model) = Sub.none(),
});
```

1. Click **Settings**, and pick "Needs approval" in the drop-down.
2. Click elsewhere in the window, or press Escape: the popover stays.
3. Click the desktop's clock: it closes.

(The Settings button here is as wide as the window, so a click "outside" may land on it and toggle. In Rapidkoi the clicks land on the page, with the same result.)

**A guess, for kivuli to confirm.** GtkDropDown's list is a popover of its own, nested in kivuli's. When it closes after a pick, the outer popover seems to lose its grab and focus, so its autohide reacts only to focus leaving the window. kivuli might give the focus back to the outer popover once the nested one closes.

**Meanwhile in Rapidkoi.** After changing a permission, the card closes with ⚙ again (`TogglePanel`) or a click outside the window.

---

## 8. Bug: a picture's `.size(n)` doesn't cap it

DECISION-034.

**What happens.** kivuli 0.48's README says `.size(n)` on a picture is "an n × n box", but `render/styling.la` applies it as GTK's size request, a minimum, so an SVG draws at its own larger size.

**What to add.** `.size(n)` as a box: a clamp, as an exact `Sizing` already is.

**Meanwhile in Rapidkoi.** Its icons are sized with `tokens.square(n)`, an exact `Sizing` with a maximum, which kivuli documents as clamping. Once `.size` is fixed, `square` can go.

---

## 9. Pictures: WebP, larger pictures, and a decode that fails

DECISION-058 (PRP-047).

**What Rapidkoi needs.** Apps show pictures from their servers (PRP-047). Rapidkoi checks each one's header before handing its bytes to `image(Image.Bytes(key, bytes))`, but kivuli 0.54 shows only PNG, JPEG and SVG, up to 4,096 × 4,096 px and 16 MB, and a picture it can't decode shows nothing, silently. Agreed with the owner (DECISION-056) were WebP too, 8,000 px a side and 40 megapixels, and telling the app (`on_fail`) about every picture that can't be shown.

**What to add.**
1. **WebP** in `Image.Bytes`: GTK 4's `gdk_texture_new_from_bytes` reads PNG, JPEG and TIFF; WebP needs gdk-pixbuf's webp loader (`webp-pixbuf-loader`) or libwebp.
2. **A higher cap**: 8,000 px a side and 40 MP, or a cap the caller passes (Rapidkoi has already checked the header with `lugha-imagesize`).
3. **Knowing a picture failed to decode**: a message, for example `image(img).on_error(fun(why: string) = Msg.Broken(why))`, sent when the texture can't be made (the `GError` from `gdk_texture_new_from_bytes`).

**How Rapidkoi will use it.** `net.downloads.checked` lets WebP and larger pictures through; `ui.pictured` passes `on_error` through, and a failed decode is kept as gone and sent to the app as its `on_fail`. No contract change: only fewer refusals.
