use std::time::{Duration, SystemTime};

const _FOO : i32 = 10;

/// Magic time is 10 seconds after the unix epoch, and will be used as the edit time of all files created in Regolith
// TODO: Look into making this a const somehow.
pub fn magic_time() -> SystemTime{
	SystemTime::UNIX_EPOCH + Duration::new(10, 0)
}