use rand::prelude::*;
use serde::Serialize;
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::require;
use zkwasm_rust_sdk::wasm_dbg;
use zkwasm_rust_sdk::wasm_input;
use zkwasm_rust_sdk::wasm_output;

use crate::cards::HUMANOID_DECK;
use crate::cards::ROYAL_DECK;
use crate::cards::UNDEAD_DECK;
use crate::structs::Card;

#[wasm_bindgen]
#[derive(Serialize)]
struct GameState {
    forcast: Vec<(i32, i32, i32)>,
    forcast_enemy: Vec<(i32, i32, i32)>,
    gamestart: bool,
    gamestage: u32,
    cursor_x: i32,
    cursor_y: i32,
    picked_race: i32,
    reset_confirm: bool,
    // main game state
    round: i32,
    round_start: bool,
    round_running: bool,
    round_end: bool,
    curr_mana: i32,
    curr_hp: i32,
    curr_cards: Vec<i32>,

    used_deck_cards: Vec<i32>,
    deck_remain_cards_count: i32,
    curr_choice: i32,
    curr_choosing_mana: Vec<i32>,
    curr_chosen: Vec<i32>,
    curr_chosen_coordinate: Vec<(i32, i32)>,
    curr_cardboard_elements: Vec<Vec<i32>>,
    curr_cardboard_elements_hp: Vec<Vec<i32>>,
    curr_cardboard_elements_attack: Vec<Vec<i32>>,
    timer: i32,

