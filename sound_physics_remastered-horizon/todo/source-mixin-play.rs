// Not ported yet: the Java @Inject(HEAD) on Channel.play (SourceMixin.play).
// Why it's off: No Horizon function mapped for Channel.play (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Channel.play: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Channel.linearAttenuation -> (not mapped yet)
//   Channel.play -> (not mapped yet)
//   Channel.setSelfPosition -> (not mapped yet)
//   Vec3.x -> (not mapped yet)
//   Vec3.y -> (not mapped yet)
//   Vec3.z -> (not mapped yet)
//
// Decompiled Java, com/sonicether/soundphysics/mixin/SourceMixin.java:30:
// | @Inject(method = "play", at = @At("HEAD"))
// | private void play(CallbackInfo ci) {
// |    if (this.pos != null) {
// |       SoundPhysics.onPlaySound(this.pos.x, this.pos.y, this.pos.z, this.source);
// |       Loggers.logALError("Sound play injector");
// |    }
// | }
