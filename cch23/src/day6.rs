//! Elf on a shelf

//! It's that time of year when the elves hide on shelves to watch
//! over the children of the world, reporting back to Santa on who's
//! been naughty or nice. However, this year's reports have been mixed
//! up with the rest of the letters to Santa, and the word "elf" is
//! hidden throughout a mountain of text.

/// Take a POST request with a raw string as input and count:
/// - how many times the substring "elf" appears
/// - the number of occurrences of the string "elf on a shelf"
/// - the number of shelves that don't have an elf on it
pub async fn runner(input: String) -> axum::Json<Response> {
    task(&input).into()
}
fn task(input: &str) -> Response {
    let elves = input.matches("elf").count();
    let shelves = input.matches("shelf").count();
    let elf_ful = input.matches("elf on a shelf").count();
    let elf_less = shelves - elf_ful;

    Response {
        elf: elves,
        elf_less,
        elf_ful,
    }
}

#[derive(serde::Serialize)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct Response {
    /// how many times the substring "elf" appears
    elf: usize,
    /// The number of occurrences of the string "elf on a shelf"
    #[serde(rename = "elf on a shelf")]
    elf_ful: usize,
    /// The number of shelves that don't have an elf on it
    #[serde(rename = "shelf with no elf on it")]
    elf_less: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn task1() {
        assert_eq!(
            task(
                "The mischievous elf peeked out from behind the toy workshop,
      and another elf joined in the festive dance.
      Look, there is also an elf on that shelf!"
            )
            .elf,
            4
        );
    }
    #[test]
    fn task2() {
        assert_eq!(
            task(
                "there is an elf on a shelf on an elf.
      there is also another shelf in Belfast."
            ),
            Response {
                elf: 5,
                elf_ful: 1,
                elf_less: 1
            }
        );
    }
}
