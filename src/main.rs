use std::env;
use reqwest;
use tokio;
use std::process::exit;
use termsearch;
use webbrowser;

//use std::{error::Error, io};

//use color_eyre::Result;
use ratatui::{
//    text::Text,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode,/* KeyEvent, KeyEventKind*/},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{SLATE},
        Color, Modifier, Style, Stylize,
    },
    // symbols,
    // text::Line,
    widgets::{
        HighlightSpacing, List, ListItem, ListState, Paragraph,
        StatefulWidget, Widget,
    },
    DefaultTerminal,
};

use termsearch::{
    SearchResult,
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let query = get_query(args);

    let output = search(&query).await;
    let res_list = ResultList {
        items: output,
        state: ListState::default(),
    };

    // color_eyre::install().unwrap();
    let terminal = ratatui::init();
    let app: App = App::new(res_list);
    app.run_app(terminal);

    ratatui::restore();

}

fn get_query(args: Vec<String>) -> String {
    if args.len() == 1 {
        println!("No query was provided\n");
        exit(1);
    }
    let query = args.iter()
        .skip(1)
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    query
}

async fn search(query: &str) -> Vec<SearchResult> {
        let client = reqwest::Client::new();
        let param = [("q", &query)];
        let res = client.post("https://lite.duckduckgo.com/lite/")
            .form(&param)
            .send()
            .await;
        let html = res.unwrap().text().await.unwrap();
        termsearch::parse(html)
}

struct ResultList {
    items: Vec<SearchResult>,
    state: ListState,
}

struct App {
    should_exit: bool,
    result_list: ResultList,
}

impl App {

    fn new(result_list: ResultList) -> Self {
        Self {
            should_exit: false,
            result_list,
        }
    }

    fn run_app(mut self,mut terminal: DefaultTerminal) {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area())).unwrap();

            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == event::KeyEventKind::Release {
                    // Skip events that are not KeyEventKind::Press
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
                    KeyCode::Char('j') | KeyCode::Down => self.result_list.state.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.result_list.state.select_previous(),
                    KeyCode::Enter => if let Some(i) = self.result_list.state.selected() {
                        let _ = webbrowser::open(&self.result_list.items[i].url);
                    }
                    _ => {}
                }
            }
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(main_area, buf);
    }
}

impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Search Results")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf:&mut Buffer) {
        Paragraph::new("Use ↓ ↑ or j, k to move | Enter to open url | q to quit")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {

        let items: Vec<ListItem> = self
            .result_list
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let color = alternate_colors(i);
                ListItem::from(item).bg(color)
            })
            .collect();

            let list = List::new(items)
                .highlight_style(Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD))
                .highlight_symbol(">")
                .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.result_list.state)
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        SLATE.c950
    } else {
        SLATE.c900
    }
}



