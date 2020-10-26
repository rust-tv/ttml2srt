use ttml2srt::raw;
use std::{env, process};

fn srt_time(tt_time: String) -> String {
    let mut iter = tt_time.split('.');
    let t = iter.next().expect("invalid time format");
    let ms = iter.next().unwrap_or_default();

    // expect_none() is not available in stable
    if iter.next().is_some() {
        panic!("invalid time format");
    }

    let ms = "0".repeat(3-ms.len()) + ms;
    t.to_owned() + "," + &ms
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: ttml2srt $ttml_file > $srt_file");
    }
    let mut srt = String::with_capacity(256*1024);
    match raw::Tt::try_from_file(&*args[1]) {
        Ok(tt_raw) => {
            tt_raw.body.div.p_vec
                .into_iter()
                .enumerate()
                .for_each(|(n, p)| {
                    srt.push_str(&(n+1).to_string());
                    srt.push('\n');
                    srt.push_str(&srt_time(p.begin));
                    srt.push_str(" --> ");
                    srt.push_str(&srt_time(p.end));
                    srt.push('\n');
                    let txt = p.val
                        .into_iter()
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>()
                        .join("\n");
                    srt.push_str(&txt);
                    srt.push('\n');
                    srt.push('\n');
                });
            println!("{}", srt)
        }
        Err(e) => {
            eprintln!("Failed to parse ttml file '{}' successfully:\n  {}", args[1], e);
            process::exit(1);
        }
    }
}
