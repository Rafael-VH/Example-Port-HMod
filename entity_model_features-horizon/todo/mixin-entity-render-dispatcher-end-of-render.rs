// Not ported yet: the Java @Inject(RETURN) on EntityRenderDispatcher.submit (MixinEntityRenderDispatcher.emf$endOfRender).
// Why it's off: No Horizon function mapped for EntityRenderDispatcher.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for EntityRenderDispatcher.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PoseStack.translate -> (not mapped yet)
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   SubmitNodeCollector.submitShadow -> (not mapped yet)
//   EntityRenderDispatcher.submit -> (not mapped yet)
//   EntityRenderer.extractShadow -> (not mapped yet)
//   EntityRenderer.submit -> (not mapped yet)
//   EntityRenderState.x -> (not mapped yet)
//   EntityRenderState.z -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/MixinEntityRenderDispatcher.java:23:
// | @Inject(method = "submit", at = @At("RETURN"))
// | private <S extends EntityRenderState> void emf$endOfRender(CallbackInfo ci, @Local(argsOnly = true) S state) {
// |    EMFEntityRenderState emfState = EMFEntityRenderState.from(state);
// |    if (EMFState.announceModels) {
// |       EMFState.anounceModels(emfState);
// |    }
// | }
