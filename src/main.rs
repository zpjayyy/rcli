// rcli csv -i input.csv -o output.json  --header -d ";"

use clap::Parser;
use rcli::Base64SubCommand;
use rcli::Opts;
use rcli::SubCommand;
use rcli::decode;
use rcli::encode;
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
        SubCommand::Base64(opts) => match opts {
            Base64SubCommand::Encode(opts) => {
                let result = encode(opts).unwrap();
                println!("{}", result);
            }
            Base64SubCommand::Decode(opts) => {
                let result = decode(opts).unwrap();
                println!("{}", result);
            }
        },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
