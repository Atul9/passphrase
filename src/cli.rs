use crate::list::model::WordList;

use clap::{App, Arg, crate_name, crate_version, crate_authors};

pub fn app(lists: &'b [WordList]) -> App<'a, 'b> {
  let names: Vec<&str> = lists.iter()
    .flat_map(|x| &x.short_names)
    .map(|x| x.as_str())
    .collect();

  App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about("Generates passphrases using dice rolls on word lists a la https://www.eff.org/dice.")
    .after_help("Please see https://www.eff.org/dice to learn about the word lists embedded in this program.")
    .version_short("v")
    .version_message("print version information")
    .help_message("print help information")
    .arg(Arg::with_name("list")
      .short("l")
      .long("list")
      .help("which list to pick words from")
      .takes_value(true)
      .possible_values(&names)
      .default_value("long"))
    .arg(Arg::with_name("words")
      .short("w")
      .long("words")
      .help("how many words to use in the passphrase")
      .takes_value(true)
      .default_value("6"))
    .arg(Arg::with_name("passphrases")
      .short("n")
      .long("amount")
      .help("how many passphrases to generate")
      .takes_value(true)
      .default_value("1"))
    .arg(Arg::with_name("separator")
      .short("s")
      .long("separator")
      .help("which character (or characters) to separate the words in the passphrases")
      .takes_value(true)
      .default_value("."))
    .arg(Arg::with_name("verbose")
      .long("verbose")
      .help("turn on verbose mode"))
    .arg(Arg::with_name("lists")
      .short("L")
      .long("lists")
      .help("print available lists and their information then exit")
      .conflicts_with_all(&["separator", "amount", "words", "list", "passphrases"]))
}
