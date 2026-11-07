// Not ported yet: the Java @WrapOperation(INVOKE) on NetherFossilStructure.findGenerationPoint (NetherFossilStructureMixin.useTemplateList).
// Why it's off: No Horizon function mapped for NetherFossilStructure.findGenerationPoint (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: No Horizon function known for NetherFossilStructure.findGenerationPoint: run `match`.
// Note: The call it hooks (Optional.of) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Optional.of -> (not mapped yet)
//   Rotation.getRandom -> horizon_world::worldgen::structure::Rotation::random (suggested, high)
//   GenerationContext.random -> (not mapped yet)
//   GenerationContext.registryAccess -> (not mapped yet)
//   GenerationContext.structureTemplateManager -> (not mapped yet)
//   GenerationStub.<init> -> (not mapped yet)
//   StructurePiecesBuilder.addPiece -> (not mapped yet)
//   NetherFossilPiece.<init> -> (not mapped yet)
//   NetherFossilStructure.findGenerationPoint -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/template/NetherFossilStructureMixin.java:21:
// | @WrapOperation(method = "findGenerationPoint", at = @At(value = "INVOKE", target = "Ljava/util/Optional;of(Ljava/lang/Object;)Ljava/util/Optional;"))
// | private Optional<GenerationStub> useTemplateList(
// |    Object stub, Operation<Optional<GenerationStub>> operation, GenerationContext context, @Local(ordinal = 0) BlockPos pos
// | ) {
// |    return !Lithostitched.breaksSeedParity() ? (Optional)operation.call(new Object[]{stub}) : Optional.of(new GenerationStub(pos, builder -> {
// |       Rotation rotation = Rotation.getRandom(context.random());
// |       Identifier template = TemplateLists.getRandom(context.registryAccess(), TemplateLists.NETHER_FOSSIL, context.random());
// |       builder.addPiece(new NetherFossilPiece(context.structureTemplateManager(), template, pos, rotation));
// |    }));
// | }
