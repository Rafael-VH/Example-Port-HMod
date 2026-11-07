// Not ported yet: the Java @Inject(RETURN) on AvatarRenderer.getTextureLocation (MixinPlayerEntityRenderer.etf$getTexture).
// Why it's off: No Horizon function mapped for AvatarRenderer.getTextureLocation (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for AvatarRenderer.getTextureLocation: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ModelPart.render -> (not mapped yet)
//   LivingEntityRenderer.<init> -> (not mapped yet)
//   AvatarRenderer.<init> -> (not mapped yet)
//   AvatarRenderer.getTextureLocation -> (not mapped yet)
//   AvatarRenderer.renderHand -> (not mapped yet)
//   OverlayTexture.NO_OVERLAY -> (not mapped yet)
//   Identifier.equals -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/MixinPlayerEntityRenderer.java:75:
// | @Inject(
// |    method = "getTextureLocation(Lnet/minecraft/client/renderer/entity/state/AvatarRenderState;)Lnet/minecraft/resources/Identifier;",
// |    at = @At("RETURN"),
// |    cancellable = true
// | )
// | private void etf$getTexture(AvatarRenderState playerRenderState, CallbackInfoReturnable<Identifier> cir) {
// |    ETFEntityRenderState state = ((HoldsETFRenderState)playerRenderState).etf$getState();
// |    if (ETF.config().getConfig().skinFeaturesEnabled) {
// |       if (state == null || !state.isPlayer()) {
// |          return;
// |       }
// |
// |       ETFPlayerEntity player = (ETFPlayerEntity)state.entity();
// |       this.etf$ETFPlayerTexture = ETFManager.getInstance().getPlayerTexture(player, (Identifier)cir.getReturnValue());
// |       if (this.etf$ETFPlayerTexture != null && this.etf$ETFPlayerTexture.hasFeatures) {
// |          Identifier texture = this.etf$ETFPlayerTexture.getBaseTextureIdentifierOrNullForVanilla(state);
// |          if (texture != null) {
// |             cir.setReturnValue(texture);
// |          }
// |       }
// |    } else {
// |       this.etf$ETFPlayerTexture = null;
// |    }
// | }
