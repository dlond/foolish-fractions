use chrono::{NaiveDate, Datelike};
use std::{cmp::Reverse, time::Instant, collections::BinaryHeap};
use ordered_float::NotNan;

type ReverseFloat = Reverse<NotNan<f64>>;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct FoolishDate(ReverseFloat, NaiveDate);

const DAYS_IN_YEAR: usize = 365;

fn main() {
    let start = Instant::now();

    let apr = foolish(NaiveDate::from_ymd(2022, 4, 1));

    let mut min_heap = BinaryHeap::with_capacity(DAYS_IN_YEAR);

    let dates = NaiveDate::from_ymd(2022, 1, 1)
        .iter_days()
        .take(DAYS_IN_YEAR);

    for d in dates {
        let f = NotNan::new((foolish(d) - apr).abs()).unwrap();
        min_heap.push(FoolishDate(Reverse(f), d));
    }

    let duration = start.elapsed();

    min_heap.pop(); // this is 1 Apr 2022
    if let Some(d) = min_heap.pop() { // this is closest to 1 Apr 2022
        println!("Found {} in {}ms", d.1, duration.as_micros());
    }
}

fn foolish(date: NaiveDate) -> f64 {
    let day = date.day() as f64;
    let month = date.month() as f64;
    let year = (date.year() - 2000) as f64;

    // Do some algebra to rewrite eqn as quadratic in F
    // a, b, c are the coeffs
    let a = day * month + year;
    let b = day.powi(2) - month.powi(2) - year.powi(2) - day * month * year;
    let c = - day.powi(2) * year - day * month;

    // solve the quadratic
    (-b + (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a)
}