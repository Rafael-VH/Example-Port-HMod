// Not ported yet: the Java @Inject(HEAD) on Minecraft.setLevel (MinecraftMixin.setLevel).
// Why it's off: No Horizon function mapped for Minecraft.setLevel (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Minecraft.setLevel: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.disconnect -> (not mapped yet)
//   Minecraft.setLevel -> (not mapped yet)
//
// Decompiled Java, com/sonicether/soundphysics/mixin/MinecraftMixin.java:24:
// | @Inject(method = "setLevel", at = @At("HEAD"))
// | private void setLevel(ClientLevel clientLevel, CallbackInfo ci) {
// |    if (this.level != null) {
// |       LevelAccessUtils.onUnloadLevel(this.level);
// |    }
// |
// |    if (this.player != null) {
// |       LevelAccessUtils.onLoadLevel(clientLevel);
// |    }
// | }
