use std::path::Path;
use std::process::Command;

use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'i', long = "input", required = true)]
    input: String,
    #[arg(short = 'o', long = "output", required = true)]
    output: String,
}

fn watch_file_changes() {}

fn run_ffmpeg_cmd(video_input: String) {
    let mut cmd = Command::new("ffmpeg");
    cmd.args(["-hide_banner", "-v", "fatal", "-stats"]);
    cmd.arg("-i").arg(&video_input);
}

fn find_videos(input: &Path) {
    for entry in WalkDir::new(input).into_iter() {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if path.is_file() {
                    // println!("PATH: {}", path.display());
                    match path.extension() {
                        Some(ext) => match ext.to_str().unwrap() {
                            "mp4" | "webm" | "mkv" => println!("V:{}", path.display()),
                            _ => {}
                        },
                        None => {}
                    }
                }
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let input = Path::new(&cli.input);

    match input.is_dir() && input.exists() {
        true => find_videos(&input),
        false => eprint!("well fuck jim"),
    }
}
