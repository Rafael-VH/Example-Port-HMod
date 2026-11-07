// Not ported yet: the Java @Inject(INVOKE) on SkinTextureDownloader.processLegacySkin (MixinPlayerSkinTexture.etf$differentAlpha).
// Why it's off: `horizon_client_render::skins::process_legacy_skin` has a closure in its body: horizon-mixins doesn't rewrite such bodies with BEFORE_CALL. A HEAD hook on it, or a BEFORE_CALL on a function it calls, may work instead.
// horizon-mixins kind: BEFORE_CALL on `horizon_client_render::skins::process_legacy_skin`
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: The call it hooks (SkinTextureDownloader.setNoAlpha) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SkinTextureDownloader.processLegacySkin -> horizon_client_render::skins::process_legacy_skin (suggested, high)
//   SkinTextureDownloader.setNoAlpha -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/MixinPlayerSkinTexture.java:35:
// | @Inject(
// |    method = "processLegacySkin",
// |    cancellable = true,
// |    require = 0,
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/texture/SkinTextureDownloader;setNoAlpha(Lcom/mojang/blaze3d/platform/NativeImage;IIII)V",
// |       shift = Shift.BEFORE,
// |       ordinal = 0
// |    )
// | )
// | private static void etf$differentAlpha(NativeImage nativeImage, String string, CallbackInfoReturnable<NativeImage> cir) {
// |    if (ETF.config().getConfig() != null && ETF.config().getConfig().skinTransparencyInExtraPixels) {
// |       etf$alpha(nativeImage, cir);
// |    }
// | }
