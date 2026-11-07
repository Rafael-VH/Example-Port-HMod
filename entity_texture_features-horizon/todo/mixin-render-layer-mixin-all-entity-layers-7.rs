// Not ported yet: the Java @ModifyVariable(HEAD) on RenderTypes.entityCutout (MixinRenderLayer.etf$mixinAllEntityLayers).
// Why it's off: No Horizon function mapped for RenderTypes.entityCutout (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for RenderTypes.entityCutout: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   RenderTypes.armorCutoutNoCull -> (not mapped yet)
//   RenderTypes.energySwirl -> (not mapped yet)
//   RenderTypes.entityCutout -> (not mapped yet)
//   RenderTypes.entityCutoutCull -> (not mapped yet)
//   RenderTypes.entityCutoutDissolve -> (not mapped yet)
//   RenderTypes.entityCutoutNoCullZOffset -> (not mapped yet)
//   RenderTypes.entityCutoutZOffset -> (not mapped yet)
//   RenderTypes.entityDecal -> (not mapped yet)
//   RenderTypes.entityNoOutline -> (not mapped yet)
//   RenderTypes.entityShadow -> (not mapped yet)
//   RenderTypes.entitySmoothCutout -> (not mapped yet)
//   RenderTypes.entitySolid -> (not mapped yet)
//   RenderTypes.entitySolidZOffsetForward -> (not mapped yet)
//   RenderTypes.entityTranslucent -> (not mapped yet)
//   RenderTypes.entityTranslucentCullItemTarget -> (not mapped yet)
//   RenderTypes.entityTranslucentEmissive -> (not mapped yet)
//   RenderTypes.eyes -> (not mapped yet)
//   RenderTypes.itemEntityTranslucentCull -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/MixinRenderLayer.java:14:
// |          "entitySolid",
// |          "eyes",
// |          "energySwirl",
// |          "entitySmoothCutout(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;",
// |          "itemEntityTranslucentCull(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;",
// |          "entityCutoutCull",
// |          "entityCutout(Lnet/minecraft/resources/Identifier;Z)Lnet/minecraft/client/renderer/rendertype/RenderType;",
// |          "entityCutoutNoCullZOffset(Lnet/minecraft/resources/Identifier;Z)Lnet/minecraft/client/renderer/rendertype/RenderType;",
// |          "entityDecal(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;",
// |          "entityNoOutline(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;",
// |          "entitySmoothCutout(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;",
// |          "entityTranslucent(Lnet/minecraft/resources/Identifier;Z)Lnet/minecraft/client/renderer/rendertype/RenderType;",
// |          "entityTranslucentEmissive",
// |          "armorCutoutNoCull",
// |          "entityShadow",
// |          "entityTranslucentCullItemTarget(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;",
// |          "entityCutoutZOffset(Lnet/minecraft/resources/Identifier;Z)Lnet/minecraft/client/renderer/rendertype/RenderType;",
// |          "entityCutoutDissolve",
// |          "entitySolidZOffsetForward"
// |    },
// |    at = @At("HEAD"),
// |    index = 0,
// |    argsOnly = true
// | )
// | private static Identifier etf$mixinAllEntityLayers(Identifier value) {
// |    return ETFUtils2.getETFVariantNotNullForInjector(value);
// | }
