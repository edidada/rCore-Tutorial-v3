use std::fs::{read_dir, File};
use std::io::{Result, Write};

fn main() {
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    insert_app_data().unwrap();
}

static TARGET_PATH: &str = "../user/build/bin/";

fn insert_app_data() -> Result<()> {
    let mut f = File::create("src/link_app.S").unwrap();
    let mut apps: Vec<_> = read_dir("../user/build/bin/")
        .unwrap()
        .into_iter()
        .map(|dir_entry| {
            let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
            name_with_ext
        })
        .collect();
    apps.sort();

    writeln!(
        f,
        r#"
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {}"#,
        apps.len()
    )?;

    for i in 0..apps.len() {
        writeln!(f, r#"    .quad app_{}_start"#, i)?;
    }
    writeln!(f, r#"    .quad app_{}_end"#, apps.len() - 1)?;

    writeln!(f, ".global _app_memory_end\n_app_memory_end:")?;
    for app in &apps {
        let elf = std::fs::read(format!("../user/build/elf/{}.elf", app))?;
        let word = |offset: usize| -> usize {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&elf[offset..offset + 8]);
            u64::from_le_bytes(bytes) as usize
        };
        let phoff = word(32);
        let phsize = u16::from_le_bytes([elf[54], elf[55]]) as usize;
        let phnum = u16::from_le_bytes([elf[56], elf[57]]) as usize;
        let mut end = 0;
        for index in 0..phnum {
            let offset = phoff + index * phsize;
            if elf[offset..offset + 4] == [1, 0, 0, 0] {
                end = end.max(word(offset + 16) + word(offset + 40));
            }
        }
        writeln!(f, "    .quad {}", end)?;
    }

    for (idx, app) in apps.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(
            f,
            r#"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "{2}{1}.bin"
app_{0}_end:"#,
            idx, app, TARGET_PATH
        )?;
    }
    Ok(())
}
