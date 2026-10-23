//! The network demo (PRP-012): the day's quote from the app's own server,
//! after signing in there with the person's key for this app.
//!
//! Build: `cargo build --release --target wasm32-unknown-unknown` in
//! `guest/rust/` (set `QUOTE_SERVER` for another server than
//! `http://localhost:8000`), then pack and serve it from `examples/quote/`.

use rapidkoi::{button, column, header, text, App, Ask, Event, Message, Next, Progress, Request, SignIn, Space, State, View};

/// What the quote's screen sends.
#[derive(Debug, Clone, PartialEq, Message)]
enum Msg {
    Again,
}

/// The app's own server: where it was published.
const SERVER: &str = match option_env!("QUOTE_SERVER") {
    Some(s) => s,
    None => "http://localhost:8000",
};

#[derive(Default, State)]
struct Quote {
    /// The day's quote, once it arrives.
    quote: Option<String>,
    /// Who the server says you are ("User 4f9a"), once signed in.
    who: Option<String>,
    /// The session token the server gave; kept in the state, never by
    /// Rapidkoi, and never in the address.
    token: Option<String>,
    /// Why the last step didn't work, as Rapidkoi or the server said it.
    problem: Option<String>,
    signing_in: SignIn,
}

impl Quote {
    fn fetch_quote(&self) -> Ask {
        let mut r = Request::get(&format!("{}/quote", SERVER));
        if let Some(t) = &self.token {
            r = r.header("authorization", &format!("Bearer {}", t));
        }
        Ask::fetch("quote", r)
    }

    /// Sign in, and fetch the quote meanwhile (it comes signed in once the token does).
    fn start(self) -> (Self, Vec<Ask>) {
        let (s, mut asks) = SignIn::start(SERVER);
        let next = Quote { signing_in: s, problem: None, ..self };
        asks.push(next.fetch_quote());
        (next, asks)
    }
}

impl App for Quote {
    type Msg = Msg;

    fn handle(self, event: Event<Msg>) -> Next<Self> {
        // Nothing of the quote's goes in its address.
        self.step(event).into()
    }

    fn view(&self) -> View<Msg> {
        self.screen()
    }
}

impl Quote {
    fn step(mut self, event: Event<Msg>) -> (Self, Vec<Ask>) {
        match self.signing_in.on(&event) {
            Progress::Asks(asks) => return (self, asks),
            Progress::SignedIn(token) => {
                let next = Quote { token: String::from_utf8(token).ok(), ..self };
                let ask = next.fetch_quote();
                return (next, vec![ask]);
            }
            Progress::Failed(why) => return (Quote { problem: Some(why), ..self }, vec![]),
            Progress::NotMine => {}
        }
        match event {
            Event::Opened => self.start(),
            Event::Message { msg: Msg::Again, .. } => self.start(),
            Event::Response { msg, status, headers, body } if msg == "quote" => {
                if status != 200 {
                    return (Quote { problem: Some(format!("The server answered {}", status)), ..self }, vec![]);
                }
                let who = header(&headers, "x-signed-in").map(String::from);
                (Quote { quote: String::from_utf8(body).ok(), who, ..self }, vec![])
            }
            Event::Failed { msg, error } if msg == "quote" => (Quote { problem: Some(error), ..self }, vec![]),
            _ => (self, vec![]),
        }
    }

    fn screen(&self) -> View<Msg> {
        let mut parts = vec![
            text("Quote of the day").title(),
            text(self.quote.as_deref().unwrap_or("Fetching today's quote…")),
            text(&match &self.who {
                Some(w) => format!("Signed in as {}", w),
                None => String::from("Not signed in"),
            })
            .dim(),
        ];
        if let Some(p) = &self.problem {
            parts.push(text(p));
        }
        parts.push(button("Try again").on_press(Msg::Again));
        column(parts).spacing(Space::Md).screen_title("Quote")
    }
}

rapidkoi::export!(Quote);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_signs_in_and_fetches() {
        let (q, asks) = Quote::default().step(Event::Opened);
        assert_eq!(asks.len(), 3);
        assert_eq!(asks[2], Ask::fetch("quote", Request::get("http://localhost:8000/quote")));
        let got = Event::Response {
            msg: "quote".into(),
            status: 200,
            headers: vec![("x-signed-in".into(), "User 4f9a".into())],
            body: b"Make it work.".to_vec(),
        };
        let (q, _) = q.step(got);
        assert_eq!((q.quote.as_deref(), q.who.as_deref()), (Some("Make it work."), Some("User 4f9a")));
    }

    #[test]
    fn a_refusal_shows() {
        let off = Event::Failed { msg: "quote".into(), error: "You turned off localhost:8000 for Quote".into() };
        let (q, _) = Quote::default().step(off);
        assert_eq!(q.problem.as_deref(), Some("You turned off localhost:8000 for Quote"));
    }
}
