use clap::{Arg, Command};

pub fn parse_args() -> (String, Option<String>, Option<String>) {
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
        )
        .arg(
            Arg::new("proxy_port")
                .short('p')
                .long("proxy")
                .value_name("PORT")
                .help("Proxy listener port  |   Default 8010")
                .required(false)
                .num_args(1)
        )
        .arg(
            Arg::new("zap_port")
            .short('z')
            .long("zap")
            .value_name("PORT")
            .help("Zap api port     |   Default 8080")
            .required(false)
            .num_args(1)
        )
        .get_matches();

        let token = matches.get_one::<String>("token").unwrap().to_string();
        let proxy = matches.get_one::<String>("proxy_port").map(|s| s.to_string());
        let zap = matches.get_one::<String>("zap_port").map(|s| s.to_string());

        return (token, proxy, zap)
    }