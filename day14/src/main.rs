use nom::{
    bytes::complete::tag,
    character::complete::{anychar, i32, multispace1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let result = day14_part1(&contents, (101, 103));
    println!("Day14 part 1 result: {result}");
}

fn day14_part1(input: &str, size: Vector) -> usize {
    let (_, robots) = read_input(input).unwrap();
    let final_robots = robots
        .into_iter()
        .map(|robot| advance_robot(robot, size, 100))
        .collect::<Vec<Robot>>();
    get_robots_per_quadrant(&final_robots, size)
        .iter()
        .product()
}

fn advance_robot(mut robot: Robot, size: Vector, times: i32) -> Robot {
    robot.position.0 = (robot.position.0 + (robot.velocity.0 * times)) % size.0;
    if robot.position.0 < 0 {
        robot.position.0 += size.0;
    }
    robot.position.1 = (robot.position.1 + (robot.velocity.1 * times)) % size.1;
    if robot.position.1 < 0 {
        robot.position.1 += size.1;
    }
    robot
}

fn get_robots_per_quadrant(final_robots: &[Robot], size: Vector) -> [usize; 4] {
    let x_middle = size.0 / 2;
    let x_separation = size.0 % 2;
    let y_middle = size.1 / 2;
    let y_separation = size.1 % 2;
    let mut first_quadrant = 0;
    let mut second_quadrant = 0;
    let mut third_quadrant = 0;
    let mut fourth_quadrant = 0;
    for robot in final_robots {
        if robot.position.0 < x_middle {
            if robot.position.1 < y_middle {
                first_quadrant += 1;
            } else if robot.position.1 >= y_middle + y_separation {
                fourth_quadrant += 1;
            }
        } else if robot.position.0 >= x_middle + x_separation {
            if robot.position.1 < y_middle {
                second_quadrant += 1;
            } else if robot.position.1 >= y_middle + y_separation {
                third_quadrant += 1;
            }
        }
    }
    [
        first_quadrant,
        second_quadrant,
        third_quadrant,
        fourth_quadrant,
    ]
}

type Vector = (i32, i32);

#[derive(Clone, Debug)]
struct Robot {
    position: Vector,
    velocity: Vector,
}

fn read_input(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(multispace1, read_robot)(input)
}

fn read_robot(input: &str) -> IResult<&str, Robot> {
    let (input, (position, velocity)) =
        separated_pair(read_vector, multispace1, read_vector)(input)?;

    Ok((input, Robot { position, velocity }))
}

fn read_vector(input: &str) -> IResult<&str, Vector> {
    separated_pair(preceded(preceded(anychar, tag("=")), i32), tag(","), i32)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_correct_output_for_test_input() {
        let contents = fs::read_to_string("test_input").unwrap();
        let result = day14_part1(&contents, (11, 7));
        assert_eq!(result, 12);
    }

    #[test]
    fn part1_correct_output_for_input() {
        let contents = fs::read_to_string("input").unwrap();
        let result = day14_part1(&contents, (101, 103));
        assert_eq!(result, 216027840);
    }
}
