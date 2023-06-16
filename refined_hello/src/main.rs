fn hello_world() {
    println!("Hello, world!");

    let germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";

    let regions = [germany, japan];

    for region in regions.iter() {
        println!("{}", &region);
        // Can also use just region, instead of &region
        // The & refers to borrowed value
    }
}

fn main() {
    hello_world();
}
