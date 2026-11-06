//! You's and Publishers' pages (PRP-075): the apps you're signed in to and
//! the keys you trust, each a list beside a details column, and the page
//! for your name.

use crate::pages::{dim, group, icon, padded, section, sys};
use crate::{Msg, Settings};
use rapidkoi::raw::{boxed, Role};
use rapidkoi::style::{
    Align, Background, Border, Corner, Family, Font, Layout, Sizing, Spacing, Text, TextSize, Weight,
};
use rapidkoi::{column, input, row, spacer, text, text_of, time, Space, View};

type V = View<Msg>;

/// The details column's width.
const DETAILS: u16 = 300;

/// An initial in a circle: the person, an app or a key's host.
fn face(name: &str, size: u16) -> V {
    let initial: String = name.chars().next().map(|c| c.to_uppercase().collect()).unwrap_or_default();
    column([text(&initial).style([Font::weight(Weight::Bold), Text::color(sys("on_accent"))])]).style([
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

fn chevron() -> V {
    icon("chevron.svg", 20, "text_dim")
}

pub(crate) fn you(s: &Settings) -> (Vec<V>, Option<V>) {
    let name = full(&s.first, &s.last);
    let me = padded(
        row([
            face(if name.is_empty() { "?" } else { &name }, 48),
            column([text(if name.is_empty() { "Not set" } else { &name }), dim("On this computer")]),
        ])
        .spacing(Space::Md)
        .style([Layout::align(Align::Center)]),
    );
    let naming = padded(
        boxed()
            .role(Role::Button)
            .label("Name apps see")
            .on_press(Msg::EditName(true))
            .style([Layout::row(), Spacing::gap(Space::Sm), Layout::align(Align::Center)])
            .children([
                text("Name apps see"),
                spacer(),
                dim(if name.is_empty() { "Not set" } else { &name }),
                chevron(),
            ]),
    );
    let mut sections = vec![section("You", group(vec![me, naming]))];
    if s.apps.is_empty() {
        return (sections, None);
    }
    let rows = s.apps.iter().enumerate().map(|(i, a)| {
        picked(
            i as u32 == s.app,
            Msg::PickApp(i as u32),
            &a.name,
            vec![
                face(&a.name, 28),
                column([text(&a.name), dim(&a.place).style([Font::size(TextSize::Small)])]),
                spacer(),
                since(a.since),
            ],
        )
    });
    sections.push(section("Signed in to", group(rows.collect()).role(Role::List)));
    let a = &s.apps[(s.app as usize).min(s.apps.len() - 1)];
    let key = if a.key.is_empty() {
        fact("Your key for it", text("Not available"))
    } else {
        fact("Your key for it", mono(&grouped(&a.key)))
    };
    let seen = full(&a.first, &a.last);
    let details = details(
        &a.name,
        &a.place,
        vec![
            fact("Name it sees", text(if seen.is_empty() { "Not set" } else { &seen })),
            key,
            fact("First opened", when(a.since)),
        ],
    );
    (sections, Some(details))
}

pub(crate) fn name_page(s: &Settings) -> Vec<V> {
    let back = boxed()
        .role(Role::Button)
        .label("Back")
        .on_press(Msg::EditName(false))
        .style([
            Layout::row(),
            Spacing::gap(Space::Sm),
            Layout::align(Align::Start),
            Spacing::padding_xy(Space::Xs, Space::Sm),
            Border::corner(Corner::Round),
        ])
        .children([text("← You")]);
    let fields = column([
        input(&s.first).placeholder("First name").label("First name").on_change(Msg::First),
        input(&s.last).placeholder("Last name").label("Last name").on_change(Msg::Last),
    ])
    .spacing(Space::Sm);
    vec![back, section("Name apps see", fields)]
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
                icon("shield.svg", 20, "text_dim"),
                column([text(&k.host), mono(&grouped(&k.key.chars().take(16).collect::<String>())).dim()]),
                spacer(),
                since(k.since),
            ],
        )
    });
    let k = &s.keys[(s.key as usize).min(s.keys.len() - 1)];
    let details = details(
        &k.host,
        "Ed25519",
        vec![
            fact("Fingerprint", mono(&grouped(&k.key))),
            fact("Signs", text(&if k.signs.is_empty() { String::from("—") } else { k.signs.join(", ") })),
            fact("Known since", when(k.since)),
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
        .style([Layout::row(), Spacing::gap(Space::Md), Spacing::padding(Space::Md), Layout::align(Align::Center)])
        .children(parts);
    if on {
        r.style([Background::color(sys("surface").mix(sys("text"), 0.08))])
    } else {
        r
    }
}

/// The details column: a face and two lines over its facts.
fn details(name: &str, under: &str, facts: Vec<V>) -> V {
    let head = padded(
        row([face(name, 40), column([text(name).style([Font::weight(Weight::Medium)]), dim(under)])])
            .spacing(Space::Md)
            .style([Layout::align(Align::Center)]),
    );
    let mut rows = vec![head];
    rows.extend(facts);
    group(rows).style([Sizing::width(DETAILS), Layout::align(Align::Start)])
}

/// A fact: its name, dim, above its value.
fn fact(name: &str, value: V) -> V {
    padded(column([dim(name), value]).spacing(Space::Xs))
}

fn mono(s: &str) -> V {
    text(s).style([Font::family(Family::Mono)])
}

/// Hex in groups of four, upper case, as fingerprints are read aloud.
fn grouped(hex: &str) -> String {
    let up = hex.to_uppercase();
    let chars: Vec<char> = up.chars().collect();
    chars.chunks(4).map(|c| c.iter().collect::<String>()).collect::<Vec<_>>().join(" ")
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
