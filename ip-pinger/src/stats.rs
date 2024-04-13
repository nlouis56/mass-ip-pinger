use crate::pings::IpProperties;

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

pub fn show_stats(properties: &Vec<IpProperties>) {
    let (up, down) = sort_up_from_down(properties);
    let up_percentage = get_up_percentage(&up, &down);
    let average_rtt = get_average_rtt(properties);
    println!("Statistics on {} addresses:", properties.len());
    println!("Up: {}, Down: {}", up.len(), down.len());
    println!("Up percentage: {}", up_percentage.round());
    println!("Average RTT: {}", average_rtt);
}
