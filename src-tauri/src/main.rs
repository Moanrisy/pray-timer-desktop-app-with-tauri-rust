// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{Datelike, Local, Timelike};
use hello_tauri::pray::{Config, Location, Madhab, Method, PrayerSchedule};
use time::macros::offset;
use time::Month;
use time::{format_description, Date};

struct AppData {
    fajr: String,
    sherook: String,
    dohr: String,
    asr: String,
    maghreb: String,
    ishaa: String,
    current_prayer: String,
    next_prayer: String,
}

fn convert_to_month_name(month: u32) -> Option<Month> {
    match month {
        1 => Some(Month::January),
        2 => Some(Month::February),
        3 => Some(Month::March),
        4 => Some(Month::April),
        5 => Some(Month::May),
        6 => Some(Month::June),
        7 => Some(Month::July),
        8 => Some(Month::August),
        9 => Some(Month::September),
        10 => Some(Month::October),
        11 => Some(Month::November),
        12 => Some(Month::December),
        _ => None,
    }
}

fn example() -> Result<AppData, Box<dyn std::error::Error>> {
    let current_date = Local::now();
    let year = current_date.year();
    let month = current_date.month();
    let day = current_date.day();
    let today_date =
        Date::from_calendar_date(year, convert_to_month_name(month).unwrap(), day as u8)
            .expect("Failed to create fallback date");

    let current_time = Local::now().time();
    let time_now = today_date
        .with_hms(
            current_time.hour() as u8,
            current_time.minute() as u8,
            current_time.second() as u8,
        )?
        .assume_offset(offset!(+7:00:00));

    // https://www.mapcoordinates.net/en
    let cluring_city = Location::new(-8.4330044, 114.1995126);
    let config = Config::new().with(Method::Karachi, Madhab::Shafi);

    let prayer_schedule = PrayerSchedule::new(cluring_city, today_date)?;
    let prayer_times = prayer_schedule
        .on(today_date)
        .with_config(config)
        .calculate()?;

    let fajr = prayer_times.fajr;
    let sherook = prayer_times.sherook;
    let dohr = prayer_times.dohr;
    let asr = prayer_times.asr;
    let maghreb = prayer_times.maghreb;
    let ishaa = prayer_times.ishaa;

    let (hour, minute) = prayer_times.time_remaining(time_now)?;
    let current_prayer = prayer_times.current(time_now)?;
    let next_prayer = prayer_times.next(time_now)?;
    let time = prayer_times.time(next_prayer);
    let format = format_description::parse("[hour]:[minute]").unwrap();
    let time = time.format(&format).unwrap();

    let app_data = AppData {
        fajr: format!("{}:{}  ", fajr.hour(), fajr.minute()),
        sherook: format!("{}:{}  ", sherook.hour(), sherook.minute()),
        dohr: format!("{}:{}", dohr.hour(), dohr.minute()),
        asr: format!("{}:{}", asr.hour(), asr.minute()),
        maghreb: format!("{}:{}", maghreb.hour(), maghreb.minute()),
        ishaa: format!("{}:{}", ishaa.hour(), ishaa.minute()),
        current_prayer: format!(
            "{}: ({}:{})",
            current_prayer.name(today_date)?,
            hour,
            minute
        ),
        next_prayer: format!("{}: ({})", next_prayer.name(today_date)?, time),
    };

    Ok(app_data)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet() -> String {
    let data = example().unwrap_or_else(|e| {
        let error_message = format!("Error: {}", e);
        AppData {
            fajr: error_message.clone(),
            sherook: error_message.clone(),
            dohr: error_message.clone(),
            asr: error_message.clone(),
            maghreb: error_message.clone(),
            ishaa: error_message.clone(),
            current_prayer: error_message.clone(),
            next_prayer: error_message,
        }
    });

    format!(
        // "fajr: {}\nsherook: {}\ndohr: {}\nasr: {}\nmaghreb: {}\nishaa: {}\n\nC:{}|N:{}",
        "{} fajr\n{} sherook\n{} dohr\n{} asr\n{} maghreb\n{} ishaa\n\nC:{}|N:{}",
        data.fajr,
        data.sherook,
        data.dohr,
        data.asr,
        data.maghreb,
        data.ishaa,
        data.current_prayer,
        data.next_prayer
    )
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
