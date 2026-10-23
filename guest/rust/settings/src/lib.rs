//! Rapidkoi's Settings (PRP-038): a built-in app, packed into `./main` like
//! the new tab. It reads the person's choices (light or dark, more
//! contrast, text size) and changes them through the built-in
//! `change_settings` permission (`scripts/settings.sh` declares it). Each
//! change applies at once; Rapidkoi tells every app, this one included.
//!
//! Build and pack: `scripts/settings.sh`, which writes `assets/apps/settings.rkb`.

use rapidkoi::{
    button, checkbox, column, row, spacer, tabs, text, App, Ask, Event, Message, Next, Preference, Prefs, Space, State,
    View,
};

/// The smallest and largest text size, in percent, and each step.
const SMALLEST: u16 = 80;
const LARGEST: u16 = 200;
const STEP: u16 = 10;

/// What Settings' screen sends.
#[derive(Debug, Clone, PartialEq, Message)]
enum Msg {
    /// Light (`true`) or dark.
    Light(bool),
    Contrast,
    Smaller,
    Bigger,
}

#[derive(Default, State)]
struct Settings {
    /// Whether Rapidkoi answered yet.
    read: bool,
    light: bool,
    more_contrast: bool,
    text_size: u16,
}

impl Settings {
    fn change(self, p: Preference) -> Next<Self> {
        Next::state(self).ask(Ask::set_preference("set", p))
    }
}

impl App for Settings {
    type Msg = Msg;

    fn handle(self, event: Event<Msg>) -> Next<Self> {
        match event {
            Event::Opened | Event::Resume | Event::Change => Next::state(self).ask(Ask::preferences("prefs")),
            Event::Data { msg, cbor } if msg == "prefs" => match Prefs::read(&cbor) {
                Some(p) => Next::state(Settings {
                    read: true,
                    light: p.light,
                    more_contrast: p.more_contrast,
                    text_size: p.text_size,
                }),
                None => Next::state(self),
            },
            Event::Message { msg, .. } => match msg {
                Msg::Light(on) => self.change(Preference::Light(on)),
                Msg::Contrast => {
                    let on = !self.more_contrast;
                    self.change(Preference::MoreContrast(on))
                }
                Msg::Smaller if self.text_size > SMALLEST => {
                    let n = self.text_size - STEP;
                    self.change(Preference::TextSize(n))
                }
                Msg::Bigger if self.text_size < LARGEST => {
                    let n = self.text_size + STEP;
                    self.change(Preference::TextSize(n))
                }
                _ => Next::state(self),
            },
            _ => Next::state(self),
        }
    }

    fn view(&self) -> View<Msg> {
        if !self.read {
            return column([text("Settings").title()]).padding(Space::Lg).screen_title("Settings");
        }
        let line = |label: &str, control: View<Msg>| row([text(label), spacer(), control]).spacing(Space::Md);
        column([
            text("Settings").title(),
            text("Appearance").heading(),
            line(
                "Theme",
                tabs(&[("dark", "Dark"), ("light", "Light")], if self.light { "light" } else { "dark" }, |k| {
                    Msg::Light(k == "light")
                }),
            ),
            line("More contrast", checkbox("More contrast", self.more_contrast).on_toggle(Msg::Contrast)),
            line(
                "Text size",
                row([
                    button("−").on_press_if(self.text_size > SMALLEST, Msg::Smaller),
                    text(&format!("{}%", self.text_size)),
                    button("+").on_press_if(self.text_size < LARGEST, Msg::Bigger),
                ])
                .spacing(Space::Sm),
            ),
        ])
        .spacing(Space::Md)
        .padding(Space::Lg)
        .screen_title("Settings")
    }
}

rapidkoi::export!(Settings);

#[cfg(test)]
mod tests {
    use super::*;

    fn at(size: u16) -> Settings {
        Settings { read: true, light: false, more_contrast: false, text_size: size }
    }

    fn press(s: Settings, msg: Msg) -> Vec<Ask> {
        s.handle(Event::Message { msg, value: None }).asks().to_vec()
    }

    #[test]
    fn it_reads_on_open_and_on_every_change() {
        for e in [Event::Opened, Event::Resume, Event::Change] {
            assert_eq!(Settings::default().handle(e).asks(), &[Ask::preferences("prefs")]);
        }
    }

    #[test]
    fn each_control_asks_for_its_change() {
        assert_eq!(press(at(100), Msg::Light(true)), vec![Ask::set_preference("set", Preference::Light(true))]);
        assert_eq!(press(at(100), Msg::Contrast), vec![Ask::set_preference("set", Preference::MoreContrast(true))]);
        assert_eq!(press(at(100), Msg::Bigger), vec![Ask::set_preference("set", Preference::TextSize(110))]);
        assert_eq!(press(at(100), Msg::Smaller), vec![Ask::set_preference("set", Preference::TextSize(90))]);
    }

    #[test]
    fn text_size_stops_at_80_and_200() {
        assert_eq!(press(at(80), Msg::Smaller), vec![]);
        assert_eq!(press(at(200), Msg::Bigger), vec![]);
        // The buttons show it: disabled at the ends.
        assert_ne!(at(80).view(), at(90).view());
    }
}
