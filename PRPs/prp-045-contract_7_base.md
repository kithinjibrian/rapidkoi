## FEATURE: Contract 7's base: one contract, older ones gone, link fields with kinds, labels on typing boxes

## OBJECTIVE
Rapidkoi runs contract 7 apps only. An older app is refused with "This app needs to be rebuilt for this Rapidkoi". Contract 7 is contract 6 (named-field state, links, boxes, texts, inputs, styles, roles, storage) with two fixes: a link can set a field that starts empty (an `Option` in Rust), and a typing box has a `label`. Rapidkoi's own apps (the new tab, Settings) run on it. For the person nothing changes, except that the address never shows a placeholder value like `note=0`.

PRP-045 of three for contract 7 (DECISION-056): this base, then scrolling (PRP-046), then pictures (PRP-047). Contract 7 stays unreleased until all three are in; the SDK's v0.2.0 follows.

## CONTEXT
- Agreed with the owner (2026-10-24, discovery Q1–Q7, DECISION-056), with "don't be afraid of breaking changes": contract 7 only, older code deleted (Q5); `label` on typing boxes and link fields with declared kinds (Q6); three PRPs, one contract (Q7).
- Today: contracts 2–6 all run (DECISION-039, 048, 049, 052): version checks in 12 files (`guest/events.la`, `guest/contract.la`, `guest/running.la`, `guest/links.la`, `window/restarts.la`, `net/loads.la`, `data/states.la`, …), contract 2's envelope, contract 3's sealed state, contract 4's `column`/`row`/`button`/`text` nodes beside contract 5's `box`/`text`/`input`. 24 test files use fixtures written for contracts 2–6 (`test/support.la`: `fixed`, `echo`, `echo_v2`, `echo_v3`, `echo_opening`, `linking`, `echo_v6`, `counting`, `crashing`).
- The specs: `docs/specs/contract-4.md`, `contract-5.md`, `contract-6.md`, `app-events.md` become one `docs/specs/contract-7.md`; the old ones are deleted (git keeps them).
- Decisions: DECISION-056 (contract 7, written with this PRP); DECISION-050 (accessible labels, kivuli); DECISION-039, 048, 049, 052 amended (older contracts no longer run).

## Q1: the SDK and the built-in apps (needs the owner)

Rapidkoi's new tab and Settings are contract 6 bundles built with `rapidkoi-rust` v0.1.0. Once this PRP lands, Rapidkoi refuses them, and the SDK only speaks contract 7 after work in `../rapidkoi-rust`, which a Rapidkoi session never writes.
- **(a) Recommended:** this PRP's Rapidkoi side is built and tested first (fixtures only, no Rust); then a `rapidkoi-rust` session moves the SDK to contract 7's base and tags `v0.2.0-pre.1`; then the built-ins move to that tag and are repacked here, and this PRP merges. PRP-046 and PRP-047 do the same (`pre.2`, `pre.3`); `v0.2.0` is tagged after PRP-047. `main` never holds a Rapidkoi that refuses its own built-ins.
- (b) The built-ins are rewritten in Lugha… not possible: apps are WebAssembly built with the SDK.

## REQUIREMENTS

### Must Do
- **`rapidkoi_version` 7 only.** Any other version is refused before the app runs: "This app needs to be rebuilt for this Rapidkoi", on the same screen as other refusals. A kept copy (PRP-043) of an older app is refused the same way, and the next open asks its site.
- **Delete everything for contracts 2–6:** the version checks and their branches; contract 2's `lifecycle`/`envelope`/`answer` events; sealed state bytes; contract 4's node kinds and `tree` decoding (only contract 5's `box`, `text`, `input` remain, through `tree5.la`, renamed `tree.la`); `events.la`'s per-version naming (only `on_` names); the tests for them.
- **Link fields with kinds.** The `on_init` result may carry `kinds`: a map from a field's path (as `link` names it, 1–4 segments) to `"number"`, `"text"` or `"bool"`, at most 16. When a link opens a fresh window, each of its fields is set where `kinds` names it, parsed as that kind (a value that doesn't parse is ignored), as well as where the fresh state already holds a value of that kind (today's rule). So a field that starts `null` (`None`) can be set by a link. A bad `kinds` (not a map, an unknown kind, a bad path, too many) stops the app with its sentence.
- **`label` on typing boxes.** `input` takes an optional `label` (1–200 bytes). Rapidkoi names the typing box by it (kivuli `.name`, so tests find it), else by its placeholder; screen readers hear it once kivuli sets accessible labels (DECISION-050).
- **`docs/specs/contract-7.md`:** one spec for the whole contract: state, links (with `kinds`), events, the tree (box, text, input with `label`), styles, roles, storage, the limits. Contracts 4–6's specs and `app-events.md` deleted.
- **The test fixtures rewritten** for contract 7 (`test/support.la`): one family of guests replying with contract 7's results, the same jobs as today's (a fixed tree, an echo of each event, a counter, a crash, storage).
- **The built-ins** move to the SDK's contract 7 (Q1) and are repacked; `test/newtab.la` and `test/settings_app.la` pass on them.
- Core decisions and DECISIONS: DECISION-056 written; 039, 048, 049, 052 amended; CHANGELOG; CLAUDE.md's STACK (contract 7 only).

### Must NOT Do
- No scrolling or pictures: PRP-046 and PRP-047.
- No contract version but 7; no compatibility code kept "just in case".
- No change to what an app can reach (permissions, storage, network) beyond what's above.

## DESIGN
- `guest/guest.la`: `rapidkoi_version` must be 7 (`Refusal` with the sentence).
- `guest/contract.la` and `guest/tree5.la` → `guest/tree.la`: one tree reader. `guest/events.la`: one encoder, `on_` names.
- `guest/links.la`: `set_fields(fresh, link, kinds)`; `kinds` read from the `on_init` result by `contract.read_update` and kept with the page while it starts.
- `guest/nodes.la`: `Typing` gains `label: Option<string>`; `ui/render.la` names the box `label ?? placeholder`.
- `window/tabs.la`, `window/restarts.la`, `net/loads.la`, `data/states.la`: the version branches go.

## TESTS
- [ ] A guest exporting `rapidkoi_version` 2–6 (and 8) is refused with the sentence; 7 runs
- [ ] A kept copy of an older app is refused and the next open asks the site
- [ ] `kinds` lets a link set a field that starts null; a value that doesn't parse as its kind is ignored; a bad `kinds` stops the app with its sentence
- [ ] A typing box with a `label` is found by it; without one, by its placeholder; a bad label (empty, over 200 bytes, not text) stops the app
- [ ] Every existing behaviour still covered, on contract 7 fixtures: events, links, storage, styles, roles, effects, sign-in, restore, keeping, upgrade
- [ ] The new tab and Settings run on the SDK's contract 7

## ACCEPTANCE
- [ ] Every test above passes; no test or source mentions contracts 2–6 except DECISIONS' history
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build src/main.la -o main` pass
- [ ] For the owner, by eye: the new tab and Settings as before
