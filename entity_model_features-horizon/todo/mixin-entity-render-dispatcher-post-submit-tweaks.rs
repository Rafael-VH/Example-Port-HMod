// Not ported yet: the Java @Inject(INVOKE) on EntityRenderDispatcher.submit (MixinEntityRenderDispatcher.postSubmitTweaks).
// Why it's off: No Horizon function mapped for EntityRenderDispatcher.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: AFTER_CALL
// Note: No Horizon function known for EntityRenderDispatcher.submit: run `match`.
// Note: The call it hooks (EntityRenderer.submit) isn't mapped to a Rust function yet.
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
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/MixinEntityRenderDispatcher.java:31:
// | @Inject(
// |    method = "submit",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/entity/EntityRenderer;submit(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V",
// |       shift = Shift.AFTER
// |    )
// | )
// | private <S extends EntityRenderState> void postSubmitTweaks(CallbackInfo ci, @Local EntityRenderer entityRenderer, @Local(argsOnly = true) S ogState) {
// |    EMFEntityRenderState state = EMFEntityRenderState.from(ogState);
// |    if (state != null) {
// |       ETFState.stackVerify(state);
// |       if (state.needsToModifyShadow()) {
// |          double x = ogState.x;
// |          double z = ogState.z;
// |          boolean useShadowX = !Float.isNaN(state.shadowX());
// |          boolean useShadowZ = !Float.isNaN(state.shadowZ());
// |          if (useShadowX) {
// |             ogState.x = ogState.x + state.shadowX();
// |          }
// |
// |          if (useShadowZ) {
// |             ogState.z = ogState.z + state.shadowZ();
// |          }
// |
// |          entityRenderer.extractShadow(ogState, Minecraft.getInstance(), EMFState.state().world());
// |          if (useShadowX) {
// |             ogState.x = x;
// |          }
// |
// |          if (useShadowZ) {
// |             ogState.z = z;
// |          }
// |       }
// |    }
// | }
