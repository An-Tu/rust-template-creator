use std::env;
use std::env::Args;
use std::path::PathBuf;

fn check_arg_value (arg: &str, arg_value: Option<&String>) {
    if arg_value.is_none() {
        panic!("arg {} should have a value", arg);
    }
    if arg_value.unwrap().starts_with("--") {
        panic!("{} value can't starts with --", arg);
    }
}

#[derive(Debug)]
pub struct Config {
    pub path: PathBuf,
    pub name: String,
}

impl Config {
    pub fn prepare(args: Args) -> Config {
        let mut peekable_args = args.skip(1).peekable();
        let mut config = Self {
            path: env::current_dir().unwrap(),
            name: "".to_string(),
        };
        while let Some(arg) = peekable_args.next() {
            let next_arg = peekable_args.peek();
            match arg.as_str() {
                "--name" => {
                    check_arg_value("--name", next_arg);
                    config.name = next_arg.unwrap().to_string();
                }
                "--path" => {
                    check_arg_value("--path", next_arg);
                    let path = PathBuf::from(next_arg.unwrap());
                    if path.is_absolute() {
                        if !path.is_dir() {
                            panic!("{:?} is not correct path", path);
                        }
                        config.path = path;
                    } else {
                        config.path = config.path.join(path).canonicalize().unwrap();
                    }
                }
                _ => (),
            }
        }
        if config.name.is_empty() {
            panic!("name can't be empty");
        }
        config.path.push(&config.name);
        config
    }
}
