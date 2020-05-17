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

use actix_web::{middleware, web, App, HttpServer};
use std::clone::Clone;
use std::sync::{Arc, RwLock};
use struct2swagger::{JsonSchemaDefinition, QueryDefinition, swagger_object::SwaggerObject};
use std::collections::HashMap;

use crate::handlers::{
    change_say, is_up, say, open_api_v3,
    Who, HelloWorldResponse,
    ChangeSayRequest, ChangeSayResponse,
};
use crate::Config;
use crate::printer::Printer;

macro_rules! get_app {
    ($printer: ident, $swagger_definition: ident) => {
        App::new()
            .app_data($printer.clone())
            .app_data($swagger_definition.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(say))
                    .route(web::post().to(change_say)),
            )
            .service(
                web::scope("/-/")
                    .route("/ready", web::get().to(is_up))
                    .route("/healthz", web::get().to(is_up))
                    .route("/checkup", web::get().to(is_up)),
            )
            .service(
                web::scope("/documentation")
                    .route("/json", web::get().to(open_api_v3))
            )
    };
}

fn create_web_data_printer(printer: Printer) -> web::Data<Arc<RwLock<Printer>>> {
    let printer = RwLock::new(printer);
    let printer = Arc::new(printer);
    web::Data::new(printer)
}

fn create_web_data_swagger_definition (config: &Config) -> web::Data::<Arc<RwLock<SwaggerObject>>> {
    let mut swagger_object = SwaggerObject::new(&config.package_name, &config.package_version);

    swagger_add_router!(
        swagger_object,
        "GET",
        "/",
        Who,
        200,
        "say",
        HelloWorldResponse
    );

    swagger_add_router!(
        swagger_object,
        "POST",
        "/",
        "request_body",
        ChangeSayRequest,
        200,
        "change say prefix",
        ChangeSayResponse
    );

    let swagger_object = RwLock::new(swagger_object);
    let swagger_object = Arc::new(swagger_object);
    web::Data::new(swagger_object)
}

pub async fn start_server(config: Config, printer: Printer) -> std::io::Result<()> {
    let printer = create_web_data_printer(printer);
    let swagger_definition = create_web_data_swagger_definition(&config);
    HttpServer::new(move || get_app!(printer, swagger_definition))
        .bind(format!("0.0.0.0:{}", config.http_port))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::{ChangeSayRequest, HelloWorldResponse};
    use actix_web::{test, web, App};

    macro_rules! get_response_body {
        ($resp:ident) => {
            serde_json::from_slice(&(*test::read_body($resp).await)).unwrap()
        };
    }

    #[actix_rt::test]
    async fn test_flow() {
        std::env::set_var("RUST_LOG", "actix_web=info");
        let printer = Printer::new("Hello".to_owned());
        let printer = create_web_data_printer(printer);
        let swagger_object = create_web_data_swagger_definition(&Config {
            package_name: "test".to_owned(),
            package_version: "test".to_owned(),
            say: "Hello".to_owned(),
            http_port: 0,
            log_level: "info".to_owned(),
        });
        let mut app = test::init_service(get_app!(printer, swagger_object)).await;

        let req = test::TestRequest::with_uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);

        let result: HelloWorldResponse = get_response_body!(resp);
        assert_eq!(
            result,
            HelloWorldResponse {
                say: "Hello World!".to_owned()
            }
        );

        let req = test::TestRequest::with_uri("/?name=Tomm").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);

        let result: HelloWorldResponse = get_response_body!(resp);
        assert_eq!(
            result,
            HelloWorldResponse {
                say: "Hello Tomm!".to_owned()
            }
        );

        let body = ChangeSayRequest {
            say: "Hi".to_owned(),
        };
        let body = serde_json::to_vec(&body).unwrap();
        let req = test::TestRequest::post()
            .uri("/")
            .header("COntent-type", "application/json")
            .set_payload(body)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);

        let req = test::TestRequest::with_uri("/?name=Tomm").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), 200);

        let result: HelloWorldResponse = get_response_body!(resp);
        assert_eq!(
            result,
            HelloWorldResponse {
                say: "Hi Tomm!".to_owned()
            }
        );
    }
}
