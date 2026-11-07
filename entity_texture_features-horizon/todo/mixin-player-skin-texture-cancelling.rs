// Not ported yet: the Java @Inject(HEAD) on SkinTextureDownloader.setNoAlpha (MixinPlayerSkinTexture.etf$cancelling).
// Why it's off: No Horizon function mapped for SkinTextureDownloader.setNoAlpha (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for SkinTextureDownloader.setNoAlpha: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SkinTextureDownloader.processLegacySkin -> horizon_client_render::skins::process_legacy_skin (suggested, high)
//   SkinTextureDownloader.setNoAlpha -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/MixinPlayerSkinTexture.java:23:
// | @Inject(method = "setNoAlpha", cancellable = true, at = @At("HEAD"))
// | private static void etf$cancelling(NativeImage image, int x1, int y1, int x2, int y2, CallbackInfo ci) {
// |    if (ETF.config().getConfig() != null) {
// |       ETFConfig.SkinTransparencyMode mode = ETF.config().getConfig().skinTransparencyMode;
// |       if (mode == ETFConfig.SkinTransparencyMode.ETF_SKINS_ONLY && ETFPlayerTexture.remappingETFSkin) {
// |          ci.cancel();
// |       } else if (mode == ETFConfig.SkinTransparencyMode.ALL) {
// |          ci.cancel();
// |       }
// |    }
// | }
