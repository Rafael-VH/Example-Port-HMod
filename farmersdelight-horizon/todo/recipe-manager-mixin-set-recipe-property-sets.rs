// Not ported yet: the Java @Accessor on RecipeManager.RECIPE_PROPERTY_SETS (RecipeManagerMixin.fdrf$setRECIPE_PROPERTY_SETS).
// Why it's off: No Horizon function mapped for RecipeManager.RECIPE_PROPERTY_SETS (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for RecipeManager.RECIPE_PROPERTY_SETS: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Holder.value -> (not mapped yet)
//   ItemStackTemplate.item -> (not mapped yet)
//   Ingredient.of -> (not mapped yet)
//   RecipeManager.<clinit> -> (not mapped yet)
//   RecipeManager.RECIPE_PROPERTY_SETS -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/RecipeManagerMixin.java:31:
// | @Accessor("RECIPE_PROPERTY_SETS")
// | @Mutable
// | private static void fdrf$setRECIPE_PROPERTY_SETS(Map<ResourceKey<RecipePropertySet>, IngredientExtractor> value) {
// |    throw new RuntimeException("");
// | }
