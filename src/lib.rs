use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};
use std::collections::{HashMap, HashSet};
use near_sdk::json_types::{U128, ValidAccountId};
use std::env::set_current_dir;
use serde::{Serialize, Deserialize};

near_sdk::setup_alloc!();

pub type Bet = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct Account {
    account_id: AccountId,
    deposit: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct SicBoResult {
    account_id: AccountId,
    dices: Vec<u8>,
    total_winning: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct RouletteResult {
    account_id: AccountId,
    roulette_value: u8,
    total_winning: Balance,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    house: Balance,
    users: UnorderedMap<AccountId, Account>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            house: 0,
            users: UnorderedMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn deposit(&mut self) -> Balance {
        let account_id = env::predecessor_account_id();

        let user = self.users.get(&account_id.clone().to_string());
        let deposit: Balance = env::attached_deposit();
        if let Some(mut user) = user {
            user.deposit += deposit;
            self.users.insert(&account_id, &user);
            user.deposit
        } else {
            let user = Account { account_id: account_id.clone(), deposit: deposit };
            self.users.insert(&account_id, &user);
            user.deposit
        }
    }

    pub fn withdraw(&mut self, amount: Balance) -> Balance {
        let account_id = env::predecessor_account_id();

        let user = self.users.get(&account_id.clone().to_string());
        if let Some(mut user) = user {
            user.deposit -= amount;
            Promise::new(account_id.clone()).transfer(amount);
            self.users.insert(&account_id, &user);
            user.deposit
        } else {
            panic!("Account does not exist");
        }
    }

    pub fn get_account(&self, account_id: ValidAccountId) -> Account {
        self.users.get(&account_id.to_string()).expect("Account Id does not exist")
    }

    // Total slots
    pub fn play_sicbo(&mut self, bets: HashMap<Bet, U128>) -> SicBoResult {
        let account_id = env::predecessor_account_id();
        let mut account = self.users.get(&account_id.to_string()).expect("Account Id does not exist");

        let mut total: u128 = 0;
        let dices: Vec<u8> = roll_dices();
        let mut total_winning: u128 = 0;

        for (bet, balance) in bets {
            let point = check_point_sicbo(&bet, dices.clone());
            total += balance.0;
            total_winning += point as u128 * balance.0;
        }

        assert!(
            total <= account.deposit,
            "Account deposit does not sufficient for the bet"
        );

        account.deposit = account.deposit - total + total_winning;
        self.house = self.house + total - total_winning;
        self.users.insert(&account_id, &account);

        SicBoResult {
            account_id: account_id,
            dices: dices,
            total_winning: total_winning,
        }
    }

    pub fn play_roulette(&mut self, bets: HashMap<Bet, U128>) -> RouletteResult {
        let account_id = env::predecessor_account_id();
        let mut account = self.users.get(&account_id.to_string()).expect("Account Id does not exist");

        let mut total: u128 = 0;
        let roulette_value: u8 = roulette_spin();
        let mut total_winning: u128 = 0;
        for (bet, balance) in bets {
            let point = check_point_roulette(&bet, roulette_value);
            total += balance.0;
            total_winning += point as u128 * balance.0;
        }

        assert!(
            total <= account.deposit,
            "Account deposit does not sufficient for the bet"
        );

        account.deposit = account.deposit - total + total_winning;
        self.house = self.house + total - total_winning;
        self.users.insert(&account_id, &account);

        RouletteResult {
            account_id,
            roulette_value,
            total_winning,
        }
    }

    pub fn play_dice(&mut self, roll_under: u8, bet: U128) -> DiceResult {
        assert!(roll_under > 1 && roll_under < 97, "roll_under guess is not at the valid range");
        let account_id = env::predecessor_account_id();
        let mut account = self.users.get(&account_id.to_string()).expect("Account Id does not exist");

        assert!(bet <= account.deposit, "Account deposit does not sufficient for the bet");

        let mut total: u128 = 0;
        let dice_value: u8 = dice_100_roll();
        if dice_value < roll_under {
            let times: f64 = 98.50/(roll_under-1);
        }
    }
}

fn check_point_sicbo(bet: &str, dices: Vec<u8>) -> u32 {
    match bet {
        "small" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all >= 4 && sum_all <= 10 {
                return 1;
            }
        }
        "big" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all >= 11 && sum_all <= 17 {
                return 1;
            }
        }
        "double_1" => {
            if dices[0] == dices[1] && dices[0] == 1 {
                return 10;
            } else if dices[0] == dices[1] && dices[0] == 1 {
                return 10;
            } else if dices[1] == dices[2] && dices[1] == 1 {
                return 10;
            }
        }
        "double_2" => {
            if dices[0] == dices[1] && dices[0] == 2 {
                return 10;
            } else if dices[0] == dices[1] && dices[0] == 2 {
                return 10;
            } else if dices[1] == dices[2] && dices[1] == 2 {
                return 10;
            }
        }
        "double_3" => {
            if dices[0] == dices[1] && dices[0] == 3 {
                return 10;
            } else if dices[0] == dices[1] && dices[0] == 3 {
                return 10;
            } else if dices[1] == dices[2] && dices[1] == 3 {
                return 10;
            }
        }
        "double_4" => {
            if dices[0] == dices[1] && dices[0] == 4 {
                return 10;
            } else if dices[0] == dices[1] && dices[0] == 4 {
                return 10;
            } else if dices[1] == dices[2] && dices[1] == 4 {
                return 10;
            }
        }
        "double_5" => {
            if dices[0] == dices[1] && dices[0] == 5 {
                return 10;
            } else if dices[0] == dices[1] && dices[0] == 5 {
                return 10;
            } else if dices[1] == dices[2] && dices[1] == 5 {
                return 10;
            }
        }
        "double_6" => {
            if dices[0] == dices[1] && dices[0] == 6 {
                return 10;
            } else if dices[0] == dices[1] && dices[0] == 6 {
                return 10;
            } else if dices[1] == dices[2] && dices[1] == 6 {
                return 10;
            }
        }
        "triple_any" => {
            if dices[0] == dices[1] && dices[0] == dices[2] {
                return 30;
            }
        }
        "triple_1" => {
            if dices[0] == dices[1] && dices[0] == dices[2] && dices[0] == 1 {
                return 180;
            }
        }
        "triple_2" => {
            if dices[0] == dices[1] && dices[0] == dices[2] && dices[0] == 2 {
                return 180;
            }
        }
        "triple_3" => {
            if dices[0] == dices[1] && dices[0] == dices[2] && dices[0] == 3 {
                return 180;
            }
        }
        "triple_4" => {
            if dices[0] == dices[1] && dices[0] == dices[2] && dices[0] == 4 {
                return 180;
            }
        }
        "triple_5" => {
            if dices[0] == dices[1] && dices[0] == dices[2] && dices[0] == 5 {
                return 180;
            }
        }
        "triple_6" => {
            if dices[0] == dices[1] && dices[0] == dices[2] && dices[0] == 6 {
                return 180;
            }
        }
        "sum_4" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all == 4 {
                return 62;
            }
        }
        "sum_5" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all == 5 {
                return 31;
            }
        }
        "sum_6" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all == 6 {
                return 18;
            }
        }
        "sum_7" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all == 7 {
                return 12;
            }
        }
        "sum_8" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all == 8 {
                return 8;
            }
        }
        "sum_9" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all == 9 {
                return 7;
            }
        }
        "sum_10" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all == 10 {
                return 6;
            }
        }
        "sum_11" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all == 11 {
                return 6;
            }
        }
        "sum_12" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all == 12 {
                return 7;
            }
        }
        "sum_13" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all == 13 {
                return 8;
            }
        }
        "sum_14" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all == 14 {
                return 12;
            }
        }
        "sum_15" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all == 15 {
                return 18;
            }
        }
        "sum_16" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all == 16 {
                return 31;
            }
        }
        "sum_17" => {
            let sum_all: u8 = dices.iter().sum();
            if sum_all == 17 {
                return 62;
            }
        }
        "comb_1_2" => {
            let mut cond: HashSet<u8> = HashSet::new();
            cond.insert(1);
            cond.insert(2);

            for dice in dices {
                cond.remove(&dice);
            }

            if cond.is_empty() {
                return 6;
            }
        }
        "comb_1_3" => {
            let mut cond: HashSet<u8> = HashSet::new();
            cond.insert(1);
            cond.insert(3);

            for dice in dices {
                cond.remove(&dice);
            }

            if cond.is_empty() {
                return 6;
            }
        }
        "comb_1_4" => {
            let mut cond: HashSet<u8> = HashSet::new();
            cond.insert(1);
            cond.insert(4);

            for dice in dices {
                cond.remove(&dice);
            }

            if cond.is_empty() {
                return 6;
            }
        }
        "comb_1_5" => {
            let mut cond: HashSet<u8> = HashSet::new();
            cond.insert(1);
            cond.insert(5);

            for dice in dices {
                cond.remove(&dice);
            }

            if cond.is_empty() {
                return 6;
            }
        }
        "comb_1_6" => {
            let mut cond: HashSet<u8> = HashSet::new();
            cond.insert(1);
            cond.insert(6);

            for dice in dices {
                cond.remove(&dice);
            }

            if cond.is_empty() {
                return 6;
            }
        }
        "comb_2_3" => {
            let mut cond: HashSet<u8> = HashSet::new();
            cond.insert(2);
            cond.insert(3);

            for dice in dices {
                cond.remove(&dice);
            }

            if cond.is_empty() {
                return 6;
            }
        }
        "comb_2_4" => {
            let mut cond: HashSet<u8> = HashSet::new();
            cond.insert(2);
            cond.insert(4);

            for dice in dices {
                cond.remove(&dice);
            }

            if cond.is_empty() {
                return 6;
            }
        }
        "comb_2_5" => {
            let mut cond: HashSet<u8> = HashSet::new();
            cond.insert(2);
            cond.insert(5);

            for dice in dices {
                cond.remove(&dice);
            }

            if cond.is_empty() {
                return 6;
            }
        }
        "comb_2_6" => {
            let mut cond: HashSet<u8> = HashSet::new();
            cond.insert(2);
            cond.insert(6);

            for dice in dices {
                cond.remove(&dice);
            }

            if cond.is_empty() {
                return 6;
            }
        }
        "comb_3_4" => {
            let mut cond: HashSet<u8> = HashSet::new();
            cond.insert(3);
            cond.insert(4);

            for dice in dices {
                cond.remove(&dice);
            }

            if cond.is_empty() {
                return 6;
            }
        }
        "comb_3_5" => {
            let mut cond: HashSet<u8> = HashSet::new();
            cond.insert(3);
            cond.insert(5);

            for dice in dices {
                cond.remove(&dice);
            }

            if cond.is_empty() {
                return 6;
            }
        }
        "comb_3_6" => {
            let mut cond: HashSet<u8> = HashSet::new();
            cond.insert(3);
            cond.insert(6);

            for dice in dices {
                cond.remove(&dice);
            }

            if cond.is_empty() {
                return 6;
            }
        }
        "comb_4_5" => {
            let mut cond: HashSet<u8> = HashSet::new();
            cond.insert(4);
            cond.insert(5);

            for dice in dices {
                cond.remove(&dice);
            }

            if cond.is_empty() {
                return 6;
            }
        }
        "comb_4_6" => {
            let mut cond: HashSet<u8> = HashSet::new();
            cond.insert(4);
            cond.insert(6);

            for dice in dices {
                cond.remove(&dice);
            }

            if cond.is_empty() {
                return 6;
            }
        }
        "comb_5_6" => {
            let mut cond: HashSet<u8> = HashSet::new();
            cond.insert(5);
            cond.insert(6);

            for dice in dices {
                cond.remove(&dice);
            }

            if cond.is_empty() {
                return 6;
            }
        }
        "single_1" => {
            let mut total: u32 = 0;
            for dice in dices {
                if dice == 1 {
                    total += 1;
                }
            }
            return total;
        }
        "single_2" => {
            let mut total: u32 = 0;
            for dice in dices {
                if dice == 2 {
                    total += 1;
                }
            }
            return total;
        }
        "single_3" => {
            let mut total: u32 = 0;
            for dice in dices {
                if dice == 3 {
                    total += 1;
                }
            }
            return total;
        }
        "single_4" => {
            let mut total: u32 = 0;
            for dice in dices {
                if dice == 4 {
                    total += 1;
                }
            }
            return total;
        }
        "single_5" => {
            let mut total: u32 = 0;
            for dice in dices {
                if dice == 5 {
                    total += 1;
                }
            }
            return total;
        }
        "single_6" => {
            let mut total: u32 = 0;
            for dice in dices {
                if dice == 6 {
                    total += 1;
                }
            }
            return total;
        }
        _ => {
            return 0;
        }
    }
    return 0;
}

