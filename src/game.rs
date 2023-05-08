use colored::Colorize;
use rand::Rng;

use crate::{
    communication::{
        self, get_input_with_exit, many_commands_with_description, print,
    },
    gamedata::{self, calculate_dmg},
};

pub fn new() -> Option<gamedata::GameState> {
    clearscreen::clear().unwrap();
    let mut game = gamedata::GameState::new();
    communication::print("new_game.txt");
    println!("press continue to enter camp...");
    communication::get_input(false);
    if !enter_camp(&mut game) {
        return None;
    }
    return Some(game);
}

pub fn resume(mut game: gamedata::GameState) -> Option<gamedata::GameState> {
    clearscreen::clear().unwrap();
    communication::print("resume_game.txt");
    println!("press continue to enter camp...");
    communication::get_input(false);
    if !enter_camp(&mut game) {
        return None;
    }
    return Some(game);
}

fn enter_camp(game: &mut gamedata::GameState) -> bool {
    clearscreen::clear().unwrap();
    communication::print("enter_camp.txt");
    loop {
        println!("{}", "What would you like to do?".yellow());
        match communication::many_commands_with_exit(
            &vec![
                "Shop".to_string(),
                "Cook".to_string(),
                "Dungeon".to_string(),
                "Inspect".to_string(),
                "Exit".to_string(),
            ],
            true,
        ) {
            Some(choice) => {
                if choice == 0 {
                    enter_shop(game);
                } else if choice == 1 {
                    enter_cooking(game);
                } else if choice == 2 {
                    if !enter_dungeon(game) {
                        enter_gameover(game);
                        return false;
                    }
                } else if choice == 3 {
                    enter_inspection(game);
                } else {
                    break;
                }
            }
            None => break,
        }
    }
    return true;
}

fn enter_dungeon(game: &mut gamedata::GameState) -> bool {
    fn shuffle_rooms() -> (Vec<gamedata::RoomType>, Vec<String>) {
        let rooms = gamedata::get_rooms();
        let mut room_names = gamedata::rooms_into_strings(&rooms);
        room_names.push("Flee".to_string());
        room_names.push("Eat".to_string());
        room_names.push("Inspect".to_string());
        (rooms, room_names)
    }
    clearscreen::clear().unwrap();
    communication::print("enter_dungeon.txt");
    let mut room_count = 0;
    let (mut _rooms, mut room_names) = shuffle_rooms();
    loop {
        clearscreen::clear().unwrap();
        println!("Room number {}", room_count.to_string().red());
        println!("{}", "Which way?".yellow());
        println!("You have {} stamina.", game.stamina.to_string().green());
        println!("You have {} food.", game.food.quantity.to_string().green());
        println!("You have {} money.", game.money.to_string().green());
        let choice = communication::many_commands(&room_names, true);
        match room_names[choice].as_str() {
            "Flee" => {
                if room_count > game.stamina {
                    println!("You don't have enough stamina to flee!");
                    continue;
                }
                game.stamina -= (room_count - game.gear.final_stats().speed).max(0);
                clearscreen::clear().unwrap();
                communication::print("fled.txt");
                println!("press enter to continue...");
                // wait for input
                communication::get_input(false);
                break true;
            }
            "Eat" => {
                println!("How much food would you like to eat?");
                println!("You have {} food.", game.food.quantity.to_string().green());
                match communication::get_input_with_exit(false) {
                    Some(input) => {
                        let amount = input.parse::<i32>().unwrap_or(0);
                        if amount > game.food.quantity {
                            println!("You don't have that much food!");
                            continue;
                        }
                        game.food.quantity -= amount;
                        game.stamina += amount - 1;
                        println!("You ate {} food.", amount.to_string().green());
                    }
                    None => continue,
                }
            }
            "Inspect" => {
                clearscreen::clear().unwrap();
                enter_inspection(game);
                println!("press enter to continue...");
                communication::get_input(false);
            }
            "Small Room" => {
                room_count += 1;
                game.stamina -= 1;
                if !enter_dungeon_room(game, gamedata::RoomType::Small) {
                    return false;
                }
                (_rooms, room_names) = shuffle_rooms();
            }
            "Big Room" => {
                room_count += 1;
                game.stamina -= 2;
                if !enter_dungeon_room(game, gamedata::RoomType::Big) {
                    return false;
                }
                (_rooms, room_names) = shuffle_rooms();
            }
            "Treasure Room" => {
                room_count += 1;
                game.stamina -= 5;
                if !enter_dungeon_room(game, gamedata::RoomType::Treasure) {
                    return false;
                }
                (_rooms, room_names) = shuffle_rooms();
            }
            "Escape passage" => {
                clearscreen::clear().unwrap();
                communication::print("escape_passage.txt");
                println!("press enter to continue...");
                // wait for input
                communication::get_input(false);
                break true;
            }
            "Final Room" => {
                game.stamina -= 1;
                if !enter_dungeon_room(game, gamedata::RoomType::Final) {
                    return false;
                }
                clearscreen::clear().unwrap();
                communication::print("dungeon/win.txt");
                println!("press enter to continue...");
                // wait for input
                communication::get_input(false);
                break true;
            }
            _ => {
                unreachable!();
            }
        }
    }
}

