use crate::pings::IpProperties;

struct Stats {
    up: usize,
    down: usize,
    up_percentage: f64,
    average_rtt: f64,
    ms_per_chunk: u128,
}

impl Clone for Stats {
    fn clone(&self) -> Self {
        return Stats {
            up: self.up,
            down: self.down,
            up_percentage: self.up_percentage,
            average_rtt: self.average_rtt,
            ms_per_chunk: self.ms_per_chunk,
        };
    }
}

impl ToString for Stats {
    fn to_string(&self) -> String {
        return format!(
            "Up: {}, Down: {}, Up percentage: {}%, Average RTT: {} ms, {} ms per chunk",
            self.up, self.down, self.up_percentage, self.average_rtt, self.ms_per_chunk
        );
    }
}

pub fn sort_up_from_down(properties: &Vec<IpProperties>) -> (Vec<IpProperties>, Vec<IpProperties>) {
    let mut up: Vec<IpProperties> = Vec::new();
    let mut down: Vec<IpProperties> = Vec::new();
    for prop in properties {
        if prop.up {
            up.push(prop.clone());
        } else {
            down.push(prop.clone());
        }
    }
    return (up, down);
}

fn get_up_percentage(up: &Vec<IpProperties>, down: &Vec<IpProperties>) -> f64 {
    // compute the percentage of up addresses
    let total = up.len() + down.len();
    let up_percentage = up.len() as f64 / total as f64 * 100.0;
    return up_percentage;
}

fn get_average_rtt(properties: &Vec<IpProperties>) -> f64 {
    let mut total = 0;
    let mut count = 0;
    for prop in properties {
        if prop.up {
            total += prop.rtt.as_millis() as i64;
            count += 1;
        }
    }
    return total as f64 / count as f64;
}

fn update_global(chunk_stats: &Stats) -> Stats {
    static mut STATS: Stats = Stats {
        up: 0,
        down: 0,
        up_percentage: 0.0,
        average_rtt: 1.0,
        ms_per_chunk: 0,
    };
    static mut TREATED_CHUNKS: usize = 1;
    let updated: Stats;
    unsafe {
        if STATS.average_rtt.is_nan() {
            STATS.average_rtt = 1.0;
        }
        STATS.up += chunk_stats.up;
        STATS.down += chunk_stats.down;
        STATS.up_percentage = STATS.up as f64 / (STATS.up + STATS.down) as f64 * 100.0;
        STATS.average_rtt = (STATS.average_rtt * (TREATED_CHUNKS as f64) + chunk_stats.average_rtt) / ((TREATED_CHUNKS + 1) as f64);
        STATS.ms_per_chunk = (STATS.ms_per_chunk * (TREATED_CHUNKS as u128) + chunk_stats.ms_per_chunk) / (TREATED_CHUNKS as u128 + 1);
        updated = STATS.clone();
        TREATED_CHUNKS += 1;
    }
    updated
}


fn get_chunk_stats(properties: &Vec<IpProperties>, elapsed: u128) -> Stats {
    let (up, down) = sort_up_from_down(properties);
    let up_percentage = get_up_percentage(&up, &down);
    let average_rtt = get_average_rtt(properties);
    Stats {
        up: up.len(),
        down: down.len(),
        up_percentage,
        average_rtt,
        ms_per_chunk: elapsed,
    }
}

pub fn show_stats(properties: &Vec<IpProperties>, elapsed: u128) {
    let chunk_stats = get_chunk_stats(properties, elapsed);
    let global_stats = update_global(&chunk_stats);
    // print the global stats once every 32 chunks
    let chunk_size = chunk_stats.up + chunk_stats.down;
    let total = global_stats.up + global_stats.down;
    if total % (chunk_size * 16) == 0 {
        println!("{}", global_stats.to_string());
    }
}
