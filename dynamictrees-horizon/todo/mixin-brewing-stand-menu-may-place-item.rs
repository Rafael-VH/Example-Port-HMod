// Not ported yet: the Java @Inject(HEAD) on class_1711.method_7631 (MixinBrewingStandMenu.mayPlaceItem).
// Why it's off: No Horizon function mapped for class_1711.method_7631 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for class_1711.method_7631: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   class_1711.method_7631 -> (not mapped yet)
//   class_1799.method_7909 -> (not mapped yet)
//
// Decompiled Java, com/dtteam/dynamictrees/mixin/MixinBrewingStandMenu.java:13:
// | @Inject(at = @At("HEAD"), method = "method_7631(Lnet/minecraft/class_1799;)Z", cancellable = true)
// | private static void mayPlaceItem(class_1799 stack, CallbackInfoReturnable<Boolean> cir) {
// |    if (stack.method_7909() instanceof DendroPotion) {
// |       cir.setReturnValue(true);
// |    }
// | }
