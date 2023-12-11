use std::fs;

const GALAXY: char = '#';

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct GalaxyPos {
    x: i64,
    y: i64,
}

fn main() {
    println!("Day 11 - Part 2");

    let str = fs::read_to_string("../_input/day11.txt").unwrap();
    let lines: Vec<&str> = str.lines().collect();

    let total = process(lines, 1000000);
    println!("Total: {}", total);
}

fn process(lines: Vec<&str>, expand: i64) -> i64 {
    let map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let (row_ex, col_ex) = dup_empty(&map, expand);

    let mut galaxy: Vec<GalaxyPos> = Vec::new();
    for (y, r) in map.iter().enumerate() {
        for (x, c) in r.iter().enumerate() {
            if GALAXY.eq(c) {
                let x = col_ex[0..=x].iter().fold(x as i64, |acc, e| acc + *e);
                let y = row_ex[0..=y].iter().fold(y as i64, |acc, e| acc + *e);

                galaxy.push(GalaxyPos { x, y });
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

fn dup_empty(map: &[Vec<char>], expand: i64) -> (Vec<i64>, Vec<i64>) {
    let mut add_row = Vec::new();
    let mut add_col = Vec::new();
    let expand = expand - 1;

    for x in 0..map.first().unwrap().len() {
        let mut col_galaxy = false;
        for (y, row) in map.iter().enumerate() {
            if !row.contains(&GALAXY) && add_row.get(y).is_none() {
                add_row.push(expand);
            } else if add_row.get(y).is_none() {
                add_row.push(0);
            }

            if GALAXY.eq(&row[x]) {
                col_galaxy = true;
            }
        }
        if !col_galaxy {
            add_col.push(expand);
        } else {
            add_col.push(0);
        }
    }

    (add_row, add_col)
}

#[cfg(test)]
mod test {
    use crate::process;

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

        assert_eq!(process(test_string.split('\n').collect(), 2), 374);
        assert_eq!(process(test_string.split('\n').collect(), 10), 1030);
        assert_eq!(process(test_string.split('\n').collect(), 100), 8410);
    }
}
