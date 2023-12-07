use std::fs;

fn iterate_file(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Something went wrong reading the file");
}

fn get_seeds(file: &str) -> Vec<u64> {
    let lines: Vec<&str> = file.split("\n").collect();
    lines[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|seed_number| seed_number.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn get_seeds_from_range(file: &str) -> Vec<u64> {
    let lines: Vec<&str> = file.split("\n").collect();
    return lines[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|seed_number| seed_number.parse::<u64>().unwrap())
        .fold(Vec::<Vec<u64>>::new(), |mut range, number| {
            if let Some(last_range) = range.last_mut() {
                if last_range.len() == 1 {
                    last_range.push(number);
                    return range;
                }
            }
            range.push(vec![number]);
            return range;
        })
        .iter()
        .map(|pair: &Vec<u64>| {
            return (pair[0]..(pair[0] + pair[1])).collect::<Vec<u64>>();
        })
        .fold(Vec::<u64>::new(), |mut range, mut pair| {
            range.append(&mut pair);
            return range;
        });
}

#[derive(Debug)]
struct SinglePath {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

fn get_maps(file: &str) -> Vec<Vec<SinglePath>> {
    return file
        .split("\n\n")
        .skip(1)
        .map(|map| {
            map.split("\n")
                .skip(1)
                .map(|single_path_string| {
                    let single_path_chunks = single_path_string
                        .split(" ")
                        .map(|number| number.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    return SinglePath {
                        destination_start: single_path_chunks[0],
                        source_start: single_path_chunks[1],
                        length: single_path_chunks[2],
                    };
                })
                .collect()
        })
        .collect();
}

fn get_lowest_location(file: &str, seeds: Vec<u64>) -> u64 {
    let maps: Vec<Vec<SinglePath>> = get_maps(&file);

    seeds
        .iter()
        .map(|seed: &u64| {
            maps.iter().fold(*seed, |location, single_paths| {
                single_paths
                    .iter()
                    .fold(
                        (location, false),
                        |(location, found), single_path: &SinglePath| {
                            if found {
                                return (location, found);
                            }

                            if single_path.source_start <= location
                                && single_path.source_start + single_path.length > location
                            {
                                return (
                                    (location - single_path.source_start)
                                        + single_path.destination_start,
                                    true,
                                );
                            }
                            return (location, false);
                        },
                    )
                    .0
            })
        })
        .min()
        .unwrap()
}

/**
 * Line 1 - seeds
 */
pub fn part1(filename: &str) -> u32 {
    let file: String = iterate_file(filename);
    let seeds: Vec<u64> = get_seeds(&file);

    get_lowest_location(&file, seeds) as u32
}

/**
 * Very Very slow solution
 * TODO: Optimize
 */
pub fn part2(filename: &str) -> u32 {
    let file: String = iterate_file(filename);
    let seeds: Vec<u64> = get_seeds_from_range(&file);

    get_lowest_location(&file, seeds) as u32
}
