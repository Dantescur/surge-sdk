/*
  src/ui.rs
*/
use crate::responses::ListResponse;
use colored::Colorize;
use tabled::{
    Table, Tabled,
    settings::{
        Alignment, Border, Modify, Padding, Style, Width, object::Columns, style::HorizontalLine,
    },
};

#[derive(Tabled)]
struct DomainRow {
    #[tabled(display = "Self::display_domain", rename = "Domain")]
    domain: String,
    #[tabled(display = "Self::display_time", rename = "Time")]
    time: String,
    #[tabled(display = "Self::display_cmd", rename = "Command")]
    command: String,
    #[tabled(display = "Self::display_platform", rename = "Platform")]
    platform: String,
    #[tabled(display = "Self::display_plan", rename = "Plan")]
    plan: String,
}

impl DomainRow {
    // Display methods as associated functions
    fn display_domain(domain: &str) -> String {
        domain.blue().to_string()
    }

    fn display_time(time: &str) -> String {
        time.green().to_string()
    }

    fn display_cmd(cmd: &str) -> String {
        cmd.yellow().to_string()
    }

    fn display_platform(platform: &str) -> String {
        platform.magenta().to_string()
    }

    fn display_plan(plan: &str) -> String {
        plan.trim().to_string().white().to_string()
    }
}

pub fn print_domain_list(list: &ListResponse) {
    let rows: Vec<DomainRow> = list
        .iter()
        .map(|entry| {
            let unknown = "Unknown".to_string();
            DomainRow {
                domain: entry.domain.clone(),
                time: entry
                    .metadata
                    .time_ago_in_words
                    .as_ref()
                    .unwrap_or(&unknown)
                    .clone(),
                command: entry.metadata.cmd.clone(),
                platform: entry.metadata.platform.clone(),
                plan: entry.plan_name.to_string(),
            }
        })
        .collect();

    let mut table = Table::new(rows);
    let style = Style::modern()
        .frame(Border::inherit(Style::rounded()))
        .horizontals([(1, HorizontalLine::inherit(Style::modern()))]);

    table
        .with(style)
        .with(Alignment::left())
        .with(Modify::new(Columns::first()).with(Width::wrap(30).keep_words(true)))
        .with(Modify::new(Columns::new(1..)).with(Width::wrap(15).keep_words(true)))
        .with(Padding::new(1, 1, 0, 0));

    println!("{}", table);
}
