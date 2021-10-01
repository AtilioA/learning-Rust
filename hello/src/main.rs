fn main() {
    for x in (2..100).rev() {
        println!("{} pissjars on the wall. {} pissjars.", x, x);
        println!("Take one down, pass it around.");
        println!("{} pissjars on the wall.", x - 1);
    }
    println!("{x} pissjars on the wall. {x} pissjars.", x = 1);
    println!("Take one down, pass it around.");
    println!("No more pissjars on the wall :(.");
}
