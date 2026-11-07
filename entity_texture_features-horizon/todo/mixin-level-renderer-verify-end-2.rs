// Not ported yet: the Java @Inject(TAIL) on LevelRenderer.submitBlockEntities (MixinLevelRenderer.emf$verifyEnd).
// Why it's off: No Horizon function mapped for LevelRenderer.submitBlockEntities (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for LevelRenderer.submitBlockEntities: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Iterator.next -> (not mapped yet)
//   LevelRenderer.submitBlockEntities -> (not mapped yet)
//   LevelRenderer.submitEntities -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/MixinLevelRenderer.java:17:
// | @Inject(method = {"submitEntities", "submitBlockEntities"}, at = @At("TAIL"))
// | private void emf$verifyEnd(CallbackInfo ci) {
// |    ETFState.stackVerifyEmpty();
// | }
