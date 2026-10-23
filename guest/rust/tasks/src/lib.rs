//! Tasks (PRP-040): Rapidkoi's build step 3, a real app built only on
//! contract 5 and this library. A list with All · Open · Done (in its
//! link), adding with Enter, a task's own screen (`editor.task` in its
//! link), and Delete asked first. Two parts, the list and the editor, each
//! with its own messages, joined with `.map`. Tasks live in the window's
//! state until storage (step 4).
//!
//! Build: `cargo build --release --target wasm32-unknown-unknown -p tasks`
//! in `guest/rust/`, then pack and serve it (CLAUDE.md COMMANDS).

use rapidkoi::raw::{boxed, Role};
use rapidkoi::style::{Background, Border, Color, Corner, Font, Layout, Space as Gap, Spacing, State as On, Text, TextSize, When};
use rapidkoi::{
    button, checkbox, column, input, row, spacer, tabs, text, text_area, App, Event, Message, Next, Space, State, View,
};

/// The longest title, in characters.
const MAX_TITLE: usize = 200;

/// The longest notes, in characters.
const MAX_NOTES: usize = 10_000;

/// One task.
#[derive(Debug, Clone, Default, PartialEq, State)]
pub struct Task {
    id: u32,
    title: String,
    notes: String,
    done: bool,
}

/// The list: what's shown, what's being typed, and every task.
#[derive(Debug, Clone, Default, PartialEq, State)]
pub struct TaskList {
    /// `all`, `open` or `done`: in the link.
    filter: String,
    draft: String,
    tasks: Vec<Task>,
}

/// The task screen: which task, by ID (in the link).
#[derive(Debug, Clone, Default, PartialEq, State)]
pub struct Editor {
    task: u32,
}

/// The whole app: which page, and each part's own state.
#[derive(Debug, Default, State)]
pub struct Tasks {
    /// `list`, `task` or `delete`: in the link.
    page: String,
    list: TaskList,
    editor: Editor,
    /// The last ID given: IDs only grow, so a link never points at a reused one.
    last_id: u32,
}

/// What the list sends.
#[derive(Debug, Clone, PartialEq, Message)]
pub enum ListMsg {
    Filter(String),
    /// The new task's box changed: its text comes as the event's `value`.
    Draft,
    /// Enter in the new task's box.
    Add,
    Toggle(u32),
    Open(u32),
}

/// What a task's screen sends.
#[derive(Debug, Clone, PartialEq, Message)]
pub enum EditorMsg {
    Back,
    Title,
    Notes,
    Done,
    Delete,
    Confirm,
    Cancel,
}

/// Everything the app's screens send: each part's own, wrapped.
#[derive(Debug, Clone, PartialEq, Message)]
pub enum AppMsg {
    List(ListMsg),
    Editor(EditorMsg),
}

/// Text cut to `most` characters.
fn cut(s: &str, most: usize) -> String {
    s.chars().take(most).collect()
}

impl Tasks {
    /// Shown on the list: `page` and the filter go in the link.
    fn to_list(self) -> Next<Self> {
        let back = Tasks { page: "list".into(), ..self };
        Next::state(back).link(Tasks::page).link(Tasks::list.then(TaskList::filter))
    }

    /// Shown on a task's page (or its delete question): `page` and the task go in the link.
    fn to_task(self, page: &str, id: u32) -> Next<Self> {
        let at = Tasks { page: page.into(), editor: Editor { task: id }, ..self };
        Next::state(at).link(Tasks::page).link(Tasks::editor.then(Editor::task))
    }

    fn task(&self) -> Option<&Task> {
        self.list.tasks.iter().find(|t| t.id == self.editor.task)
    }

    fn with_task(mut self, f: impl FnOnce(&mut Task)) -> Self {
        let id = self.editor.task;
        if let Some(t) = self.list.tasks.iter_mut().find(|t| t.id == id) {
            f(t);
        }
        self
    }

    /// The filter as shown: anything unknown is All.
    fn filter(&self) -> &str {
        match self.list.filter.as_str() {
            "open" => "open",
            "done" => "done",
            _ => "all",
        }
    }

    fn shown(&self) -> impl Iterator<Item = &Task> {
        let f = self.filter();
        self.list.tasks.iter().filter(move |t| match f {
            "open" => !t.done,
            "done" => t.done,
            _ => true,
        })
    }

    fn on_list(mut self, msg: ListMsg, value: Option<String>) -> Next<Self> {
        match msg {
            ListMsg::Filter(f) => {
                self.list.filter = f;
                self.to_list()
            }
            ListMsg::Draft => {
                self.list.draft = cut(&value.unwrap_or_default(), MAX_TITLE);
                Next::state(self)
            }
            ListMsg::Add => {
                let title = cut(value.as_deref().unwrap_or(&self.list.draft).trim(), MAX_TITLE);
                if title.is_empty() {
                    return Next::state(self);
                }
                self.last_id += 1;
                let id = self.last_id;
                self.list.tasks.push(Task { id, title, notes: String::new(), done: false });
                self.list.draft = String::new();
                Next::state(self)
            }
            ListMsg::Toggle(id) => {
                if let Some(t) = self.list.tasks.iter_mut().find(|t| t.id == id) {
                    t.done = !t.done;
                }
                Next::state(self)
            }
            ListMsg::Open(id) => self.to_task("task", id),
        }
    }

