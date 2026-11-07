// Not ported yet: the Java @Inject(RETURN) on WeighedSoundEvents.getSound (MixinWeighedSoundEvents.esf$soundModify).
// Why it's off: `horizon_client_render::sound_events::SoundRegistry::sound` has a `?` in its body: horizon-mixins doesn't rewrite such bodies with RETURN. A HEAD hook on it, or a RETURN on a function it calls, may work instead.
// horizon-mixins kind: RETURN on `horizon_client_render::sound_events::SoundRegistry::sound`
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Sound.getLocation -> (not mapped yet)
//   WeighedSoundEvents.<init> -> (not mapped yet)
//   WeighedSoundEvents.getSound -> horizon_client_render::sound_events::SoundRegistry::sound (suggested, high)
//
// Decompiled Java, traben/entity_sound_features/mixin/mixins/MixinWeighedSoundEvents.java:31:
// | @Inject(method = "getSound(Lnet/minecraft/util/RandomSource;)Lnet/minecraft/client/resources/sounds/Sound;", at = @At("RETURN"), cancellable = true)
// | private void esf$soundModify(RandomSource randomSource, CallbackInfoReturnable<Sound> cir) {
// |    if (this.esf$variator != null) {
// |       boolean announce = ((ESFConfig)ESF.config().getConfig()).announceCompatibleSounds != ESFConfig.AnnounceMode.NONE;
// |       if (announce) {
// |          ESFSoundContext.announceSound(this.esf$resourceLocation, true);
// |       }
// |
// |       Sound sound = this.esf$variator.getSoundVariantOrNull();
// |       if (sound != null) {
// |          if (announce) {
// |             ESF.log("Sound modified to: " + sound.getLocation());
// |          }
// |
// |          cir.setReturnValue(sound);
// |       } else if (announce) {
// |          ESF.log("Sound not modified: " + ((Sound)cir.getReturnValue()).getLocation());
// |       }
// |    } else if (((ESFConfig)ESF.config().getConfig()).announceCompatibleSounds != ESFConfig.AnnounceMode.NONE) {
// |       ESFSoundContext.announceSound(this.esf$resourceLocation, false);
// |    }
// | }
