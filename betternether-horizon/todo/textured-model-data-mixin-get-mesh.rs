// Not ported yet: the Java @Accessor on LayerDefinition.mesh (TexturedModelDataMixin.getMesh).
// Why it's off: No Horizon function mapped for LayerDefinition.mesh (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for LayerDefinition.mesh: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LayerDefinition.mesh -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/client/TexturedModelDataMixin.java:10:
// | @Accessor("mesh")
// | MeshDefinition getMesh();
