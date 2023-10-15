use std::string::String;

mod level;
mod object;
mod component;
mod component_property;

trait GetName {
  fn get_name(&self) -> &str;
}

trait SetName {
  fn set_name(&mut self, new_name: String);
}
