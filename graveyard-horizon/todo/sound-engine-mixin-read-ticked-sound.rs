// Not ported yet: the Java @Inject(HEAD) on SoundEngine.calculateVolume (SoundEngineMixin.graveyard$readTickedSound).
// Why it's off: No Horizon function mapped for SoundEngine.calculateVolume (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for SoundEngine.calculateVolume: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SoundInstance.getIdentifier -> (not mapped yet)
//   SoundEngine.calculateVolume -> (not mapped yet)
//   SoundEngine.play -> (not mapped yet)
//   Identifier.getNamespace -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/client/SoundEngineMixin.java:24:
// | @Inject(method = "calculateVolume(Lnet/minecraft/client/resources/sounds/SoundInstance;)F", at = @At("HEAD"))
// | private void graveyard$readTickedSound(SoundInstance sound, CallbackInfoReturnable<Float> cir) {
// |    graveyard$factor.set(graveyard$factorOf(sound));
// | }
