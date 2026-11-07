// Not ported yet: the Java @Inject(HEAD) on WoodlandMansionStructure.generatePieces (WoodlandMansionStructureMixin.injectRegistriesForTemplateList).
// Why it's off: No Horizon function mapped for WoodlandMansionStructure.generatePieces (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for WoodlandMansionStructure.generatePieces: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GenerationContext.random -> (not mapped yet)
//   GenerationContext.registryAccess -> (not mapped yet)
//   GenerationContext.structureTemplateManager -> (not mapped yet)
//   StructurePiecesBuilder.addPiece -> (not mapped yet)
//   MansionGrid.<init> -> (not mapped yet)
//   MansionPiecePlacer.<init> -> (not mapped yet)
//   MansionPiecePlacer.createMansion -> (not mapped yet)
//   WoodlandMansionStructure.generatePieces -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/template/mansion/WoodlandMansionStructureMixin.java:22:
// | @Inject(method = "generatePieces", at = @At("HEAD"), cancellable = true)
// | private void injectRegistriesForTemplateList(StructurePiecesBuilder builder, GenerationContext context, BlockPos pos, Rotation rotation, CallbackInfo ci) {
// |    if (Lithostitched.breaksSeedParity()) {
// |       LinkedList<WoodlandMansionPiece> list = Lists.newLinkedList();
// |       MansionGrid grid = new MansionGrid(context.random());
// |       MansionPiecePlacer placer = new MansionPiecePlacer(context.structureTemplateManager(), context.random());
// |       ((RegistryHolder)placer).setRegistries(context.registryAccess());
// |       placer.createMansion(pos, rotation, list, grid);
// |       list.forEach(builder::addPiece);
// |       ci.cancel();
// |    }
// | }
