use ability_data::effect::Effect;
use ability_data::predicate::Predicate;
use ability_data::standard_effect::StandardEffect;
use ability_data::triggered_ability::{TriggeredAbility, TriggeredAbilityOptions};
use chumsky::prelude::*;
use chumsky::Parser;
use core_data::numerics::{Energy, Points, Spark};

use crate::parser_utils::{
    a_or_an, a_or_count, count, number_of_times, numeric, phrase, text_number, ErrorType,
};
use crate::{
    card_predicate_parser, cost_parser, counting_expression_parser, determiner_parser,
    quantity_expression_parser, trigger_event_parser,
};

/// Parses all standard game effects
pub fn parser<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    choice((non_recursive_effects(), create_trigger_until_end_of_turn())).boxed()
}

/// Parses all standard game effects that do not recursively invoke effect
/// parsing
fn non_recursive_effects<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    choice((card_effects(), spark_effects(), game_effects(), pay_cost()))
}

fn card_effects<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    choice((
        draw_matching_card(),
        draw_cards_for_each(),
        draw_cards(),
        banish_card_from_enemy_void(),
        discard_card_from_enemy_hand(),
        return_all_but_one_character_draw_card_for_each(),
        put_on_top_of_deck(),
        spend_all_energy_draw_and_discard(),
        materialize_character_from_void(),
        materialize_character(),
        dissolve_characters_count(),
        return_to_hand(),
        copy(),
        copy_next_played(),
    ))
}

fn spark_effects<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    choice((
        gain_spark_until_next_main_for_each(),
        gain_spark(),
        abandon_and_gain_energy_for_spark(),
        each_matching_gains_spark_for_each(),
        kindle(),
    ))
}

fn game_effects<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    choice((
        dissolve_character(),
        gains_aegis_this_turn(),
        gain_energy_for_each(),
        gain_energy(),
        gain_points_for_each(),
        gain_points(),
        gain_control(),
        lose_points(),
        enemy_gains_points_equal_to_its_spark(),
        enemy_gains_points(),
        enemy_loses_points(),
        disable_activated_abilities(),
        discover_and_then_materialize(),
        discover(),
        materialize_random_characters(),
        return_from_void_to_hand(),
        return_from_void_to_play(),
        gains_reclaim_until_end_of_turn(),
        negate(),
        abandon_at_end_of_turn(),
        banish_then_materialize(),
        banish_any_number_then_materialize(),
        banish_character(),
        foresee(),
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
        .map(|(predicate, spark)| StandardEffect::GainsSpark { target: predicate, gains: spark })
}

fn gain_energy<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    numeric("gain $", Energy, "").map(|energy| StandardEffect::GainEnergy { gains: energy })
}

fn gain_spark_until_next_main_for_each<'a>(
) -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    determiner_parser::target_parser()
        .then(numeric("gains +", Spark, "spark until your next main phase for each"))
        .then(card_predicate_parser::parser())
        .then_ignore(phrase("you control"))
        .map(|((target, spark), counted)| StandardEffect::GainsSparkUntilYourNextMainForEach {
            target,
            gains: spark,
            for_each: Predicate::Your(counted),
        })
}

fn dissolve_character<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("dissolve")
        .ignore_then(determiner_parser::target_parser())
        .map(|predicate| StandardEffect::DissolveCharacter { target: predicate })
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

fn banish_card_from_enemy_void<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
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
    numeric("gain $", Energy, "").then(determiner_parser::for_each_parser()).map(
        |(gained, counted)| StandardEffect::GainEnergyForEach { gains: gained, for_each: counted },
    )
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
        .map(|(count, predicate)| StandardEffect::MaterializeRandomFromDeck { count, predicate })
        .boxed()
}

fn return_from_void_to_hand<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("return")
        .ignore_then(determiner_parser::your_action())
        .then_ignore(phrase("from your void to your hand"))
        .map(|target| StandardEffect::ReturnFromYourVoidToHand { target })
        .boxed()
}

fn return_from_void_to_play<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("return")
        .ignore_then(determiner_parser::your_action())
        .then_ignore(phrase("from your void to play"))
        .map(|target| StandardEffect::ReturnFromYourVoidToPlay { target })
        .boxed()
}

fn gains_reclaim_until_end_of_turn<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>>
{
    determiner_parser::target_parser()
        .then_ignore(phrase("gains {kw: reclaim} until end of turn"))
        .map(|target| StandardEffect::GainsReclaimUntilEndOfTurn { target })
        .boxed()
}

fn kindle<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    numeric("{kw: kindle}", Spark, "").map(|amount| StandardEffect::Kindle { amount }).boxed()
}

fn negate<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("negate")
        .ignore_then(determiner_parser::target_parser())
        .map(|target| StandardEffect::Negate { target })
        .boxed()
}

fn discard_card_from_enemy_hand<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("look at the enemy's hand. choose")
        .ignore_then(a_or_an())
        .ignore_then(card_predicate_parser::parser())
        .then_ignore(phrase("from it. the enemy discards that card"))
        .map(|predicate| StandardEffect::DiscardCardFromEnemyHand { predicate })
        .boxed()
}

fn abandon_at_end_of_turn<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("abandon")
        .ignore_then(determiner_parser::target_parser())
        .then_ignore(phrase("at end of turn"))
        .map(|target| StandardEffect::AbandonAtEndOfTurn { target })
        .boxed()
}

fn spend_all_energy_draw_and_discard<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>>
{
    phrase("spend all your remaining energy. draw x cards then discard x cards, where x is the energy spent this way")
        .to(StandardEffect::SpendAllEnergyDrawAndDiscard)
        .boxed()
}

