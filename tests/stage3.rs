extern crate asmgen;
mod common;

use common::run_test_process;
use std::path::Path;

#[test]
fn bad_order() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage3-bad-order.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn lots_of_ops() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage3-lots-of-ops.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn overflow() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage3-overflow.bas");
    run_test_process(program_path)?;
    Ok(())
}
