## FEATURE: Rapidkoi's modules grouped into folders by what they're about.

## OBJECTIVE
A refactor with no change in behaviour. `src/` goes from 47 flat modules to two at the top (`main`, `shell`) and nine folders, so a reader finds a module by its area (the owner, session 91; the grouping agreed in session 92). Everything works exactly as before: the same window, the same commands, the same 320 tests passing.

## CONTEXT

- Starting state: every module flat in `src/` (`rapidkoi.layout`, `rapidkoi.verify`, …), on `prp-017-kivuli_0_17` (the owner: stay on this branch, session 90).
- Ending state, `src/`:

| Folder | Modules (module path `rapidkoi.<folder>.<name>`) |
|---|---|
| top level | `main`, `shell` |
| `window/` | `window`, `actions`, `tabs`, `leftoff`, `navigate` |
| `ui/` | `layout`, `addressbox`, `suggestions`, `content`, `frame`, `render`, `tokens` (was `theme`), `themes` |
| `guest/` | `guest`, `running`, `limits`, `contract`, `nodes`, `props`, `placeholders`, `effects` |
| `bundles/` | `bundle`, `rules`, `signing`, `loader`, `project` |
| `trust/` | `keys`, `index`, `verify` |
| `net/` | `network`, `requests`, `dialing`, `fetching`, `loads`, `address` |
| `data/` | `store`, `remembered`, `states`, `saving` |
| `people/` | `identity`, `people`, `account`, `access` |
| `commands/` | `pack`, `publish`, `serve` |

  - Every `import x;` inside `src/` becomes `import folder.x;`; the name it binds stays `x` (Lugha binds the last segment, as `import std.math;` binds `math`), so code using `render.Msg`, `navigate.go`, `window.page_of` doesn't change. `theme` → `tokens` changes its importers' names.
  - `examples/quote/server.la`: `rapidkoi.serve`, `rapidkoi.identity`, `rapidkoi.keys` → `rapidkoi.commands.serve`, `rapidkoi.people.identity`, `rapidkoi.trust.keys`.
  - Tests import the new paths. **`test/` stays flat** (see the decision below): `test/layout.la` still tests `src/ui/layout.la`; `test/theme.la`, if any, follows `tokens`.
  - CLAUDE.md (file tree, import order, the TESTING RULE's mirroring line), `docs/DESIGN.md` (paths), CHANGELOG.md.
- Related source files: `lughac spec` section 6 (files are modules named by path; a package's own folders; a folder may not share a name with a listed package: `kivuli`, `wasmtime`, `cbor`, none of which is a folder here).
- Lugha spec sections relied on: section 6, confirmed. kivuli API: none.
- Open decisions that must be resolved first: DECISION-033 (below), answered with this PRP's approval.

## DECISION-033: tests can't sit in folders

A test file's imports start from its own folder, so `test/ui/layout.la` can't import `test/support.la` (E0312, checked in session 92). Options:
- **A) `test/` stays flat**, each file named for the module it tests. Recommended: no copies, no tooling change; the folders are visible in each test's imports.
- B) A copy of `support.la` in each test folder: nine copies to keep alike.
- C) Ask for a Lugha change letting a test import from `test/`'s top: a lughac change in `../lugha`, not Rapidkoi's to make.

## IMPLEMENTATION REQUIREMENTS

### Must Do
- Move each file with `git mv`, so history follows it.
- Update every import in `src/`, `test/` and `examples/quote/`; each module's `Depends on:` and `Used by:` lines name the new paths.
- Run `lughac fmt` (import lines may reflow).

### Must NOT Do
- Change any behaviour, name (other than `theme` → `tokens`), signature or test assertion.
- Name a folder after a listed package.
- Merge into `redesign` or `main`, or open a new branch (session 90).

## MODEL AND MESSAGES

- None.

## ERROR HANDLING REQUIREMENTS

- None.

## SECURITY CONSIDERATIONS

- None: no behaviour changes.

## TESTS TO WRITE

- None new: a refactor's test is that every existing one passes unchanged (320), and the example server's own tests (`lughac test examples/quote/server.la`).

## ROLLBACK PLAN

- `git revert` the PRP's commit on `prp-017-kivuli_0_17` (one commit: moves and import changes together).

## ACCEPTANCE CRITERIA
- [x] `src/` holds only `main.la`, `shell.la` and the nine folders
- [x] No test assertion changed; `lughac test test/` passes (320)
- [x] `lughac test examples/quote/server.la` passes
- [x] `lughac build src/main.la -o main` succeeds; `lughac build examples/quote/server.la -o quote-server` succeeds
- [x] `lughac fmt --check src test examples` is clean
- [x] No file over 500 lines
- [x] CHANGELOG.md and CLAUDE.md updated

## VALIDATION
- `lughac fmt --check src test examples`
- `lughac test test/` and `lughac test examples/quote/server.la`
- `lughac build src/main.la -o main`
- For the user: `./main` looks and behaves as before.
