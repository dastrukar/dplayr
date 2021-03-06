mod file;

fn main() -> std::io::Result<()> {
    println!("dplay");

    if file::check_config() {
        println!("\nNo config file found. \nCreating a new one");
        file::create_config();

        println!("\nExiting! Please edit your config file.");
        return Ok(());
    } else {
        println!("\nExisting config file found.");
    }

    Ok(())
}
