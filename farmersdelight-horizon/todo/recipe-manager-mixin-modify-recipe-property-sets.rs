// Not ported yet: the Java @Inject(TAIL) on RecipeManager.<clinit> (RecipeManagerMixin.fdrf$modifyRecipePropertySets).
// Why it's off: No Horizon function mapped for RecipeManager.<clinit> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for RecipeManager.<clinit>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Holder.value -> (not mapped yet)
//   ItemStackTemplate.item -> (not mapped yet)
//   Ingredient.of -> (not mapped yet)
//   RecipeManager.<clinit> -> (not mapped yet)
//   RecipeManager.RECIPE_PROPERTY_SETS -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/RecipeManagerMixin.java:37:
// | @Inject(method = "<clinit>", at = @At("TAIL"))
// | private static void fdrf$modifyRecipePropertySets(CallbackInfo ci) {
// |    Map<ResourceKey<RecipePropertySet>, IngredientExtractor> map = new HashMap<>(RECIPE_PROPERTY_SETS);
// |    map.put(
// |       ModRecipePropertySets.COOKING_POT_INPUT_ONE,
// |       recipe -> recipe instanceof CookingPotRecipe cookingPotRecipe ? Optional.ofNullable(cookingPotRecipe.input().get(0)) : Optional.empty()
// |    );
// |    map.put(
// |       ModRecipePropertySets.COOKING_POT_INPUT_TWO,
// |       recipe -> recipe instanceof CookingPotRecipe cookingPotRecipe && cookingPotRecipe.input().size() >= 2
// |          ? Optional.ofNullable(cookingPotRecipe.input().get(1))
// |          : Optional.empty()
// |    );
// |    map.put(
// |       ModRecipePropertySets.COOKING_POT_INPUT_THREE,
// |       recipe -> recipe instanceof CookingPotRecipe cookingPotRecipe && cookingPotRecipe.input().size() >= 3
// |          ? Optional.ofNullable(cookingPotRecipe.input().get(2))
// |          : Optional.empty()
// |    );
// |    map.put(
// |       ModRecipePropertySets.COOKING_POT_INPUT_FOUR,
// |       recipe -> recipe instanceof CookingPotRecipe cookingPotRecipe && cookingPotRecipe.input().size() >= 4
// |          ? Optional.ofNullable(cookingPotRecipe.input().get(3))
// |          : Optional.empty()
// |    );
// |    map.put(
// |       ModRecipePropertySets.COOKING_POT_INPUT_FIVE,
// |       recipe -> recipe instanceof CookingPotRecipe cookingPotRecipe && cookingPotRecipe.input().size() >= 5
// |          ? Optional.ofNullable(cookingPotRecipe.input().get(4))
// |          : Optional.empty()
// |    );
// |    map.put(
// |       ModRecipePropertySets.COOKING_POT_INPUT_SIX,
// |       recipe -> recipe instanceof CookingPotRecipe cookingPotRecipe && cookingPotRecipe.input().size() >= 6
// |          ? Optional.ofNullable(cookingPotRecipe.input().get(5))
// |          : Optional.empty()
// |    );
// |    map.put(
// |       ModRecipePropertySets.COOKING_POT_CONTAINER,
// |       recipe -> recipe instanceof CookingPotRecipe cookingPotRecipe && cookingPotRecipe.container() != null
// |          ? Optional.of(Ingredient.of((ItemLike)cookingPotRecipe.container().item().value()))
// |          : Optional.empty()
// |    );
// |    map.put(
// |       ModRecipePropertySets.CUTTING_BOARD_INPUT,
// |       recipe -> recipe instanceof CuttingBoardRecipe cuttingBoardRecipe ? Optional.of(cuttingBoardRecipe.getInput()) : Optional.empty()
// |    );
// |    map.put(
// |       ModRecipePropertySets.CUTTING_BOARD_TOOL,
// |       recipe -> recipe instanceof CuttingBoardRecipe cuttingBoardRecipe ? Optional.of(cuttingBoardRecipe.getTool()) : Optional.empty()
// |    );
// |    fdrf$setRECIPE_PROPERTY_SETS(ImmutableMap.copyOf(map));
// | }
