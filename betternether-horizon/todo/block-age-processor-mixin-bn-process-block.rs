// Not ported yet: the Java @Inject(HEAD) on BlockAgeProcessor.processBlock (BlockAgeProcessorMixin.bn_processBlock).
// Why it's off: No Horizon function mapped for BlockAgeProcessor.processBlock (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for BlockAgeProcessor.processBlock: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.getX -> (not mapped yet)
//   BlockPos.getZ -> (not mapped yet)
//   ServerLevel.getGameRules -> (not mapped yet)
//   RandomSource.nextFloat -> (not mapped yet)
//   ServerLevelAccessor.getLevel -> (not mapped yet)
//   Block.defaultBlockState -> (not mapped yet)
//   Blocks.OBSIDIAN -> (not mapped yet)
//   BlockState.is -> (not mapped yet)
//   GameRules.get -> (not mapped yet)
//   BlockAgeProcessor.processBlock -> (not mapped yet)
//   StructurePlaceSettings.getRandom -> (not mapped yet)
//   StructureBlockInfo.<init> -> (not mapped yet)
//   StructureBlockInfo.nbt -> (not mapped yet)
//   StructureBlockInfo.pos -> (not mapped yet)
//   StructureBlockInfo.state -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/common/BlockAgeProcessorMixin.java:21:
// | @Inject(method = "processBlock", at = @At("HEAD"), cancellable = true)
// | void bn_processBlock(
// |    LevelReader levelReader,
// |    BlockPos targetPosition,
// |    BlockPos referencePos,
// |    BlockPos templateRelativePos,
// |    StructureBlockInfo processedBlockInfo,
// |    StructurePlaceSettings structurePlaceSettings,
// |    CallbackInfoReturnable<StructureBlockInfo> cir
// | ) {
// |    boolean makeBlue = (targetPosition.getX() + targetPosition.getZ()) % 3 == 0;
// |    if (makeBlue && processedBlockInfo.state().is(Blocks.OBSIDIAN) && bn_blueRuinedPortalsEnabled(levelReader)) {
// |       BlockPos structurePos = processedBlockInfo.pos();
// |       RandomSource random = structurePlaceSettings.getRandom(structurePos);
// |       Block block = random.nextFloat() < 0.15F ? NetherObsidianBlocks.BLUE_CRYING_OBSIDIAN.block() : NetherObsidianBlocks.BLUE_OBSIDIAN.block();
// |       cir.setReturnValue(new StructureBlockInfo(structurePos, block.defaultBlockState(), processedBlockInfo.nbt()));
// |       cir.cancel();
// |    }
// | }
