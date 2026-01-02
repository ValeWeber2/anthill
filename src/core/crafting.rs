use crate::core::game::GameState;
use crate::core::game_items::GAmeItemId;

pub struct CraftingRecipe {
    pub input: Vec<GameItemId>,
    pub output: GameItemId,
}

impl GameState {
    pub fn craft(&mut self, recipe: &CraftingRecipe) -> bool {
        for item in &recipe.input {
            if !self.player.charchter.inventory.contains(item) {
                return false;
            }
        }

        for item in &recipe.input {
            let _ = self.remove_item_from_inv(*item);
        }

        let _ = self.add_item_to_inv(recipe.output);
        true
    }
}