fn put_on_top_of_deck<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("put")
        .ignore_then(determiner_parser::target_parser())
        .then_ignore(phrase("on top of the enemy's deck"))
        .map(|target| StandardEffect::PutOnTopOfEnemyDeck { target })
        .boxed()
}

fn each_matching_gains_spark_for_each<'a>(
) -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("each")
        .ignore_then(card_predicate_parser::parser())
        .then_ignore(phrase("you control gains +x spark, where x is the number of"))
        .then(card_predicate_parser::parser())
        .then_ignore(phrase("you control"))
        .map(|(matching, for_each)| StandardEffect::EachMatchingGainsSparkForEach {
            each: matching,
            gains: Spark(1),
            for_each,
        })
        .boxed()
}

fn return_all_but_one_character_draw_card_for_each<'a>(
) -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("return")
        .ignore_then(counting_expression_parser::parser())
        .then_ignore(phrase(
            "character you control to hand. draw a card for each character returned",
        ))
        .map(|count| StandardEffect::ReturnCharactersToHandDrawCardForEach { count })
        .boxed()
}

fn banish_character<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("banish")
        .ignore_then(determiner_parser::target_parser())
        .map(|predicate| StandardEffect::BanishCharacter { target: predicate })
}

fn banish_then_materialize<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("banish")
        .ignore_then(determiner_parser::target_parser())
        .then_ignore(phrase(", then materialize it"))
        .map(|target| StandardEffect::BanishThenMaterialize { target })
        .boxed()
}

fn banish_any_number_then_materialize<'a>(
) -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("banish")
        .ignore_then(counting_expression_parser::parser())
        .then(determiner_parser::counted_parser())
        .then_ignore(phrase(", then materialize them"))
        .map(|(count, target)| StandardEffect::BanishThenMaterializeCount { target, count })
}

fn materialize_character_from_void<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>>
{
    phrase("materialize")
        .ignore_then(a_or_an())
        .ignore_then(card_predicate_parser::parser())
        .then_ignore(phrase("from your void"))
        .map(|target| StandardEffect::MaterializeCharacterFromVoid { target })
        .boxed()
}

fn materialize_character<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("materialize")
        .ignore_then(determiner_parser::target_parser())
        .map(|target| StandardEffect::MaterializeCharacter { target })
        .boxed()
}

fn gain_points<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    numeric("gain", Points, "$point")
        .then_ignore(just("s").or_not())
        .map(|points| StandardEffect::GainPoints { gains: points })
}

fn foresee<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    numeric("{kw: foresee}", count, "").map(|count| StandardEffect::Foresee { count }).boxed()
}

fn lose_points<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    numeric("you lose", Points, "$point")
        .then_ignore(just("s").or_not())
        .map(|points| StandardEffect::LosePoints { loses: points })
}

fn dissolve_characters_count<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("dissolve")
        .ignore_then(counting_expression_parser::parser())
        .then(determiner_parser::counted_parser())
        .map(|(count, target)| StandardEffect::DissolveCharactersCount { target, count })
        .boxed()
}

fn enemy_gains_points<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("the enemy gains")
        .ignore_then(numeric("", count, "$point"))
        .then_ignore(just("s").or_not())
        .map(|count| StandardEffect::EnemyGainsPoints { count })
        .boxed()
}

fn enemy_gains_points_equal_to_its_spark<'a>(
) -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("the enemy gains $points equal to its spark")
        .to(StandardEffect::EnemyGainsPointsEqualToItsSpark)
        .boxed()
}

fn enemy_loses_points<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("the enemy loses")
        .ignore_then(numeric("", count, "$point"))
        .then_ignore(just("s").or_not())
        .map(|count| StandardEffect::EnemyLosesPoints { count })
        .boxed()
}

fn pay_cost<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    cost_parser::standard_cost().map(|cost| StandardEffect::PayCost { cost })
}

fn gain_control<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("gain control of")
        .ignore_then(determiner_parser::target_parser())
        .map(|target| StandardEffect::GainControl { target })
        .boxed()
}

fn return_to_hand<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("return")
        .ignore_then(determiner_parser::target_parser())
        .then_ignore(phrase("to hand"))
        .map(|target| StandardEffect::ReturnToHand { target })
        .boxed()
}

fn gain_points_for_each<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("gain")
        .ignore_then(numeric("", Points, "$point"))
        .then_ignore(just("s").or_not())
        .then_ignore(phrase("for each"))
        .then(quantity_expression_parser::parser())
        .map(|(gain, for_count)| StandardEffect::GainPointsForEach { gain, for_count })
        .boxed()
}

fn draw_cards_for_each<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("draw")
        .ignore_then(a_or_count("card", "cards"))
        .then_ignore(phrase("for each"))
        .then(quantity_expression_parser::parser())
        .map(|(count, for_count)| StandardEffect::DrawCardsForEach { count, for_each: for_count })
}

fn copy<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("copy")
        .ignore_then(determiner_parser::target_parser())
        .map(|target| StandardEffect::Copy { target })
        .boxed()
}

fn copy_next_played<'a>() -> impl Parser<'a, &'a str, StandardEffect, ErrorType<'a>> {
    phrase("copy the next")
        .ignore_then(card_predicate_parser::parser())
        .then_ignore(phrase("you play"))
        .then(number_of_times())
        .map(|(matching, times)| StandardEffect::CopyNextPlayed {
            matching: Predicate::Your(matching),
            times,
        })
        .boxed()
}
