use std::fmt::Display;

struct MyInternalError;

impl Display for MyInternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ooops!")
    }
}

fn it_wont_fail() -> Result<(), MyInternalError> {
    Ok(())
}

fn it_fails() -> Result<(), std::io::Error> {
    std::fs::read_to_string("missing_file.txt")?;
    Ok(())
}

#[derive(PartialEq, Eq, Debug)]
pub struct MyError {
    message: String,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "something's not right! ({})", self.message)
    }
}

pub fn it_runs_all(something: &str) -> Result<(), MyError> {
    let map_err = |e: &dyn Display| MyError {
        message: format!("{} -> {}", e, something),
    };
    it_wont_fail().map_err(|e| map_err(&e))?;
    it_fails().map_err(|e| map_err(&e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "something's not right! (No such file or directory (os error 2) -> something)"
                .to_string(),
            format!("{}", it_runs_all("something").err().unwrap())
        )
    }
}
