
use rand::Rng;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

// use rand::Rng;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

// ta
use ta::indicators::SimpleMovingAverage as Sma;
use ta::DataItem;
use ta::Next;

// for csv read from file
// use serde::Deserialize;

#[cfg(test)]



const CSV_STOCK_INPUT: &str = "stock_data/stock_trex_data.csv";


#[derive(Debug)]
pub struct StockData {
    datetime: DateTime<Utc>,
    high: Decimal,
    low: Decimal,
    open: Decimal,
    close: Decimal,
    #[allow(dead_code)]
    net_change: Option<Decimal>,
    #[allow(dead_code)]
    net_change_percent: Option<Decimal>,
}

impl StockData {
    pub fn new(
        datetime: DateTime<Utc>,
        high: Decimal,
        low: Decimal,
        open: Decimal,
        close: Decimal,
    ) -> Self {
        Self {
            datetime,
            high,
            low,
            open,
            close,
            net_change: None,
            net_change_percent: None,
        }
    }
}

fn generate_utc_date_from_date_string(date_string: &str) -> DateTime<Utc> {
    let day_one = NaiveDateTime::parse_from_str(date_string, "%m-%d-%Y %H:%M").unwrap();
    Utc.from_utc_datetime(&day_one)
}

#[allow(dead_code)]
fn generate_stock_data_from_csv(date_string: &str) -> StockData {
    let mut sma = Sma::new(7).unwrap();
    let mut reader = csv::Reader::from_path(CSV_STOCK_INPUT).unwrap();

    for record in reader.deserialize() {
        let (date, open, high, low, close, volume): (String, f64, f64, f64, f64, f64) =
            record.unwrap();

        let dt = DataItem::builder()
            .open(open)
            .high(high)
            .low(low)
            .close(close)
            .volume(volume)
            .build()
            .unwrap();

        let sma_val = sma.next(&dt);
        // println!("{}: {} = {:2.2}", date, sma, sma_val);
        println!(
            " {:?}, {:?}, {:?}, {:?},{:?}, {:?}, {:2.2}",
            date, open, high, low, close, volume, sma_val
        );
        // println!("{}: {} = {:2.2}", date, sma, sma_val);
    }

    // let base_stock_data_series: Vec<_> = vec![];
    let base_stock_data_series: Vec<(f64, f64, f64, f64)> = vec![];
    let base_data_series_len = base_stock_data_series.len();

    let mut rng = rand::thread_rng();

    let high = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].1)
        .unwrap()
        .round_dp(2);
    let low = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].2)
        .unwrap()
        .round_dp(2);
    let open = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].0)
        .unwrap()
        .round_dp(2);
    let close = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].3)
        .unwrap()
        .round_dp(2);

    StockData::new(
        generate_utc_date_from_date_string(date_string),
        high,
        low,
        open,
        close,
    )
}

#[test]
fn test_generate_stock_data_from_csv() {
    let stock_date: &str = "";

    generate_stock_data_from_csv(&stock_date);
}
