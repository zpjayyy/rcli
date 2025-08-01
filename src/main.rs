// rcli csv -i input.csv -o output.json  --header -d ";"

use clap::Parser;
use rcli::Base64SubCommand;
use rcli::Opts;
use rcli::SubCommand;
use rcli::TextSubCommand;
use rcli::decode;
use rcli::encode;
use rcli::gen_password;
use rcli::generate_key;
use rcli::process_csv;
use rcli::sign;
use rcli::verify;

fn main() {
    let args = Opts::parse();
    println!("{:?}", args);

    match args.option {
        SubCommand::Csv(opts) => {
            process_csv(opts).unwrap();
        }
        SubCommand::GenPass(opts) => {
            let result = gen_password(opts).unwrap();
            print!("{}", result);
        }
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
        SubCommand::Text(opts) => match opts {
            TextSubCommand::Sign(opts) => {
                println!("{:?}", opts);
                let signature = sign(opts).unwrap();
                println!("{}", signature);
            }
            TextSubCommand::Verify(opts) => {
                println!("{:?}", opts);
                let result = verify(opts);
                print!("{:?}", result);
            }
            TextSubCommand::GenerateKey(opts) => {
                println!("{:?}", opts);
                let _ = generate_key(opts).unwrap();
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
