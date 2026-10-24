# kivuli request: a placeholder in a text area

From Rapidkoi, 2026-10-24 (DECISION-055). For a kivuli session in `../kivuli`, under kivuli's own CLAUDE.md and PRP process.

## What Rapidkoi needs

An app's typing box of several lines can say what it's for while it's empty: Notes' text box says "Write something". Contract 5 carries it (`placeholder` on an `input` with `lines` over 1), and Rapidkoi passes it on, but kivuli 0.54 applies `.placeholder` to one-line fields only: `GtkEntry` has `gtk_entry_set_placeholder_text`, and `GtkTextView` has nothing like it. So an empty text area shows nothing at all, and on a page with no field border (Notes) there's no sign where to type.

## What to add

`.placeholder(hint)` on `text_area` showing `hint`, dimmed, while the buffer is empty and stopping as soon as there's text. The view API is already there; only the drawing is missing.

GTK 4 has no built-in way. Two known approaches (kivuli's to choose):
- a `GtkLabel` overlaid at the text view's top start (`GtkOverlay`), dimmed, hidden when the buffer's `changed` signal leaves it non-empty, and not taking the pointer (`gtk_widget_set_can_target(label, FALSE)`);
- or drawing the text in a `snapshot` override.

## How Rapidkoi will use it

`ui.render.typing_box` already calls `text_area(…).name(placeholder)`; it adds `.placeholder(placeholder)`. No contract change.
