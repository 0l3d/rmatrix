use rand::Rng;
use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};
use terminal_size::{Height, Width};

const DELAY: u64 = 50_000;

fn main() {
    if let Some((Width(width), Height(height))) = terminal_size::terminal_size() {
        let mut columns = vec![0u16; width as usize];
        let mut tails_length = vec![0u16; width as usize];
        let mut rng = rand::rng();

        for i in 0..width as usize {
            columns[i] = rng.random_range(0..height);
            tails_length[i] = rng.random_range(3..10);
        }

        loop {
            print!("\x1B[2J\x1B[H");
            io::stdout().flush().unwrap();

            for y in 0..height as usize {
                for x in 0..width as usize {
                    let tail_start = columns[x].saturating_sub(tails_length[x]);

                    if y <= columns[x] as usize && y > tail_start as usize {
                        if y == columns[x] as usize {
                            print!("\x1B[97m");
                        } else {
                            print!("\x1B[32m");
                        }
                        let c = (b'A' + rng.random_range(0..26)) as char;
                        print!("{c}");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }

            for i in 0..width as usize {
                columns[i] += 1;
                if columns[i] >= height + tails_length[i] {
                    columns[i] = 0;
                    tails_length[i] = rng.random_range(3..10);
                }
            }

            sleep(Duration::from_micros(DELAY));
        }
    } else {
        println!("Terminal size not found.");
    }
}
