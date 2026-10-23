# Contract 4: a state of named fields, and links

PRP-036, DECISION-048. From chat with the owner, sessions 190–192. Contract 3 (`app-events.md`) with one change: Rapidkoi can read a window's state, so the app can put part of it in the window's address.

## The state

A window's state is a CBOR map with text keys at the top level. The values may be anything: lists, maps, bytes. Rapidkoi keeps it as the map's CBOR and hands it back to `handle` and `view` as bytes, as before.

Contract 2 and 3 apps hand back sealed bytes, run unchanged, and have no links.

## `handle`'s result

```
{ "state": { "count": 3, "notes": [...] },
  "effects": [...],
  "link": ["count"] }
```

- `link` absent: the address keeps the fields it named.
- `link: null`: the address names none.
- `link: [names]`: these top-level fields, in this order. At most 16 names, each 1–64 bytes of `a`–`z`, `0`–`9` and `_`, none twice.

The values are read from the state after every event, never given in `link`, so the address can't drift from the state.

A named field holding a number, text or true/false goes in the address. One holding null, or missing, is left out. A list, a map or bytes stops the app: "This app put a list in its address". A value over 1 KB stops it too.

## A link

```
https://counter.dafifi.net/@0.3.0?count=3
        └──── the app ────┘└ver─┘└ the app's fields ┘
```

- The app's own domain, so it's a real web address. `rapidkoi://` and, for development, `http://localhost:PORT` of the same shape open too.
- `/@MAJOR.MINOR.PATCH` is the version: `host@version` would be a web login, so it's never there.
- Rapidkoi's own parts live in the path (`@`; `~` is reserved). The app's fields live in the query, percent-encoded.
- Limits: 16 fields, a 64-byte name, a 1 KB value, 4 KB of query.
- The address box shows the whole link, the running version written in.

## Opening a link

1. A link with fields opens a new window of its app. A bare address of an app that's open switches to it.
2. The version it names opens while the site's signed index lists it. A withdrawn one is refused. One no longer listed opens the current version, with the notice "This link was made with Counter 0.3.0; opened 0.5.0".
3. `on_init` makes the fresh state.
4. Each of the link's fields is set in that state only where it has that name holding the same kind of value: a number for a number, text for text, true/false for true/false. Anything else is ignored. A null field takes nothing (its kind isn't known).
5. `on_open`: the app checks what the values mean. Its "not found" is its own.

A window keeps the fields its address names through Reload and Restore (the `open` file stores them).

## The landing page

`./main publish` writes `site/index.html`: the app's name, description and publisher, **Open in Rapidkoi** (the same link as `rapidkoi://`), and that it needs Rapidkoi. A site serves it for every path outside `/.well-known/rapidkoi/`, so a link opened in a browser lands there. `./main serve SITE/` does the same.

## Rust

```rust
#[derive(Default, State)]
struct Counter { count: i64, notes: Vec<String> }

fn handle(self, event: Event) -> Next<Self> {
    match event {
        Event::Message { msg, .. } if msg == "add" => {
            Next::state(Counter { count: self.count + 1, ..self }).link(Counter::count)
        }
        _ => Next::state(self),
    }
}
```

`#[derive(State)]` (crate `rapidkoi-derive`) encodes the struct as named fields and gives each field a typed link field (`Counter::count`). `Next::link` takes only simple types, so `.link(Counter::notes)` doesn't compile. `.no_link()` clears the address, `.ask(…)` asks Rapidkoi for something. `Next::from((state, asks))` keeps the address as it was.
