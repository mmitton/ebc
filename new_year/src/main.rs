use helper::new_year::{Config, Day};
use helper::Error;

fn main() -> Result<(), Error> {
    let config = Config::new("ebc", (1..=20).map(|day| Day::new(day, 3)).collect());

    helper::new_year::main(config)
}