fn enter_dungeon_room(game: &mut gamedata::GameState, room: gamedata::RoomType) -> bool {
    clearscreen::clear().unwrap();
    match room {
        gamedata::RoomType::Small => {
            communication::print("dungeon/small_room.txt");
            println!("press enter to continue...");
            communication::get_input(false);
            // 1/2 chance to encounter a monster
            let mut rng = rand::thread_rng();
            let random = rng.gen_range(0..2);
            if random == 0 {
                println!("You encountered a {}!", "Small Monster".red());
                println!("press enter to continue...");
                communication::get_input(false);
                if !enter_combat(gamedata::MonsterTypes::Small, game, true) {
                    return false;
                }
            } else {
                println!("You were lucky and didn't encounter a monster!");
            }
            // 1/3 chance to find a small treasure chest
            let random = rng.gen_range(0..3);
            if random == 0 {
                enter_treasure(game, false);
            }
        }
        gamedata::RoomType::Big => {
            communication::print("dungeon/big_room.txt");
            println!("press enter to continue...");
            communication::get_input(false);
            // small monster or big monster + big treasure or big monster
            let mut rng = rand::thread_rng();
            let random = rng.gen_range(0..3);
            if random == 0 {
                println!("You encountered a {}!", "Small Monster".red());
                println!("press enter to continue...");
                communication::get_input(false);
                if !enter_combat(gamedata::MonsterTypes::Small, game, true) {
                    return false;
                }
            } else if random == 1 {
                println!("You encountered a {}!", "Big Monster".red());
                println!("press enter to continue...");
                communication::get_input(false);
                if !enter_combat(gamedata::MonsterTypes::Big, game, true) {
                    return false;
                }
                enter_treasure(game, true);
            } else {
                println!("You encountered a {}!", "Big Monster".red());
                println!("press enter to continue...");
                communication::get_input(false);
                if !enter_combat(gamedata::MonsterTypes::Big, game, true) {
                    return false;
                }
            }
        }
        gamedata::RoomType::Treasure => {
            communication::print("dungeon/treasure_room.txt");
            println!("press enter to continue...");
            communication::get_input(false);
            // big or small treasure
            let mut rng = rand::thread_rng();
            let random = rng.gen_range(0..2);
            if random == 0 {
                enter_treasure(game, false);
            } else {
                enter_treasure(game, true);
            }
            println!("press enter to continue...");
            communication::get_input(false);
        }
        gamedata::RoomType::Final => {
            communication::print("dungeon/final_room.txt");
            println!("press enter to continue...");
            communication::get_input(false);
            // big monster
            if !enter_combat(gamedata::MonsterTypes::Big, game, false) {
                return false;
            }
            enter_treasure(game, true);
            game.won += 1;
        }
        _ => unreachable!("This room type is not allowed!"),
    }
    true
}

