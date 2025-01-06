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

use ability_data::cost::Cost;
use chumsky::error::Rich;
use chumsky::prelude::*;
use chumsky::{extra, Parser};
use core_data::numerics::Energy;

pub fn parser<'a>() -> impl Parser<'a, &'a str, Cost, extra::Err<Rich<'a, char>>> {
    choice((
        just("$")
            .ignore_then(text::int(10))
            .map(|s: &str| Cost::Energy(Energy(s.parse().unwrap()))),
        just("Banish ")
            .ignore_then(text::int(10))
            .then_ignore(just(" cards from your void"))
            .map(|s: &str| Cost::BanishCardsFromYourVoid(s.parse().unwrap())),
    ))
    .padded()
}
