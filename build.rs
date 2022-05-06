use std::path::{PathBuf, Path};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    {
        // Build Serum Labs WebApp
        println!("cargo:rerun-if-changed=labs/src");
        println!("cargo:info=Building Serum Labs WebApp");
        execute_npm(&["run", "build"], "labs/");
    }
}

fn resolve_path<P>(path: P) -> Option<PathBuf>
    where P: AsRef<Path>,
{
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths).filter_map(|dir| {
            let full_path = dir.join(&path);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        }).next()
    })
}

fn execute_npm(args: &[&str], cwd: &str) {
    use std::process::Command;
    
    let mut npm = {
        let npm_path = resolve_path("npm.cmd").expect("npm not found");
        Command::new(npm_path)
    };

    let output = npm.current_dir(cwd)
        .args(args)
        .output()
        .expect("Failed to run npm");

    
    println!("cargo:info={}", String::from_utf8_lossy(&output.stdout));

    if !output.status.success() {
        panic!("{}", String::from_utf8_lossy(&output.stderr));
    }
}