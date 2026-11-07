// Not ported yet: the Java @Inject(HEAD) on Minecraft.disconnect (MinecraftMixin.disconnect).
// Why it's off: No Horizon function mapped for Minecraft.disconnect (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Minecraft.disconnect: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.disconnect -> (not mapped yet)
//   Minecraft.setLevel -> (not mapped yet)
//
// Decompiled Java, com/sonicether/soundphysics/mixin/MinecraftMixin.java:35:
// | @Inject(method = "disconnect(Lnet/minecraft/client/gui/screens/Screen;Z)V", at = @At("HEAD"))
// | private void disconnect(Screen screen, boolean bl, CallbackInfo ci) {
// |    if (this.level != null) {
// |       LevelAccessUtils.onUnloadLevel(this.level);
// |    }
// | }
