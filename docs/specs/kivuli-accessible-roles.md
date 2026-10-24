# kivuli request: accessible roles, states and labels on any node

From Rapidkoi, session 198 (DECISION-050, PRP-037). For a kivuli session in `../kivuli`, under kivuli's own CLAUDE.md and PRP process.

## What Rapidkoi needs

Rapidkoi's apps build widgets from styled boxes: a checkbox is a small styled square and a text inside a pressable box. Rapidkoi knows what it is, because the app says so (`role: checkbox, checked: true`). A screen reader doesn't, because kivuli gives GTK no way to hear it, so it announces "button" instead of "Buy milk, checkbox, ticked".

## What to add

Modifiers that set a node's accessible role, state and label, mapped onto GTK 4's `GtkAccessible`:

```
// Suggested API (kivuli's to decide)
button_with(row([square, text("Buy milk")]), Msg.Toggle(3))
    .role(Role.Checkbox)        // gtk_accessible: GTK_ACCESSIBLE_ROLE_CHECKBOX (at construction)
    .checked(true)              // gtk_accessible_update_state(GTK_ACCESSIBLE_STATE_CHECKED)
    .label("Buy milk")          // gtk_accessible_update_property(GTK_ACCESSIBLE_PROPERTY_LABEL)
```

Roles Rapidkoi needs first: button, checkbox, switch, tab, tab list, heading, list, list item, group, separator. States: checked, selected, disabled. A role is fixed when a widget is made (`accessible-role` is a construct-only property in GTK 4), so a node whose role changes is rebuilt.

## How Rapidkoi will use it

`ui.render` passes each contract 5 node's `role`, `checked`, `selected` and `label` straight through. No change to the contract.
