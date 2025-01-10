use ability_data::cost::Cost;
use ability_data::effect::Effect;
use ability_data::static_ability::{AlternateCost, PlayFromVoid, StaticAbility};
use chumsky::prelude::*;
use chumsky::Parser;
use core_data::numerics::{Energy, Spark};

use crate::parser_utils::{numeric, phrase, this, ErrorType};
use crate::{
    card_predicate_parser, condition_parser, cost_parser, determiner_parser, standard_effect_parser,
};

pub fn parser<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    choice((
        cost_increase(),
        cost_reduction(),
        disable_enemy_materialized_abilities(),
        once_per_turn_play_from_void(),
        enemy_added_cost_to_play(),
        other_spark_bonus(),
        has_all_character_types(),
        play_from_void(),
        simple_alternate_cost(),
        play_for_alternate_cost(),
        play_only_from_void(),
        reclaim(),
        spark_equal_to_predicate_count(),
        characters_in_hand_have_fast(),
        judgment_triggers_when_materialized(),
        look_at_top_card(),
        play_from_top_of_deck(),
    ))
    .boxed()
}

fn once_per_turn_play_from_void<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    phrase("once per turn, you may play a")
        .ignore_then(card_predicate_parser::parser())
        .then_ignore(phrase("from your void"))
        .map(|matching| StaticAbility::OncePerTurnPlayFromVoid { matching })
}

fn enemy_added_cost_to_play<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    phrase("the enemy's")
        .ignore_then(card_predicate_parser::parser())
        .then(numeric("cost $", Energy, "more"))
        .map(|(predicate, cost)| StaticAbility::EnemyCardsCostIncrease {
            matching: predicate,
            increase: cost,
        })
}

fn disable_enemy_materialized_abilities<'a>(
) -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    let enemy_characters = choice((phrase("the enemy's characters"), phrase("enemy characters")));
    phrase("disable the \"$materialized\" abilities of")
        .ignore_then(enemy_characters)
        .to(StaticAbility::DisableEnemyMaterializedAbilities)
}

fn other_spark_bonus<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    phrase("other")
        .ignore_then(card_predicate_parser::parser())
        .then(numeric("you control have +", Spark, "spark"))
        .map(|(predicate, spark)| StaticAbility::OtherCharactersSparkBonus {
            matching: predicate,
            added_spark: spark,
        })
}

fn has_all_character_types<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    phrase("this character has all character types").to(StaticAbility::HasAllCharacterTypes)
}

fn cost_increase<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    card_predicate_parser::parser().then(numeric("cost you $", Energy, "more")).map(
        |(predicate, cost)| StaticAbility::YourCardsCostIncrease {
            matching: predicate,
            reduction: cost,
        },
    )
}

fn cost_reduction<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    card_predicate_parser::parser().then(numeric("cost you $", Energy, "less")).map(
        |(predicate, cost)| StaticAbility::YourCardsCostReduction {
            matching: predicate,
            reduction: cost,
        },
    )
}

fn play_from_void<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    phrase("if")
        .ignore_then(condition_parser::parser())
        .then_ignore(phrase(","))
        .or_not()
        .then_ignore(phrase("you may play"))
        .then_ignore(this())
        .then_ignore(phrase("from your void"))
        .then(numeric("for $", Energy, "").or_not())
        .then(phrase("by").ignore_then(cost_parser::inflected_additional_cost()).or_not())
        .then(phrase(". if you do,").ignore_then(standard_effect_parser::parser()).or_not())
        .map(|(((condition, energy_cost), additional_cost), if_you_do)| {
            StaticAbility::PlayFromVoid(PlayFromVoid {
                condition,
                energy_cost,
                additional_cost: additional_cost.unwrap_or(Cost::NoCost),
                if_you_do: if_you_do.map(Effect::Effect),
            })
        })
}

fn play_for_alternate_cost<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    phrase("if")
        .ignore_then(condition_parser::parser())
        .then_ignore(phrase(","))
        .or_not()
        .then_ignore(phrase("you may play"))
        .then_ignore(this())
        .then(numeric("for $", Energy, ""))
        .then(phrase("by").ignore_then(cost_parser::inflected_additional_cost()).or_not())
        .then(phrase(". if you do,").ignore_then(standard_effect_parser::parser()).or_not())
        .map(|(((condition, energy_cost), additional_cost), if_you_do)| {
            StaticAbility::PlayForAlternateCost(AlternateCost {
                condition,
                energy_cost,
                additional_cost: additional_cost.unwrap_or(Cost::NoCost),
                if_you_do: if_you_do.map(Effect::Effect),
            })
        })
}

fn simple_alternate_cost<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    phrase("if")
        .ignore_then(condition_parser::parser())
        .then_ignore(phrase(","))
        .then_ignore(this())
        .then(numeric("costs $", Energy, ""))
        .map(|(condition, energy_cost)| {
            StaticAbility::PlayForAlternateCost(AlternateCost {
                condition: Some(condition),
                energy_cost,
                additional_cost: Cost::NoCost,
                if_you_do: None,
            })
        })
}

fn reclaim<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    phrase("{kw: reclaim}")
        .ignore_then(numeric("$", Energy, "").or_not())
        .map(|n| StaticAbility::Reclaim { cost: n.map(Cost::Energy) })
}

fn spark_equal_to_predicate_count<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    phrase("this character's spark is equal to the number of")
        .ignore_then(determiner_parser::counted_parser())
        .map(|predicate| StaticAbility::SparkEqualToPredicateCount { predicate })
        .boxed()
}

fn characters_in_hand_have_fast<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    phrase("characters in your hand have '$fast'").to(StaticAbility::CharactersInHandHaveFast)
}

fn judgment_triggers_when_materialized<'a>(
) -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    phrase("the '$judgment' ability of")
        .ignore_then(determiner_parser::counted_parser())
        .then_ignore(phrase("triggers when you materialize them"))
        .map(|predicate| StaticAbility::JudgmentTriggersWhenMaterialized { predicate })
}

fn look_at_top_card<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    phrase("you may look at the top card of your deck")
        .to(StaticAbility::YouMayLookAtTopCardOfYourDeck)
}

fn play_from_top_of_deck<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    phrase("you may play")
        .ignore_then(card_predicate_parser::parser())
        .then_ignore(phrase("from the top of your deck"))
        .map(|matching| StaticAbility::YouMayPlayFromTopOfDeck { matching })
}

fn play_only_from_void<'a>() -> impl Parser<'a, &'a str, StaticAbility, ErrorType<'a>> {
    phrase("you may only play this character from your void")
        .to(StaticAbility::PlayOnlyFromVoid)
        .boxed()
}
