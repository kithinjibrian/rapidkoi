## FEATURE: An upgrade button in the address box when a newer version of the app is ready

## OBJECTIVE
When Rapidkoi's background check (PRP-043) finds and keeps a newer version of the app in front, and it reached the app's site, the address box shows the refresh-arrow icon left of the shield, with a small red dot for urgency. Hovering it says "A new version of this app is available: upgrade". Clicking it upgrades that window to the new version, keeping what the window had. Nothing else changes: no notice, and an app that's up to date shows no button.

## CONTEXT
- Asked by the owner (2026-10-24), after updates went silent (core decision 28) and Notes 0.2.1 couldn't be reached without closing the window: "show refresh-arrow.svg on the left of shield inside the address bar to update the app wherever a new version is available and online. have a red dot to show urgency and a tooltip".
- The icon: `assets/icons/refresh-arrow.svg` (the owner's, untracked until this PRP's commit).
- Files: `src/ui/icons.la` (the icon), `src/ui/addressbox.la` (the button), `src/window/tabs.la` (`Cached` gains `newer`), `src/net/loads.la` (`refreshed` sets it; the withdrawn screen's words), `src/shell.la` (the click), `src/ui/frame_looks.la` or `themes.la` (the dot's look). Tests in `test/addressbox.la`, `test/keeping.la`.
- Reuses `navigate.reload` (`Msg.Reload`, which nothing in the UI sends since session 101): a load from the site first, the window's state carried over, `state_version` guarding a changed shape; offline, the kept copy, which is the new version.
- Decisions: core decision 28 (amended by this PRP), DECISION-031 (colours from the theme). None open.

## Q1: checking while the app stays open (needs the owner)

Today the check runs once, when an app opens from its kept copy. An app left open never checks again, so a version deployed while it's open isn't offered until the next open.
- **(a) Recommended:** also check every 15 minutes while a signed app is open and in front, and when Rapidkoi's window comes back to the front. Each check fetches only the key and index (a few hundred bytes); the build only when it's new.
- (b) Only on open, as now: simplest, but the arrow shows only after closing and reopening.

## REQUIREMENTS

### Must Do
- **When it shows:** the window's check (`Refreshed.Checked`) kept a newer build than the one this window runs, and that check reached the site. Every open window of the same app running the older build shows it too.
- **When it doesn't:** the app is up to date; the last check couldn't reach the site; the app is unsigned, a file or built in (never kept).
- **The look:** the refresh-arrow at `ADDRESS_ICON`, recoloured to the theme's text like the shield, in a round button as the shield's, left of it. A red dot (the theme's `danger`, about 6 px) at its top right.
- **The tooltip:** "A new version of this app is available: upgrade".
- **Clicking it:** upgrades this window (`navigate.reload`): its state carried over, or started over with the existing notice if its shape changed. The button goes once the window runs the newest version.
- **The withdrawn screen** (PRP-043) loses its "⟳" sentence: it says only "{name} {version} was withdrawn: {reason}", and the upgrade button shows when a newer valid version was kept.
- Core decision 28 amended: a routine update still sends no notice, but the address box offers it.

### Must NOT Do
- No notice, badge or sound beyond the button and its dot.
- No automatic switch: only the click upgrades a running window.
- No general reload button: only this one, only when there's something to upgrade to.

## DESIGN
- `Cached` gains `newer: Option<string>`: the version kept and not yet running, set by `loads.refreshed` on `Checked(_, _, true, _)` (with the version), cleared by a load of that build.
- `Refreshed.Checked` carries the kept version (`Option<string>` in place of `bool`).
- No new message: the button sends `Msg.Reload`.

## TESTS
- [x] A check that keeps a newer build marks the window `newer`, and the address box shows the button with its tooltip
- [x] An unreachable check, an up-to-date app, and an unsigned app show no button
- [x] Another open window of the same app running the older build shows it too
- [x] Clicking it upgrades the window to the new version, its state carried over; the button goes
- [x] The withdrawn screen says only the withdrawal; with a newer valid version kept, the button shows

## ACCEPTANCE
- [x] Every test above passes
- [x] `lughac test test/`, `lughac fmt --check src test` and `lughac build src/main.la -o main` pass
- [ ] For the owner, by eye: with Notes open, deploy a new Notes version, open Notes again (a second open runs the check), see the arrow with its red dot and tooltip, click it: the new version runs and the arrow goes

## AMENDMENTS

- **Q1 (the owner, at approval):** (a). The 15-minute check is built (`CHECK_EVERY_MS`, `Msg.CheckForUpdates`, `loads.check_current`); the check when Rapidkoi's window comes back to the front waits on kivuli, which has no event for it (DECISION-054).
- **Every window of the app** means every tab with the same address running another version: `loads.offered`. A withdrawn window keeps its kept copy's facts (`stopped` keeps `cached`), so the newer version is offered there too.
- **The withdrawn screen's words** never named ⟳: they stay "{name} {version} was withdrawn: {reason}".
- **`subscriptions`** stays one subscription when there's one (the loading tick or the check), a batch only for both.
