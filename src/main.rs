use clap::App;
use clap::Arg;
use colored::Colorize;
use std::io::Write;

mod input;
mod prelude;
mod solutions;

#[tokio::main]
async fn main() {
    let matches = App::new("Your CLI App")
        .version("1.0")
        .author("Your Name")
        .about("Description of your app")
        .arg(
            Arg::with_name("FUNCTION")
                .help("Specify the function to run")
                .required(true),
        )
        .arg(
            Arg::with_name("submit")
                .short("s")
                .long("submit")
                .help("Submit option"),
        )
        .arg(
            Arg::with_name("example")
                .short("e")
                .long("example")
                .help("Example option"),
        )
        .get_matches();

    let func = match matches.value_of("FUNCTION") {
        Some(val) => val.to_owned(),
        None => {
            println!();
            let prompt = String::from("Enter the function you'd like to run").on_green();
            print!("{}", prompt);
            print!(" ");
            std::io::stdout().flush().unwrap();
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            buffer.trim().to_owned()
        }
    };

    let submit = matches.is_present("submit");
    let example = matches.is_present("example");

    println!(
        "\n{}\n",
        format!(
            "    Solving {}",
            format!(" {} ", func).black().on_yellow().bold()
        )
        .bold()
        .on_blue()
    );

    use std::time::Instant;
    let now = Instant::now();
    match &func[..] {
        // INITIAL SOLUTIONS
        "d00s1" => solutions::day00::d00s1(submit, example).await,
        "d00s2" => solutions::day00::d00s2(submit, example).await,
        "d01s1" => solutions::day01::d01s1(submit, example).await,
        "d01s2" => solutions::day01::d01s2(submit, example).await,
        "d02s1" => solutions::day02::d02s1(submit, example).await,
        "d02s2" => solutions::day02::d02s2(submit, example).await,
        "d03s1" => solutions::day03::d03s1(submit, example).await,
        "d03s2" => solutions::day03::d03s2(submit, example).await,
        "d04s1" => solutions::day04::d04s1(submit, example).await,
        "d04s2" => solutions::day04::d04s2(submit, example).await,
        "d05s1" => solutions::day05::d05s1(submit, example).await,
        "d05s2" => solutions::day05::d05s2(submit, example).await,
        "d06s1" => solutions::day06::d06s1(submit, example).await,
        "d06s2" => solutions::day06::d06s2(submit, example).await,
        "d07s1" => solutions::day07s1::d07s1(submit, example).await,
        "d07s2" => solutions::day07s2::d07s2(submit, example).await,
        "d08s1" => solutions::day08::d08s1(submit, example).await,
        "d08s2" => solutions::day08::d08s2(submit, example).await,
        "d09s1" => solutions::day09::d09s1(submit, example).await,
        "d09s2" => solutions::day09::d09s2(submit, example).await,
        "d10s1" => solutions::day10::d10s1(submit, example).await,
        "d10s2" => solutions::day10::d10s2(submit, example).await,
        "d11s1" => solutions::day11::d11s1(submit, example).await,
        "d11s2" => solutions::day11::d11s2(submit, example).await,
        "d12s1" => solutions::day12::d12s1(submit, example).await,
        "d12s2" => solutions::day12::d12s2(submit, example).await,
        "d13s1" => solutions::day13::d13s1(submit, example).await,
        "d13s2" => solutions::day13::d13s2(submit, example).await,
        "d14s1" => solutions::day14::d14s1(submit, example).await,
        "d14s2" => solutions::day14::d14s2(submit, example).await,
        // "d15s1" => solutions::day15::d15s1(submit, example).await,
        // "d15s2" => solutions::day15::d15s2(submit, example).await,
        // "d16s1" => solutions::day16::d16s1(submit, example).await,
        // "d16s2" => solutions::day16::d16s2(submit, example).await,
        // "d17s1" => solutions::day17::d17s1(submit, example).await,
        // "d17s2" => solutions::day17::d17s2(submit, example).await,
        // "d18s1" => solutions::day18::d18s1(submit, example).await,
        // "d18s2" => solutions::day18::d18s2(submit, example).await,
        // "d19s1" => solutions::day19::d19s1(submit, example).await,
        // "d19s2" => solutions::day19::d19s2(submit, example).await,
        // "d20s1" => solutions::day20::d20s1(submit, example).await,
        // "d20s2" => solutions::day20::d20s2(submit, example).await,
        // "d21s1" => solutions::day21::d21s1(submit, example).await,
        // "d21s2" => solutions::day21::d21s2(submit, example).await,
        // "d22s1" => solutions::day22::d22s1(submit, example).await,
        // "d22s2" => solutions::day22::d22s2(submit, example).await,
        // "d23s1" => solutions::day23::d23s1(submit, example).await,
        // "d23s2" => solutions::day23::d23s2(submit, example).await,
        // "d24s1" => solutions::day24::d24s1(submit, example).await,
        // "d24s2" => solutions::day24::d24s2(submit, example).await,
        // "d25s1" => solutions::day25::d25s1(submit, example).await,
        // "d25s2" => solutions::day25::d25s2(submit, example).await,

        // REVISED APPROACHES
        "d05s1rev" => solutions::day05rev::d05s1(submit, example).await,
        "d05s2rev" => solutions::day05rev::d05s2(submit, example).await,

        // VISUALIZATIONS

        // ERR
        invalid => {
            println!(
                "{}\n",
                format!("Unrecognized function: {}", invalid.bold()).on_red()
            )
        }
    }
    println!(
        "{}\n",
        format!("Execution time: {:.2?}", now.elapsed())
            .black()
            .on_white()
    );
}
