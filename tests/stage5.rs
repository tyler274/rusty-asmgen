extern crate asmgen;
mod common;

use common::run_test_process;
use std::path::Path;

#[test]
fn big_locals() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage5-big-locals.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn local_test() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage5-local-test.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn local_test2() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage5-local-test2.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn lots_of_ops() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage5-lots-of-ops.bas");
    run_test_process(program_path)?;
    Ok(())
}
