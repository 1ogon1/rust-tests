use near_sdk::json_types::I64;
use robot_simulator::*;

#[test]
fn test_direction() {
    let robot = Robot::new(I64(0), I64(0), Direction::East);

    assert_eq!(robot.direction(), &Direction::East);
}
