//
// soro
// Simple Online RadiO cli player
//

use console::Term;
use std::{
    env,
    fs::{self, File},
    io::{self, BufRead, Write},
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn play_stream(url: &str) {
    mpv_ctrl("kill", None);

    println!("playing: {}...", url);
    let mut child: Child = Command::new("mpv")
        .arg(url)
        .arg("--no-video")
        .arg("--no-audio-display")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("mpv failed to start");

    mpv_ctrl("create", Some(child.id()));

    let _ = child.wait();
}

fn mpv_ctrl(action: &str, pid: Option<u32>) {
    let pid_file_path = "/tmp/.soro.pid";
    match action {
        "create" => {
            if let Some(pid) = pid {
                let mut file = File::create(pid_file_path).expect("failed to create PID file");
                writeln!(file, "{}", pid).expect("failed to write PID to file");
            }
        }
        "kill" => {
            if let Ok(pid) = fs::read_to_string(pid_file_path) {
                let pid = pid.trim();
                if !pid.is_empty() {
                    let pid: u32 = pid.parse().expect("failed to parse PID");
                    Command::new("kill")
                        .arg(format!("{}", pid))
                        .output()
                        .expect("failed to kill mpv process");
                }
                fs::remove_file(pid_file_path).expect("failed to remove PID file");
            }
        }
        _ => return,
    }
}

fn main() {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let stdin = io::stdin();
    let input = stdin.lock().lines().next();
    let urls: Vec<String> = match input {
        Some(Ok(line)) => line.split_whitespace().map(String::from).collect(),
        _ => return,
    };

    let current_index = Arc::new(Mutex::new(0));
    let quit_flag = Arc::new(Mutex::new(false));

    let playback_index = Arc::clone(&current_index);
    let playback_urls = urls.clone();
    let playback_quit_flag = Arc::clone(&quit_flag);

    // spawning a playback thread
    if !urls.is_empty() {
        thread::spawn(move || {
            while !*playback_quit_flag.lock().unwrap() {
                let index = *playback_index.lock().unwrap();
                let url = &playback_urls[index];
                play_stream(url);

                // waiting for a short time before checking the index again
                thread::sleep(Duration::from_millis(100));
            }
        });

        println!("press <Enter> to play the next track, circularly, and 'q' to quit");

        let term = Term::stdout();
        loop {
            if let Ok(key) = term.read_key() {
                match key {
                    console::Key::Enter if !urls.is_empty() => {
                        let mut index = current_index.lock().unwrap();
                        *index = (*index + 1) % urls.len();
                        mpv_ctrl("kill", None);
                    }
                    console::Key::Char('q') => {
                        println!("exiting");
                        *quit_flag.lock().unwrap() = true;
                        break;
                    }
                    _ => {}
                }
            }

            if *quit_flag.lock().unwrap() {
                break;
            }

            thread::sleep(Duration::from_millis(100));
        }

        mpv_ctrl("kill", None);
        thread::sleep(Duration::from_millis(100));
    } else {
        println!("no URLs on stdin, exiting");
    }
}
