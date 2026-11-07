// Not ported yet: the Java @Inject(HEAD) on CraftingMenu.stillValid (CraftingScreenHandlerMixin.canUse).
// Why it's off: No Horizon function mapped for CraftingMenu.stillValid (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for CraftingMenu.stillValid: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ContainerLevelAccess.evaluate -> (not mapped yet)
//   CraftingMenu.stillValid -> (not mapped yet)
//   Level.getBlockState -> (not mapped yet)
//   BlockState.getBlock -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/CraftingScreenHandlerMixin.java:20:
// | @Inject(method = "stillValid", at = @At("HEAD"), cancellable = true)
// | private void canUse(Player player, CallbackInfoReturnable<Boolean> info) {
// |    if ((Boolean)this.access.evaluate((world, pos) -> world.getBlockState(pos).getBlock() instanceof CraftingTableBlock, true)) {
// |       info.setReturnValue(true);
// |    }
// | }
