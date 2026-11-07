// Not ported yet: the Java @Inject(TAIL) on ClientLevel.tick (ClientLevelMixin.tick).
// Why it's off: No Horizon function mapped for ClientLevel.tick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for ClientLevel.tick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ClientLevel.tick -> (not mapped yet)
//
// Decompiled Java, com/sonicether/soundphysics/mixin/ClientLevelMixin.java:35:
// | @Inject(method = "tick(Ljava/util/function/BooleanSupplier;)V", at = @At("TAIL"))
// | private void tick(BooleanSupplier booleanSupplier, CallbackInfo ci) {
// |    LevelAccessUtils.tickLevelCache((ClientLevel)this);
// |    SoundRateManager.onClientTick((ClientLevel)this);
// | }
