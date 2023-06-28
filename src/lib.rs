mod transpiler;

// Struct that will have access to the methods defined from messages
pub struct Robot {}

// Generative methods to be called in a build file
pub use transpiler::{generate_robot_code, generate_robot_code_from_str};
