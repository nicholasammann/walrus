use std::collections::HashMap;
use std::string::String;

#[derive(Debug)]
pub enum PropertyValue {
  Int(i32),
  Float(f32),
  Str(String)
}


// Game
//// Level


// Level
//// Objects
#[derive(Debug)]
pub struct Level {
  pub name: String,
  pub objects: HashMap<String, Object>
}


// Object
//-- Name
//-- Components
//-- Traits for adding, removing components
#[derive(Debug)]
pub struct Object {
  pub name: String,
  pub components: HashMap<String, Component>
}


// Component
//-- Name
//-- Properties
//-- Traits for adding, removing, setting, getting properties
#[derive(Debug)]
pub struct Component {
  pub name: String,
  pub properties: HashMap<String, ComponentProperty>
}


// Properties
//-- Name
//-- Value
//-- Traits for setting/getting name and value
#[derive(Debug)]
pub struct ComponentProperty {
  pub name: String,
  pub value: PropertyValue
}
