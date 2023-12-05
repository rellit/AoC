use nom::{
    ToUsize,
    bytes::complete::{tag},
    character::complete::{self, space1, alpha1},
    multi::separated_list1,
    IResult, Err,
};

#[derive(Debug)]
struct Mapper<'a> {
    src :&'a str,
    dst: &'a str,
    mappings: Vec<Mapping>
}

impl Mapper<'_> {
    fn map(&self, src:usize) -> usize {
        for mapping in self.mappings.iter() {
            println!("Applying {mapping:?} to {src}");
            if let Some(dst) = mapping.map(src) {
                return dst
            }
        }

        return src;
    }
}

#[derive(Debug)]
struct Mapping {
    src_start: usize,
    dst_start: usize,
    map_size: usize
}

impl Mapping {
    fn map(&self, src: usize) -> Option<usize>{
        if (self.src_start..self.src_start+self.map_size).contains(&src)  {
            let dst = (src as isize - (self.src_start as isize-self.dst_start as isize))as usize;
            println!("Converting {src} to {dst} due to {self:?}");

            Some(dst)
        } else {
        None
        }
    }
}

#[derive(Debug)]
struct MappingTable<'a> {
    seeds: Vec<usize>,
    mapper:Vec<Mapper<'a>>
}

impl MappingTable<'_> {
    fn apply(self)->Vec<usize>{
        self.seeds.iter().map(|src|{
            let mut dst: usize = *src;
            for mapper in self.mapper.iter() {
                dst = (mapper).map(dst);
            }
            dst
        }).collect()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {

    let mt = mapping_table(input).unwrap().1;

    let dst:Vec<usize> = mt.apply();

    dbg!(dst);

    0
}

fn mapping_table(input: &str) -> IResult<&str, MappingTable> {
         let (input, _) = tag("seeds: ")(input)?;
         let (input, seeds) = separated_list1(space1, complete::u32)(input)?;
         let (input, _) = tag("\n\n")(input)?;
         let (input, mapper) = separated_list1(tag("\n\n"), mapper)(input)?;

         Ok((&input, MappingTable{mapper, seeds:seeds.iter().map(|s|s.to_usize()).collect()}))
}

fn mapper(input: &str) -> IResult<&str, Mapper> {
    //seed-to-soil map:
    let (input, src) = alpha1(input)?;
    let (input, _) = tag("-to-")(input)?;
    let (input, dst) = alpha1(input)?;
    let (input, _) = tag(" map:\n")(input)?;
    let (input, mappings) = separated_list1(tag("\n"), mapping)(input)?;

    Ok((&input, Mapper{dst, src, mappings}))
}

fn mapping(input: &str) -> IResult<&str, Mapping> {
    //0 15 37
    let (input, dst_start) = complete::u32(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, src_start) = complete::u32(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, map_size) = complete::u32(input)?;

    Ok((&input, Mapping{src_start:src_start.to_usize(), dst_start:dst_start.to_usize(), map_size:map_size.to_usize()}))
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
        assert_eq!(result, 35);
    }
}
