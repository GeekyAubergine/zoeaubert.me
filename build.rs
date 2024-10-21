use std::{path::Path, process::Command};

use chrono::prelude::*;

const ASSETS_DIR: &str = "_assets";

fn main() {
    let build_date = make_build_date_string();

    build_data(&build_date);

    compile_assets(&build_date);
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
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("build_data.rs");

    let build_date = format!("pub const BUILD_DATE: &str = \"{}\";", build_date);

    let contents = format!(
        r#"
        {}
        "#,
        build_date
    );

    std::fs::write(dest_path, contents).unwrap();
}

fn compile_assets(build_date: &str) {
    // Clear _assets directory

    match std::fs::remove_dir_all(ASSETS_DIR) {
        Ok(_) => println!("Removed assets directory"),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("No assets directory to remove");
            } else {
                println!("Failed to remove assets directory: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Make _assets directory
    match std::fs::create_dir_all(ASSETS_DIR) {
        Ok(_) => println!("Created assets directory"),
        Err(e) => {
            println!("Failed to create assets directory: {}", e);
            std::process::exit(1);
        }
    }

    compile_css(build_date);
}

fn compile_css(build_date: &str) {
    Command::new("mkdir")
        .arg("-p")
        .arg("_assets/css")
        .output()
        .expect("Failed to create _assets/css directory");

    let tailwind_module_location = Path::new("./node_modules/.bin/tailwindcss");
    let input_css = Path::new("./assets/css/styles.css");
    let intermediate_css = Path::new("./_assets/css/tw.css");
    let output_file_name = format!("./_assets/css/styles-{}.css", build_date);
    let output_css = Path::new(&output_file_name);

    let tw_command = format!(
        "ENVIRONMENT=production {} -i {} -o {} --postcss",
        tailwind_module_location.display(),
        input_css.display(),
        intermediate_css.display()
    );
    // let lightning_command = format!("{} --minify --bundle --targets '>= 0.25%' {} -o {}", LIGHTNING_CSS_COMMAND, intermediate_css.display(), output_css.display());
    let lightning_command = format!("cp {} {}", intermediate_css.display(), output_css.display());

    // Compile the CSS
    println!("Compiling CSS...");
    let tailwind = std::process::Command::new("sh")
        .arg("-c")
        .arg(tw_command)
        .output()
        .expect("Failed to compile Tailwind CSS");
    if !tailwind.status.success() {
        println!("Failed to compile Tailwind CSS");
        println!("stdout: {}", String::from_utf8_lossy(&tailwind.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&tailwind.stderr));
        std::process::exit(1);
    }

    let lightning = std::process::Command::new("sh")
        .arg("-c")
        .arg(lightning_command)
        .output()
        .expect("Failed to compile Lightning CSS");
    if !lightning.status.success() {
        println!("Failed to compile Lightning CSS");
        println!("stdout: {}", String::from_utf8_lossy(&lightning.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&lightning.stderr));
        std::process::exit(1);
    }
    println!("CSS compiled successfully");
}
