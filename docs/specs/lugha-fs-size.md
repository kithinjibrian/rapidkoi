# Lugha request: a file's size without reading it

From Rapidkoi, DECISION-015 and PRP-056 (DECISION-089, single files). For a Lugha session in `../lugha`, under Lugha's own CLAUDE.md and milestone process. Rapidkoi never builds or installs `lughac` itself; the owner does.

## What Rapidkoi needs

Rapidkoi lets an app open one of the person's files, chosen in the system's dialog, up to 16 MB. Today `std.fs` can say whether something is a file (`fs.is_file`) but not how big it is (checked: `lughac spec`, "The standard library", `std.fs`'s table, lughac 0.1.0). So the only way to know a file's size is `read_bytes`, which reads all of it. A 4 GB video chosen by mistake would be read whole, holding that much memory and, as any blocking read does, the window, before it could be refused. The owner chose to wait for a size query rather than ship that (2026-10-31: "wait for file size query").

Rapidkoi already reads whole files first in one place it accepted: a local app module, capped at 32 MB (DECISION-015, since 2026-10-17). It would use the query there too.

## What to add

```
fs.size(path): Result<i64, string>
```

- The size in bytes of the regular file at `path`, following symbolic links (`stat(2)`'s `st_size`), as `is_file` follows them.
- `Err` with the system's message when nothing is there or it can't be read (`No such file or directory`, `Permission denied`), as `std.fs`'s other functions word theirs; a folder or anything not a regular file is `Err` (for example `"not a file"`), so a caller never mistakes a folder's size for a file's.
- A path holding a NUL byte is `Err("path contains a NUL byte")`, as elsewhere in `std.fs`.
- No other effect: nothing is opened for reading.

Its name and exact errors are Lugha's to decide. A `metadata(path)` returning a struct (size, kind, and later a modification time) would serve as well, if Lugha prefers one call to grow; Rapidkoi needs only the size now. A modification time would later help it notice a file changed on disk without reading it, but it checks by hash today and doesn't need one.

## How Rapidkoi will use it

- **Opening a person's file (PRP-056):** `fs.size(path)` before `read_bytes`; over 16 MB is refused with "This file is over 16 MB" without reading a byte.
- **Opening a local app module:** the same check against 32 MB, resolving DECISION-015.

Both run in a task, so a slow disk doesn't hold `update`.

## What Rapidkoi does until then

PRP-056 waits (the owner's choice). DECISION-015 records the request.
