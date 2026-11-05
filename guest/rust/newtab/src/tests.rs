//! The new tab's tests (split from lib.rs at the owner's word, 2026-11-01:
//! files stay under 500 lines).

use super::*;

#[test]
fn the_screen_names_only_packed_pictures() {
    // `picture` checks each name against ASSETS while the view is built.
    let _ = NewTab::default().view();
}

#[test]
fn the_screen_places_the_widgets_slot() {
    // PRP-073: Rapidkoi draws the person's widgets there; this app never sees them.
    let bytes = NewTab::default().view().encode();
    let at = |needle: &[u8]| bytes.windows(needle.len()).position(|w| w == needle);
    assert!(at(b"slot").is_some() && at(b"widgets").is_some());
}

#[test]
fn every_picture_is_in_assets_and_packed() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/");
    let script = include_str!("../../../../scripts/newtab.sh");
    for name in ASSETS {
        assert!(std::path::Path::new(&format!("{dir}{name}")).is_file(), "assets/{name} is missing");
        assert!(script.contains(&format!("--asset {name}=")), "scripts/newtab.sh doesn't pack {name}");
    }
}

use minicbor::Encoder;

/// A session answer: `count` windows of these apps, each `{name, place, icon}`.
fn session(count: u32, places: &[&str]) -> Vec<u8> {
    let mut e = Encoder::new(Vec::new());
    e.map(2).unwrap().str("count").unwrap().u32(count).unwrap();
    e.str("apps").unwrap().array(places.len() as u64).unwrap();
    for p in places {
        e.map(3).unwrap();
        e.str("name").unwrap().str(p).unwrap();
        e.str("place").unwrap().str(p).unwrap();
        e.str("icon").unwrap().str(&format!("icon:{p}")).unwrap();
    }
    e.into_writer()
}

#[test]
fn it_asks_for_the_session_places_and_notifications_each_time_it_s_shown_or_told_they_changed() {
    // No pins (PRP-067): the dock has them, and All apps every app.
    for e in [Event::Opened, Event::Resume, Event::Change { tables: vec![] }] {
        assert_eq!(
            NewTab::default().step(e).1,
            vec![Ask::session("session"), Ask::recent("recent"), Ask::notifications("notifications")]
        );
    }
}

/// A recent answer: each `(name, address, title)`, seen at 1, 2, 3… newest first.
fn places(list: &[(&str, &str, Option<&str>)]) -> Vec<u8> {
    let mut e = Encoder::new(Vec::new());
    e.map(1).unwrap().str("places").unwrap().array(list.len() as u64).unwrap();
    for (i, (name, address, title)) in list.iter().enumerate() {
        e.map(if title.is_some() { 7 } else { 6 }).unwrap();
        e.str("name").unwrap().str(name).unwrap();
        e.str("place").unwrap().str(address).unwrap();
        e.str("icon").unwrap().str(&format!("icon:{address}")).unwrap();
        e.str("address").unwrap().str(address).unwrap();
        if let Some(t) = title {
            e.str("title").unwrap().str(t).unwrap();
        }
        e.str("seen").unwrap().i64(i as i64 + 1).unwrap();
        e.str("colors").unwrap().map(1).unwrap().str("primary").unwrap().str("#14b8a6").unwrap();
    }
    e.into_writer()
}

#[test]
fn your_places_become_recent_and_a_card_opens_its_place() {
    let cbor = places(&[("Notes", "notes.dafifi.net", Some("Shopping")), ("Counter", "counter.rkb", None)]);
    let (t, asks) = NewTab::default().step(Event::Data { msg: "recent".into(), cbor });
    assert!(asks.is_empty());
    assert_eq!(t.recent.len(), 2);
    assert_eq!(t.recent[0].title.as_deref(), Some("Shopping"));
    assert_eq!(t.recent[1].icon, "icon:counter.rkb");
    // Its app's primary colour reaches its card (PRP-063).
    assert_eq!(t.recent[0].colour, "#14b8a6");
    assert_eq!(rgb_of("#14b8a6"), (0x14, 0xb8, 0xa6));
    let _ = t.view();
    assert!(recent(&t.recent).is_some());
    let pressed = Event::Message { msg: Msg::OpenRecent(1), value: None };
    assert_eq!(t.step(pressed).1, vec![Ask::open_recent("open", 1)]);
}

#[test]
fn with_no_places_there_s_no_recent() {
    assert!(recent(&[]).is_none());
}

