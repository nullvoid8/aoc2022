use std::fs;

// use bitvec::prelude::*;
use bitvec::{
    bits,
    field::BitField,
    order::{Lsb0, Msb0},
    prelude::BitVec,
};
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take},
    multi::{length_count, length_value, many1, many_till},
    sequence::{pair, preceded},
    IResult, Parser,
};
use nom_bitvec::BSlice;

#[derive(Debug, Clone, Copy)]
enum PacketType {
    Sum,
    Prod,
    Min,
    Max,
    Lit,
    Gt,
    Lt,
    Eq,
}

impl From<usize> for PacketType {
    fn from(x: usize) -> Self {
        match x {
            0 => PacketType::Sum,
            1 => PacketType::Prod,
            2 => PacketType::Min,
            3 => PacketType::Max,
            4 => PacketType::Lit,
            5 => PacketType::Gt,
            6 => PacketType::Lt,
            7 => PacketType::Eq,
            _ => todo!(),
        }
    }
}

impl PacketType {
    fn parse(i: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, PacketType> {
        take(3u8)
            .map(|p_type: BSlice<Msb0, u8>| p_type.0.load_be::<usize>().into())
            .parse(i)
    }
}

#[derive(Debug, Clone)]
enum Packet {
    Lit {
        version: usize,
        value: usize,
    },
    Op {
        version: usize,
        op: PacketType,
        args: Vec<Packet>,
    },
}

fn parse_packet_version(i: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, usize> {
    take(3u8)
        .map(|slice: BSlice<Msb0, u8>| slice.0.load_be::<usize>())
        .parse(i)
}

fn parse_literal(i: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, usize> {
    many_till(
        preceded(tag(BSlice(bits![1])), take(4u8)),
        preceded(tag(BSlice(bits![0])), take(4u8)),
    )
    .map(
        |(partials, terminal): (Vec<BSlice<Msb0, u8>>, BSlice<Msb0, u8>)| {
            let mut vec: BitVec<Msb0, u8> = BitVec::new();
            for BSlice(slice) in partials {
                vec.extend_from_bitslice(slice);
            }
            vec.extend_from_bitslice(terminal.0);
            vec.load_be()
        },
    )
    .parse(i)
}

fn parse_operator(i: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, Vec<Packet>> {
    let (res, indicator) = take(1u8).parse(i)?;

    match indicator.0[0] {
        false => length_value(
            take(15u8).map(|s: BSlice<Msb0, u8>| s.0.load_be::<usize>()),
            many1(Packet::parse),
        )
        .parse(res),

        true => length_count(
            take(11u8).map(|s: BSlice<Msb0, u8>| s.0.load_be::<usize>()),
            Packet::parse,
        )
        .parse(res),
    }
}

impl Packet {
    fn parse(i: BSlice<Msb0, u8>) -> IResult<BSlice<Msb0, u8>, Packet> {
        let (res, (version, p_type)) = pair(parse_packet_version, PacketType::parse).parse(i)?;
        match p_type {
            PacketType::Lit => {
                let (res, value) = parse_literal(res)?;
                Ok((res, Packet::Lit { version, value }))
            }
            op => {
                let (res, args) = parse_operator(res)?;
                Ok((res, Packet::Op { version, op, args }))
            }
        }
    }

    fn version_sum(&self) -> usize {
        match self {
            Packet::Lit { version, .. } => *version,
            Packet::Op { version, args, .. } => {
                *version + args.iter().map(|x| x.version_sum()).sum::<usize>()
            }
        }
    }

    fn eval(&self) -> usize {
        match self {
            Packet::Lit { value, .. } => *value,
            Packet::Op { op, args, .. } => {
                let args = args.iter().map(|a| a.eval());
                match op {
                    PacketType::Sum => args.sum::<usize>(),
                    PacketType::Prod => args.product::<usize>(),
                    PacketType::Min => args.min().unwrap(),
                    PacketType::Max => args.max().unwrap(),
                    PacketType::Gt => {
                        let (x, y) = args.collect_tuple().unwrap();
                        (x > y) as usize
                    }
                    PacketType::Lt => {
                        let (x, y) = args.collect_tuple().unwrap();
                        (x < y) as usize
                    }
                    PacketType::Eq => {
                        let (x, y) = args.collect_tuple().unwrap();
                        (x == y) as usize
                    }

                    PacketType::Lit => todo!(),
                }
            }
        }
    }
}

pub fn day() {
    let content = fs::read_to_string("inputs/day16").expect("Couldn't find input");
    let bits: BitVec<Msb0, u8> = hex::decode(content).unwrap().into_iter().collect();

    let (_, packet) = Packet::parse(BSlice(&bits)).unwrap();

    println!("{:?} {:?}", packet.version_sum(), packet.eval());
}
