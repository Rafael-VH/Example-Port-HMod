// Not ported yet: the Java @Inject(HEAD) on Entity.walkingStepSound (IEntity.onPlayStepSounds).
// Why it's off: No Horizon function mapped for Entity.walkingStepSound (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Entity.walkingStepSound: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Entity.walkingStepSound -> (not mapped yet)
//
// Decompiled Java, eu/ha3/presencefootsteps/mixins/IEntity.java:14:
// | @Inject(method = "walkingStepSound", at = @At("HEAD"), cancellable = true)
// | private void onPlayStepSounds(BlockPos pos, BlockState state, CallbackInfo info) {
// |    if (this instanceof StepSoundSource s && s.isStepBlocked()) {
// |       info.cancel();
// |    }
// | }
