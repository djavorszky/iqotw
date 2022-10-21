/// Interview question:
///
/// Let’s say you have n doors that start out as closed. With the first pass across the doors,
/// you toggle every door open. With the second pass, you toggle every second door.
/// With the third, every third door, and so on.
///
/// Write a function that takes in an integer numberOfPasses,
/// and returns how many doors are open after the number of passes.

pub fn pass_doors(n: usize, number_of_passes: usize) -> usize {
    let mut doors = vec![false; n];

    for step in 1..=number_of_passes {
        for idx in (step - 1..n).step_by(step) {
            doors[idx] = !doors[idx];
        }
    }

    doors.iter().filter(|b| **b).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive_works() {
        assert_eq!(pass_doors(7, 3), 4);
    }
}
