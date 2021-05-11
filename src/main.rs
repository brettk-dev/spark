mod generator;

extern crate clap;

use clap::{App, Arg};

fn main() {
  let matches = App::new("Spark")
    .version("0.1")
    .author("Brett Kinnamon <bdkinna@gmail.com>")
    .about("Generates components with tests.")
    .arg(Arg::with_name("framework"))
    .arg(Arg::with_name("name"))
    .get_matches();

  let framework = matches.value_of("framework").unwrap_or("vue");
  let framework = match framework {
    "vue" => Some(generator::Framework::Vue),
    _ => None,
  };
  let name = matches.value_of("name").unwrap_or("HelloWorld");
  generator::generate_component(
    framework.unwrap_or(generator::Framework::Vue),
    name,
    Some("./output"),
  );
}
