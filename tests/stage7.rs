extern crate asmgen;
mod common;

use std::path::Path; // Run programs

use common::run_test_process;

#[test]
fn bswap() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-bswap.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn count_to_20() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-count-to-20.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn count_together() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-count-together.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn count_up_down() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-count-up-down.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn digit_powers() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-digit-powers.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn double_palindromes() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-double-palindromes.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn exponentiation() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-exponentiation.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn fizz_buzz() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-fizz-buzz.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn lcm() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-lcm.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn loops_of_ops() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-loops-of-ops.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn pascals_triangle() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-pascals-triangle.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn pi_approx() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-pi-approx.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn pi_exact() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-pi-exact.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn primes() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-primes.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn riemann_sum() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-riemann-sum.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn t_sin() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-sin.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn t_sqrt() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-sqrt.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn square_digits() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-square-digits.bas");
    run_test_process(program_path)?;
    Ok(())
}

#[test]
fn unhash() -> Result<(), Box<dyn std::error::Error>> {
    let program_path = Path::new("progs/stage7-unhash.bas");
    run_test_process(program_path)?;
    Ok(())
}
