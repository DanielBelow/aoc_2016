use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::num_traits::FromPrimitive;
use pathfinding::prelude::dijkstra;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Position(usize, usize);

impl Position {
    fn node_name(&self, map: &[Vec<char>]) -> usize {
        let chr = map[self.0][self.1];
        assert!(
            chr.is_ascii_digit(),
            "This should only be called on valid nodes!"
        );

        chr.to_digit(10).expect("Checked is_ascii_digit()") as usize
    }

    fn successors(&self, map: &[Vec<char>]) -> Vec<(Self, usize)> {
        let mut successors = Vec::new();

        let x = self.0;
        let y = self.1;

        if x > 0 && map[x - 1][y] != '#' {
            successors.push((Self(x - 1, y), 1));
        }

        if x < map.len() - 1 && map[x + 1][y] != '#' {
            successors.push((Self(x + 1, y), 1));
        }

        if y > 0 && map[x][y - 1] != '#' {
            successors.push((Self(x, y - 1), 1));
        }

        if y < map[x].len() - 1 && map[x][y + 1] != '#' {
            successors.push((Self(x, y + 1), 1));
        }

        successors
    }
}

#[aoc_generator(day24)]
pub fn generate(inp: &str) -> Vec<Vec<char>> {
    inp.lines().map(|it| it.chars().collect()).collect()
}

fn get_nodes_to_visit(map: &[Vec<char>]) -> Vec<Position> {
    let mut to_visit = Vec::new();

    for (x, row) in map.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            if tile.is_ascii_digit() {
                to_visit.push(Position(x, y));
            }
        }
    }

    to_visit.sort_by(|&lpos, &rpos| map[lpos.0][lpos.1].cmp(&map[rpos.0][rpos.1]));
    to_visit
}

fn get_vertex_names(nodes: &[Position], map: &[Vec<char>]) -> Vec<usize> {
    nodes
        .iter()
        .filter_map(|it| {
            let name = it.node_name(map);
            if name == 0 {
                None
            } else {
                Some(name)
            }
        })
        .collect()
}

#[allow(clippy::cast_precision_loss)]
fn get_distance_matrix(nodes: &[Position], map: &[Vec<char>]) -> Vec<Vec<f64>> {
    let mut distance_matrix = vec![vec![0f64; nodes.len()]; nodes.len()];

    for idx in 0..nodes.len() {
        let cur_node = nodes[idx];

        let from_node = cur_node.node_name(map);

        for target_node in &nodes[idx + 1..] {
            let dest_node = target_node.node_name(map);

            let (_, cost) = dijkstra(&cur_node, |it| it.successors(map), |it| *it == *target_node)
                .expect("There should be a path between all nodes");

            distance_matrix[from_node][dest_node] = cost as f64;
            distance_matrix[dest_node][from_node] = cost as f64;
        }
    }

    distance_matrix
}

fn calculate_possible_route_distances<Fn>(
    vertices: &[usize],
    distance_matrix: &[Vec<f64>],
    mut adjust_route: Fn,
) -> usize
where
    Fn: FnMut(&mut Vec<usize>),
{
    let mut min_dist = usize::MAX;

    for route in vertices.iter().permutations(vertices.len()) {
        let mut test_route = route.iter().map(|it| **it).collect_vec();
        adjust_route(&mut test_route);

        let dist = travelling_salesman::get_route_distance(distance_matrix, &test_route);
        let dist = usize::from_f64(dist).expect("The value should be small enough");
        if dist < min_dist {
            min_dist = dist;
        }
    }

    min_dist
}

#[aoc(day24, part1)]
pub fn part1(map: &[Vec<char>]) -> usize {
    let to_visit = get_nodes_to_visit(map);
    let distance_matrix = get_distance_matrix(&to_visit, map);
    let vertices = get_vertex_names(&to_visit, map);

    calculate_possible_route_distances(&vertices, &distance_matrix, |it| it.insert(0, 0))
}

#[aoc(day24, part2)]
pub fn part2(map: &[Vec<char>]) -> usize {
    let to_visit = get_nodes_to_visit(map);
    let distance_matrix = get_distance_matrix(&to_visit, map);
    let vertices = get_vertex_names(&to_visit, map);

    calculate_possible_route_distances(&vertices, &distance_matrix, |it| {
        it.insert(0, 0);
        it.push(0);
    })
}
