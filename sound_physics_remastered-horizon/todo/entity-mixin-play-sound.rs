// Not ported yet: the Java @ModifyArg(INVOKE) on Entity.playSound (EntityMixin.playSound).
// Why it's off: The call it hooks (Lnet/minecraft/world/level/Level;playSound(Lnet/minecraft/world/entity/Entity;DDDLnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FF)V) isn't mapped to a Rust function.
// horizon-mixins kind: MODIFY_ARG on `horizon_client::game::consume::<Emitter<'_, '_> as EmissionWorld>::play_sound`
// Note: The call it hooks (Level.playSound) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Entity.playSound -> horizon_client::game::consume::<Emitter<'_, '_> as EmissionWorld>::play_sound (suggested, high)
//   Level.playSound -> (not mapped yet)
//
// Decompiled Java, com/sonicether/soundphysics/mixin/EntityMixin.java:19:
// | @ModifyArg(
// |    method = "playSound(Lnet/minecraft/sounds/SoundEvent;FF)V",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/level/Level;playSound(Lnet/minecraft/world/entity/Entity;DDDLnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FF)V"
// |    ),
// |    index = 2
// | )
// | private double playSound(@Nullable Entity entity, double x, double y, double z, SoundEvent sound, SoundSource category, float volume, float pitch) {
// |    if (sound == null) {
// |       return y;
// |    } else {
// |       return !SoundPhysicsMod.CONFIG.enabled.get() ? y : y + SoundUtils.calculateEntitySoundYOffset(this.getEyeHeight(), sound);
// |    }
// | }
