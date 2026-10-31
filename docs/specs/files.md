# Files and folders: how apps reach the person's files (draft)

DECISION-089, open. A draft for the owner's one review: every answer is filled in. **Agreed** marks what the owner has said; everything else is Claude's recommendation and stands unless the owner objects. Nothing here is built.

The gap: an app can keep its own bytes (blobs, DECISION-087), but nothing the person owns can reach it. A photo can't be attached to a note, a document can't be opened and saved back, and a project can't be edited. The vision promises the shape: "When an app asks to open a file, Rapidkoi shows the file picker, and the app receives only the file the person chose" (`docs/source/stakeholder/vision-2026-10-17.md`).

The spec is in four parts:

1. **Fundamentals:** what every later choice answers to.
2. **The first version:** what's built first, and goes into the contract.
3. **Designed, not built:** how it grows, so the first version doesn't block it.
4. **The long-range target:** a code editor, the hardest app it should one day carry.

---

## Part 1 — Fundamentals

1. **The person decides, in Rapidkoi's screens, at the moment it's needed.** An app can't name a file and ask for it: choosing a file in Rapidkoi's chooser *is* the grant (the "powerbox" pattern: Android's document picker, macOS's sandbox, the browser's File System Access API; on Linux, GTK's chooser goes through xdg-desktop-portal).
2. **Every grant has three sides.** *Scope*: one file, several, a folder. *Rights*: read; write back to the same file; create, rename, delete. *Lifetime*: once; while the window is open; remembered. Each step up on any side is a bigger grant and needs its own reason and the owner's decision.
3. **The app learns the contents and the name, never where.** A path tells about the person (their name, their folders, other projects); a file's name, and names inside a granted folder, are what an app needs to show and link. No absolute path, ever, as CLAUDE.md already says of guests.
4. **Copy or reference, and the app says which.** A *copy* brings the bytes into the app's blobs: safe, offline, syncs, survives the file moving; edits stay inside until saved out. A *reference* is a handle to the real file: edit in place, see changes, but only on this device, and every write touches the person's disk.
5. **Integrity.** A write never leaves a half-written file, and never silently overwrites changes made elsewhere.
6. **Seeing and taking back.** A grant that outlives the moment is listed in ⚙ and can be revoked. A one-off copy needs neither.
7. **Two worlds of files.** *The system's files* (home folder, Downloads, a USB stick) are the desktop's; Rapidkoi reaches them only through its chooser. *Rapidkoi's own files* live inside Rapidkoi, sync between the person's devices and are shared between apps with consent (DECISION-078's "files"). Blobs are neither: one app's private bytes. **The system's files come first** (they're needed regardless); Rapidkoi's own files come with shared data and sync.
8. **It fits the other foundations.** A reference means nothing on another device, so only copies and Rapidkoi's own files travel with sync. A link can name a blob key, or a name inside a granted folder, never a path. Offline by default holds for copies.

---

## Part 2 — The first version

Single files: opened as a copy or to edit, saved back, saved as. Handles last while the window is open.

### 2.1 Rapidkoi's chooser

- Rapidkoi's open and save dialogs, through kivuli (GTK 4's `GtkFileDialog`; the portal where the desktop has one). Never drawn by the app.
- Titled with the app's name and what it will get, in the owner's words (open question 1): for example "Open a file for Notes", "Open a file for Notes to edit", "Save a file from Notes".
- One dialog at a time per window; an app asking while one is open gets `on_fail` "A file dialog is already open".
- Cancelled: the answer is `null`, never an error.

### 2.2 Opening a copy

`file_open { msg, key, accept? }`: the chosen file is read in a task and stored as a blob under `key` (replacing it, as `blob_set` does), ownerless. Answered `on_answer { key, name, size }`, or `null` when cancelled.

- `accept`: up to 16 extensions (`["png", "jpg"]`), only filtering what the chooser shows. Rapidkoi never trusts a type: it sniffs one only to show a picture.
- An ownerless blob is adopted by a row with a new `sql_batch` step, `{ "blob_own": key, "owner": { "table", "key" } }`, under blobs' owner rules (`docs/specs/blobs.md` 1.4). Without it, the blob lasts until `blob_delete`, Clear or Forget.

### 2.3 Opening to edit