fn check_point_roulette(bet: &str, roulette_value: u8) -> u32 {
    let roulette_value_str = roulette_value.to_string();
    if roulette_value_str == bet {
        return 35;
    } else if bet.contains('|') { // split
        let mut split = bet.split('|').collect::<Vec<&str>>();
        let mut diff: i32 = 0;
        let mut count = 0;
        let mut win = 0;
        let mut numbers: HashSet<i32> = HashSet::new();
        for (pos, num) in split.iter().enumerate() {
            count += 1;
            if pos == 0 {
                diff += num.parse::<i32>().unwrap();
            } else if pos == 1 {
                diff = (diff - num.parse::<i32>().unwrap()).abs();
            }

            numbers.insert(num.parse::<i32>().unwrap());
            if num.to_string() == roulette_value_str {
                win = 1;
            }
        }

        if count == 2 {
            assert!(diff == 3 || diff == 1 || (diff == 2 && numbers.contains(&2) && numbers.contains(&0)), "ERR_SPLIT_NOT_VALID");

            return win * 17;
        } else if count == 3 {
            assert!(
                (numbers.contains(&1) && numbers.contains(&2) && numbers.contains(&3)) ||
                (numbers.contains(&4) && numbers.contains(&5) && numbers.contains(&6)) ||
                (numbers.contains(&7) && numbers.contains(&8) && numbers.contains(&9)) ||
                (numbers.contains(&10) && numbers.contains(&11) && numbers.contains(&12)) ||
                (numbers.contains(&13) && numbers.contains(&14) && numbers.contains(&15)) ||
                (numbers.contains(&16) && numbers.contains(&17) && numbers.contains(&18)) ||
                (numbers.contains(&19) && numbers.contains(&20) && numbers.contains(&21)) ||
                (numbers.contains(&22) && numbers.contains(&23) && numbers.contains(&24)) ||
                (numbers.contains(&25) && numbers.contains(&26) && numbers.contains(&27)) ||
                (numbers.contains(&28) && numbers.contains(&29) && numbers.contains(&30)) ||
                (numbers.contains(&31) && numbers.contains(&32) && numbers.contains(&33)) ||
                (numbers.contains(&34) && numbers.contains(&35) && numbers.contains(&36)),
                "ERR_STREET_NOT_VALID"
            );

            return win * 11;
        } else if count == 4 {
            assert!(
                (numbers.contains(&1) && numbers.contains(&2) && numbers.contains(&4) && numbers.contains(&5)) ||
                (numbers.contains(&4) && numbers.contains(&5) && numbers.contains(&7) && numbers.contains(&8)) ||
                (numbers.contains(&7) && numbers.contains(&8) && numbers.contains(&10) && numbers.contains(&11)) ||
                (numbers.contains(&10) && numbers.contains(&11) && numbers.contains(&13) && numbers.contains(&14)) ||
                (numbers.contains(&13) && numbers.contains(&14) && numbers.contains(&16) && numbers.contains(&17)) ||
                (numbers.contains(&16) && numbers.contains(&17) && numbers.contains(&19) && numbers.contains(&20)) ||
                (numbers.contains(&19) && numbers.contains(&20) && numbers.contains(&22) && numbers.contains(&23)) ||
                (numbers.contains(&22) && numbers.contains(&23) && numbers.contains(&25) && numbers.contains(&26)) ||
                (numbers.contains(&25) && numbers.contains(&26) && numbers.contains(&28) && numbers.contains(&29)) ||
                (numbers.contains(&28) && numbers.contains(&29) && numbers.contains(&31) && numbers.contains(&32)) ||
                (numbers.contains(&31) && numbers.contains(&32) && numbers.contains(&34) && numbers.contains(&35)) ||
                (numbers.contains(&2) && numbers.contains(&5) && numbers.contains(&3) && numbers.contains(&6)) ||
                (numbers.contains(&5) && numbers.contains(&6) && numbers.contains(&8) && numbers.contains(&9)) ||
                (numbers.contains(&8) && numbers.contains(&9) && numbers.contains(&11) && numbers.contains(&12)) ||
                (numbers.contains(&11) && numbers.contains(&12) && numbers.contains(&14) && numbers.contains(&15)) ||
                (numbers.contains(&14) && numbers.contains(&15) && numbers.contains(&17) && numbers.contains(&18)) ||
                (numbers.contains(&17) && numbers.contains(&18) && numbers.contains(&20) && numbers.contains(&21)) ||
                (numbers.contains(&20) && numbers.contains(&21) && numbers.contains(&23) && numbers.contains(&24)) ||
                (numbers.contains(&23) && numbers.contains(&24) && numbers.contains(&26) && numbers.contains(&27)) ||
                (numbers.contains(&26) && numbers.contains(&27) && numbers.contains(&29) && numbers.contains(&30)) ||
                (numbers.contains(&29) && numbers.contains(&30) && numbers.contains(&32) && numbers.contains(&33)) ||
                (numbers.contains(&32) && numbers.contains(&33) && numbers.contains(&35) && numbers.contains(&36)),
                "ERR_CORNER_NOT_VALID"
            );

            return win * 8;
        } else if count == 6 {
            assert!(
                (numbers.contains(&1) && numbers.contains(&2) && numbers.contains(&3) && numbers.contains(&4) && numbers.contains(&5) && numbers.contains(&6)) ||
                (numbers.contains(&7) && numbers.contains(&8) && numbers.contains(&9) && numbers.contains(&4) && numbers.contains(&5) && numbers.contains(&6)) ||
                (numbers.contains(&7) && numbers.contains(&8) && numbers.contains(&9) && numbers.contains(&10) && numbers.contains(&11) && numbers.contains(&12)) ||
                (numbers.contains(&13) && numbers.contains(&14) && numbers.contains(&15) && numbers.contains(&10) && numbers.contains(&11) && numbers.contains(&12)) ||
                (numbers.contains(&13) && numbers.contains(&14) && numbers.contains(&15) && numbers.contains(&16) && numbers.contains(&17) && numbers.contains(&18)) ||
                (numbers.contains(&19) && numbers.contains(&20) && numbers.contains(&21) && numbers.contains(&16) && numbers.contains(&17) && numbers.contains(&18)) ||
                (numbers.contains(&19) && numbers.contains(&20) && numbers.contains(&21) && numbers.contains(&22) && numbers.contains(&23) && numbers.contains(&24)) ||
                (numbers.contains(&25) && numbers.contains(&26) && numbers.contains(&27) && numbers.contains(&22) && numbers.contains(&23) && numbers.contains(&24)) ||
                (numbers.contains(&25) && numbers.contains(&26) && numbers.contains(&27) && numbers.contains(&28) && numbers.contains(&29) && numbers.contains(&30)) ||
                (numbers.contains(&31) && numbers.contains(&32) && numbers.contains(&33) && numbers.contains(&28) && numbers.contains(&29) && numbers.contains(&30)) ||
                (numbers.contains(&31) && numbers.contains(&32) && numbers.contains(&33) && numbers.contains(&34) && numbers.contains(&35) && numbers.contains(&36)),
                "ERR_SIX_LINE_NOT_VALID"
            );

            return win * 5;
        } else if count == 12 {
            assert!(
                (
                    numbers.contains(&1) &&
                    numbers.contains(&4) &&
                    numbers.contains(&7) &&
                    numbers.contains(&10) &&
                    numbers.contains(&13) &&
                    numbers.contains(&16) &&
                    numbers.contains(&19) &&
                    numbers.contains(&22) &&
                    numbers.contains(&25) &&
                    numbers.contains(&28) &&
                    numbers.contains(&31) &&
                    numbers.contains(&34)
                ) ||
                (
                    numbers.contains(&2) &&
                    numbers.contains(&5) &&
                    numbers.contains(&8) &&
                    numbers.contains(&11) &&
                    numbers.contains(&14) &&
                    numbers.contains(&17) &&
                    numbers.contains(&20) &&
                    numbers.contains(&23) &&
                    numbers.contains(&26) &&
                    numbers.contains(&29) &&
                    numbers.contains(&32) &&
                    numbers.contains(&35)
                ) ||
                (
                    numbers.contains(&3) &&
                    numbers.contains(&6) &&
                    numbers.contains(&9) &&
                    numbers.contains(&12) &&
                    numbers.contains(&15) &&
                    numbers.contains(&18) &&
                    numbers.contains(&21) &&
                    numbers.contains(&24) &&
                    numbers.contains(&27) &&
                    numbers.contains(&30) &&
                    numbers.contains(&33) &&
                    numbers.contains(&36)
                ),
                "ERR_COLUMN_BET_NOT_VALID"
            );

            return win * 2;
        } else {
            return 0;
        }
    } else if bet == "1st_12" {
        if roulette_value <= 12 {
            return 2;
        }
    } else if bet == "2nd_12" {
        if roulette_value <= 24 && roulette_value > 12 {
            return 2;
        }
    } else if bet == "3rd_12" {
        if roulette_value <= 36 && roulette_value > 24 {
            return 2;
        }
    } else if bet == "low" {
        if roulette_value <= 18 {
            return 1;
        }
    } else if bet == "high" {
        if roulette_value > 18 && roulette_value <= 36 {
            return 1;
        }
    } else if bet == "even" {
        if roulette_value % 2 == 0 {
            return 1;
        }
    } else if bet == "odd"{
        if roulette_value % 2 == 1 {
            return 1;
        }
    } else if bet == "red" {
        if vec![1,3,5,7,9,12,14,16,18,19,21,23,25,27,30,32,34,36].contains(&(roulette_value as i32)) {
            return 1;
        }
    } else if bet == "black" {
        if vec![2,4,6,8,10,11,13,15,17,20,22,24,26,28,29,31,33,35].contains(&(roulette_value as i32)) {
            return 1;
        }
    }
    0
}

fn roll_dices() -> Vec<u8> {
    let mut dices: Vec<u8> = Vec::new();
    for _ in 0..3 {
        let seed_num = get_random_number(0);
        dices.push((seed_num % 6 + 1) as u8);
    }
    return dices;
}

fn roulette_spin() -> u8 {
    let seed_num = get_random_number(0);
    return (seed_num % 37) as u8;
}

fn dice_100_roll() -> u8 {
    let seed_num = get_random_number(0);
    return (seed_num % 100) as u8;
}

fn get_random_number(shift_amount: u32) -> u32 {
    let mut seed = env::random_seed();
    let seed_len = seed.len();
    let mut arr: [u8; 4] = Default::default();
    seed.rotate_left(shift_amount as usize % seed_len);
    arr.copy_from_slice(&seed[..4]);
    u32::from_le_bytes(arr)
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = StatusMessage::default();
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".to_string()).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = StatusMessage::default();
        assert_eq!(None, contract.get_status("francis.near".to_string()));
    }
}
