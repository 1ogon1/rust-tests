use near_sdk::json_types::{I64, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen,
};
use near_sdk::{env, AccountId, PanicOnDefault};

#[derive(PartialEq, Debug, BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[near_bindgen]
#[derive(PartialEq, BorshDeserialize, BorshSerialize, Deserialize, Serialize, PanicOnDefault)]
#[serde(crate = "near_sdk::serde")]
pub struct Robot {
    x: I64,
    y: I64,
    move_at: U64,
    owner_id: AccountId,
    direction: Direction,
}

#[near_bindgen]
impl Robot {
    #[init]
    pub fn new(x: I64, y: I64, direction: Direction) -> Self {
        Robot {
            x,
            y,
            direction,
            move_at: U64(env::block_timestamp()),
            owner_id: env::predecessor_account_id(),
        }
    }

    pub fn turn_right(&mut self) {
        match self.direction {
            Direction::North => self.set_direction(Direction::East),
            Direction::East => self.set_direction(Direction::South),
            Direction::South => self.set_direction(Direction::West),
            Direction::West => self.set_direction(Direction::North),
        };
    }

    pub fn turn_left(&mut self) {
        match self.direction {
            Direction::North => self.set_direction(Direction::West),
            Direction::East => self.set_direction(Direction::North),
            Direction::South => self.set_direction(Direction::East),
            Direction::West => self.set_direction(Direction::South),
        };
    }

    pub fn advance(&mut self) {
        match self.direction {
            Direction::North => self.y = I64(self.y.0 + 1),
            Direction::East => self.x = I64(self.x.0 + 1),
            Direction::South => self.y = I64(self.y.0 - 1),
            Direction::West => self.x = I64(self.x.0 - 1),
        };
    }

    pub fn instructions(&mut self, instructions: String) {
        for c in instructions.chars() {
            match c {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _ => panic!("ERR_INSTRUCTION_ACTION"),
            };
        }
    }

    pub fn position(&self) -> (i64, i64) {
        (self.x.0, self.y.0)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn last_move(&self) -> u64 {
        self.move_at.0
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn reset(&mut self) {
        self.x = I64(0);
        self.y = I64(0);
        self.direction = Direction::North;
    }

    fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
        self.move_at = U64(env::block_timestamp());
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use near_sdk::{
        test_utils::{accounts, VMContextBuilder},
        testing_env,
    };

    use super::*;

    #[test]
    fn robot_create() {
        let mut context = VMContextBuilder::new();
        let current_date =
            DateTime::parse_from_str("2022-01-01 00:00:00 +02:00", "%Y-%m-%d %H:%M:%S %z").unwrap();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .block_timestamp(current_date.timestamp() as u64)
            .build());

        let robot = Robot::new(I64(0), I64(0), Direction::East);
        println!("{}", robot.last_move());
        println!("{} - {}", accounts(0), robot.get_owner());

        assert_eq!(robot.get_owner(), accounts(0));
    }

    #[test]
    fn robot_ditection() {
        let mut context = VMContextBuilder::new();
        let current_date =
            DateTime::parse_from_str("2022-01-01 00:00:00 +02:00", "%Y-%m-%d %H:%M:%S %z").unwrap();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .block_timestamp(current_date.timestamp() as u64)
            .build());

        let mut robot = Robot::new(I64(0), I64(0), Direction::North);

        robot.turn_right();

        assert_eq!(robot.direction(), &Direction::East);
    }

    #[test]
    fn robot_reset() {
        let mut context = VMContextBuilder::new();
        let current_date =
            DateTime::parse_from_str("2022-01-01 00:00:00 +02:00", "%Y-%m-%d %H:%M:%S %z").unwrap();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .block_timestamp(current_date.timestamp() as u64)
            .build());

        let mut robot = Robot::new(I64(1), I64(2), Direction::East);

        robot.reset();

        assert_eq!(robot.direction(), &Direction::North);
        assert_eq!(robot.position(), (0, 0));
    }

    #[test]
    #[ignore]
    fn test_set_direction() {
        let mut context = VMContextBuilder::new();
        let current_date =
            DateTime::parse_from_str("2022-01-01 00:00:00 +02:00", "%Y-%m-%d %H:%M:%S %z").unwrap();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .block_timestamp(current_date.timestamp() as u64)
            .build());

        let mut robot = Robot::new(I64(0), I64(0), Direction::East);
        robot.set_direction(Direction::East);

        assert_eq!(robot.direction(), &Direction::East);
    }

    #[test]
    #[should_panic(expected = "ERR_INSTRUCTION")]
    fn test_instruction() {
        let mut context = VMContextBuilder::new();
        let current_date =
            DateTime::parse_from_str("2022-01-01 00:00:00 +02:00", "%Y-%m-%d %H:%M:%S %z").unwrap();

        testing_env!(context
            .predecessor_account_id(accounts(0))
            .block_timestamp(current_date.timestamp() as u64)
            .build());

        let mut robot = Robot::new(I64(0), I64(0), Direction::East);

        robot.instructions("LAARLAC".to_string());
    }
}
