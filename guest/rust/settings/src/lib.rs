//! Rapidkoi's Settings (PRP-038, redesigned in PRP-075): a built-in app,
//! packed into `./main` like the new tab. Categories down the side
//! (Appearance, Startup, You, Publishers, About Rapidkoi), Find above the
//! page, and a details column beside You's and Publishers' lists. Only
//! what's Rapidkoi-wide: each app's own choices stay in ⚙ and App info.
//!
//! It reads the person's choices (any app may), who they are to their apps
//! (`read_you`) and the keys they trust (`read_publishers`), and changes
//! the choices and their name with `change_settings` (DECISION-100;
//! `scripts/settings.sh` declares them). Each change applies at once;
//! Rapidkoi tells every app, this one included.
//!
//! Build and pack: `scripts/settings.sh`, which writes `assets/apps/settings.rkb`.

mod find;
mod lists;
mod pages;

use find::{found, Page};
use minicbor::{Decode, Encode};
use rapidkoi::{App, Ask, Event, Message, Next, Preference, Prefs, Publishers, Start, State, View, You};

/// The text size's stops, in percent (PRP-075): closer together near 100.
const STOPS: [u16; 8] = [80, 90, 100, 110, 125, 150, 175, 200];

/// What Settings' screen sends.
#[derive(Debug, Clone, PartialEq, Message)]
enum Msg {
    /// Open the category at this index (`find::PAGES`).
    Go(u8),
    /// Find's text changed: the event's value.
    Find,
    /// Light (`true`) or dark.
    Theme(bool),
    Contrast,
    /// The text size slider moved: its stop's index is the event's value.
    Size,
    /// Open with the last session (`true`), or a new tab.
    Start(bool),
    /// Show this app you're signed in to in the details.
    PickApp(u32),
    /// Show this key in the details.
    PickKey(u32),
    /// Your first or last name changed: the event's value.
    First,
    Last,
    /// Ask Rapidkoi for its card with your backup words (PRP-076).
    BackupWords,
    /// Ask Rapidkoi for its card to type your backup words into.
    UseBackupWords,
}

/// An app you're signed in to, as Settings keeps it.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
struct Signed {
    #[n(0)]
    name: String,
    #[n(1)]
    place: String,
    /// The name it sees.
    #[n(2)]
    first: String,
    #[n(3)]
    last: String,
    /// When you first opened it; 0 when not known.
    #[n(4)]
    since: i64,
    /// Your public key for it, as its fingerprint; empty when Rapidkoi couldn't say.
    #[n(5)]
    key: String,
    /// `icon:PLACE`: Rapidkoi draws it.
    #[n(6)]
    icon: String,
}

/// A key you trust, as Settings keeps it.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
struct Key {
    #[n(0)]
    host: String,
    #[n(1)]
    key: String,
    #[n(2)]
    since: i64,
    #[n(3)]
    signs: Vec<String>,
    #[n(4)]
    rotations: u32,
}

#[derive(Default, Clone, State)]
struct Settings {
    /// Whether Rapidkoi answered the choices yet.
    read: bool,
    /// The category open, by its index.
    page: u8,
    /// What's typed in Find.
    find: String,
    light: bool,
    more_contrast: bool,
    text_size: u16,
    restore: bool,
    /// Whether your backup words are written down: never a word (PRP-076).
    backed_up: bool,
    /// The name every app shows.
    first: String,
    last: String,
    apps: Vec<Signed>,
    keys: Vec<Key>,
    /// Which app and which key the details show.
    app: u32,
    key: u32,
}

impl Settings {
    /// The category shown: the open one, or Find's first match when it isn't one.
    fn shown(&self) -> Option<Page> {
        let matches = found(&self.find);
        let open = Page::at(self.page);
        if matches.contains(&open) {
            Some(open)
        } else {
            matches.first().copied()
        }
    }

    /// The text size's stop: the one it's at, or the nearest below.
    fn stop(&self) -> usize {
        STOPS.iter().rposition(|s| *s <= self.text_size).unwrap_or(0)
    }

