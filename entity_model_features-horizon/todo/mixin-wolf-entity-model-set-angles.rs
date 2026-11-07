// Not ported yet: the Java @Inject(HEAD) on WolfModel.setupAnim (MixinWolfEntityModel.smf$setAngles).
// Why it's off: No Horizon function mapped for WolfModel.setupAnim (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for WolfModel.setupAnim: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   WolfModel.setupAnim -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/model/MixinWolfEntityModel.java:18:
// | @Inject(method = "setupAnim(Lnet/minecraft/client/renderer/entity/state/WolfRenderState;)V", at = @At("HEAD"))
// | private void smf$setAngles(WolfRenderState wolfRenderState, CallbackInfo ci) {
// |    if (this.emf$hasCollarModel()) {
// |       this.emf$collarModel.setupAnim(wolfRenderState);
// |    }
// | }
