extern crate getopts;
extern crate worldsong_hierarchy;
extern crate system;
extern crate walker;

use std::env;
use std::fs;
use getopts::Options;
use walker::Walker;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {

    // Program args
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let opts = Options::new();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.free.is_empty() {
        print_usage(&program, opts);
        return
    }

    let utensils_dir = worldsong_hierarchy::get_worldsong_root_dir().join("utensils");
    let source_dir = worldsong_hierarchy::get_module_src_dir(&utensils_dir, "add_project");
    //println!("source dir: {}", &source_dir.display());

    let default_project_name = String::from("defaultproject");
    let default_project_dir = source_dir.join(&default_project_name);
    //println!("default project dir: {}", default_project_dir.display());

    let new_project_name = matches.free[0].clone();
    let new_project_dir = worldsong_hierarchy::get_project_dir(&new_project_name);
    //println!("new project dir: {}", new_project_dir.display());

    // copy ./defaultproject to projects_dir/{name}
    for entry in Walker::new(&default_project_dir).unwrap() {
        let source_path = entry.unwrap().path();
        //println!("source path is {}", &source_path.display());
        let diff = source_path.to_str().unwrap().trim_left_matches(&format!("{}{}", default_project_dir.to_str().unwrap(), "/")).to_string();
        //println!("diff from source_dir is {}", &diff);
        let target_path = new_project_dir.join(&diff);

        if fs::metadata(&source_path).unwrap().is_file() {
            //println!("Copying from {} to {}", &source_path.display(), &target_path.display());
            fs::copy(&source_path, &target_path).unwrap();
        } else {
            //println!("Creating directory: {}", &target_path.display());
            fs::create_dir_all(&target_path).unwrap();
        }
    }

    // distribute soft links to projects_dir/{name}
    system::distribute_utensils(&utensils_dir, &new_project_dir);

    // compile it
    let compile_path = new_project_dir.join("compile");
    system::run(&compile_path, None);
}