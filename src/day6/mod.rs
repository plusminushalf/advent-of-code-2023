fn binary_search(start: u64, end: u64, total_time: u64, distance: u64, lower: bool) -> u64 {
    let mid = (start + end) / 2;
    if mid == start || mid == end {
        return mid;
    }
    let time_left = total_time - mid;
    let distance_travelled = mid * time_left;
    if lower {
        if distance_travelled < distance {
            return binary_search(mid, end, total_time, distance, lower);
        } else if distance_travelled > distance {
            return binary_search(start, mid, total_time, distance, lower);
        } else {
            return mid;
        }
    } else {
        if distance_travelled > distance {
            return binary_search(mid, end, total_time, distance, lower);
        } else if distance_travelled < distance {
            return binary_search(start, mid, total_time, distance, lower);
        } else {
            return mid;
        }
    }
}

pub fn part1(_filename: &str) -> u32 {
    let times: Vec<u64> = vec![40828492];
    let distance: Vec<u64> = vec![233101111101487];

    let mut mul: u64 = 1;

    for i in 0..times.len() {
        let start = binary_search(0, times[i] / 2, times[i], distance[i], true);
        let end = binary_search(times[i] / 2, times[i], times[i], distance[i], false);
        let count = end - start;
        mul = mul * count;
    }

    return mul.to_string().parse::<u32>().unwrap();
}

pub fn part2(_filename: &str) -> u32 {
    return 5;
}
