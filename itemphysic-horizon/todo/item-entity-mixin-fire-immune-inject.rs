// Not ported yet: the Java @Inject(HEAD) on ItemEntity.fireImmune (ItemEntityMixin.fireImmuneInject).
// Why it's off: No Horizon function mapped for ItemEntity.fireImmune (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ItemEntity.fireImmune: run `match`.
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
// Decompiled Java, team/creative/itemphysic/mixin/ItemEntityMixin.java:110:
// | @Inject(method = "fireImmune()Z", at = @At("HEAD"), cancellable = true, require = 1)
// | public void fireImmuneInject(CallbackInfoReturnable<Boolean> info) {
// |    if (CommonPhysic.fireImmune((ItemEntity)this)) {
// |       info.setReturnValue(true);
// |    }
// | }
