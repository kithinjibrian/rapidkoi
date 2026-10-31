# kivuli request: the system's open and save dialogs

From Rapidkoi, DECISION-089 (`docs/specs/files.md`, milestone M1). For a kivuli session in `../kivuli`, under kivuli's own CLAUDE.md and PRP process. Against kivuli `v0.65.0`.

## What Rapidkoi needs

Rapidkoi runs untrusted apps that may ask for one of the person's files. The person chooses it in the system's own dialog, and choosing is the permission: the app never names a file. So Rapidkoi needs to open the desktop's file chooser, for opening and for saving, and to hear back which file was chosen, or that the person cancelled.

Today kivuli has no way to open one (checked: `src/core/effects.la`'s `Cmd`, `README.md`'s command table), and `../lugha-gtk` binds no `GtkFileDialog` and no `GAsyncReadyCallback` (checked: `grep -ri file_dialog`, `_finish`). Whether the bindings go into lugha-gtk first is kivuli's to decide.

## 1. A command that opens the chooser

GTK 4.10+ has `GtkFileDialog`: `gtk_file_dialog_open`, `gtk_file_dialog_save` and `gtk_file_dialog_select_folder`, each asynchronous (a `GAsyncReadyCallback`, then `…_finish` giving a `GFile`, or an error when cancelled). On desktops with xdg-desktop-portal it runs the portal's chooser, outside the app's process, which is what Rapidkoi wants.

Suggested API (kivuli's to decide):

```
Cmd.choose_file(Chooser {
    mode: Choosing.Open,          // Open, Save, or Folder (Folder can come later)
    title: "Open a file for Notes",
    filters: [Filter { name: "Pictures", patterns: ["*.png", "*.jpg", "*.jpeg"] }],
    suggested: None,              // Save: a suggested file name
}, fun(chosen: Option<string>) = Msg.Chose(chosen))
```

- **Answered exactly once:** `Some(path)`, the chosen file's absolute local path (`g_file_get_path`), or `None` when the person cancelled, the dialog failed, or the choice has no local path (a remote location the portal can't mount).
- **Modal to the window**, as `gtk_file_dialog_*` with the app's window as parent.
- **Filters:** a name and glob patterns each (`GtkFileFilter`, `gtk_file_filter_add_pattern`); none means every file. They only guide the person: Rapidkoi never trusts a file's type.
- **Save:** `suggested` sets the initial name (`gtk_file_dialog_set_initial_name`); the dialog's own overwrite confirmation stays.
- **Only paths, never contents:** kivuli opens and reports; reading and writing the file stay the program's (Rapidkoi does it in a task, safely).
- **One dialog at a time per window** is the program's rule to keep; kivuli needn't enforce it, but should be fine if a second command arrives while one is open (open it after, or report `None`: kivuli's to choose, and to document).
- **Same timing as the other window commands** (`Cmd.focus`, `Cmd.copy_text`): run by the live window; in a program without a window (tests, a headless run), answered `None` at once.

## 2. Testing without a display

Rapidkoi's `update` is tested without GTK, through `kivuli.testing`. It needs to:

- **find** a `choose_file` command inside a `Cmd` (as tests find `Cmd.focus`), and read its `Chooser` (mode, title, filters, suggested name), to check the words the person sees;
- **answer** it: given the command and a path (or `None`), get the message it would send, so a test can play "the person chose `/tmp/x.png`" without a dialog.

Something like `kivuli.testing.chooser(cmd): Option<Chooser>` and `kivuli.testing.choose(cmd, Some(path)): Option<Msg>`.

## 3. Later: choosing a folder

`Choosing.Folder` (`gtk_file_dialog_select_folder`), the same shape, answered with the folder's path. Rapidkoi needs it for importing a folder and, later, live folder access (`docs/specs/files.md` 3.2, 3.3). Not needed for the first version: fine to leave out of the first tag if it's simpler, as long as the API has room for it.

## Notes for kivuli

- Generic: any kivuli app that opens or saves documents needs it.
- Cancelling isn't an error to the program: `None`.
- Nothing about the chosen file reaches GTK widgets in the app's view; the path goes only to the program's message.

## What Rapidkoi does until then

Nothing: no file reaches apps. DECISION-089 records the dependency; PRP-056 (the shell's side of single files) starts once this is tagged.
