// Not ported yet: the Java @ModifyArg(INVOKE) on SoundEngine.play (SoundSystemMixin.play).
// Why it's off: No Horizon function mapped for SoundEngine.play (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for SoundEngine.play: run `match`.
// Note: The call it hooks (Multimap.put) isn't mapped to a Rust function yet.
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
// Decompiled Java, com/sonicether/soundphysics/mixin/SoundSystemMixin.java:27:
// | @ModifyArg(
// |    method = "play",
// |    at = @At(value = "INVOKE", target = "Lcom/google/common/collect/Multimap;put(Ljava/lang/Object;Ljava/lang/Object;)Z"),
// |    index = 1
// | )
// | private Object play(Object instance) {
// |    SoundInstance soundInstance = (SoundInstance)instance;
// |    SoundPhysics.setLastSoundCategoryAndName(soundInstance.getSource(), soundInstance.getIdentifier());
// |    return instance;
// | }
