use super::*;

#[test]
fn validates_title() {
    let mut input = RecipeInput {
        title: "  ".into(),
        description: String::new(),
        ingredients: String::new(),
        instructions: String::new(),
    };
    assert!(input.validate().is_err());
    input.title = "Soup".into();
    assert!(input.validate().is_ok());
    input.title = "x".repeat(201);
    assert!(input.validate().is_err());
}
