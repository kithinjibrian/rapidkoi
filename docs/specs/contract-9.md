# Contract 9

The one contract Rapidkoi speaks (DECISION-056, PRP-045, PRP-053, PRP-054): an app is a WebAssembly module exporting `rapidkoi_version` (9), `alloc`, `handle(state, event)` and `view(state)`. Anything older is refused: "This app needs to be rebuilt for this Rapidkoi". Contract 7 gathered contracts 3–6 (PRP-030, 036–039, 041) with link fields with declared `kinds` and a `label` on typing boxes, then scrolling (PRP-046) and pictures (PRP-047). Contract 8 (PRP-053) adds texts made of parts (your name and times, filled in by Rapidkoi, never known to the app) and the `now` ask, and drops the `{FIRST_NAME}` and `{LAST_NAME}` tokens. Contract 9 (PRP-054, DECISION-087) adds blobs: an app's large bytes, kept beside its database. Any change after that is a DECISIONS.md entry and a version bump.

## The state

A window's state is a CBOR map with text keys at the top level; the values may be anything (lists, maps, bytes). Rapidkoi keeps it as the map's CBOR and hands it back to `handle` and `view`. Everything the app keeps lives in it: an app keeps nothing between calls (DECISION-045).

## `handle`'s result

```
{ "state": { "count": 3, "notes": [...] },
  "effects": [...],
  "link": ["count"],
  "kinds": { "count": "number", "note": "number" } }
```

- **`state`:** required, a map of named fields.
- **`effects`:** required, at most 8 asks (below).
- **`link`:** absent keeps the address's fields; `null` clears them; a list of names sets them, at most 16, each 1–64 bytes, a path of 1–4 segments of `a`–`z`, `0`–`9` and `_` joined by `.` (`editor.task`), none twice. Values are read from the state after every event, never given in `link`.
- **`kinds`:** what kind each link field is, by its path: `"number"`, `"text"` or `"bool"`, at most 16. Rapidkoi reads it from the `on_init` result, to set a link's fields (below). The Rust library writes it from the state's types.

A named field holding a number, text or true/false goes in the address; one holding null, or missing, is left out. A list, a map or bytes stops the app ("This app put a list in its address"), as does a value over 1 KB. A path through anything but a map, or to a missing name, is left out.

## Links

```
https://counter.dafifi.net/@0.3.0?count=3
        └──── the app ────┘└ver─┘└ the app's fields ┘
```

- The app's own domain, so it's a real web address; `rapidkoi://` and, for development, `http://localhost:PORT` of the same shape open too.
- `/@MAJOR.MINOR.PATCH` is the version. Rapidkoi's own parts live in the path (`@`; `~` is reserved); the app's fields in the query, percent-encoded. Limits: 16 fields, a 64-byte name, a 1 KB value, 4 KB of query.
- The address box shows the whole link, the running version written in.

**Opening a link:**
1. A link with fields opens a new window of its app; a bare address of an app that's open switches to it.
2. The version it names opens while the site's signed index lists it; a withdrawn one is refused; one no longer listed opens the current version with "This link was made with Counter 0.3.0; opened 0.5.0".
3. `on_init` makes the fresh state.
4. Each field is set where the fresh state has that path holding the same kind of value (a number for a number, text for text, true/false for true/false), **or holding null where `kinds` names its kind**. A value that doesn't parse as its kind is ignored. Nothing on the way is created.
5. `on_open`: the app checks what the values mean; its "not found" is its own.

A window keeps the fields its address names through Restore (the `open` file stores them).

