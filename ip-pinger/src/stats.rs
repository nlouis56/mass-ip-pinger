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

fn get_up_percentage(properties: &Vec<IpProperties>) -> f64 {
    let mut up = 0;
    for prop in properties {
        if prop.up {
            up += 1;
        }
    }
    return up as f64 / properties.len() as f64;
}

fn get_average_rtt(properties: &Vec<IpProperties>) -> f64 {
    let mut total = 0;
    for prop in properties {
        total += prop.rtt.as_millis();
    }
    return total as f64 / properties.len() as f64;
}

pub fn show_stats(properties: &Vec<IpProperties>) {
    let (up, down) = sort_up_from_down(properties);
    let up_percentage = get_up_percentage(properties);
    let average_rtt = get_average_rtt(properties);
    println!("Statistics on {} addresses:", properties.len());
    println!("Up: {}, Down: {}", up.len(), down.len());
    println!("Up percentage: {}", up_percentage);
    println!("Average RTT: {}", average_rtt);
}
