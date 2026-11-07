// Not ported yet: the Java @Inject(TAIL) on ServerLevel.<init>* (ServerLevelMixin.be_onServerWorldInit).
// Why it's off: No Horizon function mapped for ServerLevel.<init>* (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: Wildcard or unknown selector `<init>*`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Holder.is -> (not mapped yet)
//   Holder.unwrapKey -> (not mapped yet)
//   Identifier.getNamespace -> (not mapped yet)
//   ResourceKey.identifier -> (not mapped yet)
//   ServerLevel.setBlockAndUpdate -> (not mapped yet)
//   ServerLevel.tickPrecipitation -> (not mapped yet)
//   Level.<init> -> (not mapped yet)
//   Blocks.ICE -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//   BuiltinDimensionTypes.END -> (not mapped yet)
//   BuiltinDimensionTypes.NETHER -> (not mapped yet)
//   BuiltinDimensionTypes.OVERWORLD -> (not mapped yet)
//   BuiltinDimensionTypes.OVERWORLD_CAVES -> (not mapped yet)
//   DimensionType.hasEnderDragonFight -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/common/ServerLevelMixin.java:59:
// | @Inject(method = "<init>*", at = @At("TAIL"))
// | private void be_onServerWorldInit(
// |    MinecraftServer minecraftServer,
// |    Executor executor,
// |    LevelStorageAccess levelStorageAccess,
// |    ServerLevelData serverLevelData,
// |    ResourceKey resourceKey,
// |    LevelStem levelStem,
// |    boolean bl,
// |    long seed,
// |    List list,
// |    boolean bl2,
// |    CallbackInfo ci
// | ) {
// |    TerrainGenerator.onServerLevelInit(ServerLevel.class.cast(this), levelStem, seed);
// | }
