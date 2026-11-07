// Not ported yet: the Java @Inject(RETURN) on BaseSpawner.getOrCreateDisplayEntity (MixinMobSpawnerLogic.etf$stabiliseMobSpawnerUUID).
// Why it's off: No Horizon function mapped for BaseSpawner.getOrCreateDisplayEntity (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for BaseSpawner.getOrCreateDisplayEntity: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.asLong -> (not mapped yet)
//   Entity.setUUID -> (not mapped yet)
//   BaseSpawner.getOrCreateDisplayEntity -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/entity/block/MixinMobSpawnerLogic.java:15:
// | @Inject(method = "getOrCreateDisplayEntity", at = @At("RETURN"))
// | private void etf$stabiliseMobSpawnerUUID(Level world, BlockPos pos, CallbackInfoReturnable<Entity> cir) {
// |    Entity entity = (Entity)cir.getReturnValue();
// |    if (entity != null) {
// |       entity.setUUID(new UUID(pos.asLong(), 53021371281465L));
// |    }
// | }