fn enter_treasure(game: &mut gamedata::GameState, big: bool) {
    let mut rng = rand::thread_rng();
    if !big {
        println!("{}", "You found a small treasure chest!".yellow());
        let random = rng.gen_range(0..3);
        if random == 0 {
            get_potion(game);
        } else if random == 1 {
            println!("{}", "There is a small sack of coins".yellow());
            let random = rng.gen_range(0..game.level + 10);
            game.money += random;
            println!("You gained {} money!", random.to_string().green());
        } else {
            println!("{}", "Unfortunately, the chest is empty!".yellow());
        }
    } else {
        println!("{}", "You found a big treasure chest!".yellow());
        for _ in 0..3 {
            let random = rng.gen_range(0..2);
            if random == 0 {
                get_potion(game);
            }
            if random == 1 {
                println!("{}", "There is a big sack of coins".yellow());
                let random = rng.gen_range(0..game.level + 20);
                game.money += random;
                println!("You gained {} money!", random.to_string().green());
            }
        }
    }
}

fn get_potion(game: &mut gamedata::GameState) {
    // get random number between 0 and 1
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..3);
    if random == 0 {
        println!("{}", "You found a potion of stamina!".yellow());
        let random = rng.gen_range(3..10);
        game.stamina += random;
        println!("You gained {} stamina!", random.to_string().green());
    } else if random == 1 {
        println!("{}", "You found a potion of saturation!".yellow());
        let random = rng.gen_range(2..6);
        game.food.quantity += random;
        println!("You gained {} food!", random.to_string().green());
    } else {
        println!("{}", "You found a potion of wealth!".yellow());
        let random = rng.gen_range(5..10);
        game.money += random;
        println!("You gained {} money!", random.to_string().green());
    }
}

fn enter_combat(
    kind: gamedata::MonsterTypes,
    game: &mut gamedata::GameState,
    can_flee: bool,
) -> bool {
    let mut monster = gamedata::Monster::new(game.level, kind.clone());
    clearscreen::clear().unwrap();
    match kind {
        gamedata::MonsterTypes::Big => {
            communication::print("dungeon/big_monster.txt");
        }
        gamedata::MonsterTypes::Small => {
            communication::print("dungeon/small_monster.txt");
        }
    }
    println!("press enter to continue...");
    communication::get_input(false);
    let mut health = game.gear.final_stats().health;
    let mut should_attack = true;
    loop {
        if should_attack {
            println!("{}", "Monster attacks!".red());
            let damage = calculate_dmg(monster.damage, 0, game.gear.final_stats().armor);
            println!("You took {} damage!", damage.to_string().red());
            health -= damage;
            if health <= 0 {
                println!("{}", "You died!".red());
                break false;
            }
            if let Some(msg) = game.gear.take_damage() {
                println!("Your {} is destroyed!", msg.magenta());
            }
        }
        should_attack = true;
        if game.stamina <= 1 {
            clearscreen::clear().unwrap();
            communication::print("combat/no_stamina.txt");
            println!("press enter to continue...");
            communication::get_input(false);
            break false;
        }
        println!("You have {} stamina.", game.stamina.to_string().green());
        println!("You have {} health.", health.to_string().green());
        println!("{}:\n{}", "monster".on_red(), monster.to_string());
        println!(
            "{}:\n{}",
            "you".on_green(),
            game.gear.final_stats().to_string()
        );
        println!("{}", "What would you like to do?".yellow());
        let mut commands = vec![
            ("Attack".to_string(), "1 stamina".to_string()),
            ("Eat".to_string(), "1 stamina".to_string()),
            ("Give up".to_string(), "0 stamina".to_string()),
        ];
        if can_flee {
            commands.push(("Flee".to_string(), "5 stamina".to_string()));
            commands.push(("Hide".to_string(), "3 stamina".to_string()));
        }
        match many_commands_with_description(&commands, false) {
            0 => {
                let damage = calculate_dmg(
                    game.gear.final_stats().damage,
                    game.gear.final_stats().luck,
                    monster.armor,
                );
                println!("You dealt {} damage!", damage.to_string().green());
                monster.health -= damage;
                game.stamina -= 1;
                if let Some(msg) = game.gear.weapon_take_dmg() {
                    println!("Your {} is destroyed!", msg.magenta());
                }
                if monster.health <= 0 {
                    println!("{}", "You won!".green());
                    println!(
                        "You gained {} experience!",
                        (monster.reward * 10).to_string().cyan()
                    );
                    game.get_exp(monster.reward * 10);
                    println!(
                        "Your reward is {} money.",
                        monster.reward.to_string().green()
                    );
                    game.money += monster.reward;
                    // wait for input
                    println!("press enter to continue...");
                    communication::get_input(false);
                    break true;
                }
            }
            1 => {
                println!("How much food would you like to eat?");
                loop {
                    let amount = get_input_with_exit(false);
                    match amount {
                        Some(amount) => match amount.parse::<i32>() {
                            Ok(amount) => {
                                if amount < 1 {
                                    println!("Please enter a positive number!");
                                    continue;
                                }
                                if amount > game.food.quantity {
                                    println!("You don't have that much food!");
                                    continue;
                                }
                                game.food.quantity -= amount;
                                game.stamina += amount - 1;
                                health += (amount as f32 * 1.5) as i32;
                                println!("You ate {} food.", amount.to_string().green());
                                println!("You have {} stamina.", game.stamina.to_string().green());
                                break;
                            }
                            Err(_) => {
                                println!("Please enter a number!");
                                continue;
                            }
                        },
                        None => {
                            should_attack = false;
                            break;
                        }
                    }
                }
            }
            2 => {
                clearscreen::clear().unwrap();
                print("dungeon/give_up.txt");
                println!("press enter to continue...");
                communication::get_input(false);
                break false;
            }
            3 => {
                if game.stamina < 7 {
                    println!("You don't have enough stamina!");
                    should_attack = false;
                    continue;
                }
                game.stamina -= 5;
                clearscreen::clear().unwrap();
                print("dungeon/flee.txt");
                println!("press enter to continue...");
                communication::get_input(false);
                break true;
            }
            4 => {
                // chance to hide is max 50% based on luck
                if game.stamina < 4 {
                    println!("You don't have enough stamina!");
                    should_attack = false;
                    continue;
                }
                game.stamina -= 3;
                let chance = game.gear.final_stats().luck.max(100) / 2;
                let roll = rand::thread_rng().gen_range(0..100);
                if roll < chance {
                    clearscreen::clear().unwrap();
                    print("dungeon/hide.txt");
                    println!("press enter to continue...");
                    communication::get_input(false);
                    break true;
                } else {
                    println!("You failed to hide!");
                }
            }
            _ => {}
        }
    }
}

