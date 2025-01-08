use ability_data::effect::{Effect, EffectWithOptions, StandardEffect};
use ability_data::predicate::Predicate;
use chumsky::prelude::*;
use chumsky::Parser;
use core_data::numerics::{Energy, Spark};

use crate::parser_utils::{count, numeric, phrase, ErrorType};
use crate::{card_predicate_parser, condition_parser, determiner_parser};

pub fn effect<'a>() -> impl Parser<'a, &'a str, Effect, ErrorType<'a>> {
    single_effect().repeated().at_least(1).collect::<Vec<_>>().map(|effects| {
        match effects.as_slice() {
            [effect] => effect.clone().to_effect(),
            effects => Effect::List(effects.to_vec()),
        }
    })
}

fn single_effect<'a>() -> impl Parser<'a, &'a str, EffectWithOptions, ErrorType<'a>> {
    conditional_effect()
        .or(optional_effect())
        .or(standard_effect().map(EffectWithOptions::new))
        .then_ignore(just("."))
}

fn optional_effect<'a>() -> impl Parser<'a, &'a str, EffectWithOptions, ErrorType<'a>> {
    phrase("you may").ignore_then(standard_effect()).map(|game_effect| EffectWithOptions {
        effect: game_effect,
        optional: true,
        condition: None,
    })
}

fn conditional_effect<'a>() -> impl Parser<'a, &'a str, EffectWithOptions, ErrorType<'a>> {
    phrase("if")
        .ignore_then(condition_parser::parser())
        .then_ignore(phrase(","))
        .then(choice((optional_effect(), standard_effect().map(EffectWithOptions::new))))
        .map(|(condition, effect)| effect.with_condition(condition))
}

fn standard_effect<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    choice((
        discard_cards(),
        dissolve_character(),
        draw_cards(),
        draw_matching_card(),
        gain_energy(),
        gain_spark_until_next_main_for_each(),
        gain_spark(),
        gains_aegis_this_turn(),
        banish_card_from_void(),
    ))
}

fn draw_cards<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("draw")
        .ignore_then(choice((phrase("a card").to(1), numeric("", count, "cards"))))
        .map(|count| StandardEffect::DrawCards { count })
}

fn gain_spark<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    determiner_parser::target_parser()
        .then(numeric("gains +", Spark, "spark"))
        .map(|(predicate, spark)| StandardEffect::GainsSpark { target: predicate, gained: spark })
}

fn gain_energy<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    numeric("gain $", Energy, "").map(|energy| StandardEffect::GainEnergy { gained: energy })
}

fn gain_spark_until_next_main_for_each<'a>(
) -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    determiner_parser::target_parser()
        .then(numeric("gains +", Spark, "spark until your next main phase for each"))
        .then(card_predicate_parser::parser())
        .then_ignore(phrase("you control"))
        .map(|((target, spark), counted)| {
            StandardEffect::TargetGainsSparkUntilYourNextMainPhaseForEach {
                target,
                gained: spark,
                for_each: Predicate::Your(counted),
            }
        })
}

fn dissolve_character<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("dissolve")
        .ignore_then(determiner_parser::target_parser())
        .map(|predicate| StandardEffect::DissolveCharacter { target: predicate })
}

fn discard_cards<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("discard")
        .ignore_then(choice((phrase("a card").to(1), numeric("", count, "cards"))))
        .map(|count| StandardEffect::DiscardCards { count })
}

fn gains_aegis_this_turn<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    determiner_parser::target_parser()
        .then_ignore(phrase("gains {kw: aegis} this turn"))
        .map(|target| StandardEffect::GainsAegisThisTurn { target })
}

fn draw_matching_card<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("draw a")
        .ignore_then(card_predicate_parser::parser())
        .then_ignore(phrase("from your deck"))
        .map(|card_predicate| StandardEffect::DrawMatchingCard { predicate: card_predicate })
}

fn banish_card_from_void<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("banish")
        .ignore_then(choice((phrase("a card").to(1), numeric("", count, "cards"))))
        .then_ignore(phrase("from the enemy's void"))
        .map(|count| StandardEffect::BanishCardsFromEnemyVoid { count })
}
