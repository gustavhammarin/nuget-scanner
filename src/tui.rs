use cvss::v3::Base;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::schemas::OsvVuln;

pub struct App {
    pub entries: Vec<OsvVuln>,
    pub state: TableState,
}

impl App {
    pub fn new(entries: Vec<OsvVuln>) -> Self {
        let mut state = TableState::default();
        if !entries.is_empty() { state.select(Some(0)); }
        Self { entries, state }
    }

    pub fn next(&mut self) {
        let i = self.state.selected().map(|i| (i + 1) % self.entries.len()).unwrap_or(0);
        self.state.select(Some(i));
    }

    pub fn prev(&mut self) {
        let len = self.entries.len();
        let i = self.state.selected().map(|i| if i == 0 { len - 1 } else { i - 1 }).unwrap_or(0);
        self.state.select(Some(i));
    }
}

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(f.area());

    // ── Tabell ──────────────────────────────────────────────
    let header = Row::new(vec!["ID", "ECOSYSTEM", "PACKAGE", "SEVERITY", "PUBLISHED"])
        .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = app.entries.iter().map(|e| {
        let sev = get_severity_label(e);
        let sev_color = match sev {
            "CRITICAL" => Color::Red,
            "HIGH"     => Color::Yellow,
            "MEDIUM"   => Color::Cyan,
            "LOW"     => Color::Green,
            _         => Color::DarkGray,
        };

        Row::new(vec![
            Cell::from(e.id.as_str()),
            Cell::from(get_ecosystem(e)),
            Cell::from(get_package(e)),
            Cell::from(sev).style(Style::default().fg(sev_color)),
            Cell::from(e.published.as_deref().unwrap_or("—")),
        ])
    }).collect();

    let table = Table::new(rows, [
        Constraint::Length(28),
        Constraint::Length(12),
        Constraint::Fill(1),
        Constraint::Length(14),
        Constraint::Length(12),
    ])
    .header(header)
    .block(Block::default().borders(Borders::ALL).title(" sårbarheter "))
    .row_highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
    .highlight_symbol("▶ ");

    f.render_stateful_widget(table, chunks[0], &mut app.state);

    // ── Detaljpanel ─────────────────────────────────────────
    if let Some(i) = app.state.selected() {
        if let Some(entry) = app.entries.get(i) {
            let aliases = entry.aliases.as_ref()
                .map(|a| a.join(", "))
                .unwrap_or_else(|| "—".to_string());

            let summary = entry.summary.as_deref().unwrap_or("ingen sammanfattning");

            let text = format!(
                "ID:       {}\nAlias:    {}\nPaket:    {} ({})\nPubl.:    {}\n\n{}",
                entry.id,
                aliases,
                get_package(entry),
                get_ecosystem(entry),
                entry.published.as_deref().unwrap_or("—"),
                summary,
            );

            let detail = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL).title(" detaljer "))
                .wrap(ratatui::widgets::Wrap { trim: true });

            f.render_widget(detail, chunks[1]);
        }
    }
}


fn get_ecosystem(entry: &OsvVuln) -> &str {
        entry.affected.as_ref()
        .and_then(|a| a.first())
        .and_then(|a| a.package.as_ref())
        .and_then(|p| Some(p.ecosystem.as_ref()))
        .unwrap_or("—")
}

fn get_package(entry: &OsvVuln) -> &str {
    entry.affected.as_ref()
        .and_then(|a| a.first())
        .and_then(|a| a.package.as_ref())
        .and_then(|p| Some(p.name.as_ref()))
        .unwrap_or("—")
}

fn get_severity_label(entry: &OsvVuln) -> &str {
    let score = entry.severity.as_ref()
        .and_then(|s| s.first())
        .map(|s| s.score.as_str())
        .unwrap_or("");

    let val: f64 = score
        .parse::<Base>()
        .map(|b| b.score().value())
        .unwrap_or(0.0);

    match val {
        v if v >= 9.0 => "CRITICAL",
        v if v >= 7.0 => "HIGH",
        v if v >= 4.0 => "MEDIUM",
        v if v > 0.0  => "LOW",
        _             => "—",
    }
}