# App events

What Rapidkoi tells an app, and when. A draft from chat with the owner (session 165): names and moments for contract 3. Nothing here is built yet.

## The rule for names

On the wire, every event is named `on_` and a verb in snake case: `on_init`, `on_open`. The name is what the app reacts to, the way Android (`onResume`) and the web (`onload`) name theirs. Libraries and apps name them however suits their language (the owner, session 165): Rust's `Event::Resume`, a `fn on_resume`, or anything else.

## The events

| Event | When Rapidkoi sends it | What the app does with it | Today |
|---|---|---|---|
| `on_init` | A fresh start, before anything else: the first open, a Restart after a crash, a new build that changed its state version, Start over. | Makes its first state. | `init` |
| `on_open` | Every time it starts up, fresh or from saved state: Continue, a pin, an address, Rapidkoi restarting, Reload with a new build. Right after `on_init` on a fresh start. | Asks for what it needs: its key, its data, its server's news. | `opened` |
| `on_resume` | It comes to the front while already running: its tab is clicked or switched back to, or the tab in front of it closes. The new tab gets it each time the empty tab is shown. Never right after `on_open`. | Refreshes what may have changed while it was away. | new |
| `on_pause` | It goes to the back while still running: another tab, or the empty tab, becomes current. | Stops what's only worth doing while seen. | new, later |
| `on_message` | You clicked or typed in its screen. Carries its own message, and an input's text. | Changes its state. | `message` |
| `on_answer` | Rapidkoi replies to something it asked for: its key, a signature, and, once A is built, the new tab's pins and where you left off. | Takes the reply into its state. | `answer` |
| `on_response` | A request it sent came back, whatever the status (a redirect too). | Reads the reply. | `answer` with a reply |
| `on_fail` | A request wasn't sent or didn't come back. Carries Rapidkoi's sentence why. | Says so, or tries again. | `answer` with an error |

`on_pause` is named now so it pairs with `on_resume`, and is built when an app needs it.

## Moments with no event

| Moment | Why no event |
|---|---|
| Rapidkoi's window goes to the back, or is minimised | Most platforms fold it into resume and pause. If it's wanted, it becomes `on_pause`/`on_resume` too, once kivuli tells Rapidkoi. |
| Its account changes (sign out, sign in, Start over) | The app finds out from its next `on_answer`. Start over is a fresh start (`on_init`). |
| A server is switched off or on | The app finds out from its next request (`on_fail`). |
| Its tab closes | Rapidkoi already keeps its state. A goodbye (`on_close`) can come later, if an app needs to tell its server. |
| Rapidkoi quits | State is saved on every change. |
| It's forgotten | It's closed first, then its state is deleted. |
| Rapidkoi stops it (a crash, a limit) | It can't run any more. |
| A new build replaces it (Reload) | The new build gets `on_open`. |

## Changing the contract

- Contract 2's `event` field holds `init`, `opened`, `message` and `answer`. Contract 3 uses the names above on the wire, and splits `answer` into `on_answer`, `on_response` and `on_fail`.
- An app that doesn't know an event changes nothing (the Rust library already says "more kinds will come"), so `on_resume` and `on_pause` break no published app.
- A DECISIONS.md entry records the names and the bump.

## Open

- **Words in Rapidkoi's own docs:** "resumed" has meant "started from saved state" so far. With `on_resume` it means "back to the front", and the old sense becomes "restored".
