// Not ported yet: the Java @ModifyVariable(HEAD) on Channel.linearAttenuation (SourceMixin.injected).
// Why it's off: No Horizon function mapped for Channel.linearAttenuation (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for Channel.linearAttenuation: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Channel.linearAttenuation -> (not mapped yet)
//   Channel.play -> (not mapped yet)
//   Channel.setSelfPosition -> (not mapped yet)
//   Vec3.x -> (not mapped yet)
//   Vec3.y -> (not mapped yet)
//   Vec3.z -> (not mapped yet)
//
// Decompiled Java, com/sonicether/soundphysics/mixin/SourceMixin.java:38:
// | @ModifyVariable(method = "linearAttenuation", at = @At("HEAD"), ordinal = 0, argsOnly = true)
// | private float injected(float attenuation) {
// |    return !SoundPhysicsMod.CONFIG.enabled.get() ? attenuation : attenuation / SoundPhysicsMod.CONFIG.attenuationFactor.get();
// | }
