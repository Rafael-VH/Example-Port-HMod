// Not ported yet: the Java @Inject(RETURN) on Channel.linearAttenuation (SourceMixin.linearAttenuation2).
// Why it's off: No Horizon function mapped for Channel.linearAttenuation (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
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
// Decompiled Java, com/sonicether/soundphysics/mixin/SourceMixin.java:43:
// | @Inject(method = "linearAttenuation", at = @At("RETURN"))
// | private void linearAttenuation2(float attenuation, CallbackInfo ci) {
// |    SoundPhysics.applyReferenceDistanceWorkaround(this.source, attenuation);
// | }
