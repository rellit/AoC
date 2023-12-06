use std::ops::Range;

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
    fn apply(&self, ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut ret = Vec::new();

        ranges.iter().for_each(|range| {
            let mut rest = Some(range.clone());
            for mapping in self.mappings.iter() {
                let (pre, mapped, post) = mapping.apply_to(rest.unwrap().clone());

                if pre.is_some() {
                    ret.push(pre.unwrap())
                }
                if mapped.is_some() {
                    ret.push(mapped.unwrap())
                }

                rest = post.clone();

                if post.is_none() {
                    break;
                }
            }
            if rest.is_some() {
                ret.push(rest.unwrap())
            }
        });

        ret
    }
}

#[derive(Debug)]
struct Mapping {
    range: Range<u64>,
    correct: i64,
}

impl Mapping {
    fn apply_to(
        &self,
        range: Range<u64>,
    ) -> (Option<Range<u64>>, Option<Range<u64>>, Option<Range<u64>>) {
        let pre = if range.start < self.range.start {
            let end = self.range.start.min(range.end);
            Some(range.start..end)
        } else {
            None
        };

        let mapped = if self.range.contains(&range.start) || self.range.contains(&range.end) {
            let start = range
                .start
                .max(self.range.start)
                .checked_add_signed(self.correct)
                .unwrap();
            let end = range
                .end
                .min(self.range.end)
                .checked_add_signed(self.correct)
                .unwrap();
            Some(start..end)
        } else {
            None
        };

        let post = if range.end > self.range.end {
            let start = range.start.max(self.range.end);
            Some(start..range.end)
        } else {
            None
        };

        (pre, mapped, post)
    }
}

#[derive(Debug)]
struct MappingTable<'a> {
    seeds: Vec<Range<u64>>,
    mapper: Vec<Mapper<'a>>,
}

impl MappingTable<'_> {
    fn apply(&self) -> Vec<Range<u64>> {
        let mut new = self.seeds.clone();
        self.mapper.iter().for_each(|mapper| {
            new = mapper.apply(new.clone());
        });
        new
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u64 {
    let mt = mapping_table(input).unwrap().1;

    let mut calc: Vec<Range<u64>> = mt.apply();

    calc.sort_by(|r1, r2| r1.start.cmp(&r2.start));

    calc.first().unwrap().start
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
    let (input, mut mappings) = separated_list1(tag("\n"), mapping)(input)?;

    mappings.sort_by(|mapping1, mapping2| mapping1.range.start.cmp(&mapping2.range.start));

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
            correct: dst_start as i64 - src_start as i64,
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
