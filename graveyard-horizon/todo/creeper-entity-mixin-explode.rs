// Not ported yet: the Java @Inject(HEAD) on Creeper.explodeCreeper (CreeperEntityMixin.explode).
// Why it's off: No Horizon function mapped for Creeper.explodeCreeper (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Creeper.explodeCreeper: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Creeper.explodeCreeper -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/CreeperEntityMixin.java:12:
// | @Inject(method = "explodeCreeper", at = @At("HEAD"), cancellable = true)
// | private void explode(CallbackInfo info) {
// |    Creeper creeperEntity = (Creeper)this;
// |    CreeperExplosionAccessor.explode(creeperEntity, info);
// | }
