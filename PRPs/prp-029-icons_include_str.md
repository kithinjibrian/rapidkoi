## FEATURE: Rapidkoi's icons are read from `assets/icons/` at compile time with `include_str`, not pasted into source.

## OBJECTIVE
From chat with the owner (session 162; Lugha milestone 128). Nothing changes on screen. `src/ui/icons.la` names each SVG file instead of holding its text, so `assets/icons/` is the only copy: adding or changing an icon means changing the file, then rebuilding.

## CONTEXT

- Starting state: `src/ui/icons.la` holds each Iconsax SVG as a string literal (PRP-019, when Lugha couldn't embed files); `test/icons.la` keeps each equal to its file in `assets/icons/`.
- Ending state:
  - `src/ui/icons.la`: `svg(i)` returns `include_str("../../assets/icons/<file>")` for each icon; `file(i)`, `recolour`, `icon` and `ALL` unchanged. The module's doc comment says where the icons come from.
  - `test/icons.la`: the equality test goes (the file is the only source); in its place, every icon in `ALL` is an SVG (`<svg` at its start) holding the white (`#ffffff`) that `recolour` swaps.
  - CLAUDE.md (the `assets/icons/` line: no longer "change both together"), DESIGN.md if it says the same, CHANGELOG.md.
- Lugha spec sections relied on: section 5, "Included files" (`include_str`, E0447, E0448); confirmed in the installed lughac.
- kivuli API used: none new.
- Open decisions: none (DECISION-038 resolved, session 162).

## IMPLEMENTATION REQUIREMENTS

### Must Do
- One literal path per icon (`include_str` takes a literal only), relative to `src/ui/`.
- Every icon draws exactly as before: same bytes, same image keys.

### Must NOT Do
- Change any icon, its colour or size.
- Merge into `redesign` or `main` (session 90).

## MODEL AND MESSAGES

- None.

## ERROR HANDLING REQUIREMENTS

- None at run time: a missing, oversized or non-UTF-8 file is a compile error (E0447, E0448).

## SECURITY CONSIDERATIONS

- None: the files are Rapidkoi's own, inside the package, and apps never reach them.

## TESTS TO WRITE

`test/icons.la`:
- [x] Every icon in `ALL` starts with `<svg` and holds `#ffffff` (so `recolour` has something to colour).
- [x] The existing recolour and image-key tests stay.

## ROLLBACK PLAN

- Branch: `prp-017-kivuli_0_17`; `git revert` the commit.

## ACCEPTANCE CRITERIA
- [x] No SVG text left in `src/ui/icons.la`
- [ ] `lughac test test/icons.la` passes (the owner runs it); build ok; fmt clean
- [x] CLAUDE.md, CHANGELOG.md updated

## VALIDATION
- `lughac check src/main.la`, `lughac test test/icons.la`
- For the user: `./main`: the header and address box icons look as before.
