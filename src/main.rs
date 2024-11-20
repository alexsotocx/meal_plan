pub mod schema;
mod meal_plan;
use meal_plan::models::{Family, User};

fn main() {
    let user = User {
      name: "Hi".to_string(),
      password: "test".to_string(),
      family_id: 1,
    };

    let family = Family {
      id: 1,
    };
    println!("Hello, world! User {:?}", user);
    println!("Hello, world! Family {:?}", family);
}
