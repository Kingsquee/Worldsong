use std::old_io;
use std::old_io::fs::PathExtensions;
use std::old_io::process::StdioContainer;
use std::old_path::Path;
use std::old_path::GenericPath;

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
    let mut command = old_io::Command::new(app.clone());
    if args.is_some() {
        for arg in args.unwrap().iter() {
            command.arg(arg);
        }
    }
    command.cwd(&app.dir_path());
    execute_command(&mut command);
}

pub fn execute_command(command: &mut old_io::Command) {
    // Try to run this thing
    command.stdout(StdioContainer::InheritFd(1));
    command.stderr(StdioContainer::InheritFd(2));
    let mut result = match command.spawn() {
        Ok(r) => r,
        Err(e) => panic!("Failed to run: {}", e),
    };

    // If it ran, how'd it do?

    if !result.wait().unwrap().success() {
        panic!("Build failed");
    };
}