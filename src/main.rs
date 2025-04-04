use clap::{Parser, ValueEnum};
use std::f64::consts::PI;
use std::thread::sleep;
use std::time::{Duration, Instant};
use rand::Rng;

/// CLI Signal Generator – mock analog signals (sine, random, flat)
#[derive(Parser, Debug)]
#[command(name = "mock-signal")]
#[command(about = "A CLI signal simulator that outputs fake analog data", long_about = None)]
struct Cli {
    /// Output pattern: sine, random, flat
    #[arg(short, long, default_value = "sine")]
    pattern: Pattern,

    /// Output frequency in Hz (used by sine)
    #[arg(short, long, default_value_t = 1.0)]
    freq: f64,

    /// Signal amplitude
    #[arg(short, long, default_value_t = 5.0)]
    amp: f64,

    /// Center value of the wave
    #[arg(short, long, default_value_t = 20.0)]
    baseline: f64,

    /// Interval between outputs in milliseconds
    #[arg(short, long, default_value_t = 100)]
    interval: u64,
}

#[derive(ValueEnum, Clone, Debug)]
enum Pattern {
    Sine,
    Random,
    Flat,
}

fn main() {
    let cli = Cli::parse();
    println!("Parsed args: {:?}", cli);

    let start = Instant::now();

    loop {
        let elapsed = start.elapsed().as_secs_f64();
        let signal = generate_signal(&cli.pattern, cli.freq * elapsed, cli.amp, cli.baseline);

        println!("{:.2}", signal);
        sleep(Duration::from_millis(cli.interval));
    }
}

/// Signal generator logic
fn generate_signal(pattern: &Pattern, t: f64, amp: f64, baseline: f64) -> f64 {
    match pattern {
        Pattern::Sine => baseline + amp * (2.0 * PI * t).sin(),
        Pattern::Flat => baseline,
        Pattern::Random => {
            let mut rng = rand::thread_rng();
            baseline + amp * (rng.gen_range(-1.0..=1.0))
        }
    }
}
