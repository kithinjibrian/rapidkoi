## FEATURE: The frame has no back or forward buttons: moving around inside an app is the app's own UI.

## OBJECTIVE
From chat with the owner (sessions 173–174). The header's ← and → go, and so does ⌂ (the app's home), since all three only ever meant navigating inside an app. Moving between an app's screens is the app's own business: it draws its own "‹ Inbox" where it makes sense, as on iOS. Rapidkoi keeps only what no app can do itself: a way back to the app you came from when a link opened another one ("‹ Mail"). That arrives with links between apps, not now.

## WHY (the owner's discussion, recorded)

- Browsers have ← and → because their pages are untrusted documents from many sites that can't see each other; the browser must own the way back. Most apps have neither (iOS: none; Android: back only; desktop apps: mostly none), and forward is rarely used anywhere.
- In Rapidkoi each app has one tab, and apps can draw buttons. What only Rapidkoi can do is return across apps, because apps can't see each other.
- Without frame navigation, Rapidkoi needs no pages, routes, stacks or move-replay from apps: the design question that started this (pages vs a stack vs moves) goes away.
- Links still need something from apps (open at a place; say where you are when sharing). That's its own, smaller question, for links (build step 2).

## CONTEXT

- Starting state: `src/ui/header.la` `start(m)` shows ← ⌂ → on an app page, all disabled ("… (comes with links)"); nothing on an empty tab. `test/header.la` checks them. The vision says "Navigate with back and forward, which follow each app's routes" (`docs/source/stakeholder/vision-2026-10-17.md`), and MEMORY.md decision 4 summarises it.
- Ending state:
  - `src/ui/header.la`: `start(m)` is empty on every tab; `Icon.Back`, `Icon.Forward` and `Icon.Home` aren't drawn.
  - `src/ui/icons.la`, `assets/icons/`: `Back`, `Forward` and `Home` stay. They're unused but kept for the "‹ Mail" return and apps' own buttons later; deleting and re-adding Iconsax files is churn.
  - `Msg.Home` stays: it's "show the empty tab", used by the window and tests (PRP-024).
  - `test/header.la`: the ← ⌂ → tests become "the header starts empty on every tab".
  - DECISIONS.md: DECISION-041 (resolved): no back or forward in the frame. MEMORY.md decision 4 amended (the vision's back/forward line), plus a decision entry.
  - CLAUDE.md KNOWN ISSUES ("Back, forward, share, settings and more are shown disabled" loses back and forward), DESIGN.md (the header), CHANGELOG.md.
- Open decisions: none.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- Remove ←, ⌂ and → from the header on every tab; nothing takes their place for now.
- Record the decision and amend the vision summary, so later PRPs (links) don't bring them back by default.

### Must NOT Do
- Build the "‹ Mail" return now: no app can open another app's link yet (links, build step 2; architecture rule 5).
- Give Alt+← or a mouse's back button a meaning yet: that comes with an event for apps, decided with links.
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- None.

## ERROR HANDLING REQUIREMENTS

- None.

## SECURITY CONSIDERATIONS

- None.

## TESTS TO WRITE

`test/header.la`:
- [ ] The header's start is empty on an app page and on an empty tab: no Back, Home or Forward button.
- [ ] Account and menu are still there (unchanged tests).

## ROLLBACK PLAN

- Branch: `prp-017-kivuli_0_17`; `git revert` the commit.

## ACCEPTANCE CRITERIA
- [ ] No ←, ⌂ or → in the header on any tab
- [ ] `lughac test test/` passes (the owner runs it); build ok; fmt clean
- [ ] DECISIONS.md, MEMORY.md, CLAUDE.md, DESIGN.md, CHANGELOG.md updated

## VALIDATION
- For the user: `./main`, open an app: the header starts with the address box; no ←, ⌂ or →.

## TO CONFIRM ON APPROVAL
- ⌂ goes too. The owner said "drop back and forward"; ⌂ was the app's own home, which is the same kind of in-app navigation.
