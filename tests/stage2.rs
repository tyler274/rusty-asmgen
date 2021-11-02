extern crate asmgen;
mod common;

use std::path::Path; // Run programs

use common::run_test_process;

#[test]
fn one_plus_one() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage2-1-plus-1.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn one_plus_two_plus_three() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage2-1-plus-2-plus-3.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn one_plus_five() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage2-1-plus-5.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn five_plus_one() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage2-5-plus-1.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn big_stack() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage2-big-stack.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn overflow() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage2-overflow.bas");
    run_test_process(program_path)?;
    Ok(())
}
