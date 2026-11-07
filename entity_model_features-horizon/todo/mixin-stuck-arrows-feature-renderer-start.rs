// Not ported yet: the Java @Inject(HEAD) on StuckInBodyLayer.submit (MixinStuckArrowsFeatureRenderer.emf$start).
// Why it's off: No Horizon function mapped for StuckInBodyLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
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
// Decompiled Java, traben/entity_model_features/mixin/mixins/rendering/arrows/MixinStuckArrowsFeatureRenderer.java:53:
// | @Inject(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/AvatarRenderState;FF)V",
// |    at = @At("HEAD"),
// |    cancellable = true
// | )
// | private void emf$start(
// |    PoseStack poseStack, SubmitNodeCollector submitNodeCollector, int packedLight, AvatarRenderState playerRenderState, float f2, float g2, CallbackInfo ci
// | ) {
// |    EMFState.isInGroundOverride = true;
// |    if (((IEMFModel)this.getParentModel()).emf$isEMFModel()) {
// |       ci.cancel();
// |       EMFModelPartRoot root = ((IEMFModel)this.getParentModel()).emf$getEMFRootModel();
// |       int i = this.numStuck(playerRenderState);
// |       RandomSource randomSource = RandomSource.create(playerRenderState.id);
// |       if (i > 0) {
// |          for (int j = 0; j < i; j++) {
// |             Random partRand = new Random(j);
// |             poseStack.pushPose();
// |             Pair<ModelPart, Runnable> modelPart = this.emf$bestFromListMutable(new ArrayList<>(root.getAllVanillaPartsEMF()), partRand, poseStack, true);
// |             if (modelPart == null) {
// |                EMFState.isInGroundOverride = false;
// |                poseStack.popPose();
// |                return;
// |             }
// |
// |             ((Runnable)modelPart.getSecond()).run();
// |             float f = randomSource.nextFloat();
// |             float g = randomSource.nextFloat();
// |             float h = randomSource.nextFloat();
// |             if (this.placementStyle == PlacementStyle.ON_SURFACE) {
// |                int n = randomSource.nextInt(3);
// |                switch (n) {
// |                   case 0:
// |                      f = snapToFace(f);
// |                      break;
// |                   case 1:
// |                      g = snapToFace(g);
// |                      break;
// |                   default:
// |                      h = snapToFace(h);
// |                }
// |             }
// |
// |             if (!((ModelPart)modelPart.getFirst()).cubes.isEmpty()) {
// |                Cube cube = ((ModelPart)modelPart.getFirst()).getRandomCube(randomSource);
// |                float k = Mth.lerp(f, cube.minX, cube.maxX) / 16.0F;
// |                float l = Mth.lerp(g, cube.minY, cube.maxY) / 16.0F;
// |                float m = Mth.lerp(h, cube.minZ, cube.maxZ) / 16.0F;
// |                poseStack.translate(k, l, m);
// |             }
// |
// |             f = -1.0F * (f * 2.0F - 1.0F);
// |             g = -1.0F * (g * 2.0F - 1.0F);
// |             h = -1.0F * (h * 2.0F - 1.0F);
// |             this.submitStuckItem(poseStack, submitNodeCollector, packedLight, f, g, h, playerRenderState.outlineColor);
// |             poseStack.popPose();
// |          }
// |       }
// |    }
// | }
