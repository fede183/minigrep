use art::PrimaryColor;
use art::SecondaryColor;
use art::mix;

fn main() {
    let c1 = PrimaryColor::Red;
    let c2 = PrimaryColor::Yellow;
    let result : SecondaryColor = mix(c1, c2);
    match result {
        SecondaryColor::Orange =>
            println!("mix color: Orange"),
        _ => {},
    }
}
