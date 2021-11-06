use std::{fs::File, io::Write, os::unix::prelude::MetadataExt, path::Path};

use assert_cmd::Command;

pub fn get_output_name(program_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let output_name = program_path.file_stem().unwrap().to_str().unwrap();
    Ok(output_name.to_string())
}

pub fn get_output_asm_path(program_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let output_name = get_output_name(program_path)?;
    let output_asm_path = "./out/".to_owned() + &output_name + ".s";
    Ok(output_asm_path.to_string())
}

pub fn get_asm_bin_path(program_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let output_bin_path = "bin/".to_owned() + &get_output_name(program_path)?;
    Ok(output_bin_path.to_string())
}

pub fn write_assembly(program_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut asmgen_compilation = Command::cargo_bin("asmgen")?;
    // pass the args to the command, then run, capturing its stdout and stderr outputs
    asmgen_compilation.arg(program_path);

    let output = asmgen_compilation.assert().get_output().clone();
    if !output.status.success() {
        eprint!(
            "Failed to compile AST to ASM: {} \nWritten x86 ASM: \n{}\nParsed AST: \n{}\n",
            get_output_asm_path(program_path)?,
            std::str::from_utf8(&output.stdout)?,
            std::str::from_utf8(&output.stderr)?
        );
    }

    let output_asm_path = get_output_asm_path(program_path)?;

    let mut output_asm = File::create(output_asm_path.clone())?;

    let output_result = output_asm.write(output.stdout.as_slice())?;
    assert_eq!(output_result, output.stdout.len());
    assert_eq!(output_asm.metadata()?.size() as usize, output.stdout.len());

    Ok(())
}

pub fn clang_compile(program_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // assert_eq!(output_name, "stage1-1337");
    let asm_path = get_output_asm_path(program_path)?;
    let asm_bin_path = get_asm_bin_path(program_path)?;
    let mut clang_asm = Command::new("clang");
    clang_asm
        .arg("-g")
        .arg("-nostartfiles")
        .arg(asm_path)
        .arg("./runtime/print_int.s")
        .arg("./runtime/call_check.s")
        .arg("-o")
        .arg(asm_bin_path.clone());

    clang_asm.assert().success();

    Ok(())
}

pub fn record_asm_bin_output(program_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut output_cmd = Command::new(get_asm_bin_path(program_path)?);
    let output_record = output_cmd.assert().get_output().clone();

    if !output_record.status.success() {
        eprintln!(
            "FAILED running output cmd with binary path: {}",
            get_asm_bin_path(program_path)?
        );
    }

    // let output_path = "progs/".to_owned() + &get_output_name(program_path)? + "-actual.txt";
    // let mut output_actual = File::create(output_path.clone())?;
    // let output_actual_write_result = output_actual.write(output_record.stdout.as_slice())?;
    // assert_eq!(output_actual_write_result, output_record.stdout.len());

    Ok(())
}
