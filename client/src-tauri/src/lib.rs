use std::env;

use core_data::numerics::Points;
use core_data::types::{CardFacing, Url};
use display_data::battle_view::{BattleView, ClientBattleId, DisplayPlayer, PlayerView};
use display_data::card_view::{CardView, ClientCardId, RevealedCardView};
use display_data::object_position::{ObjectPosition, Position};
use rand::Rng;
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let args: Vec<_> = env::args().collect();
    let specta_builder = Builder::<tauri::Wry>::new().commands(collect_commands![fetch_battle,]);

    if args.len() > 1 && args[1] == "--generate-bindings" {
        #[cfg(debug_assertions)]
        specta_builder
            .export(Typescript::default(), "../src/bindings.ts")
            .expect("Failed to export typescript bindings");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(specta_builder.invoke_handler())
        .invoke_handler(tauri::generate_handler![fetch_battle])
        .setup(move |app| {
            specta_builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
#[specta::specta]
fn fetch_battle(id: ClientBattleId) -> BattleView {
    BattleView {
        id,
        user: PlayerView {
            score: Points(0),
            can_act: false,
        },
        enemy: PlayerView {
            score: Points(0),
            can_act: false,
        },
        cards: vec![
            random_position_card(0),
            random_position_card(1),
            random_position_card(2),
            random_position_card(3),
            random_position_card(4),
            random_position_card(5),
        ],
        status_description: "Status".to_string(),
        controls: vec![],
    }
}

fn random_position_card(sorting_key: u32) -> CardView {
    let position = rand::thread_rng().gen_range(0..3);
    match position {
        0 => user_hand_card(sorting_key),
        1 => user_battlefield_card(sorting_key),
        _ => enemy_battlefield_card(sorting_key),
    }
}

fn user_hand_card(sorting_key: u32) -> CardView {
    card(Position::InHand(DisplayPlayer::User), sorting_key)
}

fn user_battlefield_card(sorting_key: u32) -> CardView {
    card(Position::OnBattlefield(DisplayPlayer::User), sorting_key)
}

fn enemy_battlefield_card(sorting_key: u32) -> CardView {
    card(Position::OnBattlefield(DisplayPlayer::Enemy), sorting_key)
}

fn card(position: Position, sorting_key: u32) -> CardView {
    CardView {
        id: ClientCardId::CardId(format!("#{}", sorting_key)),
        position: ObjectPosition {
            position,
            sorting_key,
            sorting_sub_key: 0,
        },
        card_back: Url("".to_string()),
        revealed: Some(RevealedCardView {
            image: Url("".to_string()),
            name: "Card".to_string(),
            rules_text: "Rules".to_string(),
            status: None,
            is_ability: false,
            is_token: false,
            can_drag: true,
        }),
        revealed_to_opponents: true,
        card_facing: CardFacing::FaceUp,
        create_position: None,
        destroy_position: None,
    }
}
