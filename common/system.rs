use unicode::str::UnicodeStr;
use std::io::Read;
use std::path::{Path};
use std::process::{Command, Stdio};
//use std::env;
//use std::fs;
//use std::fs::PathExt;
//use regex::Regex;

use std::fs::File;
use hierarchy;
use settings;

/*
[Sunday, November 30, 2014] [12:28:23 ▾] <Kingsqueee>   Is there a way I can tell rustup.sh to install to a local directory?
[Sunday, November 30, 2014] [12:28:47 ▾] <Kingsqueee>   I'd like a 'portable' compiler
[Sunday, November 30, 2014] [12:29:08 ▾] <geofft>   Kingsqueee: yeah, it takes --prefix
[Sunday, November 30, 2014] [12:29:36 ▾] <geofft>   I run rustup with --prefix=/home/geofft/b because I don't like installing stuff globally
[Sunday, November 30, 2014] [12:29:50 ▾] <geofft>   so I have to export PATH=$PATH:/home/geofft/b/bin and export LD_LIBRARY_PATH=/home/geofft/b/lib
[Sunday, November 30, 2014] [12:29:53 ▾] <geofft>   and it works
[Sunday, November 30, 2014] [12:31:00 ▾] <Kingsqueee>   geofft: awesome!
*/

pub fn run(app: &Path, args: Option<Vec<&str>>) {
    println!("Running {}", app.display());
    let mut command = Command::new(app.clone());
    if args.is_some() {
        for arg in args.unwrap().iter() {
            command.arg(arg);
        }
    }
    command.current_dir(&app.parent().unwrap());
    execute_command(&mut command);
}

pub fn execute_command(command: &mut Command) {
    // Try to run this thing
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());
    let mut result = match command.spawn() {
        Ok(r) => r,
        Err(e) => panic!("Failed to run: {}", e),
    };

    // If it ran, how'd it do?

    if !result.wait().unwrap().success() {
        panic!("Build failed");
    };
}

//TODO: Move these string configs to settings
pub fn get_compile_config(command: &mut Command, current_dir: &Path, source_filename: &str, target_dir: &Path) -> String {

    let mut config_display = String::new();
    match File::open(&Path::new(&hierarchy::get_compile_config(&current_dir))) {
        Err(_) => {
            command.args(&settings::get_default_rustc_flags());
        }
        Ok(ref mut file) => {
            let mut file_contents = String::new();
            match file.read_to_string(&mut file_contents) {
                Err(_) => {
                    command.args(&settings::get_default_rustc_flags());
                }
                Ok(_) => {
                    if file_contents.is_empty() || file_contents.is_whitespace() {
                        command.args(&settings::get_default_rustc_flags());
                    } else {
                        config_display.push_str("with \"");
                        for line in file_contents.lines_any() {
                            config_display.push_str(line);
                            config_display.push_str(" ");

                            for cmd in line.split(' ') {
                                command.arg(cmd);
                            }
                        }
                        config_display.push_str("\"");
                    }
                }
            }
        }
    }

    command.arg("--out-dir").arg(target_dir.as_os_str().to_str().unwrap());
    command.arg("-C").arg("prefer-dynamic");
    command.arg(source_filename);

    config_display
}

pub fn link_libraries(command: &mut Command, lib_dir: &Path){
        //println!("Linking dir {:?}", lib_dir);
        let s = format!("{}", lib_dir.as_os_str().to_str().unwrap());
        command.arg("-L").arg(s);

        // The following regex matches the library filenames output by cargo and rustc
        // It's been tested against these filenames:

        /*      librand-7b0a3af7ae4685dc.rlib
                libcommon-314a083a5b53d6a9.so
                liblazy_static-2622010a235b638a.rlib
                libfixed_update.so
        */
/*
        let s = format!(r"(?:{})([a-zA-Z_]*)(?:-\w*)?.(?:\w*)", env::consts::DLL_PREFIX);
        let re = Regex::new(&s).unwrap();

        //println!("Accessing the dir");
        for entry in fs::read_dir(lib_dir).unwrap() {
            //println!("Accessing path for entry");
            let entry = entry.unwrap().path();
            //println!("Entry path is {:?}", entry);
            if entry.is_file() {

                //println!("Accessing regex captures");
                let caps = re.captures(entry.as_os_str().to_str().unwrap()).unwrap();
                //println!("Accessing name");
                let mut name = caps.at(1).unwrap();

                // Blacklist
                if name == "rand" { name = "extern_rand" }

                //println!("name: {}", name);
                println!("Linked {}", name);

                command.arg("--extern").arg(format!("{}={}", name, entry.as_os_str().to_str().unwrap()));
            }
        }*/
        //println!("{:?}", command)
}