    fn on_editor(self, msg: EditorMsg, value: Option<String>) -> Next<Self> {
        match msg {
            EditorMsg::Back | EditorMsg::Confirm if self.task().is_none() => self.to_list(),
            EditorMsg::Back => self.to_list(),
            EditorMsg::Title => {
                let title = cut(&value.unwrap_or_default(), MAX_TITLE);
                Next::state(self.with_task(|t| t.title = title))
            }
            EditorMsg::Notes => {
                let notes = cut(&value.unwrap_or_default(), MAX_NOTES);
                Next::state(self.with_task(|t| t.notes = notes))
            }
            EditorMsg::Done => Next::state(self.with_task(|t| t.done = !t.done)),
            EditorMsg::Delete => {
                let id = self.editor.task;
                self.to_task("delete", id)
            }
            EditorMsg::Cancel => {
                let id = self.editor.task;
                self.to_task("task", id)
            }
            EditorMsg::Confirm => {
                let id = self.editor.task;
                let mut gone = self;
                gone.list.tasks.retain(|t| t.id != id);
                gone.to_list()
            }
        }
    }

    /// A link opened (or a fresh start): what it says is checked here.
    fn opened(mut self) -> Next<Self> {
        if !matches!(self.list.filter.as_str(), "all" | "open" | "done") {
            self.list.filter = "all".into();
        }
        match self.page.as_str() {
            "task" | "delete" if self.task().is_some() => {
                let (page, id) = (self.page.clone(), self.editor.task);
                self.to_task(&page, id)
            }
            // A task that isn't there (a link from another window, or deleted): the list.
            _ => self.to_list(),
        }
    }
}

impl App for Tasks {
    type Msg = AppMsg;

    fn handle(self, event: Event<AppMsg>) -> Next<Self> {
        match event {
            Event::Init => Tasks { list: TaskList { filter: "all".into(), ..self.list.clone() }, ..self }.to_list(),
            Event::Opened => self.opened(),
            Event::Message { msg: AppMsg::List(m), value } => self.on_list(m, value),
            Event::Message { msg: AppMsg::Editor(m), value } => self.on_editor(m, value),
            _ => Next::state(self),
        }
    }

    fn view(&self) -> View<AppMsg> {
        let screen = match (self.page.as_str(), self.task()) {
            ("task", Some(t)) => editor_view(t).map(AppMsg::Editor),
            ("delete", Some(t)) => delete_view(t).map(AppMsg::Editor),
            _ => list_view(self).map(AppMsg::List),
        };
        column([screen]).padding(Space::Lg).screen_title("Tasks")
    }
}

fn list_view(app: &Tasks) -> View<ListMsg> {
    let rows: Vec<View<ListMsg>> = app.shown().map(task_row).collect();
    let open = app.list.tasks.iter().filter(|t| !t.done).count();
    let done = app.list.tasks.len() - open;
    let body = if app.list.tasks.is_empty() {
        text("No tasks yet").dim()
    } else {
        column(rows).spacing(Space::Xs)
    };
    column([
        text("Tasks").title(),
        tabs(&[("all", "All"), ("open", "Open"), ("done", "Done")], app.filter(), |k| ListMsg::Filter(k.into())),
        input(&app.list.draft).placeholder("What needs doing?").on_change(ListMsg::Draft).on_submit(ListMsg::Add),
        body,
        text(&format!("{open} open · {done} done")).dim().style([Font::size(TextSize::Small)]),
    ])
    .spacing(Space::Md)
}

/// A task's row: its tick, and the rest of the row opens it. Two presses, so ticking never opens.
fn task_row(t: &Task) -> View<ListMsg> {
    let title = text(&t.title).style([Text::color(Color::system(if t.done { "text_dim" } else { "text" }))]);
    let opens = boxed()
        .role(Role::Button)
        .label(&t.title)
        .on_press(ListMsg::Open(t.id))
        .style([Layout::row(), Layout::grow(), Spacing::gap(Gap::Sm)])
        .children([title, spacer(), text("›").dim()]);
    row([checkbox("", t.done).label(&t.title).on_toggle(ListMsg::Toggle(t.id)), opens])
        .spacing(Space::Sm)
        .style([
            Spacing::padding_xy(Gap::Sm, Gap::Md),
            Background::color(Color::system("surface")),
            Border::corner(Corner::Medium),
            When(On::Hover, [Background::color(Color::system("surface").mix(Color::system("text"), 0.06))]),
        ])
}

fn editor_view(t: &Task) -> View<EditorMsg> {
    column([
        button("‹ Tasks").subtle().on_press(EditorMsg::Back),
        input(&t.title).placeholder("Title").on_change(EditorMsg::Title),
        text_area(&t.notes).placeholder("Notes…").on_change(EditorMsg::Notes),
        checkbox("Done", t.done).on_toggle(EditorMsg::Done),
        row([spacer(), button("Delete").danger().on_press(EditorMsg::Delete)]),
    ])
    .spacing(Space::Md)
}

