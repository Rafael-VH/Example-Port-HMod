// Not ported yet: the Java @WrapWithCondition(INVOKE) on WitchItemLayer.applyTranslation (Mixin_Witch_BlockOffset.cancel2).
// Why it's off: No Horizon function mapped for WitchItemLayer.applyTranslation (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: Call `original` only when the condition holds.
// Note: No Horizon function known for WitchItemLayer.applyTranslation: run `match`.
// Note: The call it hooks (WitchModel.translateToHead) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ModelPart.translateAndRotate -> (not mapped yet)
//   WitchModel.translateToHead -> (not mapped yet)
//   CrossedArmsItemLayer.<init> -> (not mapped yet)
//   WitchItemLayer.applyTranslation -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/attachments/Mixin_Witch_BlockOffset.java:56:
// | @WrapWithCondition(
// |    method = "applyTranslation(Lnet/minecraft/client/renderer/entity/state/WitchRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;)V",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/client/model/monster/witch/WitchModel;translateToHead(Lcom/mojang/blaze3d/vertex/PoseStack;)V")
// | )
// | private boolean cancel2(WitchModel instance, PoseStack poseStack, @Share("needsCancel") LocalBooleanRef needsCancel) {
// |    return !needsCancel.get();
// | }
