use chrono::prelude::*;

const ASSETS_DIR: &str = "_assets";

const TAILWIND_COMMAND: &str = "ENVIRONMENT=production node_modules/.bin/tailwindcss -i assets/css/styles.css -o _assets/css/tw.css --postcss";
// const LIGHTNING_CSS_COMMAND: &str = "node_modules/.bin/lightningcss --minify --bundle --targets '>= 0.25%' _assets/css/tw.css -o _assets/css/styles";
const LIGHTNING_CSS_COMMAND: &str = "cp _assets/css/tw.css _assets/css/styles";

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

    // match std::fs::remove_dir_all(ASSETS_DIR) {
    //     Ok(_) => println!("Removed assets directory"),
    //     Err(e) => {
    //         if e.kind() == std::io::ErrorKind::NotFound {
    //             println!("No assets directory to remove");
    //         } else {
    //             println!("Failed to remove assets directory: {}", e);
    //             std::process::exit(1);
    //         }
    //     }
    // }

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
    // Compile the CSS
    println!("Compiling CSS...");
    let tailwind = std::process::Command::new("sh")
        .arg("-c")
        .arg(TAILWIND_COMMAND)
        .output()
        .expect("Failed to compile Tailwind CSS");
    if !tailwind.status.success() {
        println!("Failed to compile Tailwind CSS");
        println!("stdout: {}", String::from_utf8_lossy(&tailwind.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&tailwind.stderr));
        std::process::exit(1);
    }

    let command = format!("{}-{}.css", LIGHTNING_CSS_COMMAND, build_date);

    println!("Running command: {}", command);

    let lightning = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
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
