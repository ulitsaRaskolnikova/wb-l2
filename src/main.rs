use std::{
    env, io::{stdin, stdout, Write, Error}, os::unix::process::CommandExt, path::Path, process::{Child, Command, Stdio, exit}, str::FromStr
};

use fork::{daemon, Fork};


#[derive(PartialEq, Eq)]
enum ExecutingMode {
    Default,
    Exec,
    Fork
}

impl FromStr for ExecutingMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(ExecutingMode::Default),
            "exec" => Ok(ExecutingMode::Exec),
            "fork" => Ok(ExecutingMode::Fork),
            _ => Err(format!("Unknown ExecutingMode: {}", s))
        }
    }
}

// if you want to execute command in fork or exec mode type exec or fork before it like 'exec <command>'
fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        print!("> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.is_empty() {
            eprintln!("Empty!");
            continue;
        }

        let mut commands = input.split(" | ").peekable();
        let mut previous_command: Option<Child> = None;

        while let Some(command) = commands.next() {

            let mut parts = command.trim().split_whitespace();
            let mode_or_command = parts.next().unwrap();
            let mode = ExecutingMode::from_str(mode_or_command).unwrap_or(ExecutingMode::Default);
            let command = if mode == ExecutingMode::Default {
                mode_or_command
            } else {
                parts.next().unwrap()
            };
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek()
                        .map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                },
                "\\quit" => return Ok(()),
                command => {
                    let stdin = previous_command
                        .as_mut()
                        .and_then(|output| output.stdout.take())
                        .map_or(Stdio::inherit(), |stdout| Stdio::from(stdout));
                

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    match mode {
                        ExecutingMode::Default => {
                            let output = Command::new(command)
                                .args(args)
                                .stdin(stdin)
                                .stdout(stdout)
                                .spawn();
                            match output {
                                Ok(output) => { previous_command = Some(output); },
                                Err(e) => {
                                    previous_command = None;
                                    eprintln!("{}", e);
                                },
                            };
                        },
                        ExecutingMode::Exec => {
                            let output = Command::new(command)
                                .args(args)
                                .stdin(stdin)
                                .stdout(stdout)
                                .exec();
                            eprintln!("{}", output);
                        },
                        ExecutingMode::Fork => {
                            match daemon(false, false) {
                                Ok(Fork::Parent(_)) => {
                                    exit(0);
                                }
                                Ok(Fork::Child) => {
                                    let output = Command::new(command)
                                        .args(args)
                                        .stdin(stdin)
                                        .stdout(stdout)
                                        .output();
                                    match output {
                                        Ok(output) => {
                                            println!("{:?}", output);
                                        },
                                        Err(e) => {
                                            eprintln!("{}", e);
                                        }
                                    }
                                },
                                Err(_e) => {
                                    eprint!("{}", Error::new(std::io::ErrorKind::Other, "Failed to fork"));
                                },
                            };
                        }
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            final_command.wait()?;
        }
    }
}
