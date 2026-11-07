// Not ported yet: the Java @Inject(RETURN) on StuckInBodyLayer.submit (MixinStuckArrowsFeatureRenderer.emf$end).
// Why it's off: No Horizon function mapped for StuckInBodyLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for StuckInBodyLayer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PoseStack.popPose -> (not mapped yet)
//   PoseStack.pushPose -> (not mapped yet)
//   PoseStack.translate -> (not mapped yet)
//   ModelPart.children -> (not mapped yet)
//   ModelPart.cubes -> (not mapped yet)
//   ModelPart.getRandomCube -> (not mapped yet)
//   ModelPart.skipDraw -> (not mapped yet)
//   ModelPart.translateAndRotate -> (not mapped yet)
//   ModelPart.visible -> (not mapped yet)
//   Cube.maxX -> (not mapped yet)
//   Cube.maxY -> (not mapped yet)
//   Cube.maxZ -> (not mapped yet)
//   Cube.minX -> (not mapped yet)
//   Cube.minY -> (not mapped yet)
//   Cube.minZ -> (not mapped yet)
//   RenderLayer.<init> -> (not mapped yet)
//   StuckInBodyLayer.submit -> (not mapped yet)
//   PlacementStyle.ON_SURFACE -> (not mapped yet)
//   AvatarRenderState.id -> (not mapped yet)
//   AvatarRenderState.outlineColor -> (not mapped yet)
//   Mth.lerp -> horizon_world::stepped_interpolation::lerp (suggested, high)
//   RandomSource.create -> (not mapped yet)
//   RandomSource.nextFloat -> (not mapped yet)
//   RandomSource.nextInt -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/arrows/MixinStuckArrowsFeatureRenderer.java:114:
// | @Inject(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/AvatarRenderState;FF)V",
// |    at = @At("RETURN")
// | )
// | private void emf$end(CallbackInfo ci) {
// |    EMFState.isInGroundOverride = false;
// | }
