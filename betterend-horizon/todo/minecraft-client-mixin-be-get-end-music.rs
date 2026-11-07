// Not ported yet: the Java @Inject(HEAD) on Minecraft.getSituationalMusic (MinecraftClientMixin.be_getEndMusic).
// Why it's off: No Horizon function mapped for Minecraft.getSituationalMusic (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for Minecraft.getSituationalMusic: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.getSituationalMusic -> (not mapped yet)
//   Gui.hud -> (not mapped yet)
//   Gui.screen -> (not mapped yet)
//   Hud.getBossOverlay -> (not mapped yet)
//   BossHealthOverlay.shouldPlayMusic -> (not mapped yet)
//   ClientLevel.getBiomeManager -> (not mapped yet)
//   LocalPlayer.blockPosition -> (not mapped yet)
//   LocalPlayer.getX -> (not mapped yet)
//   LocalPlayer.getZ -> (not mapped yet)
//   LocalPlayer.level -> (not mapped yet)
//   Holder.value -> (not mapped yet)
//   Musics.END -> (not mapped yet)
//   Musics.END_BOSS -> (not mapped yet)
//   BackgroundMusic.EMPTY -> (not mapped yet)
//   BackgroundMusic.select -> horizon_client::music::BackgroundMusic::select (suggested, high)
//   EnvironmentAttributeMap.applyModifier -> (not mapped yet)
//   EnvironmentAttributes.BACKGROUND_MUSIC -> (not mapped yet)
//   Level.END -> (not mapped yet)
//   Level.dimension -> (not mapped yet)
//   Biome.getAttributes -> (not mapped yet)
//   BiomeManager.getNoiseBiomeAtPosition -> (not mapped yet)
//
// Decompiled Java, org/betterx/betterend/mixin/client/MinecraftClientMixin.java:32:
// | @Inject(method = "getSituationalMusic", at = @At("HEAD"), cancellable = true)
// | private void be_getEndMusic(CallbackInfoReturnable<Music> info) {
// |    if (!(this.gui.screen() instanceof WinScreen) && this.player != null && this.player.level().dimension() == Level.END) {
// |       if (this.gui.hud.getBossOverlay().shouldPlayMusic() && MHelper.lengthSqr(this.player.getX(), this.player.getZ()) < 250000.0) {
// |          info.setReturnValue(Musics.END_BOSS);
// |       } else {
// |          BackgroundMusic backgroundMusic = (BackgroundMusic)((Biome)this.level
// |                .getBiomeManager()
// |                .getNoiseBiomeAtPosition(this.player.blockPosition())
// |                .value())
// |             .getAttributes()
// |             .applyModifier(EnvironmentAttributes.BACKGROUND_MUSIC, BackgroundMusic.EMPTY);
// |          Music sound = backgroundMusic.select(false, false).orElse(Musics.END);
// |          info.setReturnValue(sound);
// |       }
// |
// |       info.cancel();
// |    }
// | }
