use crate::parser::value::Value;

use rand::{thread_rng, Rng};

pub fn clock() -> Value {
    Value::Number(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64(),
    )
}

pub fn random() -> Value {
    Value::Number(thread_rng().gen_range(0.0..1.0))
}
