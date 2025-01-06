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

use ability_data::ability::Ability;
use ability_data::effect::{Effect, GameEffect};
use ability_data::predicate::Predicate;
use ability_data::triggered_ability::TriggeredAbility;
use chumsky::prelude::*;
use core_data::numerics::Spark;

use crate::trigger_event_parser;

/// Takes a string containing card rules text and parses it into an [Ability]
/// data structure.
pub fn parse(text: &str) -> ParseResult<Ability, Rich<char>> {
    parser().parse(text)
}

fn parser<'a>() -> impl Parser<'a, &'a str, Ability, extra::Err<Rich<'a, char>>> {
    trigger_keyword()
        .ignore_then(trigger_event_parser::parser())
        .then_ignore(just(","))
        .then(effect_list())
        .then_ignore(just("."))
        .then_ignore(end())
        .map(|(event, effects)| Ability::Triggered(TriggeredAbility::new(event, effects)))
}

fn trigger_keyword<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Rich<'a, char>>> {
    text::keyword("Whenever").or(text::keyword("When"))
}

fn effect_list<'a>() -> impl Parser<'a, &'a str, Effect, extra::Err<Rich<'a, char>>> {
    choice((
        just("this character gains +1 spark")
            .to(Effect::Effect(GameEffect::GainSpark(Predicate::This, Spark(1)))),
        just("draw a card").to(Effect::Effect(GameEffect::DrawCards(1))),
    ))
    .padded()
}
