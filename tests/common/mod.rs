use core::fmt;
use std::{
    fs::{read, File},
    io::Write,
    path::Path,
    str::from_utf8,
};

use assert_cmd::Command;
use console::{style, Style};
use similar::{ChangeTag, TextDiff};

// https://github.com/mitsuhiko/similar/blob/main/examples/terminal-inline.rs
pub struct Line(pub Option<usize>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:<4}", idx + 1),
        }
    }
}

pub fn get_output_name(program_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let output_name = program_path.file_stem().unwrap().to_str().unwrap();
    Ok(output_name.to_string())
}

pub fn get_output_asm_path(program_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let output_name = get_output_name(program_path)?;
    let output_asm_path = "./out/".to_owned() + &output_name + ".s";
    Ok(output_asm_path.to_string())
}

pub fn get_output_path(program_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let output_path = "progs/".to_owned() + &get_output_name(program_path)? + "-actual.txt";
    Ok(output_path.to_string())
}

pub fn get_output_bin_path(program_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let output_bin_path = "bin/".to_owned() + &get_output_name(program_path)?;
    Ok(output_bin_path.to_string())
}

pub fn get_expected_path(program_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let expected_path = "./progs/".to_owned() + &get_output_name(program_path)? + "-expected.txt";
    Ok(expected_path)
}

pub fn make_expected(program_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let expected_path = get_expected_path(program_path)?;
    let mut expected_file = File::create(expected_path.clone())?;
    let program = &read(program_path)?;
    let program_comments = from_utf8(program)?.lines();
    for line in program_comments {
        if line.starts_with("#") {
            expected_file.write((line.split("#").last().unwrap().to_owned() + "\n").as_bytes())?;
        }
    }
    Ok(())
}

pub fn write_output(program_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut asmgen_compilation = Command::cargo_bin("asmgen")?;
    // pass the args to the command, then run, capturing its stdout and stderr outputs
    asmgen_compilation.arg(program_path);
    let output = asmgen_compilation.assert().success().get_output().clone();

    eprint!("{}", from_utf8(&output.stderr)?);
    let output_asm_path = get_output_asm_path(program_path)?;

    let mut output_asm = File::create(output_asm_path.clone())?;

    let output_result = output_asm.write(output.stdout.as_slice())?;
    assert_eq!(output_result, output.stdout.len());

    Ok(())
}

pub fn clang_compile(program_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // assert_eq!(output_name, "stage1-1337");
    let output_asm_path = get_output_asm_path(program_path)?;
    let output_bin_path = get_output_bin_path(program_path)?;
    let mut clang_asm = Command::new("clang");
    clang_asm
        .arg("-g")
        .arg("-nostartfiles")
        .arg(output_asm_path)
        .arg("./runtime/print_int.s")
        .arg("./runtime/call_check.s")
        .arg("-o")
        .arg(output_bin_path.clone());

    clang_asm.assert().success();

    Ok(())
}

pub fn record_output(program_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut output_cmd = Command::new(get_output_bin_path(program_path)?);
    eprint!(
        "running output cmd with binary path: {}\n",
        get_output_bin_path(program_path)?
    );
    let output_record = output_cmd.assert().success().get_output().clone();
    let output_path = "progs/".to_owned() + &get_output_name(program_path)? + "-actual.txt";
    let mut output_actual = File::create(output_path.clone())?;
    let output_actual_write_result = output_actual.write(output_record.stdout.as_slice())?;
    assert_eq!(output_actual_write_result, output_record.stdout.len());

    Ok(())
}

// https://github.com/mitsuhiko/similar/blob/main/examples/terminal-inline.rs
pub fn show_diff(program_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let expected = &read(get_expected_path(program_path)?).unwrap();
    let actual = &read(get_output_path(program_path)?).unwrap();
    let diff = TextDiff::from_lines(from_utf8(&expected)?, from_utf8(actual)?);

    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            eprintln!("{:-^1$}", "-", 80);
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, s) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new().dim()),
                };
                eprint!(
                    "{}{} |{}",
                    style(Line(change.old_index())).dim(),
                    style(Line(change.new_index())).dim(),
                    s.apply_to(sign).bold(),
                );
                for (emphasized, value) in change.iter_strings_lossy() {
                    if emphasized {
                        eprint!("{}", s.apply_to(value).underlined().on_black());
                    } else {
                        eprint!("{}", s.apply_to(value));
                    }
                }
                if change.missing_newline() {
                    eprintln!();
                }
            }
        }
    }

    // for scaffolding tests keep this commented out, then enable for the pain.
    assert!(diff.ratio() == 1.0);
    Ok(())
}

pub fn run_test_process(program_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    write_output(program_path)?;
    clang_compile(program_path)?;
    record_output(program_path)?;
    make_expected(program_path)?;
    show_diff(program_path)?;
    Ok(())
}
