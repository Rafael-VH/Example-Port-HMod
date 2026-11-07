// Not ported yet: the Java @Inject(HEAD) on Minecraft.runTick (MixinMinecraftClient.etf$recycleSubmitData).
// Why it's off: No Horizon function mapped for Minecraft.runTick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Minecraft.runTick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.runTick -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/reloading/MixinMinecraftClient.java:18:
// | @Inject(method = "runTick", at = @At("HEAD"))
// | private void etf$recycleSubmitData(CallbackInfo ci) {
// |    ETFSubmitData.recycle();
// | }
