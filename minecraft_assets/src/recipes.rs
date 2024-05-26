//! A Module containing data structures for a recipe
//!
//! # Sources
//! - [minecraft.fandom.com](https://minecraft.fandom.com/wiki/Recipe)

/// A struct containing the common data that every recipe stores
#[derive(PartialEq, Debug)]
pub struct CommonRecipeData {
    /// The type of the recipe
    pub recipe_type: String,
    /// The group inside of the recipe book
    pub group: String,
}

/// A struct for a blast furnace recipe
#[derive(PartialEq, Debug)]
pub struct BlastFurnaceRecipe {
    /// The common recipe data
    pub common: CommonRecipeData,
    /// The ingredient
    pub ingredient: Ingredient,
    /// Acceptable ingredients
    pub ingredients: Vec<Ingredient>,
    /// The id of the result item
    pub result: String,
    /// The output of experience
    pub experience: f64,
    /// The cooking time
    ///
    /// # Note
    ///
    /// This defaults to `100` ticks
    pub cooking_time: Option<i32>,
}

/// A struct for ingredient data
#[derive(PartialEq, Debug)]
pub struct Ingredient {
    /// The id of the item
    pub item: String,
    /// The tag of the item
    pub tag: String,
}
