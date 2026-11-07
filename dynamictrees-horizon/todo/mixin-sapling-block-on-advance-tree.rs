// Not ported yet: the Java @Inject(HEAD) on class_2473.method_10507 (MixinSaplingBlock.onAdvanceTree).
// Why it's off: No Horizon function mapped for class_2473.method_10507 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for class_2473.method_10507: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   class_2473.method_10507 -> (not mapped yet)
//   class_2680.method_26204 -> (not mapped yet)
//   class_3218.method_8650 -> (not mapped yet)
//
// Decompiled Java, com/dtteam/dynamictrees/mixin/MixinSaplingBlock.java:19:
// | @Inject(
// |    method = "method_10507(Lnet/minecraft/class_3218;Lnet/minecraft/class_2338;Lnet/minecraft/class_2680;Lnet/minecraft/class_5819;)V",
// |    at = @At("HEAD"),
// |    cancellable = true
// | )
// | private void onAdvanceTree(class_3218 level, class_2338 pos, class_2680 state, class_5819 random, CallbackInfo ci) {
// |    if ((Boolean)DTConfigs.COMMON.replaceVanillaSaplings.get()) {
// |       class_2248 block = state.method_26204();
// |       if (DynamicSaplingBlock.SAPLING_REPLACERS.containsKey(block)) {
// |          Species species = DynamicSaplingBlock.SAPLING_REPLACERS.get(block).selfOrLocationOverride(level, pos);
// |          level.method_8650(pos, false);
// |          ci.cancel();
// |          if (species.isValid() && DynamicSaplingBlock.canSaplingStay(level, species, pos)) {
// |             species.transitionToTree(level, pos);
// |          }
// |       }
// |    }
// | }
