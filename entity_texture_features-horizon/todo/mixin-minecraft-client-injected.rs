// Not ported yet: the Java @Inject(TAIL) on Minecraft.disconnect* (MixinMinecraftClient.etf$injected).
// Why it's off: No Horizon function mapped for Minecraft.disconnect* (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: Wildcard or unknown selector `disconnect*`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.runTick -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/reloading/MixinMinecraftClient.java:13:
// | @Inject(method = "disconnect*", at = @At("TAIL"))
// | private void etf$injected(CallbackInfo ci) {
// |    ETFManager.resetInstance();
// | }
