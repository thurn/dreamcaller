// Copyright (c) dreamcaller 2025-present
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ability_data::trigger_event::{TriggerEvent, TriggerKeyword};
use chumsky::prelude::choice;
use chumsky::{IterParser, Parser};

use crate::determiner_parser;
use crate::parser_utils::{phrase, ErrorType};

pub fn event_parser<'a>() -> impl Parser<'a, &'a str, TriggerEvent, ErrorType<'a>> {
    choice((materialize(), play())).boxed()
}

pub fn keyword_parser<'a>() -> impl Parser<'a, &'a str, TriggerEvent, ErrorType<'a>> {
    let single_keyword = choice((
        phrase("$materialized").to(TriggerKeyword::Materialized),
        phrase("$judgment").to(TriggerKeyword::Judgment),
        phrase("$dissolved").to(TriggerKeyword::Dissolved),
    ));

    single_keyword
        .separated_by(phrase(","))
        .at_least(1)
        .collect::<Vec<_>>()
        .map(TriggerEvent::Keywords)
        .boxed()
}

fn materialize<'a>() -> impl Parser<'a, &'a str, TriggerEvent, ErrorType<'a>> {
    phrase("you materialize")
        .ignore_then(determiner_parser::your_action())
        .map(TriggerEvent::Materialize)
        .boxed()
}

fn play<'a>() -> impl Parser<'a, &'a str, TriggerEvent, ErrorType<'a>> {
    phrase("you play").ignore_then(determiner_parser::your_action()).map(TriggerEvent::Play).boxed()
}
