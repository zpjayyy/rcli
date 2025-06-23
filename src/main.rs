// rcli csv -i input.csv -o output.json  --header -d ";"

use clap::Parser;
use rcli::Cli;
use rcli::Options;
use rcli::process_csv;

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);

    match args.option {
        Options::Csv { input, output, .. } => {
            process_csv(&input, &output).unwrap();
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
