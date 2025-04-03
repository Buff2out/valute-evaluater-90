/*
Cтруктура программы:

1. Генерация списка дат за последние 90 дней.

2. Для каждой даты выполнить GET-запрос к API ЦБ.

3. Распарсить XML, извлечь дату и список валют с курсами.

4. Обработать полученные за итерацию данные, 
посчитать минимум и максимум, 
добавить в сумму значения для подготовки к вычислению матожидания (ср.значения).

5. Сохранить данные в структуру.

6. После сбора всех данных обработать их:

- Посчитать среднее значение всех курсов.

7. Вывести результаты.
*/

use chrono::{Duration, Local};
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

struct CurrencyStats {
    max: (f64, String, String),
    min: (f64, String, String),
    sum: f64,
    count: usize,
}

impl CurrencyStats {
    fn new() -> Self {
        Self {
            max: (f64::MIN, String::new(), String::new()),
            min: (f64::MAX, String::new(), String::new()),
            sum: 0.0,
            count: 0,
        }
    }

    fn update(&mut self, value: f64, code: &str, date: &str) {
        self.sum += value;
        self.count += 1;

        if value > self.max.0 {
            self.max = (value, code.to_string(), date.to_string());
        }

        if value < self.min.0 {
            self.min = (value, code.to_string(), date.to_string());
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let end_date = Local::now().date_naive();
    let start_date = end_date - Duration::days(89);
    let mut stats = CurrencyStats::new();

    for day in 0..90 {
        let date = start_date + Duration::days(day);
        let url = format!(
            "http://www.cbr.ru/scripts/XML_daily_eng.asp?date_req={}",
            date.format("%d/%m/%Y")
        );

        match process_day(&url) {
            Ok(day_stats) => {
                for (value, code, date) in day_stats {
                    stats.update(value, &code, &date);
                }
            }
            Err(e) => eprintln!("Error processing {}: {}", date, e),
        }
    }

    print_results(stats);
    Ok(())
}

fn process_day(url: &str) -> Result<Vec<(f64, String, String)>, Box<dyn Error>> {
    let response = get(url)?.text()?;
    let val_curs: ValCurs = from_str(&response)?;
    
    val_curs.valutes
        .into_iter()
        .map(|v| {
            Ok((
                v.value.replace(',', ".").parse::<f64>()?,
                v.code,
                val_curs.date.clone(),
            ))
        })
        .collect()
}

fn print_results(stats: CurrencyStats) {
    println!("Maximum rate: {:.4} {} on {}", stats.max.0, stats.max.1, stats.max.2);
    println!("Minimum rate: {:.4} {} on {}", stats.min.0, stats.min.1, stats.min.2);
    println!("Average rate: {:.4}", stats.sum / stats.count as f64);
}

