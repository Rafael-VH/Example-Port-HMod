// Not ported yet: the Java @Inject(RETURN) on SoundEngine.calculateVolume (SoundEngineMixin.graveyard$scaleVolume).
// Why it's off: No Horizon function mapped for SoundEngine.calculateVolume (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for SoundEngine.calculateVolume: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SoundInstance.getIdentifier -> (not mapped yet)
//   SoundEngine.calculateVolume -> (not mapped yet)
//   SoundEngine.play -> (not mapped yet)
//   Identifier.getNamespace -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/client/SoundEngineMixin.java:29:
// | @Inject(method = "calculateVolume(FLnet/minecraft/sounds/SoundSource;)F", at = @At("RETURN"), cancellable = true)
// | private void graveyard$scaleVolume(float volume, SoundSource source, CallbackInfoReturnable<Float> cir) {
// |    cir.setReturnValue(cir.getReturnValueF() * graveyard$factor.get());
// | }
