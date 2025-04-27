// TODO: Make this actually usable later
fn main() {
    let mut args = std::env::args();
    match args.nth(1).expect("doodoo brain, no arg detected").as_str() {
        "tar" => {
            println!("pack   -> tar -czvf archive.tar.gz /path/to/directory_or_file");
            println!("unpack -> tar -xzvf archive_name.tar.gz");
        }
        _ => println!("don't know this one bruh"),
    }
}
