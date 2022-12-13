use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn get_node_from_grid(
    grid: &Vec<Vec<char>>,
    to_visit: (usize, usize, String),
    offset: (isize, isize),
) -> Option<(usize, usize, String)> {
    if offset.1 + (to_visit.1 as isize) < 0 {
        return None;
    }
    if offset.0 + (to_visit.0 as isize) < 0 {
        return None;
    }
    if let Some(current_row) = grid.get(to_visit.1) {
        if let Some(current) = current_row.get(to_visit.0) {
            if let Some(offset_row) = grid.get((to_visit.1 as isize + offset.1) as usize) {
                if let Some(offset_node) = offset_row.get((to_visit.0 as isize + offset.0) as usize)
                {
                    if *current as u32 == *offset_node as u32
                        || *current as u32 + 1_u32 == *offset_node as u32
                        || *current == 'S'
                        || (*current == 'z' && *offset_node == 'E')
                        || *current as u32 > *offset_node as u32
                    {
                        return Some((
                            (to_visit.0 as isize + offset.0) as usize,
                            (to_visit.1 as isize + offset.1) as usize,
                            format!("{}{}", to_visit.2, *current),
                        ));
                    }
                }
            }
        }
    }
    None
}

fn visit_grid(
    grid: &mut Vec<Vec<char>>,
    visit: &mut VecDeque<(usize, usize, String)>,
) -> Option<usize> {
    let mut visited = HashSet::new();
    loop {
        let mut to_visit = visit.pop_front();
        if to_visit.is_none() {
            return None;
        }
        let to_visit = to_visit.unwrap();
        // println!("{},{}", to_visit.0, to_visit.1);
        let check = (to_visit.0, to_visit.1);
        if visited.contains(&check) {
            continue;
        }
        if let Some(current_row) = grid.get(to_visit.1) {
            if let Some(current) = current_row.get(to_visit.0) {
                visited.insert((to_visit.0, to_visit.1));
                if *current == 'E' {
                    dbg!(&to_visit);
                    dbg!(&to_visit.2.len() - 1);
                    return Some(to_visit.2.len() - 1);
                }
            }
        }

        if let Some(top) = get_node_from_grid(
            &grid,
            (to_visit.0, to_visit.1, String::from(&to_visit.2)),
            (0, 1),
        ) {
            visit.push_back(top);
        }

        if let Some(bottom) = get_node_from_grid(
            &grid,
            (to_visit.0, to_visit.1, String::from(&to_visit.2)),
            (0, -1),
        ) {
            visit.push_back(bottom);
        }

        if let Some(left) = get_node_from_grid(
            &grid,
            (to_visit.0, to_visit.1, String::from(&to_visit.2)),
            (-1, 0),
        ) {
            visit.push_back(left);
        }
        if let Some(right) = get_node_from_grid(
            &grid,
            (to_visit.0, to_visit.1, String::from(&to_visit.2)),
            (1, 0),
        ) {
            visit.push_back(right);
        }
    }
}

fn main() {
    let mut grid = Vec::new();
    let mut visit = VecDeque::new();
    let input = fs::read_to_string("./inputs/12.txt").unwrap();
    let mut starting_position = (0, 0, String::from(""));
    let mut ending_position = (0, 0);
    for (y, line) in input.lines().enumerate() {
        grid.push(line.chars().collect::<Vec<char>>());
        if let Some(x) = line.find('S') {
            starting_position = (x, y, "S".to_string());
        }
        if let Some(x) = line.find('E') {
            ending_position = (x, y);
        }
    }
    dbg!(ending_position);

    let mut min = 999;
    for (y, y_val) in grid.clone().iter().enumerate() {
        for (x, x_val) in y_val.iter().enumerate() {
            if *x_val == 'a' {
                visit.truncate(0);
                visit.push_back((x, y, "a".to_string()));
                if let Some(val) = visit_grid(&mut grid, &mut visit) {
                    if val < min {
                        min = val;
                    }
                }
            }
        }
    }
    dbg!(min);

    // visit.push_back(starting_position);
    // visit_grid(&mut grid, &mut visit);
}
