use std::collections::HashMap;

use anyhow::{anyhow, bail};
use chrono::{Timelike, Utc};
use chrono_tz::Tz;

fn main() -> anyhow::Result<()> {
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
    let hour_in_zone: HashMap<Tz, u32> = zones
        .into_iter()
        .map(|tz| (tz, Utc::now().with_timezone(&tz).hour()))
        .collect();
    let mut hour_in_zone: Vec<(Tz, u32)> = hour_in_zone.into_iter().collect();
    hour_in_zone.sort_by_key(|(_, h)| *h);
    let hours: Vec<u32> = (0..24).collect();
    let mut table = comfy_table::Table::new();
    table.load_preset(comfy_table::presets::NOTHING); // No borders or dividers.
    for (tz, tz_h) in hour_in_zone {
        let mut row: Vec<String> = hours
            .iter()
            .map(|h| {
                let h_str = h.to_string();
                if *h == tz_h {
                    console::style(h_str).bold().to_string()
                } else {
                    console::style(h_str).dim().to_string()
                }
            })
            .collect();
        row.insert(0, tz.to_string());
        table.add_row(row);
    }
    println!("{}", table);
    Ok(())
}
