use std::{env::var, path::Path, process::Command};

use chrono::prelude::*;

const OUTPUT_DIR: &str = "./_assets";
const TAILWIND_INPUT_FILE: &str = "./assets/css/styles.css";
const TAILWIND_INTERMEDIATE_FILE: &str = "./_assets/styles-tmp.css";

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

    compile_tailwind(TAILWIND_INPUT_FILE, Some(TAILWIND_INTERMEDIATE_FILE));
    run_lightning(
        TAILWIND_INTERMEDIATE_FILE,
        &format!("./_assets/css/styles-{}.css", build_date),
    );

    match std::fs::remove_file(Path::new(TAILWIND_INTERMEDIATE_FILE)) {
        Ok(_) => println!("Removed intermediate Tailwind CSS file"),
        Err(e) => {
            println!("Failed to remove intermediate Tailwind CSS file: {}", e);
            std::process::exit(1);
        }
    }
}

fn compile_tailwind(input: &str, output: Option<&str>) {
    println!("Compiling Tailwind CSS");
    let input = Path::new(input);
    let output = output.map(Path::new);
    let tailwind_module_location = Path::new("./node_modules/.bin/tailwindcss");

    let mut command = format!(
        "ENVIRONMENT=production {} -i {}",
        tailwind_module_location.display(),
        input.display(),
    );

    if let Some(output) = output {
        command = format!("{} -o {}", command, output.display());
    }

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

fn run_lightning(input: &str, output: &str) {
    println!("Running Lightning CSS");
    let command = format!(
        "./node_modules/.bin/lightningcss --minify --bundle --targets '>= 0.25%' {} -o {}",
        input, output
    );

    match std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
    {
        Ok(output) => match output.status.success() {
            true => {
                println!("Successfully compiled Lightning CSS");
                println!(
                    "--------\n{}\n--------",
                    String::from_utf8_lossy(&output.stdout)
                );
            }
            false => {
                println!("Failed to compile Lightning CSS");
                println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                std::process::exit(1);
            }
        },
        Err(e) => {
            println!("Failed to compile Lightning CSS");
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
