#![allow(dead_code, unused, unused_mut)]

use std::{cmp::Reverse, collections::BinaryHeap, str::FromStr};

use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{newline, u64},
    combinator::{eof, opt},
    multi::{count, many0, separated_list1},
    sequence::terminated,
};

use crate::util::Answer;
pub fn solve(input: &str) -> anyhow::Result<String> {
    let mut graph = Graph::new(parse_input(input)?);
    let p1 = solve_part_one(&mut graph, 1000)?;
    Answer::first(8, p1).report()
}

fn solve_part_one(graph: &mut Graph, connections: usize) -> anyhow::Result<usize> {
    graph
        .connect_closest(connections)
        .map_err(|n_connected| anyhow::anyhow!("Could only connect {n_connected}"))?;
    let p = graph.largest_components(3).into_iter().product();
    Ok(p)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Point(u64, u64, u64);

impl Point {
    fn distance_to(&self, other: &Self) -> u64 {
        (self.0.abs_diff(other.0).pow(2)
            + self.1.abs_diff(other.1).pow(2)
            + self.2.abs_diff(other.2).pow(2))
        .isqrt()
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn part(input: &str) -> nom::IResult<&str, u64> {
            terminated(u64, opt(tag(","))).parse(input)
        }
        fn point(input: &str) -> nom::IResult<&str, Point> {
            let (empty, (parts, _)) = (count(part, 3), eof).parse(input)?;
            Ok((empty, Point(parts[0], parts[1], parts[2])))
        }
        point(s)
            .map(|(_empty, point)| point)
            .map_err(|e| e.to_owned().into())
    }
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Point>> {
    input.lines().map(Point::from_str).collect()
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Edge {
    distance: u64,
    a_id: usize,
    b_id: usize,
}

struct Graph {
    points: Vec<Point>,
    parents: Vec<usize>,
    sizes: Vec<usize>,
    ordered_edges: BinaryHeap<Reverse<Edge>>,
}

impl Graph {
    fn new(points: Vec<Point>) -> Self {
        let n_points = points.len();
        let ordered_edges = min_heap_from_points(&points);
        Self {
            points,
            parents: (0..n_points).collect(),
            sizes: vec![1; n_points],
            ordered_edges,
        }
    }

    fn find(&self, mut id: usize) -> usize {
        while self.parents[id] != id {
            id = self.parents[id];
        }
        id
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let a_root = self.find(a);
        let b_root = self.find(b);
        if a_root == b_root {
            // Already connected
            return false;
        }

        let a_root_size = self.sizes[a_root];
        let b_root_size = self.sizes[b_root];
        if a_root_size >= b_root_size {
            // self.parents[a] = a_root;
            self.parents[b] = a_root;
            self.parents[b_root] = a_root;
            self.sizes[a_root] += self.sizes[b_root];
            self.sizes[b_root] = 0;
        } else {
            self.parents[a] = b_root;
            // self.parents[b] = b_root;
            self.parents[a_root] = b_root;
            self.sizes[b_root] += self.sizes[a_root];
            self.sizes[a_root] = 0;
        }
        true
    }

    fn connect_closest(&mut self, n: usize) -> Result<(), usize> {
        let mut connected = 0;

        for connected in 0..n {
            let Some(Reverse(edge)) = self.ordered_edges.pop() else {
                return Err(connected);
            };
            let _ = self.union(edge.a_id, edge.b_id);
        }
        Ok(())
    }

    fn largest_components(&self, n: usize) -> Vec<usize> {
        let mut v = self.sizes.clone();
        v.sort_unstable();
        v.into_iter().rev().take(3).collect()
    }

    fn n_components(&self) -> usize {
        self.sizes.iter().filter(|n| **n != 0).count()
    }
}

fn min_heap_from_points(points: &[Point]) -> BinaryHeap<Reverse<Edge>> {
    let edges = points
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((a_id, a), (b_id, b))| Edge {
            a_id,
            b_id,
            distance: a.distance_to(b),
        });
    let mut ordered_edges = BinaryHeap::with_capacity(points.len().pow(2));
    for edge in edges {
        ordered_edges.push(Reverse(edge));
    }
    ordered_edges
}

#[cfg(test)]
mod test {
    use std::cmp::Reverse;

    use crate::days::day08::solve_part_one;

    use super::{Edge, Graph, Point, parse_input};

    static TEST_INPUT: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn parse_test_input() {
        let points = parse_input(TEST_INPUT).unwrap();
        assert_eq!(points[0], Point(162, 817, 812));
        assert_eq!(points[19], Point(425, 690, 689));
        assert_eq!(points.len(), 20);
    }

    #[test]
    fn test_input_heap() {
        let points = parse_input(TEST_INPUT).unwrap();
        let graph = Graph::new(points);
        let mut heap = graph.ordered_edges;
        assert_eq!(
            heap.pop().unwrap().0,
            Edge {
                distance: 316,
                a_id: 0,
                b_id: 19
            }
        );
        assert_eq!(
            heap.pop().unwrap().0,
            Edge {
                distance: 321,
                a_id: 0,
                b_id: 7,
            }
        );
        assert_eq!(
            heap.pop().unwrap().0,
            Edge {
                distance: 322,
                a_id: 2,
                b_id: 13,
            }
        );
    }

    #[test]
    pub fn small_test_input_union() {
        let points = parse_input(TEST_INPUT).unwrap();
        let mut graph = Graph::new(points);
        assert_eq!(graph.connect_closest(10), Ok(()));
        assert_eq!(graph.largest_components(3), vec![5, 4, 2]);
    }

    #[test]
    pub fn part_one_test_input() {
        let mut graph = Graph::new(parse_input(TEST_INPUT).unwrap());
        assert_eq!(solve_part_one(&mut graph, 10).unwrap(), 40);
    }

    #[test]
    pub fn part_one_known_answer() {
        let points = parse_input(crate::days::get_input(8).unwrap()).unwrap();
        let mut graph = Graph::new(points);
        assert_eq!(solve_part_one(&mut graph, 1000).unwrap(), 54600);
    }
}
