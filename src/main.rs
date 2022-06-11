#![allow(unused_must_use)]

use std::io::BufRead;
#[cfg(test)]
use std::io::Cursor;

#[derive(Debug, PartialEq)]
struct Puzzle {
    a: Vec<usize>,
    b: Vec<usize>,
    c: Vec<usize>
}


fn main() {
    let mut puzzles = read_puzzles(&mut std::io::stdin().lock());
    puzzles.iter_mut()
        .map(|puzzle| println!("{}", solve_puzzle(puzzle)))
        .count();
}


fn read_puzzle<R>(input: &mut R) -> Puzzle
    where R: BufRead
{
    let mut buf = String::new();
    
    // Find out how big each Vec needs to be
    input.read_line(&mut buf);
    let n = buf.trim().parse().unwrap();

    // Make the puzzle
    let mut puzzle = Puzzle {
        a: Vec::with_capacity(n),
        b: Vec::with_capacity(n),
        c: Vec::with_capacity(n),
    };

    // Fill the puzzle Vecs
    let mut fill_vec = |v: &mut Vec<usize>| {
        buf.clear();
        input.read_line(&mut buf);
        for n in buf.trim().split(' ').map(|ns|{ns.parse().unwrap()}) {
            v.push(n);
        }
    };

    fill_vec(&mut puzzle.a);
    fill_vec(&mut puzzle.b);
    fill_vec(&mut puzzle.c);

    puzzle
}


fn read_puzzles<R>(input: &mut R) -> Vec<Puzzle>
    where R: BufRead
{
    let mut buf = String::new();
    
    // Find out how many puzzles to read
    input.read_line(&mut buf);
    let n = buf.trim().parse().unwrap();

    // Read n puzzles
    (0..n).map( |_|{read_puzzle(input)} ).collect()
}


fn solve_puzzle(puzzle: &mut Puzzle) -> u64 {
    let n =  puzzle.a.len();

    // Turn 1-indexed values into 0-indexed and record values of "a"
    let mut a_lookup = vec![0;n];
    for i in 0..n {
        puzzle.a[i] -= 1;
        puzzle.b[i] -= 1;
        a_lookup[puzzle.a[i]] = i; 
    }

    // Identify each index with a loop ID
    let mut loop_ids = vec![0;n];
    let mut current_loop_id = 0;
    for i in 0..n {
        if loop_ids[i] != 0 || puzzle.a[i] == puzzle.b[i] { 
            continue;
        }

        current_loop_id += 1;
        let mut current_loop_ind = i;
        while loop_ids[current_loop_ind] != current_loop_id {
            loop_ids[current_loop_ind] = current_loop_id;
            current_loop_ind = a_lookup[puzzle.b[current_loop_ind]];
        }
    }
    let loop_count = current_loop_id;

    // Figure out how many of the loops are already fully constrained
    let mut loop_is_bound = vec![false; loop_count];
    for i in 0..n {
        if puzzle.c[i] != 0 && loop_ids[i] != 0 {
            loop_is_bound[loop_ids[i]-1] = true;
        }
    }
    let free_loop_count = loop_is_bound
        .iter()
        .filter(|&&bound| !bound)
        .count();

    // Final answer is (2^free_loop_count) % (10^9 + 7)
    let two:u64 = 2;
    two.pow(free_loop_count as u32) % 1_000_000_007
}

#[test]
fn test_sample_input() {
    let mut input = Cursor::new(include_str!("../test_input.txt"));
    let mut puzzles = read_puzzles(&mut input);
    let solutions = puzzles.iter_mut()
        .map(|puzzle| solve_puzzle(puzzle))
        .collect::<Vec<u64>>();
    assert_eq![solutions, vec![4,1,2,2,1,8,1,2,2]]
}

#[test]
fn test_read_puzzles() {
    let mut input = Cursor::new(
    "2\n \
    2\n \
    1 2\n \
    2 1\n \
    0 0\n \
    1\n \
    1\n \
    1\n \
    1");

    assert_eq!(read_puzzles(&mut input), vec![
        Puzzle {a: vec![1,2], b: vec![2,1], c: vec![0,0]},
        Puzzle {a: vec![1],   b: vec![1],   c: vec![1]}
    ]);
}
