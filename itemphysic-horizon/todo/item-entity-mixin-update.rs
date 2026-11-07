// Not ported yet: the Java @Inject(INVOKE) on ItemEntity.tick (ItemEntityMixin.update).
// Why it's off: The call it hooks (Lnet/minecraft/world/entity/item/ItemEntity;setDeltaMovement(Lnet/minecraft/world/phys/Vec3;)V) isn't mapped to a Rust function.
// horizon-mixins kind: BEFORE_CALL on `horizon_client::entities::ItemEntity::inside_block`
// Note: Only whole statements, plain let initializers and block tails can be selected.
// Note: The call it hooks (ItemEntity.setDeltaMovement) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ParticleTypes.SMOKE -> (not mapped yet)
//   ServerLevel.playSound -> (not mapped yet)
//   SoundEvents.GENERIC_BURN -> (not mapped yet)
//   SoundEvents.WOOL_FALL -> (not mapped yet)
//   RandomSource.nextDouble -> (not mapped yet)
//   RandomSource.nextFloat -> (not mapped yet)
//   DamageSources.lava -> (not mapped yet)
//   Entity.<init> -> (not mapped yet)
//   Entity.checkFallDamage -> (not mapped yet)
//   Entity.getFluidInteractionBox -> (not mapped yet)
//   Entity.onSyncedDataUpdated -> (not mapped yet)
//   ItemEntity.applyGravity -> (not mapped yet)
//   ItemEntity.fallDistance -> (not mapped yet)
//   ItemEntity.fireImmune -> (not mapped yet)
//   ItemEntity.getItem -> (not mapped yet)
//   ItemEntity.hurt -> (not mapped yet)
//   ItemEntity.isInLava -> (not mapped yet)
//   ItemEntity.isInWater -> (not mapped yet)
//   ItemEntity.playSound -> (not mapped yet)
//   ItemEntity.playerTouch -> (not mapped yet)
//   ItemEntity.setDeltaMovement -> (not mapped yet)
//   ItemEntity.tick -> horizon_client::entities::ItemEntity::inside_block (suggested, high)
//   Level.addParticle -> (not mapped yet)
//   AABB.inflate -> (not mapped yet)
//
// Decompiled Java, team/creative/itemphysic/mixin/ItemEntityMixin.java:136:
// | @Inject(
// |    method = "tick()V",
// |    at = @At(value = "INVOKE", target = "Lnet/minecraft/world/entity/item/ItemEntity;setDeltaMovement(Lnet/minecraft/world/phys/Vec3;)V", ordinal = 0),
// |    require = 1
// | )
// | public void update(CallbackInfo info) {
// |    CommonPhysic.update((ItemEntity)this);
// | }
