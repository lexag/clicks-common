pub fn format_hms(time_t: u64) -> String {
    let day_time = time_t % 86400;
    let h = day_time / 3600;
    let m = day_time / 60 % 60;
    let s = day_time % 60;
    format!("{h:0>2}:{m:0>2}:{s:0>2}")
}

pub fn format_hm(time_t: u64) -> String {
    let day_time = time_t % 86400;
    let h = day_time / 3600;
    let m = day_time / 60 % 60;
    format!("{h:0>2}:{m:0>2}")
}
