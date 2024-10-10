//! # shmoo
//!
//! `shmoo`, all things.

pub use self::stuff::hey;

pub mod stuff {
  /// Identify a Shmoo!
  ///
  /// # Examples
  ///
  /// ```
  /// let arg = "hotdog";
  /// let answer = shmoo::hey(arg);
  ///
  /// assert_eq!("Hey hotdog, yer a Shmoo!", answer);
  /// ```
  pub fn hey(noun: &str) -> String {
    format!("Hey {}, yer a Shmoo!", noun)
  }
}
