// Not ported yet: the Java @Inject(TAIL) on LootTable.getRandomItems (LootTableMixin.setTarget).
// Why it's off: No Horizon function mapped for LootTable.getRandomItems (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for LootTable.getRandomItems: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   RandomSource.nextFloat -> (not mapped yet)
//   RandomSource.nextInt -> (not mapped yet)
//   ItemStack.<init> -> (not mapped yet)
//   ItemStack.getCount -> horizon_world::loot::LootStack::get_count (suggested, high)
//   ItemStack.setCount -> (not mapped yet)
//   Items.BEETROOT -> (not mapped yet)
//   Items.CARROT -> (not mapped yet)
//   Items.POTATO -> (not mapped yet)
//   CropBlock.AGE -> (not mapped yet)
//   BlockState.getBlock -> (not mapped yet)
//   BlockState.getValue -> (not mapped yet)
//   BlockState.hasProperty -> (not mapped yet)
//   LootContext.getOptional -> (not mapped yet)
//   LootContext.getRandom -> (not mapped yet)
//   LootTable.getRandomItems -> (not mapped yet)
//   LootContextParams.BLOCK_STATE -> (not mapped yet)
//   LootContextParams.THIS_ENTITY -> (not mapped yet)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/abilities/LootTableMixin.java:25:
// | @Inject(
// |    at = @At("TAIL"),
// |    method = "getRandomItems(Lnet/minecraft/world/level/storage/loot/LootContext;)Lit/unimi/dsi/fastutil/objects/ObjectArrayList;",
// |    cancellable = true
// | )
// | public void setTarget(LootContext context, CallbackInfoReturnable<ObjectArrayList<ItemStack>> cir) {
// |    ObjectArrayList<ItemStack> generatedLoot = (ObjectArrayList<ItemStack>)cir.getReturnValue();
// |    BlockState blockState = (BlockState)context.getOptional(LootContextParams.BLOCK_STATE);
// |    boolean grassVariant = false;
// |    if (blockState != null) {
// |       if (blockState.getBlock() instanceof TallGrassBlock) {
// |          grassVariant = true;
// |       }
// |
// |       if (grassVariant
// |          || blockState.getBlock() instanceof CropBlock && blockState.hasProperty(CropBlock.AGE) && (Integer)blockState.getValue(CropBlock.AGE) == 7) {
// |          boolean modifiedLoot = false;
// |          Entity entity = (Entity)context.getOptional(LootContextParams.THIS_ENTITY);
// |          if (entity instanceof Player player && BackpackAbilities.ABILITIES.checkBackpack(player, ModItems.HAY_TRAVELERS_BACKPACK)) {
// |             if (grassVariant) {
// |                if (context.getRandom().nextFloat() < 0.15F) {
// |                   List<ItemStack> possibleCrops = List.of(new ItemStack(Items.CARROT), new ItemStack(Items.POTATO), new ItemStack(Items.BEETROOT));
// |                   if (!possibleCrops.isEmpty()) {
// |                      ItemStack randomCrop = possibleCrops.get(context.getRandom().nextInt(possibleCrops.size()));
// |                      generatedLoot.add(randomCrop);
// |                      modifiedLoot = true;
// |                   }
// |                }
// |             } else {
// |                ObjectListIterator var12 = generatedLoot.iterator();
// |
// |                while (var12.hasNext()) {
// |                   ItemStack stack = (ItemStack)var12.next();
// |                   if (context.getRandom().nextFloat() < 0.4F) {
// |                      int count = stack.getCount();
// |                      stack.setCount(count * 2);
// |                      modifiedLoot = true;
// |                   }
// |                }
// |             }
// |          }
// |
// |          if (modifiedLoot) {
// |             cir.setReturnValue(generatedLoot);
// |          }
// |       }
// |    }
// | }
