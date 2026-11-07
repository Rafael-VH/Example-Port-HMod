// Not ported yet: the Java @Inject(HEAD) on SoundEngine.play (SoundEngineMixin.graveyard$readPlayedSound).
// Why it's off: No Horizon function mapped for SoundEngine.play (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for SoundEngine.play: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SoundInstance.getIdentifier -> (not mapped yet)
//   SoundEngine.calculateVolume -> (not mapped yet)
//   SoundEngine.play -> (not mapped yet)
//   Identifier.getNamespace -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/client/SoundEngineMixin.java:19:
// | @Inject(method = "play", at = @At("HEAD"))
// | private void graveyard$readPlayedSound(SoundInstance sound, CallbackInfoReturnable<PlayResult> cir) {
// |    graveyard$factor.set(graveyard$factorOf(sound));
// | }
