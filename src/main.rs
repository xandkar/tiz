use std::{collections::HashMap, fmt::Debug};

use anyhow::anyhow;
use chrono::{Timelike, Utc};
use chrono_tz::Tz;

fn main() -> anyhow::Result<()> {
    let zones = read_input()?;
    let local = tz_local();
    let table = table(&zones[..], local);
    println!("{table}");
    Ok(())
}

fn read_input() -> anyhow::Result<Vec<Tz>> {
    let mut input: Vec<String> = std::env::args().skip(1).collect();
    if input.is_empty() {
        input = stdin_read_lines()?;
    };
    zones_parse(&input[..])
}

fn stdin_read_lines() -> anyhow::Result<Vec<String>> {
    let mut lines = Vec::new();
    for line_result in std::io::stdin().lines() {
        let line = line_result?;
        lines.push(line)
    }
    Ok(lines)
}

fn zones_parse<S>(strings: &[S]) -> anyhow::Result<Vec<Tz>>
where
    S: AsRef<str> + Debug,
{
    let mut zones: Vec<Tz> = Vec::new();
    for s in strings {
        let zone = s
            .as_ref()
            .parse::<Tz>()
            .map_err(|_| anyhow!("Invalid time zone: {:?}", s))?;
        zones.push(zone);
    }
    Ok(zones)
}

fn tz_local() -> Option<Tz> {
    std::env::var("TZ").ok().and_then(|tz| tz.parse().ok())
}

fn tz_hours_ordered(zones: &[Tz]) -> Vec<(Tz, u32)> {
    let tz_hours: HashMap<Tz, u32> = zones
        .into_iter()
        .map(|tz| (*tz, Utc::now().with_timezone(tz).hour()))
        .collect();
    let mut tz_hours_ordered: Vec<(Tz, u32)> = tz_hours.into_iter().collect();
    tz_hours_ordered.sort_by_key(|(_, hour)| *hour);
    tz_hours_ordered
}

fn table(zones: &[Tz], tz_local: Option<Tz>) -> comfy_table::Table {
    let mut table = comfy_table::Table::new();
    table.load_preset(comfy_table::presets::NOTHING); // No borders or dividers.
    let colors_enabled = console::colors_enabled();
    for (tz, tz_hour) in tz_hours_ordered(zones) {
        let mut row: Vec<String> = (0..24)
            .map(|hour| {
                if hour == tz_hour {
                    match tz_local {
                        Some(tz_local) if tz == tz_local => {
                            hour_fmt_active_local(hour, colors_enabled)
                        }
                        Some(_) | None => {
                            hour_fmt_active(hour, colors_enabled)
                        }
                    }
                } else {
                    hour_fmt_inactive(hour, colors_enabled)
                }
            })
            .collect();
        row.insert(0, tz.to_string());
        table.add_row(row);
    }
    table
}

fn hour_fmt_inactive(hour: u32, colors_enabled: bool) -> String {
    if colors_enabled {
        console::style(hour).white().dim().to_string()
    } else {
        format!(" {hour} ")
    }
}

fn hour_fmt_active(hour: u32, colors_enabled: bool) -> String {
    if colors_enabled {
        console::style(hour).white().bold().to_string()
    } else {
        format!("[{hour}]")
    }
}

fn hour_fmt_active_local(hour: u32, colors_enabled: bool) -> String {
    if colors_enabled {
        console::style(hour).green().bold().to_string()
    } else {
        format!(">{hour}<")
    }
}
