use std::time;

use rand::Rng;

const START_MONEY: i32 = 15;
const START_FOOD: i32 = 10;
const START_STAMINA: i32 = 100;
const START_LVL: i32 = 1;

#[derive(Debug)]
pub struct GameState {
    pub gear: Gear,
    pub money: i32,
    pub food: FoodInfo,
    pub shop: Shop,
    pub stamina: i32,
    pub won: i32,
    pub level: i32,
    pub exp: i32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            gear: Gear {
                weapon: Some(Weapon::new(1)),
                body: None,
                head: None,
                legs: None,
            },
            money: START_MONEY,
            stamina: START_STAMINA,
            won: 0,
            level: START_LVL,
            exp: 33,
            food: FoodInfo {
                quantity: START_FOOD,
                currently_cooking: None,
                cooking_end_time: None,
            },
            shop: Shop {
                gear: Gear {
                    weapon: Some(Weapon::new(1)),
                    body: Some(Body::new(1)),
                    head: Some(Head::new(1)),
                    legs: Some(Legs::new(1)),
                },
                food: 1,
                last_update: 1,
            },
        }
    }
    pub fn get_exp(&mut self, exp: i32) {
        self.exp += exp;
        while self.exp >= 100 + self.level * 3 {
            self.exp -= 100 + self.level * 3;
            self.level += 1;
            println!("You leveled up! You are now level {}", self.level.to_string().green());
        }
    }
}

#[derive(Debug)]
pub struct ExplorationState {
    pub stamina: i32,
}

#[derive(Debug)]
pub struct Shop {
    pub gear: Gear,
    pub food: i32,
    pub last_update: i32,
}

impl Shop {
    pub fn update(&mut self, lvl: i32) {
        if self.last_update < lvl {
            self.last_update = lvl;
            self.food = lvl * 3;
            self.gear.weapon = Some(Weapon::new(lvl));
            self.gear.body = Some(Body::new(lvl));
            self.gear.head = Some(Head::new(lvl));
            self.gear.legs = Some(Legs::new(lvl));
        }
    }
}

#[derive(Debug)]
pub struct FoodInfo {
    pub quantity: i32,
    pub currently_cooking: Option<i32>,
    pub cooking_end_time: Option<time::Instant>,
}

#[derive(Debug)]
pub struct Gear {
    pub weapon: Option<Weapon>,
    pub body: Option<Body>,
    pub head: Option<Head>,
    pub legs: Option<Legs>,
}

impl Gear {
    pub fn is_empty(&self) -> bool {
        self.weapon.is_none() && self.body.is_none() && self.head.is_none() && self.legs.is_none()
    }
    pub fn final_stats(&self) -> Stats {
        let mut damage = 5;
        let mut luck = 0;
        let mut armor = 0;
        let mut speed = 0;
        let mut health = 100;
        if let Some(weapon) = &self.weapon {
            damage += weapon.damage;
            luck += weapon.luck;
        }
        if let Some(body) = &self.body {
            armor += body.armor;
            speed += body.speed;
            health += body.health;
            damage += body.damage;
        }
        if let Some(head) = &self.head {
            armor += head.armor;
            luck += head.luck;
            luck += head.luck;
        }
        if let Some(legs) = &self.legs {
            armor += legs.armor;
            speed += legs.speed;
            health += legs.health;
            luck += legs.luck;
        }
        Stats {
            damage,
            luck,
            armor,
            speed,
            health,
        }
    }
    pub fn take_damage(&mut self) -> Option<String> {
        // generate random number between 1 and 3
        let mut rng = rand::thread_rng();
        let rng = rng.gen_range(1..4);
        if rng == 1 {
            if let Some(body) = &mut self.body {
                body.durability -= 1;
                if body.durability == 0 {
                    self.body = None;
                    return Some("body armor".to_string());
                }
            }
        }
        if rng == 2 {
            if let Some(head) = &mut self.head {
                head.durability -= 1;
                if head.durability == 0 {
                    self.head = None;
                    return Some("head armor".to_string());
                }
            }
        }
        if rng == 3 {
            if let Some(legs) = &mut self.legs {
                legs.durability -= 1;
                if legs.durability == 0 {
                    self.legs = None;
                    return Some("legs armor".to_string());
                }
            }
        }
        return None;
    }
    pub fn weapon_take_dmg(&mut self) -> Option<String> {
        if let Some(weapon) = &mut self.weapon {
            weapon.durability -= 1;
            if weapon.durability == 0 {
                self.weapon = None;
                return Some("weapon".to_string())
            }
        }
        None
    }
}