    enemy_hp: i32,
    enemy_curr_mana: i32,
    enemy_used_deck_cards: Vec<i32>,
    enemy_curr_cards: Vec<i32>,
    // enemy_curr_choice: i32,
    // enemy_curr_choosing_mana: Vec<i32>,
    // enemy_curr_chosen: Vec<i32>,
    // enemy_curr_chosen_coordinate: Vec<(i32, i32)>,
    enemy_curr_cardboard_elements: Vec<Vec<i32>>,
    enemy_curr_cardboard_elements_hp: Vec<Vec<i32>>,
    enemy_curr_cardboard_elements_attack: Vec<Vec<i32>>,
    state_history: Vec<String>, // Add this new field to store state history
    is_generated: bool,
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen]
    pub fn initialize() -> GameState {
        GameState {
            forcast: Vec::new(),
            forcast_enemy: Vec::new(),
            gamestart: false,
            gamestage: 0,
            cursor_x: 120,
            cursor_y: 100,
            picked_race: -1,
            reset_confirm: false,

            // main game state
            round: 0,
            round_start: true,
            round_running: false,
            round_end: false,
            curr_mana: 2,
            curr_choosing_mana: Vec::new(),
            curr_hp: 20,
            curr_cards: Vec::new(),
            used_deck_cards: Vec::new(),
            deck_remain_cards_count: 26,
            curr_choice: -1,
            curr_chosen: Vec::new(),
            curr_chosen_coordinate: Vec::new(),
            curr_cardboard_elements: Vec::new(),
            curr_cardboard_elements_hp: Vec::new(),
            curr_cardboard_elements_attack: Vec::new(),
            timer: 30,

            enemy_hp: 20,
            enemy_curr_mana: 2,
            enemy_used_deck_cards: Vec::new(),
            enemy_curr_cards: Vec::new(),
            // enemy_curr_choice: -1,
            // enemy_curr_choosing_mana: Vec::new(),
            // enemy_curr_chosen:  Vec::new(),
            // enemy_curr_chosen_coordinate: Vec::new(),
            enemy_curr_cardboard_elements: Vec::new(),
            enemy_curr_cardboard_elements_hp: Vec::new(),
            enemy_curr_cardboard_elements_attack: Vec::new(),
            state_history: Vec::new(),
            is_generated: false,
        }
    }

    #[wasm_bindgen]
    pub fn calculate_distance_and_update_state(
        state: &mut GameState,
        race_center_x: i32,
        race_center_y: i32,
        race_id: i32,
    ) -> i32 {
        let cursor_center_x = state.cursor_x + 5;
        let cursor_center_y = state.cursor_y + 7;
        let dx = cursor_center_x - race_center_x;
        let dy = cursor_center_y - race_center_y;

        let distance_squared = dx.pow(2) + dy.pow(2);
        let radii_sum_squared = 22_i32.pow(2);

        if 0 <= distance_squared && distance_squared <= radii_sum_squared {
            state.picked_race = race_id;
        } else {
            state.picked_race = -1;
        }

        state.picked_race
    }

    #[wasm_bindgen]
    pub fn check_within_confirm(state: &mut GameState) -> bool {
        let cursor_center_x = state.cursor_x + 5;
        let cursor_center_y = state.cursor_y + 7;
        let range_x = (80, 180);
        let range_y = (120, 140);
        if range_x.0 <= cursor_center_x
            && cursor_center_x <= range_x.1
            && range_y.0 <= cursor_center_y
            && cursor_center_y <= range_y.1
        {
            return true;
        } else {
            return false;
        }
    }

    #[wasm_bindgen]
    pub fn determine_can_place(index: i32, state: &mut GameState) -> i8 {
        if state.curr_chosen.contains(&index) {
            0
        } else {
            let mut mana = 0;
            if state.picked_race == 0 {
                mana = ROYAL_DECK[state.curr_cards[index as usize] as usize].mana;
            } else if state.picked_race == 1 {
                mana = HUMANOID_DECK[state.curr_cards[index as usize] as usize].mana;
            } else if state.picked_race == 2 {
                mana = UNDEAD_DECK[state.curr_cards[index as usize] as usize].mana;
            }

            let sum: i32 = state.curr_choosing_mana.iter().sum();
            if mana > (state.curr_mana - sum) {
                1
            } else {
                state.curr_choice = index;
                10
            }
        }
    }

    #[wasm_bindgen]
    pub fn check_card_selection(state: &mut GameState) {
        let card1x_range = (0, 46);
        let card2x_range = (46, 92);
        let card3x_range = (92, 138);
        let card4x_range = (138, 184);

        let card1y_range = (100, 146);
        let card2y_range = (100, 146);
        let card3y_range = (100, 146);
        let card4y_range = (100, 146);

        if state.gamestage == 2
            && (state.cursor_x >= 0 && state.cursor_x <= 34)
            && (state.cursor_y >= 68 && state.cursor_y <= 78)
        {
            // pass clicked
            Self::simulate_enemy(state);
            Self::simulate_fight(state);
            state.curr_chosen.clear();
            state.curr_choosing_mana.clear();
            state.curr_chosen_coordinate.clear();
            state.round_running = false;
            state.round_end = true;
            state.cursor_x = 120;
            state.cursor_y = 80;
        } else if state.gamestage == 2
            && (state.cursor_x >= 0 && state.cursor_x <= 34)
            && (state.cursor_y >= 80 && state.cursor_y <= 90)
        {
            if state.curr_chosen.len() > 0 {
                // cancel clicked
                state.curr_chosen.clear();
                state.curr_choosing_mana.clear();
                state.curr_chosen_coordinate.clear();
            }
        } else if state.gamestage == 2
            && (state.cursor_x >= 36 && state.cursor_x <= 70)
            && (state.cursor_y >= 80 && state.cursor_y <= 90)
        {
            if state.curr_chosen.len() > 0 {
                // attack clicked
                for i in 0..state.curr_chosen.len() {
                    let mut curr_card: &Card =
                        &ROYAL_DECK[state.curr_cards[state.curr_chosen[i] as usize] as usize];
                    if state.picked_race == 0 {
                        curr_card =
                            &ROYAL_DECK[state.curr_cards[state.curr_chosen[i] as usize] as usize];
                    } else if state.picked_race == 1 {
                        curr_card = &HUMANOID_DECK
                            [state.curr_cards[state.curr_chosen[i] as usize] as usize];
                    } else if state.picked_race == 2 {
                        curr_card =
                            &UNDEAD_DECK[state.curr_cards[state.curr_chosen[i] as usize] as usize];
                    }

                    state.curr_cardboard_elements[state.curr_chosen_coordinate[i].0 as usize]
                        [state.curr_chosen_coordinate[i].1 as usize] = curr_card.id;
                    state.curr_cardboard_elements_hp[state.curr_chosen_coordinate[i].0 as usize]
                        [state.curr_chosen_coordinate[i].1 as usize] = curr_card.hp;
                    state.curr_cardboard_elements_attack
                        [state.curr_chosen_coordinate[i].0 as usize]
                        [state.curr_chosen_coordinate[i].1 as usize] = curr_card.attack;
                    // text!(&format!("curr: {} {}", state.curr_chosen[i], state.curr_cards.len()), x = 0, y = 10 + 10*i as i32, font = Font::M, color = 0xffffffff); // Render the score
                    // state.curr_cards.remove(state.curr_chosen[i] as usize);
                    state.curr_cards[state.curr_chosen[i] as usize] = -1;
                }
                state.curr_cards.retain(|&card| card != -1);
                Self::simulate_enemy(state);
                Self::simulate_fight(state);
                state.curr_chosen.clear();
                state.curr_choosing_mana.clear();
                state.curr_chosen_coordinate.clear();
                state.round_running = false;
                state.round_end = true;
            }
        } else {
            if state.curr_cards.len() > 0
                && state.gamestage == 2
                && (card1x_range.0 <= state.cursor_x && state.cursor_x <= card1x_range.1)
                && (card1y_range.0 <= state.cursor_y && state.cursor_y <= card1y_range.1)
            {
                Self::determine_can_place(0, state);
            } else if state.curr_cards.len() > 1
                && state.gamestage == 2
                && (card2x_range.0 <= state.cursor_x && state.cursor_x <= card2x_range.1)
                && (card2y_range.0 <= state.cursor_y && state.cursor_y <= card2y_range.1)
            {
                Self::determine_can_place(1, state);
            } else if state.curr_cards.len() > 2
                && state.gamestage == 2
                && (card3x_range.0 <= state.cursor_x && state.cursor_x <= card3x_range.1)
                && (card3y_range.0 <= state.cursor_y && state.cursor_y <= card3y_range.1)
            {
                Self::determine_can_place(2, state);
            } else if state.curr_cards.len() > 3
                && state.gamestage == 2
                && (card4x_range.0 <= state.cursor_x && state.cursor_x <= card4x_range.1)
                && (card4y_range.0 <= state.cursor_y && state.cursor_y <= card4y_range.1)
            {
                Self::determine_can_place(3, state);
            } else {
                if state.curr_choice != -1 {
                    Self::set_curr_chosen_coordinate(state);
                }
                state.curr_choice = -1;
            }
        }
    }

    #[wasm_bindgen]
    pub fn set_curr_chosen_coordinate(state: &mut GameState) {
        let grid_x = (state.cursor_x - 80) / 16;
        let grid_y = (state.cursor_y - 48) / 16;
        if state.curr_choice == -1 {
            state.curr_chosen_coordinate.clear();
        } else {
            if (grid_x < 0)
                || (grid_x > 2)
                || (grid_y < 0)
                || (grid_y > 2)
                || state.curr_cardboard_elements[grid_x as usize][grid_y as usize] != -1
                || state.curr_chosen_coordinate.contains(&(grid_x, grid_y))
            {
                state.curr_chosen.clear();
                state.curr_choosing_mana.clear();
                state.curr_chosen_coordinate.clear();
            } else {
                state.curr_chosen.push(state.curr_choice);
                state.curr_chosen_coordinate.push((grid_x, grid_y));
                let mut mana = 0;
                if state.picked_race == 0 {
                    mana = ROYAL_DECK[state.curr_cards[state.curr_choice as usize] as usize].mana;
                } else if state.picked_race == 1 {
                    mana =
                        HUMANOID_DECK[state.curr_cards[state.curr_choice as usize] as usize].mana;
                } else if state.picked_race == 2 {
                    mana = UNDEAD_DECK[state.curr_cards[state.curr_choice as usize] as usize].mana;
                }

                state.curr_choosing_mana.push(mana);
            }
        }
    }

    #[wasm_bindgen]
    pub fn init_cardboard(state: &mut GameState) {
        state.curr_cardboard_elements_hp = vec![vec![0; 3]; 3];
        state.curr_cardboard_elements_attack = vec![vec![-1; 3]; 3];
        state.curr_cardboard_elements = vec![vec![-1; 3]; 3];

        state.enemy_curr_cardboard_elements_hp = vec![vec![0; 3]; 3];
        state.enemy_curr_cardboard_elements_attack = vec![vec![-1; 3]; 3];
        state.enemy_curr_cardboard_elements = vec![vec![-1; 3]; 3];
    }

    #[wasm_bindgen]
    pub fn init_enemy_hand(state: &mut GameState) {
        while state.enemy_curr_cards.len() < 4 {
            let number = (rand::random::<i32>() % 30) as i32;
            if !state.enemy_used_deck_cards.contains(&(number as i32)) {
                state.enemy_curr_cards.push(number);
                state.enemy_used_deck_cards.push(number);
            }
        }
    }

    #[wasm_bindgen]
    pub fn calc_enemy_hand(state: &mut GameState) {
        let curr_count = state.enemy_curr_cards.len();
        if curr_count < 4 {
            if state.enemy_used_deck_cards.len() == 30 {
            } else {
                while state.enemy_curr_cards.len() < curr_count + 1 {
                    let number = (rand::random::<i32>() % 30) as i32;
                    if !state.enemy_used_deck_cards.contains(&(number as i32)) {
                        state.enemy_curr_cards.push(number);
                        state.enemy_used_deck_cards.push(number);
                    }
                }
            }
        }
    }

    #[wasm_bindgen]
    pub fn simulate_enemy(state: &mut GameState) {
        for i in 0..state.enemy_curr_cards.len() {
            if UNDEAD_DECK[state.enemy_curr_cards[i] as usize].mana <= state.enemy_curr_mana {
                // chose and attack
                let mut random_x = rand::random::<i32>() % 3;
                let mut random_y = rand::random::<i32>() % 3;
                while state.enemy_curr_cardboard_elements[random_x as usize][random_y as usize]
                    != -1
                {
                    random_x = rand::random::<i32>() % 3;
                    random_y = rand::random::<i32>() % 3;
                }
                state.enemy_curr_cardboard_elements[random_x as usize][random_y as usize] =
                    state.enemy_curr_cards[i];
                state.enemy_curr_cardboard_elements_hp[random_x as usize][random_y as usize] =
                    UNDEAD_DECK[state.enemy_curr_cards[i] as usize].hp;
                state.enemy_curr_cardboard_elements_attack[random_x as usize][random_y as usize] =
                    UNDEAD_DECK[state.enemy_curr_cards[i] as usize].attack;
                state.enemy_curr_cards.remove(i);
                break;
            }
        }
        // pass if nothing can be done
    }

    #[wasm_bindgen]
    pub fn simulate_fight(state: &mut GameState) {
        state.forcast.clear();
        state.forcast_enemy.clear();
        for i in 0..3 {
            // action
            let mut a = 2;
            let mut b = 0;

            let mut defense_a = a;
            let mut defense_b = b;

            let mut init = true;

            loop {
                while a >= 0 && state.curr_cardboard_elements[a as usize][i as usize] == -1 {
                    a -= 1;
                }

                while b <= 2 && state.enemy_curr_cardboard_elements[b as usize][i as usize] == -1 {
                    b += 1;
                }

                if init == true {
                    defense_a = a;
                    defense_b = b;
                    init = false;
                }

                zkwasm_rust_sdk::dbg!(
                    "current row {} a {} b {} da {} db{} ",
                    i,
                    a,
                    b,
                    defense_a,
                    defense_b
                );
                // perform turn

                if defense_a == -1 && b <= 2 {
                    // no more defend,  enemy attack , you lose hp
                    for j in b..3 {
                        if state.enemy_curr_cardboard_elements[j as usize][i as usize] != -1 {
                            println!(
                                "you loses {} ",
                                state.enemy_curr_cardboard_elements_attack[j as usize][i as usize],
                            );
                            state.curr_hp -=
                                state.enemy_curr_cardboard_elements_attack[j as usize][i as usize];
                        }
                    }
                    break;
                }

                if defense_b == 3 && a >= 0 {
                    // no more defend, you attack , enemy lose hp
                    for j in 0..a + 1 {
                        if state.curr_cardboard_elements[j as usize][i as usize] != -1 {
                            println!(
                                "enemy loses {}",
                                state.curr_cardboard_elements_attack[j as usize][i as usize]
                            );
                            state.enemy_hp -=
                                state.curr_cardboard_elements_attack[j as usize][i as usize];
                        }
                    }
                    break;
                }

                // edge case
                if a == -1 && b == 3 {
                    println!("next available all empty");
                    break; // next available for both is empty
                }

                if a == -1 && b <= 2 {
                    // enemy attack , you lose hp
                    for j in b..3 {
                        if state.curr_cardboard_elements[j as usize][i as usize] != -1 {
                            println!(
                                "you loses {}",
                                state.enemy_curr_cardboard_elements_attack[j as usize][i as usize]
                            );
                            state.curr_hp -=
                                state.enemy_curr_cardboard_elements_attack[j as usize][i as usize];
                        }
                    }
                    break;
                }

                if b == 3 && a >= 0 {
                    // you attack , enemy lose hp
                    for j in 0..a + 1 {
                        if state.curr_cardboard_elements[j as usize][i as usize] != -1 {
                            println!(
                                "enemy loses {}",
                                state.curr_cardboard_elements_attack[j as usize][i as usize]
                            );
                            state.enemy_hp -=
                                state.curr_cardboard_elements_attack[j as usize][i as usize];
                        }
                    }
                    break;
                }

                if a >= 0 && b <= 2 {
                    // both attack
                    zkwasm_rust_sdk::dbg!("both attack");
                    println!(
                        "your card curr hp {} loses {}",
                        state.curr_cardboard_elements_hp[defense_a as usize][i as usize],
                        state.enemy_curr_cardboard_elements_attack[b as usize][i as usize]
                    );
                    println!(
                        "ene card curr hp {} loses {}",
                        state.enemy_curr_cardboard_elements_hp[defense_b as usize][i as usize],
                        state.curr_cardboard_elements_attack[a as usize][i as usize]
                    );

                    state.forcast.push((
                        i,
                        defense_a,
                        state.enemy_curr_cardboard_elements_attack[b as usize][i as usize],
                    ));
                    state.forcast_enemy.push((
                        i,
                        defense_b,
                        state.curr_cardboard_elements_attack[a as usize][i as usize],
                    ));

                    state.curr_cardboard_elements_hp[defense_a as usize][i as usize] -=
                        state.enemy_curr_cardboard_elements_attack[b as usize][i as usize];
                    state.enemy_curr_cardboard_elements_hp[defense_b as usize][i as usize] -=
                        state.curr_cardboard_elements_attack[a as usize][i as usize];

                    // check if a character is down
                    if state.curr_cardboard_elements_hp[defense_a as usize][i as usize] <= 0 {
                        state.curr_cardboard_elements[defense_a as usize][i as usize] = -1;
                        while defense_a >= 0
                            && state.curr_cardboard_elements[defense_a as usize][i as usize] == -1
                        {
                            defense_a -= 1;
                        }
                    }
                    if state.enemy_curr_cardboard_elements_hp[defense_b as usize][i as usize] <= 0 {
                        state.enemy_curr_cardboard_elements[defense_b as usize][i as usize] = -1;
                        defense_b += 1;
                        while defense_b <= 2
                            && state.enemy_curr_cardboard_elements[defense_b as usize][i as usize]
                                == -1
                        {
                            defense_b += 1;
                        }
                    }

                    a -= 1;
                    b += 1;
                }
            }
        }
    }

    #[wasm_bindgen]
    pub fn get_state(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self).unwrap()
    }
}

#[wasm_bindgen]
pub fn zkmain() {
    zkwasm_rust_sdk::dbg!("generating zkwasm");

    let mut hasher = Sha256::new();

    // TODO

    let msghash = hasher.finalize();
    print!("ToK application with command hash {:?}\n", msghash);
    zkwasm_rust_sdk::dbg!("ToK application with command hash {:?}\n", msghash);
}
