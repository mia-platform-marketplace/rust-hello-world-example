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

#[cfg_attr(test, derive(Clone))]
pub struct Printer {
    say: String,
}

impl Printer {
    pub fn new(say: String) -> Self {
        Printer { say }
    }

    pub fn format(&self, name: Option<String>) -> String {
        let name: String = match name {
            None => "World".to_owned(),
            Some(name) => name,
        };
        format!("{} {}!", self.say, name)
    }

    pub fn change_say(self: &mut Self, say: String) {
        self.say = say;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_should_set_say_member() {
        let printer = Printer::new("Hello".to_owned());
        assert_eq!(printer.say, "Hello".to_owned());
    }

    #[test]
    fn test_format_should_return_the_right_string() {
        let printer = Printer::new("Hello".to_owned());
        let result = printer.format(Some("Tom".to_owned()));
        assert_eq!(result, "Hello Tom!".to_owned());
    }

    #[test]
    fn test_format_should_fallback_on_none() {
        let printer = Printer::new("Hello".to_owned());
        let result = printer.format(None);
        assert_eq!(result, "Hello World!".to_owned());
    }

    #[test]
    fn test_change_say_should_change_the_say() {
        let mut printer = Printer::new("Hello".to_owned());
        printer.change_say("Hi".to_owned());
        let result = printer.format(None);
        assert_eq!(result, "Hi World!".to_owned());
    }
}