pub struct Stats {
    pub damage: i32,
    pub luck: i32,
    pub armor: i32,
    pub speed: i32,
    pub health: i32,
}

impl Stats {
    pub fn to_string(&self) -> String {
        format!(
            " - Damage: {}\n - Luck: {}\n - Armor: {}\n - Speed: {}\n - Health: {}",
            self.damage.to_string().cyan(),
            self.luck.to_string().cyan(),
            self.armor.to_string().cyan(),
            self.speed.to_string().cyan(),
            self.health.to_string().cyan()
        )
    }
}

pub fn calculate_cost(original: i32, durability: i32, original_durability: i32) -> i32 {
    let ratio = durability as f32 / original_durability as f32;
    (original as f32 * ratio).round() as i32
}

#[derive(Debug, Clone)]
pub struct Weapon {
    pub damage: i32,
    pub luck: i32,
    pub durability: i32,
    pub original_durability: i32,
    pub cost: i32,
}

use colored::Colorize;

impl Weapon {
    pub fn new(player_level: i32) -> Self {
        // get random number between -5 and 5
        let budget = player_level + 10 + rand::thread_rng().gen_range(-5..6);
        let cost = player_level + 10 + rand::thread_rng().gen_range(-5..6);
        // get random number between 0 and budget
        let damage = rand::thread_rng().gen_range(0..budget);
        // get random number between 0 and budget - damage
        let luck = budget - damage;
        let durability = (player_level + 10 + cost - budget) * 3;
        Weapon {
            damage,
            luck,
            durability,
            original_durability: durability,
            cost,
        }
    }

    pub fn cmp(&self, other: &Self) -> String {
        let d1 = cmp_color(self.damage, other.damage);
        let l1 = cmp_color(self.luck, other.luck);
        let c1 = cmp_color_inverse(self.cost, other.cost);
        let dur1 = cmp_color(self.durability, other.durability);
        let d2 = cmp_color(other.damage, self.damage);
        let l2 = cmp_color(other.luck, self.luck);
        let c2 = cmp_color_inverse(other.cost, self.cost);
        let dur2 = cmp_color(other.durability, self.durability);
        format!(
            " - Damage: {} | {}\n - Luck: {} | {}\n - Durability: {} | {}\n - Cost: {} | {}",
            d1, d2, l1, l2, dur1, dur2, c1, c2
        )
        
    }
    pub fn to_string(&self) -> String {
        format!(
            " - Damage: {}\n - Luck: {}\n - Durability: {}\n - Cost: {}",
            self.damage.to_string().cyan(),
            self.luck.to_string().cyan(),
            self.durability.to_string().purple(),
            calculate_cost(self.cost, self.durability, self.original_durability)
                .to_string()
                .yellow()
        )
    }
}
fn cmp_color(a: i32, b: i32) -> colored::ColoredString {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => a.to_string().red(),
        std::cmp::Ordering::Equal => a.to_string().yellow(),
        std::cmp::Ordering::Greater => a.to_string().green(),
    }
}

fn cmp_color_inverse(a: i32, b: i32) -> colored::ColoredString {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => a.to_string().green(),
        std::cmp::Ordering::Equal => a.to_string().yellow(),
        std::cmp::Ordering::Greater => a.to_string().red(),
    }
}

#[derive(Debug, Clone)]
pub struct Body {
    pub armor: i32,
    pub health: i32,
    pub damage: i32,
    pub speed: i32,
    pub durability: i32,
    pub original_durability: i32,
    pub cost: i32,
}

