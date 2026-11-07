// Not ported yet: the Java @Accessor on StructureTemplatePool.templates (StructureTemplatePoolAccessor.setVanillaTemplates).
// Why it's off: No Horizon function mapped for StructureTemplatePool.templates (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for StructureTemplatePool.templates: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   StructureTemplatePool.rawTemplates -> (not mapped yet)
//   StructureTemplatePool.templates -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/StructureTemplatePoolAccessor.java:26:
// | @Accessor("templates")
// | @Mutable
// | void setVanillaTemplates(ObjectArrayList<StructurePoolElement> var1);
