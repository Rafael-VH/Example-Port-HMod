// Not ported yet: the Java @WrapOperation(INVOKE) on ServerExplosion.hurtEntities (ExplosionMixin.onDetonate).
// Why it's off: No Horizon function mapped for ServerExplosion.hurtEntities (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for ServerExplosion.hurtEntities: run `match`.
// Note: The call it hooks (ServerLevel.getEntities) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ServerLevel.getEntities -> (not mapped yet)
//   ServerExplosion.hurtEntities -> (not mapped yet)
//
// Decompiled Java, team/creative/enhancedvisuals/mixin/ExplosionMixin.java:17:
// | @WrapOperation(
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/server/level/ServerLevel;getEntities(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;"
// |    ),
// |    method = "hurtEntities(Ljava/util/List;)V"
// | )
// | private List<Entity> onDetonate(ServerLevel level, Entity causer, AABB box, Operation<List<Entity>> entity) {
// |    List<Entity> list = (List<Entity>)entity.call(new Object[]{level, causer, box});
// |    EnhancedVisuals.EVENTS.explosion((Explosion)this, list);
// |    return list;
// | }
