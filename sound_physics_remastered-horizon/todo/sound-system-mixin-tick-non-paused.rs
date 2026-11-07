// Not ported yet: the Java @Inject(INVOKE) on SoundEngine.tickInGameSound (SoundSystemMixin.tickNonPaused).
// Why it's off: No Horizon function mapped for SoundEngine.tickInGameSound (run `match`, or map it in a --mappings file).
// horizon-mixins kind: BEFORE_CALL
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: No Horizon function known for SoundEngine.tickInGameSound: run `match`.
// Note: The call it hooks (ChannelHandle.isStopped) isn't mapped to a Rust function yet.
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
// Decompiled Java, com/sonicether/soundphysics/mixin/SoundSystemMixin.java:38:
// | @Inject(
// |    method = "tickInGameSound",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/client/sounds/ChannelAccess$ChannelHandle;isStopped()Z"),
// |    locals = LocalCapture.CAPTURE_FAILHARD
// | )
// | private void tickNonPaused(CallbackInfo ci, Iterator<?> iterator, Entry<SoundInstance, ChannelHandle> map, ChannelHandle channelHandle, SoundInstance sound) {
// |    if (SoundPhysicsMod.CONFIG.updateMovingSounds.get()) {
// |       if (this.minecraft.level != null
// |          && (this.minecraft.level.getGameTime() + sound.hashCode()) % SoundPhysicsMod.CONFIG.soundUpdateInterval.get().intValue() == 0L) {
// |          channelHandle.execute(
// |             channel -> SoundPhysics.processSound(
// |                ((ChannelAccessor)channel).getSource(), sound.getX(), sound.getY(), sound.getZ(), sound.getSource(), sound.getIdentifier()
// |             )
// |          );
// |       }
// |    }
// | }
