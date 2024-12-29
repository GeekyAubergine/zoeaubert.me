use std::{env::var, path::Path, process::Command};

use chrono::prelude::*;

fn main() {
    let build_date = make_build_date_string();

    println!("Build date: {}", build_date);

    build_data(&build_date);

    validate_tailwind_file();
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

fn validate_tailwind_file() {
    let tailwind_module_location = Path::new("./node_modules/.bin/tailwindcss");
    let input_css = Path::new("./assets/css/styles.css");

    let tw_command = format!(
        "ENVIRONMENT=production {} -i {}",
        tailwind_module_location.display(),
        input_css.display(),
    );
    let tailwind = Command::new("sh")
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
}
