## FEATURE: [one sentence]

## OBJECTIVE
[2–3 sentences describing what "done" looks like from the user's perspective: what they see and can do in the window]

## CONTEXT

- Starting state: [which files currently exist and are relevant]
- Ending state: [which files will be created or modified]
- Related existing code: [specific file paths to read before starting, including in ../kivuli]
- Related source files: [docs/source/… notes that shaped this feature]
- Lugha spec sections relied on: [`lughac spec` sections; confirm each feature exists]
- kivuli API used: [widgets, modifiers, commands, subscriptions; confirm each exists in ../kivuli]
- Open decisions that must be resolved first: [DECISIONS.md entries that block this feature]

## IMPLEMENTATION REQUIREMENTS

### Must Do
- [specific requirement]

### Must NOT Do
- [explicit exclusion, and why]

## MODEL AND MESSAGES

- Model fields added or changed: [name: type, meaning]
- Messages added: [variant(payload): when it is sent]

## ERROR HANDLING REQUIREMENTS

- [Which failures return `Result` and with which error enum variants]
- [How `update` turns each `Err` into model state, and what the view shows]
- [Which conditions are programmer errors and panic, and the panic message]

## SECURITY CONSIDERATIONS

- [External input (files, args, env, network) and how it is validated]
- [New package, `extern` or `[link]` dependencies, and why]
- [Data that must never be logged or shown]
- [If auth, crypto, payments, secrets or processes apply: human review required before merging]

## TESTS TO WRITE

List the specific test cases before any implementation begins. Pure ones need no display:
- [ ] Happy path: [describe]
- [ ] Error path: [each error variant]
- [ ] Edge case: [describe]
- [ ] View (`kivuli.testing`): [what is pressed or typed, and what the next model/view must show]

## ROLLBACK PLAN

If this feature needs to be abandoned mid-implementation:
- Branch to return to: [branch name]
- State the codebase should be in: [describe]

## ACCEPTANCE CRITERIA
- [ ] [testable criterion]
- [ ] `lughac test test/` passes
- [ ] `lughac build src/main.la -o main` succeeds
- [ ] `lughac fmt --check src test` is clean
- [ ] Every `public` item has a `/** … */` doc comment
- [ ] No file over 300 lines
- [ ] CHANGELOG.md updated

## VALIDATION
Run these commands to verify completion:
- `lughac fmt --check src test`
- `lughac test test/`
- `lughac build src/main.la -o main`
- [feature-specific check]
- For the user: [what to open and click to confirm it visually]
