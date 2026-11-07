// Not ported yet: the Java @Inject(HEAD) on SimpleRegionStorage.upgradeChunkTag (EntityChunkUpgradeMixin.graveyard$upgradeAroundModEntities).
// Why it's off: No Horizon function mapped for SimpleRegionStorage.upgradeChunkTag (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for SimpleRegionStorage.upgradeChunkTag: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   DataFixTypes.ENTITY_CHUNK -> (not mapped yet)
//   SimpleRegionStorage.upgradeChunkTag -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/EntityChunkUpgradeMixin.java:20:
// | @Inject(
// |    method = "upgradeChunkTag(Lnet/minecraft/nbt/CompoundTag;ILnet/minecraft/nbt/CompoundTag;I)Lnet/minecraft/nbt/CompoundTag;",
// |    at = @At("HEAD"),
// |    cancellable = true
// | )
// | private void graveyard$upgradeAroundModEntities(
// |    CompoundTag tag, int defaultVersion, CompoundTag context, int targetVersion, CallbackInfoReturnable<CompoundTag> cir
// | ) {
// |    if (this.dataFixType == DataFixTypes.ENTITY_CHUNK) {
// |       SimpleRegionStorage self = (SimpleRegionStorage)this;
// |       CompoundTag result = EntityChunkUpgrade.upgradeWithoutGraveyardEntities(
// |          tag, defaultVersion, targetVersion, copy -> self.upgradeChunkTag(copy, defaultVersion, context, targetVersion)
// |       );
// |       if (result != null) {
// |          cir.setReturnValue(result);
// |       }
// |    }
// | }
