use rand::{RngExt, rng};
use std::env::args;

type Interval = usize;

fn is_interval(i: usize) -> bool {
    return (i > 0) && (i < 8);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Motion(i8);

impl Motion {
    fn new(motion: i8) -> Option<Self> {
        if [-8, -6, -5, -4, -3, -2, 1, 2, 3, 4, 5, 6, 8].contains(&motion) {
            Some(Self(motion))
        } else {
            None
        }
    }

    fn interval(self) -> Interval {
        (match self.0 {
            ..0 => self.0 + 9,
            1..8 => self.0,
            8.. => self.0 - 7,
            0 => unreachable!(),
        }) as Interval
    }

    fn displacement(self) -> i8 {
        match self.0 {
            1.. => self.0 - 1,
            ..0 => self.0 + 1,
            0 => unreachable!(),
        }
    }

    fn from_displacement(displacement: i8) -> Option<Self> {
        Self::new(match displacement {
            0.. => displacement + 1,
            ..0 => displacement - 1,
        })
    }
}

// struct Pitch {
//     letter: Interval,
//     number: usize,
// }

fn main() {
    let mut args = args().skip(1);

    let mut canon_interval: Interval = 5;
    let mut cantus_firmus_str: String = String::from("2 2 3 -2 1 -2 1 2 1 -2 -2 -2");
    let mut verbose: bool = false;

    while let Some(flag) = args.next() {
        match flag.as_str() {
            "-i" | "--interval" => {
                canon_interval = args.next().unwrap().parse().unwrap();
            }
            "-c" | "--cantus-firmus" => {
                cantus_firmus_str = args.next().unwrap();
            }
            "-v" | "--verbose" => {
                verbose = true;
            }
            "-h" | "--help" => {
                println!("Usage: ");
                return;
            }
            _ => {
                eprintln!("Unknown flag: {flag}");
                std::process::exit(1);
            }
        }
    }

    println!("canon_interval = {}", canon_interval);

    assert!(is_interval(canon_interval));

    let cantus_firmus: Vec<Motion> = cantus_firmus_str
        .split_whitespace()
        .map(|s| s.parse::<i8>().unwrap())
        .map(|m| Motion::new(m).unwrap())
        .collect();

    println!("cantus_firmus = {:?}", cantus_firmus);

    let motion_count = cantus_firmus.len();

    let allowed_intervals: [Vec<Interval>; 7] = {
        let mut intervals = [
            vec![1, 3, 5, 6],
            vec![6],
            vec![1, 3, 5],
            vec![1, 6],
            vec![3, 5],
            vec![1, 3, 6],
            vec![5],
        ];

        if canon_interval == 2 {
            intervals[3] = vec![3, 6];
        }

        intervals.rotate_right(canon_interval - 1);

        intervals
    };

    let forbidden_motions: Vec<Motion> = [
        vec![-2, 2],
        vec![-8, -6, 1, 3, 8],
        vec![-5, 2, 4],
        vec![-6, -4, 3, 5],
        vec![-5, -3, 4, 6],
        vec![-4, -2, 5],
        vec![-8, -3, 1, 6, 8],
    ][canon_interval - 1]
        .iter()
        .map(|&m| Motion::new(m).unwrap())
        .collect();

    let forbidden_successive_motions: Vec<Motion> = [
        vec![-8, -5, 1, 5, 8],
        vec![-4, 2, 6],
        vec![-6, -3, 3],
        vec![-5, -2, 4, 8],
        vec![-8, -4, 2, 5],
        vec![-3, 3, 6],
        vec![-6, -2, 4],
    ][canon_interval - 1]
        .iter()
        .map(|&m| Motion::new(m).unwrap())
        .collect();

    let forbidden_successive_diagonal_intervals: Vec<Interval> = [
        vec![1, 5, 8],
        vec![4, 7],
        vec![3, 6],
        vec![2, 5],
        vec![1, 4, 8],
        vec![3, 7],
        vec![2, 6],
    ][canon_interval - 1]
        .clone();

    let mut rng = rng();
    let mut candidate_intervals: Vec<Interval>;
    let mut random_index: usize;
    let mut chosen_intervals: Vec<Interval> = Vec::with_capacity(motion_count);
    let mut chosen_motions: Vec<Motion> = Vec::with_capacity(motion_count - 1);
    let mut success: bool;
    let mut candidate_interval: Interval;
    let mut candidate_displacement: i8;
    let mut candidate_motion: Motion;
    let mut previous_diagonal_interval: Interval;
    let mut candidate_diagonal_interval: Interval;
    let mut j: i32 = 0;

    for i in 0..motion_count {
        if verbose {
            println!("\ni = {}", i);
        }
        candidate_intervals = allowed_intervals[cantus_firmus[i].interval() - 1].clone();
        if verbose {
            println!("candidate_intervals = {:?}", candidate_intervals);
        }
        if i == 0 {
            random_index = rng.random_range(0..candidate_intervals.len());
            chosen_intervals.push(candidate_intervals[random_index]);
        } else {
            success = false;
            if verbose {
                j = 0;
            }
            while candidate_intervals.len() > 0 {
                if verbose {
                    println!("    j = {}", j);
                }
                random_index = rng.random_range(0..candidate_intervals.len());
                candidate_interval = candidate_intervals.swap_remove(random_index);
                if verbose {
                    println!("    candidate_interval = {}", candidate_interval);
                }
                candidate_displacement = cantus_firmus[i - 1].displacement()
                    + ((candidate_interval as i8) - 1)
                    - ((chosen_intervals[i - 1] as i8) - 1);
                candidate_motion = Motion::from_displacement(candidate_displacement).unwrap();
                if verbose {
                    println!("    candidate_motion = {:?}", candidate_motion);
                }
                success = !forbidden_motions.contains(&candidate_motion);
                if i > 0 {
                    previous_diagonal_interval = (
                        ((chosen_intervals[i - 1] as i8) - 1)
                        - cantus_firmus[i - 1].displacement()
                    ) as Interval;
                    candidate_diagonal_interval = (
                        ((candidate_interval as i8) - 1)
                        - cantus_firmus[i].displacement()
                    ) as Interval;
                    success &= !((candidate_diagonal_interval == previous_diagonal_interval)
                        && (forbidden_successive_diagonal_intervals
                            .contains(&candidate_diagonal_interval)));
                }
                if i > 1 {
                    success &= !((candidate_motion == chosen_motions[i - 2])
                        && (forbidden_successive_motions.contains(&candidate_motion)));
                }
                if success {
                    chosen_intervals.push(candidate_interval);
                    chosen_motions.push(candidate_motion);
                    break;
                }
                if verbose {
                    j += 1;
                }
            }
            assert!(success);
        }
        if verbose {
            println!();
            println!("chosen_intervals = {:?}", chosen_intervals);
            println!("chosen_motions = {:?}", chosen_motions);
        }
    }

    if !verbose {
        println!("chosen_intervals = {:?}", chosen_intervals);
        println!("chosen_motions = {:?}", chosen_motions);
    }
}
