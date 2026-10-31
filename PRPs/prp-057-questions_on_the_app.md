## FEATURE: Questions about an app stay on its screen: Forget and an app's approval dim only that app's area, and stay with its tab

## OBJECTIVE
When Rapidkoi asks about one app ("Forget Notes?", or an app asking for a permission), only that app's area dims and freezes; the question's card sits in its middle. The sidebar, the address box, ☰ and the other tabs keep working. Switching to another tab leaves the question waiting on its own; coming back shows it again. Forget always forgets the app it asked about. Escape and a click on the dimmed area answer Cancel (or Don't allow), as the window-wide dialog does today. About, renaming a window and naming a separate app stay window-wide.

## CONTEXT
- **Decision:** DECISION-085, resolved (the owner: "okay let's do 085"). Follows PRP-052 (Forget's words) and PRP-031 (approvals).
- **kivuli 0.66**, checked in `../kivuli/README.md`: `overlay(base, layers)` with `.place(across, down)`; `.inert(on)` on any node; `.shortcut("Escape")` on a button; `kivuli.ui`'s `card`, `dialog_header`, `dialog_footer`. No new kivuli feature is needed.
- **Theme:** `scrim` already exists in `src/ui/schemes.la` (each scheme; "the dim layer over the app").
- **Files to read first:** `src/ui/settings.la` (403 lines: `dialog`, `dialog_view`), `src/ui/layout.la` (112: `content_area`, `view`), `src/ui/grants.la` (`question`), `src/shell.la` (483: `Ask`, `Cancel`, `Forget`, the panel's messages, `on_tab`), `src/window/actions.la` (`Confirm`, `Approval`), `src/window/navigate.la` (455: Forget from a tab's menu), `src/ui/addressbox.la` (`card_open`).
- **Creates:** `src/ui/questions.la` (the question over an app's area); `test/questions.la`.
- **Changes:** `actions.la` (`Confirm.Forget(i64)`: the tab asked about), `settings.la` (only window-wide questions in `dialog`), `layout.la` (the question over the content area), `shell.la` and `navigate.la` (asking with the tab; Forget acting on it; the panel's messages no longer clearing a question), `addressbox.la`, CHANGELOG.md.

## REQUIREMENTS

### Must Do
- **Which questions:** Forget (`m.confirm`) and an app's approval (`m.approval`, for a tab or the new tab) move over the app's area. Renaming (`m.renaming`), naming a separate app (`m.naming`) and About stay in the root's `.dialog`.
- **Over the area:** `overlay(content.inert(true), [scrim, card])`: the scrim fills the area in the theme's `scrim` colour and answers Cancel (Don't allow) when clicked; the card is placed in the middle, Rapidkoi's dialog card look, with today's words and buttons unchanged.
- **Escape** answers Cancel (Don't allow) through the button's `.shortcut("Escape")`, only while the question shows.
- **Tied to its tab:** `Confirm.Forget(tab)` and `Approval.tab` decide where a question shows: over the content area only while that tab (or the new tab, for its approval) is in front. Switching tabs, opening the panel or the shield, and closing the panel don't drop it; closing its tab does.
- **Forget acts on its own tab:** `Msg.Forget` forgets the tab the question names, not `m.current`.
- **The ⚙ card** still steps aside while Forget is asked (PRP-052), since the question is now in the app's area beside it.

### Must NOT Do
- Change any question's words, or add any.
- Move About, renaming or naming a separate app off the root dialog.
- Let a question for one tab act on, or show over, another.
- Add a kivuli feature or work around one: everything used is in 0.66.

## DESIGN

**Decided (Claude; the owner reviews them here):**
1. **`Confirm.Forget(i64)`** carries its tab; asking it (from ⚙, from a tab's menu, from `navigate`) records the tab in front. `confirm` stays one field: one question at a time, as now.
2. **A question outlives the panel.** Today `ClosePanel`, `TogglePanel`, `PanelGo` and `PanelBack` clear `confirm`; they won't: a popover may report closing more than once (session 171), and the question no longer lives in the card. `Cancel`, Forget being done, and the tab closing clear it.
3. **No mark in the sidebar** for a tab with a question waiting: it was asked while you were on it, and the owner asked for no new words or marks.
4. **`window.la` is untouched** (500 lines): `Confirm` lives in `actions.la`; only its variant changes.
5. **Tests through `kivuli.testing`:** the view's texts and buttons, `click` on Cancel / Forget, `press` Escape, and the frame's controls still clickable while a question shows (they aren't inert).

## TESTS
- [ ] `test/questions.la`: Forget asked on tab A shows over the content area, with the frame (+, ☰, the sidebar's tabs) still clickable; switching to tab B shows no question; back on A it shows again; Forget there forgets A even when the message comes while B is in front.
- [ ] Cancel, Escape and a click on the scrim each clear the question; closing tab A clears it.
- [ ] An app's approval shows over its area with Allow / Don't allow, and Escape is Don't allow; the new tab's approval shows over the new tab.
- [ ] Renaming, naming a separate app and About still use the root dialog.
- [ ] Every existing test of Forget, approvals and the panel passes, updated for `Confirm.Forget(tab)`.

## ACCEPTANCE
- [ ] Every test above passes; no file over 500 lines.
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass.
- [ ] For the owner, by eye: Forget from ⚙ dims only the app's area; the sidebar, address box and ☰ still work; switching tabs and back keeps the question; Escape and a click on the dim cancel; an app asking for a permission (the new tab's Restore, say) shows the same way.
