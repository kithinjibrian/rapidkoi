# Signing in to your app's server

A Rapidkoi app can prove to its own server who the person is to that app, without passwords and without Rapidkoi holding the app's tokens (PRP-011, PRP-012). This page says exactly what the server checks. `test/signin.la` runs a server that does exactly this against Rapidkoi's real signing.

## What the person has

For each signed app, Rapidkoi makes a key pair for the person, on their computer:

- The **public key** is 32 bytes, Ed25519. It is the person's ID *to this app only*: another app sees a different key, so apps can't tell they share a person.
- The secret half never leaves Rapidkoi. An app can't read it.
- "Start over as someone new" (the Account page) gives a new key, and the old account can't be used from Rapidkoi again.

## The three steps

The app does these through Rapidkoi's effects (`identity`, `sign`, `fetch`). The Rust library's `SignIn` does all three.

1. **The app fetches a challenge:** `GET https://<host>/rapidkoi/challenge`.
   - The server answers `200` with 16 to 1024 random bytes as the body (32 is plenty).
   - At the same time, the app asks Rapidkoi for the person's public key (`identity`).
2. **The app asks Rapidkoi to sign the challenge** (`sign`). Rapidkoi signs this message with the app's key, and nothing else:

   ```
   CBOR map, in this order:
     "kind"      → text  "rapidkoi sign-in"
     "host"      → text  the app's verified host, as its address shows it
     "challenge" → bytes the challenge, unchanged
   ```

   - `host` is the name the app was opened from, for example `notes.acme.com`, or `localhost:8000` while developing. A port that isn't the default is part of it.
   - The map is canonical CBOR as Rapidkoi writes it (definite lengths, the keys in the order above).
   - Rebuild it byte for byte; don't parse it from anything the app sends.
3. **The app posts the proof:** `POST https://<host>/rapidkoi/sign-in` with `content-type: application/cbor` and the body

   ```
   CBOR map:
     "key"       → bytes  the public key (32)
     "challenge" → bytes  the challenge from step 1
     "signature" → bytes  the signature from step 2 (64)
   ```

   The server answers `200` with whatever the app should keep, usually a session token, or `401`. The app keeps the token in its own state and sends it as a header on later requests. Rapidkoi stores no cookies and adds no credentials of its own.

## What the server checks

Each of these is required. Skipping any one lets someone sign in as someone else.

1. **The challenge is one you issued, and hasn't been used.** Keep issued challenges (in memory or a store), each with an expiry of a few minutes, and delete one as soon as a proof uses it, whether the proof passes or fails. A replayed proof must fail.
2. **The host in the message is your own.** Rebuild the message with your own host name, never one the request supplies. A signature made for another site then fails to verify, so a site can't relay your challenge to the person and use what comes back.
3. **The signature verifies** with Ed25519, using the posted `key`, over the rebuilt message.
4. **The sizes are right:** the key is 32 bytes, the signature 64, and the challenge is the one you issued.

When all four pass, the person is whoever owns `key` *for your app*. Treat the key as the account's ID: the first time you see a key, that's a new account.

## What it doesn't do

- **It isn't an email or a phone number.** The key says nothing about the person outside your app. Names are only placeholders the app shows (`{FIRST_NAME}`), never sent.
- **Signed out is no proof.** If the person signed out of your app on Rapidkoi's Account page, `sign` returns nothing, and the app has nothing to post.
- **Unsigned apps can't sign in.** Only an app opened from a verified, signed site gets a key, because the key is bound to that site's host.
