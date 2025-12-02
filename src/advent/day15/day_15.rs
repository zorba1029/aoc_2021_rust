use crate::advent::day15::day_15_part1;
use crate::advent::day15::day_15_part1_faster;
use crate::advent::day15::day_15_part2;
use crate::advent::day15::day_15_part2_faster;
use crate::advent::day15::day_15_part2_faster_async;

pub fn do_day_15() {
    do_day15_part_one();
    // do_day15_part_two();
    // do_day15_part_two_faster();
    do_day15_part_two_faster_async();
}

pub fn do_day15_part_one() {
    day_15_part1::day_15_part_one();
}
pub fn do_day15_part_one_faster() {
    day_15_part1_faster::day_15_part_one();
}

pub fn do_day15_part_two() {
    day_15_part2::day_15_part_two();
}

pub fn do_day15_part_two_faster() {
    day_15_part2_faster::day_15_part_two();
}

pub fn do_day15_part_two_faster_async() {
    day_15_part2_faster_async::day_15_part_two();
}
