// Not ported yet: the Java @Inject(FIELD) on HumanoidModel.setupAnim (HumanoidModelMixin.farmersdelightrefabricated$setupSkilletThirdPersonAnim).
// Why it's off: No horizon-mixins counterpart: FIELD, NEW, JUMP and CONSTANT points have no horizon-mixins counterpart yet.
// Note: FIELD, NEW, JUMP and CONSTANT points have no horizon-mixins counterpart yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   Minecraft.level -> (not mapped yet)
//   HumanoidModel.setupAnim -> (not mapped yet)
//   HumanoidRenderState.mainArm -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/HumanoidModelMixin.java:15:
// | @Inject(
// |    method = "setupAnim(Lnet/minecraft/client/renderer/entity/state/HumanoidRenderState;)V",
// |    at = @At(
// |       value = "FIELD",
// |       target = "Lnet/minecraft/client/renderer/entity/state/HumanoidRenderState;useItemHand:Lnet/minecraft/world/InteractionHand;",
// |       opcode = 180
// |    )
// | )
// | private <T extends HumanoidRenderState> void farmersdelightrefabricated$setupSkilletThirdPersonAnim(T humanoidRenderState, CallbackInfo ci) {
// |    SkilletFlipItemRenderer.applyTransform(
// |       Minecraft.getInstance().level, (HumanoidModel<?>)this, (SkilletEntityRenderState)humanoidRenderState, humanoidRenderState.mainArm
// |    );
// | }
