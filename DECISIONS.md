# DECISIONS.md — rapidkoi

Tracks architectural and design questions that are open, deferred, or resolved.

Rules:
- Every open decision blocks implementation of the code it affects.
- The AI must not implement anything that depends on an open decision.
- When a decision is resolved, move it to the RESOLVED section and record the outcome.
- Once resolved, copy the outcome to MEMORY.md as an architectural decision.

---

## OPEN — Requires human input before implementation

### DECISION-003 — Which WebAssembly runtime the shell embeds

**Status:** open
**Raised:** 2026-10-17 — Session 2
**Resolved by:** human
**Blocks:** Build path step 1 (the shell): loading and running any bundle.

**Question:** Which C-API WebAssembly runtime does the shell link through `[link]` and `extern`s?

**Options:**
- A) **wasmtime** (C API). Mature, fast (Cranelift), fuel/epoch limits to stop runaway guests, memory limits, strong security record. Large; not in Ubuntu's apt, so it's a prebuilt tarball or a source build. *Recommended.*
- B) **WAMR** (wasm-micro-runtime). Small, interpreter or AOT, runs on Android too. Less hardened, more build configuration.
- C) **wasm3**. Tiny interpreter, trivial to build. Slow, and barely maintained.
- D) **wasm2c** (its `wasm-rt.h` is installed here). Rules itself out: it translates to C, so running a downloaded bundle needs a C compiler on the person's machine.

**Notes:** Every runtime call is a hand-written `extern` that has to match the C header exactly (SECURITY RULE). The sandbox is only as strong as this runtime.

---

### DECISION-004 — Where an app's state lives

**Status:** open
**Raised:** 2026-10-17 — Session 2
**Resolved by:** human
**Blocks:** The guest↔host contract (DECISION-005), and so step 1.

**Question:** Does the guest own its state, or does the platform hold it?

**Options:**
- A) **Platform-held model.** The guest is Elm-shaped: it exports `init`, `update(model, msg) → model` and `view(model) → tree`. The model is serialisable bytes that Rapidkoi stores. Links to exact state, history, snapshots, live sessions and offline sync then all work on the platform's copy, for every app, with no extra code in the app. Costs: every app's state has to be serialisable, and there's a copy per update. *Recommended: it is what makes signature features 1, 2 and 4 possible.*
- B) **Guest-held state.** The guest keeps its own memory and just emits view trees. Simpler for app authors and faster, but time travel, snapshots and live sessions each need the app's cooperation (a save/restore hook), so they stop being free.

**Notes:** Very hard to change once apps exist. See `docs/source/stakeholder/vision-2026-10-17.md`.

---

### DECISION-005 — The guest↔host UI contract

**Status:** open
**Raised:** 2026-10-17 — Session 2
**Resolved by:** human
**Blocks:** Step 1: rendering anything.

**Question:** How does a bundle hand its view tree to the shell, and how do events get back to it?

**Options:**
- A) **A serialised tree per render.** The guest writes the whole view tree into its linear memory in a versioned, compact encoding (CBOR or a custom binary format) and returns pointer and length. The shell decodes it into kivuli `View`s, and kivuli diffs. Events go back as an encoded message to an exported function. One crossing per render, language-neutral, easy to inspect (it fits the inspector feature). *Recommended.*
- B) **Host builder imports.** The guest calls imported functions (`ui_text`, `ui_button`, …) to build the tree. Many crossings per render, and the order of calls becomes the API.
- C) **Component Model + WIT.** A standard interface with generated bindings for many languages. wasmtime's C API support for components is limited, and Lugha would need its own bindings.

**Notes:** This is the platform's core API: its encoding needs a version field from day one.

---

### DECISION-006 — The first guest language and test bundles

**Status:** open
**Raised:** 2026-10-17 — Session 2
**Resolved by:** human
**Blocks:** Step 1: having a bundle to load.

**Question:** What language is the first guest UI library and demo bundle written in, and where does it live?

**Options:**
- A) **Rust** (`wasm32-unknown-unknown`), as in the vision's example, in this repo under `guest/rust/`. Hand-written `.wat` fixtures for the shell's own tests, so `lughac test` doesn't need Rust. *Recommended.*
- B) **Lugha** compiled to WebAssembly. Not possible today: `lughac` emits native code only.
- C) **C** with clang's wasm target. Small, but a poor model for the app-author experience.

**Notes:** The demo bundle's code is not in the shell's Lugha test suite; it needs its own build command in CLAUDE.md.

---

## DEFERRED — Acknowledged, not yet needed

### DECISION-002 — Publishing

**Status:** deferred
**Raised:** 2026-10-17 — Session 1
**Revisit when:** The user wants a remote, or a first release.

**Question:** Where is the repository hosted (e.g. `kithinjibrian/rapidkoi` on GitHub), public or private, and under what license?

**Notes:** Safe to defer: work is local-only until then, and no push happens without asking.

---

### DECISION-007 — Platforms beyond GTK desktop

**Status:** deferred
**Raised:** 2026-10-17 — Session 2
**Revisit when:** Work on Android, macOS or Windows starts.

**Question:** kivuli renders to GTK 4 only. How do Android (and native macOS/Windows) get a renderer?

**Notes:** Safe to defer: the vision's build path is desktop first, and Linux is the development machine. Keeping the guest contract (DECISION-005) independent of GTK keeps this open.

---

### DECISION-008 — Process isolation for apps

**Status:** deferred
**Raised:** 2026-10-17 — Session 2
**Revisit when:** Before the shell runs any bundle it didn't load from a local file the user chose (build path step 2).

**Question:** The vision says each app runs separately from Rapidkoi itself. In-process sandbox only, or a separate process per app?

**Notes:** In step 1 the person only opens local bundles they picked, and the runtime's sandbox plus fuel and memory limits contain a faulty one.

---

### DECISION-009 — Manifest, bundle format and signing

**Status:** deferred
**Raised:** 2026-10-17 — Session 2
**Revisit when:** Build path step 2 (links and addresses).

**Question:** The manifest's format and location at a domain, the bundle's container format, content addressing, and how domain-tied keys are verified.

**Notes:** Signing is cryptography: human review is required (SECURITY RULE). Step 1 loads a bare `.wasm` file.

---

## RESOLVED

### DECISION-001 — What rapidkoi is for

**Status:** resolved
**Raised:** 2026-10-17 — Session 1
**Resolved:** 2026-10-17 — Session 2

**Question:** What does rapidkoi do, and for whom?

**Outcome:** A native app for running apps: each app lives at an address, every screen has a link carrying route and state, and apps are sandboxed, signed WebAssembly bundles that describe their UI as a component tree Rapidkoi renders. Build path: shell → links → a real app → storage and sync → live sessions → history.

**Rationale:** The project owner's vision, recorded in `docs/source/stakeholder/vision-2026-10-17.md`.

**Copied to MEMORY.md:** yes
