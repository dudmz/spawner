use crate::{lib, crawler};

pub fn help() {
    println!("
usage: spawner [OPTIONS] --mode MODE COMMAND

OPTIONS:

--start-url\tURL where spawner will crawl first
--worker\tAddress to worker with port, if using distributed mode

MODE:

standalone\tSingle node executor
distributed\tMultiple node executor, must have --worker OPT set

COMMAND:

start\tWell, start crawling, right?
serve\tStart a node instance, only available in distributed mode
    ");
}

fn check_invalid_arg(i: usize, args: &Vec<String>) {
    if args.len() == i + 1 {
        return;
    }
    if args.get(i).unwrap().starts_with("-") && args.get(i + 1).unwrap().starts_with("-") {
        help();
        lib::exit("invalid argument provided");
    }
}

fn get_arg_value(i: usize, args: &Vec<String>) -> &String {
    let ret: &String = args.get(i + 1).unwrap();
    ret
}

pub struct Program {
    pub command: Option<String>,
    pub opts: ProgramOpts
}

impl Program {
    pub fn new(args: &Vec<String>) -> Program {
        if args.len() == 1 {
            help();
            lib::exit("insufficient number of arguments");
        }
        let mut command: Option<String> = Default::default();
        for arg in args {
            match arg.as_ref() {
                "start" => command = Some(arg.to_owned()),
                _ => {},
            }
        }

        match command {
            Some(_) => {},
            None => {
                help();
                lib::exit("no command was provided");
            }
        }

        Self {
            command,
            opts: ProgramOpts::new(args),
        }
    }

    pub fn execute(&self) -> Result<(), &'static str> {
        match self.command.clone().unwrap().as_str() {
            "serve" => {
                match self.opts.mode {
                    crawler::lib::CrawlerMode::DISTRIBUTED => {
                        // TODO: write code to start the server
                        Ok(())
                    },
                    crawler::lib::CrawlerMode::STANDALONE => {
                        Err("serve command is noop for standalone mode")
                    }
                    _ => {
                        panic!("unreachable");
                    }
                }
            }
            "start" => {
                match self.opts.mode {
                    crawler::lib::CrawlerMode::DISTRIBUTED => {
                        // TODO: write code to start the web crawler
                        Ok(())
                    },
                    crawler::lib::CrawlerMode::STANDALONE => {
                        crawler::standalone::execute(self.opts.start_url.clone().unwrap());
                        Ok(())
                    }
                    _ => {
                        panic!("unreachable");
                    }
                }
            }
            &_ => {
                Err("invalid command")
            }
        }
    }
}


pub struct ProgramOpts {
    pub start_url: Option<String>,
    pub mode: crawler::lib::CrawlerMode,
    pub worker: Option<String>
}

impl ProgramOpts {
    fn new(args: &Vec<String>) -> ProgramOpts {
        let mut start_url: Option<String> = Default::default();
        let mut mode: crawler::lib::CrawlerMode = crawler::lib::CrawlerMode::INVALID;
        let mut worker: Option<String> = Default::default();

        let unmoved_args: &Vec<String> = args;

        for (i, arg) in args.clone().into_iter().enumerate() {
            if i == 0 {
                continue;
            }
            check_invalid_arg(i, &unmoved_args);
            match arg.as_ref() {
                "--start-url" => {
                    start_url = Some(get_arg_value(i, unmoved_args).clone());
                },
                "--mode" => {
                    mode = match Some(get_arg_value(i, args).clone().as_ref()) {
                        Some("distributed") => crawler::lib::CrawlerMode::DISTRIBUTED,
                        Some("standalone") => crawler::lib::CrawlerMode::STANDALONE,
                        Some(_) => crawler::lib::CrawlerMode::INVALID,
                        None => crawler::lib::CrawlerMode::INVALID,
                    }
                },
                "--worker" => {
                    worker = Some(get_arg_value(i, unmoved_args).clone());
                }
                _ => {},
            }
        }

        match mode {
            crawler::lib::CrawlerMode::INVALID => {
                help();
                lib::exit("a valid mode is required")
            },
            _ => {}
        }

        match start_url {
            Some(_) => {},
            None => {
                help();
                lib::exit("start_url is required");
            }
        }

        match worker {
            Some(_) => {
                match mode {
                    crawler::lib::CrawlerMode::DISTRIBUTED => {},
                    _ => {
                        help();
                        lib::exit("worker argument only valid for distributed mode")
                    }
                }
            }
            None => {
                match mode {
                    crawler::lib::CrawlerMode::DISTRIBUTED => {
                        help();
                        lib::exit("worker argument is required for distributed mode")
                    }
                    _ => {}
                }
            }
        }

        Self {
            start_url,
            mode,
            worker
        }
    }
}
