use std::process::Command;

pub fn compile_and_link(path: &str) {
    let mut nasm = Command::new("nasm");
    nasm.arg("-felf64");
    nasm.arg("-o");
    nasm.arg(format!("{path}.o"));
    nasm.arg(format!("{path}.asm"));
    nasm.status().unwrap();
    let mut ld = Command::new("ld");
    ld.arg("-o");
    ld.arg(format!("{path}"));
    ld.arg(format!("{path}.o"));
    ld.status().unwrap();
}

pub fn remove_file_extension(path: &str) -> String {
    let dot = path
        .char_indices()
        .find_map(|(index, char)| {
            if char == '.' {
                return Some(index);
            } else {
                return None;
            }
        })
        .unwrap_or_else(|| path.len());

     path[..dot].to_string()
}
