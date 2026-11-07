// Not ported yet: the Java @ModifyArg(INVOKE) on GlowSquid.aiStep (MixinGlowSquidEntity.mixin).
// Why it's off: No Horizon function mapped for GlowSquid.aiStep (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for GlowSquid.aiStep: run `match`.
// Note: The call it hooks (Level.addParticle) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GlowSquid.aiStep -> (not mapped yet)
//   Squid.<init> -> (not mapped yet)
//   Level.addParticle -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/misc/MixinGlowSquidEntity.java:19:
// | @ModifyArg(
// |    method = "aiStep",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/level/Level;addParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)V"),
// |    index = 2
// | )
// | private double mixin(double x) {
// |    return ETF.config().getConfig().canDoCustomTextures() && ETFManager.getInstance().ENTITY_TYPE_IGNORE_PARTICLES.contains(this.getType()) ? -500.0 : x;
// | }
