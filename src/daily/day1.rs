use crate::daily::common;

pub fn day1() {
    let path = "src/resources/day1.txt";
    let raw = common::read(path);
    let pos = run(&raw);
    println!("Final counts: {:?}", pos);
}

fn run(raw: &String) -> Position {
    let moves = _parse_moves(&raw);

    let mut pos = Position {
        cur_pos: 50,
        origin_count: 0,
        total_moves: 0,
        times_crossed_origin: 0,
    };
    for mv in &moves {
        pos.turn(mv);
    }
    return pos;
}

fn _parse_moves(raw: &String) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let lines = raw.lines();
    for line in lines {
        let mut iterator = line.char_indices();
        let is_right = iterator.next().expect("Expected direction").1 == 'R';
        let mut dist = 0;

        loop {
            let next = iterator.next();
            if next.is_none() {
                break;
            };
            dist = (dist * 10) + next.unwrap().1.to_digit(10).expect("Expected good digit");
        }

        moves.push(Move {
            right: is_right,
            distance: dist,
        });
    }
    return moves;
}

#[derive(Debug)]
struct Move {
    right: bool,
    distance: u32,
}

#[derive(Clone, Copy, Debug)]
struct Position {
    cur_pos: u8,
    origin_count: u32,
    total_moves: u32,
    times_crossed_origin: u32,
}

impl Position {
    fn turn(&mut self, mv: &Move) {
        let mut times_crossed_origin = mv.distance / 100;
        let mut distance = (mv.distance % 100) as i8;
        if !mv.right {
            distance *= -1;
        }

        let relative_distance: u8 = {
            let absolute_final_position: i16 = self.cur_pos as i16 + distance as i16;

            if distance < 0 {
                if absolute_final_position == 0 {
                    times_crossed_origin += 1;
                }

                if absolute_final_position < 0 && self.cur_pos > 0 {
                    times_crossed_origin += 1;
                }

                (distance + 100) as u8
            } else {
                if absolute_final_position >= 100 {
                    times_crossed_origin += 1;
                }

                distance as u8
            }
        };

        self.times_crossed_origin += times_crossed_origin;
        let new_pos = (self.cur_pos + relative_distance) % 100;
        self.cur_pos = new_pos;
        if new_pos == 0 {
            self.origin_count += 1;
        }
        self.total_moves += 1;
    }
}

#[test]
fn test_example() {
    let raw = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string();
    let pos = run(&raw);
    assert_eq!(pos.origin_count, 3);
    assert_eq!(pos.times_crossed_origin, 6);
}