fn enter_gameover(game: &mut gamedata::GameState) {
    clearscreen::clear().unwrap();
    print("gameover.txt");
    println!("Your final money: {}", game.money.to_string().green());
    println!(
        "Number of successful runs: {}",
        game.won.to_string().green()
    );
    println!(
        "Your final stats:\n{}",
        game.gear.final_stats().to_string().green()
    );
    println!("press enter to continue...");
    communication::get_input(false);
}

pub fn enter_shop(game: &mut gamedata::GameState) {
    println!("{}", "Welcome to the shop!".yellow());
    loop {
        println!("You have {} money.", game.money.to_string().green());
        println!("What would you like to buy?");
        match communication::many_commands_with_exit(
            &vec![
                "Gear".to_string(),
                "Food".to_string(),
                "Sell gear".to_string(),
                "Inspect".to_string(),
                "Exit".to_string(),
            ],
            true,
        ) {
            Some(choice) => {
                if choice == 0 {
                    enter_gear_shop(game);
                } else if choice == 1 {
                    enter_food_shop(game);
                } else if choice == 2 {
                    enter_sell_shop(game);
                } else if choice == 3 {
                    enter_inspection(game);
                    println!("");
                }else {
                    break;
                }
            }
            None => break,
        }
    }
}

fn enter_food_shop(game: &mut gamedata::GameState) {
    if game.shop.food == 0 {
        println!("{}", "The shop is out of food!".red());
        return;
    }
    println!("{}", "How much food would you like to buy?".yellow());
    println!("The shop has {} food.", game.shop.food.to_string().green());
    let mut input = communication::get_input_with_exit(false).unwrap();
    let mut amount = input.parse::<i32>().unwrap_or(0);
    while amount > game.shop.food {
        println!(
            "The shop only has {} food!",
            game.shop.food.to_string().green()
        );
        input = communication::get_input_with_exit(false).unwrap();
        amount = input.parse::<i32>().unwrap_or(0);
    }
    game.shop.food -= amount;
    game.food.quantity += amount;
    game.money -= amount * 2;
    println!(
        "You bought {} food for {} money.",
        amount.to_string().green(),
        (amount * 2).to_string().green()
    );
}

