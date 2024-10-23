use std::time::SystemTime;

extern crate nsrb;
 
 #[derive(Clone, Copy, Debug)]
 pub struct LogEntry {
     pub time_date : SystemTime,
     pub entry : [u8;256]
 }
 
 impl Default for LogEntry {
    fn default() -> Self {
        Self { time_date: SystemTime::now(), entry: [Default::default();256] }
    }
 }
 
 nsrb::ring!(#[derive(Debug)] pub(crate) LogRB[LogEntry; 10]);

 #[test]
 fn ring_lower_limit() {
    let mut log = LogRB::new();
    log.push(LogEntry { time_date: SystemTime::now(), entry: [Default::default();256] });
 }