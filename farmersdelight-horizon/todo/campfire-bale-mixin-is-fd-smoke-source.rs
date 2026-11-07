// Not ported yet: the Java @Inject(HEAD) on CampfireBlock.isSmokeSource (CampfireBaleMixin.isFDSmokeSource).
// Why it's off: No Horizon function mapped for CampfireBlock.isSmokeSource (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for CampfireBlock.isSmokeSource: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CampfireBlock.isSmokeSource -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/CampfireBaleMixin.java:13:
// | @Inject(at = @At("HEAD"), method = "isSmokeSource", cancellable = true)
// | public void isFDSmokeSource(BlockState state, CallbackInfoReturnable<Boolean> cir) {
// |    if (state.is(ModTags.Blocks.CAMPFIRE_SIGNAL_SMOKE)) {
// |       cir.setReturnValue(true);
// |    }
// | }
