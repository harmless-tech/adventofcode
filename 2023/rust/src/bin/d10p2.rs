use std::fs;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

fn main() {
    println!("Day 10 - Part 2");

    let str = fs::read_to_string("../_input/day10.txt").unwrap();
    let lines: Vec<&str> = str.lines().collect();

    let total = process(lines);
    println!("Total: {}", total);
}

fn process(lines: Vec<&str>) -> i32 {
    let map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let mut yx = None;
    for (iy, y) in map.iter().enumerate() {
        for (ix, x) in y.iter().enumerate() {
            if *x == 'S' {
                yx = Some((iy, ix));
            }
        }
    }

    let (y, x) = yx.unwrap();

    let (start_pos, n, e, s, w) = get_starting(&map, y, x);

    let mut points: Vec<Pos> = Vec::new();
    points.push(start_pos);

    let mut filled = false;
    if let Some(n) = n {
        if !filled {
            fill(&map, &mut points, start_pos, n);
            filled = true;
        }
    }
    if let Some(e) = e {
        if !filled {
            fill(&map, &mut points, start_pos, e);
            filled = true;
        }
    }
    if let Some(s) = s {
        if !filled {
            fill(&map, &mut points, start_pos, s);
            filled = true;
        }
    }
    if let Some(w) = w {
        if !filled {
            fill(&map, &mut points, start_pos, w);
        }
    }

    points.push(points[0]);

    let mut shoe = 0;
    for p in points.windows(2) {
        let Pos { x: a, y: c } = p[0];
        let Pos { x: b, y: d } = p[1];
        let det = (a * d) - (b * c);
        shoe += det;
    }

    let shoe = (shoe.abs() / 2) as f64;
    let points = (points.len() - 1) as f64;

    (shoe - 0.5 * points + 1.0) as i32
}

fn fill(map: &[Vec<char>], points: &mut Vec<Pos>, mut last: Pos, mut pos: Pos) {
    while !pos.eq(points.first().unwrap()) {
        let y = pos.y as usize;
        let x = pos.x as usize;

        match map[y][x] {
            '|' => {
                let n = Pos {
                    x: pos.x,
                    y: pos.y - 1,
                };
                let s = Pos {
                    x: pos.x,
                    y: pos.y + 1,
                };

                points.push(pos);

                if last.eq(&s) {
                    last = pos;
                    pos = n;
                } else {
                    last = pos;
                    pos = s;
                }
            }
            '-' => {
                let e = Pos {
                    x: pos.x + 1,
                    y: pos.y,
                };
                let w = Pos {
                    x: pos.x - 1,
                    y: pos.y,
                };

                points.push(pos);

                if last.eq(&w) {
                    last = pos;
                    pos = e;
                } else {
                    last = pos;
                    pos = w;
                }
            }
            'L' => {
                let n = Pos {
                    x: pos.x,
                    y: pos.y - 1,
                };
                let e = Pos {
                    x: pos.x + 1,
                    y: pos.y,
                };

                points.push(pos);

                if last.eq(&e) {
                    last = pos;
                    pos = n;
                } else {
                    last = pos;
                    pos = e;
                }
            }
            'J' => {
                let n = Pos {
                    x: pos.x,
                    y: pos.y - 1,
                };
                let w = Pos {
                    x: pos.x - 1,
                    y: pos.y,
                };

                points.push(pos);

                if last.eq(&w) {
                    last = pos;
                    pos = n;
                } else {
                    last = pos;
                    pos = w;
                }
            }
            '7' => {
                let s = Pos {
                    x: pos.x,
                    y: pos.y + 1,
                };
                let w = Pos {
                    x: pos.x - 1,
                    y: pos.y,
                };

                points.push(pos);

                if last.eq(&w) {
                    last = pos;
                    pos = s;
                } else {
                    last = pos;
                    pos = w;
                }
            }
            'F' => {
                let s = Pos {
                    x: pos.x,
                    y: pos.y + 1,
                };
                let e = Pos {
                    x: pos.x + 1,
                    y: pos.y,
                };

                points.push(pos);

                if last.eq(&e) {
                    last = pos;
                    pos = s;
                } else {
                    last = pos;
                    pos = e;
                }
            }
            _ => panic!("NOT PIPE, NOT PIPE!"),
        }
    }
}

fn get_starting(
    map: &[Vec<char>],
    y: usize,
    x: usize,
) -> (Pos, Option<Pos>, Option<Pos>, Option<Pos>, Option<Pos>) {
    let yy = y as i32;
    let xx = x as i32;

    let mut n = None;
    let mut s = None;
    let mut e = None;
    let mut w = None;

    if y > 0 {
        let c = map[y - 1][x];
        if c == '|' || c == '7' || c == 'F' {
            n = Some(Pos { y: yy - 1, x: xx });
        }
    }

    let c = map[y + 1][x];
    if c == '|' || c == 'L' || c == 'J' {
        s = Some(Pos { y: yy + 1, x: xx });
    }

    let c = map[y][x + 1];
    if c == '-' || c == 'J' || c == '7' {
        e = Some(Pos { y: yy, x: xx + 1 });
    }

    if x > 0 {
        let c = map[y][x - 1];
        if c == '-' || c == 'L' || c == 'F' {
            w = Some(Pos { y: yy, x: xx - 1 });
        }
    }

    (Pos { x: xx, y: yy }, n, e, s, w)
}

#[cfg(test)]
mod test {
    use crate::process;

    #[test]
    fn test() {
        assert_eq!(
            process(
                r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
                    .split('\n')
                    .collect()
            ),
            4
        );

        assert_eq!(
            process(
                r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
                    .split('\n')
                    .collect()
            ),
            8
        );

        assert_eq!(
            process(
                r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
                    .split('\n')
                    .collect()
            ),
            10
        );
    }
}
