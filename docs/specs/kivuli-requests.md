# kivuli requests from Rapidkoi

Everything Rapidkoi needs from kivuli and doesn't have yet, in one place, most useful first. For kivuli sessions in `../kivuli`, under kivuli's own CLAUDE.md and PRP process: each section is a request (or a bug), its suggested API is kivuli's to decide, and Rapidkoi works around none of them. Each names its Rapidkoi decision in `DECISIONS.md`. When one is done, Rapidkoi moves its decision to resolved and removes its section here.

Fixed and kept for the record: `kivuli-group-hover.md` (0.52), `kivuli-scroll-fit-and-scroll-to.md` (0.53, 0.54).

7. [A file dialog](#6-a-file-dialog) (DECISION-020)
8. [More than one window of one app](#7-more-than-one-window-of-one-app) (DECISION-042)
9. [Knowing what fits: the window's width, or a row that overflows](#8-knowing-what-fits-the-windows-width-or-a-row-that-overflows) (DECISION-043)
10. [Bug: a drop-down inside a popover breaks the popover's dismissal](#9-bug-a-drop-down-inside-a-popover-breaks-the-popovers-dismissal) (DECISION-040)
11. [Bug: a picture's `.size(n)` doesn't cap it](#10-bug-a-pictures-sizen-doesnt-cap-it) (DECISION-034)
13. [Pictures: WebP, larger pictures, and a decode that fails](#13-pictures-webp-larger-pictures-and-a-decode-that-fails) (DECISION-058)
15. [Bug: a popover's `destroy` handler unparents a popover GTK already took apart (a crash)](#7-bug-a-popovers-destroy-handler-unparents-a-popover-gtk-already-took-apart-a-crash) (DECISION-063)
14. [Bug: an overlay layer with a maximum size takes the clicks of everything under it (0.62)](#14-bug-an-overlay-layer-with-a-maximum-size-takes-the-clicks-of-everything-under-it-062) (DECISION-062)
16. [An input group's add-ons, inset as the caller asks](#16-an-input-groups-add-ons-inset-as-the-caller-asks) (DECISION-065)


---

## 7. A file dialog

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

## 8. More than one window of one app

DECISION-042 (session 177).

**What Rapidkoi needs.** Rapidkoi shows one app instance at a time (PRP-033). To see two side by side, the owner's answer is a second Rapidkoi window, which doesn't work today (section 5).

**What would cover it (kivuli's to choose).**
1. **Several windows in one kivuli program:** one model, each window drawing its own view, and a command to open another (`Cmd.open_window(id)`). Messages from each window carry which window they came from.
2. **Or no single-instance** (section 5's `NON_UNIQUE`): each launch its own process and window, with its own model, so no state shared between windows.

Option 1 fits Rapidkoi better: instances, pins and trust stay one model.

---

## 9. Knowing what fits: the window's width, or a row that overflows

DECISION-043 (session 177).

**What Rapidkoi needs.** The header's start holds the current app's instances as pills, the rest behind ⋯. As the window narrows, pills should move into ⋯ one by one, so the address box stays centred (GTK centres a header's middle only while both sides fit) and the window can shrink. Today the strip is a fixed 520 px, which also sets a minimum window width. kivuli 0.54 has no window size in the model and no container that hides what doesn't fit.

**What would cover it (kivuli's to choose).**
1. **The window's size as a subscription:** `Sub.window_size(fun(w, h) = Msg.Resized(w, h))`, on GTK's `notify::default-width`/`-height`. Generic: any responsive layout wants it.
2. **Or a row that overflows:** `overflow_row(children, more: fun(hidden: i64[]) = View)`, showing the children that fit and calling `more` with the hidden ones' indices to draw a ⋯. The layout stays in GTK, where the real widths are.

Either is enough.

---

## 10. Bug: a drop-down inside a popover breaks the popover's dismissal

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

## 11. Bug: a picture's `.size(n)` doesn't cap it

DECISION-034.

**What happens.** kivuli 0.48's README says `.size(n)` on a picture is "an n × n box", but `render/styling.la` applies it as GTK's size request, a minimum, so an SVG draws at its own larger size.

**What to add.** `.size(n)` as a box: a clamp, as an exact `Sizing` already is.

**Meanwhile in Rapidkoi.** Its icons are sized with `tokens.square(n)`, an exact `Sizing` with a maximum, which kivuli documents as clamping. Once `.size` is fixed, `square` can go.

---

---

## 13. Pictures: WebP, larger pictures, and a decode that fails

DECISION-058 (PRP-047).

**What Rapidkoi needs.** Apps show pictures from their servers (PRP-047). Rapidkoi checks each one's header before handing its bytes to `image(Image.Bytes(key, bytes))`, but kivuli 0.54 shows only PNG, JPEG and SVG, up to 4,096 × 4,096 px and 16 MB, and a picture it can't decode shows nothing, silently. Agreed with the owner (DECISION-056) were WebP too, 8,000 px a side and 40 megapixels, and telling the app (`on_fail`) about every picture that can't be shown.

**What to add.**
1. **WebP** in `Image.Bytes`: GTK 4's `gdk_texture_new_from_bytes` reads PNG, JPEG and TIFF; WebP needs gdk-pixbuf's webp loader (`webp-pixbuf-loader`) or libwebp.
2. **A higher cap**: 8,000 px a side and 40 MP, or a cap the caller passes (Rapidkoi has already checked the header with `lugha-imagesize`).
3. **Knowing a picture failed to decode**: a message, for example `image(img).on_error(fun(why: string) = Msg.Broken(why))`, sent when the texture can't be made (the `GError` from `gdk_texture_new_from_bytes`).

**How Rapidkoi will use it.** `net.downloads.checked` lets WebP and larger pictures through; `ui.pictured` passes `on_error` through, and a failed decode is kept as gone and sent to the app as its `on_fail`. No contract change: only fewer refusals.

---

## 14. Bug: an overlay layer with a maximum size takes the clicks of everything under it (0.62)

DECISION-062 (2026-10-25).

**What happens.** Since 0.62 (PRP-066), a node with a maximum size and an alignment other than fill gets a clamp that fills its parent and places the node itself. In an `overlay`, a layer's clamp then covers the whole base: it's invisible, but GTK picks it for every click in that area, so nothing under it can be pressed. Rapidkoi's update button is an overlay of the button and a 7 px red dot (`square(7)`, `.place(Align.End, Align.Start)`): the button can't be clicked. The striped overflow band in development (`fixed_height`, `.place(Align.Fill, Align.End)`) would cover a whole app the same way.

**What would fix it.** The clamp takes input only where its child is: `contains` (or `pick`) answering for the child's allocation alone, or `gtk_widget_set_can_target(clamp, FALSE)` with the child targetable, when the clamp is larger than its child. Or, in an overlay layer, keep the pre-0.62 placement (the clamp at the node's alignment).

**How Rapidkoi will use it.** Nothing changes in Rapidkoi: the update button and the overflow band work again.

---

## 15. Bug: a popover's `destroy` handler unparents a popover GTK already took apart (a crash)

DECISION-063 (2026-10-25).

**What happens.** Clicking ⚙ in Rapidkoi's address box (its settings card is a kivuli popover) sometimes crashes Rapidkoi with a segfault, after `Gtk-CRITICAL: gtk_widget_unparent: assertion 'GTK_IS_WIDGET (widget)' failed`. With `G_DEBUG=fatal-criticals`, the critical comes from `render/patch/apply`'s closure dropping a node (`g_object_unref`), which emits a signal whose handler calls `gtk_widget_unparent` on something that is no longer a widget:

```
#9  g_object_unref ()                                   libgobject
#10 kivuli/render/patch/apply<Msg>.6
#11 kivuli/core/paths/update_in<Msg>
…
#18 kivuli/render/patch/apply<Msg>
#19 kivuli/render/live/dispatch<Model, Msg>
```

`render/panels.la` (a popover node) connects `box`'s `destroy` to `gtk_widget_unparent(pop)`; `render/popovers.la` does the same for a context menu's `popover`. When the box is disposed, GTK may already have unparented and finalized the popover (a widget's dispose takes its children apart), so the handler unparents a freed pointer: the critical, and sometimes a crash.

**What would fix it.** Hold a reference on the popover for as long as the handler can run (`g_object_ref` when parenting, `g_object_unref` after unparenting), and unparent only if it's still the box's child (`gtk_widget_get_parent(pop) == box`); or unparent the popover before the box is dropped, in the patch, instead of in `destroy`.

**How Rapidkoi will use it.** Nothing changes in Rapidkoi: ⚙'s card, ☰'s menu and the tabs' context menus stop risking a crash.

---

## 16. An input group's add-ons, inset as the caller asks

DECISION-065 (2026-10-30).

**What Rapidkoi needs.** Rapidkoi's address box is a `ui.input_group`, and its start holds pills now: a chip with the app's colour and name, and the shield's "Not signed"/"Key changed" pill, each filled and exactly the box's inner height. `input_group` puts its add-ons 12 px from the frame's edge (`ui/input_groups.la`, `add_ons`: `Spacing.padding_left(12)` on the start side, `padding_right(12)` on the end side), while above and below them there's only the frame's 1 px edge. So a filled pill sits 12 px in from the box's left end but flush with its top and bottom: the room around it isn't even. The owner wants it even. `.style` on the group reaches only its outer row, not the add-ons' row, so Rapidkoi can't change the 12 px.

**What to add (kivuli's to decide).** A way for the caller to set the add-ons' inset from the frame's edge, for example a variant given through `.style`:

```
// Suggested API (kivuli's to decide)
input_group(start, field, end).style([InputGroup.inset(2)])
```

or an `input_group_inset(start, field, end, px)`. The default stays 12, so nothing else changes.

**How Rapidkoi will use it.** The address box's add-ons inset by the frame's edge (or by `SPACE_XS` with the same padding above and below), so its pills have equal room on every side. No change to apps or the contract.

