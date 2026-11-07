// Not ported yet: the Java @Inject(HEAD) on CapeLayer.submit (CapeLayerMixin.be_checkCustomElytra).
// Why it's off: No Horizon function mapped for CapeLayer.submit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for CapeLayer.submit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   CapeLayer.submit -> (not mapped yet)
//   AvatarRenderState.chestEquipment -> (not mapped yet)
//   ItemStack.getItem -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/client/CapeLayerMixin.java:16:
// | @Inject(
// |    method = "submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/AvatarRenderState;FF)V",
// |    at = @At("HEAD"),
// |    cancellable = true
// | )
// | public void be_checkCustomElytra(
// |    PoseStack poseStack, SubmitNodeCollector submitNodeCollector, int i, AvatarRenderState avatarRenderState, float f, float g, CallbackInfo info
// | ) {
// |    ItemStack itemStack = avatarRenderState.chestEquipment;
// |    if (itemStack.getItem() instanceof ArmoredElytra) {
// |       info.cancel();
// |    }
// | }
