// Not ported yet: the Java @Inject(HEAD) on ClientLevel.playSound (MixinClientLevel.esf$discoverEntity).
// Why it's off: No Horizon function mapped for ClientLevel.playSound (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ClientLevel.playSound: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientLevel.playSound -> (not mapped yet)
//   BlockPos.toShortString -> (not mapped yet)
//   SoundEvent.location -> (not mapped yet)
//   Level.<init> -> (not mapped yet)
//   LevelEntityGetter.getAll -> (not mapped yet)
//
// Decompiled Java, traben/entity_sound_features/mixin/mixins/MixinClientLevel.java:43:
// | @Inject(method = "playSound", at = @At("HEAD"))
// | private void esf$discoverEntity(
// |    double x, double y, double z, SoundEvent soundEvent, SoundSource soundSource, float g, float h, boolean bl, long l, CallbackInfo ci
// | ) {
// |    ESFSoundContext.entitySource = null;
// |    if (ESFSoundContext.shouldCaptureEntity(soundEvent.location())) {
// |       ESFSoundContext.searchForEntity(this.getEntities().getAll(), x, y, z);
// |       if (ESFSoundContext.entitySource == null) {
// |          ESFSoundContext.searchForBlockEntity(this, x, y, z);
// |       }
// |    }
// |
// |    if (((ESFConfig)ESF.config().getConfig()).announceCompatibleSounds.all() && ESFSoundContext.entitySource != null) {
// |       ESF.log("found entity at " + ESFSoundContext.entitySource.blockPos().toShortString() + ", for sound at " + x + ", " + y + ", " + z);
// |    }
// | }
