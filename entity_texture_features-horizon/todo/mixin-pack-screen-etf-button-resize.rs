// Not ported yet: the Java @Inject(TAIL) on PackSelectionScreen.repositionElements (MixinPackScreen.etf$etfButtonResize).
// Why it's off: No Horizon function mapped for PackSelectionScreen.repositionElements (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for PackSelectionScreen.repositionElements: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.getResourcePackDirectory -> (not mapped yet)
//   Button.getHeight -> (not mapped yet)
//   Button.getWidth -> (not mapped yet)
//   Button.getX -> (not mapped yet)
//   Button.getY -> (not mapped yet)
//   ImageButton.setX -> (not mapped yet)
//   ImageButton.setY -> (not mapped yet)
//   WidgetSprites.<init> -> (not mapped yet)
//   Screen.<init> -> (not mapped yet)
//   PackSelectionScreen.init -> (not mapped yet)
//   PackSelectionScreen.repositionElements -> (not mapped yet)
//   Component.nullToEmpty -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/MixinPackScreen.java:46:
// | @Inject(method = "repositionElements", at = @At("TAIL"))
// | private void etf$etfButtonResize(CallbackInfo ci) {
// |    if (this.etf$button != null) {
// |       int[] vals = this.etf$etfButtonReSize();
// |       if (vals != null) {
// |          int x = vals[0];
// |          int y = vals[1];
// |          this.etf$button.setX(x);
// |          this.etf$button.setY(y);
// |       }
// |    }
// | }
