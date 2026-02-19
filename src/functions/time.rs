use crate::functions::expect_arity;
use crate::interpreter::Value;
use chrono::{Datelike, Local, Timelike};

pub fn register(map: &mut std::collections::HashMap<String, fn(Vec<Value>) -> Value>) {
    map.insert("time".into(), time);
}

fn time(args: Vec<Value>) -> Value {
    if let Err(e) = expect_arity("time", &args, 1) {
        return e;
    }

    let unit = match &args[0] {
        Value::Symbol(s) | Value::Str(s) => s.as_str(),
        _ => return Value::Error("time expects unit name".into()),
    };

    let now = Local::now();

    let value = match unit {
        "millisecond" | "milliseconds" | "ms" => now.timestamp_millis() % 1000,
        "second" | "seconds" | "sec" | "s" => now.second() as i64,
        "minute" | "minutes" | "min" | "m" => now.minute() as i64,
        "hour" | "hours" | "h" => now.hour() as i64,
        "day" | "days" => now.day() as i64,
        "month" | "months" => now.month() as i64,
        "year" | "years" => now.year() as i64,
        _ => return Value::Error("invalid time unit".into()),
    };

    Value::Num(value)
}
