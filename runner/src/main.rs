use helper::Error;

const README_HEADER: &str = "[Everybody Codes](https://everybody.codes/)
Michael Conrad";

fn main() -> Result<(), Error> {
    helper::runner::main::<_, 3>(false, README_HEADER, |runners| {
        ebc_2024::register(runners);
    })
}