`file_open { msg, key, accept?, edit: true }`: as 2.2, and the answer also carries `handle`: `{ key, name, size, handle }`. The handle is an opaque text the app keeps in its state; it reaches only that file.

### 2.4 Saving back

`file_save { msg, handle, key }`: Rapidkoi writes blob `key`'s bytes over the handle's file, in a task.

- **Safely:** a temporary file beside it, synced, renamed over the original, the folder synced. A crash leaves the old file or the new one.
- **Never over someone else's change:** Rapidkoi keeps the file's SHA-256 from when it was read (or last saved). If the file on disk differs, the save is refused with `on_fail` "This file changed since it was opened", and nothing is written. The app can offer to reload (`file_open` again) or save as.
- Answered `on_answer { size }`.
- The app can't name another file, create, rename or delete. The worst it can do is damage the one file the person handed it.

### 2.5 Saving as

`file_save_as { msg, key, name?, accept? }`: Rapidkoi's save dialog, `name` suggested; the blob's bytes written to the chosen file, safely. Answered `{ name, size, handle }` (a handle to edit it further), or `null` when cancelled. This is also how an app exports anything.

### 2.6 Handles

- Opaque, made by Rapidkoi, meaningless outside the window that got them.
- They last **while the app's window is open** (agreed lifetime for the first version, open question 2): closing the window, reloading the app or Forget ends them. A handle no longer valid answers `on_fail` "This file is no longer open here".
- Rapidkoi holds the path beside the handle in the window's model, never in the app's state, never saved to disk.

### 2.7 What the app learns

The file's **name** (its last part: `receipt.pdf`) and **size**. Never its path or folder, dates, owner, permissions or any other metadata.

### 2.8 Limits and sentences

- A file at most 16 MB in either direction (the blob limit, until write sessions and lughac file handles, `docs/specs/blobs.md` 3.3): "This file is over 16 MB".
- The app's blob quota applies to what's opened: "This app's files are full (1 GB)".
- A file that can't be read or written: "Rapidkoi couldn't read this file" / "Rapidkoi couldn't save this file". The system's message is never shown to the app.
- A missing `key` on save: "This app has no blob with that key".

### 2.9 Which apps, and the contract

- Any app with a blob store (signed and local development apps). The new tab and built-ins: "This app keeps no files here".
- The asks (`file_open`, `file_save`, `file_save_as`) and the `blob_own` step go into the contract. Whether that's an addition to contract 9 (as PRP-055's were) or contract 10 is open question 3.

---

## Part 3 — Designed, not built

Each is its own decision and PRP when an app needs it, and a capability for the owner's review. The first version's shapes are chosen so each fits without a rewrite: a handle names a file now and can name a folder later; every ask takes a handle and, later, a name inside it.

### 3.1 Several files at once

`file_open { …, many: true }`: several chosen files, each stored as its own blob under `key` plus a suffix (`key/1`, `key/2`…), answered as a list.

### 3.2 Importing a folder (a copy)

The person chooses a folder; Rapidkoi copies its files into the app's blobs within blob limits (and stops with a sentence past them), answering `[{ key, name, size }]` with names relative to the folder. Nearly free on top of 2.2; nothing new exposed.

### 3.3 Live folder access

The person chooses a folder; the app gets a folder handle. Asks take the handle and a **relative name** inside it:

- read rights first: `folder_list { handle, sub? }`, `folder_read { handle, name, key }` (into a blob);
- then write rights, a separate decision: `folder_write { handle, name, key }` (as 2.4, safe and conflict-checked), `folder_create`, `folder_rename`, `folder_delete`.

Rapidkoi refuses `..`, absolute names, and symlinks leading out of the folder, and checks each name again at the moment it acts. Listings are capped. This is what a gallery, a music player and a code editor need.

### 3.4 Watching for changes

`on_change` gains `files: [names]` when a handle's file, or a file inside a folder handle, changed on disk (inotify on Linux). So an editor reloads after a `git pull`.

### 3.5 Remembered grants

