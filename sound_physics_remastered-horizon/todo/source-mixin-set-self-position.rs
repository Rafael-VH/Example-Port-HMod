// Not ported yet: the Java @Inject(HEAD) on Channel.setSelfPosition (SourceMixin.setSelfPosition).
// Why it's off: No Horizon function mapped for Channel.setSelfPosition (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Channel.setSelfPosition: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Channel.linearAttenuation -> (not mapped yet)
//   Channel.play -> (not mapped yet)
//   Channel.setSelfPosition -> (not mapped yet)
//   Vec3.x -> (not mapped yet)
//   Vec3.y -> (not mapped yet)
//   Vec3.z -> (not mapped yet)
//
// Decompiled Java, com/sonicether/soundphysics/mixin/SourceMixin.java:25:
// | @Inject(method = "setSelfPosition", at = @At("HEAD"))
// | private void setSelfPosition(Vec3 poss, CallbackInfo ci) {
// |    this.pos = poss;
// | }
