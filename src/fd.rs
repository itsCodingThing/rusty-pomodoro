use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use devicons::FileIcon;
use ratatui::{
    layout::{Constraint, Layout, Position},
    style::{palette::tailwind::SLATE, Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};
use std::{fs, str::FromStr};

#[derive(Debug)]
pub struct FdPath {
    path: String,
    name: String,
    spacer: String,
    is_dir: bool,
    is_expended: bool,
    total_paths: usize,
}

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const NESTED_SPACER: &str = "|_";

pub fn init() -> Result<()> {
    let paths = retreive_paths(String::from("./"));
    let mut fd = Fd::new(paths);

    ratatui::run(|terminal| fd.run(terminal))
}

fn retreive_paths(path: String) -> Vec<FdPath> {
    let mut paths: Vec<FdPath> = Vec::new();

    for entry in fs::read_dir(path).expect("unable to read path") {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = entry.metadata().unwrap();
        let p = format!("{}", path.display());
        let spacer = String::from("");

        if metadata.is_dir() {
            paths.push(FdPath {
                path: p,
                name: entry.file_name().into_string().unwrap(),
                spacer,
                is_expended: false,
                is_dir: true,
                total_paths: 0,
            });
        } else if metadata.is_file() {
            paths.push(FdPath {
                path: p,
                name: entry.file_name().into_string().unwrap(),
                spacer,
                is_expended: false,
                is_dir: false,
                total_paths: 0,
            });
        }
    }

    paths
}

#[derive(Debug)]
pub struct Fd {
    exit: bool,
    list_state: ListState,
    list: Vec<FdPath>,
    is_renaming: bool,
}

impl Fd {
    pub fn new(paths: Vec<FdPath>) -> Fd {
        Fd {
            exit: false,
            list_state: ListState::default(),
            list: paths,
            is_renaming: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        // defalt to select first item
        self.list_state.select_first();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let bottom_height = if self.is_renaming { 3 } else { 1 };

        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(bottom_height),
        ]);
        let horizontal = Layout::horizontal([Constraint::Fill(1); 2]);

        let [title_area, main_area, bottom_area] = vertical.areas(frame.area());
        let [list_area, preview_area] = horizontal.areas(main_area);

        let items: Vec<ListItem> = self
            .list
            .iter()
            .map(|p| {
                let icon = FileIcon::from(&p.path);
                let spacer = &p.spacer;

                ListItem::from(
                    Text::from(Line::from(vec![
                        // spacer to show
                        Span::from(spacer),
                        // icon with color
                        Span::from(icon.to_string())
                            .style(Style::new().fg(Color::from_str(icon.color).unwrap())),
                        Span::from(" "),
                        // filename
                        Span::from(p.name.to_owned()),
                    ]))
                    .bold(),
                )
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::bordered()
                    .title(Line::from(" All File/Dir ".bold()).centered())
                    .border_set(border::THICK),
            )
            .highlight_symbol(">> ".bold())
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_style(SELECTED_STYLE);

        // title area
        frame.render_widget(Line::from(" File Explorer ".bold()).centered(), title_area);

        // list area
        frame.render_stateful_widget(list, list_area, &mut self.list_state);

        // preview area
        frame.render_widget(
            Block::bordered()
                .title(Line::from(" Preview ".bold()).centered())
                .border_set(border::THICK),
            preview_area,
        );

        // bottom area
        if self.is_renaming {
            // set cursor position inside box
            frame.set_cursor_position(Position::new(bottom_area.x + 1, bottom_area.y + 1));

            frame.render_widget(
                Paragraph::new("rename this file").block(
                    Block::bordered()
                        .title(Line::from(" Rename ".bold()).centered())
                        .title_bottom(Line::from(" submit <enter> "))
                        .border_set(border::THICK),
                ),
                bottom_area,
            );
        } else {
            frame.render_widget(
                Line::from(vec![
                    " Down ".into(),
                    "<j>".blue().bold(),
                    " Up ".into(),
                    "<k>".blue().bold(),
                    " Quit ".into(),
                    "<q> ".blue().bold(),
                ]),
                bottom_area,
            );
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }

            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),

            KeyCode::Char('j') => {
                self.down();
            }

            KeyCode::Char('k') => {
                self.up();
            }

            KeyCode::Enter => {
                self.toggle_expend();
            }

            KeyCode::Char('r') => {
                self.is_renaming = !self.is_renaming;
            }

            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn down(&mut self) {
        self.list_state.select_next();
    }

    fn up(&mut self) {
        self.list_state.select_previous();
    }

    fn toggle_expend(&mut self) {
        if let Some(idx) = self.list_state.selected() {
            if let Some(item) = self.list.get_mut(idx) {
                if item.is_dir {
                    if item.is_expended {
                        item.is_expended = false;

                        let remove_idx_from = idx + 1;
                        let remove_idx_to = item.total_paths + remove_idx_from;

                        item.total_paths = 0;
                        self.list.drain(remove_idx_from..remove_idx_to);
                    } else {
                        item.is_expended = true;

                        let insert_idx = idx + 1;
                        let mut paths = retreive_paths(item.path.to_owned());

                        item.total_paths += paths.len();

                        paths.iter_mut().for_each(|p| {
                            p.spacer = item.spacer.to_owned() + NESTED_SPACER;
                        });

                        self.list.splice(insert_idx..insert_idx, paths);
                    }
                }
            }
        }
    }
}