fn delete_view(t: &Task) -> View<EditorMsg> {
    column([
        text(&format!("Delete \"{}\"?", t.title)).heading(),
        row([button("Cancel").on_press(EditorMsg::Cancel), button("Delete").danger().on_press(EditorMsg::Confirm)])
            .spacing(Space::Sm),
    ])
    .spacing(Space::Md)
}

rapidkoi::export!(Tasks);

#[cfg(test)]
mod tests {
    use super::*;

    fn start() -> Tasks {
        Tasks::default().handle(Event::Init).into_state()
    }

    fn list(app: Tasks, m: ListMsg, value: Option<&str>) -> Next<Tasks> {
        app.handle(Event::Message { msg: AppMsg::List(m), value: value.map(String::from) })
    }

    fn edit(app: Tasks, m: EditorMsg, value: Option<&str>) -> Next<Tasks> {
        app.handle(Event::Message { msg: AppMsg::Editor(m), value: value.map(String::from) })
    }

    fn with(titles: &[&str]) -> Tasks {
        let mut app = start();
        for t in titles {
            app = list(app, ListMsg::Add, Some(t)).into_state();
        }
        app
    }

    fn titles(app: &Tasks) -> Vec<String> {
        app.shown().map(|t| t.title.clone()).collect()
    }

    #[test]
    fn adding_trims_refuses_empty_and_ids_only_grow() {
        let app = with(&["  Buy milk ", "   ", "Pay rent"]);
        assert_eq!(titles(&app), ["Buy milk", "Pay rent"]);
        assert_eq!(app.list.tasks.iter().map(|t| t.id).collect::<Vec<_>>(), [1, 2]);
        // Delete the last, add again: a new ID, never 2 again.
        let app = edit(list(app, ListMsg::Open(2), None).into_state(), EditorMsg::Confirm, None).into_state();
        let app = list(app, ListMsg::Add, Some("Call")).into_state();
        assert_eq!(app.list.tasks.last().map(|t| t.id), Some(3));
    }

    #[test]
    fn toggling_and_filtering() {
        let app = list(with(&["Buy milk", "Pay rent"]), ListMsg::Toggle(1), None).into_state();
        assert!(app.list.tasks[0].done);
        let open = list(app, ListMsg::Filter("open".into()), None);
        assert_eq!(open.linked(), Some(vec![String::from("page"), String::from("list.filter")]));
        let open = open.into_state();
        assert_eq!(titles(&open), ["Pay rent"]);
        let done = list(open, ListMsg::Filter("done".into()), None).into_state();
        assert_eq!(titles(&done), ["Buy milk"]);
        // Anything else is All.
        let odd = list(done, ListMsg::Filter("weird".into()), None).into_state();
        assert_eq!(titles(&odd), ["Buy milk", "Pay rent"]);
    }

    #[test]
    fn opening_a_task_links_it_and_back_links_the_list() {
        let opened = list(with(&["Buy milk", "Pay rent"]), ListMsg::Open(2), None);
        assert_eq!(opened.linked(), Some(vec![String::from("page"), String::from("editor.task")]));
        let app = opened.into_state();
        assert_eq!((app.page.as_str(), app.editor.task), ("task", 2));
        let back = edit(app, EditorMsg::Back, None);
        assert_eq!(back.linked(), Some(vec![String::from("page"), String::from("list.filter")]));
        assert_eq!(back.into_state().page, "list");
    }

    #[test]
    fn delete_asks_first() {
        let app = list(with(&["Buy milk"]), ListMsg::Open(1), None).into_state();
        let asking = edit(app, EditorMsg::Delete, None).into_state();
        assert_eq!(asking.page, "delete");
        let kept = edit(asking, EditorMsg::Cancel, None).into_state();
        assert_eq!((kept.page.as_str(), kept.list.tasks.len()), ("task", 1));
        let gone = edit(edit(kept, EditorMsg::Delete, None).into_state(), EditorMsg::Confirm, None).into_state();
        assert_eq!((gone.page.as_str(), gone.list.tasks.len()), ("list", 0));
    }

    #[test]
    fn a_link_to_a_missing_task_lands_on_the_list() {
        // As a link sets it on a fresh window: no tasks there yet (storage is step 4).
        let mut app = start();
        app.page = "task".into();
        app.editor.task = 42;
        app.list.filter = "sideways".into();
        let opened = app.handle(Event::Opened).into_state();
        assert_eq!((opened.page.as_str(), opened.list.filter.as_str()), ("list", "all"));
    }

    #[test]
    fn titles_and_notes_are_cut() {
        let long = "x".repeat(300);
        let app = list(with(&[]), ListMsg::Add, Some(&long)).into_state();
        assert_eq!(app.list.tasks[0].title.chars().count(), MAX_TITLE);
        let app = list(app, ListMsg::Open(1), None).into_state();
        let app = edit(app, EditorMsg::Notes, Some(&"n".repeat(MAX_NOTES + 5))).into_state();
        assert_eq!(app.list.tasks[0].notes.chars().count(), MAX_NOTES);
    }
}
