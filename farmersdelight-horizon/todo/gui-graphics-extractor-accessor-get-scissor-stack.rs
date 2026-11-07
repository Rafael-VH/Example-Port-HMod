// Not ported yet: the Java @Accessor on GuiGraphicsExtractor.scissorStack (GuiGraphicsExtractorAccessor.fdrf$getScissorStack).
// Why it's off: No Horizon function mapped for GuiGraphicsExtractor.scissorStack (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for GuiGraphicsExtractor.scissorStack: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GuiGraphicsExtractor.guiRenderState -> (not mapped yet)
//   GuiGraphicsExtractor.scissorStack -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/GuiGraphicsExtractorAccessor.java:14:
// | @Accessor("scissorStack")
// | ScissorStack fdrf$getScissorStack();
