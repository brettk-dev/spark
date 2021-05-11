mod generator;

extern crate clap;

use clap::{App, Arg, SubCommand};
// use std::fs::File;
// use std::io::prelude::*;

fn vue_component() -> String {
  return "<template>
  <div>
  </div>
</template>

<script>
export default {
  setup() {
    return {};
  },
};
</script>

<style scoped>
</style>
"
  .to_string();
}

fn vue_test(name: &str) -> String {
  return format!(
    "import {{ shallowMount }} from '@vue/test-utils'
import {name} from './{name}.vue'

describe('HelloWorld', () => {{
  let component
  beforeEach(() => {{
    component = shallowMount({name})
  }})
}})
",
    name = name
  );
}

fn main() {
  let matches = App::new("Spark")
    .version("0.1")
    .author("Brett Kinnamon <bdkinna@gmail.com>")
    .about("Generates components with tests.")
    .subcommand(
      SubCommand::with_name("generate")
        .subcommand(SubCommand::with_name("vue").arg(Arg::with_name("name")))
        .subcommand(SubCommand::with_name("react").arg(Arg::with_name("name"))),
    )
    .get_matches();

  if let Some(matches) = matches.subcommand_matches("generate") {
    let framework = matches.subcommand_name().unwrap();

    if framework == "vue" {
      println!("Generating vue component ...");
      if let Some(matches) = matches.subcommand_matches(framework) {
        let component_name = matches.value_of("name").unwrap_or("HelloWorld");
        generator::generate_component(generator::Framework::Vue, component_name, None);

        // let component_filename = format!("./src/components/{}.vue", component_name);
        // let component_text = vue_component();
        // let mut component_file = match File::create(&component_filename) {
        //     Ok(f) => f,
        //     Err(why) => panic!("Could not create '{}': {}", component_filename, why),
        // };
        // match component_file.write_all(component_text.as_bytes()) {
        //     Ok(_) => println!("Successfully wrote {}", component_filename),
        //     Err(why) => panic!("Could not write to '{}': {}", component_filename, why),
        // }

        // let test_filename = format!("./src/components/{}.spec.vue", component_name);
        // let test_text = vue_test(component_name);
        // let mut test_file = match File::create(&test_filename) {
        //     Ok(f) => f,
        //     Err(why) => panic!("Could not create '{}': {}", test_filename, why),
        // };
        // match test_file.write_all(test_text.as_bytes()) {
        //     Ok(_) => println!("Successfully wrote {}", test_filename),
        //     Err(why) => panic!("Could not write to '{}': {}", test_filename, why),
        // }
      }
    }
    if framework == "react" {
      println!("Generating react component ...");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_vue_component() {
    assert_eq!(
      vue_component(),
      "<template>
  <div>
  </div>
</template>

<script>
export default {
  setup() {
    return {};
  },
};
</script>

<style scoped>
</style>
"
    );
  }

  #[test]
  fn test_vue_test() {
    assert_eq!(
      vue_test("HelloWorld"),
      "import { shallowMount } from '@vue/test-utils'
import HelloWorld from './HelloWorld.vue'

describe('HelloWorld', () => {
  let component
  beforeEach(() => {
    component = shallowMount(HelloWorld)
  })
})
"
    )
  }
}
