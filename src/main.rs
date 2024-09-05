use std::collections::HashMap;

use anyhow::{anyhow, bail};
use chrono::{Timelike, Utc};
use chrono_tz::Tz;

fn main() -> anyhow::Result<()> {
    let zones = args_parse()?;
    println!("{}", table(&zones[..], tz_local()));
    Ok(())
}

fn args_parse() -> anyhow::Result<Vec<Tz>> {
    let mut args = std::env::args();
    let exec = args.next().unwrap_or_else(|| unreachable!());
    let args: Vec<String> = args.collect();
    let mut zones: Vec<Tz> = Vec::new();
    if args.is_empty() {
        bail!("USAGE: {} TIME_ZONE [TIME_ZONE] ...", exec);
    }
    for zone in args {
        let zone = zone
            .parse::<Tz>()
            .map_err(|_| anyhow!("Invalid time zone: {:?}", zone))?;
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
    for (tz, tz_hour) in tz_hours_ordered(zones) {
        let mut row: Vec<String> = (0..24)
            .map(|hour| {
                let hour_style = console::style(hour);
                match (hour == tz_hour, tz_local) {
                    (true, Some(tz_local)) if tz == tz_local => {
                        hour_style.green().bold()
                    }
                    (true, _) => hour_style.white().bold(),
                    _ => hour_style.white().dim(),
                }
                .to_string()
            })
            .collect();
        row.insert(0, tz.to_string());
        table.add_row(row);
    }
    table
}
