extern crate asmgen;
mod common;

use common::run_test_process;
use std::path::Path;

#[test]
fn basic_division() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage4-basic-division.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn lots_of_ops() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage4-lots-of-ops.bas");
    run_test_process(program_path)?;
    Ok(())
}
