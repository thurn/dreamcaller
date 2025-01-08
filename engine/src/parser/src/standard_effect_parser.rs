use ability_data::effect::{Effect, StandardEffect};
use ability_data::predicate::Predicate;
use ability_data::triggered_ability::{TriggeredAbility, TriggeredAbilityOptions};
use chumsky::prelude::*;
use chumsky::Parser;
use core_data::numerics::{Energy, Spark};

use crate::parser_utils::{count, numeric, phrase, text_number, ErrorType};
use crate::{card_predicate_parser, determiner_parser, trigger_event_parser};

/// Parses all standard game effects
pub fn parser<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    choice((non_recursive_effects(), create_trigger_until_end_of_turn())).boxed()
}

/// Parses all standard game effects that do not recursively invoke effect
/// parsing
fn non_recursive_effects<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    choice((
        dissolve_character(),
        draw_cards(),
        draw_matching_card(),
        discard_cards(),
        gains_aegis_this_turn(),
        gain_spark_until_next_main_for_each(),
        gain_spark(),
        gain_energy_for_each(),
        gain_energy(),
        banish_card_from_void(),
        disable_activated_abilities(),
        abandon_and_gain_energy_for_spark(),
        discover_and_then_materialize(),
        discover(),
        materialize_random_characters(),
        return_from_void_to_play(),
    ))
    .boxed()
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

fn disable_activated_abilities<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("disable the activated abilities of")
        .ignore_then(determiner_parser::target_parser())
        .then_ignore(phrase("while this character is in play"))
        .map(|target| StandardEffect::DisableActivatedAbilitiesWhileInPlay { target })
}

fn abandon_and_gain_energy_for_spark<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>>
{
    phrase("abandon")
        .ignore_then(determiner_parser::your_action())
        .then(numeric("and gain $", Energy, ""))
        .then_ignore(phrase("for each point of spark that character had"))
        .map(|(predicate, energy)| StandardEffect::AbandonAndGainEnergyForSpark {
            target: predicate,
            energy_per_spark: energy,
        })
}

fn gain_energy_for_each<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    numeric("gain $", Energy, "")
        .then(determiner_parser::counted_parser())
        .map(|(gained, counted)| StandardEffect::GainEnergyForEach { gained, for_each: counted })
}

fn create_trigger_until_end_of_turn<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>>
{
    phrase("until end of turn, whenever")
        .ignore_then(trigger_event_parser::event_parser())
        .then_ignore(phrase(","))
        .then(non_recursive_effects())
        .map(move |(trigger, effect)| StandardEffect::CreateTriggerUntilEndOfTurn {
            trigger: Box::new(TriggeredAbility {
                trigger,
                effect: Effect::Effect(effect),
                options: Some(TriggeredAbilityOptions {
                    once_per_turn: false,
                    until_end_of_turn: true,
                }),
            }),
        })
}

fn discover<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("{kw: discover}")
        .ignore_then(choice((phrase("a"), phrase("an"))))
        .ignore_then(card_predicate_parser::parser())
        .map(|predicate| StandardEffect::Discover { predicate })
        .boxed()
}

fn discover_and_then_materialize<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("{kw: discover}")
        .ignore_then(choice((phrase("a"), phrase("an"))))
        .ignore_then(card_predicate_parser::parser())
        .then_ignore(phrase("and materialize it"))
        .map(|predicate| StandardEffect::DiscoverAndThenMaterialize { predicate })
        .boxed()
}

fn materialize_random_characters<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("materialize")
        .ignore_then(choice((
            phrase("a random").to(1),
            text_number().then_ignore(phrase("random")),
        )))
        .then(card_predicate_parser::parser())
        .then_ignore(phrase("from your deck"))
        .map(|(count, predicate)| StandardEffect::MaterializeRandomCharacters { count, predicate })
        .boxed()
}

fn return_from_void_to_play<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("return")
        .ignore_then(determiner_parser::your_action())
        .then_ignore(phrase("from your void to play"))
        .map(|predicate| StandardEffect::ReturnFromYourVoidToPlay { predicate })
        .boxed()
}
