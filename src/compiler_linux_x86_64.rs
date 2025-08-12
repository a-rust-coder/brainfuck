use std::{
    fs::{self, remove_file, File},
    io::Write,
    process::{Command, Stdio},
};

fn compile(bf: &[u8]) -> String {
    let mut asm = String::new();
    let mut loop_stack = Vec::new();
    let mut loop_counter = 0;

    asm.push_str("section .bss\nmem resb 30000\n");
    asm.push_str("section .text\nglobal _start\n_start:\n");
    asm.push_str("    mov rbx, mem\n");

    for c in bf {
        match c {
            b'>' => asm.push_str("    add rbx, 1\n"),
            b'<' => asm.push_str("    sub rbx, 1\n"),
            b'+' => asm.push_str("    inc byte [rbx]\n"),
            b'-' => asm.push_str("    dec byte [rbx]\n"),
            b'.' => asm.push_str(
                "    mov rax, 1\n    mov rdi, 1\n    mov rsi, rbx\n    mov rdx, 1\n    syscall\n",
            ),
            b',' => asm.push_str(
                "    mov rax, 0\n    mov rdi, 0\n    mov rsi, rbx\n    mov rdx, 1\n    syscall\n",
            ),
            b'[' => {
                let id = loop_counter;
                loop_counter += 1;
                loop_stack.push(id);
                asm.push_str(&format!(
                    "loop_start_{}:\n    cmp byte [rbx], 0\n    je loop_end_{}\n",
                    id, id
                ));
            }
            b']' => {
                let id = loop_stack.pop().unwrap();
                asm.push_str(&format!(
                    "    cmp byte [rbx], 0\n    jne loop_start_{}\nloop_end_{}:\n",
                    id, id
                ));
            }
            _ => {}
        }
    }

    asm.push_str("    mov rax, 60\n    xor rdi, rdi\n    syscall\n");
    asm
}

pub fn compiler(program: &[u8], out_file: &str, execute: bool) {
    let prog = compile(program);

    let mut f = File::create(out_file.to_owned() + ".asm.tmp").unwrap();
    f.write_all(prog.as_bytes()).unwrap();
    f.flush().unwrap();
    drop(f);

    Command::new("nasm")
        .args([
            "-felf64",
            &(out_file.to_owned() + ".asm.tmp"),
            "-o",
            &(out_file.to_owned() + ".o.tmp"),
        ])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Command::new("ld")
        .args([&(out_file.to_owned() + ".o.tmp"), "-o", out_file])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    remove_file(out_file.to_owned() + ".asm.tmp").unwrap();
    remove_file(out_file.to_owned() + ".o.tmp").unwrap();

    if execute {
        while let Err(_) = set_executable(out_file) {}

        enable_raw_mode().unwrap();

        Command::new(out_file)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        disable_raw_mode().unwrap();
    }
}

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn set_executable(path: &str) -> std::io::Result<()> {
    let metadata = fs::metadata(path)?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o755); // rwxr-xr-x
    fs::set_permissions(path, permissions)?;
    Ok(())
}
