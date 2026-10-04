//! The clock and the randomness of a `host` build (feature `host`): on
//! wasm32 without JavaScript, the embedder gives them. Until it does, the
//! clock reads the Unix epoch and the randomness is a fixed sequence.

use std::sync::Mutex;

static CLOCK: Mutex<Option<fn() -> i64>> = Mutex::new(None);
static RANDOM: Mutex<Option<fn() -> f64>> = Mutex::new(None);
static SEED: Mutex<u64> = Mutex::new(0x9e37_79b9_7f4a_7c15);

/// Sets the clock: milliseconds since January 1, 1970, UTC.
pub fn set_clock(f: fn() -> i64) {
    if let Ok(mut c) = CLOCK.lock() {
        *c = Some(f);
    }
}

/// Sets the randomness: a number in [0, 1).
pub fn set_random(f: fn() -> f64) {
    if let Ok(mut r) = RANDOM.lock() {
        *r = Some(f);
    }
}

#[allow(dead_code)]
pub(crate) fn milliseconds_since_epoch() -> i64 {
    CLOCK.lock().ok().and_then(|c| *c).map_or(0, |f| f())
}

#[allow(dead_code)]
pub(crate) fn random() -> f64 {
    if let Some(f) = RANDOM.lock().ok().and_then(|r| *r) {
        return f();
    }
    // xorshift64*, the top 53 bits as a fraction.
    let mut s = SEED.lock().unwrap_or_else(|e| e.into_inner());
    *s ^= *s >> 12;
    *s ^= *s << 25;
    *s ^= *s >> 27;
    let x = s.wrapping_mul(0x2545_f491_4f6c_dd1d);
    (x >> 11) as f64 / (1u64 << 53) as f64
}
