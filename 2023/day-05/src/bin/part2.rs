use std::ops::Range;

use indicatif::ProgressIterator;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct Mapper<'a> {
    _src: &'a str,
    _dst: &'a str,
    mappings: Vec<Mapping>,
}

impl Mapper<'_> {
    fn map(&self, src: u64) -> u64 {
        for mapping in self.mappings.iter() {
            match mapping.map(src) {
                Some(dst) => return dst,
                None => (),
            }
        }
        return src;
    }
}

#[derive(Debug)]
struct Mapping {
    range: Range<u64>,
    correct: isize,
}

impl Mapping {
    fn map(&self, src: u64) -> Option<u64> {
        match self.range.contains(&src) {
            true => Some((src as isize - self.correct) as u64),
            false => None,
        }
    }
}

#[derive(Debug)]
struct MappingTable<'a> {
    seeds: Vec<Range<u64>>,
    mapper: Vec<Mapper<'a>>,
}

impl MappingTable<'_> {
    fn apply_for(&self, src: u64) -> u64 {
        let mut dst: u64 = src;
        self.mapper.iter().for_each(|mapper| {
            dst = (mapper).map(dst);
        });
        dst
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u64 {
    let mt = mapping_table(input).unwrap().1;

    let mut lowest: u64 = u64::MAX;

    let total: u64 = mt
        .seeds
        .iter()
        .map(|seed_range| seed_range.clone().count() as u64)
        .sum();

    mt.seeds
        .iter()
        .flat_map(|seed_range| seed_range.clone().into_iter())
        .progress_count(total)
        .for_each(|seed| {
            let new = mt.apply_for(seed);
            lowest = lowest.min(new);
        });

    lowest as u64
}

fn seeds(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    let (input, seeds) =
        separated_list1(space1, separated_pair(complete::u64, space1, complete::u64))(input)?;

    let seeds = seeds
        .iter()
        .map(|(start, size)| *start..start + size)
        .collect();
    Ok((&input, seeds))
}

fn mapping_table(input: &str) -> IResult<&str, MappingTable> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = seeds(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, mapper) = separated_list1(tag("\n\n"), mapper)(input)?;

    Ok((&input, MappingTable { mapper, seeds }))
}

fn mapper(input: &str) -> IResult<&str, Mapper> {
    //seed-to-soil map:
    let (input, src) = alpha1(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, dst) = alpha1(input)?;
    let (input, _) = tag(" map:\n")(input)?;
    let (input, mappings) = separated_list1(tag("\n"), mapping)(input)?;

    Ok((
        &input,
        Mapper {
            _dst: dst,
            _src: src,
            mappings,
        },
    ))
}

fn mapping(input: &str) -> IResult<&str, Mapping> {
    //0 15 37
    let (input, dst_start) = complete::u64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, src_start) = complete::u64(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, map_size) = complete::u64(input)?;

    Ok((
        &input,
        Mapping {
            range: (src_start..src_start + map_size),
            correct: src_start as isize - dst_start as isize,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_code() {
        let result = solve(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, 46);
    }
}
