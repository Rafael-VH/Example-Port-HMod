// Not ported yet: the Java @Inject(HEAD) on Projectile.onHit (ProjectileMixin.onHit).
// Why it's off: No Horizon function mapped for Projectile.onHit (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Projectile.onHit: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Projectile.onHit -> (not mapped yet)
//
// Decompiled Java, team/creative/enhancedvisuals/mixin/ProjectileMixin.java:13:
// | @Inject(at = @At("HEAD"), method = "Lnet/minecraft/world/entity/projectile/Projectile;onHit(Lnet/minecraft/world/phys/HitResult;)V")
// | private void onHit(HitResult hitResult, CallbackInfo ci) {
// |    EnhancedVisuals.EVENTS.impact((Projectile)this);
// | }
