// Not ported yet: the Java @Inject(TAIL) on AvatarRenderer.renderHand (MixinPlayerEntityRenderer.etf$renderHandFeatures).
// Why it's off: No Horizon function mapped for AvatarRenderer.renderHand (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for AvatarRenderer.renderHand: run `match`.
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
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/MixinPlayerEntityRenderer.java:50:
// | @Inject(method = "renderHand", at = @At("TAIL"))
// | private void etf$renderHandFeatures(
// |    PoseStack poseStack, SubmitNodeCollector submitNodeCollector, int i, Identifier resourceLocation, ModelPart modelPart, boolean bl, CallbackInfo ci
// | ) {
// |    for (ETFPlayerTexture skin : ETFManager.getInstance().PLAYER_TEXTURE_MAP.values()) {
// |       if (skin.getOriginal().equals(resourceLocation)) {
// |          Identifier emissive = skin.getBaseTextureEmissiveIdentifierOrNullForNone();
// |          if (emissive != null) {
// |             ETFUtils2.submitEmissiveModelPart(poseStack, submitNodeCollector, modelPart, emissive);
// |          }
// |
// |          Identifier enchant = skin.baseEnchantIdentifier;
// |          if (enchant != null) {
// |             ETFUtils2.submitEnchantedModelPart(poseStack, submitNodeCollector, i, modelPart, enchant);
// |          }
// |          break;
// |       }
// |    }
// | }
