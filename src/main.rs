extern crate clap;
extern crate dirs;
extern crate serde;
extern crate serde_json;

use clap::Clap;
use std::collections::VecDeque;
use std::fs;
use std::io::Write;

#[derive(Clap)]
#[clap(
    version = "1.0.0",
    author = "xztaityoz",
    about = "shq: shell queue\n  simple string queue for command line"
)]
struct Opts {
    #[clap(
        short = "s",
        long = "source",
        help = "Queue source file (default: /path/to/cache_dir/shq/defalut)"
    )]
    source: Option<String>,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(name = "push", about = "push to queue")]
    Push(Push),
    #[clap(name = "pop", about = "pop from queue")]
    Pop(Pop),
    #[clap(name = "clear", about = "clear queue")]
    Clear,
    #[clap(name = "show", about = "show queue")]
    Show,
}

#[derive(Clap)]
struct Push {
    value: String,
    #[clap(short = "f", long = "front", help = "push to front")]
    front: bool,
}

#[derive(Clap)]
struct Pop {
    #[clap(short = "b", long = "back", help = "pop from back")]
    back: bool,
}

fn main() -> std::io::Result<()> {
    let opt: Opts = Opts::parse();
    let src = match opt.source {
        None => {
            let mut p = dirs::cache_dir().unwrap();
            p.push("shq");

            {
                // make cache directory
                let d = p.to_str().unwrap();
                fs::create_dir_all(d)?
            }

            p.push("default");
            p
        }
        Some(s) => std::path::PathBuf::from(s),
    };
    let src = src.into_os_string().into_string().unwrap();
    let dst = src.clone();

    // read or create queue file
    let mut deq = {
        if std::path::Path::new(&src).exists() {
            let g: VecDeque<String> = serde_json::from_reader(fs::File::open(src)?)?;
            g
        } else {
            VecDeque::new()
        }
    };

    match opt.subcmd {
        SubCommand::Push(p) => {
            if !p.front {
                deq.push_back(p.value);
            } else {
                deq.push_front(p.value);
            }
        }
        SubCommand::Pop(p) => {
            let item = if p.back {
                deq.pop_back()
            } else {
                deq.pop_front()
            };
            println!("{}", item.expect("Queue is empty"))
        }
        SubCommand::Clear => deq.clear(),
        SubCommand::Show => {
            println!("{:?}", deq);
            return Ok(());
        }
    }

    {
        // write to queue file
        let json = serde_json::to_string(&deq)?;
        let mut dst = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(dst)?;
        dst.write_all(&json.as_bytes())?;
        dst.flush()?;
    }
    Ok(())
}
