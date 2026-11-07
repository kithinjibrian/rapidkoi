//! You's and Publishers' pages (PRP-075): the apps you're signed in to and
//! the keys you trust, each a list beside a details column, and the page
//! for your name.

use crate::pages::{dim, group, mid, padded, section, sys};
use crate::{Msg, Settings};
use rapidkoi::raw::{boxed, Role};
use rapidkoi::style::{
    Align, Background, Border, Corner, Family, Font, Layout, Sizing, Spacing, Text, TextSize, Weight,
};
use rapidkoi::{column, image, input, row, spacer, text, text_of, time, Space, View};

type V = View<Msg>;

/// The details column's least width: it takes the rest of the row beside the list.
const DETAILS: u16 = 300;

/// The width of First name and Last name, so their fields line up.
const NAME_LABEL: u16 = 120;

/// An initial in a circle: the person, an app or a key's host.
/// An app's own icon, `icon:PLACE`, drawn by Rapidkoi (PRP-060); a site's place is its host.
fn app_icon(src: &str, size: u16) -> V {
    mid(image(src).style([Sizing::width(size), Sizing::height(size)]))
}

fn face(name: &str, size: u16) -> V {
    let initial: String = name.chars().next().map(|c| c.to_uppercase().collect()).unwrap_or_default();
    // The initial in the middle both ways: room above and below it.
    column([spacer(), mid(text(&initial).style([Font::weight(Weight::Bold), Text::color(sys("on_accent"))])), spacer()])
        .style([
            Sizing::width(size),
            Sizing::height(size),
            Border::corner(Corner::Round),
            Background::color(sys("accent")),
            Layout::align(Align::Center),
        ])
}

fn full(first: &str, last: &str) -> String {
    format!("{first} {last}").trim().to_string()
}

pub(crate) fn you(s: &Settings) -> (Vec<V>, Option<V>) {
    let name = full(&s.first, &s.last);
    let me = padded(
        row([
            face(if name.is_empty() { "?" } else { &name }, 48),
            // The rest of the row: alone in it, a text wraps to its shortest.
            mid(text(if name.is_empty() { "Not set" } else { &name }).style([Layout::grow()])),
        ])
        .spacing(Space::Md),
    );
    // Your name, typed where it's shown: each edit is saved at once.
    let field = |label: &str, value: &str, msg: Msg| {
        padded(
            row([
                mid(text(label)).style([Sizing::width(NAME_LABEL)]),
                input(value).placeholder(label).label(label).on_change(msg).style([
                    Layout::grow(),
                    Spacing::padding_xy(Space::Sm, Space::Md),
                    Border::corner(Corner::Medium),
                ]),
            ])
            .spacing(Space::Md),
        )
    };
    let mut sections = vec![
        section(
            "You",
            group(vec![me, field("First name", &s.first, Msg::First), field("Last name", &s.last, Msg::Last)]),
        ),
        section("Backup", backup(s)),
    ];
    if s.apps.is_empty() {
        return (sections, None);
    }
    let rows = s.apps.iter().enumerate().map(|(i, a)| {
        picked(
            i as u32 == s.app,
            Msg::PickApp(i as u32),
            &a.name,
            vec![
                app_icon(&a.icon, 28),
                column([text(&a.name), dim(&a.place).style([Font::size(TextSize::Small)])]),
                spacer(),
                since(a.since),
            ],
        )
    });
    sections.push(section("Signed in to", group(rows.collect()).role(Role::List)));
    let a = &s.apps[(s.app as usize).min(s.apps.len() - 1)];
    let key = if a.key.is_empty() { fact("Key", text("Not available")) } else { fact("Key", mono(&a.key)) };
    let seen = full(&a.first, &a.last);
    let details = details(
        app_icon(&a.icon, 40),
        &a.name,
        &a.place,
        vec![
            fact("Name", text(if seen.is_empty() { "Not set" } else { &seen })),
            key,
            fact("First opened", when(a.since)),
        ],
    );
    (sections, Some(details))
}

/// Backup words (PRP-076): written down or not, and restoring. Rapidkoi's
/// own card opens from each; once written down, the first opens nothing.
fn backup(s: &Settings) -> V {
    let words =
        boxed().role(Role::Button).label("Backup words").style([Layout::row(), Spacing::gap(Space::Sm)]).children([
            mid(text("Backup words")),
            spacer(),
            mid(text(if s.backed_up { "Backed up" } else { "Not backed up" })
                .style([Text::color(sys(if s.backed_up { "text_dim" } else { "caution" }))])),
        ]);
    let words = if s.backed_up { words } else { words.on_press(Msg::BackupWords) };
    let restore = boxed()
        .role(Role::Button)
        .label("Use backup words")
        .on_press(Msg::UseBackupWords)
        .style([Layout::row()])
        .children([mid(text("Use backup words"))]);
    group(vec![padded(words), padded(restore)])
}

pub(crate) fn publishers(s: &Settings) -> (Vec<V>, Option<V>) {
    if s.keys.is_empty() {
        return (vec![section("Keys you trust", dim("None"))], None);
    }
    let rows = s.keys.iter().enumerate().map(|(i, k)| {
        picked(
            i as u32 == s.key,
            Msg::PickKey(i as u32),
            &k.host,
            vec![
                app_icon(&format!("icon:{}", k.host), 28),
                column([text(&k.host), mono(&k.key).dim()]),
                spacer(),
                since(k.since),
            ],
        )
    });
    let k = &s.keys[(s.key as usize).min(s.keys.len() - 1)];
    let details = details(
        app_icon(&format!("icon:{}", k.host), 40),
        &k.host,
        "Ed25519",
        vec![
            fact("Key", mono(&k.key)),
            fact("Apps", text(&if k.signs.is_empty() { String::from("—") } else { k.signs.join(", ") })),
            fact("Since", when(k.since)),
            fact("Rotations", text(&k.rotations.to_string())),
        ],
    );
    (vec![section("Keys you trust", group(rows.collect()).role(Role::List))], Some(details))
}

/// A row of a list that picks what the details show; the picked one shaded.
fn picked(on: bool, msg: Msg, label: &str, parts: Vec<V>) -> V {
    let r = boxed()
        .role(Role::ListItem)
        .selected(on)
        .label(label)
        .on_press(msg)
        .style([Layout::row(), Spacing::gap(Space::Md), Spacing::padding(Space::Md)])
        .children(parts.into_iter().map(mid));
    if on {
        r.style([Background::color(sys("surface").mix(sys("text"), 0.08))])
    } else {
        r
    }
}

/// The details column: its app's icon and two lines over its facts.
fn details(picture: V, name: &str, under: &str, facts: Vec<V>) -> V {
    let head = padded(
        row([picture, mid(column([text(name).style([Font::weight(Weight::Medium)]), dim(under)]))]).spacing(Space::Md),
    );
    let mut rows = vec![head];
    rows.extend(facts);
    // The room the capped list leaves, so no gap opens between them.
    group(rows).style([Sizing::min_width(DETAILS), Layout::grow(), Layout::align(Align::Start)])
}

/// A fact: its name, dim, above its value.
fn fact(name: &str, value: V) -> V {
    padded(column([dim(name), value]).spacing(Space::Xs))
}

fn mono(s: &str) -> V {
    text(s).style([Font::family(Family::Mono)])
}

fn since(ms: i64) -> V {
    if ms <= 0 {
        dim("")
    } else {
        text_of(["Since ".into(), time(ms).date()]).dim()
    }
}

fn when(ms: i64) -> V {
    if ms <= 0 {
        text("Not known")
    } else {
        text_of([time(ms).date()])
    }
}
