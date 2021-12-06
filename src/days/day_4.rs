use array2d::Array2D;
use itermore::IterMore;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Game {
    moves: Vec<u32>,
    boards: Vec<Board>,
}

#[derive(Debug)]
struct Board {
    data: Array2D<u32>,
}

fn find_on<T>(arr: &Array2D<T>, elem: T) -> Vec<(usize, usize)>
where
    T: PartialEq<T> + Clone,
{
    arr.elements_row_major_iter()
        .enumerate()
        .filter(|(_, x)| **x == elem)
        .map(|(i, _)| (i % arr.num_columns(), i / arr.num_columns()))
        .collect::<Vec<_>>()
}

impl Game {
    fn new<F>(file: &mut F) -> Game
    where
        F: BufRead,
    {
        // read rules
        let mut buf = Vec::new();
        file.read_until(b'\n', &mut buf);

        let rules = String::from_utf8(buf)
            .unwrap()
            .split(',')
            .map(|num| u32::from_str_radix(num.trim(), 10).unwrap())
            .collect::<Vec<_>>();

        let mut buf = Vec::new();
        // read boards
        file.read_to_end(&mut buf);

        let boards_str = String::from_utf8(buf).unwrap();
        let boards = boards_str
            .split_whitespace()
            .map(|s| u32::from_str_radix(s.trim(), 10).unwrap())
            .chunks::<25>()
            .map(|flat_arr| Board {
                data: Array2D::from_row_major(&flat_arr, 5, 5),
            })
            .collect::<Vec<_>>();

        Game {
            moves: rules,
            boards,
        }
    }

    fn winning_score(&self) -> Option<usize> {
        let move_size = self.boards.len();
        let mut board_data = self
            .boards
            .iter()
            .map(|board| (board, Array2D::filled_with(false, 5, 5)))
            .collect::<Vec<_>>();

        for called in &self.moves {
            board_data.iter_mut().for_each(|(board, ref mut flags)| {
                find_on(&board.data, *called).iter().for_each(|index| {
                    flags[*index] = true;
                });
            });
        }

        None
    }
}

pub fn solution_part1(file: &mut File) {
    let game = Game::new(&mut BufReader::new(file));
    game.winning_score();

    println!("{:?}", find_on(&game.boards[0].data, 63));
    todo!()
}
