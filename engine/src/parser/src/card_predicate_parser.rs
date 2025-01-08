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

use ability_data::predicate::{CardPredicate, Operator};
use chumsky::prelude::choice;
use chumsky::Parser;
use core_data::character_type::CharacterType;
use core_data::numerics::{Energy, Spark};

use crate::parser_utils::{numeric, phrase, ErrorType};

pub fn parser<'a>() -> impl Parser<'a, &'a str, CardPredicate, ErrorType<'a>> {
    choice((
        character_with_cost_compared_to_controlled(),
        character_with_cost_compared_to_abandoned(),
        non_recursive_predicate(),
    ))
    .boxed()
}

fn non_recursive_predicate<'a>() -> impl Parser<'a, &'a str, CardPredicate, ErrorType<'a>> {
    choice((
        character_with_cost(),
        character_with_spark(),
        character_with_materialized_ability(),
        character_type().map(CardPredicate::CharacterType),
        choice((phrase("cards"), phrase("card"))).to(CardPredicate::Card),
        character().to(CardPredicate::Character),
        choice((phrase("events"), phrase("event"))).to(CardPredicate::Event),
    ))
    .boxed()
}

fn character_with_cost<'a>() -> impl Parser<'a, &'a str, CardPredicate, ErrorType<'a>> {
    character()
        .ignore_then(numeric("with cost $", Energy, ""))
        .then(choice((
            phrase("or less").to(Operator::OrLess),
            phrase("or more").to(Operator::OrMore),
        )))
        .map(|(cost, operator)| CardPredicate::CharacterWithCost(cost, operator))
        .boxed()
}

fn character_with_spark<'a>() -> impl Parser<'a, &'a str, CardPredicate, ErrorType<'a>> {
    character()
        .ignore_then(numeric("with spark", Spark, ""))
        .then(choice((
            phrase("or less").to(Operator::OrLess),
            phrase("or more").to(Operator::OrMore),
        )))
        .map(|(spark, operator)| CardPredicate::CharacterWithSpark(spark, operator))
        .boxed()
}

fn character_with_cost_compared_to_controlled<'a>(
) -> impl Parser<'a, &'a str, CardPredicate, ErrorType<'a>> {
    non_recursive_predicate()
        .then_ignore(phrase("with cost"))
        .then(choice((
            phrase("less than or equal to").to(Operator::OrLess),
            phrase("equal to").to(Operator::Exactly),
            phrase("greater than or equal to").to(Operator::OrMore),
        )))
        .then(
            phrase("the number of")
                .ignore_then(non_recursive_predicate())
                .then_ignore(phrase("you control")),
        )
        .map(|((target, cost_operator), count_matching)| {
            CardPredicate::CharacterWithCostComparedToControlled {
                target: Box::new(target),
                cost_operator,
                count_matching: Box::new(count_matching),
            }
        })
        .boxed()
}

fn character_with_cost_compared_to_abandoned<'a>(
) -> impl Parser<'a, &'a str, CardPredicate, ErrorType<'a>> {
    non_recursive_predicate()
        .then_ignore(phrase("with cost"))
        .then(choice((
            numeric("$", Energy, "higher than").map(Operator::HigherBy),
            numeric("$", Energy, "lower than").map(Operator::LowerBy),
            phrase("greater than").to(Operator::OrMore),
            phrase("less than").to(Operator::OrLess),
            phrase("equal to").to(Operator::Exactly),
        )))
        .then_ignore(phrase("the abandoned character"))
        .map(|(target, cost_operator)| CardPredicate::CharacterWithCostComparedToAbandoned {
            target: Box::new(target),
            cost_operator,
        })
        .boxed()
}

fn character_with_materialized_ability<'a>(
) -> impl Parser<'a, &'a str, CardPredicate, ErrorType<'a>> {
    character()
        .ignore_then(phrase("with a $materialized ability"))
        .to(CardPredicate::CharacterWithMaterializedAbility)
        .boxed()
}

fn character_type<'a>() -> impl Parser<'a, &'a str, CharacterType, ErrorType<'a>> {
    phrase("{cardtype: ")
        .ignore_then(choice((
            choice((phrase("warriors"), phrase("warrior"))).to(CharacterType::Warrior),
            choice((phrase("survivors"), phrase("survivor"))).to(CharacterType::Survivor),
            choice((phrase("spirit animals"), phrase("spirit animal")))
                .to(CharacterType::SpiritAnimal),
        )))
        .then_ignore(phrase("}"))
        .boxed()
}

fn character<'a>() -> impl Parser<'a, &'a str, &'a str, ErrorType<'a>> {
    choice((phrase("characters"), phrase("character"))).boxed()
}
