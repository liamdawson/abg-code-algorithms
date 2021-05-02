pub(crate) mod golem;
pub(crate) mod ingredients;
mod reference;
mod web;

pub use golem::{GolemAssignments, GOLEM_VALUES};
pub use ingredients::{IngredientAssignments, ORDERED_ALCHEMICALS};
pub use reference::{decode, DecodeError};
pub use web::match_web;

#[derive(Clone, Debug)]
pub struct CodeAssignments {
    pub ingredients: IngredientAssignments,
    pub golem: Option<GolemAssignments>,
}