**The landing page:** `./main publish` writes `site/index.html` (the app's name, description, publisher, **Open in Rapidkoi**), which a site serves for every path outside `/.well-known/rapidkoi/`, so a link opened in a browser lands there.

## Events

Every event is a map `{"event": "on_…", …}`, sent to `handle`:

| Event | When | Carries |
|---|---|---|
| `on_init` | A fresh start, before anything else: the first open, a Restart after a crash, a changed state version, Start over | |
| `on_open` | Every start, fresh or from saved state; right after `on_init` (and a link's fields) on a fresh start | |
| `on_resume` | Back to the front while running: its tab shown again; for the new tab, the empty tab shown. Never right after `on_open` | |
| `on_change` | Something it may read changed while it's in front (pins, where you left off, its permission settings); the person's light/dark, contrast or text size; or its data in another of its windows | `tables`, for its data |
| `on_message` | You pressed or typed in its screen | `msg`, and an input's text as `value` |
| `on_answer` | A reply to an ask (a key, a signature, the new tab's reads, the person's choices, a database's rows or what changed) | `msg`, `value` |
| `on_response` | A request came back, whatever the status | `msg`, `value`: `{status, headers, body}` |
| `on_fail` | A request or a database call didn't work | `msg`, `value`: `{error}`, Rapidkoi's sentence |

An app that doesn't know an event changes nothing. There's no event when Rapidkoi's window goes to the back, an account changes, a server is switched off, a tab closes, Rapidkoi quits, or an app is forgotten.

## The screen

`view` returns one node: `{ "kind", "props", "children"? }`, at most 4 MB, 10,000 nodes, 64 deep.

| Kind | Props | Children |
|---|---|---|
| `box` | `style`, `role`, `checked`, `selected`, `disabled`, `label`, `on_press`, and `title` on the root | yes |
| `text` | `text` or `parts` (one of them, not both), `style`, `role`, `label` | no |
| `input` | `value` (required), `placeholder`, `label` (1–200 bytes), `lines` (1–20; more than 1 is a box of several lines), `on_change` (none: disabled), `on_submit` (Enter, on one line; sent with the text as `value`), `style` | no |
| `image` | `src` (required: `asset:NAME`, `blob:KEY` or an `http(s)://` address, at most 2 KB), `fit` (`contain`, the default, or `cover`), `color` (an `asset:` SVG only), `label` (1–200 bytes), `on_load`, `on_fail`, `style` | no |

- A **box with `on_press`** is pressable: focusable, and pressed by a click, Enter or Space; it sends `on_message` with its message. `disabled: true` greys it, and it sends nothing.
- A typing box's **`label`** says what it's for: Rapidkoi names the box by it, and screen readers hear it. Without one, its placeholder names it.
- Any other kind is shown as an unsupported placeholder, its subtree still counted against the limits.
- **Parts** (PRP-053): a text's `parts` is a list of at most 64, each a string (shown as written), `{ "name": "first" | "last" }` (your name for this app; nothing when unset), or `{ "time": ms, "show": … }` (a moment, whole milliseconds since 1970-01-01 00:00 UTC, within years 1 to 9999). Rapidkoi fills them in while drawing, in your zone, 24-hour clock, day before month; the app never learns the result. `show`: `time` "14:32"; `date` "Thu 31 Oct" (the year when it isn't this one); `date_time` "Thu 31 Oct, 14:32"; `weekday` "Thursday"; `relative` on your calendar: "just now", "5 minutes ago", "in 3 hours", "yesterday", "tomorrow", "Mon", "next Sun", "Thu 24 Oct", "24 Oct 2023", kept current by Rapidkoi. A moment Rapidkoi can't place in your zone is shown in UTC, " UTC" after it. Parts exist only on texts: typing boxes, titles and labels are never filled. Braces in a string are only braces.

### Style

One map, every key optional, at most 32 keys. A known key with a bad value stops the app with its sentence; an unknown key is ignored.

| Key | Value |
|---|---|
| `direction` | `"column"` (default) or `"row"` |
| `gap`, `padding` | `"xs"`…`"xl"` or 0–256 px; `padding` also `[vertical, horizontal]` |
| `align` | `"start"`, `"center"`, `"end"`, `"stretch"` |
| `grow` | `true`: take the spare room along the parent's direction |
| `width`, `height`, `min_width`, `max_width`, `min_height`, `max_height` | 0–4096 px |
| `background`, `color` | a colour |
| `border` | `{ "width": 0–16, "color": colour }` |
| `outline` | `{ "width": 0–8, "color": colour }` |
| `corner` | `"small"`, `"medium"`, `"large"`, `"round"`, or 0–256 px |
| `shadow` | `"none"`, `"sm"`, `"md"`, `"lg"` |
| `opacity` | 0–1 |
| `size` | `"title"`, `"heading"`, `"body"`, `"small"`, `{ "pt": 6–72 }` or `{ "rem": 0.5–6 }` |
| `weight` | `"normal"`, `"medium"`, `"bold"` |
| `font` | `"body"`, `"mono"`, `"serif"`, `"sans"` |
| `italic` | `true` |
| `truncate` | `true`: a text stays on one line, cut with "…"; without it a text wraps to its box's width |
| `scroll` | `"vertical"`, `"horizontal"` or `"both"`, on a box's own look (below) |
| `hover`, `pressed`, `focus`, `disabled`, `checked` | a style map without these keys |

**A colour** is one of Rapidkoi's names (`background`, `surface`, `text`, `text_dim`, `accent`, `on_accent`, `border`, `danger`, `on_danger`), the app's own `#rrggbb`/`#rrggbbaa`, or `{ "mix": [colour, colour, 0–1] }` (no mix inside a mix). Rapidkoi's names, spacing, corner and text sizes are resolved when Rapidkoi draws: the app never learns their values, and they follow the theme. A pressable box starts with no look but an accent ring when it has the keyboard; a text in the theme's text colour; a typing box in Rapidkoi's field look.

### Scrolling

Rapidkoi builds apps, not sites (PRP-046): an app's page is exactly the window's content area, and Rapidkoi never scrolls it. The app lays its screen out for the window and marks what scrolls:

- A box with `scroll` shows what fits and scrolls the rest: as tall as its content until its parent runs out of room, or, with `grow`, the room left. Its sizes bound the scrolled area.
- A scrolling box inside another that scrolls the same way (`both` shares both) stops the app: "This app put a scrolling box inside another that scrolls the same way". Across is fine: a row of thumbnails in a list.
- What still doesn't fit is cut off at the window's edge, and never makes the window bigger: silently in a published app; in development (a local file or a `localhost` address), with a striped band along the cut edge and "Bottom overflowed by 42 px" (or Right) in its corner.
- A `vertical` box scrolls only up and down (what's wider is cut), and a `horizontal` one only sideways.

### Pictures

PRP-047. A picture comes from the app's bundle or from a server it may reach, never anywhere else, and Rapidkoi loads it: the app names it in its view, as a web page names an `<img>`.

- **From the bundle:** `asset:NAME`, packed from `[assets]` in `rapidkoi.toml`. A bundle holds PNG and JPEG (at most 4 MB, 4,096 px a side) and SVG (at most 1 MB, UTF-8). Shown at once. With `color`, an SVG's `currentColor` is drawn in that colour (Rapidkoi's names follow the theme).
- **From a server:** an address `fetch` could reach (the app's own host or a server it declares; HTTPS; `http://localhost` from this machine). Downloaded in the background with a plain GET (no headers, no cookies), at most 6 at a time a window, in the order the screen shows them, each within 30 seconds and 10 MB; PNG or JPEG, at most 4,096 px a side and 16 MP, checked from its header before it's kept. WebP waits on kivuli (DECISION-058). Kept while the window lasts; past 256 MB decoded, those off screen go, least recently shown first.
- **From your blobs** (PRP-055): `blob:KEY`, a blob your app stored. Read from this device, never passing through your app, under the same rules as a server's picture (PNG or JPEG, at most 10 MB, 4,096 px a side and 16 MP). No such key: "This app has no blob with that key". When a write of yours stores, replaces or removes blobs, every window of your app reads its `blob:` pictures again.
- **While it loads** its box is empty, at its style's size. **When it can't be shown** (refused, not found, too large, not PNG or JPEG), Rapidkoi's broken-picture mark takes its place, and the app runs on.
- **`on_load`** arrives as `on_message` with its `msg` once the picture is shown; **`on_fail`** as `on_message` with its `msg` and Rapidkoi's sentence as `value`. Neither is sent without a message, and each once while the picture stays on screen.
- **A broken view stops the app:** a bad `src`, an asset the bundle doesn't have, `color` on anything but a bundle SVG, a server the app may never reach, or more than 256 pictures in one screen. A server the person switched off only fails that picture.

### Roles

`role`: `button`, `checkbox`, `switch`, `tab`, `tab_list`, `heading`, `list`, `list_item`, `group`, `separator` (AccessKit's names). `checked` for a checkbox or switch only; `selected` for a tab or list item only; `disabled`; `label`, what a screen reader says when the text isn't enough. Rapidkoi uses roles for focus, pressing by keyboard and the checked look.

## Asks (`effects`)

Each has a `kind` and a `msg` (any CBOR value, carried back in the reply):

| Kind | What | Answered |
|---|---|---|
| `identity` | the person's key for this app (signed apps) | `on_answer`: bytes, or null |
| `sign` + `challenge` (1–1024 bytes) | a sign-in signature | `on_answer`: bytes, or null |
| `fetch` + `method`, `url`, `headers`, `body` or `body_blob` | a request to the app's own or declared servers (HTTPS; `http://localhost` from this machine); `body_blob` (a blob's key, PRP-055) sends that blob as the body, up to 16 MB, read once the request is allowed; never both | `on_response` or `on_fail` |
| `preferences` | `{ "theme", "more_contrast", "text_size" }` | `on_answer` |
| `now` | the time now in UTC, in milliseconds since 1970, rounded down to a second (PRP-053); nothing about where you are | `on_answer` |
| `set_preference` + `name`, `value` | Settings only (`change_settings`) | not answered |
| `sql`, `sql_batch` | its database (below) | `on_answer` or `on_fail` |
| `blob_get`, `blob_list` | its blobs (below) | `on_answer` or `on_fail` |
| `pins`, `left_off`, `open_pin` + `index`, `continue`, `session`, `restore_session` | the new tab's built-in permissions | `on_answer` |

A bad ask stops the app with its sentence.

## Storage

Every app gets its own SQLite database on this device: a signed app one for its host and origin key; a local development app one for its server's host and port, or its file's path; built-in apps none. The file is `databases/<sha256 hex>.db`, so no app's name becomes a path. It lasts until the person clears it (⚙ → Data → Clear…) or forgets the app. No permission is asked.

- **`sql`** `{ "msg", "sql", "params", "max_rows"? }`: one statement; answered `{ "columns", "rows", "cut" }` (`cut`: rows left out past `max_rows`, default and at most 10,000, or 4 MB).
- **`sql_batch`** `{ "msg", "steps": [{ "sql", "params" }] }`: every step in one transaction, all or nothing; answered `{ "changes", "last_rowid" }`, `changes` the sum over the app's steps. A step may also store or remove a blob (below).
- Values are SQLite's types as CBOR (integers, floats, text, bytes, null; a boolean is stored as 1 or 0), and only ever go in as parameters.
- Bounds: SQL 100 KB, 999 parameters, a value 4 MB, 100 steps; every call 1 second, 100 MB a database, 256 MB of SQLite's memory for all apps. Every connection is guarded by lugha-sqlite's `Guard.untrusted()`.
- When a write in one window changes tables, the app's other windows get `on_change` with `tables`. A window copies the rows it asked for into its state and asks again then.
- An app's writes run one at a time, across its windows; reads run beside them. Asks run side by side, so read what a write changed after its answer.
- Names starting `rapidkoi_` (tables, indexes, triggers, views) are Rapidkoi's: your SQL is refused them. Skip them when you list or copy your schema.

### Blobs

Large bytes (photos, files, your own formats) kept beside the database, each under a key you name (DECISION-087, `docs/specs/blobs.md`). Rapidkoi stores them by their hash, never looks inside them, never runs them, and never gives them to another app.

- **Storing and removing** are steps of `sql_batch`, committed with your rows:
  - `{ "blob_set": key, "bytes": <bytes>, "owner"?: { "table", "key" } }` stores or replaces a key's bytes;
  - `{ "blob_delete": key }` removes a key (none is nothing to do).
- A key is text, 1–256 bytes, no control characters. A blob is at most 16 MB; an app's blobs at most 1 GB together (apart from its database) and 10,000 in number. A batch over one fails whole: "This blob is over 16 MB", "This app's files are full (1 GB)", "This app has 10,000 blobs already".
- **Owners:** with `owner`, the blob belongs to that row: its table (one that already exists, before the batch, with a primary key of one column) and its key's value. Deleting the row deletes the blob as part of the same write, and so does dropping the table. A migration in one batch (new table, copy, drop, rename) keeps them. **Changing a row's primary key drops its blobs.** Changing which key a row names, without deleting the row, needs `blob_set` or `blob_delete`.
- **`blob_get`** `{ "msg", "key", "from"?, "len"?, "hash"? }`: answered `{ "bytes", "from", "size", "hash" }`, at most 4 MB from `from` (default 0); a larger blob is read in pieces. Pass the first answer's `hash` with later pieces: if the blob changed in between, `on_fail` "This blob changed while it was being read", and start over. No such key: "This app has no blob with that key".
- **`blob_list`** `{ "msg", "prefix"? }`: answered `{ "blobs": [{ "key", "size", "owner"? }], "cut" }`, in key order, at most 10,000 (`cut`: those left out).
- Your app keeps nothing between calls, so bytes you hold sit in the window's state (16 MB, saved with it): handle each piece as it arrives and keep the result, not the bytes.
- ⚙'s Data row counts blobs with the database; Clear and Forget remove them.

**Changing the database's shape is the app's to manage** (a note, not a rule of the contract; DECISION-066). The database belongs to the app: Rapidkoi never reads it, migrates it or keeps copies of it. What Rapidkoi guarantees is that only one version of an app uses its database at a time: every window of an app moves to a new version together (PRP-048). An *older* version can still open data a newer one changed: the publisher withdraws the newer version, a link names an older one, or an older build is published again. So:

- record the shape your version writes in `PRAGMA user_version`, and migrate in one `sql_batch`, so a failed step leaves the database as it was;
- on open, read `user_version` before writing it: a number higher than your version knows means newer data. Then it's your app's choice what to do: say so and ask the person to update, open read-only, or migrate down if you wrote that path. Never set `user_version` lower without migrating down.

## The person's choices

`preferences` gives `{ "theme": "dark" | "light", "more_contrast": bool, "text_size": 80–200 }` and nothing else; when one changes, every running app gets `on_change`. Text size sets the body size (`body`, `small`, `rem`, and Rapidkoi's named sizes follow it). Per app, ⚙'s **Use my colours** turns raw and mixed colours into Rapidkoi's, and **Use my text size** makes raw sizes follow too.

## Messages

A message is any CBOR value. The Rust library writes typed ones (`#[derive(Message)]`): a unit variant as its name (`"Add"`), one with fields as `["Toggle", 3]`, a part's wrapped as `["List", ["Toggle", 3]]`.

## Rust

`rapidkoi-rust` (`v0.2.0` once contract 7 was released; contract 8's parts and `now`: `docs/specs/rapidkoi-rust-contract-8.md`; contract 9's blobs: `docs/specs/rapidkoi-rust-contract-9.md`): `#[derive(State)]` encodes a struct as named fields, gives each field a typed link field (`Counter::count`, `App::editor.then(Editor::task)`), and writes `kinds`; `Next::state(s).link(field)`, `.no_link()`, `.ask(…)`; an `Option` field is "none" in the address and can be set by a link.
