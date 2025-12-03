use clap::Parser;
use rand::RngCore;
use std::time::{SystemTime, UNIX_EPOCH};

/// Simple CLI to generate UUID v7 values.
#[derive(Parser, Debug)]
#[command(author, version, about = "Generate UUID v7", long_about = None)]
struct Args {
    /// Number of UUIDs to generate
    #[arg(short = 'n', long = "count", default_value_t = 1)]
    count: usize,

    /// Output without hyphens
    #[arg(long = "no-hyphen")]
    no_hyphen: bool,

    /// Wrap UUIDs in braces
    #[arg(long = "braced")]
    braced: bool,

    /// Uppercase hex letters
    #[arg(long = "upper")]
    upper: bool,
}

fn generate_v7(rng: &mut impl RngCore) -> [u8; 16] {
    let mut uuid = [0u8; 16];

    // 48-bit unix epoch milliseconds
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let ms = (now.as_millis() as u64) & 0x0000_FFFF_FFFF_FFFFu64;

    uuid[0] = (ms >> 40) as u8;
    uuid[1] = (ms >> 32) as u8;
    uuid[2] = (ms >> 24) as u8;
    uuid[3] = (ms >> 16) as u8;
    uuid[4] = (ms >> 8) as u8;
    uuid[5] = ms as u8;

    rng.fill_bytes(&mut uuid[6..16]);

    // set version (7) in the high 4 bits of byte 6
    uuid[6] = (uuid[6] & 0x0f) | (7 << 4);

    // set RFC 4122 variant: the two most significant bits of byte 8 to 10
    uuid[8] = (uuid[8] & 0x3f) | 0x80;

    uuid
}

fn format_uuid(b: [u8; 16], no_hyphen: bool, braced: bool, upper: bool) -> String {
    use std::fmt::Write;

    let mut s = String::with_capacity(36 + if braced { 2 } else { 0 });

    let write_byte = |s: &mut String, b: u8, upper: bool| {
        if upper {
            write!(s, "{:02X}", b).unwrap();
        } else {
            write!(s, "{:02x}", b).unwrap();
        }
    };

    if no_hyphen {
        for i in 0..16 {
            write_byte(&mut s, b[i], upper);
        }
    } else {
        for i in 0..4 {
            write_byte(&mut s, b[i], upper);
        }
        s.push('-');
        for i in 4..6 {
            write_byte(&mut s, b[i], upper);
        }
        s.push('-');
        for i in 6..8 {
            write_byte(&mut s, b[i], upper);
        }
        s.push('-');
        for i in 8..10 {
            write_byte(&mut s, b[i], upper);
        }
        s.push('-');
        for i in 10..16 {
            write_byte(&mut s, b[i], upper);
        }
    }

    if braced {
        format!("{{{}}}", s)
    } else {
        s
    }
}

fn main() {
    let args = Args::parse();

    let mut rng = rand::thread_rng();

    for _ in 0..args.count {
        let u = generate_v7(&mut rng);
        println!("{}", format_uuid(u, args.no_hyphen, args.braced, args.upper));
    }
}
