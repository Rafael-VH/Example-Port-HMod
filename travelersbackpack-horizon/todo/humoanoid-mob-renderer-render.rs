// Not ported yet: the Java @Inject(TAIL) on HumanoidMobRenderer.extractHumanoidRenderState (HumoanoidMobRenderer.render).
// Why it's off: No Horizon function mapped for HumanoidMobRenderer.extractHumanoidRenderState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for HumanoidMobRenderer.extractHumanoidRenderState: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   HumanoidMobRenderer.extractHumanoidRenderState -> (not mapped yet)
//   EquipmentSlot.CHEST -> (not mapped yet)
//   LivingEntity.getItemBySlot -> (not mapped yet)
//   Player.getGameProfile -> (not mapped yet)
//   ItemStack.getItem -> (not mapped yet)
//   ItemStack.isEmpty -> (not mapped yet)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/HumoanoidMobRenderer.java:25:
// | @Inject(at = @At("TAIL"), method = "extractHumanoidRenderState")
// | private static void render(LivingEntity entity, HumanoidRenderState reusedState, float partialTick, ItemModelResolver itemModelResolver, CallbackInfo ci) {
// |    if (entity instanceof Player player && reusedState instanceof HumanoidRenderStateBackpackInject injectedReusedState) {
// |       ItemStack backpack = AttachmentUtils.getWearingBackpack(player);
// |       if (!backpack.isEmpty()) {
// |          injectedReusedState.setBackpackStack(backpack);
// |       } else if (injectedReusedState.getBackpackStack() != null) {
// |          injectedReusedState.setBackpackStack(null);
// |       }
// |
// |       injectedReusedState.setName(player.getGameProfile().name());
// |    }
// |
// |    if ((entity instanceof Zombie || entity instanceof Skeleton || entity instanceof Enderman || entity instanceof WitherSkeleton || entity instanceof Piglin)
// |       && reusedState instanceof HumanoidRenderStateBackpackInject injectedReusedState) {
// |       ItemStack backpack = entity.getItemBySlot(EquipmentSlot.CHEST);
// |       if (backpack.getItem() instanceof TravelersBackpackItem) {
// |          injectedReusedState.setChestItem(backpack);
// |       }
// |    }
// | }
