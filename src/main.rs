mod handlers;
mod printer;
mod server;

#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;
#[macro_use]
extern crate struct2swagger;
#[macro_use]
extern crate struct2swagger_derive;
#[macro_use]
extern crate serde_json;

use envconfig::Envconfig;

use printer::Printer;
use server::start_server;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "LOG_LEVEL", default = "info")]
    pub log_level: String,

    #[envconfig(from = "HTTP_PORT", default = "3000")]
    pub http_port: u16,

    #[envconfig(from = "INITIAL_SAY", default = "Hello")]
    pub say: String,

    #[envconfig(from = "CARGO_PKG_NAME", default = "unknown")]
    pub package_name: String,

    #[envconfig(from = "CARGO_PKG_VERSION", default = "unknown")]
    pub package_version: String,

}

/*
 * Copyright 2020 Mia srl
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

async fn start() -> std::io::Result<()> {
    let config = Config::init().unwrap();

    std::env::set_var("RUST_LOG", format!("actix_web={}", config.log_level));
    env_logger::init();

    let printer = Printer::new(config.say.clone());
    start_server(config, printer).await.unwrap();

    Ok(())
}

#[actix_rt::main]
async fn main() -> Result<(), String> {
    start().await.unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use std::time;

    #[actix_rt::test]
    async fn test_start() {
        std::env::set_var("LOG_LEVEL", "info");
        std::env::set_var("HTTP_PORT", "3456");

        std::thread::spawn(|| {
            let ten_millis = time::Duration::from_secs(1);
            std::thread::sleep(ten_millis);

            let current_pid = std::process::id().to_string();
            // suicide process
            let output = Command::new("kill")
                .args(&["-SIGTERM", &*current_pid])
                .output()
                .expect("failed to execute process");
            println!("{:?}", String::from_utf8(output.stderr));
        });

        let res = start().await;

        assert_eq!(res.is_ok(), true);
    }
}
