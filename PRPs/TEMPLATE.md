## FEATURE: [one sentence]

## OBJECTIVE
[What "done" looks like for the person using Rapidkoi: what they see and can do.]

## CONTEXT
- Files to read first, and the files this creates or changes.
- Specs, sources and sibling APIs relied on (`lughac spec`, `../kivuli/`, …): confirm each exists.
- Decisions it depends on (DECISIONS.md): none may be open.

## REQUIREMENTS

### Must Do
- [specific requirement]

### Must NOT Do
- [exclusion, and why]

## DESIGN
[Only what isn't obvious: model fields and messages, contract changes, error enums and what the view shows for each, security notes (untrusted input, new dependencies, anything needing the owner's review).]

## TESTS
- [ ] [happy path, each error, edge cases, what's pressed or typed and what then shows]

## ACCEPTANCE
- [ ] [testable criterion]
- [ ] `lughac test test/`, `lughac fmt --check src test` and `lughac build src/main.la -o main` pass
- [ ] For the owner, by eye: [what to open and try]
