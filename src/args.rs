use clap::{Arg, Command};

pub fn parse_args() -> String {
    let matches = Command::new("request-app")
        .version("1.0")
        .author("DroBoV1tya")
        .about("rustzap api ")
        .arg(
            Arg::new("token")
                .short('t')
                .long("token")
                .value_name("TOKEN")
                .help("Zap api token")
                .required(true)
                .num_args(1),
        ).get_matches();

        let token = matches.get_one::<String>("token").unwrap().to_string();
        return token
    }