use crate::interpreter::Value;
use chrono::{Datelike, Timelike, Local};

pub fn register(map: &mut std::collections::HashMap<String, fn(Vec<Value>) -> Value>) {
    map.insert("time".into(), time);
}

fn time(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        return Value::Error("time expects 1 argument".into());
    }

    let unit = match &args[0] {
        Value::Symbol(s) | Value::Str(s) => s.as_str(),
        _ => return Value::Error("time expects unit name".into()),
    };

    let now = Local::now();

    let value = match unit {
        "millisecond" | "ms" => now.timestamp_millis() % 1000,
        "second" | "sec" | "s" => now.second() as i64,
        "minute" | "min" |"m" => now.minute() as i64,
        "hour" | "h" => now.hour() as i64,
        "day" => now.day() as i64,
        "month" => now.month() as i64,
        "year" => now.year() as i64,
        _ => return Value::Error("invalid time unit".into()),
    };

    Value::Num(value)
}