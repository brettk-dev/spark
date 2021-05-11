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
    .arg(Arg::with_name("dir"))
    .get_matches();

  let framework = matches.value_of("framework").unwrap_or("vue");
  let framework = match framework {
    "vue" => Some(generator::Framework::Vue),
    "react" => Some(generator::Framework::React),
    "svelte" => Some(generator::Framework::Svelte),
    _ => None,
  };
  let name = matches.value_of("name").unwrap_or("HelloWorld");
  let dir = matches.value_of("dir");
  generator::generate_component(framework.unwrap_or(generator::Framework::Vue), name, dir);
}
