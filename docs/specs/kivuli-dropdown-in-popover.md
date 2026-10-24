# kivuli bug: a drop-down inside a popover breaks the popover's dismissal

From Rapidkoi, session 172 (DECISION-040). For a kivuli session in `../kivuli`, under kivuli's own CLAUDE.md and PRP process.

## What happens

Rapidkoi's settings card is a `popover` holding `ui.dropdown`s (its Permissions page). After you pick a choice in one of them, the popover can no longer be dismissed:
- a click elsewhere in the window doesn't close it;
- Escape doesn't either.

Only a click on another surface closes it, such as the desktop's clock or a panel button. The model and the widget drift apart: the repro below showed `open: true · close messages: 2` with the popover still on screen.

A dialog opened from inside the same popover (a confirm dialog) works normally. It's the drop-down's own list closing that matters.

## Repro (kivuli 0.54, no Rapidkoi code)

`lugha.toml`: a package depending on `kivuli = { git = "https://github.com/kithinjibrian/kivuli", version = "0.54" }`. `src/main.la`:

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

Note: the Settings button here is as wide as the window, so a click "outside" may land on it and toggle. In Rapidkoi the clicks land on the page, with the same result.

## A guess, for kivuli to confirm

GtkDropDown's list is a popover of its own, nested in kivuli's. When it closes after a pick, the outer popover seems to lose its grab and focus. Its autohide then reacts only to focus leaving the window. kivuli might give the focus back to the outer popover once the nested one closes, or GTK may need something else. That's kivuli's to find.

## What Rapidkoi does until then

Nothing works around it (CLAUDE.md SCOPE RULE). After changing a permission, the card closes with ⚙ again (`TogglePanel`) or a click outside the window. Listed in CLAUDE.md KNOWN ISSUES.