fn enter_gear_shop(game: &mut gamedata::GameState) {
    game.shop.update(game.level);
    if game.shop.gear.is_empty() {
        println!("The shop is out of gear!");
        return;
    }
    fn print_sortiment(game: &mut gamedata::GameState) {
        println!("{}", "What would you like to buy?".yellow());
        if game.shop.gear.weapon.is_some() {
            let kind = if game.shop.gear.weapon.as_ref().unwrap().cost > game.money {
                "Weapon".on_red()
            } else {
                "Weapon".on_green()
            };
            println!(
                "{}:\n{}",
                kind,
                game.shop.gear.weapon.as_ref().unwrap().to_string()
            );
        }
        if game.shop.gear.body.is_some() {
            let kind = if game.shop.gear.body.as_ref().unwrap().cost > game.money {
                "Body".on_red()
            } else {
                "Body".on_green()
            };
            println!(
                "{}:\n{}",
                kind,
                game.shop.gear.body.as_ref().unwrap().to_string()
            );
        }
        if game.shop.gear.head.is_some() {
            let kind = if game.shop.gear.head.as_ref().unwrap().cost > game.money {
                "Head".on_red()
            } else {
                "Head".on_green()
            };
            println!(
                "{}:\n{}",
                kind,
                game.shop.gear.head.as_ref().unwrap().to_string()
            );
        }
        if game.shop.gear.legs.is_some() {
            let kind = if game.shop.gear.legs.as_ref().unwrap().cost > game.money {
                "Legs".on_red()
            } else {
                "Legs".on_green()
            };
            println!(
                "{}:\n{}",
                kind,
                game.shop.gear.legs.as_ref().unwrap().to_string()
            );
        }
    }
    loop {
        print_sortiment(game);
        let mut gear = Vec::new();
        if game.shop.gear.weapon.is_some() {
            gear.push("Weapon".to_string());
        }
        if game.shop.gear.body.is_some() {
            gear.push("Body".to_string());
        }
        if game.shop.gear.head.is_some() {
            gear.push("Head".to_string());
        }
        if game.shop.gear.legs.is_some() {
            gear.push("Legs".to_string());
        }
        gear.push("Exit".to_string());
        match communication::many_commands_with_exit(&gear, true) {
            Some(choice) => match gear[choice].as_str() {
                "Weapon" => {
                    if game.shop.gear.weapon.as_ref().unwrap().cost > game.money {
                        println!("You don't have enough money!");
                        continue;
                    }
                    if let Some(wp) = &game.gear.weapon {
                        let cost = gamedata::calculate_cost(
                            wp.cost,
                            wp.durability,
                            wp.original_durability,
                        );
                        println!("{}", "You already have a weapon!".red());
                        println!("Would you like to sell your weapon?");
                        println!("You will get {} money.", cost.to_string().green());
                        println!("{}", game.gear.weapon.as_ref().unwrap().cmp(&game.shop.gear.weapon.as_ref().unwrap()));
                        if communication::yesno() {
                            game.money += cost;
                            game.gear.weapon = None;
                        } else {
                            continue;
                        }
                    }
                    game.money -= game.shop.gear.weapon.as_ref().unwrap().cost;
                    game.gear.weapon = game.shop.gear.weapon.clone();
                    game.shop.gear.weapon = None;
                    println!(
                        "You bought weapon for {} money.",
                        game.gear.weapon.as_ref().unwrap().cost.to_string().green()
                    );
                }
                "Body" => {
                    if game.shop.gear.body.as_ref().unwrap().cost > game.money {
                        println!("You don't have enough money!");
                        continue;
                    }
                    if let Some(bp) = &game.gear.body {
                        let cost = gamedata::calculate_cost(
                            bp.cost,
                            bp.durability,
                            bp.original_durability,
                        );
                        println!("{}", "You already have a body piece!".red());
                        println!("Would you like to sell your body piece?");
                        println!("You will get {} money.", cost.to_string().green());
                        println!("{}", game.gear.body.as_ref().unwrap().cmp(&game.shop.gear.body.as_ref().unwrap()));
                        if communication::yesno() {
                            game.money += cost;
                            game.gear.body = None;
                        } else {
                            continue;
                        }
                    }
                    game.money -= game.shop.gear.body.as_ref().unwrap().cost;
                    game.gear.body = game.shop.gear.body.clone();
                    game.shop.gear.body = None;
                    println!(
                        "You bought the body piece for {} money.",
                        game.gear.body.as_ref().unwrap().cost.to_string().green()
                    );
                }
                "Head" => {
                    if game.shop.gear.head.as_ref().unwrap().cost > game.money {
                        println!("You don't have enough money!");
                        continue;
                    }
                    if let Some(hp) = &game.gear.head {
                        let cost = gamedata::calculate_cost(
                            hp.cost,
                            hp.durability,
                            hp.original_durability,
                        );
                        println!("{}", "You already have a head piece!".red());
                        println!("Would you like to sell your head piece?");
                        println!("You will get {} money.", cost.to_string().green());
                        println!("{}", game.gear.head.as_ref().unwrap().cmp(&game.shop.gear.head.as_ref().unwrap()));
                        if communication::yesno() {
                            game.money += cost;
                            game.gear.head = None;
                        } else {
                            continue;
                        }
                    }
                    game.money -= game.shop.gear.head.as_ref().unwrap().cost;
                    game.gear.head = game.shop.gear.head.clone();
                    game.shop.gear.head = None;
                    println!(
                        "You bought head piece for {} money.",
                        game.gear.head.as_ref().unwrap().cost.to_string().green()
                    );
                }
                "Legs" => {
                    if game.shop.gear.legs.as_ref().unwrap().cost > game.money {
                        println!("You don't have enough money!");
                        continue;
                    }
                    if let Some(lp) = &game.gear.legs {
                        let cost = gamedata::calculate_cost(
                            lp.cost,
                            lp.durability,
                            lp.original_durability,
                        );
                        println!("{}", "You already have a leg piece!".red());
                        println!("Would you like to sell your leg piece?");
                        println!("You will get {} money.", cost.to_string().green());
                        println!("{}", game.gear.legs.as_ref().unwrap().cmp(&game.shop.gear.legs.as_ref().unwrap()));
                        if communication::yesno() {
                            game.money += cost;
                            game.gear.legs = None;
                        } else {
                            continue;
                        }
                    }
                    game.money -= game.shop.gear.legs.as_ref().unwrap().cost;
                    game.gear.legs = game.shop.gear.legs.clone();
                    game.shop.gear.legs = None;
                    println!(
                        "You bought leg piece for {} money.",
                        game.gear.legs.as_ref().unwrap().cost.to_string().green()
                    );
                }
                _ => break,
            },
            None => break,
        }
        println!("You have {} money left.", game.money.to_string().green());
    }
}

