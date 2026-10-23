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
    button, checkbox, column, input, params, row, spacer, tabs, text, text_area, App, Ask, Event, Message, Next, Space,
    State, Value, View,
};

/// The longest title, in characters.
const MAX_TITLE: usize = 200;

/// The longest notes, in characters.
const MAX_NOTES: usize = 10_000;

/// The database's shape. `AUTOINCREMENT` keeps IDs from ever being reused,
/// so a link never points at another task than the one it was made for.
const SETUP: [&str; 2] = [
    "CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, notes TEXT NOT NULL DEFAULT '', done INTEGER NOT NULL DEFAULT 0)",
    "PRAGMA user_version = 1",
];

/// Every task, in the order they were added.
const LOAD: &str = "SELECT id, title, notes, done FROM tasks ORDER BY id";

/// One task: the window's copy of a row.
#[derive(Debug, Clone, Default, PartialEq, State)]
pub struct Task {
    id: u32,
    title: String,
    notes: String,
    done: bool,
}

/// The list: what's shown, what's being typed, and the copy of every task.
#[derive(Debug, Clone, Default, PartialEq, State)]
pub struct TaskList {
    /// `all`, `open` or `done`: in the link.
    filter: String,
    draft: String,
    /// What this window last read from its database.
    tasks: Vec<Task>,
}

/// The task screen: which task, by ID (in the link).
#[derive(Debug, Clone, Default, PartialEq, State)]
pub struct Editor {
    task: u32,
}

/// The whole window: which page, each part's own state, and whether the tasks have arrived.
#[derive(Debug, Default, State)]
pub struct Tasks {
    /// `list`, `task` or `delete`: in the link.
    page: String,
    list: TaskList,
    editor: Editor,
    /// The database answered at least once.
    read: bool,
    /// Why the last call to the database didn't work, if it didn't.
    problem: Option<String>,
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

/// A write, then a fresh read when it's done.
fn write(sql: &str, values: Vec<Value>) -> Ask {
    Ask::sql_batch("saved", vec![(sql.into(), values)])
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
                self.list.draft = String::new();
                Next::state(self).ask(write("INSERT INTO tasks (title) VALUES (?)", params![title]))
            }
            ListMsg::Toggle(id) => {
                // The window's copy changes at once; the database follows.
                let mut done = false;
                if let Some(t) = self.list.tasks.iter_mut().find(|t| t.id == id) {
                    t.done = !t.done;
                    done = t.done;
                }
                Next::state(self).ask(write("UPDATE tasks SET done = ? WHERE id = ?", params![done, id]))
            }
            ListMsg::Open(id) => self.to_task("task", id),
        }
    }

    fn on_editor(self, msg: EditorMsg, value: Option<String>) -> Next<Self> {
        let id = self.editor.task;
        match msg {
            EditorMsg::Back => self.to_list(),
            EditorMsg::Title => {
                let title = cut(&value.unwrap_or_default(), MAX_TITLE);
                let saved = title.clone();
                Next::state(self.with_task(|t| t.title = title))
                    .ask(write("UPDATE tasks SET title = ? WHERE id = ?", params![saved, id]))
            }
            EditorMsg::Notes => {
                let notes = cut(&value.unwrap_or_default(), MAX_NOTES);
                let saved = notes.clone();
                Next::state(self.with_task(|t| t.notes = notes))
                    .ask(write("UPDATE tasks SET notes = ? WHERE id = ?", params![saved, id]))
            }
            EditorMsg::Done => {
                let app = self.with_task(|t| t.done = !t.done);
                let done = app.task().map(|t| t.done).unwrap_or(false);
                Next::state(app).ask(write("UPDATE tasks SET done = ? WHERE id = ?", params![done, id]))
            }
            EditorMsg::Delete => self.to_task("delete", id),
            EditorMsg::Cancel => self.to_task("task", id),
            EditorMsg::Confirm => {
                let mut gone = self;
                gone.list.tasks.retain(|t| t.id != id);
                gone.to_list().ask(write("DELETE FROM tasks WHERE id = ?", params![id]))
            }
        }
    }

    /// The tasks have arrived: the window's copy, then whatever its link said, checked.
    fn loaded(mut self, rows: Vec<Vec<Value>>) -> Next<Self> {
        self.list.tasks = rows
            .iter()
            .map(|r| Task { id: r[0].int() as u32, title: r[1].text(), notes: r[2].text(), done: r[3].bool() })
            .collect();
        self.read = true;
        self.problem = None;
        if !matches!(self.list.filter.as_str(), "all" | "open" | "done") {
            self.list.filter = "all".into();
        }
        match self.page.as_str() {
            // A link's task, found in the database: its screen.
            "task" | "delete" if self.task().is_some() => {
                let (page, id) = (self.page.clone(), self.editor.task);
                self.to_task(&page, id)
            }
            "task" | "delete" => self.to_list(),
            _ => Next::state(self),
        }
    }
}

