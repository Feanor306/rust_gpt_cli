use chrono::Local;

pub fn get_timestamp() -> String {
    return Local::now().format("[%d/%m/%y %H:%M:%S]").to_string();
}