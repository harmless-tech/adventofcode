use std::fs;

const GALAXY: char = '#';
const EMPTY: char = '.';

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct GalaxyPos {
    x: i32,
    y: i32,
}

fn main() {
    println!("Day 11 - Part 1");

    let str = fs::read_to_string("../_input/day11.txt").unwrap();
    let lines: Vec<&str> = str.lines().collect();

    let total = process(lines);
    println!("Total: {}", total);
}

fn process(lines: Vec<&str>) -> i32 {
    let mut map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    dup_empty(&mut map);
    let map = map;

    let mut galaxy: Vec<GalaxyPos> = Vec::new();
    for (y, r) in map.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            if GALAXY.eq(c) {
                galaxy.push(GalaxyPos {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    let galaxy = galaxy;

    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for (xid, _) in galaxy.iter().enumerate() {
        for yid in (xid + 1)..galaxy.len() {
            pairs.push((xid, yid));
        }
    }

    let mut total = 0;
    for (xid, yid) in pairs {
        let GalaxyPos { x: xx, y: xy } = galaxy[xid];
        let GalaxyPos { x: yx, y: yy } = galaxy[yid];

        total += (xx.max(yx) - xx.min(yx)) + (xy.max(yy) - xy.min(yy));
    }

    total
}

fn dup_empty(map: &mut Vec<Vec<char>>) {
    let mut dup_row = Vec::new();
    let mut dup_col = Vec::new();
    for x in 0..map.first().unwrap().len() {
        let mut col_galaxy = false;
        for (y, row) in map.iter().enumerate() {
            if !row.contains(&GALAXY) && !dup_row.contains(&y) {
                dup_row.push(y);
            }
            if GALAXY.eq(&row[x]) {
                col_galaxy = true;
            }
        }
        if !col_galaxy {
            dup_col.push(x + dup_col.len());
        }
    }
    for row in map.iter_mut() {
        for d in dup_col.iter() {
            row.insert(*d, EMPTY);
        }
    }
    for (i, d) in dup_row.iter().enumerate() {
        let d = d + i;
        map.insert(d, map[d].clone());
    }
}

#[cfg(test)]
mod test {
    use crate::{dup_empty, process};

    #[test]
    fn test() {
        let test_string = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(process(test_string.split('\n').collect()), 374);
    }

    #[test]
    fn dupe() {
        let test_string = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let mut map: Vec<Vec<char>> = test_string
            .split('\n')
            .collect::<Vec<&str>>()
            .iter()
            .map(|l| l.chars().collect())
            .collect();
        dup_empty(&mut map);

        let dup_string = r"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

        let dup_map: Vec<Vec<char>> = dup_string
            .split('\n')
            .collect::<Vec<&str>>()
            .iter()
            .map(|l| l.chars().collect())
            .collect();

        assert_eq!(dup_map.len(), map.len());
        assert_eq!(dup_map[0].len(), map[0].len());
        for ((r, d), m) in dup_map.iter().enumerate().zip(map.iter()) {
            for ((c, d), m) in d.iter().enumerate().zip(m.iter()) {
                dbg!(r, c);
                assert_eq!(*d, *m);
            }
        }
    }
}
