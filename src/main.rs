use hostname::get;
use local_ip_address::local_ip;
use mac_address::get_mac_address;

fn main() {
    let device_mac = get_mac_address().expect("Could not get MAC address.");
    println!("MAC address: {:#?}", device_mac.unwrap().to_string());

    let device_ip = local_ip().expect("Could not get IP address.");
    println!("IP address: {:#?}", device_ip.to_string());

    let hn = get();
    println!("Hostname: {:#?}", hn.unwrap());
}
