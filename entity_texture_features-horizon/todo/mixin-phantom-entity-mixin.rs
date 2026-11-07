// Not ported yet: the Java @ModifyArg(INVOKE) on Phantom.tick (MixinPhantomEntity.mixin).
// Why it's off: No Horizon function mapped for Phantom.tick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for Phantom.tick: run `match`.
// Note: The call it hooks (Level.addParticle) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Entity.<init> -> (not mapped yet)
//   Phantom.tick -> (not mapped yet)
//   Level.addParticle -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/misc/MixinPhantomEntity.java:17:
// | @ModifyArg(
// |    method = "tick",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/level/Level;addParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)V"),
// |    index = 2
// | )
// | private double mixin(double x) {
// |    return ETF.config().getConfig().canDoCustomTextures() && ETFManager.getInstance().ENTITY_TYPE_IGNORE_PARTICLES.contains(this.getType()) ? -500.0 : x;
// | }
