mod core;

use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
  let mut keyboard_input = String::new();

  let mut level_one = core::Level {
    name: "LevelOne".to_string(),
    objects: HashMap::new()
  };

  loop {
    println!("");
    println!("----------");
    println!("Commands");
    println!("----------");
    println!("Make: Creates an object and adds it to the level");
    println!("Show: Debug prints the level");
    println!("Exit: Closes the game");
    println!("----------");
    println!("");

    io::stdout().flush().unwrap();

    keyboard_input.clear();
    io::stdin().read_line(&mut keyboard_input).unwrap();

    if keyboard_input.eq("Exit\n") {
      println!("Exiting game  :)");
      break;
    }
    else if keyboard_input.eq("Make\n") {
      let mut object_one = core::Object {
        name: "ObjectOne".to_string(),
        components: HashMap::new()
      };

      let mut component_one = core::Component {
        name: "ComponentOne".to_string(),
        properties: HashMap::new()
      };

      let int_property_one = core::ComponentProperty {
        name: "IntPropertyOne".to_string(),
        value: core::PropertyValue::Int(5)
      };
      
      let float_property_one = core::ComponentProperty {
        name: "FloatPropertyOne".to_string(),
        value: core::PropertyValue::Float(3.41)
      };

      let str_property_one = core::ComponentProperty {
        name: "StrPropertyOne".to_string(),
        value: core::PropertyValue::Str("Enums Worked!".to_string())
      };

      component_one.properties.insert("IntPropertyOne".to_string(), int_property_one);
      component_one.properties.insert("FloatPropertyOne".to_string(), float_property_one);
      component_one.properties.insert("StrPropertyOne".to_string(), str_property_one);

      object_one.components.insert("ComponentOne".to_string(), component_one);

      level_one.objects.insert("ObjectOne".to_string(), object_one);
    }
    else if keyboard_input.eq("Show\n") {
      println!("{:#?}", level_one);
    }
    else {
      println!("{keyboard_input}");
    }
  }
}
