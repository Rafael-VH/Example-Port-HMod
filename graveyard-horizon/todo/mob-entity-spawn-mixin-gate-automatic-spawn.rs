// Not ported yet: the Java @Inject(HEAD) on Mob.checkSpawnRules (MobEntitySpawnMixin.graveyard$gateAutomaticSpawn).
// Why it's off: No Horizon function mapped for Mob.checkSpawnRules (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Mob.checkSpawnRules: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   EntitySpawnReason.SPAWNER -> (not mapped yet)
//   EntitySpawnReason.STRUCTURE -> (not mapped yet)
//   Mob.blockPosition -> (not mapped yet)
//   Mob.checkSpawnRules -> (not mapped yet)
//   Mob.discard -> (not mapped yet)
//   Mob.finalizeSpawn -> horizon_client::mobs::creaking_heart::finalize_spawn (suggested, high)
//   Mob.getType -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/MobEntitySpawnMixin.java:18:
// | @Inject(
// |    method = "checkSpawnRules(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/world/entity/EntitySpawnReason;)Z",
// |    at = @At("HEAD"),
// |    cancellable = true
// | )
// | private void graveyard$gateAutomaticSpawn(LevelAccessor world, EntitySpawnReason spawnReason, CallbackInfoReturnable<Boolean> callback) {
// |    Mob mob = (Mob)this;
// |    if (MobSpawnGate.blocks(mob.getType(), spawnReason) || MobSpawnGate.inForbiddenBiome(world, mob.blockPosition(), mob.getType(), spawnReason)) {
// |       callback.setReturnValue(false);
// |    }
// | }
