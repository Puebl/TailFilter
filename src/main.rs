use std::{env, fs::File, io::{BufRead, BufReader, Seek, SeekFrom}, thread, time::Duration};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: cargo run -- <file> [filter]");
        return Ok(());
    }
    let path = &args[1];
    let filter = if args.len() >= 3 { Some(args[2].clone()) } else { None };

    let mut f = File::open(path)?;
    let mut pos = f.metadata()?.len();
    f.seek(SeekFrom::Start(pos))?;

    loop {
        let len = f.metadata()?.len();
        if len < pos { // truncated
            pos = 0; f.seek(SeekFrom::Start(0))?;
        }
        if len > pos {
            f.seek(SeekFrom::Start(pos))?;
            let mut reader = BufReader::new(&f);
            let mut buf = String::new();
            while reader.read_line(&mut buf)? > 0 {
                let line = buf.trim_end_matches(['\n','\r']);
                match &filter {
                    Some(q) => if line.contains(q) { println!("{}", line); },
                    None => println!("{}", line),
                }
                buf.clear();
            }
            pos = f.stream_position()?;
        }
        thread::sleep(Duration::from_millis(300));
    }
}
