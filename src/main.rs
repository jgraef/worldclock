use std::{
    borrow::Cow,
    ops::Deref,
    path::PathBuf,
};

use chrono::{
    DateTime,
    Local,
    Utc,
};
use color_eyre::eyre::{
    eyre,
    Error,
};
use prettytable::{
    format::consts::FORMAT_CLEAN,
    Attr,
    Cell,
    Row,
    Table,
};
use serde::Deserialize;
use structopt::StructOpt;

/// Shows the current time in multiple time zones.
#[derive(Clone, Debug, StructOpt)]
struct Args {
    /// Path to config file that specifies the timezones you want to have
    /// displayed.PathBuf
    ///
    /// If not specified, ~/.config/worldclock.toml will be used.
    ///
    /// The file consists of a series of `[[clock]]` definitions that must
    /// specify a timezone with the `tz` key. To list available timezones,
    /// you can use `timedatectl list-timezones` (using systemd).
    ///
    /// Optionally you can specify a custom name for the clock. If omitted, the
    /// name of the time zone is used.
    ///
    /// Example:
    ///
    ///     # Local clock
    ///     [[clocks]]
    ///
    ///     [[clocks]]
    ///     tz = "Europe/Berlin"
    ///
    ///     [[clocks]]
    ///     name = "Costa Rica"
    ///     tz = "America/Costa_Rica"
    ///
    ///     [[clocks]]
    ///     name = "New York"
    ///     tz = "America/New_York"
    #[structopt(verbatim_doc_comment, short, long)]
    config: Option<PathBuf>,
    /*
    /// Instead of displaying the current time, use the specified time.
    // FIXME: Parse properly
    #[structopt(short, long)]
    time: Option<NaiveDateTime>,

    /// If `--time` is used, it will be interpreted as UTC.
    #[structopt(short, long)]
    utc: bool,
    */
}

#[derive(Clone, Debug, Deserialize, Default)]
struct Clock {
    name: Option<String>,
    tz: Option<Tz>,
}

#[derive(Clone, Debug, Deserialize)]
struct Config {
    #[serde(default)]
    clocks: Vec<Clock>,
}

fn print_clocks(clocks: &[Clock], time: DateTime<Utc>) {
    let mut table = Table::new();
    table.set_format(*FORMAT_CLEAN);

    for clock in clocks {
        let local_time;
        let tz_name;

        if let Some(tz) = &clock.tz {
            local_time = time.with_timezone(&tz.0).naive_local();
            tz_name = tz.0.to_string();
        }
        else {
            local_time = time.with_timezone(&Local).naive_local();
            tz_name = "Local".to_string();
        }

        let name = clock.name.as_ref().unwrap_or(&tz_name);

        table.add_row(Row::new(vec![
            Cell::new(&name).with_style(Attr::Bold),
            Cell::new(&local_time.format("%H:%M:%S").to_string()),
        ]));
    }

    table.printstd();
}

fn main() -> Result<(), Error> {
    let args = Args::from_args();

    let config_path = if let Some(config_path) = args.config {
        config_path
    }
    else {
        dirs::config_dir()
            .ok_or_else(|| eyre!("Could not determine config directory"))?
            .join("worldclock.toml")
    };

    let config_text = std::fs::read_to_string(&config_path)
        .map_err(|e| eyre!("Could not open file: {}: {:#}", config_path.display(), e))?;
    let mut config: Config = toml::from_str(&config_text)?;

    // If no clocks are specified, we will add a local one.
    if config.clocks.is_empty() {
        config.clocks.push(Clock::default());
    }

    // TODO: Parse the --time option properly from the command line
    /*let time = match (args.time, args.utc) {
        (Some(time), false) => Utc
            .from_local_datetime(&time)
            .single()
            .ok_or_else(|| anyhow!("Conversion from local time failed. This can happen during time transition."))?,
        (Some(time), true) => Utc.from_utc_datetime(&time),
        (None, false) => Utc::now(),
        (None, true) => bail!("--utc can only be used with --time."),
    };*/

    let time = Utc::now();

    print_clocks(&config.clocks, time);

    Ok(())
}

/// Wrapper to implement Deserialize for [`Tz`].
#[derive(Clone, Debug)]
struct Tz(pub chrono_tz::Tz);

impl Deref for Tz {
    type Target = chrono_tz::Tz;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Tz {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: Cow<'de, str> = Deserialize::deserialize(deserializer)?;
        s.parse().map(Self).map_err(serde::de::Error::custom)
    }
}
