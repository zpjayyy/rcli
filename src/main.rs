// rcli csv -i input.csv -o output.json  --header -d ";"

use clap::Parser;
use rcli::Opts;
use rcli::SubCommand;
use rcli::gen_password;
use rcli::process_csv;

fn main() {
    let args = Opts::parse();
    println!("{:?}", args);

    match args.option {
        SubCommand::Csv(opts) => {
            process_csv(opts).unwrap();
        }
        SubCommand::GenPass(opts) => gen_password(opts).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
