use std::fs::File;
use std::io::prelude::*;

pub enum Framework {
  Vue,
  React,
  Svelte,
}

pub fn generate_component(framework: Framework, name: &str, dir: Option<&str>) {
  let actual_dir = dir.unwrap_or("./src/components");
  match framework {
    Framework::Vue => generate_vue(name, actual_dir),
    Framework::React => generate_vue(name, actual_dir),
    Framework::Svelte => generate_vue(name, actual_dir),
  };
}

fn generate_filename(name: &str, dir: &str, ext: &str) -> String {
  let slashed_dir = if dir.ends_with("/") {
    dir.to_string()
  } else {
    format!("{}/", dir)
  };
  return format!("{}{}.{}", slashed_dir, name, ext);
}

fn create_file(filename: &str, text: &str) {
  let mut file = match File::create(&filename) {
    Ok(f) => f,
    Err(why) => panic!("Could not create '{}': {}", filename, why),
  };
  match file.write_all(text.as_bytes()) {
    Ok(_) => println!("Successfully wrote {}", filename),
    Err(why) => panic!("Could not write to '{}': {}", filename, why),
  }
}

fn generate_vue(name: &str, dir: &str) {
  println!("Generating a Vue component ...");

  let component_template = include_str!("./templates/vue/Component.vue");
  let test_template = include_str!("./templates/vue/Component.test.js");
  let test_contents = str::replace(test_template, "{name}", name);

  create_file(&generate_filename(name, dir, "vue"), &component_template);
  create_file(&generate_filename(name, dir, "spec.js"), &test_contents);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generate_filename_regular() {
    assert_eq!(
      generate_filename("TestName", "./src/components/", "vue"),
      "./src/components/TestName.vue"
    );
  }

  #[test]
  fn test_generate_filename_without_slash() {
    assert_eq!(
      generate_filename("TestName", "./src/components", "vue"),
      "./src/components/TestName.vue"
    );
  }

  #[test]
  fn test_generate_filename_irregular() {
    assert_eq!(generate_filename("One", "Two", "Three"), "Two/One.Three");
  }
}
