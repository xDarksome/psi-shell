use chrono::Timelike;

pub struct Clock {
    pub hour: u8,
    pub minute: u8,
}

impl Clock {
    pub fn now() -> Self {
        let dt = chrono::Local::now();
        Self { hour: dt.hour() as u8, minute: dt.minute() as u8 }
    }
}