impl Body {
    pub fn new(player_level: i32) -> Self {
        // get random number between -5 and 5
        let budget = player_level + 10 + rand::thread_rng().gen_range(-5..6);
        let cost = player_level + 10 + rand::thread_rng().gen_range(-5..6);
        // get random number between 0 and budget
        let armor = rand::thread_rng().gen_range(0..budget);
        // get random number between 0 and budget - armor
        let health = rand::thread_rng().gen_range(0..(budget - armor));
        // get random number between 0 and budget - armor - health
        let damage = rand::thread_rng().gen_range(0..(budget - armor - health));
        let speed = budget - armor - health - damage;
        let durability = player_level + 10 + cost - budget;
        Body {
            armor,
            health,
            damage,
            speed,
            durability,
            original_durability: durability,
            cost,
        }
    }
    pub fn to_string(&self) -> String {
        format!(
            " - Armor: {}\n - Health: {}\n - Damage: {}\n - Speed: {}\n - Durability: {}\n - Cost: {}",
            self.armor.to_string().cyan(),
            self.health.to_string().cyan(),
            self.damage.to_string().cyan(),
            self.speed.to_string().cyan(),
            self.durability.to_string().purple(),
            calculate_cost(self.cost, self.durability, self.original_durability)
                .to_string()
                .yellow()
        )
    }
    pub fn cmp(&self, other: &Self) -> String {
        let a1 = cmp_color(self.armor, other.armor);
        let h1 = cmp_color(self.health, other.health);
        let d1 = cmp_color(self.damage, other.damage);
        let s1 = cmp_color(self.speed, other.speed);
        let c1 = cmp_color_inverse(self.cost, other.cost);
        let dur1 = cmp_color(self.durability, other.durability);
        let a2 = cmp_color(other.armor, self.armor);
        let h2 = cmp_color(other.health, self.health);
        let d2 = cmp_color(other.damage, self.damage);
        let s2 = cmp_color(other.speed, self.speed);
        let c2 = cmp_color_inverse(other.cost, self.cost);
        let dur2 = cmp_color(other.durability, self.durability);
        format!(
            " - Armor: {} | {}\n - Health: {} | {}\n - Damage: {} | {}\n - Speed: {} | {}\n - Durability: {} | {}\n - Cost: {} | {}",
            a1, a2, h1, h2, d1, d2, s1, s2, dur1, dur2, c1, c2
        )
    }
}

#[derive(Debug, Clone)]
pub struct Head {
    pub luck: i32,
    pub armor: i32,
    pub damage: i32,
    pub durability: i32,
    pub original_durability: i32,
    pub cost: i32,
}

impl Head {
    pub fn new(player_level: i32) -> Self {
        // get random number between -5 and 5
        let budget = player_level + 10 + rand::thread_rng().gen_range(-5..6);
        let cost = player_level + 10 + rand::thread_rng().gen_range(-5..6);
        // get random number between 0 and budget
        let luck = rand::thread_rng().gen_range(0..budget);
        // get random number between 0 and budget - luck
        let armor = rand::thread_rng().gen_range(0..(budget - luck));
        // get random number between 0 and budget - luck - armor
        let damage = budget - luck - armor;
        let durability = player_level + 10 + cost - budget;
        Head {
            luck,
            armor,
            damage,
            durability,
            original_durability: durability,
            cost,
        }
    }
    pub fn to_string(&self) -> String {
        format!(
            " - Luck: {}\n - Armor: {}\n - Damage: {}\n - Durability: {}\n - Cost: {}",
            self.luck.to_string().cyan(),
            self.armor.to_string().cyan(),
            self.damage.to_string().cyan(),
            self.durability.to_string().purple(),
            calculate_cost(self.cost, self.durability, self.original_durability)
                .to_string()
                .yellow()
        )
    }
    pub fn cmp(&self, other: &Self) -> String {
        let l1 = cmp_color(self.luck, other.luck);
        let a1 = cmp_color(self.armor, other.armor);
        let d1 = cmp_color(self.damage, other.damage);
        let c1 = cmp_color_inverse(self.cost, other.cost);
        let dur1 = cmp_color(self.durability, other.durability);
        let l2 = cmp_color(other.luck, self.luck);
        let a2 = cmp_color(other.armor, self.armor);
        let d2 = cmp_color(other.damage, self.damage);
        let c2 = cmp_color_inverse(other.cost, self.cost);
        let dur2 = cmp_color(other.durability, self.durability);
        format!(
            " - Luck: {} | {}\n - Armor: {} | {}\n - Damage: {} | {}\n - Durability: {} | {}\n - Cost: {} | {}",
            l1, l2, a1, a2, d1, d2, dur1, dur2, c1, c2
        )
    }
}

#[derive(Debug, Clone)]
pub struct Legs {
    pub speed: i32,
    pub armor: i32,
    pub health: i32,
    pub luck: i32,
    pub durability: i32,
    pub original_durability: i32,
    pub cost: i32,
}

