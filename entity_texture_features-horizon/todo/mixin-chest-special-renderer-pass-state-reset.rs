// Not ported yet: the Java @Inject(TAIL) on ChestSpecialRenderer.submit (Mixin_ChestSpecialRenderer_PassState.emf$reset).
// Why it's off: No Horizon function mapped for ChestSpecialRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for ChestSpecialRenderer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChestSpecialRenderer.submit -> (not mapped yet)
//   BlockPos.ZERO -> (not mapped yet)
//   Block.defaultBlockState -> (not mapped yet)
//   Blocks.CHEST -> (not mapped yet)
//   ChestBlockEntity.<init> -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/submit/backupstates/Mixin_ChestSpecialRenderer_PassState.java:23:
// | @Inject(method = "submit", at = @At("TAIL"))
// | private static void emf$reset(CallbackInfo ci) {
// |    ETFState.unMount();
// | }
