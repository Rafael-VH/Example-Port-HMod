// Not ported yet: the Java @ModifyConstant on SoundEvent.getRange (SoundEventMixin.allowance1).
// Why it's off: No Horizon function mapped for SoundEvent.getRange (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_CONSTANT
// Note: Selects an exact Rust literal: pick it in the target function.
// Note: No Horizon function known for SoundEvent.getRange: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SoundEvent.getRange -> (not mapped yet)
//
// Decompiled Java, com/sonicether/soundphysics/mixin/SoundEventMixin.java:11:
// | @ModifyConstant(method = "getRange", constant = @Constant(floatValue = 16.0F), expect = 2)
// | private float allowance1(float value) {
// |    return !SoundPhysicsMod.CONFIG.enabled.get() ? value : value * SoundPhysicsMod.CONFIG.soundDistanceAllowance.get();
// | }
