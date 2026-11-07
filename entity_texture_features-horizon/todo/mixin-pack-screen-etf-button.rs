// Not ported yet: the Java @Inject(TAIL) on PackSelectionScreen.init (MixinPackScreen.etf$etfButton).
// Why it's off: No Horizon function mapped for PackSelectionScreen.init (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for PackSelectionScreen.init: run `match`.
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
// Decompiled Java, traben/entity_texture_features/mixin/mixins/MixinPackScreen.java:59:
// | @Inject(method = "init", at = @At("TAIL"))
// | private void etf$etfButton(CallbackInfo ci) {
// |    if (ETF.config().getConfig().configButtonLoc != ETFConfig.SettingsButtonLocation.OFF) {
// |       if (this.minecraft != null && this.packDir.equals(this.minecraft.getResourcePackDirectory()) && ETF.isFabric() == ETF.FABRIC_API) {
// |          int[] vals = this.etf$etfButtonReSize();
// |          if (vals != null) {
// |             int x = vals[0];
// |             int y = vals[1];
// |             this.etf$button = (ImageButton)this.addRenderableWidget(
// |                new ImageButton(
// |                   x,
// |                   y,
// |                   24,
// |                   20,
// |                   new WidgetSprites(etf$UNFOCUSED, etf$FOCUSED),
// |                   button -> UScreen.setScreen(new ETFConfigScreenMain(this)),
// |                   Component.nullToEmpty("")
// |                ) {
// |                   {
// |                      this.setTooltip(Tooltip.create(ETF.getTextFromTranslation("config.entity_features.button_tooltip")));
// |                   }
// |
// |                   public void extractContents(GuiGraphicsExtractor context, int mouseX, int mouseY, float delta) {
// |                      Identifier identifier = this.isHoveredOrFocused() ? MixinPackScreen.etf$FOCUSED : MixinPackScreen.etf$UNFOCUSED;
// |                      context.blit(
// |                         RenderPipelines.GUI_TEXTURED, identifier, this.getX(), this.getY(), 0.0F, 0.0F, this.width, this.height, this.width, this.height
// |                      );
// |                   }
// |                }
// |             );
// |          }
// |       }
// |    }
// | }
