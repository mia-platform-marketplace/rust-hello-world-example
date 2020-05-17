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

use std::sync::{Arc, RwLock};

use actix_web::{body::Body, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::printer::Printer;

use struct2swagger::{JsonSchemaDefinition, QueryDefinition, swagger_object::SwaggerObject};

#[derive(Debug, Deserialize, Swagger)]
pub struct Who {
    pub name: Option<String>,
}
#[cfg_attr(test, derive(Deserialize, PartialEq))]
#[derive(Debug, Serialize, Swagger)]
pub struct HelloWorldResponse {
    pub say: String,
}
#[cfg_attr(test, derive(Serialize))]
#[derive(Debug, Deserialize, Swagger)]
pub struct ChangeSayRequest {
    pub say: String,
}
#[derive(Debug, Serialize, Swagger)]
pub struct ChangeSayResponse {}
#[derive(Debug, Serialize, Swagger)]
pub struct IsUpResponse {}

pub fn say(printer: web::Data<Arc<RwLock<Printer>>>, item: web::Query<Who>) -> HttpResponse {
    let item: Who = item.into_inner();
    let say = {
        let printer = printer.read();
        let printer = match printer {
            Err(_) => {
                return HttpResponse::InternalServerError().message_body(Body::from_slice(b"Error"))
            }
            Ok(printer) => printer,
        };
        printer.format(item.name)
    };
    HttpResponse::Ok().json(HelloWorldResponse { say })
}

pub fn change_say(
    printer: web::Data<Arc<RwLock<Printer>>>,
    item: web::Json<ChangeSayRequest>,
) -> HttpResponse {
    let item: ChangeSayRequest = item.into_inner();
    {
        let printer = printer.write();
        let mut printer = match printer {
            Err(_) => {
                return HttpResponse::InternalServerError().message_body(Body::from_slice(b"Error"))
            }
            Ok(printer) => printer,
        };
        printer.change_say(item.say);
    };
    HttpResponse::Ok().json(ChangeSayResponse {})
}

pub fn is_up(printer: web::Data<Arc<RwLock<Printer>>>) -> HttpResponse {
    let is_up = {
        let lock = printer.read();
        lock.is_ok()
    };
    if is_up {
        HttpResponse::Ok().json(IsUpResponse {})
    } else {
        HttpResponse::InternalServerError().message_body(Body::from_slice(b"Error"))
    }
}

pub fn open_api_v3(swagger_object: web::Data::<Arc<RwLock<SwaggerObject>>>) -> HttpResponse {
    let swagger_object = swagger_object.read().unwrap();
    let swagger_object = &*swagger_object;
    HttpResponse::Ok().json(swagger_object)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[actix_rt::test]
    async fn test_is_up_ok() {
        let printer = Printer::new("Hello".to_owned());
        let printer = RwLock::new(printer);
        let printer = Arc::new(printer);
        let printer = web::Data::new(printer);

        let resp = is_up(printer).await.unwrap();
        assert_eq!(resp.status(), 200);
    }

    #[actix_rt::test]
    async fn test_is_up_ko() {
        let printer = Printer::new("Hello".to_owned());
        let printer = RwLock::new(printer);
        let printer = Arc::new(printer);
        make_rwlock_poisoned(printer.clone());

        let printer = web::Data::new(printer);

        let resp = is_up(printer).await.unwrap();
        assert_eq!(resp.status(), 500);
    }

    fn make_rwlock_poisoned(c_mutex: Arc<RwLock<Printer>>) {
        let _ = thread::spawn(move || {
            let mut data = c_mutex.write().unwrap();
            data.change_say("Hi".to_owned());
            panic!();
        })
        .join();
    }
}