fn enter_sell_shop(game: &mut gamedata::GameState) {
    if game.gear.is_empty() {
        println!("You have nothing to sell!");
        return;
    }
    loop {
        println!("{}", "What would you like to sell?".yellow());
        let mut options = Vec::new();
        if game.gear.weapon.is_some() {
            println!(
                "Weapon:\n{}",
                game.gear.weapon.as_ref().unwrap().to_string()
            );
            options.push("Weapon".to_string());
        }
        if game.gear.body.is_some() {
            println!("Body:\n{}", game.gear.body.as_ref().unwrap().to_string());
            options.push("Body".to_string());
        }
        if game.gear.head.is_some() {
            println!("Head:\n{}", game.gear.head.as_ref().unwrap().to_string());
            options.push("Head".to_string());
        }
        if game.gear.legs.is_some() {
            println!("Legs:\n{}", game.gear.legs.as_ref().unwrap().to_string());
            options.push("Legs".to_string());
        }
        options.push("Exit".to_string());
        match communication::many_commands_with_exit(&options, true) {
            Some(choice) => match options[choice].as_str() {
                "Weapon" => {
                    game.money += game.gear.weapon.as_ref().unwrap().cost;
                    game.gear.weapon = None;
                }
                "Body" => {
                    game.money += game.gear.body.as_ref().unwrap().cost;
                    game.gear.body = None;
                }
                "Head" => {
                    game.money += game.gear.head.as_ref().unwrap().cost;
                    game.gear.head = None;
                }
                "Legs" => {
                    game.money += game.gear.legs.as_ref().unwrap().cost;
                    game.gear.legs = None;
                }
                _ => return,
            },
            None => return,
        }
        println!("You have {} money.", game.money.to_string().green());
    }
}

