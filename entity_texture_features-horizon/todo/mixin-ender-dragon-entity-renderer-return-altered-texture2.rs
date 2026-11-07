// Not ported yet: the Java @ModifyArg(INVOKE) on EnderDragonRenderer.submit (MixinEnderDragonEntityRenderer.etf$returnAlteredTexture2).
// Why it's off: No Horizon function mapped for EnderDragonRenderer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for EnderDragonRenderer.submit: run `match`.
// Note: The call it hooks (SubmitNodeCollector.submitModel) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SubmitNodeCollector.submitModel -> (not mapped yet)
//   EnderDragonRenderer.submit -> (not mapped yet)
//   EntityRenderer.<init> -> (not mapped yet)
//   RenderTypes.eyes -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/MixinEnderDragonEntityRenderer.java:34:
// | @ModifyArg(
// |    method = "submit(Lnet/minecraft/client/renderer/entity/state/EnderDragonRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/SubmitNodeCollector;submitModel(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;III)V"
// |    )
// | )
// | private RenderType etf$returnAlteredTexture2(RenderType texturedRenderLayer) {
// |    return getType(texturedRenderLayer);
// | }
