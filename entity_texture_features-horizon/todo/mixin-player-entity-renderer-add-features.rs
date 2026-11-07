// Not ported yet: the Java @Inject(TAIL) on AvatarRenderer.<init> (MixinPlayerEntityRenderer.etf$addFeatures).
// Why it's off: No Horizon function mapped for AvatarRenderer.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for AvatarRenderer.<init>: run `match`.
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
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/renderer/MixinPlayerEntityRenderer.java:45:
// | @Inject(method = "<init>", at = @At("TAIL"))
// | private void etf$addFeatures(Context ctx, boolean slim, CallbackInfo ci) {
// |    this.addLayer(new ETFPlayerFeatureRenderer(this));
// | }
