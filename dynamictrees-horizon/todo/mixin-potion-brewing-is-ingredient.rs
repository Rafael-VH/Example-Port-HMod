// Not ported yet: the Java @Inject(HEAD) on class_1845.method_8077 (MixinPotionBrewing.isIngredient).
// Why it's off: No Horizon function mapped for class_1845.method_8077 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for class_1845.method_8077: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   class_1799.method_7909 -> (not mapped yet)
//   class_1799.method_7960 -> (not mapped yet)
//   class_1845.method_59700 -> (not mapped yet)
//   class_1845.method_8072 -> (not mapped yet)
//   class_1845.method_8077 -> (not mapped yet)
//   class_1845.method_8078 -> (not mapped yet)
//
// Decompiled Java, com/dtteam/dynamictrees/mixin/MixinPotionBrewing.java:15:
// | @Inject(at = @At("HEAD"), method = "method_8077(Lnet/minecraft/class_1799;)Z", cancellable = true)
// | private void isIngredient(class_1799 stack, CallbackInfoReturnable<Boolean> cir) {
// |    if (DendroPotionRecipeHandler.isIngredient(stack)) {
// |       cir.setReturnValue(true);
// |    }
// | }
