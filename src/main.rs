mod input;
mod year2024;

use dotenv::dotenv;
use input::get_input;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let year = "2024";
    let day = "1";
    let input = get_input(year, day).await.unwrap();
    match year {
        "2024" => year2024::solve(day, input),
        _ => println!("Year {year} is not solved yet")
    }
    Ok(())
}
