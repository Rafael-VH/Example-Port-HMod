// Not ported yet: the Java @Redirect(INVOKE) on JigsawStructure.findGenerationPoint (JigsawStructureMixin.init).
// Why it's off: No Horizon function mapped for JigsawStructure.findGenerationPoint (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: A redirect is a WRAP_OPERATION that doesn't call `original`.
// Note: No Horizon function known for JigsawStructure.findGenerationPoint: run `match`.
// Note: The call it hooks (JigsawPlacement.addPieces) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ConstantInt.of -> (not mapped yet)
//   VerticalAnchor.BOTTOM -> (not mapped yet)
//   ConstantHeight.of -> (not mapped yet)
//   JigsawPlacement.addPieces -> (not mapped yet)
//   JigsawStructure.findGenerationPoint -> (not mapped yet)
//   MaxDistance.horizontal -> (not mapped yet)
//   MaxDistance.vertical -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/JigsawStructureMixin.java:37:
// | @Redirect(
// |    method = "findGenerationPoint",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/level/levelgen/structure/pools/JigsawPlacement;addPieces(Lnet/minecraft/world/level/levelgen/structure/Structure$GenerationContext;Lnet/minecraft/core/Holder;Ljava/util/Optional;ILnet/minecraft/core/BlockPos;ZLjava/util/Optional;Lnet/minecraft/world/level/levelgen/structure/structures/JigsawStructure$MaxDistance;Lnet/minecraft/world/level/levelgen/structure/pools/alias/PoolAliasLookup;Lnet/minecraft/world/level/levelgen/structure/pools/DimensionPadding;Lnet/minecraft/world/level/levelgen/structure/templatesystem/LiquidSettings;)Ljava/util/Optional;"
// |    )
// | )
// | private Optional<GenerationStub> init(
// |    GenerationContext context,
// |    Holder<StructureTemplatePool> startPool,
// |    Optional<Identifier> startJigsawName,
// |    int size,
// |    BlockPos pos,
// |    boolean useExpansionHack,
// |    Optional<Types> heightmapProjection,
// |    MaxDistance maxDistToCenter,
// |    PoolAliasLookup lookup,
// |    DimensionPadding padding,
// |    LiquidSettings liquidSettings
// | ) {
// |    return AlternateJigsawStructure.generate(
// |       context,
// |       new AlternateJigsawConfig(
// |          startPool,
// |          startJigsawName,
// |          ConstantInt.of(size),
// |          false,
// |          ConstantHeight.of(VerticalAnchor.BOTTOM),
// |          useExpansionHack,
// |          heightmapProjection.map(Either::right),
// |          MaxDistanceFromCenter.of(maxDistToCenter.horizontal(), maxDistToCenter.vertical()),
// |          this.poolAliases,
// |          padding,
// |          liquidSettings
// |       ),
// |       true,
// |       size,
// |       pos,
// |       lookup
// |    );
// | }
