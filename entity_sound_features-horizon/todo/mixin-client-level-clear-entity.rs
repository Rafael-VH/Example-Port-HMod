// Not ported yet: the Java @Inject(TAIL) on ClientLevel.playSound (MixinClientLevel.esf$clearEntity).
// Why it's off: No Horizon function mapped for ClientLevel.playSound (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for ClientLevel.playSound: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientLevel.playSound -> (not mapped yet)
//   BlockPos.toShortString -> (not mapped yet)
//   SoundEvent.location -> (not mapped yet)
//   Level.<init> -> (not mapped yet)
//   LevelEntityGetter.getAll -> (not mapped yet)
//
// Decompiled Java, traben/entity_sound_features/mixin/mixins/MixinClientLevel.java:60:
// | @Inject(method = "playSound", at = @At("TAIL"))
// | private void esf$clearEntity(CallbackInfo ci) {
// |    ESFSoundContext.entitySource = null;
// | }
