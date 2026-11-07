// Not ported yet: the Java @Inject(HEAD) on SmallFireball.onHitEntity (SmallFireballEntityMixin.onEntityHit).
// Why it's off: No Horizon function mapped for SmallFireball.onHitEntity (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for SmallFireball.onHitEntity: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Fireball.<init> -> (not mapped yet)
//   SmallFireball.onHitEntity -> (not mapped yet)
//   Level.isClientSide -> horizon_world::level::is_client_side (suggested, high)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/abilities/SmallFireballEntityMixin.java:21:
// | @Inject(at = @At("HEAD"), method = "onHitEntity", cancellable = true)
// | public void onEntityHit(EntityHitResult entityHitResult, CallbackInfo ci) {
// |    if (TravelersBackpackConfig.serverSpec.isLoaded()
// |       && (Boolean)TravelersBackpackConfig.SERVER.backpackAbilities.enableBackpackAbilities.get()
// |       && !this.level().isClientSide()
// |       && this instanceof SmallFireball smallFireball) {
// |       BackpackAbilities.blazeAbility(entityHitResult, smallFireball, ci);
// |    }
// | }
