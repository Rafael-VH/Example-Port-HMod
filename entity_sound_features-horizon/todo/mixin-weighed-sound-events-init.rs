// Not ported yet: the Java @Inject(TAIL) on WeighedSoundEvents.<init> (MixinWeighedSoundEvents.esf$init).
// Why it's off: No Horizon function mapped for WeighedSoundEvents.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for WeighedSoundEvents.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Sound.getLocation -> (not mapped yet)
//   WeighedSoundEvents.<init> -> (not mapped yet)
//   WeighedSoundEvents.getSound -> horizon_client_render::sound_events::SoundRegistry::sound (suggested, high)
//
// Decompiled Java, traben/entity_sound_features/mixin/mixins/MixinWeighedSoundEvents.java:25:
// | @Inject(method = "<init>", at = @At("TAIL"))
// | private void esf$init(Identifier resourceLocation, String string, CallbackInfo ci) {
// |    this.esf$resourceLocation = resourceLocation;
// |    this.esf$variator = ESFVariantSupplier.getOrNull(resourceLocation);
// | }
