#[derive(Debug)]
enum CMD {
    Login,
    Connect,
    Send,
    Read,
    Help
}


pub struct Config {
    cmd: CMD, // login | connect | send | read | help
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, std::io::Error> {
        let valid_cmds = vec!["help", "login", "connect", "send", "read"];
        let default_cmd = String::from("help");
        let cmd = args.get(1).unwrap_or(&default_cmd);

        let cmd = if valid_cmds.contains(&&cmd[..]) {
            cmd.clone()
        } else  {
            default_cmd.clone()
        };

        let cmd: CMD = match &cmd[..] {
            "login" => CMD::Login,
            "connect" => CMD::Connect,
            "send" => CMD::Send,
            "read" => CMD::Read,
            "help" => CMD::Help,
            &_ => CMD::Help,
        };

        Ok(Config { cmd })
    }
}

pub fn run(config: Config) -> Result<String, std::io::Error> {
    match config.cmd {
        CMD::Login => login(),
        CMD::Connect => connect(),
        CMD::Send => send(),
        CMD::Read => read(),
        CMD::Help => help()
    }
}
use std::io::Write;

/**
 * https://tools.ietf.org/html/draft-ietf-jose-cfrg-curves-06#section-2
 */
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct X25519JWK {
    kty: String, // Must be "OKP"
    crv: String, // Must be "Ed25519"
    x: String,   // base64 encoded public key
    d: String,   // base64 encoded private key
}

/**
 * Login - Creates a public/private key-pair if does not already exists, linked to the given name.
 */ 
fn login() -> Result<String, std::io::Error> {
    if !std::fs::metadata(".didchat/").is_ok() {
        std::fs::create_dir_all(".didchat/").unwrap_or_default();
    }

    let seed_path = format!(".didchat/seed");

    if !std::fs::metadata(&seed_path).is_ok() {
        let mut csprng = rand_core::OsRng{};
        let seed = ed25519_dalek::SecretKey::generate(&mut csprng);
        let seed_bytes = seed.as_bytes();
        let mut file = std::fs::File::create(seed_path).unwrap();
        file.write(seed_bytes).unwrap();
        Ok(format!("New user signed in."))
    } else {
        Ok(format!("Existing user already signed in."))
    }
}

fn connect() -> Result<String, std::io::Error> {
    Ok(String::from("Connecting to friend"))
}

fn send() -> Result<String, std::io::Error> {
    Ok(String::from("Sending message"))
}

fn read() -> Result<String, std::io::Error> {
    Ok(String::from("Reading message"))
}

fn help() -> Result<String, std::io::Error> {
    Ok(String::from("
    Usage: 

        cargo run <login|connect|send|read|help>
        didchat   <login|connect|send|read|help>

        didchat login <username>
"))
}
