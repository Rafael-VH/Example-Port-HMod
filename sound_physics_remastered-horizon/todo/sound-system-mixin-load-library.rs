// Not ported yet: the Java @Inject(INVOKE) on SoundEngine.loadLibrary (SoundSystemMixin.loadLibrary).
// Why it's off: No Horizon function mapped for SoundEngine.loadLibrary (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for SoundEngine.loadLibrary: run `match`.
// Note: The call it hooks (Listener.reset) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Multimap.put -> (not mapped yet)
//   Listener.reset -> (not mapped yet)
//   Minecraft.getInstance -> `self`. Hooks are methods on Game: self is what Minecraft.getInstance() gave.
//   Minecraft.level -> (not mapped yet)
//   ClientLevel.getGameTime -> (not mapped yet)
//   SoundInstance.getIdentifier -> (not mapped yet)
//   SoundInstance.getSource -> (not mapped yet)
//   SoundInstance.getX -> (not mapped yet)
//   SoundInstance.getY -> (not mapped yet)
//   SoundInstance.getZ -> (not mapped yet)
//   SoundInstance.hashCode -> (not mapped yet)
//   ChannelHandle.execute -> (not mapped yet)
//   ChannelHandle.isStopped -> (not mapped yet)
//   SoundEngine.loadLibrary -> (not mapped yet)
//   SoundEngine.play -> (not mapped yet)
//   SoundEngine.tickInGameSound -> (not mapped yet)
//
// Decompiled Java, com/sonicether/soundphysics/mixin/SoundSystemMixin.java:22:
// | @Inject(method = "loadLibrary", at = @At(value = "INVOKE", target = "Lcom/mojang/blaze3d/audio/Listener;reset()V"))
// | private void loadLibrary(CallbackInfo ci) {
// |    SoundPhysics.init();
// | }