fn enter_cooking(game: &mut gamedata::GameState) {
    println!("{}", "Welcome to the cooking station!".yellow());
    // check if the last batch is done
    if game.food.currently_cooking.is_some() {
        if game.food.cooking_end_time.unwrap() > std::time::Instant::now() {
            println!(
                "You are still cooking {} food.",
                game.food.currently_cooking.unwrap().to_string().green()
            );
            println!(
                "It will be done in {} seconds.",
                (game.food.cooking_end_time.unwrap() - std::time::Instant::now())
                    .as_secs()
                    .to_string()
                    .green()
            );
            return;
        } else {
            println!(
                "You finished cooking {} food!",
                game.food.currently_cooking.unwrap().to_string().green()
            );
            game.food.quantity += game.food.currently_cooking.unwrap() * 3;
            game.food.currently_cooking = None;
            game.food.cooking_end_time = None;
        }
    }

    if game.food.quantity == 0 {
        println!("You don't have any food!");
        return;
    }
    loop {
        println!("You have {} food.", game.food.quantity.to_string().green());
        println!("You have {} money", game.money.to_string().green());
        println!(
            "This means you can cook up to {} food",
            (game.money / 2).min(game.food.quantity).to_string().green()
        );
        println!("How much food would you like to cook? (2 coins per each)");
        let amount = communication::get_input(false).parse::<i32>().unwrap_or(0);
        if amount > game.food.quantity {
            println!("You don't have that much food!");
            continue;
        }
        if amount * 2 > game.money {
            println!("You don't have enough money!");
            continue;
        }
        if amount == 0 {
            break;
        }
        game.food.quantity -= amount;
        game.food.currently_cooking = Some(amount);
        game.food.cooking_end_time =
            Some(std::time::Instant::now() + std::time::Duration::from_secs(amount as u64 * 30));
        game.money -= amount * 2;
        println!("You started cooking {} food!", amount.to_string().green());
        println!(
            "Amount of {} will be ready in {} seconds.",
            (amount * 3).to_string().green(),
            (amount * 30).to_string().green()
        );
        println!("It costs you {} money.", (amount * 2).to_string().green());
        break;
    }
}

fn enter_inspection(game: &mut gamedata::GameState) {
    println!("{}", "I see you have decided to relax for a bit.".yellow());
    println!("Your level is {}, {} exp.", game.level.to_string().cyan(), game.exp.to_string().cyan());
    println!("You have {} food.", game.food.quantity.to_string().green());
    // check if you are cooking
    if game.food.currently_cooking.is_some() {
        if game.food.cooking_end_time.unwrap() > std::time::Instant::now() {
            println!(
                "You are still cooking {} food.",
                game.food.currently_cooking.unwrap().to_string().green()
            );
            println!(
                "It will be done in {} seconds.",
                (game.food.cooking_end_time.unwrap() - std::time::Instant::now())
                    .as_secs()
                    .to_string()
                    .green()
            );
        } else {
            println!(
                "You finished cooking {} food!",
                game.food.currently_cooking.unwrap().to_string().green()
            );
        }
    }
    println!("You have {} money.", game.money.to_string().green());
    println!("You have {} stamina.", game.stamina.to_string().green());
    println!("Your gear:");
    if game.gear.weapon.is_some() {
        println!(
            "{}:\n{}",
            "Weapon".on_green(),
            game.gear.weapon.as_ref().unwrap().to_string()
        );
    }
    if game.gear.body.is_some() {
        println!(
            "{}:\n{}",
            "Body".on_green(),
            game.gear.body.as_ref().unwrap().to_string()
        );
    }
    if game.gear.head.is_some() {
        println!(
            "{}:\n{}",
            "Head".on_green(),
            game.gear.head.as_ref().unwrap().to_string()
        );
    }
    if game.gear.legs.is_some() {
        println!(
            "{}:\n{}",
            "Legs".on_green(),
            game.gear.legs.as_ref().unwrap().to_string()
        );
    }
    println!(
        "{}:\n{}",
        "Your stats".on_cyan(),
        game.gear.final_stats().to_string()
    );
}
