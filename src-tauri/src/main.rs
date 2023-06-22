// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{Local, Timelike};
use islam::salah::{Config, Location, Madhab, Method, PrayerSchedule};

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

fn example() -> Result<AppData, Box<dyn std::error::Error>> {
    // https://www.mapcoordinates.net/en
    let cluring_city = Location::new(-8.4330044, 114.1995126);
    let config = Config::new().with(Method::Karachi, Madhab::Shafi);

    let prayer_schedule = PrayerSchedule::new(cluring_city)?;
    let prayer_times = prayer_schedule
        .on(Local::now().date_naive())
        .with_config(config)
        .calculate()?;

    let fajr = prayer_times.fajr;
    let sherook = prayer_times.sherook;
    let dohr = prayer_times.dohr;
    let asr = prayer_times.asr;
    let maghreb = prayer_times.maghreb;
    let ishaa = prayer_times.ishaa;

    let current_prayer = prayer_times.current()?;
    let (hour, minute) = prayer_times.time_remaining()?;

    let next_prayer = prayer_times.next()?;
    let time = prayer_times.time(next_prayer);
    let time = time.format("%H:%M").to_string();

    let app_data = AppData {
        fajr: format!("{}:{}  ", fajr.hour(), fajr.minute()),
        sherook: format!("{}:{}  ", sherook.hour(), sherook.minute()),
        dohr: format!("{}:{}", dohr.hour(), dohr.minute()),
        asr: format!("{}:{}", asr.hour(), asr.minute()),
        maghreb: format!("{}:{}", maghreb.hour(), maghreb.minute()),
        ishaa: format!("{}:{}", ishaa.hour(), ishaa.minute()),
        current_prayer: format!("{}: ({}:{})", current_prayer.name()?, hour, minute),
        next_prayer: format!("{}: ({})", next_prayer.name()?, time),
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
