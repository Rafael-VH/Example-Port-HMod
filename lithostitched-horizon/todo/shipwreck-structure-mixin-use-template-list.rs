// Not ported yet: the Java @WrapOperation(INVOKE) on ShipwreckStructure.generatePieces (ShipwreckStructureMixin.useTemplateList).
// Why it's off: No Horizon function mapped for ShipwreckStructure.generatePieces (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for ShipwreckStructure.generatePieces: run `match`.
// Note: The call it hooks (ShipwreckPieces.addRandomPiece) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GenerationContext.registryAccess -> (not mapped yet)
//   StructurePiecesBuilder.addPiece -> (not mapped yet)
//   ShipwreckPieces.addRandomPiece -> (not mapped yet)
//   ShipwreckPiece.<init> -> (not mapped yet)
//   ShipwreckStructure.generatePieces -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/template/ShipwreckStructureMixin.java:22:
// | @WrapOperation(
// |    method = "generatePieces",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/world/level/levelgen/structure/structures/ShipwreckPieces;addRandomPiece(Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplateManager;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Rotation;Lnet/minecraft/world/level/levelgen/structure/pieces/StructurePiecesBuilder;Lnet/minecraft/util/RandomSource;Z)Lnet/minecraft/world/level/levelgen/structure/structures/ShipwreckPieces$ShipwreckPiece;"
// |    )
// | )
// | private ShipwreckPiece useTemplateList(
// |    StructureTemplateManager templateManager,
// |    BlockPos pos,
// |    Rotation rotation,
// |    StructurePiecesBuilder builder,
// |    RandomSource random,
// |    boolean beached,
// |    Operation<ShipwreckPiece> operation,
// |    @Local(ordinal = 0, argsOnly = true) GenerationContext context
// | ) {
// |    if (!Lithostitched.breaksSeedParity()) {
// |       return (ShipwreckPiece)operation.call(new Object[]{templateManager, pos, rotation, builder, random, beached});
// |    }
// |
// |    Identifier id = TemplateLists.getRandom(context.registryAccess(), beached ? TemplateLists.SHIPWRECK_BEACHED : TemplateLists.SHIPWRECK_OCEAN, random);
// |    ShipwreckPiece piece = new ShipwreckPiece(templateManager, id, pos, rotation, beached);
// |    builder.addPiece(piece);
// |    return piece;
// | }
