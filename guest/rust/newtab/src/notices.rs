//! Notifications on the new tab (PRP-065, board A5): only while something
//! is unread, the heading with the unread count in the accent, then the
//! newest four, unread first, two by two. Each is its app's icon, its
//! sentence, and its app and when, in dim; the row opens its place and ×
//! removes it. Rapidkoi gives the app's name and icon: the app that sent it
//! gave only the sentence and the place.

use minicbor::{Decode, Encode};
use rapidkoi::style::{Align, Border, Color, Corner, Font, Layout, Sizing, Spacing, Text, TextSize, Weight};
use rapidkoi::{column, image, row, spacer, text, Notification, Space};

use super::{card, halves, heading, note_at, picture, Msg, V};

/// How many the section shows.
const SHOWN: usize = 4;

/// One of your notifications as the new tab shows it.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub(crate) struct Alert {
    #[n(0)]
    pub(crate) name: String,
    #[n(1)]
    pub(crate) icon: String,
    #[n(2)]
    pub(crate) text: String,
    #[n(3)]
    pub(crate) sent: i64,
}

impl From<Notification> for Alert {
    fn from(n: Notification) -> Self {
        Alert { name: n.name, icon: n.icon, text: n.text, sent: n.sent }
    }
}

/// Notifications, or nothing while none is unread.
pub(crate) fn section(unread: u32, alerts: &[Alert]) -> Option<V> {
    if unread == 0 {
        return None;
    }
    let count = column([text(&unread.to_string()).style([
        Text::color(Color::system("on_accent")),
        Font::size(TextSize::Small),
        Font::weight(Weight::Bold),
    ])])
    .style([
        rapidkoi::style::Background::color(Color::system("accent")),
        Border::corner(Corner::Round),
        Spacing::padding_xy(Space::None, Space::Px(7)),
    ]);
    let head =
        row([heading("NOTIFICATIONS").style([Layout::align(Align::Center)]), count]).style([Spacing::gap(Space::Sm)]);
    let items = alerts.iter().take(SHOWN).enumerate().map(|(i, a)| one(i as u32, a));
    Some(column([head, halves(items)]).style([Spacing::gap(Space::Px(12))]))
}

/// One notification: the row opens it, × beside it removes it.
fn one(i: u32, a: &Alert) -> V {
    let body = row([
        image(&a.icon).style([Sizing::width(40), Sizing::height(40), Layout::align(Align::Center)]),
        column([text(&a.text).style([Text::truncate()]), note_at(&[&a.name], a.sent)]).style([
            Spacing::gap(Space::Px(3)),
            Layout::grow(),
            Layout::align(Align::Center),
        ]),
    ])
    .style([Spacing::gap(Space::Px(14)), Layout::grow()])
    .on_press(Msg::OpenNotice(i));
    // A box around the ×: Rapidkoi presses boxes only. The picture is the
    // shell's x.svg in its faint grey, which reads on light and dark.
    let dismiss = column([picture("x.svg", 16, 16)])
        .style([Layout::align(Align::Center)])
        .label("Dismiss")
        .on_press(Msg::DismissNotice(i));
    row([body, spacer(), dismiss]).style(
        [vec![Spacing::gap(Space::Px(14)), Spacing::padding_xy(Space::Px(12), Space::Px(14))], card(14, false)]
            .concat(),
    )
}