#[test]
fn restore_shows_the_session_s_apps_while_there_is_one() {
    let data = Event::Data { msg: "session".into(), cbor: session(5, &["notes.dafifi.net", "trip.app"]) };
    let (t, asks) = NewTab::default().step(data);
    assert_eq!(t.restore, Some(vec!["icon:notes.dafifi.net".to_string(), "icon:trip.app".to_string()]));
    assert!(asks.is_empty());
    let _ = t.view();
    // Nothing to restore: no pill.
    let (none, _) = t.step(Event::Answer { msg: "session".into(), value: None });
    assert_eq!(none.restore, None);
}

#[test]
fn at_most_three_icons_then_how_many_more() {
    let icons: Vec<String> = (0..5).map(|i| format!("icon:a{i}")).collect();
    assert_eq!(shown_and_more(&icons), (&icons[..3], 2));
    assert_eq!(shown_and_more(&icons[..2]), (&icons[..2], 0));
}

#[test]
fn pressing_restore_asks_rapidkoi_to_restore() {
    let pressed = Event::Message { msg: Msg::Restore, value: None };
    assert_eq!(NewTab::default().step(pressed).1, vec![Ask::restore_session("restore")]);
}

#[test]
fn it_has_its_icon() {
    // Every app has one (PRP-060).
    let script = include_str!("../../../../scripts/newtab.sh");
    assert!(script.contains("--icon guest/rust/newtab/icon.svg"));
    assert!(std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/icon.svg")).is_file());
}

#[test]
fn a_card_leaves_out_an_empty_address_and_an_unknown_time() {
    // Rapidkoi's own apps have no address; a place from before has no time (0).
    let settings = Spot {
        name: "Settings".into(),
        icon: "icon:builtin:settings".into(),
        address: "".into(),
        title: None,
        seen: 0,
        colour: "#71717a".into(),
    };
    let _ = recent(&[settings]);
    let _ = note_at(&["Settings", ""], 0);
}

/// A notifications answer: `unread`, then each `(name, text)` from an app, sent at 5.
fn alerts(unread: u32, list: &[(&str, &str)]) -> Vec<u8> {
    let mut e = Encoder::new(Vec::new());
    e.map(2).unwrap().str("unread").unwrap().u32(unread).unwrap();
    e.str("items").unwrap().array(list.len() as u64).unwrap();
    for (name, said) in list {
        e.map(8).unwrap();
        for (k, v) in [("kind", "app"), ("name", name), ("place", "counter.rkb"), ("icon", "icon:counter.rkb")] {
            e.str(k).unwrap().str(v).unwrap();
        }
        e.str("text").unwrap().str(said).unwrap().str("address").unwrap().str("counter.rkb").unwrap();
        e.str("sent").unwrap().i64(5).unwrap().str("read").unwrap().bool(false).unwrap();
    }
    e.into_writer()
}

#[test]
fn notifications_show_only_while_one_is_unread_and_open_or_dismiss_by_index() {
    // PRP-065: the newest four, each opening its place, × removing it.
    let five = alerts(5, &[("Counter", "a"), ("Counter", "b"), ("Notes", "c"), ("Notes", "d"), ("Notes", "e")]);
    let (t, asks) = NewTab::default().step(Event::Data { msg: "notifications".into(), cbor: five });
    assert!(asks.is_empty());
    assert_eq!((t.unread, t.notices.len()), (5, 5));
    assert_eq!(
        t.notices[0],
        Alert { name: "Counter".into(), icon: "icon:counter.rkb".into(), text: "a".into(), sent: 5 }
    );
    assert!(notices::section(t.unread, &t.notices).is_some());
    let _ = t.view();
    let open = Event::Message { msg: Msg::OpenNotice(2), value: None };
    assert_eq!(t.step(open).1, vec![Ask::open_notification("open", 2)]);
    let (t, _) =
        NewTab::default().step(Event::Data { msg: "notifications".into(), cbor: alerts(1, &[("Counter", "a")]) });
    let dismiss = Event::Message { msg: Msg::DismissNotice(0), value: None };
    assert_eq!(t.step(dismiss).1, vec![Ask::dismiss_notification("dismiss", 0)]);
    // Everything read: no section.
    let (read, _) =
        NewTab::default().step(Event::Data { msg: "notifications".into(), cbor: alerts(0, &[("Counter", "a")]) });
    assert!(notices::section(read.unread, &read.notices).is_none());
}

#[test]
fn notifications_sit_four_on_one_line() {
    // The owner's word (2026-11-05): one line of four, as the widgets, not two by two.
    let items = || [text::<Msg>("a"), text("b")];
    let expected = wrap(items()).style([Layout::equal(), Layout::per_line(4), Spacing::gap(Space::Px(16))]);
    assert_eq!(notices::in_a_line(items()), expected);
}
