// Not ported yet: the Java @Inject(INVOKE) on LevelRenderer.submitBlockEntities (MixinLevelRenderer.emf$verify).
// Why it's off: No Horizon function mapped for LevelRenderer.submitBlockEntities (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for LevelRenderer.submitBlockEntities: run `match`.
// Note: The call it hooks (Iterator.next) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Iterator.next -> (not mapped yet)
//   LevelRenderer.submitBlockEntities -> (not mapped yet)
//   LevelRenderer.submitEntities -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/MixinLevelRenderer.java:12:
// | @Inject(method = {"submitEntities", "submitBlockEntities"}, at = @At(value = "INVOKE", target = "Ljava/util/Iterator;next()Ljava/lang/Object;"))
// | private void emf$verify(CallbackInfo ci) {
// |    ETFState.stackVerifyEmpty();
// | }
