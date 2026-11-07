// Not ported yet: the Java @Inject(RETURN) on CrafterBlock.getPotentialResults (CrafterBlockMixin.graveyard$skipStaffRecipes).
// Why it's off: No Horizon function mapped for CrafterBlock.getPotentialResults (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for CrafterBlock.getPotentialResults: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CrafterBlock.getPotentialResults -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/CrafterBlockMixin.java:17:
// | @Inject(method = "getPotentialResults", at = @At("RETURN"), cancellable = true)
// | private static void graveyard$skipStaffRecipes(ServerLevel level, CraftingInput input, CallbackInfoReturnable<Optional<RecipeHolder<CraftingRecipe>>> info) {
// |    if (((Optional)info.getReturnValue()).filter(StaffRecipeLock::isStaffRecipe).isPresent()) {
// |       info.setReturnValue(Optional.empty());
// |    }
// | }
