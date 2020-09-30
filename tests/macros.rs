#![feature(asm)]
use iaca_marker_macros::{iaca_end_marker, iaca_start_marker};
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(test)]
#[test]
fn test_macro() -> Result<(), Box<dyn Error>> {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH)?.subsec_nanos();

    iaca_start_marker!();
    // emit a loop so we can manually verify this with iaca
    let sum = (0..nanos as i128).sum::<i128>();
    iaca_end_marker!();

    println!("{}", sum);

    assert!(sum >= nanos as i128);

    Ok(())
}
