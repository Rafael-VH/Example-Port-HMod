// Not ported yet: the Java @Inject(HEAD) on class_2261.method_9558 (MixinBushBlock.canSurvive).
// Why it's off: No Horizon function mapped for class_2261.method_9558 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for class_2261.method_9558: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   class_2248.method_9564 -> (not mapped yet)
//   class_2261.method_9558 -> (not mapped yet)
//   class_2338.method_10074 -> (not mapped yet)
//   class_2680.method_11654 -> (not mapped yet)
//   class_2680.method_26204 -> (not mapped yet)
//   class_4538.method_8320 -> (not mapped yet)
//
// Decompiled Java, com/dtteam/dynamictrees/mixin/MixinBushBlock.java:23:
// | @Inject(at = @At("HEAD"), cancellable = true, method = "method_9558(Lnet/minecraft/class_2680;Lnet/minecraft/class_4538;Lnet/minecraft/class_2338;)Z")
// | private void canSurvive(class_2680 state, class_4538 level, class_2338 pos, CallbackInfoReturnable<Boolean> cir) {
// |    class_2338 blockpos = pos.method_10074();
// |    class_2680 belowBlockState = level.method_8320(blockpos);
// |    if (belowBlockState.method_26204() instanceof BasicRootsBlock roots
// |       && belowBlockState.method_11654(BasicRootsBlock.LAYER) == BasicRootsBlock.Layer.COVERED) {
// |       class_2248 block = BasicRootsBlock.Layer.COVERED.getPrimitive(roots.getFamily()).orElse(null);
// |       if (block == null) {
// |          return;
// |       }
// |
// |       cir.setReturnValue(this.method_9695(block.method_9564(), level, blockpos));
// |    }
// | }
