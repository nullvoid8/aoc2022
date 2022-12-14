use itertools::Itertools;
use nom::{
    branch, bytes::complete as bytes, character::complete as character, combinator, multi,
    sequence, IResult,
};

type Input = Vec<(PacketStream, PacketStream)>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PacketStream {
    Value(u32),
    List(Vec<PacketStream>),
}

impl From<Vec<PacketStream>> for PacketStream {
    fn from(x: Vec<PacketStream>) -> Self {
        PacketStream::List(x)
    }
}

impl From<u32> for PacketStream {
    fn from(x: u32) -> Self {
        PacketStream::Value(x)
    }
}

impl Ord for PacketStream {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PacketStream::Value(x), PacketStream::Value(y)) => x.cmp(y),
            (PacketStream::List(x), PacketStream::List(y)) => x.cmp(y),
            (&PacketStream::Value(x), y @ PacketStream::List(_)) => {
                PacketStream::List(vec![PacketStream::Value(x)]).cmp(y)
            }
            (x @ PacketStream::List(_), &PacketStream::Value(y)) => {
                x.cmp(&PacketStream::List(vec![PacketStream::Value(y)]))
            }
        }
    }
}

impl PartialOrd for PacketStream {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn packet_stream(i: &str) -> IResult<&str, PacketStream> {
    combinator::map(
        sequence::delimited(
            bytes::tag("["),
            multi::separated_list0(
                bytes::tag(","),
                branch::alt((
                    combinator::map(character::u32, PacketStream::Value),
                    packet_stream,
                )),
            ),
            bytes::tag("]"),
        ),
        |x| PacketStream::List(x),
    )(i)
}

fn packet_pair(i: &str) -> IResult<&str, (PacketStream, PacketStream)> {
    sequence::pair(
        sequence::terminated(packet_stream, character::line_ending),
        sequence::terminated(packet_stream, character::line_ending),
    )(i)
}

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    let res = multi::separated_list0(character::line_ending, packet_pair)(&input);
    Ok(res.unwrap().1)
}

pub fn run(input: Input) -> () {
    let p1: usize = input
        .iter()
        .enumerate()
        .filter_map(|(i, (x, y))| if x < y { Some(i + 1) } else { None })
        .sum();
    println!("{:?}", p1);

    let mut p2 = input
        .into_iter()
        .flat_map(|pair| [pair.0, pair.1])
        .collect_vec();

    let two = PacketStream::List(vec![PacketStream::List(vec![PacketStream::Value(2)])]);
    let six = PacketStream::List(vec![PacketStream::List(vec![PacketStream::Value(6)])]);

    p2.push(two.clone());
    p2.push(six.clone());

    p2.sort();

    let p2: usize = p2
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| ((x == two) | (x == six)).then_some(i + 1))
        .product();

    println!("{:?}", p2);
}
