use std::{collections::HashMap, fs};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: u32,
    y: u32,
}

#[derive(Clone, Debug)]
enum Pipe {
    Vertical {
        n: Pos,
        s: Pos,
    },
    Horizontal {
        e: Pos,
        w: Pos,
    },
    NEBend {
        n: Pos,
        e: Pos,
    },
    NWBend {
        n: Pos,
        w: Pos,
    },
    SWBend {
        s: Pos,
        w: Pos,
    },
    SEBend {
        s: Pos,
        e: Pos,
    },
    Starting {
        n: Option<Pos>,
        e: Option<Pos>,
        s: Option<Pos>,
        w: Option<Pos>,
    },
}

#[derive(Debug)]
struct PosPipe {
    pipe: Pipe,
    pos: Pos,
}

fn main() {
    println!("Day 10 - Part 1");

    let str = fs::read_to_string("../_input/day10.txt").unwrap();
    let lines: Vec<&str> = str.lines().collect();

    let total = process(lines);
    println!("Total: {}", total);
}

fn process(lines: Vec<&str>) -> u32 {
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
    let starting = get_starting(&map, y, x);

    let mut network: HashMap<Pos, Pipe> = HashMap::new();
    network.insert(starting.pos, starting.pipe.clone());

    let mut dirs = Vec::new();
    if let Pipe::Starting { n, e, s, w } = starting.pipe {
        let mut filled = false;
        if let Some(n) = n {
            dirs.push(n);
            if !filled {
                fill(&map, &mut network, starting.pos, n);
                filled = true;
            }
        }
        if let Some(e) = e {
            dirs.push(e);
            if !filled {
                fill(&map, &mut network, starting.pos, e);
                filled = true;
            }
        }
        if let Some(s) = s {
            dirs.push(s);
            if !filled {
                fill(&map, &mut network, starting.pos, s);
                filled = true;
            }
        }
        if let Some(w) = w {
            dirs.push(w);
            if !filled {
                fill(&map, &mut network, starting.pos, w);
            }
        }
    } else {
        panic!("Huh?");
    }

    piped(&network, starting.pos, starting.pos, dirs[0], dirs[1])
}

fn piped(network: &HashMap<Pos, Pipe>, alast: Pos, blast: Pos, a: Pos, b: Pos) -> u32 {
    if a.eq(&b) {
        return 1;
    }

    let ap = {
        let pipe = network.get(&a).unwrap();
        next_pos(pipe.clone(), alast)
    };
    let bp = {
        let pipe = network.get(&b).unwrap();
        next_pos(pipe.clone(), blast)
    };

    piped(network, a, b, ap, bp) + 1
}

fn next_pos(pipe: Pipe, last: Pos) -> Pos {
    match pipe {
        Pipe::Vertical { n, s } => {
            if last.eq(&s) {
                n
            } else {
                s
            }
        }
        Pipe::Horizontal { e, w } => {
            if last.eq(&w) {
                e
            } else {
                w
            }
        }
        Pipe::NEBend { n, e } => {
            if last.eq(&e) {
                n
            } else {
                e
            }
        }
        Pipe::NWBend { n, w } => {
            if last.eq(&w) {
                n
            } else {
                w
            }
        }
        Pipe::SWBend { s, w } => {
            if last.eq(&w) {
                s
            } else {
                w
            }
        }
        Pipe::SEBend { s, e } => {
            if last.eq(&e) {
                s
            } else {
                e
            }
        }
        Pipe::Starting { .. } => panic!("How at start?"),
    }
}

fn fill(map: &[Vec<char>], network: &mut HashMap<Pos, Pipe>, mut last: Pos, mut pos: Pos) {
    while !network.contains_key(&pos) {
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

                network.insert(pos, Pipe::Vertical { n, s });

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

                network.insert(pos, Pipe::Horizontal { e, w });

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

                network.insert(pos, Pipe::NEBend { n, e });

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

                network.insert(pos, Pipe::NWBend { n, w });

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

                network.insert(pos, Pipe::SWBend { s, w });

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

                network.insert(pos, Pipe::SEBend { s, e });

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

fn get_starting(map: &[Vec<char>], y: usize, x: usize) -> PosPipe {
    let yy = y as u32;
    let xx = x as u32;

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

    PosPipe {
        pipe: Pipe::Starting { n, e, s, w },
        pos: Pos { x: xx, y: yy },
    }
}

#[cfg(test)]
mod test {
    use crate::process;

    #[test]
    fn test() {
        assert_eq!(
            process(
                r".....
.S-7.
.|.|.
.L-J.
....."
                    .split('\n')
                    .collect()
            ),
            4
        );

        assert_eq!(
            process(
                r"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
                    .split('\n')
                    .collect()
            ),
            8
        );
    }
}
