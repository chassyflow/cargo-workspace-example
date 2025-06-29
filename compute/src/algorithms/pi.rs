use std::{thread::sleep, time::Duration};

use chudnovsky::chudnovsky;
use tracing::info;

pub fn compute(max_n: usize) {
    let mut i = 2;
    let mut pi = chudnovsky(i);
    info!(
        "Computed chunk: ({} digits) {}",
        pi.digits(),
        pi.to_string()
    );
    while pi.digits() < max_n {
        pi = chudnovsky(i + 1);
        info!("Computed chunk({} digits) {}", pi.digits(), pi.to_string());
        i += 1;
        sleep(Duration::from_secs(3))
    }
}
