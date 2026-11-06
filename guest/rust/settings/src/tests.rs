use super::*;
use minicbor::Encoder;

fn send(s: Settings, msg: Msg, value: Option<&str>) -> Next<Settings> {
    s.handle(Event::Message { msg, value: value.map(String::from) })
}

fn read() -> Settings {
    Settings { read: true, text_size: 100, ..Settings::default() }
}

fn signed(name: &str, key: &str) -> Signed {
    Signed {
        name: name.into(),
        place: format!("{}.dafifi.net", name.to_lowercase()),
        first: "Ada".into(),
        last: String::new(),
        since: 1_760_000_000_000,
        key: key.into(),
    }
}

fn trusted(host: &str) -> Key {
    Key {
        host: host.into(),
        key: "c1d944e07a2f13b6".into(),
        since: 1_761_000_000_000,
        signs: vec!["Counter".into()],
        rotations: 1,
    }
}

/// The texts on screen.
fn texts(s: &Settings) -> String {
    format!("{:?}", s.view())
}

#[test]
fn it_reads_everything_on_open_and_on_every_change() {
    let all = [Ask::preferences("prefs"), Ask::you("you"), Ask::publishers("keys")];
    for e in [Event::Opened, Event::Resume, Event::Change { tables: vec![] }] {
        assert_eq!(Settings::default().handle(e).asks(), &all);
    }
}

#[test]
fn each_category_opens_and_picking_one_clears_find() {
    let s = send(read(), Msg::Find, Some("key")).into_state();
    let s = send(s, Msg::Go(Page::Startup.index()), None).into_state();
    assert_eq!((s.shown(), s.find.as_str()), (Some(Page::Startup), ""));
    for p in find::PAGES {
        assert_eq!(send(read(), Msg::Go(p.index()), None).into_state().shown(), Some(p));
    }
}

#[test]
fn find_narrows_by_name_or_words_and_opens_the_first_match() {
    assert_eq!(found("KEY"), vec![Page::You, Page::Publishers]);
    assert_eq!(found("contrast"), vec![Page::Appearance]);
    assert_eq!(found(""), find::PAGES.to_vec());
    let s = send(read(), Msg::Find, Some("restore")).into_state();
    assert_eq!(s.shown(), Some(Page::Startup));
    let none = send(read(), Msg::Find, Some("zzz")).into_state();
    assert_eq!(none.shown(), None);
    assert!(texts(&none).contains("No results"));
}

#[test]
fn each_control_asks_for_its_change() {
    let set = |msg: Msg, value: Option<&str>| send(read(), msg, value).asks().to_vec();
    assert_eq!(set(Msg::Theme(true), None), vec![Ask::set_preference("set", Preference::Light(true))]);
    assert_eq!(set(Msg::Contrast, None), vec![Ask::set_preference("set", Preference::MoreContrast(true))]);
    assert_eq!(set(Msg::Size, Some("4")), vec![Ask::set_preference("set", Preference::TextSize(125))]);
    assert_eq!(set(Msg::Size, Some("7")), vec![Ask::set_preference("set", Preference::TextSize(200))]);
    assert_eq!(set(Msg::Start(true), None), vec![Ask::set_preference("set", Preference::Start(Start::Restore))]);
    // Past the last stop, or not a number: nothing.
    assert_eq!(set(Msg::Size, Some("8")), vec![]);
    assert_eq!(set(Msg::Size, Some("x")), vec![]);
}

#[test]
fn a_size_between_stops_shows_at_the_stop_below_and_keeps_its_number() {
    let s = Settings { text_size: 120, ..read() };
    assert_eq!(STOPS[s.stop()], 110);
    assert!(texts(&s).contains("120%"));
    assert_eq!(STOPS[Settings { text_size: 200, ..read() }.stop()], 200);
    assert_eq!(STOPS[Settings { text_size: 80, ..read() }.stop()], 80);
}

#[test]
fn the_answers_are_kept() {
    let mut e = Encoder::new(Vec::new());
    e.map(4).unwrap();
    e.str("theme").unwrap().str("light").unwrap();
    e.str("more_contrast").unwrap().bool(true).unwrap();
    e.str("text_size").unwrap().u16(150).unwrap();
    e.str("start").unwrap().str("restore").unwrap();
    let s = Settings::default().handle(Event::Data { msg: "prefs".into(), cbor: e.into_writer() }).into_state();
    assert_eq!((s.read, s.light, s.more_contrast, s.text_size, s.restore), (true, true, true, 150, true));

    let mut y = Encoder::new(Vec::new());
    y.map(3).unwrap();
    y.str("first").unwrap().str("Ada").unwrap();
    y.str("last").unwrap().str("L").unwrap();
    y.str("apps").unwrap().array(1).unwrap().map(2).unwrap();
    y.str("name").unwrap().str("Notes").unwrap();
    y.str("key").unwrap().str("4b2e").unwrap();
    let s = s.handle(Event::Data { msg: "you".into(), cbor: y.into_writer() }).into_state();
    assert_eq!((s.first.as_str(), s.apps.len(), s.apps[0].key.as_str()), ("Ada", 1, "4b2e"));
}

#[test]
fn you_lists_the_apps_and_shows_the_picked_one_s_details() {
    let s =
        Settings { page: Page::You.index(), apps: vec![signed("Notes", "4b2e9d71"), signed("Counter", "")], ..read() };
    let shown = texts(&s);
    assert!(shown.contains("Signed in to") && shown.contains("Notes") && shown.contains("Counter"));
    assert!(shown.contains("4B2E 9D71"));
    let other = send(s, Msg::PickApp(1), None).into_state();
    assert!(texts(&other).contains("Not available"));
}

#[test]
fn no_apps_no_list_and_no_details() {
    let s = Settings { page: Page::You.index(), ..read() };
    let shown = texts(&s);
    assert!(!shown.contains("Signed in to"));
    assert!(!shown.contains("Your key for it"));
}

#[test]
fn publishers_lists_the_keys_and_shows_the_picked_one_s_details() {
    let s = Settings { page: Page::Publishers.index(), keys: vec![trusted("counter.dafifi.net")], ..read() };
    let shown = texts(&s);
    assert!(shown.contains("counter.dafifi.net") && shown.contains("C1D9 44E0 7A2F 13B6"));
    assert!(shown.contains("Fingerprint") && shown.contains("Rotations"));
}

#[test]
fn the_name_page_sends_both_parts_as_typed() {
    let s = Settings { page: Page::You.index(), first: "Ada".into(), last: "L".into(), ..read() };
    let s = send(s, Msg::EditName(true), None).into_state();
    assert!(texts(&s).contains("First name"));
    let next = send(s, Msg::First, Some("Grace"));
    assert_eq!(next.asks(), &[Ask::set_name("name", "Grace", "L")]);
    let s = next.into_state();
    assert_eq!(send(s, Msg::Last, Some("Hopper")).asks(), &[Ask::set_name("name", "Grace", "Hopper")]);
}
