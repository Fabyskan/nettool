use std::net::Ipv4Addr;

fn main() {
    let adresse = Ipv4Addr::new(10, 0, 4, 0);
    let adresse_bits = adresse.to_bits();
    let adresse_oktett = adresse.octets();
    println!("{adresse}");
    println!("{:032b}", adresse_bits);
    println!("{:?}", adresse_oktett);
}
