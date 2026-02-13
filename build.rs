use std::{env::var, path::Path};

use chrono::prelude::*;

const OUTPUT_DIR: &str = "./_assets";
const TAILWIND_INPUT_FILE: &str = "./assets/css/styles.css";

fn main() {
    let build_date = make_build_date_string();

    println!("Build date: {}", build_date);

    build_data(&build_date);

    compile_css(&build_date);
}

fn make_build_date_string() -> String {
    let now = Local::now();
    format!(
        "{}{:02}{:02}{:02}{:02}{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    )
}

fn build_data(build_date: &str) {
    let out_dir = var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_data.rs");

    let build_date = format!("pub const BUILD_DATE: &str = \"{}\";", build_date);

    let contents = format!(
        r#"
        {}
        "#,
        build_date
    );

    std::fs::write(dest_path, contents).unwrap();
}

fn compile_css(build_date: &str) {
    match std::fs::create_dir_all(Path::new(OUTPUT_DIR)) {
        Ok(_) => println!("Created output directory"),
        Err(e) => {
            println!("Failed to create output directory: {}", e);
            std::process::exit(1);
        }
    }

    compile_tailwind(
        TAILWIND_INPUT_FILE,
        &format!("./_assets/css/styles-{}.css", build_date),
    );
}

fn compile_tailwind(input: &str, output: &str) {
    println!("Compiling Tailwind CSS");
    let input = Path::new(input);
    let output = Path::new(output);

    let command = format!(
        "ENVIRONMENT=production ENVIRONMENT=production npx @tailwindcss/cli -i {} -o {} -m",
        input.display(),
        output.display()
    );

    match std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
    {
        Ok(output) => match output.status.success() {
            true => {
                println!("Successfully compiled Tailwind CSS");
                println!(
                    "--------\n{}\n--------",
                    String::from_utf8_lossy(&output.stdout)
                );
            }
            false => {
                println!("Failed to compile Tailwind CSS");
                println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }
        },
        Err(e) => {
            println!("Failed to compile Tailwind CSS");
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
