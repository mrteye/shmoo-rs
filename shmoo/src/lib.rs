//! # shmoo
//!
//! `shmoo`, all things.

pub use self::stuff::hey;

pub mod stuff {
  use rand::Rng; // Import the Rng trait

  /// Identify a Shmoo!
  ///
  /// # Examples
  ///
  /// ```
  /// let arg = "hotdog";
  /// let answer = shmoo::hey(arg);
  /// assert!(answer.starts_with("Hey hotdog, yer a Shmoo ("));
  /// ```
  pub fn hey(noun: &str) -> String {
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(0..=43);
    format!("Hey {}, yer a Shmoo ({})!", noun, random_number)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hey() {
    let arg = "hotdog";
    let answer = stuff::hey(arg);
    assert!(answer.starts_with("Hey hotdog, "));
  }
}
