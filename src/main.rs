/*
Cтруктура программы:

1. Генерация списка дат за последние 90 дней.

2. Для каждой даты выполнить GET-запрос к API ЦБ.

3. Распарсить XML, извлечь дату и список валют с курсами.

4. Сохранить данные в структуру.

5. После сбора всех данных обработать их:

- Найти максимум и минимум среди всех курсов с указанием валюты и даты.

- Посчитать среднее значение всех курсов.

6. Вывести результаты.
*/

use chrono::{Duration, Local, NaiveDate};
use quick_xml::de::from_str;
use reqwest::blocking::get;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct ValCurs {
    #[serde(rename = "@Date")]
    date: String,
    #[serde(rename = "Valute")]
    valutes: Vec<Valute>,
}

#[derive(Debug, Deserialize)]
struct Valute {
    #[serde(rename = "CharCode")]
    code: String,
    #[serde(rename = "Value")]
    value: String,
}

#[derive(Debug)]
struct CurrencyStats {
    max: (f64, String, String),
    min: (f64, String, String),
    avg: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let end_date = Local::now().date_naive();
    let start_date = end_date - Duration::days(89);

    let mut all_rates: Vec<(f64, String, String)> = Vec::new();
    
    for day in 0..90 {
        let date = start_date + Duration::days(day);
        let url = format!(
            "http://www.cbr.ru/scripts/XML_daily_eng.asp?date_req={}",
            date.format("%d/%m/%Y")
        );
        
        let response = get(&url)?.text()?;
        let val_curs: ValCurs = from_str(&response)?;

        for valute in val_curs.valutes {
            let value: f64 = valute.value.replace(',', ".").parse::<f64>()?;
            all_rates.push((
                value,
                valute.code,
                val_curs.date.clone(),
            ));
        }
    }

    let stats = calculate_stats(&all_rates);
    print_results(stats);

    Ok(())
}

fn calculate_stats(rates: &[(f64, String, String)]) -> CurrencyStats {
    let mut max = (f64::MIN, String::new(), String::new());
    let mut min = (f64::MAX, String::new(), String::new());
    let mut sum: f64 = 0.0;

    for (value, code, date) in rates {
        sum += value;
        
        if *value > max.0 {
            max = (*value, code.clone(), date.clone());
        }
        
        if *value < min.0 {
            min = (*value, code.clone(), date.clone());
        }
    }

    CurrencyStats {
        max,
        min,
        avg: sum / rates.len() as f64,
    }
}

fn print_results(stats: CurrencyStats) {
    println!("Maximum rate: {:.4} {} on {}", stats.max.0, stats.max.1, stats.max.2);
    println!("Minimum rate: {:.4} {} on {}", stats.min.0, stats.min.1, stats.min.2);
    println!("Average rate: {:.4}", stats.avg);
}