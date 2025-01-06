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

use std::{env, process};

use ariadne::{Color, Label, Report, ReportKind, Source};
use parser::ability_parser;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: parser_cli <expression>");
        process::exit(0)
    }

    let (result, errs) = ability_parser::parse(&args[1]).into_output_errors();
    if let Some(output) = result.as_ref() {
        println!(
            "{}",
            ron::ser::to_string_pretty(
                output,
                ron::ser::PrettyConfig::default().struct_names(true)
            )
            .unwrap(),
        );
    }

    errs.into_iter().for_each(|e| {
        Report::build(ReportKind::Error, (), e.span().start)
            .with_message(e.to_string())
            .with_label(
                Label::new(e.span().into_range())
                    .with_message(e.reason().to_string())
                    .with_color(Color::Red),
            )
            .finish()
            .eprint(Source::from(&args[1]))
            .unwrap()
    });
}
