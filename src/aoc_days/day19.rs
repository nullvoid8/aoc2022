use cgmath::{InnerSpace, Point3, Vector3};
use itertools::{iproduct, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

type Point = Point3<i32>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Beacon(Point);

impl Beacon {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (res, (x, _, y, _, z)) = tuple((i32, tag(","), i32, tag(","), i32))(i)?;
        Ok((res, Beacon((x, y, z).into())))
    }
}

#[derive(Debug, Clone)]
struct Scanner(HashMap<Point, HashSet<i32>>);

impl Scanner {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (res, (_, beacons)) = tuple((
            tuple((tag("--- scanner "), i32, tag(" ---"), line_ending)),
            separated_list1(line_ending, Beacon::parse),
        ))(i)?;

        let mut out = Scanner(HashMap::default());

        for (Beacon(here), Beacon(other)) in iproduct!(&beacons, &beacons) {
            out.0
                .entry(*here)
                .or_default()
                .insert((here - other).magnitude2());
        }

        Ok((res, out))
    }
}

fn parse_input(i: &str) -> IResult<&str, Vec<Scanner>> {
    separated_list1(many1(line_ending), Scanner::parse)(i)
}

fn is_match(here: &HashSet<i32>, there: &HashSet<i32>) -> bool {
    here.intersection(there).count() >= 12
}

pub fn day() {
    let content = fs::read_to_string("inputs/day19").expect("Couldn't find input");
    let mut scanners: VecDeque<Scanner> = parse_input(&content).unwrap().1.into();

    let mut realspace: Scanner = scanners.pop_front().unwrap();
    let mut offsets = vec![Vector3::new(0, 0, 0)];

    while let Some(scanner) = scanners.pop_front() {
        let overlaps: Vec<(Point, Point)> = iproduct!(&realspace.0, &scanner.0)
            .filter(|((_, here), (_, there))| is_match(here, there))
            .map(|(x, y)| (*x.0, *y.0))
            .collect_vec();

        // if no overlap, put it back.
        if overlaps.len() < 2 {
            scanners.push_back(scanner);
            continue;
        }

        let (knowns, unknowns): (Vec<_>, Vec<_>) = overlaps.into_iter().unzip();

        let mut translate = Vector3::new(0, 0, 0);

        // try transforming the first two pairs to match.
        // I believe 2 points are all we need to determine rotation + translation.
        // It's certainly enough for the provided input.
        let rot = (0..24).find(|rot| {
            let unknowns = unknowns
                .iter()
                .take(2)
                .map(|p| rotate(*rot, *p))
                .collect_vec();
            translate = knowns[0] - unknowns[0];
            unknowns[1] + translate == knowns[1]
        });

        let rot = match rot {
            Some(rot) => rot,
            None => panic!("no rotation"),
        };

        {
            // sanity check
            let hs_knowns: HashSet<_> = knowns.clone().into_iter().collect();
            let hs_unknowns: HashSet<_> = unknowns
                .iter()
                .map(|p| rotate(rot, *p) + translate)
                .collect();
            assert_eq!(hs_knowns, hs_unknowns, "sanity checking transformation");
        }

        offsets.push(translate);

        // remap new scanner into known space
        let mut scanner = Scanner(
            scanner
                .0
                .into_iter()
                .map(|(k, v)| (rotate(rot, k) + translate, v))
                .collect(),
        );

        // Update all existing points with dists to new points
        for (p, dists) in &mut realspace.0 {
            dists.extend(scanner.0.keys().map(|q| (p - q).magnitude2()));
        }

        // update new points with dists to known points and merge into known space.
        for (p, dists) in &mut scanner.0 {
            dists.extend(realspace.0.keys().map(|q| (p - q).magnitude2()));
            realspace.0.entry(*p).or_default().extend(dists.iter());
        }
    }

    let p1 = realspace.0.len();

    let p2 = iproduct!(&offsets, &offsets)
        .map(|(p, q)| match (p - q).map(|x| x.abs()) {
            Vector3 { x, y, z } => x + y + z,
        })
        .max()
        .unwrap();

    println!("{} {}", p1, p2);
}

// pick rotation by "index". Inital list ~~stolen~~ adapted from xnvbko
fn rotate(r: usize, Point3 { x, y, z }: Point) -> Point {
    let p = match r / 3 {
        0 => Point3::new(x, y, z),    // ( x,  y,  z), 000 0
        1 => Point3::new(-y, x, z),   // (-y,  x,  z), 100 1
        2 => Point3::new(y, -x, z),   // ( y, -x,  z), 010 2
        3 => Point3::new(-x, -y, z),  // (-x, -y,  z), 110 3
        4 => Point3::new(y, x, -z),   // ( y,  x, -z), 001 4
        5 => Point3::new(-x, y, -z),  // (-x,  y, -z), 101 5
        6 => Point3::new(x, -y, -z),  // ( x, -y, -z), 011 6
        7 => Point3::new(-y, -x, -z), // (-y, -x, -z), 111 7
        _ => panic!("div 3 too big"),
    };

    match r % 3 {
        0 => p,
        1 => p.yzx(),
        2 => p.zxy(),
        _ => panic!("mod 3 to big"),
    }
}
