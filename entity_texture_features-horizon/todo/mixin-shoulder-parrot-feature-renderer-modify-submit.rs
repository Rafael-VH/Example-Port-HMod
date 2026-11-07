// Not ported yet: the Java @Inject(INVOKE) on ParrotOnShoulderLayer.submitOnShoulder (MixinShoulderParrotFeatureRenderer.etf$modifySubmit).
// Why it's off: No Horizon function mapped for ParrotOnShoulderLayer.submitOnShoulder (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for ParrotOnShoulderLayer.submitOnShoulder: run `match`.
// Note: The call it hooks (SubmitNodeCollector.submitModel) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SubmitNodeCollector.submitModel -> (not mapped yet)
//   ParrotOnShoulderLayer.submitOnShoulder -> (not mapped yet)
//   RenderLayer.<init> -> (not mapped yet)
//   EntitySpawnReason.COMMAND -> (not mapped yet)
//   EntityType.create -> horizon_client::mobs::finalize::create (suggested, high)
//   Player.level -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/feature/MixinShoulderParrotFeatureRenderer.java:32:
// | @Inject(
// |    method = "submitOnShoulder",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/SubmitNodeCollector;submitModel(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/resources/Identifier;III)V"
// |    )
// | )
// | private void etf$modifySubmit(
// |    PoseStack poseStack,
// |    SubmitNodeCollector submitNodeCollector,
// |    int i,
// |    AvatarRenderState avatarRenderState,
// |    Variant variant,
// |    float f,
// |    float g,
// |    boolean bl,
// |    CallbackInfo ci,
// |    @Local ParrotRenderState parrotRenderState
// | ) {
// |    ETFEntityRenderState state = ((HoldsETFRenderState)avatarRenderState).etf$getState();
// |    if (state != null && state.entity() instanceof Player playerEntity) {
// |       this.etf$setParrotAsCurrentEntity(playerEntity, parrotRenderState);
// |    }
// | }
