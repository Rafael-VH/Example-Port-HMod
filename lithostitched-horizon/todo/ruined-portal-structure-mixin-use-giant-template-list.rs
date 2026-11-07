// Not ported yet: the Java @Redirect(INVOKE) on RuinedPortalStructure.findGenerationPoint (RuinedPortalStructureMixin.useGiantTemplateList).
// Why it's off: No Horizon function mapped for RuinedPortalStructure.findGenerationPoint (run `match`, or map it in a --mappings file).
// horizon-mixins kind: WRAP_OPERATION
// Note: A redirect is a WRAP_OPERATION that doesn't call `original`.
// Note: No Horizon function known for RuinedPortalStructure.findGenerationPoint: run `match`.
// Note: The call it hooks (Identifier.withDefaultNamespace) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Identifier.withDefaultNamespace -> (not mapped yet)
//   GenerationContext.random -> (not mapped yet)
//   GenerationContext.registryAccess -> (not mapped yet)
//   RuinedPortalStructure.findGenerationPoint -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/template/RuinedPortalStructureMixin.java:28:
// | @Redirect(
// |    method = "findGenerationPoint",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/resources/Identifier;withDefaultNamespace(Ljava/lang/String;)Lnet/minecraft/resources/Identifier;",
// |       ordinal = 0
// |    )
// | )
// | private Identifier useGiantTemplateList(String name, GenerationContext context) {
// |    return !Lithostitched.breaksSeedParity()
// |       ? Identifier.withDefaultNamespace(name)
// |       : TemplateLists.getRandom(context.registryAccess(), TemplateLists.RUINED_PORTAL_GIANT, context.random());
// | }
