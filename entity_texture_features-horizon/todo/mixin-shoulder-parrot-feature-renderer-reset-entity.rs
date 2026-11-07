// Not ported yet: the Java @Inject(RETURN) on ParrotOnShoulderLayer.submitOnShoulder (MixinShoulderParrotFeatureRenderer.etf$resetEntity).
// Why it's off: No Horizon function mapped for ParrotOnShoulderLayer.submitOnShoulder (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for ParrotOnShoulderLayer.submitOnShoulder: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SubmitNodeCollector.submitModel -> (not mapped yet)
//   ParrotOnShoulderLayer.submitOnShoulder -> (not mapped yet)
//   RenderLayer.<init> -> (not mapped yet)
//   EntitySpawnReason.COMMAND -> (not mapped yet)
//   EntityType.create -> horizon_client::mobs::finalize::create (suggested, high)
//   Player.level -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/feature/MixinShoulderParrotFeatureRenderer.java:73:
// | @Inject(method = "submitOnShoulder", at = @At("RETURN"))
// | private void etf$resetEntity(CallbackInfo ci) {
// |    ETFState.unMount();
// | }
