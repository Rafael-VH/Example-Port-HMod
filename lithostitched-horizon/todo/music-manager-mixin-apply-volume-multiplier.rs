// Not ported yet: the Java @WrapOperation(INVOKE) on MusicManager.tick (MusicManagerMixin.applyVolumeMultiplier).
// Why it's off: `horizon_client::music::MusicManager::tick` has a closure in its body: horizon-mixins doesn't rewrite such bodies with WRAP_OPERATION. A HEAD hook on it, or a WRAP_OPERATION on a function it calls, may work instead.
// horizon-mixins kind: WRAP_OPERATION on `horizon_client::music::MusicManager::tick`
// Note: The call it hooks (Minecraft.getMusicVolume) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.getMusicVolume -> (not mapped yet)
//   Minecraft.level -> (not mapped yet)
//   MusicManager.tick -> horizon_client::music::MusicManager::tick (suggested, high)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/client/MusicManagerMixin.java:17:
// | @WrapOperation(method = "tick", at = @At(value = "INVOKE", target = "Lnet/minecraft/client/Minecraft;getMusicVolume()F"))
// | private float applyVolumeMultiplier(Minecraft minecraft, Operation<Float> operation) {
// |    float baseVolume = (Float)operation.call(new Object[]{minecraft});
// |    if (minecraft.level == null) {
// |       return baseVolume;
// |    } else {
// |       return StructureAttributesHolder.from(minecraft.level).shouldFadeMusic() ? -1.0F : baseVolume;
// |    }
// | }