impl App for Tasks {
    type Msg = AppMsg;

    fn handle(self, event: Event<AppMsg>) -> Next<Self> {
        match event {
            Event::Init => Tasks { list: TaskList { filter: "all".into(), ..self.list.clone() }, ..self }.to_list(),
            // Every open: its tables made if they aren't yet, then the tasks read.
            Event::Opened => Next::state(self).ask(Ask::sql_batch(
                "setup",
                SETUP.iter().map(|s| (String::from(*s), vec![])).collect(),
            )),
            Event::Done { msg, .. } if msg == "setup" || msg == "saved" => {
                Next::state(self).ask(Ask::sql("tasks", LOAD, vec![]))
            }
            Event::Rows { msg, rows, .. } if msg == "tasks" => self.loaded(rows),
            // Another window changed them: read them again.
            Event::Change { tables } if tables.iter().any(|t| t == "tasks") => {
                Next::state(self).ask(Ask::sql("tasks", LOAD, vec![]))
            }
            Event::Failed { error, .. } => Next::state(Tasks { problem: Some(error), ..self }),
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
        let mut parts = vec![screen];
        if let Some(p) = &self.problem {
            parts.push(text(p).dim());
        }
        column(parts).spacing(Space::Md).padding(Space::Lg).screen_title("Tasks")
    }
}

fn list_view(app: &Tasks) -> View<ListMsg> {
    let rows: Vec<View<ListMsg>> = app.shown().map(task_row).collect();
    let open = app.list.tasks.iter().filter(|t| !t.done).count();
    let done = app.list.tasks.len() - open;
    let body = if !app.read {
        text("Loading…").dim()
    } else if app.list.tasks.is_empty() {
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

    fn row(id: i64, title: &str, done: bool) -> Vec<Value> {
        vec![Value::Integer(id), Value::Text(title.into()), Value::Text(String::new()), Value::Integer(done as i64)]
    }

    /// A window that has read these tasks.
    fn with(rows: Vec<Vec<Value>>) -> Tasks {
        start()
            .handle(Event::Rows { msg: "tasks".into(), columns: vec![], rows, cut: false })
            .into_state()
    }

    fn titles(app: &Tasks) -> Vec<String> {
        app.shown().map(|t| t.title.clone()).collect()
    }

    fn sql_of(a: &Ask) -> String {
        match a {
            Ask::Sql(_, s, _) => s.clone(),
            Ask::SqlBatch(_, steps) => steps[0].0.clone(),
            _ => String::new(),
        }
    }

    #[test]
    fn opening_makes_its_table_then_reads() {
        let opened = start().handle(Event::Opened);
        assert_eq!(sql_of(&opened.asks()[0]), SETUP[0]);
        let ready = opened.into_state().handle(Event::Done { msg: "setup".into(), changes: 0, last_rowid: 0 });
        assert_eq!(ready.asks(), &[Ask::sql("tasks", LOAD, vec![])]);
    }

    #[test]
    fn rows_become_the_window_s_copy() {
        let app = with(vec![row(1, "Buy milk", true), row(2, "Pay rent", false)]);
        assert!(app.read);
        assert_eq!(titles(&app), ["Buy milk", "Pay rent"]);
        assert!(app.list.tasks[0].done);
    }

    #[test]
    fn adding_trims_refuses_empty_and_writes() {
        let added = list(start(), ListMsg::Add, Some("  Buy milk "));
        assert_eq!(added.asks(), &[write("INSERT INTO tasks (title) VALUES (?)", params!["Buy milk"])]);
        assert_eq!(list(start(), ListMsg::Add, Some("   ")).asks(), &[]);
        // Written: read again.
        let saved = added.into_state().handle(Event::Done { msg: "saved".into(), changes: 1, last_rowid: 1 });
        assert_eq!(saved.asks(), &[Ask::sql("tasks", LOAD, vec![])]);
    }

    #[test]
    fn toggling_writes_and_filtering_shows() {
        let ticked = list(with(vec![row(1, "Buy milk", false), row(2, "Pay rent", false)]), ListMsg::Toggle(1), None);
        assert_eq!(ticked.asks(), &[write("UPDATE tasks SET done = ? WHERE id = ?", params![true, 1u32])]);
        let open = list(ticked.into_state(), ListMsg::Filter("open".into()), None);
        assert_eq!(open.linked(), Some(vec![String::from("page"), String::from("list.filter")]));
        assert_eq!(titles(&open.into_state()), ["Pay rent"]);
    }

    #[test]
    fn opening_a_task_links_it() {
        let opened = list(with(vec![row(2, "Pay rent", false)]), ListMsg::Open(2), None);
        assert_eq!(opened.linked(), Some(vec![String::from("page"), String::from("editor.task")]));
        let back = edit(opened.into_state(), EditorMsg::Back, None);
        assert_eq!(back.into_state().page, "list");
    }

    #[test]
    fn delete_asks_first() {
        let app = list(with(vec![row(1, "Buy milk", false)]), ListMsg::Open(1), None).into_state();
        let asking = edit(app, EditorMsg::Delete, None).into_state();
        assert_eq!(asking.page, "delete");
        let kept = edit(asking, EditorMsg::Cancel, None).into_state();
        assert_eq!(kept.page, "task");
        let gone = edit(edit(kept, EditorMsg::Delete, None).into_state(), EditorMsg::Confirm, None);
        assert_eq!(gone.asks(), &[write("DELETE FROM tasks WHERE id = ?", params![1u32])]);
        assert_eq!(gone.into_state().page, "list");
    }

    #[test]
    fn a_link_s_task_opens_once_read_or_falls_back() {
        // As a link sets it on a fresh window.
        let mut app = start();
        app.page = "task".into();
        app.editor.task = 2;
        let found = app.handle(Event::Rows { msg: "tasks".into(), columns: vec![], rows: vec![row(2, "Pay rent", false)], cut: false });
        assert_eq!(found.into_state().page, "task");
        let mut missing = start();
        missing.page = "task".into();
        missing.editor.task = 42;
        missing.list.filter = "sideways".into();
        let landed = missing.handle(Event::Rows { msg: "tasks".into(), columns: vec![], rows: vec![], cut: false }).into_state();
        assert_eq!((landed.page.as_str(), landed.list.filter.as_str()), ("list", "all"));
    }

    #[test]
    fn another_window_s_change_reads_again() {
        let app = with(vec![]);
        assert_eq!(app.handle(Event::Change { tables: vec!["tasks".into()] }).asks(), &[Ask::sql("tasks", LOAD, vec![])]);
        assert_eq!(with(vec![]).handle(Event::Change { tables: vec![] }).asks(), &[]);
    }

    #[test]
    fn titles_and_notes_are_cut() {
        let long = "x".repeat(300);
        let added = list(start(), ListMsg::Add, Some(&long));
        match &added.asks()[0] {
            Ask::SqlBatch(_, steps) => assert_eq!(steps[0].1, params!["x".repeat(MAX_TITLE)]),
            _ => panic!("expected a write"),
        }
        let app = list(with(vec![row(1, "a", false)]), ListMsg::Open(1), None).into_state();
        let noted = edit(app, EditorMsg::Notes, Some(&"n".repeat(MAX_NOTES + 5))).into_state();
        assert_eq!(noted.list.tasks[0].notes.chars().count(), MAX_NOTES);
    }

    #[test]
    fn a_failed_call_says_why() {
        let app = with(vec![]).handle(Event::Failed { msg: "saved".into(), error: "This app has no room left (100 MB)".into() });
        assert_eq!(app.into_state().problem.as_deref(), Some("This app has no room left (100 MB)"));
    }
}
