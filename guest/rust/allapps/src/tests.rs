use super::*;

fn app(name: &str, from: &str, opened: i64) -> Item {
    Item {
        at: 0,
        name: name.into(),
        icon: format!("icon:{from}"),
        from: from.into(),
        address: from.into(),
        publisher: String::new(),
        signed: false,
        pinned: false,
        opened,
    }
}

/// All apps with `items`, A–Z as Rapidkoi gives them, each at its index.
fn with(items: Vec<Item>) -> AllApps {
    let apps = items.into_iter().enumerate().map(|(i, a)| Item { at: i as u32, ..a }).collect();
    AllApps { read: true, apps, ..AllApps::default() }
}

fn three() -> AllApps {
    with(vec![app("Counter", "counter.rkb", 5), app("Notes", "notes.dafifi.net", 9), app("Notes", "notes.rkb", 1)])
}

fn send(s: AllApps, msg: Msg, value: Option<&str>) -> Next<AllApps> {
    s.handle(Event::Message { msg, value: value.map(String::from) })
}

fn names(s: &AllApps) -> Vec<String> {
    s.shown().iter().map(|a| a.name.clone()).collect()
}

#[test]
fn it_reads_your_apps_on_open_and_on_every_change() {
    for e in [Event::Opened, Event::Resume, Event::Change { tables: vec![] }] {
        assert_eq!(AllApps::default().handle(e).asks(), &[Ask::apps("apps")]);
    }
}

#[test]
fn a_to_z_by_default_and_last_used_newest_first() {
    assert_eq!(names(&three()), ["Counter", "Notes", "Notes"]);
    let last = send(three(), Msg::Order(true), None).into_state();
    let order: Vec<i64> = last.shown().iter().map(|a| a.opened).collect();
    assert_eq!(order, [9, 5, 1]);
}

#[test]
fn typing_filters_by_name_ignoring_case() {
    let s = send(three(), Msg::Typed, Some("no")).into_state();
    assert_eq!(names(&s), ["Notes", "Notes"]);
    let none = send(three(), Msg::Typed, Some("zz")).into_state();
    assert!(none.shown().is_empty());
}

#[test]
fn enter_opens_the_first_match_by_its_index() {
    let s = send(three(), Msg::Typed, Some("no")).into_state();
    assert_eq!(send(s, Msg::Submit, None).asks(), &[Ask::open_app("open", 1)]);
    // Nothing matches: nothing opens.
    let none = send(three(), Msg::Typed, Some("zz")).into_state();
    assert_eq!(send(none, Msg::Submit, None).asks(), &[]);
}

#[test]
fn where_it_is_from_shows_only_when_names_clash() {
    let s = three();
    let shown: Vec<bool> = s.shown().iter().map(|a| s.clashes(a)).collect();
    assert_eq!(shown, [false, true, true]);
}

#[test]
fn each_action_asks_for_its_app_by_index() {
    assert_eq!(send(three(), Msg::Open(2), None).asks(), &[Ask::open_app("open", 2)]);
    assert_eq!(send(three(), Msg::Pin(1), None).asks(), &[Ask::pin_app("pin", 1)]);
    assert_eq!(send(three(), Msg::Unpin(1), None).asks(), &[Ask::unpin_app("unpin", 1)]);
    assert_eq!(send(three(), Msg::Info(0), None).asks(), &[Ask::app_info("info", 0)]);
    assert_eq!(send(three(), Msg::Forget(0), None).asks(), &[Ask::forget_app("forget", 0)]);
}

#[test]
fn each_tile_has_a_menu() {
    // PRP-071, PRP-072: Open, the pin item for its state, App info and Forget.
    let a = three().apps[1].clone();
    let expected = vec![
        item("Open", Msg::Open(1)),
        item("Pin to dock", Msg::Pin(1)),
        item("App info", Msg::Info(1)),
        separator(),
        item("Forget this app…", Msg::Forget(1)),
    ];
    assert_eq!(rapidkoi::raw::boxed().menu(menu_of(&a)), rapidkoi::raw::boxed().menu(expected));
    let pinned = Item { pinned: true, ..a };
    let unpin = rapidkoi::raw::boxed().menu(menu_of(&pinned)).encode();
    assert!(unpin.windows(15).any(|w| w == b"Unpin from dock"));
    let bytes = three().view().encode();
    assert!(bytes.windows(4).any(|w| w == b"menu"));
    assert!(!bytes.windows(10).any(|w| w == b"on_context"));
}
