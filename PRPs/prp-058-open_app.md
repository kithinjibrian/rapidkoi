## FEATURE: "Open App…": pick a local app file (`.rkb`, `.wasm`) in the system's chooser instead of typing its path

## OBJECTIVE
☰'s card gains **Open App…** at its top, above Settings, with a line between; Ctrl+O does the same from anywhere. The system's file chooser opens, showing `.rkb` and `.wasm` files; the chosen one opens exactly as typing its path into the address box does today (the current tab, or the empty tab it's on). Cancelling does nothing. Typing a path keeps working.

## CONTEXT
- **Decisions:** DECISION-020 (kivuli's file dialog, shipped in 0.66, "an Open… item for local apps can be its own PRP"); the owner's words (2026-11-01): **"Open App…"**. None open.
- **kivuli 0.66:** `Cmd.open_file(title, filters, chosen)`, `FileFilter`; `kivuli.testing`'s `chooser`, `answer_chooser`; `.shortcut("Ctrl+O")` on a button.
- **CLAUDE.md:** opening a local app file the person chose is one of `update`'s allowed exceptions (≤ 32 MB), as typing its path already is.
- **Files to read first:** `src/ui/header.la` (`menu_card`, `item`), `src/window/actions.la` (`Msg`), `src/shell.la` (`OpenSettings`, the address's submit), `src/window/navigate.la` (`open_file`).
- **Changes:** `header.la` (the item and its shortcut), `actions.la` (`Msg.OpenApp`, `Msg.AppChosen(Option<string>)`), `shell.la` (two arms), CHANGELOG.md; tests in `test/menus.la`.

## REQUIREMENTS

### Must Do
- **The item:** "Open App…" first in ☰'s card, a separator after it, then Settings, About Rapidkoi, Quit as now. It closes the card.
- **Ctrl+O** opens the same chooser from anywhere in the window.
- **The chooser:** `Cmd.open_file` with the title and filter name the owner gives (question below), patterns `*.rkb` and `*.wasm`.
- **Chosen:** `navigate.open_file(m, path)`, the same path a typed address takes, so links, pins, "where you left off", the 32 MB cap and every refusal behave as today. **Cancelled:** nothing changes.

### Must NOT Do
- Change how a typed path opens, or the address box.
- Any word on screen beyond "Open App…" and the chooser's title and filter name.

## DESIGN
- **Where Ctrl+O lives:** on the "Open App…" item itself (kivuli builds a closed popover's content too, and its shortcuts work from anywhere in the window). If on screen it doesn't fire while the card is closed, that's a kivuli question, recorded and asked, never worked around.
- **One chooser at a time:** kivuli answers a second chooser `None`, so Ctrl+O while an app's file dialog is open does nothing.

**The owner's answer:** the chooser's title **"Open an app"** and its filter's name **"Rapidkoi apps"** ("approved, use your suggestions").

## TESTS
- [ ] `test/menus.la`: the card's items in order ("Open App…" first); clicking it sends `Msg.OpenApp`; `press(view, "Ctrl+O")` sends it too.
- [ ] `OpenApp` closes the card and returns a chooser (`kivuli.testing.chooser`) with the title, the filter's name and `["*.rkb", "*.wasm"]`.
- [ ] `answer_chooser(cmd, Some(path))` of a fixture bundle opens it in the current tab, as typing it does; on an empty tab it fills that tab; `answer_chooser(cmd, None)` changes nothing.

## ACCEPTANCE
- [ ] Every test above passes; no file over 500 lines.
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build -O 2 src/main.la -o main` pass.
- [ ] For the owner, by eye: ☰ shows "Open App…" first; it and Ctrl+O open the chooser with the right title and filter; a `.rkb` opens; Cancel does nothing.

## BUILD NOTES

- **Status:** approved (the owner, 2026-11-01); built.
- `Msg.OpenApp` closes ☰'s card and returns `Cmd.open_file`; `Msg.AppChosen(Some(path))` goes to `navigate.open_file`, as a typed path does; `None` does nothing. Ctrl+O is the item's `.shortcut`, found by `kivuli.testing.press` on the whole window's view (the card is built while closed).
- `shell.la` is at 499 lines.
