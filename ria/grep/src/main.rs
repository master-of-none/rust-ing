fn main() {
    let search_term = "picture";

    let quote = "\
        Every photo, every picture, every face, bedroom window, is a picture in search of what
        It is same with books.
        What do we seek, through million of pages.
        ";

    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}", line);
        }
    }
}
