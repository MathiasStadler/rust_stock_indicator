// FROM HERE
// https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html

// /w stock data ./data/trx_us_d.csv

// use std::error::Error;
// use csv::Reader;
use csv::Writer;

use ta::indicators::SimpleMovingAverage as Sma;
use ta::DataItem;
use ta::Next;

// use serde::Deserialize;

// https://github.com/BurntSushi/rust-csv/issues/125

// Date,Open,High,Low,Close,Volume

// #[derive(Debug, Deserialize)]
// struct Record {
//     #[serde(rename = "Date")]
//     date: String,

//     #[serde(rename = "Open")]
//     open: f32,

//     #[serde(rename = "High")]
//     high: f32,

//     #[serde(rename = "Low")]
//     low: f32,

//     #[serde(rename = "Close")]
//     close: f32,

//     #[serde(rename = "Volume")]
//     volume: f32,
// }

fn main() {
    let mut sma_7 = Sma::new(7).unwrap();

    // get stock data see project_path.md## add stock data in folder
    let mut reader = csv::Reader::from_path("stock_data/stock_trex_data.csv").unwrap();
    // FROM HERE - https://docs.rs/csv/latest/csv/struct.Writer.html
    let mut wtr = Writer::from_path("stock_data/output_stock_data_w_sma.csv").unwrap();
   

    wtr.write_record(&["Date", "Open", "High", "Low", "Close", "Volume", "SMA(7)"])
        .unwrap();
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

        let sma_7_val = sma_7.next(&dt);
        // println!("{}: {} = {:2.2}", date, sma_7, sma_7_val
        
        println!(
            " {:?}, {:?}, {:?}, {:?},{:?}, {:?}, {:2.2}",
            date, open, high, low, close, volume, sma_7_val

        );
        wtr.write_record(&[
            date,
            open.to_string(),
            high.to_string(),
            low.to_string(),
            close.to_string(),
            volume.to_string(),
            sma_7_val
            .to_string(),
        ])
        .unwrap();
        
        
    }
}

// cargo run --example 01_get_stock_data
