// Not ported yet: the Java @Inject(HEAD) on class_1845.method_8078 (MixinPotionBrewing.mix).
// Why it's off: No Horizon function mapped for class_1845.method_8078 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for class_1845.method_8078: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   class_1799.method_7909 -> (not mapped yet)
//   class_1799.method_7960 -> (not mapped yet)
//   class_1845.method_59700 -> (not mapped yet)
//   class_1845.method_8072 -> (not mapped yet)
//   class_1845.method_8077 -> (not mapped yet)
//   class_1845.method_8078 -> (not mapped yet)
//
// Decompiled Java, com/dtteam/dynamictrees/mixin/MixinPotionBrewing.java:39:
// | @Inject(at = @At("HEAD"), method = "method_8078(Lnet/minecraft/class_1799;Lnet/minecraft/class_1799;)Lnet/minecraft/class_1799;", cancellable = true)
// | private void mix(class_1799 ingredient, class_1799 potionBase, CallbackInfoReturnable<class_1799> cir) {
// |    for (DendroBrewingMix mix : DendroPotionRecipeHandler.getAllDendroRecipes()) {
// |       class_1799 result = mix.getOutput(potionBase, ingredient);
// |       if (!result.method_7960()) {
// |          cir.setReturnValue(result);
// |          return;
// |       }
// |    }
// | }
