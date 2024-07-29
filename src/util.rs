#[macro_export]
macro_rules! async_synchronized {
    () => {
        lazy_static::lazy_static! {
            static ref LOCK: futures::lock::Mutex<i32> = futures::lock::Mutex::new(0);
        }
        let _guard = LOCK.lock().await;
    };
}
