// Not ported yet: the Java @Inject(HEAD) on class_2589.method_5437 (MixinBrewingStandBlockEntity.canPlaceItem).
// Why it's off: No Horizon function mapped for class_2589.method_5437 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for class_2589.method_5437: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   class_1799.method_7909 -> (not mapped yet)
//   class_2589.method_5437 -> (not mapped yet)
//
// Decompiled Java, com/dtteam/dynamictrees/mixin/MixinBrewingStandBlockEntity.java:13:
// | @Inject(at = @At("HEAD"), method = "method_5437(ILnet/minecraft/class_1799;)Z", cancellable = true)
// | private void canPlaceItem(int slot, class_1799 stack, CallbackInfoReturnable<Boolean> cir) {
// |    if (slot != 3 && slot != 4 && stack.method_7909() instanceof DendroPotion) {
// |       cir.setReturnValue(true);
// |    }
// | }
