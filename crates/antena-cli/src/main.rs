use antena_core::{half_wave_dipole_m, parse_gw};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        ["--version"] => println!("antena {}", env!("CARGO_PKG_VERSION")),
        ["dipole", mhz] => match mhz.parse::<f64>() {
            Ok(f) if f > 0.0 => println!("{:.3} m", half_wave_dipole_m(f)),
            _ => usage(),
        },
        ["gw", card] => match parse_gw(card) {
            Ok(w) => println!(
                "length {:.4} m, segment {:.4} m",
                w.length(),
                w.segment_length()
            ),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        },
        _ => usage(),
    }
}

fn usage() {
    eprintln!("usage: antena dipole MHZ | gw \"GW ...\" | --version");
    std::process::exit(2);
}
