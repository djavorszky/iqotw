use iqotw::y23::dice;

fn main() -> Result<(), String> {
    run_dice()
}

fn run_dice() -> Result<(), String> {
    let input = "1d8+3d12+2d6";

    println!("{}", dice::roll_dice(input));

    Ok(())
}
