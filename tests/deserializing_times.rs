use chrono::{DateTime, Utc};
use magnus::{eval, Error, Time};
use serde_magnus::deserialize;

#[test]
fn test_deserializing_times() -> Result<(), Error> {
    let _cleanup = unsafe { magnus::embed::init() };

    let time = Time::from_value(eval!("Time.now")?).unwrap();
    let string: String = deserialize(time)?;
    let _output: DateTime<Utc> = string.parse().unwrap();

    Ok(())
}
