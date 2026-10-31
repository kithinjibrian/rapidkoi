# Developers and users — talk in chat, 2026-10-31

The owner asked "what would incentivize devs to build for rapidkoi", then "thinking more in terms of capabilities the apps have", then "what about users". Mostly unanswered so far: the open questions are DECISION-081 and 082. One direction was taken: data shared between apps (DECISION-078).

## Why developers pick a platform

Reach, less work, money, freedom, their own skills. A new platform has no audience, so it wins first on less work and freedom.

- **Rapidkoi's strengths:** no backend to build or run (storage now; sync, live sessions and history from the platform and the Pond); near-zero cost to serve people; publish without approval or a cut; links into any screen; accessibility, permissions and export for free.
- **Weak today:** a Rust SDK only (most developers write TypeScript); Linux only; nothing decided for a link opened without Rapidkoi (the vision's `open.rapidkoi.app`; apps could run in a browser, being WebAssembly); no way to get paid (payments are security-sensitive); no way to be found; no users.
- **Claude's recommendation:** lead with "a whole app with no backend: offline, synced, multiplayer, private, free to run", for independent developers and small teams building local-first tools.

## Capabilities

What apps can do under contract 7: boxes, texts, typing boxes, pictures, scrolling, styles, roles; links with state; their own SQLite database; `fetch` to declared servers; a per-app key and signatures; three preferences.

- **Missing basics** (people leave over these): time and timers (contract 7 has neither), files through the picker, clipboard, notifications, a canvas, rich text, shortcuts, drag and drop, menus, camera, microphone, location, audio and video.
- **Only on Rapidkoi** (people come for these): the free backend; links with state; sign-in without passwords; data shared between apps with consent (the owner: "yes add it to the vision", DECISION-078); notifications without the developer's server, through the Pond; search, export and accessibility everywhere.
- **Claude's recommendation:** time, files and clipboard next, being small and unblocking many apps; the free backend as the headline, following the build path; shared data decided early, before apps keep contacts their own way.

## Users

Why people adopt: an app they want, a fix for something that annoys them, a link from someone, a better feel, a safe try.

- **Rapidkoi's strengths:** offline; no passwords; private (sandboxed, declared servers, a Pond that can't read); their data theirs; links to exact screens; consistent and accessible.
- **Missing:** apps they want; platforms beyond Linux; what a link shows without Rapidkoi; an installer (DECISION-002); finding apps; trust in a new platform.
- **Claude's recommendation:** Rapidkoi's own everyday apps (notes, tasks, calendar, contacts) as the first reason to install, each in its own repository like Notes (core decision 27), seeding shared data; privacy-minded people who want their data back as the first audience; decide early what a link shows someone without Rapidkoi.

## How to keep this

The owner asked whether to have users.md and devs.md. Not split by audience: most topics touch both, and two files would drift. Decisions stay in DECISIONS.md, discussions here, topic guides like `docs/sync/`; guides for developers and users later, built from the decisions (the owner: "yes").
