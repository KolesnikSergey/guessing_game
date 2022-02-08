//use std::net::TcpListener;
//use std::io::;
use std::env::var;

fn main() {
get_env("JAVA_HOME");
}

fn get_env(env_name: &str)  {
    println!("{:?}", env_name);
    println!();

    let config_home = var(env_name)
        .or_else(|_| var("HOME") );
    println!("{:?}", config_home);
    println!("{:?}", config_home.unwrap());
}