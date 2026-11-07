// Not ported yet: the Java @Inject(HEAD) on ChestSpecialRenderer.submit (Mixin_ChestSpecialRenderer_PassState.emf$dummyState).
// Why it's off: No Horizon function mapped for ChestSpecialRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ChestSpecialRenderer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChestSpecialRenderer.submit -> (not mapped yet)
//   BlockPos.ZERO -> (not mapped yet)
//   Block.defaultBlockState -> (not mapped yet)
//   Blocks.CHEST -> (not mapped yet)
//   ChestBlockEntity.<init> -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/submit/backupstates/Mixin_ChestSpecialRenderer_PassState.java:17:
// | @Inject(method = "submit", at = @At("HEAD"))
// | private static void emf$dummyState(CallbackInfo ci) {
// |    ETFEntityRenderState state = ETFEntityRenderState.forEntity((ETFEntity)(new ChestBlockEntity(BlockPos.ZERO, Blocks.CHEST.defaultBlockState())));
// |    ETFState.mount(state);
// | }
