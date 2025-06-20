// rcli csv -i input.csv -o output.json  --header -d ";"

use clap::Parser;
use rcli::Cli;
use rcli::Options;
use rcli::read_csv;
use rcli::write_json;

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);

    match args.option {
        Options::Csv { input, output, .. } => {
            let players = read_csv(&input).unwrap();
            write_json(&players, &output).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
