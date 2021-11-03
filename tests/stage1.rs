extern crate asmgen;
mod common;

use std::path::Path;

use common::run_test_process;

#[test]
fn leet() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("./progs/stage1-1337.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn forty_two() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("./progs/stage1-42.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn print_multiple() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("./progs/stage1-print-multiple.bas");
    run_test_process(program_path)?;
    Ok(())
}
