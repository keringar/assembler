use std::time::{Duration, Instant};

// Time resource, updated every frame
pub struct Time {
    // Elapsed time since last frame
    pub delta_time: Duration,
    // Rate at which fixed updates are called
    pub fixed_step: Duration,
    // Time of the last fixed update
    pub last_fixed_update: Instant,
}

impl Time {
    pub fn new(tps: u32) -> Time {
        // 1 second equals 10^9 nanoseconds
        let nanos_per_tick = 1000000000 / tps;

        Time {
            delta_time: Duration::new(0, 0),
            fixed_step: Duration::new(0, nanos_per_tick),
            last_fixed_update: Instant::now(),
        }
    }
}

