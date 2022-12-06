use bit_set::BitSet;

pub struct Bag {
    left: BitSet<u64>,
    right: BitSet<u64>,
}
type Input = Vec<Bag>;

fn map_from_char(c: char) -> usize {
    match c {
        'a'..='z' => ((c as u32) - ('a' as u32) + 1) as usize,
        'A'..='Z' => ((c as u32) - ('A' as u32) + 26 + 1) as usize,
        _ => 0,
    }
}

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    Ok(input
        .lines()
        .map(|line| {
            let mut left: BitSet<u64> = BitSet::default();
            let mut right: BitSet<u64> = BitSet::default();

            let (l, r) = line.split_at(line.len() / 2);

            l.chars().map(map_from_char).for_each(|n| {
                left.insert(n);
            });

            r.chars().map(map_from_char).for_each(|n| {
                right.insert(n);
            });

            Bag { left, right }
        })
        .collect())
}

pub fn run(input: Input) -> () {
    let matching: usize = input
        .iter()
        .map(|bag| bag.left.intersection(&bag.right).sum::<usize>())
        .sum();
    println!("{}", matching);

    let id: usize = input
        .chunks_exact(3)
        .map(|group| {
            let elf0 = group[0].left.union(&group[0].right);
            let elf1 = group[1].left.union(&group[1].right);
            let elf2 = group[2].left.union(&group[2].right);

            let mut pool: BitSet<u64> = elf0.collect();
            pool.intersect_with(&elf1.collect());
            pool.intersect_with(&elf2.collect());

            pool.iter().sum::<usize>()
        })
        .sum::<usize>();
    println!("{}", id);
}
