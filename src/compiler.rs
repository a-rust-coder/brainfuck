use std::{
    fs::File,
    io::Write,
    process::{Command, Stdio},
};

fn compile_bf_to_asm(bf: &[u8]) -> String {
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
    let prog = compile_bf_to_asm(program);

    File::create(out_file.to_owned() + ".asm")
        .unwrap()
        .write_all(prog.as_bytes())
        .unwrap();

    Command::new("nasm")
        .args([
            "-felf64",
            &(out_file.to_owned() + ".asm"),
            "-o",
            &(out_file.to_owned() + ".o"),
        ])
        .spawn()
        .unwrap();
    Command::new("ld")
        .args([&(out_file.to_owned() + ".o"), "-o", out_file])
        .spawn()
        .unwrap();

    if execute {
        let mut child = Command::new("./".to_owned() + out_file)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdin(Stdio::inherit())
            .spawn()
            .unwrap();
        child.wait().unwrap();
    }
}
