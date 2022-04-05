use chrono::{NaiveDate, Datelike};
use std::cmp::Ordering;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    
    let apr_foolish = foolish(&NaiveDate::from_ymd(2022, 4, 1));
    let mut dates: Vec<NaiveDate> =  NaiveDate::from_ymd(2022, 1, 1)
        .iter_days()
        .take(365)
        .collect();

    dates.sort_by(|d1, d2| foolish_cmp(foolish(d1), foolish(d2), apr_foolish));
    // dates[0] is 1 Apr 2022, dates[1] is the next closest

    let duration = start.elapsed();

    println!("{} in {}ms", dates[1], duration.as_micros());
}

fn foolish_cmp(a: f64, b: f64, o: f64) -> Ordering
{
    let a = (a - o).abs();
    let b = (b - o).abs();
    
    a.partial_cmp(&b).unwrap()
}

fn foolish(date: &NaiveDate) -> f64 {
    let day = date.day() as f64;
    let month = date.month() as f64;
    let year = date.year() as f64;
    let year = year - 2000.0;

    let a = day * month + year;
    let b = day.powi(2) - month.powi(2) - year.powi(2) - day * month * year;
    let c = - day.powi(2) * year - day * month;

    (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a)
}