extern crate asmgen;
mod common;

use std::path::Path; // Run programs

use common::run_test_process;

#[test]
fn comparisons() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage6-comparisons.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn nested() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage6-nested.bas");
    run_test_process(program_path)?;
    Ok(())
}
