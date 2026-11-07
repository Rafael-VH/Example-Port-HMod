// Not ported yet: the Java @ModifyReturnValue(RETURN) on SoundEngine.calculateVolume (MSoundSystem.adjustSoundVolume).
// Why it's off: No Horizon function mapped for SoundEngine.calculateVolume (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: No Horizon function known for SoundEngine.calculateVolume: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Options.getFinalSoundSourceVolume -> (not mapped yet)
//   SoundInstance.getSource -> (not mapped yet)
//   SoundInstance.getVolume -> (not mapped yet)
//   SoundEngine.calculateVolume -> (not mapped yet)
//   Mth.clamp -> horizon_client_render::chat::clamp (suggested, high)
//
// Decompiled Java, eu/ha3/presencefootsteps/mixins/MSoundSystem.java:20:
// | @ModifyReturnValue(method = "calculateVolume(Lnet/minecraft/client/resources/sounds/SoundInstance;)F", at = @At("RETURN"))
// | private float adjustSoundVolume(float volume, SoundInstance sound) {
// |    return sound instanceof ImmediateSoundPlayer.UncappedSoundInstance t
// |       ? Mth.clamp(sound.getVolume() * this.options.getFinalSoundSourceVolume(sound.getSource()), 0.0F, t.getMaxVolume())
// |       : volume;
// | }