A handle that survives restarts (an editor's recent projects). Rapidkoi keeps the path, by the app's signed identity, in its data folder; ⚙ lists each app's grants with Revoke; a file or folder gone is a sentence, not a crash. Needed before 3.3 is comfortable to use.

### 3.6 "Open with…" and default apps

- A signed app's manifest declares file types it opens (by extension and kind) and whether it views or edits.
- Rapidkoi registers with the desktop as a handler (a `.desktop` file with `MimeType=`); kivuli's `on_open` already delivers files given on the command line.
- When a file arrives (from the desktop, or from another app's "open this attachment"), Rapidkoi's chooser lists the kept apps that declared its type, with "Always use this app"; a **Default apps** screen in Settings shows and changes every choice. No app becomes the default unless the person chose it. Defaults are keyed by the app's signed identity.
- The receiving app gets `on_opened { key, name, size, handle? }`: the same grant as 2.2 or 2.3. A sending app never learns which app received it.

### 3.7 Tasks between apps

Verbs on data ("share to", "view a picture", "pick a contact", "compose"), declared in manifests, chosen by the person the same way as 3.6. Designed with shared data (DECISION-078).

### 3.8 Rapidkoi's own files

Files that live inside Rapidkoi, sync between devices, and are shared between apps with consent: DECISION-078's "files". The chooser would offer "From this device" and "From Rapidkoi" side by side; the asks stay the same. Comes with sync and shared data.

### 3.9 Large files

Past 16 MB, files stream through blob write sessions (`docs/specs/blobs.md` 3.3), needing lughac file handles.

---

## Part 4 — The long-range target: a code editor

The hardest app this should one day carry, and the test of every choice above.

| It needs | From |
|---|---|
| A project folder, read and write, create, rename, delete | 3.3 |
| Reloading when files change on disk | 3.4 |
| Reopening the project after a restart | 3.5 |
| Opening a file from the file manager in it | 3.6 |
| Links to a file and line (`?file=src/main.la&line=40`) | names inside a folder handle (3.3), on this device |
| Search across the project | `folder_read` in steps, within each call's fuel |
| Syntax highlighting, cursors, large files | **a code-editing component** in the contract and kivuli (not files: its own decision; DECISION-081 lists rich text among the missing basics) |
| Completion, errors, go to definition | language tooling compiled to WebAssembly inside the app, or the app's own server; **whether Rapidkoi ever runs tools for apps** is the owner's question about the sandbox itself |
| Building, a terminal | the same question; until then, the system's own tools |
| Git | inside WebAssembly, on folder access |
| Projects shared live with another person | Rapidkoi's own files (3.8) and live sessions (build step 5) |

So, in stages: a single-file editor needs only Part 2 and a better text component; a project editor needs 3.3–3.5 and the code component; an IDE needs the owner's decision on tools.

---

## Dependencies

Requests to other repositories, under their own rules; never worked around here.

1. **kivuli: open and save dialogs.** A command that opens the system's chooser (GTK 4 `GtkFileDialog`, through the portal where there is one) with a title, filters and a suggested name, and sends back the chosen absolute path, or nothing when cancelled. A folder mode later (3.2, 3.3). **Needed for Part 2.**
2. **lughac: file handles** for files past 16 MB (3.9), and **file change notices** (inotify) for 3.4. Not needed for Part 2.

## For the owner's security review

- the three asks (2.2–2.5) and `blob_own`: host functions untrusted code can call;
- writing back to the person's files (2.4): what a hostile app could do with one file it was handed;
- the chooser's words (2.1): the consent itself;
- later, each of Part 3 on its own.

## Open questions for the owner

1. **The chooser's words.** Rapidkoi's own screens take only the copy the owner asks for. Suggested: "Open a file for {app}", "Open a file for {app} to edit", "Save a file from {app}".
2. **Lifetime of a handle in the first version.** Recommended: while the app's window is open (2.6); remembered grants later (3.5).
3. **Contract 10, or an addition to contract 9?** Recommended: an addition to 9, as PRP-055's were (apps on 9 never send the new asks, so nothing they do changes), unless the owner wants every new ask to bump the version.

## Order of work

1. The owner reviews this draft; DECISION-089 is resolved from it.
2. kivuli gets request 1 (`docs/specs/kivuli-file-dialogs.md`), built and tagged there.
3. A PRP for Part 2; rapidkoi-rust gains the asks; then the shell.
4. Notes attaching a photo (2.2) and a small text editor (2.3–2.5) prove it.
