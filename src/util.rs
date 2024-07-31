use serde::{Deserialize, Deserializer};

#[macro_export]
macro_rules! async_synchronized {
    () => {
        lazy_static::lazy_static! {
            static ref LOCK: futures::lock::Mutex<i32> = futures::lock::Mutex::new(0);
        }
        let _guard = LOCK.lock().await;
    };
}

pub fn opt_csv_deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt_str_sequence = Option::<String>::deserialize(deserializer)?;
    match opt_str_sequence {
        Some(str_sequence) => Ok(Some(str_sequence.split(',').map(|item| item.trim().to_owned()).collect())),
        None => Ok(None),
    }
}