impl Legs {
    pub fn new(player_level: i32) -> Self {
        // get random number between -5 and 5
        let budget = player_level + 10 + rand::thread_rng().gen_range(-5..6);
        let cost = player_level + 10 + rand::thread_rng().gen_range(-5..6);
        // get random number between 0 and budget
        let speed = rand::thread_rng().gen_range(0..budget);
        // get random number between 0 and budget - speed
        let armor = rand::thread_rng().gen_range(0..(budget - speed));
        // get random number between 0 and budget - speed - armor
        let health = rand::thread_rng().gen_range(0..(budget - speed - armor));
        // get random number between 0 and budget - speed - armor - health
        let luck = budget - speed - armor - health;
        let durability = player_level + 10 + cost - budget;
        Legs {
            speed,
            armor,
            health,
            luck,
            durability,
            original_durability: durability,
            cost,
        }
    }
    pub fn to_string(&self) -> String {
        format!(
            " - Speed: {}\n - Armor: {}\n - Health: {}\n - Luck: {}\n - Durability: {}\n - Cost: {}",
            self.speed.to_string().cyan(),
            self.armor.to_string().cyan(),
            self.health.to_string().cyan(),
            self.luck.to_string().cyan(),
            self.durability.to_string().purple(),
            calculate_cost(self.cost, self.durability, self.original_durability)
                .to_string()
                .yellow()
        )
    }
    pub fn cmp(&self, other: &Self) -> String {
        let s1 = cmp_color(self.speed, other.speed);
        let a1 = cmp_color(self.armor, other.armor);
        let h1 = cmp_color(self.health, other.health);
        let l1 = cmp_color(self.luck, other.luck);
        let c1 = cmp_color_inverse(self.cost, other.cost);
        let dur1 = cmp_color(self.durability, other.durability);
        let s2 = cmp_color(other.speed, self.speed);
        let a2 = cmp_color(other.armor, self.armor);
        let h2 = cmp_color(other.health, self.health);
        let l2 = cmp_color(other.luck, self.luck);
        let c2 = cmp_color_inverse(other.cost, self.cost);
        let dur2 = cmp_color(other.durability, self.durability);
        format!(
            " - Speed: {} | {}\n - Armor: {} | {}\n - Health: {} | {}\n - Luck: {} | {}\n - Durability: {} | {}\n - Cost: {} | {}",
            s1, s2, a1, a2, h1, h2, l1, l2, dur1, dur2, c1, c2
        )
    }
}

#[derive(Debug)]
pub struct Monster {
    pub health: i32,
    pub damage: i32,
    pub armor: i32,
    pub reward: i32,
}

#[derive(Debug, Clone)]
pub enum MonsterTypes {
    Small,
    Big,
}

impl MonsterTypes {
    fn into_budget(&self) -> i32 {
        match self {
            MonsterTypes::Small => 2,
            MonsterTypes::Big => 5,
        }
    }
}

impl Monster {
    pub fn new(player_level: i32, kind: MonsterTypes) -> Self {
        let budget = player_level * kind.into_budget() + rand::thread_rng().gen_range(-5..6) + 5;
        let mut health = rand::thread_rng().gen_range(0..budget);
        let damage = rand::thread_rng().gen_range(0..(budget - health));
        let armor = budget - health - damage;
        let reward = budget / 2;
        health += player_level / 2 + 10;
        Monster {
            health,
            damage,
            armor,
            reward,
        }
    }
    pub fn to_string(&self) -> String {
        format!(
            " - Health: {}\n - Damage: {}\n - Armor: {}\n - Reward: {}",
            self.health.to_string().cyan(),
            self.damage.to_string().cyan(),
            self.armor.to_string().cyan(),
            self.reward.to_string().yellow()
        )
    }
}

#[derive(Debug)]
pub enum RoomType {
    Small,
    Big,
    Treasure,
    Escape,
    Final,
}

pub fn get_rooms() -> Vec<RoomType> {
    let mut rooms = Vec::new();
    let room_num = if rand::thread_rng().gen_range(0..11) > 3 {
        3
    } else {
        2
    };
    let mut rng = rand::thread_rng();
    let mut room_count = 0;
    while room_count < room_num {
        let room_type = rng.gen_range(0..70);
        if room_type < 10 {
            rooms.push(RoomType::Treasure);
        } else if room_type < 11 {
            rooms.push(RoomType::Escape);
        } else if room_type < 16 {
            rooms.push(RoomType::Final);
        } else if room_type < 40 {
            rooms.push(RoomType::Big);
        } else {
            rooms.push(RoomType::Small);
        }
        room_count += 1;
    }
    rooms
}

pub fn rooms_into_strings(rooms: &Vec<RoomType>) -> Vec<String> {
    let mut room_strings = Vec::new();
    for room in rooms {
        match room {
            RoomType::Small => room_strings.push("Small Room".to_string()),
            RoomType::Big => room_strings.push("Big Room".to_string()),
            RoomType::Treasure => room_strings.push("Treasure Room".to_string()),
            RoomType::Escape => room_strings.push("Escape passage".to_string()),
            RoomType::Final => room_strings.push("Final Room".to_string()),
        }
    }
    room_strings
}

pub fn calculate_dmg(damage: i32, luck: i32, armor: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let mut dmg = damage + rng.gen_range(-5..10);
    if rng.gen_range(0..101) < luck {
        dmg *= 2;
    }
    dmg = dmg * 70 / (70 + armor);
    dmg.max(0)
}