    fn change(self, p: Preference) -> Next<Self> {
        Next::state(self).ask(Ask::set_preference("set", p))
    }

    fn named(self) -> Next<Self> {
        let ask = Ask::set_name("name", &self.first, &self.last);
        Next::state(self).ask(ask)
    }

    fn read_all(self) -> Next<Self> {
        Next::state(self).ask(Ask::preferences("prefs")).ask(Ask::you("you")).ask(Ask::publishers("keys"))
    }
}

impl App for Settings {
    type Msg = Msg;

    fn handle(self, event: Event<Msg>) -> Next<Self> {
        match event {
            Event::Opened | Event::Resume | Event::Change { .. } => self.read_all(),
            Event::Data { msg, cbor } => match msg.as_str() {
                "prefs" => match Prefs::read(&cbor) {
                    Some(p) => Next::state(Settings {
                        read: true,
                        light: p.light,
                        more_contrast: p.more_contrast,
                        text_size: p.text_size,
                        restore: p.start == Start::Restore,
                        ..self
                    }),
                    None => Next::state(self),
                },
                "you" => match You::read(&cbor) {
                    Some(y) => {
                        let apps: Vec<Signed> = y
                            .apps
                            .into_iter()
                            .map(|a| Signed {
                                name: a.name,
                                place: a.place,
                                first: a.first,
                                last: a.last,
                                since: a.since,
                                key: a.key,
                                icon: a.icon,
                            })
                            .collect();
                        let app = if (self.app as usize) < apps.len() { self.app } else { 0 };
                        let backed_up = y.backed_up;
                        Next::state(Settings { apps, app, first: y.first, last: y.last, backed_up, ..self })
                    }
                    None => Next::state(self),
                },
                "keys" => match Publishers::read(&cbor) {
                    Some(p) => {
                        let keys: Vec<Key> = p
                            .items
                            .into_iter()
                            .map(|k| Key {
                                host: k.host,
                                key: k.key,
                                since: k.since,
                                signs: k.signs,
                                rotations: k.rotations,
                            })
                            .collect();
                        let key = if (self.key as usize) < keys.len() { self.key } else { 0 };
                        Next::state(Settings { keys, key, ..self })
                    }
                    None => Next::state(self),
                },
                _ => Next::state(self),
            },
            Event::Message { msg, value } => match msg {
                // Picking a category clears Find.
                Msg::Go(i) => Next::state(Settings { page: i, find: String::new(), ..self }),
                Msg::Find => Next::state(Settings { find: value.unwrap_or_default(), ..self }),
                Msg::Theme(light) => self.change(Preference::Light(light)),
                Msg::Contrast => {
                    let on = !self.more_contrast;
                    self.change(Preference::MoreContrast(on))
                }
                Msg::Size => match value.and_then(|v| v.parse::<usize>().ok()).and_then(|i| STOPS.get(i)) {
                    Some(size) => {
                        let size = *size;
                        Settings { text_size: size, ..self }.change(Preference::TextSize(size))
                    }
                    None => Next::state(self),
                },
                Msg::Start(restore) => {
                    let start = if restore { Start::Restore } else { Start::NewTab };
                    Settings { restore, ..self }.change(Preference::Start(start))
                }
                Msg::PickApp(i) => Next::state(Settings { app: i, ..self }),
                Msg::PickKey(i) => Next::state(Settings { key: i, ..self }),
                Msg::First => Settings { first: value.unwrap_or_default(), ..self }.named(),
                Msg::Last => Settings { last: value.unwrap_or_default(), ..self }.named(),
                // Rapidkoi's own cards: the words never come here.
                Msg::BackupWords => Next::state(self).ask(Ask::backup_words("backup")),
                Msg::UseBackupWords => Next::state(self).ask(Ask::use_backup_words("restore")),
            },
            _ => Next::state(self),
        }
    }

    fn view(&self) -> View<Msg> {
        pages::screen(self)
    }
}

rapidkoi::export!(Settings);

#[cfg(test)]
mod tests;
