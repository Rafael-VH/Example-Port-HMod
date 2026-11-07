# Java translated to Rust

Translated 24 of 1075 methods (1 more is done another way in Horizon) and 28 of 62 static fields to Rust; 0 of 1 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 833 | `IrisHandler.IrisHandlerImpl.IrisHandlerImpl(0)`: its class isn't translated (it extends `IrisHandler`) |
| Other Java this translator doesn't handle yet | 139 | `IrisHandler.isShaderActive(0)`: it has no body |
| Uses Minecraft classes (needs Horizon's game API) | 63 | `ETFConfigScreenSkinTool.renderEntityInInventoryFollowsMouse(11)`: it uses Minecraft's `LivingEntity` |
| Calls the game where Horizon has no helper yet | 3 | `ETFScreenOldCompat.renderGUITexture(6)`: it calls Minecraft's GuiGraphics.blit (no Horizon helper for it yet) |
| Uses Java collections other than List (Map, Set...) | 3 | `ETF.randomQuip(0)`: it creates `Random` (java.util) |
| Uses null | 3 | `ETFConfigWarnings.registerConfigWarning(1)`: `null` (Rust has no null: this needs an Option) |
| Uses other libraries | 2 | `SodiumGetBufferInjector.get(0)`: it uses `TriConsumer` (org.apache.logging.log4j.util) |
| Uses try/catch | 2 | `ETFUtils2.replaceIdentifier(3)`: a try/catch (IdentifierException, Exception) |
| Done another way in Horizon | 1 | `ETFModMenu.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) |
| Uses Fabric API | 1 | `ETF.modsLoaded(0)`: it calls Fabric's FabricLoader.getAllMods |
| Uses translated text (Component.translatable) | 1 | `ETF.getTextFromTranslation(1)`: a translated text with a computed key |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| traben/entity_texture_features/ETFInit.java:9 | `init` | not translated: it calls ETF.start, which isn't translated |

## Every class


### `IrisHandler`

- [x] `IrisHandler.instance` (traben/entity_texture_features/compat/IrisHandler.java:9)
- [ ] `IrisHandler.isShaderActive(0)`: it has no body (traben/entity_texture_features/compat/IrisHandler.java:11)
- [ ] `IrisHandler.getInstance(0)`: its body couldn't be read: an anonymous class (traben/entity_texture_features/compat/IrisHandler.java:13)

### `IrisHandler.IrisHandlerImpl`: it extends `IrisHandler`

- [ ] `IrisHandler.IrisHandlerImpl.IrisHandlerImpl(0)`: its class isn't translated (it extends `IrisHandler`) (traben/entity_texture_features/compat/IrisHandler.java:32)
- [ ] `IrisHandler.IrisHandlerImpl.isShaderActive(0)`: its class isn't translated (it extends `IrisHandler`) (traben/entity_texture_features/compat/IrisHandler.java:40)

### `SodiumGetBufferInjector`

- [ ] `SodiumGetBufferInjector.INSTANCE`: it uses `TriConsumer` (org.apache.logging.log4j.util) (traben/entity_texture_features/compat/SodiumGetBufferInjector.java:9)
- [ ] `SodiumGetBufferInjector.inject(3)`: it uses URenderTypeToVertexConsumer, which isn't translated (it uses Minecraft's `RenderTypeFeatureRenderer`) (traben/entity_texture_features/compat/SodiumGetBufferInjector.java:11)
- [ ] `SodiumGetBufferInjector.get(0)`: it uses `TriConsumer` (org.apache.logging.log4j.util) (traben/entity_texture_features/compat/SodiumGetBufferInjector.java:17)

### `ETFConfig`: it extends `TConfig` (traben.tconfig)

- [ ] `ETFConfig.isPropertyDisabled(1)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/config/ETFConfig.java:88)
- [ ] `ETFConfig.canPropertyUpdate(1)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/config/ETFConfig.java:92)
- [ ] `ETFConfig.canDoCustomTextures(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/config/ETFConfig.java:96)
- [ ] `ETFConfig.canDoEmissiveTextures(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/config/ETFConfig.java:110)
- [ ] `ETFConfig.getEmissiveRenderMode(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/config/ETFConfig.java:124)
- [ ] `ETFConfig.getRenderLayerOverride(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/config/ETFConfig.java:138)
- [ ] `ETFConfig.getLightOverride(3)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/config/ETFConfig.java:152)
- [ ] `ETFConfig.getLightOverrideBE(1)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/config/ETFConfig.java:169)
- [ ] `ETFConfig.getLightOverrideBE(2)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/config/ETFConfig.java:173)
- [ ] `ETFConfig.getGUIOptions(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/config/ETFConfig.java:195)
- [ ] `ETFConfig.getEntitySettings(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/config/ETFConfig.java:427)
- [ ] `ETFConfig.addEntityConfigs(2)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/config/ETFConfig.java:455)
- [ ] `ETFConfig.getPropertySettings(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/config/ETFConfig.java:512)
- [ ] `ETFConfig.getPlayerSkinEditorButton(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/config/ETFConfig.java:556)
- [ ] `ETFConfig.getModIcon(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/config/ETFConfig.java:598)

### `ETFConfig.DebugLogMode`: its constants carry values

- [ ] `ETFConfig.DebugLogMode.DebugLogMode(1)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/ETFConfig.java:610)
- [ ] `ETFConfig.DebugLogMode.toString(0)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/ETFConfig.java:614)

### `ETFConfig.EmissiveRenderModes`

- [ ] `ETFConfig.EmissiveRenderModes.toString(0)`: `this` used as a value (traben/entity_texture_features/config/ETFConfig.java:624)

### `ETFConfig.IllegalPathMode`: its constants carry values

- [ ] `ETFConfig.IllegalPathMode.IllegalPathMode(1)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/ETFConfig.java:640)
- [ ] `ETFConfig.IllegalPathMode.toString(0)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/ETFConfig.java:644)

### `ETFConfig.OverrideBooleanType`

- [ ] `ETFConfig.OverrideBooleanType.toString(0)`: `this` used as a value (traben/entity_texture_features/config/ETFConfig.java:654)

### `ETFConfig.RenderLayerOverride`: its constants carry values

- [ ] `ETFConfig.RenderLayerOverride.RenderLayerOverride(1)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/ETFConfig.java:668)
- [ ] `ETFConfig.RenderLayerOverride.toString(0)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/ETFConfig.java:672)

### `ETFConfig.SettingsButtonLocation`


### `ETFConfig.SkinTransparencyMode`: its constants carry values

- [ ] `ETFConfig.SkinTransparencyMode.SkinTransparencyMode(1)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/ETFConfig.java:693)
- [ ] `ETFConfig.SkinTransparencyMode.toString(0)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/ETFConfig.java:697)

### `ETFConfig.String2BooleanNullMap`: it extends `HashMap` (java.util)

- [ ] `ETFConfig.String2BooleanNullMap.putNullable(2)`: its class isn't translated (it extends `HashMap` (java.util)) (traben/entity_texture_features/config/ETFConfig.java:704)
- [ ] `ETFConfig.String2BooleanNullMap.getNullable(1)`: its class isn't translated (it extends `HashMap` (java.util)) (traben/entity_texture_features/config/ETFConfig.java:712)

### `ETFConfig.String2EnumNullMap`: it's generic

- [ ] `ETFConfig.String2EnumNullMap.putNullable(2)`: its class isn't translated (it's generic) (traben/entity_texture_features/config/ETFConfig.java:722)
- [ ] `ETFConfig.String2EnumNullMap.getNullable(1)`: its class isn't translated (it's generic) (traben/entity_texture_features/config/ETFConfig.java:730)

### `ETFConfig.UpdateFrequency`: its constants carry values

- [ ] `ETFConfig.UpdateFrequency.UpdateFrequency(2)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/ETFConfig.java:746)
- [ ] `ETFConfig.UpdateFrequency.getDelay(0)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/ETFConfig.java:751)
- [ ] `ETFConfig.UpdateFrequency.toString(0)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/ETFConfig.java:755)

### `ETFConfigWarning`

- [ ] `ETFConfigWarning.isConditionMet(0)`: it has no body (traben/entity_texture_features/config/ETFConfigWarning.java:10)
- [ ] `ETFConfigWarning.getTitle(0)`: it has no body (traben/entity_texture_features/config/ETFConfigWarning.java:12)
- [ ] `ETFConfigWarning.getSubTitle(0)`: it has no body (traben/entity_texture_features/config/ETFConfigWarning.java:14)
- [ ] `ETFConfigWarning.getID(0)`: it has no body (traben/entity_texture_features/config/ETFConfigWarning.java:16)
- [ ] `ETFConfigWarning.testWarningAndApplyFixIfEnabled(0)`: it has no body (traben/entity_texture_features/config/ETFConfigWarning.java:18)
- [ ] `ETFConfigWarning.isEnabled(0)`: it calls ETFConfigWarning.isConditionMet, which isn't translated (traben/entity_texture_features/config/ETFConfigWarning.java:20)
- [ ] `ETFConfigWarning.doesShowDisableButton(0)`: it has no body (traben/entity_texture_features/config/ETFConfigWarning.java:24)

### `ETFConfigWarning.Simple`: it extends `ETFConfigWarning`

- [ ] `ETFConfigWarning.Simple.Simple(5)`: its class isn't translated (it extends `ETFConfigWarning`) (traben/entity_texture_features/config/ETFConfigWarning.java:34)
- [ ] `ETFConfigWarning.Simple.Simple(5)`: its class isn't translated (it extends `ETFConfigWarning`) (traben/entity_texture_features/config/ETFConfigWarning.java:48)
- [ ] `ETFConfigWarning.Simple.isConditionMet(0)`: its class isn't translated (it extends `ETFConfigWarning`) (traben/entity_texture_features/config/ETFConfigWarning.java:58)
- [ ] `ETFConfigWarning.Simple.getTitle(0)`: its class isn't translated (it extends `ETFConfigWarning`) (traben/entity_texture_features/config/ETFConfigWarning.java:63)
- [ ] `ETFConfigWarning.Simple.getSubTitle(0)`: its class isn't translated (it extends `ETFConfigWarning`) (traben/entity_texture_features/config/ETFConfigWarning.java:68)
- [ ] `ETFConfigWarning.Simple.getID(0)`: its class isn't translated (it extends `ETFConfigWarning`) (traben/entity_texture_features/config/ETFConfigWarning.java:73)
- [ ] `ETFConfigWarning.Simple.testWarningAndApplyFixIfEnabled(0)`: its class isn't translated (it extends `ETFConfigWarning`) (traben/entity_texture_features/config/ETFConfigWarning.java:78)
- [ ] `ETFConfigWarning.Simple.doesShowDisableButton(0)`: its class isn't translated (it extends `ETFConfigWarning`) (traben/entity_texture_features/config/ETFConfigWarning.java:85)

### `ETFConfigWarnings`

- [ ] `ETFConfigWarnings.REGISTERED_WARNINGS`: a Map or Set keyed by a ETFConfigWarning (traben/entity_texture_features/config/ETFConfigWarnings.java:7)
- [ ] `ETFConfigWarnings.registerConfigWarning(1)`: `null` (Rust has no null: this needs an Option) (traben/entity_texture_features/config/ETFConfigWarnings.java:9)
- [ ] `ETFConfigWarnings.getRegisteredWarnings(0)`: a Map or Set keyed by a ETFConfigWarning (traben/entity_texture_features/config/ETFConfigWarnings.java:17)

### `ETFModMenu`: it implements `ModMenuApi` (com.terraformersmc.modmenu.api)

- [ ] `ETFModMenu.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (traben/entity_texture_features/config/ETFModMenu.java:11)

### `ETFConfigScreenMain`: it extends `TConfigScreenMain` (traben.tconfig.gui)

- [ ] `ETFConfigScreenMain.ETFConfigScreenMain(1)`: its class isn't translated (it extends `TConfigScreenMain` (traben.tconfig.gui)) (traben/entity_texture_features/config/screens/ETFConfigScreenMain.java:46)
- [ ] `ETFConfigScreenMain.drawEntity12106(9)`: its class isn't translated (it extends `TConfigScreenMain` (traben.tconfig.gui)) (traben/entity_texture_features/config/screens/ETFConfigScreenMain.java:70)
- [ ] `ETFConfigScreenMain.init(0)`: its class isn't translated (it extends `TConfigScreenMain` (traben.tconfig.gui)) (traben/entity_texture_features/config/screens/ETFConfigScreenMain.java:74)
- [ ] `ETFConfigScreenMain.extractRenderState(4)`: its class isn't translated (it extends `TConfigScreenMain` (traben.tconfig.gui)) (traben/entity_texture_features/config/screens/ETFConfigScreenMain.java:89)
- [ ] `ETFConfigScreenMain.renderEntitySample(3)`: its class isn't translated (it extends `TConfigScreenMain` (traben.tconfig.gui)) (traben/entity_texture_features/config/screens/ETFConfigScreenMain.java:122)

### `ETFConfigScreenMain.LogoCreeperRenderer`: it uses Minecraft's `ModelPart`

- [ ] `ETFConfigScreenMain.LogoCreeperRenderer.LogoCreeperRenderer(0)`: its class isn't translated (it uses Minecraft's `ModelPart`) (traben/entity_texture_features/config/screens/ETFConfigScreenMain.java:159)

### `ETFConfigScreenWarnings`: it extends `TConfigScreen` (traben.tconfig.gui)

- [ ] `ETFConfigScreenWarnings.ETFConfigScreenWarnings(2)`: its class isn't translated (it extends `TConfigScreen` (traben.tconfig.gui)) (traben/entity_texture_features/config/screens/ETFConfigScreenWarnings.java:20)
- [ ] `ETFConfigScreenWarnings.getIgnoredWarnings(0)`: it uses ETF.warningConfigHandler, which isn't translated (it uses TConfigHandler, which isn't translated (it's generic)) (traben/entity_texture_features/config/screens/ETFConfigScreenWarnings.java:25)
- [ ] `ETFConfigScreenWarnings.onClose(0)`: its class isn't translated (it extends `TConfigScreen` (traben.tconfig.gui)) (traben/entity_texture_features/config/screens/ETFConfigScreenWarnings.java:29)
- [ ] `ETFConfigScreenWarnings.init(0)`: its class isn't translated (it extends `TConfigScreen` (traben.tconfig.gui)) (traben/entity_texture_features/config/screens/ETFConfigScreenWarnings.java:35)
- [ ] `ETFConfigScreenWarnings.extractRenderState(4)`: its class isn't translated (it extends `TConfigScreen` (traben.tconfig.gui)) (traben/entity_texture_features/config/screens/ETFConfigScreenWarnings.java:79)

### `ETFConfigScreenWarnings.WarningConfig`: it extends `NoGUI`


### `ETFConfigScreenSkinTool`: it extends `ETFScreenOldCompat`

- [ ] `ETFConfigScreenSkinTool.APPLY_OVERLAY`: it calls ETFUtils2.res, which isn't translated (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:26)
- [ ] `ETFConfigScreenSkinTool.REMOVE_OVERLAY`: it calls ETFUtils2.res, which isn't translated (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:27)
- [ ] `ETFConfigScreenSkinTool.WHOLE_FACE_OVERLAY`: it calls ETFUtils2.res, which isn't translated (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:28)
- [ ] `ETFConfigScreenSkinTool.SMALL_EYE_OVERLAY`: it calls ETFUtils2.res, which isn't translated (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:29)
- [ ] `ETFConfigScreenSkinTool.BOXES_OVERLAY`: it calls ETFUtils2.res, which isn't translated (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:30)
- [ ] `ETFConfigScreenSkinTool.ETFConfigScreenSkinTool(1)`: its class isn't translated (it extends `ETFScreenOldCompat`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:49)
- [x] `ETFConfigScreenSkinTool.getPixelColour(1)` (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:53)
- [ ] `ETFConfigScreenSkinTool.onExit(0)`: its class isn't translated (it extends `ETFScreenOldCompat`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:76)
- [ ] `ETFConfigScreenSkinTool.onClose(0)`: its class isn't translated (it extends `ETFScreenOldCompat`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:85)
- [ ] `ETFConfigScreenSkinTool.init(0)`: its class isn't translated (it extends `ETFScreenOldCompat`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:91)
- [ ] `ETFConfigScreenSkinTool.updateButtons(0)`: its class isn't translated (it extends `ETFScreenOldCompat`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:418)
- [ ] `ETFConfigScreenSkinTool.extractRenderState(4)`: its class isn't translated (it extends `ETFScreenOldCompat`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:511)
- [ ] `ETFConfigScreenSkinTool.applyExistingOverlayToSkin(1)`: its body couldn't be read: an assert (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:540)
- [ ] `ETFConfigScreenSkinTool.printPlayerSkinCopy(0)`: its class isn't translated (it extends `ETFScreenOldCompat`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:575)
- [ ] `ETFConfigScreenSkinTool.renderEntityInInventoryFollowsMouse(11)`: it uses Minecraft's `LivingEntity` (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:591)
- [ ] `ETFConfigScreenSkinTool.drawEntity(7)`: its class isn't translated (it extends `ETFScreenOldCompat`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:624)

### `ETFConfigScreenSkinTool.BlinkType`

- [x] `ETFConfigScreenSkinTool.BlinkType.get(1)` (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:638)
- [ ] `ETFConfigScreenSkinTool.BlinkType.getExampleOverlay(0)`: `this` used as a value (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:655)
- [ ] `ETFConfigScreenSkinTool.BlinkType.getTitle(0)`: it uses Minecraft's `Component` (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:663)
- [ ] `ETFConfigScreenSkinTool.BlinkType.getBlinkPixelColour(0)`: `this` used as a value (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:680)
- [ ] `ETFConfigScreenSkinTool.BlinkType.getMaxEyePixelHeight(0)`: `this` used as a value (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:697)
- [ ] `ETFConfigScreenSkinTool.BlinkType.next(0)`: `this` used as a value (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:710)

### `ETFConfigScreenSkinTool.CoatStyle`

- [x] `ETFConfigScreenSkinTool.CoatStyle.get(1)` (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:740)
- [ ] `ETFConfigScreenSkinTool.CoatStyle.getTitle(0)`: it uses Minecraft's `Component` (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:763)
- [ ] `ETFConfigScreenSkinTool.CoatStyle.getCoatPixelColour(0)`: `this` used as a value (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:786)
- [ ] `ETFConfigScreenSkinTool.CoatStyle.next(0)`: `this` used as a value (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:809)

### `ETFConfigScreenSkinTool.ConfirmScreen`: it extends Minecraft's `Screen`

- [ ] `ETFConfigScreenSkinTool.ConfirmScreen.ConfirmScreen(2)`: its class isn't translated (it extends Minecraft's `Screen`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:837)
- [ ] `ETFConfigScreenSkinTool.ConfirmScreen.onClose(0)`: its class isn't translated (it extends Minecraft's `Screen`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:842)
- [ ] `ETFConfigScreenSkinTool.ConfirmScreen.shouldCloseOnEsc(0)`: its class isn't translated (it extends Minecraft's `Screen`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:857)
- [ ] `ETFConfigScreenSkinTool.ConfirmScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:861)
- [ ] `ETFConfigScreenSkinTool.ConfirmScreen.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:872)

### `ETFConfigScreenSkinTool.NoseType`: its constants carry values

- [ ] `ETFConfigScreenSkinTool.NoseType.NoseType(2)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:922)
- [ ] `ETFConfigScreenSkinTool.NoseType.getByColorId(1)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:927)
- [ ] `ETFConfigScreenSkinTool.NoseType.appliesTextureOverlay(0)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:937)
- [ ] `ETFConfigScreenSkinTool.NoseType.getButtonText(0)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:941)
- [ ] `ETFConfigScreenSkinTool.NoseType.next(0)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:945)
- [ ] `ETFConfigScreenSkinTool.NoseType.getNosePixelColour(0)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinTool.java:971)

### `ETFConfigScreenSkinToolOutcome`: it extends `ETFScreenOldCompat`

- [ ] `ETFConfigScreenSkinToolOutcome.ETFConfigScreenSkinToolOutcome(3)`: its class isn't translated (it extends `ETFScreenOldCompat`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinToolOutcome.java:45)
- [ ] `ETFConfigScreenSkinToolOutcome.uploadSkin(1)`: it calls `InetAddress` (java.net).getLocalHost (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinToolOutcome.java:51)
- [x] `ETFConfigScreenSkinToolOutcome.concat(1)` (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinToolOutcome.java:86)
- [ ] `ETFConfigScreenSkinToolOutcome.init(0)`: its body couldn't be read: an assert (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinToolOutcome.java:104)
- [ ] `ETFConfigScreenSkinToolOutcome.extractRenderState(4)`: its class isn't translated (it extends `ETFScreenOldCompat`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinToolOutcome.java:218)

### `ETFConfigScreenSkinToolPixelSelection`: it extends `ETFScreenOldCompat`

- [ ] `ETFConfigScreenSkinToolPixelSelection.ETFConfigScreenSkinToolPixelSelection(2)`: its class isn't translated (it extends `ETFScreenOldCompat`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinToolPixelSelection.java:27)
- [ ] `ETFConfigScreenSkinToolPixelSelection.init(0)`: its class isn't translated (it extends `ETFScreenOldCompat`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinToolPixelSelection.java:39)
- [ ] `ETFConfigScreenSkinToolPixelSelection.getButtonPixels(3)`: its body couldn't be read: an anonymous class (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinToolPixelSelection.java:88)
- [ ] `ETFConfigScreenSkinToolPixelSelection.applyCurrentSelectedPixels(0)`: its class isn't translated (it extends `ETFScreenOldCompat`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinToolPixelSelection.java:112)
- [ ] `ETFConfigScreenSkinToolPixelSelection.extractRenderState(4)`: its class isn't translated (it extends `ETFScreenOldCompat`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinToolPixelSelection.java:127)
- [ ] `ETFConfigScreenSkinToolPixelSelection.drawEntity(7)`: its class isn't translated (it extends `ETFScreenOldCompat`) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinToolPixelSelection.java:159)

### `ETFConfigScreenSkinToolPixelSelection.SelectionMode`: its constants carry values

- [ ] `ETFConfigScreenSkinToolPixelSelection.SelectionMode.SelectionMode(2)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/config/screens/skin/ETFConfigScreenSkinToolPixelSelection.java:172)

### `ETFScreenOldCompat`: it extends `TConfigScreen` (traben.tconfig.gui)

- [ ] `ETFScreenOldCompat.ETFScreenOldCompat(3)`: its class isn't translated (it extends `TConfigScreen` (traben.tconfig.gui)) (traben/entity_texture_features/config/screens/skin/ETFScreenOldCompat.java:21)
- [ ] `ETFScreenOldCompat.renderGUITexture(6)`: it calls Minecraft's GuiGraphics.blit (no Horizon helper for it yet) (traben/entity_texture_features/config/screens/skin/ETFScreenOldCompat.java:25)
- [ ] `ETFScreenOldCompat.booleanAsOnOff(1)`: it calls Minecraft's `CommonComponents`.optionStatus (traben/entity_texture_features/config/screens/skin/ETFScreenOldCompat.java:31)
- [ ] `ETFScreenOldCompat.getETFButton(6)`: its class isn't translated (it extends `TConfigScreen` (traben.tconfig.gui)) (traben/entity_texture_features/config/screens/skin/ETFScreenOldCompat.java:35)
- [ ] `ETFScreenOldCompat.getETFButton(7)`: its class isn't translated (it extends `TConfigScreen` (traben.tconfig.gui)) (traben/entity_texture_features/config/screens/skin/ETFScreenOldCompat.java:39)

### `ETF`

- [x] `ETF.MOD_ID` (traben/entity_texture_features/ETF.java:30)
- [x] `ETF.LOGGER` (traben/entity_texture_features/ETF.java:31)
- [x] `ETF.EMISSIVE_FEATURE_LIGHT_VALUE` (traben/entity_texture_features/ETF.java:32)
- [ ] `ETF.warningConfigHandler`: it uses TConfigHandler, which isn't translated (it's generic) (traben/entity_texture_features/ETF.java:33)
- [x] `ETF.IRIS_DETECTED` (traben/entity_texture_features/ETF.java:34)
- [x] `ETF.FABRIC_API` (traben/entity_texture_features/ETF.java:35)
- [x] `ETF.EYES_FEATURE_LIGHT_VALUE` (traben/entity_texture_features/ETF.java:36)
- [ ] `ETF.etfRenderStateConstructor`: it uses ETFRenderStateInit, which isn't translated (it's an interface) (traben/entity_texture_features/ETF.java:37)
- [x] `ETF.SKIN_LAYERS_DETECTED` (traben/entity_texture_features/ETF.java:38)
- [ ] `ETF.configHandlers`: it uses TConfigHandler, which isn't translated (it's generic) (traben/entity_texture_features/ETF.java:39)
- [ ] `ETF.CONFIG`: it uses TConfigHandler, which isn't translated (it's generic) (traben/entity_texture_features/ETF.java:40)
- [ ] `ETF.config(0)`: it uses TConfigHandler, which isn't translated (it's generic) (traben/entity_texture_features/ETF.java:42)
- [ ] `ETF.start(0)`: it uses ETF.CONFIG, which isn't translated (it uses TConfigHandler, which isn't translated (it's generic)) (traben/entity_texture_features/ETF.java:50)
- [ ] `ETF.randomQuip(0)`: it creates `Random` (java.util) (traben/entity_texture_features/ETF.java:94)
- [ ] `ETF.registerConfigHandler(1)`: it uses TConfigHandler, which isn't translated (it's generic) (traben/entity_texture_features/ETF.java:125)
- [ ] `ETF.getConfigScreen(1)`: it uses Minecraft's `Screen` (traben/entity_texture_features/ETF.java:133)
- [ ] `ETF.getConfigScreen(2)`: it uses Minecraft's `Screen` (traben/entity_texture_features/ETF.java:141)
- [ ] `ETF.getBiomeString(2)`: it uses Minecraft's `Level` (traben/entity_texture_features/ETF.java:145)
- [ ] `ETF.getTextFromTranslation(1)`: a translated text with a computed key (traben/entity_texture_features/ETF.java:150)
- [x] `ETF.getConfigDirectory(0)` (traben/entity_texture_features/ETF.java:155)
- [x] `ETF.isThisModLoaded(1)` (traben/entity_texture_features/ETF.java:159)
- [ ] `ETF.modsLoaded(0)`: it calls Fabric's FabricLoader.getAllMods (traben/entity_texture_features/ETF.java:163)
- [x] `ETF.isForge(0)` (traben/entity_texture_features/ETF.java:167)
- [x] `ETF.isFabric(0)` (traben/entity_texture_features/ETF.java:171)

### `ETFApi`

- [x] `ETFApi.ETFApiVersion` (traben/entity_texture_features/ETFApi.java:31)
- [ ] `ETFApi.ETF_GENERIC_UUID`: it uses `UUID` (java.util) (traben/entity_texture_features/ETFApi.java:32)
- [x] `ETFApi.ETF_SPAWNER_MARKER` (traben/entity_texture_features/ETFApi.java:33)
- [ ] `ETFApi.getETFConfigObject`: it uses ETFConfig, which isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/ETFApi.java:35)
- [ ] `ETFApi.getETFConfigObject(0)`: it uses ETFConfig, which isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/ETFApi.java:37)
- [ ] `ETFApi.setETFConfigObject(1)`: it uses ETFConfig, which isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/ETFApi.java:41)
- [ ] `ETFApi.getCopyOfETFConfigObject(0)`: it uses ETFConfig, which isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/ETFApi.java:50)
- [ ] `ETFApi.getDefaultETFConfigObject(0)`: it uses ETFConfig, which isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_texture_features/ETFApi.java:54)
- [ ] `ETFApi.getBlockEntityTypeToTranslationKey(1)`: it uses Minecraft's `BlockEntityType` (traben/entity_texture_features/ETFApi.java:58)
- [ ] `ETFApi.saveETFConfigChangesAndResetETF(0)`: it calls ETF.config, which isn't translated (traben/entity_texture_features/ETFApi.java:62)
- [ ] `ETFApi.resetETF(0)`: it calls ETFManager.resetInstance, which isn't translated (traben/entity_texture_features/ETFApi.java:67)
- [ ] `ETFApi.getUUIDForBlockEntity(1)`: it uses `UUID` (java.util) (traben/entity_texture_features/ETFApi.java:71)
- [ ] `ETFApi.stateOfEntityOrEntityState(1)`: it uses ETFEntityRenderState, which isn't translated (it's an interface) (traben/entity_texture_features/ETFApi.java:79)
- [ ] `ETFApi.getCurrentETFVariantTextureOfEntity(2)`: it uses Minecraft's `Entity` (traben/entity_texture_features/ETFApi.java:89)
- [ ] `ETFApi.getCurrentETFVariantTextureOfBlockEntity(2)`: it uses Minecraft's `BlockEntity` (traben/entity_texture_features/ETFApi.java:106)
- [ ] `ETFApi.getCurrentETFVariantTextureOfBlockEntity(3)`: it uses Minecraft's `BlockEntity` (traben/entity_texture_features/ETFApi.java:115)
- [ ] `ETFApi.getCurrentETFVariantTextureOfBlockEntityInternal(2)`: it uses ETFEntity, which isn't translated (it's an interface) (traben/entity_texture_features/ETFApi.java:126)
- [ ] `ETFApi.getCurrentETFVariantTextureOfEntity(3)`: it uses Minecraft's `BlockEntity` (traben/entity_texture_features/ETFApi.java:139)
- [ ] `ETFApi.getCurrentETFEmissiveTextureOfEntityOrNull(2)`: it uses Minecraft's `Entity` (traben/entity_texture_features/ETFApi.java:145)
- [ ] `ETFApi.getCurrentETFEmissiveTextureOfBlockEntityOrNull(2)`: it uses Minecraft's `BlockEntity` (traben/entity_texture_features/ETFApi.java:157)
- [ ] `ETFApi.getVariantSupplierOrNull(3)`: it uses ETFVariantSuffixProvider, which isn't translated (it's an interface) (traben/entity_texture_features/ETFApi.java:169)
- [ ] `ETFApi.getLastMatchingRuleOfEntity(1)`: it uses Minecraft's `Entity` (traben/entity_texture_features/ETFApi.java:176)
- [ ] `ETFApi.getLastMatchingRuleOfBlockEntity(1)`: it uses Minecraft's `BlockEntity` (traben/entity_texture_features/ETFApi.java:181)
- [ ] `ETFApi.registerCustomRandomPropertyFactory(2)`: it uses RandomPropertyFactory, which isn't translated (it's an interface) (traben/entity_texture_features/ETFApi.java:186)
- [ ] `ETFApi.registerCustomETFConfigWarning(2)`: `null` (Rust has no null: this needs an Option) (traben/entity_texture_features/ETFApi.java:193)

### `ETFApi.ETFVariantSuffixProvider`: it's an interface

- [ ] `ETFApi.ETFVariantSuffixProvider.getVariantProviderOrNull(3)`: it uses ETFVariantSuffixProvider, which isn't translated (it's an interface) (traben/entity_texture_features/ETFApi.java:201)
- [ ] `ETFApi.ETFVariantSuffixProvider.entityCanUpdate(1)`: it has no body (traben/entity_texture_features/ETFApi.java:226)
- [ ] `ETFApi.ETFVariantSuffixProvider.getAllSuffixes(0)`: it has no body (traben/entity_texture_features/ETFApi.java:228)
- [ ] `ETFApi.ETFVariantSuffixProvider.size(0)`: it has no body (traben/entity_texture_features/ETFApi.java:230)
- [ ] `ETFApi.ETFVariantSuffixProvider.getSuffixForEntity(1)`: its class isn't translated (it's an interface) (traben/entity_texture_features/ETFApi.java:232)
- [ ] `ETFApi.ETFVariantSuffixProvider.getSuffixForEntityState(1)`: its class isn't translated (it's an interface) (traben/entity_texture_features/ETFApi.java:236)
- [ ] `ETFApi.ETFVariantSuffixProvider.getSuffixForBlockEntity(1)`: its class isn't translated (it's an interface) (traben/entity_texture_features/ETFApi.java:240)
- [ ] `ETFApi.ETFVariantSuffixProvider.getSuffixForETFEntity(1)`: it has no body (traben/entity_texture_features/ETFApi.java:244)
- [ ] `ETFApi.ETFVariantSuffixProvider.setRandomSupplier(1)`: its class isn't translated (it's an interface) (traben/entity_texture_features/ETFApi.java:246)

### `ETFApi.ETFVariantSuffixProvider.EntityRandomSeedFunction`: it's an interface

- [ ] `ETFApi.ETFVariantSuffixProvider.EntityRandomSeedFunction.toInt(1)`: it has no body (traben/entity_texture_features/ETFApi.java:252)

### `ETFException`: it extends `RuntimeException`

- [ ] `ETFException.ETFException(1)`: its class isn't translated (it extends `RuntimeException`) (traben/entity_texture_features/ETFException.java:8)
- [ ] `ETFException.amendMessage(1)`: it uses ETFEntityRenderState, which isn't translated (it's an interface) (traben/entity_texture_features/ETFException.java:12)

### `ETFInit`: it implements Fabric's `ClientModInitializer`

- [ ] `ETFInit.onInitializeClient(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (traben/entity_texture_features/ETFInit.java:9)

### `ETFManager`: it uses ETFLruCache, which isn't translated (it's generic)

- [ ] `ETFManager.ETF_ERROR_TEXTURE`: it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/ETFManager.java:34)
- [ ] `ETFManager.instance`: it uses ETFManager, which isn't translated (it uses ETFLruCache, which isn't translated (it's generic)) (traben/entity_texture_features/features/ETFManager.java:35)
- [ ] `ETFManager.ETFManager(0)`: its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)) (traben/entity_texture_features/features/ETFManager.java:51)
- [ ] `ETFManager.getInstance(0)`: it uses ETFManager, which isn't translated (it uses ETFLruCache, which isn't translated (it's generic)) (traben/entity_texture_features/features/ETFManager.java:94)
- [ ] `ETFManager.resetInstance(0)`: it calls ETFState.clear, which isn't translated (traben/entity_texture_features/features/ETFManager.java:102)
- [ ] `ETFManager.getErrorETFTexture(0)`: it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/ETFManager.java:108)
- [ ] `ETFManager.getEmissiveMode(0)`: it calls ETF.config, which isn't translated (traben/entity_texture_features/features/ETFManager.java:114)
- [ ] `ETFManager.getGeneralPrintout(0)`: its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)) (traben/entity_texture_features/features/ETFManager.java:126)
- [ ] `ETFManager.doTheBigBoyPrintoutKronk(0)`: its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)) (traben/entity_texture_features/features/ETFManager.java:133)
- [ ] `ETFManager.grabSpecialProperties(2)`: its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)) (traben/entity_texture_features/features/ETFManager.java:192)
- [ ] `ETFManager.markEntityForDebugPrint(1)`: its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)) (traben/entity_texture_features/features/ETFManager.java:235)
- [ ] `ETFManager.getETFTextureNoVariation(1)`: its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)) (traben/entity_texture_features/features/ETFManager.java:241)
- [ ] `ETFManager.getETFTextureVariant(2)`: its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)) (traben/entity_texture_features/features/ETFManager.java:246)
- [ ] `ETFManager.getOrCreateETFTexture(1)`: its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)) (traben/entity_texture_features/features/ETFManager.java:273)
- [ ] `ETFManager.getPlayerTexture(2)`: its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)) (traben/entity_texture_features/features/ETFManager.java:289)
- [ ] `ETFManager.getPlayerTexture(2)`: its class isn't translated (it uses ETFLruCache, which isn't translated (it's generic)) (traben/entity_texture_features/features/ETFManager.java:294)

### `ETFManager.TextureSource`


### `ETFRenderContext`


### `ETFPlayerEntity`: it's an interface

- [ ] `ETFPlayerEntity.etf$getEntity(0)`: it has no body (traben/entity_texture_features/features/player/ETFPlayerEntity.java:11)
- [ ] `ETFPlayerEntity.etf$isTeammate(1)`: it has no body (traben/entity_texture_features/features/player/ETFPlayerEntity.java:13)
- [ ] `ETFPlayerEntity.etf$getInventory(0)`: it has no body (traben/entity_texture_features/features/player/ETFPlayerEntity.java:15)
- [ ] `ETFPlayerEntity.etf$isPartVisible(1)`: it has no body (traben/entity_texture_features/features/player/ETFPlayerEntity.java:17)
- [ ] `ETFPlayerEntity.etf$getName(0)`: it has no body (traben/entity_texture_features/features/player/ETFPlayerEntity.java:20)
- [ ] `ETFPlayerEntity.etf$getUuidAsString(0)`: it has no body (traben/entity_texture_features/features/player/ETFPlayerEntity.java:22)

### `ETFPlayerFeatureRenderer`: it's generic

- [ ] `ETFPlayerFeatureRenderer.villagerNose`: it uses Minecraft's `ModelPart` (traben/entity_texture_features/features/player/ETFPlayerFeatureRenderer.java:30)
- [ ] `ETFPlayerFeatureRenderer.textureNose`: it uses Minecraft's `ModelPart` (traben/entity_texture_features/features/player/ETFPlayerFeatureRenderer.java:31)
- [ ] `ETFPlayerFeatureRenderer.jacket`: it uses Minecraft's `ModelPart` (traben/entity_texture_features/features/player/ETFPlayerFeatureRenderer.java:32)
- [ ] `ETFPlayerFeatureRenderer.fatJacket`: it uses Minecraft's `ModelPart` (traben/entity_texture_features/features/player/ETFPlayerFeatureRenderer.java:33)
- [ ] `ETFPlayerFeatureRenderer.VILLAGER_TEXTURE`: it calls ETFUtils2.res, which isn't translated (traben/entity_texture_features/features/player/ETFPlayerFeatureRenderer.java:34)
- [ ] `ETFPlayerFeatureRenderer.ETFPlayerFeatureRenderer(1)`: its class isn't translated (it's generic) (traben/entity_texture_features/features/player/ETFPlayerFeatureRenderer.java:37)
- [ ] `ETFPlayerFeatureRenderer.getModelData(1)`: it uses Minecraft's `MeshDefinition` (traben/entity_texture_features/features/player/ETFPlayerFeatureRenderer.java:42)
- [ ] `ETFPlayerFeatureRenderer.renderSkullFeatures(6)`: it uses Minecraft's `SubmitNodeCollector` (traben/entity_texture_features/features/player/ETFPlayerFeatureRenderer.java:62)
- [ ] `ETFPlayerFeatureRenderer.renderEnchanted(5)`: it uses Minecraft's `SubmitNodeCollector` (traben/entity_texture_features/features/player/ETFPlayerFeatureRenderer.java:82)
- [ ] `ETFPlayerFeatureRenderer.renderNose(4)`: it uses Minecraft's `SubmitNodeCollector` (traben/entity_texture_features/features/player/ETFPlayerFeatureRenderer.java:88)
- [ ] `ETFPlayerFeatureRenderer.submit(6)`: its class isn't translated (it's generic) (traben/entity_texture_features/features/player/ETFPlayerFeatureRenderer.java:114)
- [ ] `ETFPlayerFeatureRenderer.renderFeatures(6)`: its class isn't translated (it's generic) (traben/entity_texture_features/features/player/ETFPlayerFeatureRenderer.java:126)
- [ ] `ETFPlayerFeatureRenderer.renderCoat(6)`: its class isn't translated (it's generic) (traben/entity_texture_features/features/player/ETFPlayerFeatureRenderer.java:151)

### `ETFPlayerSkinHolder`: it's an interface

- [ ] `ETFPlayerSkinHolder.etf$getETFPlayerTexture(0)`: it has no body (traben/entity_texture_features/features/player/ETFPlayerSkinHolder.java:6)

### `ETFPlayerTexture`: it uses NoseType, which isn't translated (its constants carry values)

- [x] `ETFPlayerTexture.SKIN_NAMESPACE` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:37)
- [ ] `ETFPlayerTexture.clientPlayerOriginalSkinImageForTool`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:38)
- [x] `ETFPlayerTexture.remappingETFSkin` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:39)
- [ ] `ETFPlayerTexture.TRY_AGAIN_LATER`: it uses ETFException, which isn't translated (it extends `RuntimeException`) (traben/entity_texture_features/features/player/ETFPlayerTexture.java:65)
- [ ] `ETFPlayerTexture.downloader`: it uses Minecraft's `SkinTextureDownloader` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:66)
- [ ] `ETFPlayerTexture.getOriginal(0)`: its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)) (traben/entity_texture_features/features/player/ETFPlayerTexture.java:71)
- [ ] `ETFPlayerTexture.ETFPlayerTexture(2)`: its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)) (traben/entity_texture_features/features/player/ETFPlayerTexture.java:75)
- [ ] `ETFPlayerTexture.etf$profileSkinTextureLocation(1)`: it uses Minecraft's `GameProfile` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:93)
- [ ] `ETFPlayerTexture.etf$tryLoadOriginalSkinFromRendererIfProfileTextureIdDiffers(1)`: its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)) (traben/entity_texture_features/features/player/ETFPlayerTexture.java:103)
- [ ] `ETFPlayerTexture.getSkinOfPlayer(2)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:132)
- [ ] `ETFPlayerTexture.ETFPlayerTexture(2)`: its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)) (traben/entity_texture_features/features/player/ETFPlayerTexture.java:198)
- [ ] `ETFPlayerTexture.ETFPlayerTexture(0)`: its body couldn't be read: an assert (traben/entity_texture_features/features/player/ETFPlayerTexture.java:204)
- [ ] `ETFPlayerTexture.returnMatchPixels(3)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:220)
- [ ] `ETFPlayerTexture.returnMatchPixels(4)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:225)
- [ ] `ETFPlayerTexture.returnNullIfEmptyImage(1)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:270)
- [ ] `ETFPlayerTexture.getSkinPixelBounds(1)`: a switch expression giving a int[] (traben/entity_texture_features/features/player/ETFPlayerTexture.java:286)
- [ ] `ETFPlayerTexture.returnOptimizedBlinkFace(3)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:318)
- [ ] `ETFPlayerTexture.returnOptimizedBlinkFace(4)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:322)
- [ ] `ETFPlayerTexture.forceSolidLowerSkin(1)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:333)
- [ ] `ETFPlayerTexture.getCoatTexture(3)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:349)
- [ ] `ETFPlayerTexture.copyToPixels(5)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:363)
- [ ] `ETFPlayerTexture.copyToPixels(8)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:367)
- [ ] `ETFPlayerTexture.deletePixels(5)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:378)
- [x] `ETFPlayerTexture.getSkinPixelColourToNumber(1)` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:386)
- [x] `ETFPlayerTexture.getSkinNumberToPixelColour(1)` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:401)
- [ ] `ETFPlayerTexture.stripAlphaInclusive(5)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/features/player/ETFPlayerTexture.java:416)
- [ ] `ETFPlayerTexture.isCorrectObjectForThisSkin(1)`: its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)) (traben/entity_texture_features/features/player/ETFPlayerTexture.java:424)
- [ ] `ETFPlayerTexture.getBaseTextureIdentifierOrNullForVanilla(1)`: its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)) (traben/entity_texture_features/features/player/ETFPlayerTexture.java:428)
- [ ] `ETFPlayerTexture.getBaseTextureIdentifierOrNullForVanilla(1)`: its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)) (traben/entity_texture_features/features/player/ETFPlayerTexture.java:433)
- [ ] `ETFPlayerTexture.getBaseHeadTextureIdentifierOrNullForVanilla(0)`: its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)) (traben/entity_texture_features/features/player/ETFPlayerTexture.java:439)
- [ ] `ETFPlayerTexture.getBaseTextureEmissiveIdentifierOrNullForNone(0)`: its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)) (traben/entity_texture_features/features/player/ETFPlayerTexture.java:444)
- [ ] `ETFPlayerTexture.canUseFeaturesForThisPlayer(0)`: its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)) (traben/entity_texture_features/features/player/ETFPlayerTexture.java:451)
- [ ] `ETFPlayerTexture.skinFailed(1)`: its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)) (traben/entity_texture_features/features/player/ETFPlayerTexture.java:461)
- [ ] `ETFPlayerTexture.skinFailed(2)`: its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)) (traben/entity_texture_features/features/player/ETFPlayerTexture.java:465)
- [ ] `ETFPlayerTexture.checkTexture(1)`: its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)) (traben/entity_texture_features/features/player/ETFPlayerTexture.java:473)
- [ ] `ETFPlayerTexture.changeSkinToThisForTool(1)`: its class isn't translated (it uses NoseType, which isn't translated (its constants carry values)) (traben/entity_texture_features/features/player/ETFPlayerTexture.java:861)

### `PropertiesRandomProvider`: it implements `ETFVariantSuffixProvider`

- [ ] `PropertiesRandomProvider.PropertiesRandomProvider(2)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:35)
- [ ] `PropertiesRandomProvider.of(3)`: it uses PropertiesRandomProvider, which isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:40)
- [ ] `PropertiesRandomProvider.getAllValidPropertyObjects(3)`: it uses RandomPropertyRule, which isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))) (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:86)
- [ ] `PropertiesRandomProvider.getSeedOffset(2)`: parses something that isn't a string (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:113)
- [ ] `PropertiesRandomProvider.validateRuleNumbers(2)`: it calls ETFUtils2.logWarn, which isn't translated (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:125)
- [x] `PropertiesRandomProvider.getCaseNumbers(1)` (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:151)
- [ ] `PropertiesRandomProvider.getSuffixes(3)`: it calls `Arrays` (java.util).toString (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:171)
- [ ] `PropertiesRandomProvider.getWeights(2)`: it calls SimpleIntegerArrayProperty.getGenericIntegerSplitWithRanges, which isn't translated (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:185)
- [ ] `PropertiesRandomProvider.setOnMeetsRuleHook(1)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:190)
- [ ] `PropertiesRandomProvider.getPackName(0)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:196)
- [ ] `PropertiesRandomProvider.isHigherPackThan(1)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:201)
- [ ] `PropertiesRandomProvider.entityCanUpdate(1)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:205)
- [ ] `PropertiesRandomProvider.getAllSuffixes(0)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:210)
- [ ] `PropertiesRandomProvider.size(0)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:221)
- [ ] `PropertiesRandomProvider.getSuffixForETFEntity(1)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:226)
- [ ] `PropertiesRandomProvider.testEntityAgainstRules(2)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/PropertiesRandomProvider.java:249)

### `AngryProperty`: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `AngryProperty.AngryProperty(2)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/AngryProperty.java:18)
- [ ] `AngryProperty.getPropertyOrNull(2)`: it uses AngryProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/AngryProperty.java:22)
- [ ] `AngryProperty.getValueFromEntity(1)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/AngryProperty.java:30)
- [ ] `AngryProperty.getPropertyIds(0)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/AngryProperty.java:63)

### `BiomeTagProperty`: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)

- [ ] `BiomeTagProperty.BiomeTagProperty(2)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BiomeTagProperty.java:24)
- [ ] `BiomeTagProperty.getPropertyOrNull(2)`: it uses BiomeTagProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BiomeTagProperty.java:30)
- [ ] `BiomeTagProperty.testEntityInternal(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BiomeTagProperty.java:38)
- [ ] `BiomeTagProperty.fail(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BiomeTagProperty.java:65)
- [ ] `BiomeTagProperty.getPropertyIds(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BiomeTagProperty.java:73)
- [ ] `BiomeTagProperty.getPrintableRuleInfo(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BiomeTagProperty.java:79)

### `BlockAboveProperty`: it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)

- [ ] `BlockAboveProperty.BlockAboveProperty(3)`: its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockAboveProperty.java:15)
- [ ] `BlockAboveProperty.getPropertyOrNull(2)`: it uses BlocksProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockAboveProperty.java:19)
- [ ] `BlockAboveProperty.getTestingBlocks(1)`: its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockAboveProperty.java:27)
- [ ] `BlockAboveProperty.getPropertyIds(0)`: its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockAboveProperty.java:54)

### `BlockAboveSolidProperty`: it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)

- [ ] `BlockAboveSolidProperty.BlockAboveSolidProperty(3)`: its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockAboveSolidProperty.java:15)
- [ ] `BlockAboveSolidProperty.getPropertyOrNull(2)`: it uses BlocksProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockAboveSolidProperty.java:19)
- [ ] `BlockAboveSolidProperty.getTestingBlocks(1)`: its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockAboveSolidProperty.java:27)
- [ ] `BlockAboveSolidProperty.getPropertyIds(0)`: its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockAboveSolidProperty.java:58)

### `BlockBelowProperty`: it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)

- [ ] `BlockBelowProperty.BlockBelowProperty(3)`: its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockBelowProperty.java:15)
- [ ] `BlockBelowProperty.getPropertyOrNull(2)`: it uses BlocksProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockBelowProperty.java:19)
- [ ] `BlockBelowProperty.getTestingBlocks(1)`: its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockBelowProperty.java:27)
- [ ] `BlockBelowProperty.getPropertyIds(0)`: its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockBelowProperty.java:50)

### `BlockBelowSolidProperty`: it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)

- [ ] `BlockBelowSolidProperty.BlockBelowSolidProperty(3)`: its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockBelowSolidProperty.java:15)
- [ ] `BlockBelowSolidProperty.getPropertyOrNull(2)`: it uses BlocksProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockBelowSolidProperty.java:19)
- [ ] `BlockBelowSolidProperty.getTestingBlocks(1)`: its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockBelowSolidProperty.java:27)
- [ ] `BlockBelowSolidProperty.getPropertyIds(0)`: its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockBelowSolidProperty.java:50)

### `BlockSpawnedProperty`: it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)

- [ ] `BlockSpawnedProperty.BlockSpawnedProperty(3)`: its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockSpawnedProperty.java:9)
- [ ] `BlockSpawnedProperty.getPropertyOrNull(2)`: it uses BlocksProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockSpawnedProperty.java:13)
- [ ] `BlockSpawnedProperty.getPropertyIds(0)`: its class isn't translated (it extends `BlocksProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/BlockSpawnedProperty.java:21)

### `ChargedCreeperProperty`: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `ChargedCreeperProperty.ChargedCreeperProperty(2)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ChargedCreeperProperty.java:12)
- [ ] `ChargedCreeperProperty.getPropertyOrNull(2)`: it uses ChargedCreeperProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ChargedCreeperProperty.java:16)
- [ ] `ChargedCreeperProperty.getValueFromEntity(1)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ChargedCreeperProperty.java:24)
- [ ] `ChargedCreeperProperty.getPropertyIds(0)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ChargedCreeperProperty.java:30)

### `ClientPlayerProperty`: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `ClientPlayerProperty.ClientPlayerProperty(2)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ClientPlayerProperty.java:13)
- [ ] `ClientPlayerProperty.getPropertyOrNull(2)`: it uses ClientPlayerProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ClientPlayerProperty.java:17)
- [ ] `ClientPlayerProperty.getValueFromEntity(1)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ClientPlayerProperty.java:25)
- [ ] `ClientPlayerProperty.getPropertyIds(0)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ClientPlayerProperty.java:34)

### `CreativeProperty`: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `CreativeProperty.CreativeProperty(2)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/CreativeProperty.java:12)
- [ ] `CreativeProperty.getPropertyOrNull(2)`: it uses CreativeProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/CreativeProperty.java:16)
- [ ] `CreativeProperty.getValueFromEntity(1)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/CreativeProperty.java:24)
- [ ] `CreativeProperty.getPropertyIds(0)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/CreativeProperty.java:30)

### `DimensionProperty`: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `DimensionProperty.DimensionProperty(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/DimensionProperty.java:20)
- [ ] `DimensionProperty.getPropertyOrNull(2)`: it uses DimensionProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/DimensionProperty.java:25)
- [ ] `DimensionProperty.getValueFromEntity(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/DimensionProperty.java:33)
- [ ] `DimensionProperty.getPropertyIds(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/DimensionProperty.java:73)
- [ ] `DimensionProperty.shouldForceLowerCaseCheck(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/DimensionProperty.java:79)

### `DistanceToPlayerProperty`: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `DistanceToPlayerProperty.DistanceToPlayerProperty(2)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/DistanceToPlayerProperty.java:12)
- [ ] `DistanceToPlayerProperty.getPropertyOrNull(2)`: it uses DistanceToPlayerProperty, which isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/DistanceToPlayerProperty.java:16)
- [ ] `DistanceToPlayerProperty.getRangeValueFromEntity(1)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/DistanceToPlayerProperty.java:24)
- [ ] `DistanceToPlayerProperty.getPropertyIds(0)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/DistanceToPlayerProperty.java:29)

### `ClientGameModeProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `ClientGameModeProperty.ClientGameModeProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ClientGameModeProperty.java:12)
- [ ] `ClientGameModeProperty.getPropertyOrNull(2)`: it uses ClientGameModeProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ClientGameModeProperty.java:16)
- [ ] `ClientGameModeProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ClientGameModeProperty.java:24)
- [ ] `ClientGameModeProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ClientGameModeProperty.java:30)

### `DifficultyProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `DifficultyProperty.DifficultyProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/DifficultyProperty.java:10)
- [ ] `DifficultyProperty.getPropertyOrNull(2)`: it uses DifficultyProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/DifficultyProperty.java:14)
- [ ] `DifficultyProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/DifficultyProperty.java:22)
- [ ] `DifficultyProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/DifficultyProperty.java:28)

### `HardcoreProperty`: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `HardcoreProperty.HardcoreProperty(2)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/HardcoreProperty.java:11)
- [ ] `HardcoreProperty.getPropertyOrNull(2)`: it uses HardcoreProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/HardcoreProperty.java:15)
- [ ] `HardcoreProperty.getValueFromEntity(1)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/HardcoreProperty.java:23)
- [ ] `HardcoreProperty.getPropertyIds(0)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/HardcoreProperty.java:29)

### `HourProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `HourProperty.HourProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/HourProperty.java:11)
- [ ] `HourProperty.getPropertyOrNull(2)`: it uses HourProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/HourProperty.java:15)
- [ ] `HourProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/HourProperty.java:23)
- [ ] `HourProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/HourProperty.java:29)

### `LanguageProperty`: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `LanguageProperty.LanguageProperty(2)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/LanguageProperty.java:12)
- [ ] `LanguageProperty.getPropertyOrNull(2)`: it uses LanguageProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/LanguageProperty.java:16)
- [ ] `LanguageProperty.getValueFromEntity(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/LanguageProperty.java:24)
- [ ] `LanguageProperty.getPropertyIds(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/LanguageProperty.java:30)
- [ ] `LanguageProperty.shouldForceLowerCaseCheck(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/LanguageProperty.java:36)

### `MinecraftVersionProperty`: it extends `SemVerRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `MinecraftVersionProperty.MinecraftVersionProperty(2)`: its class isn't translated (it extends `SemVerRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MinecraftVersionProperty.java:16)
- [ ] `MinecraftVersionProperty.getPropertyOrNull(2)`: it uses MinecraftVersionProperty, which isn't translated (it extends `SemVerRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MinecraftVersionProperty.java:20)
- [ ] `MinecraftVersionProperty.getRangeValueFromEntity(1)`: its class isn't translated (it extends `SemVerRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MinecraftVersionProperty.java:28)
- [ ] `MinecraftVersionProperty.getPropertyIds(0)`: its class isn't translated (it extends `SemVerRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MinecraftVersionProperty.java:33)

### `MinuteProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `MinuteProperty.MinuteProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MinuteProperty.java:11)
- [ ] `MinuteProperty.getPropertyOrNull(2)`: it uses MinuteProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MinuteProperty.java:15)
- [ ] `MinuteProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MinuteProperty.java:23)
- [ ] `MinuteProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MinuteProperty.java:29)

### `ModLoadedProperty`: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `ModLoadedProperty.ModLoadedProperty(2)`: its body couldn't be read: an assert (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ModLoadedProperty.java:14)
- [ ] `ModLoadedProperty.getPropertyOrNull(2)`: it uses ModLoadedProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ModLoadedProperty.java:29)
- [ ] `ModLoadedProperty.getValueFromEntity(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ModLoadedProperty.java:37)
- [ ] `ModLoadedProperty.testEntity(2)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ModLoadedProperty.java:43)
- [ ] `ModLoadedProperty.getPropertyIds(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ModLoadedProperty.java:48)
- [ ] `ModLoadedProperty.shouldForceLowerCaseCheck(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ModLoadedProperty.java:54)

### `MonthDayProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `MonthDayProperty.MonthDayProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MonthDayProperty.java:11)
- [ ] `MonthDayProperty.getPropertyOrNull(2)`: it uses MonthDayProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MonthDayProperty.java:15)
- [ ] `MonthDayProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MonthDayProperty.java:23)
- [ ] `MonthDayProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MonthDayProperty.java:29)

### `MonthProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `MonthProperty.MonthProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MonthProperty.java:11)
- [ ] `MonthProperty.getPropertyOrNull(2)`: it uses MonthProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MonthProperty.java:15)
- [ ] `MonthProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MonthProperty.java:23)
- [ ] `MonthProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/MonthProperty.java:29)

### `NBTVehicleProperty`: it extends `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)

- [ ] `NBTVehicleProperty.NBTVehicleProperty(3)`: its class isn't translated (it extends `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/NBTVehicleProperty.java:12)
- [ ] `NBTVehicleProperty.getPropertyOrNull(2)`: it uses NBTProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/NBTVehicleProperty.java:16)
- [ ] `NBTVehicleProperty.getEntityNBT(1)`: its class isn't translated (it extends `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/NBTVehicleProperty.java:24)

### `RegionalDifficultyProperty`: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `RegionalDifficultyProperty.RegionalDifficultyProperty(2)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/RegionalDifficultyProperty.java:10)
- [ ] `RegionalDifficultyProperty.getPropertyOrNull(2)`: it uses RegionalDifficultyProperty, which isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/RegionalDifficultyProperty.java:14)
- [ ] `RegionalDifficultyProperty.getPropertyIds(0)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/RegionalDifficultyProperty.java:22)
- [ ] `RegionalDifficultyProperty.getRangeValueFromEntity(1)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/RegionalDifficultyProperty.java:28)

### `ResourcePackLoadedProperty`: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `ResourcePackLoadedProperty.ResourcePackLoadedProperty(2)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ResourcePackLoadedProperty.java:13)
- [ ] `ResourcePackLoadedProperty.getPropertyOrNull(2)`: it uses ResourcePackLoadedProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ResourcePackLoadedProperty.java:17)
- [ ] `ResourcePackLoadedProperty.getValueFromEntity(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ResourcePackLoadedProperty.java:25)
- [ ] `ResourcePackLoadedProperty.testEntity(2)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ResourcePackLoadedProperty.java:31)
- [ ] `ResourcePackLoadedProperty.getPropertyIds(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ResourcePackLoadedProperty.java:37)
- [ ] `ResourcePackLoadedProperty.shouldForceLowerCaseCheck(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ResourcePackLoadedProperty.java:43)

### `SecondProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `SecondProperty.SecondProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/SecondProperty.java:11)
- [ ] `SecondProperty.getPropertyOrNull(2)`: it uses SecondProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/SecondProperty.java:15)
- [ ] `SecondProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/SecondProperty.java:23)
- [ ] `SecondProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/SecondProperty.java:29)

### `ShadersEnabledProperty`: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `ShadersEnabledProperty.ShadersEnabledProperty(2)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ShadersEnabledProperty.java:12)
- [ ] `ShadersEnabledProperty.getPropertyOrNull(2)`: it uses ShadersEnabledProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ShadersEnabledProperty.java:16)
- [ ] `ShadersEnabledProperty.getValueFromEntity(1)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ShadersEnabledProperty.java:24)
- [ ] `ShadersEnabledProperty.getPropertyIds(0)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/ShadersEnabledProperty.java:30)

### `WeekDayProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `WeekDayProperty.WeekDayProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/WeekDayProperty.java:11)
- [ ] `WeekDayProperty.getPropertyOrNull(2)`: it uses WeekDayProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/WeekDayProperty.java:15)
- [ ] `WeekDayProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/WeekDayProperty.java:23)
- [ ] `WeekDayProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/WeekDayProperty.java:29)

### `YearDayProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `YearDayProperty.YearDayProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/YearDayProperty.java:11)
- [ ] `YearDayProperty.getPropertyOrNull(2)`: it uses YearDayProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/YearDayProperty.java:15)
- [ ] `YearDayProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/YearDayProperty.java:23)
- [ ] `YearDayProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/YearDayProperty.java:29)

### `YearProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `YearProperty.YearProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/YearProperty.java:11)
- [ ] `YearProperty.getPropertyOrNull(2)`: it uses YearProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/YearProperty.java:15)
- [ ] `YearProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/YearProperty.java:23)
- [ ] `YearProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/external/YearProperty.java:29)

### `ItemProperty`: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `ItemProperty.ItemProperty(2)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ItemProperty.java:12)
- [ ] `ItemProperty.getPropertyOrNull(2)`: it uses ItemProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ItemProperty.java:16)
- [ ] `ItemProperty.shouldForceLowerCaseCheck(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ItemProperty.java:24)
- [ ] `ItemProperty.testEntityInternal(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ItemProperty.java:29)
- [ ] `ItemProperty.getValueFromEntity(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ItemProperty.java:78)
- [ ] `ItemProperty.getPropertyIds(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ItemProperty.java:84)

### `JumpProperty`: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `JumpProperty.JumpProperty(2)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/JumpProperty.java:12)
- [ ] `JumpProperty.getPropertyOrNull(2)`: it uses JumpProperty, which isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/JumpProperty.java:16)
- [ ] `JumpProperty.getRangeValueFromEntity(1)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/JumpProperty.java:24)
- [ ] `JumpProperty.getPropertyIds(0)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/JumpProperty.java:29)

### `LightProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `LightProperty.LightProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/LightProperty.java:10)
- [ ] `LightProperty.getPropertyOrNull(2)`: it uses LightProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/LightProperty.java:14)
- [ ] `LightProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/LightProperty.java:22)
- [ ] `LightProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/LightProperty.java:28)

### `LlamaInventoryProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `LlamaInventoryProperty.LlamaInventoryProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/LlamaInventoryProperty.java:11)
- [ ] `LlamaInventoryProperty.getPropertyOrNull(2)`: it uses LlamaInventoryProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/LlamaInventoryProperty.java:15)
- [ ] `LlamaInventoryProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/LlamaInventoryProperty.java:23)
- [ ] `LlamaInventoryProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/LlamaInventoryProperty.java:29)

### `MaxHealthProperty`: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `MaxHealthProperty.MaxHealthProperty(2)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/MaxHealthProperty.java:12)
- [ ] `MaxHealthProperty.getPropertyOrNull(2)`: it uses MaxHealthProperty, which isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/MaxHealthProperty.java:16)
- [ ] `MaxHealthProperty.getRangeValueFromEntity(1)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/MaxHealthProperty.java:24)
- [ ] `MaxHealthProperty.getPropertyIds(0)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/MaxHealthProperty.java:29)

### `MovingProperty`: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `MovingProperty.MovingProperty(2)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/MovingProperty.java:11)
- [ ] `MovingProperty.getPropertyOrNull(2)`: it uses MovingProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/MovingProperty.java:15)
- [ ] `MovingProperty.getValueFromEntity(1)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/MovingProperty.java:23)
- [ ] `MovingProperty.getPropertyIds(0)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/MovingProperty.java:29)

### `NBTClientProperty`: it extends `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)

- [ ] `NBTClientProperty.NBTClientProperty(3)`: its class isn't translated (it extends `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/NBTClientProperty.java:12)
- [ ] `NBTClientProperty.getPropertyOrNull(2)`: it uses NBTProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/NBTClientProperty.java:16)
- [ ] `NBTClientProperty.getEntityNBT(1)`: its class isn't translated (it extends `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/NBTClientProperty.java:24)

### `PandaGeneProperty`: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `PandaGeneProperty.PandaGeneProperty(2)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/PandaGeneProperty.java:12)
- [ ] `PandaGeneProperty.getPropertyOrNull(2)`: it uses PandaGeneProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/PandaGeneProperty.java:16)
- [ ] `PandaGeneProperty.shouldForceLowerCaseCheck(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/PandaGeneProperty.java:24)
- [ ] `PandaGeneProperty.getValueFromEntity(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/PandaGeneProperty.java:29)
- [ ] `PandaGeneProperty.getPropertyIds(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/PandaGeneProperty.java:35)

### `PlayerCreatedProperty`: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `PlayerCreatedProperty.PlayerCreatedProperty(2)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/PlayerCreatedProperty.java:12)
- [ ] `PlayerCreatedProperty.getPropertyOrNull(2)`: it uses PlayerCreatedProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/PlayerCreatedProperty.java:16)
- [ ] `PlayerCreatedProperty.getValueFromEntity(1)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/PlayerCreatedProperty.java:24)
- [ ] `PlayerCreatedProperty.getPropertyIds(0)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/PlayerCreatedProperty.java:30)

### `ScreamingGoatProperty`: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `ScreamingGoatProperty.ScreamingGoatProperty(2)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ScreamingGoatProperty.java:12)
- [ ] `ScreamingGoatProperty.getPropertyOrNull(2)`: it uses ScreamingGoatProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ScreamingGoatProperty.java:16)
- [ ] `ScreamingGoatProperty.getValueFromEntity(1)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ScreamingGoatProperty.java:24)
- [ ] `ScreamingGoatProperty.getPropertyIds(0)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/ScreamingGoatProperty.java:30)

### `SpawnerProperty`: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `SpawnerProperty.SpawnerProperty(2)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/SpawnerProperty.java:11)
- [ ] `SpawnerProperty.getPropertyOrNull(2)`: it uses SpawnerProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/SpawnerProperty.java:15)
- [ ] `SpawnerProperty.getValueFromEntity(1)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/SpawnerProperty.java:23)
- [ ] `SpawnerProperty.getPropertyIds(0)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/SpawnerProperty.java:29)

### `SpeedProperty`: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `SpeedProperty.SpeedProperty(2)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/SpeedProperty.java:12)
- [ ] `SpeedProperty.getPropertyOrNull(2)`: it uses SpeedProperty, which isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/SpeedProperty.java:16)
- [ ] `SpeedProperty.getRangeValueFromEntity(1)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/SpeedProperty.java:24)
- [ ] `SpeedProperty.getPropertyIds(0)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/SpeedProperty.java:29)

### `TeammateProperty`: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `TeammateProperty.TeammateProperty(2)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TeammateProperty.java:13)
- [ ] `TeammateProperty.getPropertyOrNull(2)`: it uses TeammateProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TeammateProperty.java:17)
- [ ] `TeammateProperty.getValueFromEntity(1)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TeammateProperty.java:25)
- [ ] `TeammateProperty.getPropertyIds(0)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TeammateProperty.java:33)

### `TeamProperty`: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `TeamProperty.TeamProperty(2)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TeamProperty.java:11)
- [ ] `TeamProperty.getPropertyOrNull(2)`: it uses TeamProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TeamProperty.java:15)
- [ ] `TeamProperty.getValueFromEntity(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TeamProperty.java:23)
- [ ] `TeamProperty.getPropertyIds(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TeamProperty.java:29)
- [ ] `TeamProperty.shouldForceLowerCaseCheck(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TeamProperty.java:35)

### `TemperatureProperty`: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `TemperatureProperty.TemperatureProperty(2)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TemperatureProperty.java:14)
- [ ] `TemperatureProperty.getPropertyOrNull(2)`: it uses TemperatureProperty, which isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TemperatureProperty.java:18)
- [ ] `TemperatureProperty.getRangeValueFromEntity(1)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TemperatureProperty.java:26)
- [ ] `TemperatureProperty.getPropertyIds(0)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TemperatureProperty.java:41)

### `TextureRuleIndexProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `TextureRuleIndexProperty.TextureRuleIndexProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TextureRuleIndexProperty.java:11)
- [ ] `TextureRuleIndexProperty.getPropertyOrNull(2)`: it uses TextureRuleIndexProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TextureRuleIndexProperty.java:15)
- [ ] `TextureRuleIndexProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TextureRuleIndexProperty.java:23)
- [ ] `TextureRuleIndexProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TextureRuleIndexProperty.java:29)

### `TextureSuffixProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `TextureSuffixProperty.TextureSuffixProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TextureSuffixProperty.java:11)
- [ ] `TextureSuffixProperty.getPropertyOrNull(2)`: it uses TextureSuffixProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TextureSuffixProperty.java:15)
- [ ] `TextureSuffixProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TextureSuffixProperty.java:23)
- [ ] `TextureSuffixProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/TextureSuffixProperty.java:29)

### `VariantProperty`: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `VariantProperty.VariantProperty(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/VariantProperty.java:24)
- [ ] `VariantProperty.getPropertyOrNull(2)`: it uses VariantProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/VariantProperty.java:28)
- [ ] `VariantProperty.shouldForceLowerCaseCheck(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/VariantProperty.java:36)
- [ ] `VariantProperty.getValueFromEntity(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/VariantProperty.java:41)
- [ ] `VariantProperty.getPropertyIds(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/etf_properties/VariantProperty.java:82)

### `BooleanProperty`: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)

- [ ] `BooleanProperty.BooleanProperty(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/generic_properties/BooleanProperty.java:12)
- [ ] `BooleanProperty.getGenericBooleanThatCanNull(3)`: it calls ETFUtils2.logWarn, which isn't translated (traben/entity_texture_features/features/property_reading/properties/generic_properties/BooleanProperty.java:20)
- [ ] `BooleanProperty.testEntityInternal(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/generic_properties/BooleanProperty.java:40)
- [ ] `BooleanProperty.getValueFromEntity(1)`: it has no body (traben/entity_texture_features/features/property_reading/properties/generic_properties/BooleanProperty.java:46)
- [ ] `BooleanProperty.getPrintableRuleInfo(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/generic_properties/BooleanProperty.java:49)

### `FloatRangeFromStringArrayProperty`: it extends `NumberRangeFromStringArrayProperty`

- [ ] `FloatRangeFromStringArrayProperty.FloatRangeFromStringArrayProperty(1)`: its class isn't translated (it extends `NumberRangeFromStringArrayProperty`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/FloatRangeFromStringArrayProperty.java:8)
- [ ] `FloatRangeFromStringArrayProperty.getRangeTesterFromString(1)`: its class isn't translated (it extends `NumberRangeFromStringArrayProperty`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/FloatRangeFromStringArrayProperty.java:12)

### `LongRangeFromStringArrayProperty`: it extends `NumberRangeFromStringArrayProperty`

- [ ] `LongRangeFromStringArrayProperty.LongRangeFromStringArrayProperty(1)`: its class isn't translated (it extends `NumberRangeFromStringArrayProperty`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/LongRangeFromStringArrayProperty.java:7)
- [ ] `LongRangeFromStringArrayProperty.getRangeTesterFromString(1)`: its class isn't translated (it extends `NumberRangeFromStringArrayProperty`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/LongRangeFromStringArrayProperty.java:11)

### `NumberRangeFromStringArrayProperty`: it's generic

- [ ] `NumberRangeFromStringArrayProperty.NumberRangeFromStringArrayProperty(1)`: its class isn't translated (it's generic) (traben/entity_texture_features/features/property_reading/properties/generic_properties/NumberRangeFromStringArrayProperty.java:15)
- [ ] `NumberRangeFromStringArrayProperty.testEntityInternal(1)`: its class isn't translated (it's generic) (traben/entity_texture_features/features/property_reading/properties/generic_properties/NumberRangeFromStringArrayProperty.java:37)
- [ ] `NumberRangeFromStringArrayProperty.getRangeValueFromEntity(1)`: it has no body (traben/entity_texture_features/features/property_reading/properties/generic_properties/NumberRangeFromStringArrayProperty.java:59)
- [ ] `NumberRangeFromStringArrayProperty.getRangeTesterFromString(1)`: it has no body (traben/entity_texture_features/features/property_reading/properties/generic_properties/NumberRangeFromStringArrayProperty.java:62)
- [ ] `NumberRangeFromStringArrayProperty.getPropertyIds(0)`: it has no body (traben/entity_texture_features/features/property_reading/properties/generic_properties/NumberRangeFromStringArrayProperty.java:65)
- [ ] `NumberRangeFromStringArrayProperty.getPrintableRuleInfo(0)`: its class isn't translated (it's generic) (traben/entity_texture_features/features/property_reading/properties/generic_properties/NumberRangeFromStringArrayProperty.java:69)

### `NumberRangeFromStringArrayProperty.RangeTester`: it's an interface

- [ ] `NumberRangeFromStringArrayProperty.RangeTester.isValueWithinRangeOrEqual(1)`: it has no body (traben/entity_texture_features/features/property_reading/properties/generic_properties/NumberRangeFromStringArrayProperty.java:75)

### `SemVerRangeFromStringArrayProperty`: it extends `NumberRangeFromStringArrayProperty`

- [ ] `SemVerRangeFromStringArrayProperty.SemVerRangeFromStringArrayProperty(1)`: its class isn't translated (it extends `NumberRangeFromStringArrayProperty`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java:8)
- [ ] `SemVerRangeFromStringArrayProperty.getRangeTesterFromString(1)`: its class isn't translated (it extends `NumberRangeFromStringArrayProperty`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java:12)

### `SemVerRangeFromStringArrayProperty.SemVerNumber`: it extends `Number`

- [ ] `SemVerRangeFromStringArrayProperty.SemVerNumber.SemVerNumber(1)`: its class isn't translated (it extends `Number`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java:32)
- [x] `SemVerRangeFromStringArrayProperty.SemVerNumber.parse(1)` (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java:36)
- [ ] `SemVerRangeFromStringArrayProperty.SemVerNumber.sameAs(1)`: its class isn't translated (it extends `Number`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java:44)
- [ ] `SemVerRangeFromStringArrayProperty.SemVerNumber.betweenInclusive(2)`: its class isn't translated (it extends `Number`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java:58)
- [ ] `SemVerRangeFromStringArrayProperty.SemVerNumber.largerThanOrEqual(1)`: its class isn't translated (it extends `Number`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java:62)
- [ ] `SemVerRangeFromStringArrayProperty.SemVerNumber.smallerThanOrEqual(1)`: its class isn't translated (it extends `Number`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java:66)
- [ ] `SemVerRangeFromStringArrayProperty.SemVerNumber.largerThan(1)`: its class isn't translated (it extends `Number`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java:70)
- [ ] `SemVerRangeFromStringArrayProperty.SemVerNumber.smallerThan(1)`: its class isn't translated (it extends `Number`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java:84)
- [ ] `SemVerRangeFromStringArrayProperty.SemVerNumber.intValue(0)`: its class isn't translated (it extends `Number`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java:98)
- [ ] `SemVerRangeFromStringArrayProperty.SemVerNumber.longValue(0)`: its class isn't translated (it extends `Number`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java:103)
- [ ] `SemVerRangeFromStringArrayProperty.SemVerNumber.floatValue(0)`: its class isn't translated (it extends `Number`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java:108)
- [ ] `SemVerRangeFromStringArrayProperty.SemVerNumber.doubleValue(0)`: its class isn't translated (it extends `Number`) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SemVerRangeFromStringArrayProperty.java:113)

### `SimpleIntegerArrayProperty`: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)

- [x] `SimpleIntegerArrayProperty.RANGE_PATTERN` (traben/entity_texture_features/features/property_reading/properties/generic_properties/SimpleIntegerArrayProperty.java:17)
- [ ] `SimpleIntegerArrayProperty.SimpleIntegerArrayProperty(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SimpleIntegerArrayProperty.java:19)
- [ ] `SimpleIntegerArrayProperty.getGenericIntegerSplitWithRanges(3)`: `null` (Rust has no null: this needs an Option) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SimpleIntegerArrayProperty.java:27)
- [ ] `SimpleIntegerArrayProperty.getIntRange(1)`: it calls `Integer`.parseInt (traben/entity_texture_features/features/property_reading/properties/generic_properties/SimpleIntegerArrayProperty.java:61)
- [ ] `SimpleIntegerArrayProperty.testEntityInternal(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SimpleIntegerArrayProperty.java:80)
- [ ] `SimpleIntegerArrayProperty.getValueFromEntity(1)`: it has no body (traben/entity_texture_features/features/property_reading/properties/generic_properties/SimpleIntegerArrayProperty.java:86)
- [ ] `SimpleIntegerArrayProperty.getPrintableRuleInfo(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/generic_properties/SimpleIntegerArrayProperty.java:88)

### `SimpleIntegerArrayProperty.IntRange`

- [x] `SimpleIntegerArrayProperty.IntRange.IntRange(2)` (traben/entity_texture_features/features/property_reading/properties/generic_properties/SimpleIntegerArrayProperty.java:97)
- [x] `SimpleIntegerArrayProperty.IntRange.isWithinRange(1)` (traben/entity_texture_features/features/property_reading/properties/generic_properties/SimpleIntegerArrayProperty.java:107)
- [ ] `SimpleIntegerArrayProperty.IntRange.getAllWithinRangeAsList(0)`: List.toArray (traben/entity_texture_features/features/property_reading/properties/generic_properties/SimpleIntegerArrayProperty.java:111)
- [x] `SimpleIntegerArrayProperty.IntRange.getLower(0)` (traben/entity_texture_features/features/property_reading/properties/generic_properties/SimpleIntegerArrayProperty.java:125)
- [x] `SimpleIntegerArrayProperty.IntRange.getHigher(0)` (traben/entity_texture_features/features/property_reading/properties/generic_properties/SimpleIntegerArrayProperty.java:129)

### `StringArrayOrRegexProperty`: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)

- [x] `StringArrayOrRegexProperty.GROUP_BY_QUOTATION_PATTERN` (traben/entity_texture_features/features/property_reading/properties/generic_properties/StringArrayOrRegexProperty.java:14)
- [ ] `StringArrayOrRegexProperty.StringArrayOrRegexProperty(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/generic_properties/StringArrayOrRegexProperty.java:21)
- [ ] `StringArrayOrRegexProperty.getStringMatcher_Regex_Pattern_List_Single(1)`: it uses RegexAndPatternPropertyMatcher, which isn't translated (it's an interface) (traben/entity_texture_features/features/property_reading/properties/generic_properties/StringArrayOrRegexProperty.java:56)
- [ ] `StringArrayOrRegexProperty.testArray(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/generic_properties/StringArrayOrRegexProperty.java:101)
- [ ] `StringArrayOrRegexProperty.testEntityInternal(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/generic_properties/StringArrayOrRegexProperty.java:118)
- [ ] `StringArrayOrRegexProperty.shouldForceLowerCaseCheck(0)`: it has no body (traben/entity_texture_features/features/property_reading/properties/generic_properties/StringArrayOrRegexProperty.java:139)
- [ ] `StringArrayOrRegexProperty.getValueFromEntity(1)`: it has no body (traben/entity_texture_features/features/property_reading/properties/generic_properties/StringArrayOrRegexProperty.java:141)
- [ ] `StringArrayOrRegexProperty.getPrintableRuleInfo(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/generic_properties/StringArrayOrRegexProperty.java:144)

### `StringArrayOrRegexProperty.RegexAndPatternPropertyMatcher`: it's an interface

- [ ] `StringArrayOrRegexProperty.RegexAndPatternPropertyMatcher.testString(1)`: it has no body (traben/entity_texture_features/features/property_reading/properties/generic_properties/StringArrayOrRegexProperty.java:150)

### `BabyProperty`: it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `BabyProperty.BabyProperty(2)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BabyProperty.java:12)
- [ ] `BabyProperty.getPropertyOrNull(2)`: it uses BabyProperty, which isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BabyProperty.java:16)
- [ ] `BabyProperty.getValueFromEntity(1)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BabyProperty.java:24)
- [ ] `BabyProperty.getPropertyIds(0)`: its class isn't translated (it extends `BooleanProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BabyProperty.java:30)

### `BiomeProperty`: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `BiomeProperty.BiomeProperty(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BiomeProperty.java:13)
- [ ] `BiomeProperty.getPropertyOrNull(2)`: it uses BiomeProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BiomeProperty.java:17)
- [ ] `BiomeProperty.shouldForceLowerCaseCheck(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BiomeProperty.java:97)
- [ ] `BiomeProperty.getValueFromEntity(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BiomeProperty.java:102)
- [ ] `BiomeProperty.getPropertyIds(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BiomeProperty.java:113)

### `BlocksProperty`: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `BlocksProperty.BlocksProperty(3)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BlocksProperty.java:24)
- [ ] `BlocksProperty.getPropertyOrNull(2)`: it uses BlocksProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BlocksProperty.java:46)
- [ ] `BlocksProperty.getFromStateBlockNameOnly(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BlocksProperty.java:54)
- [ ] `BlocksProperty.getFromStateBlockNameWithStateData(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BlocksProperty.java:63)
- [ ] `BlocksProperty.testBlocks(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BlocksProperty.java:78)
- [ ] `BlocksProperty.shouldForceLowerCaseCheck(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BlocksProperty.java:107)
- [ ] `BlocksProperty.getTestingBlocks(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BlocksProperty.java:112)
- [ ] `BlocksProperty.testEntityInternal(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BlocksProperty.java:129)
- [ ] `BlocksProperty.getValueFromEntity(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BlocksProperty.java:161)
- [ ] `BlocksProperty.getPropertyIds(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/BlocksProperty.java:167)

### `ColorProperty`: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `ColorProperty.ColorProperty(2)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/ColorProperty.java:22)
- [ ] `ColorProperty.getPropertyOrNull(2)`: it uses ColorProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/ColorProperty.java:26)
- [ ] `ColorProperty.shouldForceLowerCaseCheck(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/ColorProperty.java:34)
- [ ] `ColorProperty.getValueFromEntity(1)`: its body couldn't be read: expected `:` but found `wolf` (line 72) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/ColorProperty.java:39)
- [ ] `ColorProperty.getPropertyIds(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/ColorProperty.java:80)

### `HealthProperty`: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `HealthProperty.HealthProperty(2)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/HealthProperty.java:15)
- [ ] `HealthProperty.getPropertyOrNull(2)`: it uses HealthProperty, which isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/HealthProperty.java:19)
- [ ] `HealthProperty.getRangeValueFromEntity(1)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/HealthProperty.java:27)
- [ ] `HealthProperty.getPropertyIds(0)`: its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/HealthProperty.java:37)

### `HeightProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `HeightProperty.HeightProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/HeightProperty.java:10)
- [ ] `HeightProperty.getPropertyOrNull(2)`: it uses HeightProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/HeightProperty.java:14)
- [ ] `HeightProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/HeightProperty.java:38)
- [ ] `HeightProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/HeightProperty.java:44)

### `MoonPhaseProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `MoonPhaseProperty.MoonPhaseProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/MoonPhaseProperty.java:11)
- [ ] `MoonPhaseProperty.getPropertyOrNull(2)`: it uses MoonPhaseProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/MoonPhaseProperty.java:15)
- [ ] `MoonPhaseProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/MoonPhaseProperty.java:23)
- [ ] `MoonPhaseProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/MoonPhaseProperty.java:29)

### `NameProperty`: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `NameProperty.NameProperty(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NameProperty.java:17)
- [ ] `NameProperty.getPropertyOrNull(2)`: it uses NameProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NameProperty.java:21)
- [ ] `NameProperty.shouldForceLowerCaseCheck(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NameProperty.java:51)
- [ ] `NameProperty.getValueFromEntity(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NameProperty.java:56)
- [ ] `NameProperty.getPropertyIds(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NameProperty.java:77)

### `NBTProperty`: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)

- [ ] `NBTProperty.INTENTIONAL_FAILURE`: it uses Minecraft's `CompoundTag` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:29)
- [x] `NBTProperty.crashMessages` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:30)
- [ ] `NBTProperty.NBTProperty(3)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:35)
- [ ] `NBTProperty.getPropertyOrNull(2)`: it uses NBTProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:59)
- [ ] `NBTProperty.isStringValidInt(1)`: it calls `Integer`.parseInt (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:67)
- [ ] `NBTProperty.getEntityNBT(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:76)
- [ ] `NBTProperty.testEntityInternal(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:81)
- [ ] `NBTProperty.testAllNBTCases(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:137)
- [ ] `NBTProperty.getAsString(1)`: it uses Minecraft's `Tag` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:175)
- [ ] `NBTProperty.getAsNumber(1)`: it uses `Number` (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:179)
- [ ] `NBTProperty.formatNbtPretty(1)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:183)
- [ ] `NBTProperty.findNBTElements(2)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:225)
- [ ] `NBTProperty.findByIteration(3)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:232)
- [ ] `NBTProperty.handleListInstruction(3)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:253)
- [ ] `NBTProperty.getPropertyIds(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:287)
- [ ] `NBTProperty.getPrintableRuleInfo(0)`: its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:293)

### `NBTProperty.NBTTester`: it uses Minecraft's `Tag`

- [ ] `NBTProperty.NBTTester.NBTTester(4)`: its class isn't translated (it uses Minecraft's `Tag`) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:298)
- [ ] `NBTProperty.NBTTester.of(2)`: it uses NBTTester, which isn't translated (it uses Minecraft's `Tag`) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:299)
- [ ] `NBTProperty.NBTTester.inverts(0)`: its class isn't translated (it uses Minecraft's `Tag`) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:298)
- [ ] `NBTProperty.NBTTester.tester(0)`: its class isn't translated (it uses Minecraft's `Tag`) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:298)
- [ ] `NBTProperty.NBTTester.wantsBlank(0)`: its class isn't translated (it uses Minecraft's `Tag`) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:298)
- [ ] `NBTProperty.NBTTester.print(0)`: its class isn't translated (it uses Minecraft's `Tag`) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/NBTProperty.java:298)

### `ProfessionProperty`: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `ProfessionProperty.ProfessionProperty(2)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/ProfessionProperty.java:14)
- [ ] `ProfessionProperty.getPropertyOrNull(2)`: it uses ProfessionProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/ProfessionProperty.java:18)
- [ ] `ProfessionProperty.testEntityInternal(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/ProfessionProperty.java:26)
- [ ] `ProfessionProperty.shouldForceLowerCaseCheck(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/ProfessionProperty.java:77)
- [ ] `ProfessionProperty.getValueFromEntity(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/ProfessionProperty.java:82)
- [ ] `ProfessionProperty.getPropertyIds(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/ProfessionProperty.java:87)

### `SizeProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `SizeProperty.SizeProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/SizeProperty.java:12)
- [ ] `SizeProperty.getPropertyOrNull(2)`: it uses SizeProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/SizeProperty.java:16)
- [ ] `SizeProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/SizeProperty.java:24)
- [ ] `SizeProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/SizeProperty.java:30)

### `TimeOfDayProperty`: it extends `LongRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `TimeOfDayProperty.TimeOfDayProperty(2)`: its class isn't translated (it extends `LongRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/TimeOfDayProperty.java:11)
- [ ] `TimeOfDayProperty.getPropertyOrNull(2)`: it uses TimeOfDayProperty, which isn't translated (it extends `LongRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/TimeOfDayProperty.java:15)
- [ ] `TimeOfDayProperty.getRangeValueFromEntity(1)`: its class isn't translated (it extends `LongRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/TimeOfDayProperty.java:23)
- [ ] `TimeOfDayProperty.getPropertyIds(0)`: its class isn't translated (it extends `LongRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/TimeOfDayProperty.java:28)

### `WeatherProperty`: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `WeatherProperty.WeatherProperty(2)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/WeatherProperty.java:11)
- [ ] `WeatherProperty.getPropertyOrNull(2)`: it uses WeatherProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/WeatherProperty.java:18)
- [ ] `WeatherProperty.shouldForceLowerCaseCheck(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/WeatherProperty.java:26)
- [ ] `WeatherProperty.getValueFromEntity(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/WeatherProperty.java:31)
- [ ] `WeatherProperty.getPropertyIds(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_texture_features/features/property_reading/properties/optifine_properties/WeatherProperty.java:45)

### `RandomProperties`

- [ ] `RandomProperties.REGISTERED_PROPERTIES`: it uses RandomPropertyFactory, which isn't translated (it's an interface) (traben/entity_texture_features/features/property_reading/properties/RandomProperties.java:77)
- [ ] `RandomProperties.forEachProperty(1)`: it uses RandomPropertyFactory, which isn't translated (it's an interface) (traben/entity_texture_features/features/property_reading/properties/RandomProperties.java:79)
- [ ] `RandomProperties.register(1)`: it uses RandomPropertyFactory, which isn't translated (it's an interface) (traben/entity_texture_features/features/property_reading/properties/RandomProperties.java:83)
- [ ] `RandomProperties.getAllRegisteredRandomPropertiesOfIndex(2)`: it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`)) (traben/entity_texture_features/features/property_reading/properties/RandomProperties.java:91)

### `RandomProperties.RandomPropertyFactory`: it's an interface

- [ ] `RandomProperties.RandomPropertyFactory.of(3)`: it uses RandomPropertyFactory, which isn't translated (it's an interface) (traben/entity_texture_features/features/property_reading/properties/RandomProperties.java:261)
- [ ] `RandomProperties.RandomPropertyFactory.of(4)`: its body couldn't be read: an anonymous class (traben/entity_texture_features/features/property_reading/properties/RandomProperties.java:268)
- [ ] `RandomProperties.RandomPropertyFactory.getPropertyOrNull(2)`: it has no body (traben/entity_texture_features/features/property_reading/properties/RandomProperties.java:325)
- [ ] `RandomProperties.RandomPropertyFactory.getPropertyId(0)`: it has no body (traben/entity_texture_features/features/property_reading/properties/RandomProperties.java:328)
- [ ] `RandomProperties.RandomPropertyFactory.updatesOverTime(0)`: it has no body (traben/entity_texture_features/features/property_reading/properties/RandomProperties.java:331)
- [ ] `RandomProperties.RandomPropertyFactory.getExplanationTranslationKey(0)`: it has no body (traben/entity_texture_features/features/property_reading/properties/RandomProperties.java:333)

### `RandomProperty`: it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`)

- [x] `RandomProperty.readPropertiesOrThrow(3)` (traben/entity_texture_features/features/property_reading/properties/RandomProperty.java:13)
- [ ] `RandomProperty.testEntity(2)`: its class isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`)) (traben/entity_texture_features/features/property_reading/properties/RandomProperty.java:31)
- [ ] `RandomProperty.testEntityInternal(1)`: it has no body (traben/entity_texture_features/features/property_reading/properties/RandomProperty.java:44)
- [ ] `RandomProperty.canPropertyUpdate(0)`: its class isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`)) (traben/entity_texture_features/features/property_reading/properties/RandomProperty.java:46)
- [ ] `RandomProperty.setCanUpdate(1)`: its class isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`)) (traben/entity_texture_features/features/property_reading/properties/RandomProperty.java:50)
- [ ] `RandomProperty.getPropertyIds(0)`: it has no body (traben/entity_texture_features/features/property_reading/properties/RandomProperty.java:54)
- [ ] `RandomProperty.getPropertyId(0)`: its class isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`)) (traben/entity_texture_features/features/property_reading/properties/RandomProperty.java:57)
- [ ] `RandomProperty.getPrintableRuleInfo(0)`: it has no body (traben/entity_texture_features/features/property_reading/properties/RandomProperty.java:62)
- [ ] `RandomProperty.toString(0)`: its class isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`)) (traben/entity_texture_features/features/property_reading/properties/RandomProperty.java:64)
- [ ] `RandomProperty.cacheEntityInitialResult(1)`: its class isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`)) (traben/entity_texture_features/features/property_reading/properties/RandomProperty.java:69)

### `RandomProperty.RandomPropertyException`: it extends `Exception`

- [ ] `RandomProperty.RandomPropertyException.RandomPropertyException(1)`: its class isn't translated (it extends `Exception`) (traben/entity_texture_features/features/property_reading/properties/RandomProperty.java:78)

### `RandomPropertyRule`: it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))

- [ ] `RandomPropertyRule.DEFAULT_RETURN`: its value couldn't be read: an anonymous class (traben/entity_texture_features/features/property_reading/RandomPropertyRule.java:24)
- [ ] `RandomPropertyRule.RandomPropertyRule(0)`: its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))) (traben/entity_texture_features/features/property_reading/RandomPropertyRule.java:41)
- [ ] `RandomPropertyRule.isAlwaysMet(0)`: its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))) (traben/entity_texture_features/features/property_reading/RandomPropertyRule.java:54)
- [ ] `RandomPropertyRule.RandomPropertyRule(7)`: its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))) (traben/entity_texture_features/features/property_reading/RandomPropertyRule.java:58)
- [ ] `RandomPropertyRule.getSuffixSet(0)`: its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))) (traben/entity_texture_features/features/property_reading/RandomPropertyRule.java:129)
- [ ] `RandomPropertyRule.doesEntityMeetConditionsOfThisCase(3)`: its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))) (traben/entity_texture_features/features/property_reading/RandomPropertyRule.java:133)
- [ ] `RandomPropertyRule.getVariantSuffixFromThisCase(1)`: its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))) (traben/entity_texture_features/features/property_reading/RandomPropertyRule.java:160)
- [ ] `RandomPropertyRule.getVariantSuffixFromThisCase(1)`: its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))) (traben/entity_texture_features/features/property_reading/RandomPropertyRule.java:177)
- [ ] `RandomPropertyRule.getSeedFrom(1)`: its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))) (traben/entity_texture_features/features/property_reading/RandomPropertyRule.java:194)
- [ ] `RandomPropertyRule.cacheEntityInitialResultsOfNonUpdatingProperties(1)`: its class isn't translated (it uses RandomProperty, which isn't translated (it uses UUIDBoolean, which isn't translated (it extends `ETFLruCache`))) (traben/entity_texture_features/features/property_reading/RandomPropertyRule.java:203)

### `RandomPropertyRule.SeedSource`


### `TrueRandomProvider`: it implements `ETFVariantSuffixProvider`

- [ ] `TrueRandomProvider.TrueRandomProvider(2)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/TrueRandomProvider.java:25)
- [ ] `TrueRandomProvider.of(1)`: it uses TrueRandomProvider, which isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/TrueRandomProvider.java:30)
- [ ] `TrueRandomProvider.getPackName(0)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/TrueRandomProvider.java:72)
- [ ] `TrueRandomProvider.entityCanUpdate(1)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/TrueRandomProvider.java:77)
- [ ] `TrueRandomProvider.getAllSuffixes(0)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/TrueRandomProvider.java:82)
- [ ] `TrueRandomProvider.size(0)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/TrueRandomProvider.java:87)
- [ ] `TrueRandomProvider.getSuffixForETFEntity(1)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/TrueRandomProvider.java:92)
- [ ] `TrueRandomProvider.setRandomSupplier(1)`: its class isn't translated (it implements `ETFVariantSuffixProvider`) (traben/entity_texture_features/features/property_reading/TrueRandomProvider.java:97)

### `ETFEntityRenderState`: it's an interface

- [ ] `ETFEntityRenderState.uuid(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:26)
- [ ] `ETFEntityRenderState.canRenderBright(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:28)
- [ ] `ETFEntityRenderState.isBlockEntity(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:30)
- [ ] `ETFEntityRenderState.entityType(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:32)
- [ ] `ETFEntityRenderState.world(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:34)
- [ ] `ETFEntityRenderState.blockPos(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:36)
- [ ] `ETFEntityRenderState.optifineId(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:38)
- [ ] `ETFEntityRenderState.optifineVehicleId(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:40)
- [ ] `ETFEntityRenderState.blockY(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:42)
- [ ] `ETFEntityRenderState.nbt(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:44)
- [ ] `ETFEntityRenderState.hasCustomName(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:46)
- [ ] `ETFEntityRenderState.customName(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:48)
- [ ] `ETFEntityRenderState.scoreboardTeam(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:50)
- [ ] `ETFEntityRenderState.itemsEquipped(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:52)
- [ ] `ETFEntityRenderState.handItems(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:54)
- [ ] `ETFEntityRenderState.armorItems(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:56)
- [ ] `ETFEntityRenderState.velocity(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:58)
- [ ] `ETFEntityRenderState.pose(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:60)
- [ ] `ETFEntityRenderState.entityKey(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:63)
- [ ] `ETFEntityRenderState.entity(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:65)
- [ ] `ETFEntityRenderState.isPlayer(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:68)
- [ ] `ETFEntityRenderState.isClientPlayer(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:70)
- [ ] `ETFEntityRenderState.vanillaState(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:72)
- [ ] `ETFEntityRenderState.setVanillaState(1)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:75)
- [ ] `ETFEntityRenderState.vanillaBlockState(0)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:77)
- [ ] `ETFEntityRenderState.setVanillaBlockState(1)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:80)
- [ ] `ETFEntityRenderState.distanceTo(1)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:82)
- [ ] `ETFEntityRenderState.cacheEntityNBTForState(2)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:84)
- [ ] `ETFEntityRenderState.preSubmitActivate(2)`: its class isn't translated (it's an interface) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:86)
- [ ] `ETFEntityRenderState.activate(1)`: its class isn't translated (it's an interface) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:89)
- [ ] `ETFEntityRenderState.deactivate(1)`: its class isn't translated (it's an interface) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:92)
- [ ] `ETFEntityRenderState.setEtfRenderStateConstructor(2)`: it uses ETFRenderStateInit, which isn't translated (it's an interface) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:95)
- [ ] `ETFEntityRenderState.forEntity(1)`: it uses ETFEntityRenderState, which isn't translated (it's an interface) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:100)
- [ ] `ETFEntityRenderState.from(1)`: it uses ETFEntityRenderState, which isn't translated (it's an interface) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:104)

### `ETFEntityRenderState.ETFRenderStateInit`: it's an interface

- [ ] `ETFEntityRenderState.ETFRenderStateInit.make(1)`: it has no body (traben/entity_texture_features/features/state/ETFEntityRenderState.java:110)

### `ETFEntityRenderState.Null`: it implements `ETFEntityRenderState`

- [ ] `ETFEntityRenderState.Null.uuid(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:114)
- [ ] `ETFEntityRenderState.Null.canRenderBright(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:119)
- [ ] `ETFEntityRenderState.Null.isBlockEntity(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:124)
- [ ] `ETFEntityRenderState.Null.entityType(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:129)
- [ ] `ETFEntityRenderState.Null.world(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:134)
- [ ] `ETFEntityRenderState.Null.blockPos(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:139)
- [ ] `ETFEntityRenderState.Null.optifineId(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:144)
- [ ] `ETFEntityRenderState.Null.optifineVehicleId(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:149)
- [ ] `ETFEntityRenderState.Null.blockY(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:154)
- [ ] `ETFEntityRenderState.Null.nbt(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:159)
- [ ] `ETFEntityRenderState.Null.hasCustomName(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:164)
- [ ] `ETFEntityRenderState.Null.customName(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:169)
- [ ] `ETFEntityRenderState.Null.scoreboardTeam(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:174)
- [ ] `ETFEntityRenderState.Null.itemsEquipped(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:179)
- [ ] `ETFEntityRenderState.Null.handItems(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:184)
- [ ] `ETFEntityRenderState.Null.armorItems(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:189)
- [ ] `ETFEntityRenderState.Null.velocity(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:194)
- [ ] `ETFEntityRenderState.Null.pose(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:199)
- [ ] `ETFEntityRenderState.Null.entityKey(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:204)
- [ ] `ETFEntityRenderState.Null.entity(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:209)
- [ ] `ETFEntityRenderState.Null.isPlayer(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:214)
- [ ] `ETFEntityRenderState.Null.isClientPlayer(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:219)
- [ ] `ETFEntityRenderState.Null.vanillaState(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:224)
- [ ] `ETFEntityRenderState.Null.setVanillaState(1)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:229)
- [ ] `ETFEntityRenderState.Null.vanillaBlockState(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:233)
- [ ] `ETFEntityRenderState.Null.setVanillaBlockState(1)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:238)
- [ ] `ETFEntityRenderState.Null.distanceTo(1)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:242)
- [ ] `ETFEntityRenderState.Null.cacheEntityNBTForState(2)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderState.java:247)

### `ETFEntityRenderStateViaReference`: it implements `ETFEntityRenderState`

- [ ] `ETFEntityRenderStateViaReference.ETFEntityRenderStateViaReference(1)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:32)
- [ ] `ETFEntityRenderStateViaReference.entity(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:39)
- [ ] `ETFEntityRenderStateViaReference.isPlayer(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:45)
- [ ] `ETFEntityRenderStateViaReference.isClientPlayer(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:50)
- [ ] `ETFEntityRenderStateViaReference.vanillaState(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:55)
- [ ] `ETFEntityRenderStateViaReference.setVanillaState(1)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:60)
- [ ] `ETFEntityRenderStateViaReference.vanillaBlockState(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:65)
- [ ] `ETFEntityRenderStateViaReference.setVanillaBlockState(1)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:70)
- [ ] `ETFEntityRenderStateViaReference.uuid(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:75)
- [ ] `ETFEntityRenderStateViaReference.canRenderBright(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:84)
- [ ] `ETFEntityRenderStateViaReference.isBlockEntity(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:89)
- [ ] `ETFEntityRenderStateViaReference.entityType(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:94)
- [ ] `ETFEntityRenderStateViaReference.world(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:99)
- [ ] `ETFEntityRenderStateViaReference.blockPos(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:104)
- [ ] `ETFEntityRenderStateViaReference.optifineId(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:109)
- [ ] `ETFEntityRenderStateViaReference.optifineVehicleId(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:114)
- [ ] `ETFEntityRenderStateViaReference.blockY(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:119)
- [ ] `ETFEntityRenderStateViaReference.nbt(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:124)
- [ ] `ETFEntityRenderStateViaReference.hasCustomName(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:129)
- [ ] `ETFEntityRenderStateViaReference.customName(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:134)
- [ ] `ETFEntityRenderStateViaReference.scoreboardTeam(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:139)
- [ ] `ETFEntityRenderStateViaReference.itemsEquipped(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:144)
- [ ] `ETFEntityRenderStateViaReference.handItems(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:149)
- [ ] `ETFEntityRenderStateViaReference.armorItems(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:154)
- [ ] `ETFEntityRenderStateViaReference.velocity(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:159)
- [ ] `ETFEntityRenderStateViaReference.pose(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:164)
- [ ] `ETFEntityRenderStateViaReference.entityKey(0)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:170)
- [ ] `ETFEntityRenderStateViaReference.distanceTo(1)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:175)
- [ ] `ETFEntityRenderStateViaReference.cacheEntityNBTForState(2)`: its class isn't translated (it implements `ETFEntityRenderState`) (traben/entity_texture_features/features/state/ETFEntityRenderStateViaReference.java:180)

### `ETFState`

- [ ] `ETFState.stateStack`: its value couldn't be read: an anonymous class (traben/entity_texture_features/features/state/ETFState.java:22)
- [x] `ETFState.stackVerifyHasFailed` (traben/entity_texture_features/features/state/ETFState.java:32)
- [x] `ETFState.currentModelPartDepth` (traben/entity_texture_features/features/state/ETFState.java:33)
- [x] `ETFState.isRenderingFeatures` (traben/entity_texture_features/features/state/ETFState.java:34)
- [x] `ETFState.onlyRandomizeViaPropertiesFiles` (traben/entity_texture_features/features/state/ETFState.java:35)
- [x] `ETFState.allowTexturePatching` (traben/entity_texture_features/features/state/ETFState.java:36)
- [ ] `ETFState.specialPhaseStack`: it uses `Deque` (java.util) (traben/entity_texture_features/features/state/ETFState.java:37)
- [ ] `ETFState.renderLayerModifyStack`: it uses `Deque` (java.util) (traben/entity_texture_features/features/state/ETFState.java:38)
- [ ] `ETFState.mount(1)`: it uses ETFEntityRenderState, which isn't translated (it's an interface) (traben/entity_texture_features/features/state/ETFState.java:40)
- [ ] `ETFState.unMount(0)`: it calls Minecraft's `RenderSystem`.isOnRenderThread (traben/entity_texture_features/features/state/ETFState.java:59)
- [ ] `ETFState.mountNone(0)`: it calls ETFState.mount, which isn't translated (traben/entity_texture_features/features/state/ETFState.java:81)
- [ ] `ETFState.state(0)`: it uses ETFEntityRenderState, which isn't translated (it's an interface) (traben/entity_texture_features/features/state/ETFState.java:85)
- [ ] `ETFState.isStateActive(0)`: it calls ETFState.state, which isn't translated (traben/entity_texture_features/features/state/ETFState.java:90)
- [ ] `ETFState.clear(0)`: it uses ETFEntityRenderState, which isn't translated (it's an interface) (traben/entity_texture_features/features/state/ETFState.java:94)
- [ ] `ETFState.announceStack(1)`: it calls ETF.config, which isn't translated (traben/entity_texture_features/features/state/ETFState.java:111)
- [ ] `ETFState.stackVerify(1)`: it uses ETFEntityRenderState, which isn't translated (it's an interface) (traben/entity_texture_features/features/state/ETFState.java:126)
- [ ] `ETFState.stackVerifyEmpty(0)`: it uses ETFState.stateStack, which isn't translated (its value couldn't be read: an anonymous class) (traben/entity_texture_features/features/state/ETFState.java:161)
- [ ] `ETFState.isIsInSpecialRenderOverlayPhase(0)`: it calls Minecraft's `RenderSystem`.isOnRenderThread (traben/entity_texture_features/features/state/ETFState.java:182)
- [ ] `ETFState.startSpecialRenderOverlayPhase(0)`: it calls Minecraft's `RenderSystem`.isOnRenderThread (traben/entity_texture_features/features/state/ETFState.java:186)
- [ ] `ETFState.endSpecialRenderOverlayPhase(0)`: it calls Minecraft's `RenderSystem`.isOnRenderThread (traben/entity_texture_features/features/state/ETFState.java:192)
- [ ] `ETFState.isAllowedToRenderLayerTextureModify(0)`: it calls ETF.config, which isn't translated (traben/entity_texture_features/features/state/ETFState.java:198)
- [ ] `ETFState.pushRenderLayerModifyState(1)`: it calls Minecraft's `RenderSystem`.isOnRenderThread (traben/entity_texture_features/features/state/ETFState.java:211)
- [ ] `ETFState.popRenderLayerModifyState(0)`: it calls Minecraft's `RenderSystem`.isOnRenderThread (traben/entity_texture_features/features/state/ETFState.java:217)
- [ ] `ETFState.modifyRenderLayerIfRequired(1)`: it uses Minecraft's `RenderType` (traben/entity_texture_features/features/state/ETFState.java:223)
- [ ] `ETFState.insertETFDataIntoVertexConsumer(3)`: it uses URenderTypeToVertexConsumer, which isn't translated (it uses Minecraft's `RenderTypeFeatureRenderer`) (traben/entity_texture_features/features/state/ETFState.java:248)
- [ ] `ETFState.canRenderInBrightMode(0)`: it calls ETFManager.getEmissiveMode, which isn't translated (traben/entity_texture_features/features/state/ETFState.java:254)

### `ETFSubmitData`: it uses ETFEntityRenderState, which isn't translated (it's an interface)

- [x] `ETFSubmitData.POOL_CAPACITY` (traben/entity_texture_features/features/state/ETFSubmitData.java:15)
- [ ] `ETFSubmitData.POOL`: it uses ETFSubmitData, which isn't translated (it uses ETFEntityRenderState, which isn't translated (it's an interface)) (traben/entity_texture_features/features/state/ETFSubmitData.java:16)
- [x] `ETFSubmitData.poolSize` (traben/entity_texture_features/features/state/ETFSubmitData.java:17)
- [x] `ETFSubmitData.poolNext` (traben/entity_texture_features/features/state/ETFSubmitData.java:18)
- [ ] `ETFSubmitData.DATA_IN`: it uses ETFSubmitData, which isn't translated (it uses ETFEntityRenderState, which isn't translated (it's an interface)) (traben/entity_texture_features/features/state/ETFSubmitData.java:19)
- [ ] `ETFSubmitData.DATA_OUT`: it uses ETFSubmitData, which isn't translated (it uses ETFEntityRenderState, which isn't translated (it's an interface)) (traben/entity_texture_features/features/state/ETFSubmitData.java:20)
- [ ] `ETFSubmitData.acquire(0)`: it uses ETFSubmitData, which isn't translated (it uses ETFEntityRenderState, which isn't translated (it's an interface)) (traben/entity_texture_features/features/state/ETFSubmitData.java:22)
- [ ] `ETFSubmitData.recycle(0)`: it uses ETFSubmitData.POOL, which isn't translated (a ETFSubmitData with no value (null in Java)) (traben/entity_texture_features/features/state/ETFSubmitData.java:42)
- [ ] `ETFSubmitData.from(1)`: it uses ETFSubmitData, which isn't translated (it uses ETFEntityRenderState, which isn't translated (it's an interface)) (traben/entity_texture_features/features/state/ETFSubmitData.java:51)

### `ETFSubmitExtension`: it's an interface

- [ ] `ETFSubmitExtension.emf$getData(0)`: it has no body (traben/entity_texture_features/features/state/ETFSubmitExtension.java:6)

### `HoldsETFRenderState`: it's an interface

- [ ] `HoldsETFRenderState.etf$getState(0)`: it has no body (traben/entity_texture_features/features/state/HoldsETFRenderState.java:6)
- [ ] `HoldsETFRenderState.etf$initState(1)`: it has no body (traben/entity_texture_features/features/state/HoldsETFRenderState.java:8)

### `ETFArmorHandler`: it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`))

- [ ] `ETFArmorHandler.start(0)`: its class isn't translated (it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`))) (traben/entity_texture_features/features/texture_handlers/ETFArmorHandler.java:18)
- [ ] `ETFArmorHandler.end(0)`: its class isn't translated (it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`))) (traben/entity_texture_features/features/texture_handlers/ETFArmorHandler.java:22)
- [ ] `ETFArmorHandler.renderTrimEmissive(3)`: it uses Minecraft's `Model` (traben/entity_texture_features/features/texture_handlers/ETFArmorHandler.java:26)
- [ ] `ETFArmorHandler.setTrim(1)`: it calls ETF.config, which isn't translated (traben/entity_texture_features/features/texture_handlers/ETFArmorHandler.java:41)

### `ETFDirectory`: its constants carry values

- [ ] `ETFDirectory.ETFDirectory(1)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/features/texture_handlers/ETFDirectory.java:22)
- [ ] `ETFDirectory.getCache(0)`: it uses ETFDirectory, which isn't translated (its constants carry values) (traben/entity_texture_features/features/texture_handlers/ETFDirectory.java:26)
- [ ] `ETFDirectory.getDirectoryVersionOf(1)`: it uses ETFDirectory, which isn't translated (its constants carry values) (traben/entity_texture_features/features/texture_handlers/ETFDirectory.java:30)
- [ ] `ETFDirectory.getDirectoryOf(1)`: it uses ETFDirectory, which isn't translated (its constants carry values) (traben/entity_texture_features/features/texture_handlers/ETFDirectory.java:45)
- [ ] `ETFDirectory.findDirectoryOf(1)`: it uses ETFDirectory, which isn't translated (its constants carry values) (traben/entity_texture_features/features/texture_handlers/ETFDirectory.java:57)
- [ ] `ETFDirectory.getIdentifierAsDirectory(2)`: it uses ETFDirectory, which isn't translated (its constants carry values) (traben/entity_texture_features/features/texture_handlers/ETFDirectory.java:109)
- [ ] `ETFDirectory.doesReplace(0)`: its class isn't translated (its constants carry values) (traben/entity_texture_features/features/texture_handlers/ETFDirectory.java:116)

### `ETFSprite`: it uses Minecraft's `TextureAtlasSprite`

- [ ] `ETFSprite.ETFSprite(3)`: its class isn't translated (it uses Minecraft's `TextureAtlasSprite`) (traben/entity_texture_features/features/texture_handlers/ETFSprite.java:27)
- [ ] `ETFSprite.load(2)`: it uses Minecraft's `SpriteContents` (traben/entity_texture_features/features/texture_handlers/ETFSprite.java:100)
- [ ] `ETFSprite.getEmissive(0)`: its class isn't translated (it uses Minecraft's `TextureAtlasSprite`) (traben/entity_texture_features/features/texture_handlers/ETFSprite.java:132)
- [ ] `ETFSprite.isEmissive(0)`: its class isn't translated (it uses Minecraft's `TextureAtlasSprite`) (traben/entity_texture_features/features/texture_handlers/ETFSprite.java:137)
- [ ] `ETFSprite.getSpriteVariant(0)`: its class isn't translated (it uses Minecraft's `TextureAtlasSprite`) (traben/entity_texture_features/features/texture_handlers/ETFSprite.java:141)

### `ETFTexture`: it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)

- [x] `ETFTexture.PATCH_NAMESPACE_PREFIX` (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:30)
- [ ] `ETFTexture.ETFTexture(1)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:52)
- [ ] `ETFTexture.manual(12)`: it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:66)
- [ ] `ETFTexture.manual(3)`: it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:96)
- [ ] `ETFTexture.ETFTexture(12)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:100)
- [ ] `ETFTexture.ETFTexture(2)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:148)
- [ ] `ETFTexture.ofUnmodifiable(2)`: it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:153)
- [ ] `ETFTexture.patchTextureToRemoveZFightingWithOtherTexture(2)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:157)
- [ ] `ETFTexture.doesAnimaticaVersionExist(1)`: it calls Minecraft's Minecraft.getTextureManager (no Horizon helper for it yet) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:174)
- [ ] `ETFTexture.setupBlinking(0)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:183)
- [ ] `ETFTexture.exists(0)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:222)
- [ ] `ETFTexture.setupEmissives(0)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:230)
- [ ] `ETFTexture.setupEnchants(0)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:260)
- [ ] `ETFTexture.getResourceOrModifyForTrims(1)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:298)
- [ ] `ETFTexture.getTextureIdentifier(1)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:312)
- [ ] `ETFTexture.getBlinkingIdentifier(1)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:323)
- [ ] `ETFTexture.setBlink(2)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:343)
- [ ] `ETFTexture.setGUIBlink(0)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:359)
- [ ] `ETFTexture.isEmissive(0)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:365)
- [ ] `ETFTexture.isEnchanted(0)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:369)
- [ ] `ETFTexture.canPatch(0)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:373)
- [ ] `ETFTexture.doesBlink(0)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:377)
- [ ] `ETFTexture.getPaintingSprite(2)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:381)
- [ ] `ETFTexture.doesBlink2(0)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:390)
- [ ] `ETFTexture.toString(0)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:394)
- [ ] `ETFTexture.getEmissiveRenderLayer(1)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:399)
- [ ] `ETFTexture.getEmissiveRenderLayer(2)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:404)
- [ ] `ETFTexture.getEmissiveVertexConsumerWrapped(2)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:412)
- [ ] `ETFTexture.modifyTextureState(1)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:432)
- [ ] `ETFTexture.identifierOfCurrentState(0)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:447)
- [ ] `ETFTexture.getEmissiveIdentifierOfCurrentState(0)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:460)
- [ ] `ETFTexture.getEnchantIdentifierOfCurrentState(0)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:470)
- [ ] `ETFTexture.assertPatchedTextures(0)`: its class isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:480)

### `ETFTexture.TextureReturnState`

- [ ] `ETFTexture.TextureReturnState.toString(0)`: `this` used as a value (traben/entity_texture_features/features/texture_handlers/ETFTexture.java:547)

### `ETFTextureVariator`

- [ ] `ETFTextureVariator.of(1)`: it uses ETFVariantSuffixProvider, which isn't translated (it's an interface) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:21)
- [ ] `ETFTextureVariator.getVariantOf(1)`: it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:31)
- [ ] `ETFTextureVariator.logDebugMessageIfNeeded(2)`: it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:37)
- [ ] `ETFTextureVariator.getVanillaVariantDetails(0)`: it calls ETFDirectory.getIdentifierAsDirectory, which isn't translated (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:60)
- [ ] `ETFTextureVariator.getPrintout(0)`: it has no body (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:84)
- [ ] `ETFTextureVariator.getVariantOfInternal(1)`: it has no body (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:86)
- [ ] `ETFTextureVariator.getVanillaIdentifier(0)`: it has no body (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:89)

### `ETFTextureVariator.ETFTextureMultiple`: it extends `ETFTextureVariator`

- [ ] `ETFTextureVariator.ETFTextureMultiple.ETFTextureMultiple(2)`: its body couldn't be read: an anonymous class (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:101)
- [ ] `ETFTextureVariator.ETFTextureMultiple.equals(1)`: its class isn't translated (it extends `ETFTextureVariator`) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:156)
- [ ] `ETFTextureVariator.ETFTextureMultiple.hashCode(0)`: its class isn't translated (it extends `ETFTextureVariator`) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:168)
- [ ] `ETFTextureVariator.ETFTextureMultiple.getPrintout(0)`: its class isn't translated (it extends `ETFTextureVariator`) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:173)
- [ ] `ETFTextureVariator.ETFTextureMultiple.checkIfShouldExpireEntity(2)`: its class isn't translated (it extends `ETFTextureVariator`) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:185)
- [ ] `ETFTextureVariator.ETFTextureMultiple.getVariantOfInternal(1)`: its class isn't translated (it extends `ETFTextureVariator`) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:203)
- [ ] `ETFTextureVariator.ETFTextureMultiple.determineTextureSource(1)`: its class isn't translated (it extends `ETFTextureVariator`) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:219)
- [ ] `ETFTextureVariator.ETFTextureMultiple.determineNewSuffix(2)`: its class isn't translated (it extends `ETFTextureVariator`) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:227)
- [ ] `ETFTextureVariator.ETFTextureMultiple.getBaseEntitySuffixOrNew(1)`: its class isn't translated (it extends `ETFTextureVariator`) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:239)
- [ ] `ETFTextureVariator.ETFTextureMultiple.getVanillaIdentifier(0)`: its class isn't translated (it extends `ETFTextureVariator`) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:245)

### `ETFTextureVariator.ETFTextureSingleton`: it extends `ETFTextureVariator`

- [ ] `ETFTextureVariator.ETFTextureSingleton.ETFTextureSingleton(1)`: its class isn't translated (it extends `ETFTextureVariator`) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:256)
- [ ] `ETFTextureVariator.ETFTextureSingleton.texture(0)`: its class isn't translated (it extends `ETFTextureVariator`) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:265)
- [ ] `ETFTextureVariator.ETFTextureSingleton.getVariantOfInternal(1)`: its class isn't translated (it extends `ETFTextureVariator`) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:270)
- [ ] `ETFTextureVariator.ETFTextureSingleton.getVanillaIdentifier(0)`: its class isn't translated (it extends `ETFTextureVariator`) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:276)
- [ ] `ETFTextureVariator.ETFTextureSingleton.getPrintout(0)`: its class isn't translated (it extends `ETFTextureVariator`) (traben/entity_texture_features/features/texture_handlers/ETFTextureVariator.java:281)

### `CancelTarget`

- [ ] `TooltipAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:10)
- [ ] `MixinMobSpawnerLogic`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:14)
- [ ] `MixinSkullBlockEntityRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:7)
- [ ] `MixinBlazeEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:14)
- [ ] `MixinBlockEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:33)
- [ ] `MixinBlockEntityRenderDispatcher`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:17)
- [ ] `MixinBlockEntityRenderState`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:20)
- [ ] `MixinEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:33)
- [ ] `MixinEntityRenderDispatcher`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:17)
- [ ] `MixinEntityRenderState`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:12)
- [ ] `MixinGlowSquidEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:14)
- [ ] `MixinMob_LeashNBTFix`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:12)
- [ ] `MixinPhantomEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:12)
- [ ] `MixinPlayerEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:15)
- [ ] `MixinPlayerEntity_DebugClick`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:18)
- [ ] `MixinSkullBlockEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:17)
- [ ] `MixinElytraFeatureRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:12)
- [ ] `MixinEyeFeatureRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:16)
- [ ] `MixinMooshroomMushroomFeatureRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:31)
- [ ] `MixinShoulderParrotFeatureRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:27)
- [ ] `MixinWardenExtraTextureParts`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:15)
- [ ] `MixinEndCrystalRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:7)
- [ ] `MixinEnderDragonEntityRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:22)
- [ ] `MixinEntityRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:18)
- [ ] `MixinIllusionerRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:7)
- [ ] `MixinLivingEntityRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:19)
- [ ] `MixinPaintingEntityRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:38)
- [ ] `MixinPlayerEntityRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:35)
- [ ] `MixinGameRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:11)
- [ ] `MixinIdentifier`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:16)
- [ ] `MixinLevelRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:11)
- [ ] `MixinModelPart`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:21)
- [ ] `MixinMultiPhase`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:16)
- [ ] `MixinPackScreen`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:29)
- [ ] `MixinPlayerSkinTexture`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:18)
- [ ] `MixinRenderLayer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:11)
- [ ] `MixinSpriteIdentifier`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:7)
- [ ] `MixinBatchableBufferSource`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:7)
- [ ] `MixinFullyBufferedMultiBufferSource`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:9)
- [ ] `MixinInnerWrappedRenderType`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:9)
- [ ] `MixinOuterWrappedRenderType`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:14)
- [ ] `MixinFullyBufferedMultiBufferSource`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:9)
- [ ] `MixinInnerWrappedRenderType`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:9)
- [ ] `MixinOuterWrappedRenderType`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:9)
- [ ] `MixinCustomizableModelPart`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:7)
- [ ] `MixinModelPartSodium`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:7)
- [ ] `MixinSodiumBufferBuilder`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:9)
- [ ] `MixinSodiumWorldRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:7)
- [ ] `MixinMinecraftClient`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:12)
- [ ] `MixinResourceReload`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:12)
- [ ] `Mixin_ChestSpecialRenderer_PassState`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:16)
- [ ] `Mixin_GuiEntityRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:18)
- [ ] `Mixin_ModelPartRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:7)
- [ ] `Mixin_ModelRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:19)
- [ ] `Mixin_ModelSubmit_AddData`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:16)
- [ ] `MixinBufferBuilder`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:16)
- [ ] `MixinSheetedDecalTextureGen`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:16)
- [ ] `MixinSpriteCoordExpander`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:16)
- [ ] `MixinVertexConsumerProvider$Immediate`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:16)
- [ ] `MixinVertexMultiConsumer$Double`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_texture_features/mixin/CancelTarget.java:7)

### `Plugin`: it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)

- [x] `Plugin.log` (traben/entity_texture_features/mixin/Plugin.java:15)
- [ ] `Plugin.onLoad(1)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_texture_features/mixin/Plugin.java:17)
- [ ] `Plugin.getRefMapperConfig(0)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_texture_features/mixin/Plugin.java:21)
- [ ] `Plugin.shouldApplyMixin(2)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_texture_features/mixin/Plugin.java:25)
- [ ] `Plugin.hasClass(1)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_texture_features/mixin/Plugin.java:31)
- [ ] `Plugin.acceptTargets(2)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_texture_features/mixin/Plugin.java:40)
- [ ] `Plugin.getMixins(0)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_texture_features/mixin/Plugin.java:43)
- [ ] `Plugin.preApply(4)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_texture_features/mixin/Plugin.java:47)
- [ ] `Plugin.postApply(4)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_texture_features/mixin/Plugin.java:50)

### `EntityBooleanLRU`: it extends `ETFLruCache`


### `EntityIntLRU`: it extends `ETFLruCache`


### `ETFEntity`: it's an interface

- [ ] `ETFEntity.etf$canBeBright(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:17)
- [ ] `ETFEntity.etf$isBlockEntity(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:19)
- [ ] `ETFEntity.etf$getType(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:21)
- [ ] `ETFEntity.etf$getUuid(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:24)
- [ ] `ETFEntity.etf$getWorld(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:26)
- [ ] `ETFEntity.etf$getBlockPos(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:28)
- [ ] `ETFEntity.etf$getOptifineId(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:30)
- [ ] `ETFEntity.etf$getOptifineVehicleId(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:32)
- [ ] `ETFEntity.etf$getBlockY(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:34)
- [ ] `ETFEntity.etf$getNbt(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:36)
- [ ] `ETFEntity.etf$hasCustomName(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:38)
- [ ] `ETFEntity.etf$getCustomName(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:40)
- [ ] `ETFEntity.etf$getScoreboardTeam(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:42)
- [ ] `ETFEntity.etf$getItemsEquipped(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:44)
- [ ] `ETFEntity.etf$getHandItems(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:46)
- [ ] `ETFEntity.etf$getArmorItems(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:48)
- [ ] `ETFEntity.etf$distanceTo(1)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:50)
- [ ] `ETFEntity.etf$getVelocity(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:52)
- [ ] `ETFEntity.etf$getPose(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:54)
- [ ] `ETFEntity.etf$getEntityKey(0)`: it has no body (traben/entity_texture_features/utils/ETFEntity.java:57)

### `ETFLruCache`: it's generic

- [ ] `ETFLruCache.ETFLruCache(0)`: its class isn't translated (it's generic) (traben/entity_texture_features/utils/ETFLruCache.java:12)
- [ ] `ETFLruCache.ETFLruCache(1)`: its class isn't translated (it's generic) (traben/entity_texture_features/utils/ETFLruCache.java:17)
- [ ] `ETFLruCache.removeEldestEntry(1)`: its class isn't translated (it's generic) (traben/entity_texture_features/utils/ETFLruCache.java:22)
- [ ] `ETFLruCache.removeEntryOnly(1)`: its class isn't translated (it's generic) (traben/entity_texture_features/utils/ETFLruCache.java:28)
- [ ] `ETFLruCache.get(1)`: its class isn't translated (it's generic) (traben/entity_texture_features/utils/ETFLruCache.java:32)
- [ ] `ETFLruCache.defaultReturnValue(1)`: its class isn't translated (it's generic) (traben/entity_texture_features/utils/ETFLruCache.java:37)

### `ETFLruCache.UUIDBoolean`: it extends `ETFLruCache`

- [ ] `ETFLruCache.UUIDBoolean.UUIDBoolean(0)`: its class isn't translated (it extends `ETFLruCache`) (traben/entity_texture_features/utils/ETFLruCache.java:42)
- [ ] `ETFLruCache.UUIDBoolean.UUIDBoolean(1)`: its class isn't translated (it extends `ETFLruCache`) (traben/entity_texture_features/utils/ETFLruCache.java:46)

### `ETFLruCache.UUIDInteger`: it extends `ETFLruCache`

- [ ] `ETFLruCache.UUIDInteger.UUIDInteger(0)`: its class isn't translated (it extends `ETFLruCache`) (traben/entity_texture_features/utils/ETFLruCache.java:53)
- [ ] `ETFLruCache.UUIDInteger.UUIDInteger(1)`: its class isn't translated (it extends `ETFLruCache`) (traben/entity_texture_features/utils/ETFLruCache.java:57)

### `ETFRenderLayerWithTexture`: it's an interface

- [ ] `ETFRenderLayerWithTexture.etf$getId(0)`: it has no body (traben/entity_texture_features/utils/ETFRenderLayerWithTexture.java:7)

### `ETFUtils2`

- [x] `ETFUtils2.FULL_BRIGHT` (traben/entity_texture_features/utils/ETFUtils2.java:38)
- [ ] `ETFUtils2.res(1)`: it calls Minecraft's `Identifier`.parse (traben/entity_texture_features/utils/ETFUtils2.java:40)
- [x] `ETFUtils2.res(2)` (traben/entity_texture_features/utils/ETFUtils2.java:45)
- [ ] `ETFUtils2.setPixel(4)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/utils/ETFUtils2.java:50)
- [ ] `ETFUtils2.getPixel(3)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/utils/ETFUtils2.java:54)
- [x] `ETFUtils2.packLight(2)` (traben/entity_texture_features/utils/ETFUtils2.java:58)
- [ ] `ETFUtils2.submitModelPart(6)`: it uses Minecraft's `SubmitNodeCollector` (traben/entity_texture_features/utils/ETFUtils2.java:62)
- [ ] `ETFUtils2.submitModelPart(7)`: it uses Minecraft's `SubmitNodeCollector` (traben/entity_texture_features/utils/ETFUtils2.java:71)
- [ ] `ETFUtils2.submitEmissiveModelPart(4)`: it uses Minecraft's `SubmitNodeCollector` (traben/entity_texture_features/utils/ETFUtils2.java:86)
- [ ] `ETFUtils2.submitEnchantedModelPart(5)`: it uses Minecraft's `SubmitNodeCollector` (traben/entity_texture_features/utils/ETFUtils2.java:90)
- [x] `ETFUtils2.optifineHashing(1)` (traben/entity_texture_features/utils/ETFUtils2.java:94)
- [ ] `ETFUtils2.printDebugImage(1)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/utils/ETFUtils2.java:102)
- [ ] `ETFUtils2.getETFVariantNotNullForInjector(1)`: it calls ETFState.isStateActive, which isn't translated (traben/entity_texture_features/utils/ETFUtils2.java:116)
- [ ] `ETFUtils2.renderEmissive(3)`: it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/utils/ETFUtils2.java:130)
- [ ] `ETFUtils2.getFoilBuffer(2)`: it uses Minecraft's `VertexConsumer` (traben/entity_texture_features/utils/ETFUtils2.java:151)
- [ ] `ETFUtils2.renderEnchanted(4)`: it uses ETFTexture, which isn't translated (it uses ETFSprite, which isn't translated (it uses Minecraft's `TextureAtlasSprite`)) (traben/entity_texture_features/utils/ETFUtils2.java:155)
- [ ] `ETFUtils2.addVariantNumberSuffix(2)`: it's overloaded with the same number of parameters (traben/entity_texture_features/utils/ETFUtils2.java:170)
- [ ] `ETFUtils2.addVariantNumberSuffix(2)`: it's overloaded with the same number of parameters (traben/entity_texture_features/utils/ETFUtils2.java:176)
- [ ] `ETFUtils2.replaceIdentifier(3)`: a try/catch (IdentifierException, Exception) (traben/entity_texture_features/utils/ETFUtils2.java:188)
- [ ] `ETFUtils2.returnNameOfHighestPackFromTheseMultiple(1)`: it calls ETFManager.getInstance, which isn't translated (traben/entity_texture_features/utils/ETFUtils2.java:204)
- [ ] `ETFUtils2.returnNameOfHighestPackFromTheseTwo(2)`: String.equals with 1 values (traben/entity_texture_features/utils/ETFUtils2.java:216)
- [ ] `ETFUtils2.readAndReturnPropertiesElseNull(1)`: a try-with-resources over something other than a file reader or writer (traben/entity_texture_features/utils/ETFUtils2.java:229)
- [ ] `ETFUtils2.readAndReturnAllLayeredPropertiesElseNull(1)`: it calls Minecraft's Minecraft.getResourceManager (no Horizon helper for it yet) (traben/entity_texture_features/utils/ETFUtils2.java:241)
- [ ] `ETFUtils2.getNativeImageElseNull(1)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/utils/ETFUtils2.java:265)
- [ ] `ETFUtils2.logMessage(1)`: it calls ETFUtils2.logMessage, which isn't translated (traben/entity_texture_features/utils/ETFUtils2.java:291)
- [ ] `ETFUtils2.logMessage(2)`: it creates Minecraft's `LiteralContents` (traben/entity_texture_features/utils/ETFUtils2.java:295)
- [ ] `ETFUtils2.logWarn(1)`: it calls ETFUtils2.logWarn, which isn't translated (traben/entity_texture_features/utils/ETFUtils2.java:308)
- [ ] `ETFUtils2.logWarn(2)`: it creates Minecraft's `LiteralContents` (traben/entity_texture_features/utils/ETFUtils2.java:312)
- [ ] `ETFUtils2.logError(1)`: it calls ETFUtils2.logError, which isn't translated (traben/entity_texture_features/utils/ETFUtils2.java:327)
- [ ] `ETFUtils2.logError(2)`: it creates Minecraft's `LiteralContents` (traben/entity_texture_features/utils/ETFUtils2.java:331)
- [ ] `ETFUtils2.emptyNativeImage(0)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/utils/ETFUtils2.java:347)
- [ ] `ETFUtils2.emptyNativeImage(2)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/utils/ETFUtils2.java:351)
- [ ] `ETFUtils2.registerNativeImageToIdentifier(2)`: it uses Minecraft's `NativeImage` (traben/entity_texture_features/utils/ETFUtils2.java:357)
- [ ] `ETFUtils2.checkModCompatibility(0)`: it calls ETFConfigWarnings.getRegisteredWarnings, which isn't translated (traben/entity_texture_features/utils/ETFUtils2.java:382)

### `ETFUtils2.RenderMethodForOverlay`: it's an interface

- [ ] `ETFUtils2.RenderMethodForOverlay.render(2)`: it has no body (traben/entity_texture_features/utils/ETFUtils2.java:389)

### `ETFVertexConsumer`: it's an interface

- [ ] `ETFVertexConsumer.etf$getETFTexture(0)`: it has no body (traben/entity_texture_features/utils/ETFVertexConsumer.java:8)
- [ ] `ETFVertexConsumer.etf$getProvider(0)`: it has no body (traben/entity_texture_features/utils/ETFVertexConsumer.java:11)
- [ ] `ETFVertexConsumer.etf$getRenderLayer(0)`: it has no body (traben/entity_texture_features/utils/ETFVertexConsumer.java:14)
- [ ] `ETFVertexConsumer.etf$initETFVertexConsumer(2)`: it has no body (traben/entity_texture_features/utils/ETFVertexConsumer.java:17)

### `UEntityTypes`

- [ ] `UEntityTypes.PARROT`: it uses Minecraft's `EntityType` (traben/entity_texture_features/utils/UEntityTypes.java:7)
- [ ] `UEntityTypes.PLAYER`: it uses Minecraft's `EntityType` (traben/entity_texture_features/utils/UEntityTypes.java:8)

### `URenderTypeToVertexConsumer`: it uses Minecraft's `RenderTypeFeatureRenderer`

- [ ] `URenderTypeToVertexConsumer.URenderTypeToVertexConsumer(1)`: its class isn't translated (it uses Minecraft's `RenderTypeFeatureRenderer`) (traben/entity_texture_features/utils/URenderTypeToVertexConsumer.java:10)
- [ ] `URenderTypeToVertexConsumer.getBuffer(1)`: its class isn't translated (it uses Minecraft's `RenderTypeFeatureRenderer`) (traben/entity_texture_features/utils/URenderTypeToVertexConsumer.java:14)

### `UScreen`

- [ ] `UScreen.currentScreen(0)`: it uses Minecraft's `Screen` (traben/entity_texture_features/utils/UScreen.java:7)
- [ ] `UScreen.setScreen(1)`: it uses Minecraft's `Screen` (traben/entity_texture_features/utils/UScreen.java:11)

### `TConfigEntry`: it extends `TConfigEntryForList`

- [x] `TConfigEntry.CHANGED_COLOR` (traben/tconfig/gui/entries/TConfigEntry.java:11)
- [ ] `TConfigEntry.TConfigEntry(2)`: its class isn't translated (it extends `TConfigEntryForList`) (traben/tconfig/gui/entries/TConfigEntry.java:15)
- [ ] `TConfigEntry.TConfigEntry(1)`: its class isn't translated (it extends `TConfigEntryForList`) (traben/tconfig/gui/entries/TConfigEntry.java:20)
- [ ] `TConfigEntry.getText(0)`: its class isn't translated (it extends `TConfigEntryForList`) (traben/tconfig/gui/entries/TConfigEntry.java:24)
- [ ] `TConfigEntry.getTooltip(0)`: its class isn't translated (it extends `TConfigEntryForList`) (traben/tconfig/gui/entries/TConfigEntry.java:28)
- [ ] `TConfigEntry.setEnabled(1)`: its class isn't translated (it extends `TConfigEntryForList`) (traben/tconfig/gui/entries/TConfigEntry.java:32)
- [ ] `TConfigEntry.hasChangedFromInitial(0)`: it has no body (traben/tconfig/gui/entries/TConfigEntry.java:38)
- [ ] `TConfigEntry.saveValuesToConfig(0)`: it has no body (traben/tconfig/gui/entries/TConfigEntry.java:40)
- [ ] `TConfigEntry.setValuesToDefault(0)`: it has no body (traben/tconfig/gui/entries/TConfigEntry.java:42)
- [ ] `TConfigEntry.resetValuesToInitial(0)`: it has no body (traben/tconfig/gui/entries/TConfigEntry.java:44)

### `TConfigEntry.Empty`: it extends `TConfigEntry`

- [ ] `TConfigEntry.Empty.Empty(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntry.java:47)
- [ ] `TConfigEntry.Empty.getWidget(4)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntry.java:51)
- [ ] `TConfigEntry.Empty.hasChangedFromInitial(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntry.java:56)
- [ ] `TConfigEntry.Empty.saveValuesToConfig(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntry.java:61)
- [ ] `TConfigEntry.Empty.setValuesToDefault(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntry.java:66)
- [ ] `TConfigEntry.Empty.resetValuesToInitial(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntry.java:70)

### `TConfigEntryBoolean`: it extends `TConfigEntryValue`

- [ ] `TConfigEntryBoolean.TConfigEntryBoolean(5)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryBoolean.java:17)
- [ ] `TConfigEntryBoolean.TConfigEntryBoolean(4)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryBoolean.java:24)
- [ ] `TConfigEntryBoolean.setType(1)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryBoolean.java:28)
- [ ] `TConfigEntryBoolean.getValueFromWidget(0)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryBoolean.java:33)
- [ ] `TConfigEntryBoolean.getWidget(4)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryBoolean.java:37)
- [ ] `TConfigEntryBoolean.setWidgetToDefaultValue(0)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryBoolean.java:43)
- [ ] `TConfigEntryBoolean.resetWidgetToInitialValue(0)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryBoolean.java:49)

### `TConfigEntryBoolean.BooleanButtonWidget`: it extends Minecraft's `Plain`

- [ ] `TConfigEntryBoolean.BooleanButtonWidget.BooleanButtonWidget(7)`: its class isn't translated (it extends Minecraft's `Plain`) (traben/tconfig/gui/entries/TConfigEntryBoolean.java:60)
- [ ] `TConfigEntryBoolean.BooleanButtonWidget.updateMessage(0)`: its class isn't translated (it extends Minecraft's `Plain`) (traben/tconfig/gui/entries/TConfigEntryBoolean.java:70)
- [ ] `TConfigEntryBoolean.BooleanButtonWidget.onPress(1)`: its class isn't translated (it extends Minecraft's `Plain`) (traben/tconfig/gui/entries/TConfigEntryBoolean.java:74)

### `TConfigEntryBoolean.Type`: its constants carry values

- [ ] `TConfigEntryBoolean.Type.Type(2)`: its class isn't translated (its constants carry values) (traben/tconfig/gui/entries/TConfigEntryBoolean.java:87)
- [ ] `TConfigEntryBoolean.Type.get(1)`: its class isn't translated (its constants carry values) (traben/tconfig/gui/entries/TConfigEntryBoolean.java:92)

### `TConfigEntryCategory`: it extends `TConfigEntry`

- [ ] `TConfigEntryCategory.TConfigEntryCategory(2)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:25)
- [ ] `TConfigEntryCategory.TConfigEntryCategory(1)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:30)
- [ ] `TConfigEntryCategory.setAlign(1)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:35)
- [ ] `TConfigEntryCategory.getOptions(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:39)
- [ ] `TConfigEntryCategory.getScreen(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:43)
- [ ] `TConfigEntryCategory.getWidget(4)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:62)
- [ ] `TConfigEntryCategory.saveValuesToConfig(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:67)
- [ ] `TConfigEntryCategory.setWidgetBackgroundToFullWidth(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:78)
- [ ] `TConfigEntryCategory.setValuesToDefault(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:82)
- [ ] `TConfigEntryCategory.resetValuesToInitial(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:89)
- [ ] `TConfigEntryCategory.add(1)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:96)
- [ ] `TConfigEntryCategory.addAll(1)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:104)
- [ ] `TConfigEntryCategory.add(1)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:112)
- [ ] `TConfigEntryCategory.addOrMerge(1)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:123)
- [ ] `TConfigEntryCategory.hasChangedFromInitial(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:134)
- [ ] `TConfigEntryCategory.setEmptyTooltip(1)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:139)
- [ ] `TConfigEntryCategory.setRenderFeature(1)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:144)

### `TConfigEntryCategory.CategoryButton`: it extends Minecraft's `Plain`

- [ ] `TConfigEntryCategory.CategoryButton.CategoryButton(6)`: its class isn't translated (it extends Minecraft's `Plain`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:149)
- [ ] `TConfigEntryCategory.CategoryButton.getMessage(0)`: its class isn't translated (it extends Minecraft's `Plain`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:157)

### `TConfigEntryCategory.Empty`: it extends `TConfigEntryCategory`

- [ ] `TConfigEntryCategory.Empty.Empty(0)`: its class isn't translated (it extends `TConfigEntryCategory`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:164)
- [ ] `TConfigEntryCategory.Empty.getWidget(4)`: its class isn't translated (it extends `TConfigEntryCategory`) (traben/tconfig/gui/entries/TConfigEntryCategory.java:168)

### `TConfigEntryCustomButton`: it extends `TConfigEntry`

- [ ] `TConfigEntryCustomButton.TConfigEntryCustomButton(3)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomButton.java:11)
- [ ] `TConfigEntryCustomButton.TConfigEntryCustomButton(2)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomButton.java:16)
- [ ] `TConfigEntryCustomButton.getWidget(4)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomButton.java:20)
- [ ] `TConfigEntryCustomButton.saveValuesToConfig(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomButton.java:26)
- [ ] `TConfigEntryCustomButton.setValuesToDefault(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomButton.java:31)
- [ ] `TConfigEntryCustomButton.resetValuesToInitial(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomButton.java:35)
- [ ] `TConfigEntryCustomButton.hasChangedFromInitial(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomButton.java:39)

### `TConfigEntryCustomScreenOpener`: it extends `TConfigEntry`

- [ ] `TConfigEntryCustomScreenOpener.TConfigEntryCustomScreenOpener(7)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomScreenOpener.java:19)
- [ ] `TConfigEntryCustomScreenOpener.TConfigEntryCustomScreenOpener(6)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomScreenOpener.java:37)
- [ ] `TConfigEntryCustomScreenOpener.TConfigEntryCustomScreenOpener(4)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomScreenOpener.java:48)
- [ ] `TConfigEntryCustomScreenOpener.TConfigEntryCustomScreenOpener(3)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomScreenOpener.java:52)
- [ ] `TConfigEntryCustomScreenOpener.getScreen(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomScreenOpener.java:56)
- [ ] `TConfigEntryCustomScreenOpener.getWidget(4)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomScreenOpener.java:68)
- [ ] `TConfigEntryCustomScreenOpener.saveValuesToConfig(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomScreenOpener.java:74)
- [ ] `TConfigEntryCustomScreenOpener.setValuesToDefault(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomScreenOpener.java:79)
- [ ] `TConfigEntryCustomScreenOpener.resetValuesToInitial(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomScreenOpener.java:84)
- [ ] `TConfigEntryCustomScreenOpener.hasChangedFromInitial(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryCustomScreenOpener.java:89)

### `TConfigEntryEnumButton`: it's generic

- [ ] `TConfigEntryEnumButton.TConfigEntryEnumButton(6)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:19)
- [ ] `TConfigEntryEnumButton.TConfigEntryEnumButton(5)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:30)
- [ ] `TConfigEntryEnumButton.TConfigEntryEnumButton(5)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:34)
- [ ] `TConfigEntryEnumButton.TConfigEntryEnumButton(4)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:38)
- [ ] `TConfigEntryEnumButton.allowNullValue(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:42)
- [ ] `TConfigEntryEnumButton.getValueFromWidget(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:48)
- [ ] `TConfigEntryEnumButton.getWidget(4)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:52)
- [ ] `TConfigEntryEnumButton.setWidgetToDefaultValue(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:58)
- [ ] `TConfigEntryEnumButton.resetWidgetToInitialValue(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:63)

### `TConfigEntryEnumButton.EnumButtonWidget`: it's generic

- [ ] `TConfigEntryEnumButton.EnumButtonWidget.EnumButtonWidget(4)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:73)
- [ ] `TConfigEntryEnumButton.EnumButtonWidget.getIndex(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:81)
- [ ] `TConfigEntryEnumButton.EnumButtonWidget.getChoiceCount(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:85)
- [ ] `TConfigEntryEnumButton.EnumButtonWidget.getValue(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:89)
- [ ] `TConfigEntryEnumButton.EnumButtonWidget.setValue(1)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:94)
- [ ] `TConfigEntryEnumButton.EnumButtonWidget.updateMessage(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:99)
- [ ] `TConfigEntryEnumButton.EnumButtonWidget.onPress(1)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumButton.java:104)

### `TConfigEntryEnumSlider`: it's generic

- [ ] `TConfigEntryEnumSlider.TConfigEntryEnumSlider(6)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:17)
- [ ] `TConfigEntryEnumSlider.TConfigEntryEnumSlider(5)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:28)
- [ ] `TConfigEntryEnumSlider.TConfigEntryEnumSlider(4)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:32)
- [ ] `TConfigEntryEnumSlider.TConfigEntryEnumSlider(5)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:36)
- [ ] `TConfigEntryEnumSlider.allowNullValue(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:40)
- [ ] `TConfigEntryEnumSlider.getValueFromWidget(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:46)
- [ ] `TConfigEntryEnumSlider.getWidget(4)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:50)
- [ ] `TConfigEntryEnumSlider.setWidgetToDefaultValue(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:56)
- [ ] `TConfigEntryEnumSlider.resetWidgetToInitialValue(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:61)

### `TConfigEntryEnumSlider.EnumSliderWidget`: it's generic

- [ ] `TConfigEntryEnumSlider.EnumSliderWidget.EnumSliderWidget(4)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:70)
- [ ] `TConfigEntryEnumSlider.EnumSliderWidget.getValue(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:78)
- [ ] `TConfigEntryEnumSlider.EnumSliderWidget.setValue(1)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:83)
- [ ] `TConfigEntryEnumSlider.EnumSliderWidget.updateMessage(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:88)
- [ ] `TConfigEntryEnumSlider.EnumSliderWidget.applyValue(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:96)
- [ ] `TConfigEntryEnumSlider.EnumSliderWidget.getChoiceCount(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:99)
- [ ] `TConfigEntryEnumSlider.EnumSliderWidget.getIndex(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryEnumSlider.java:103)

### `TConfigEntryInt`: it extends `TConfigEntryValue`

- [ ] `TConfigEntryInt.TConfigEntryInt(9)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryInt.java:17)
- [ ] `TConfigEntryInt.TConfigEntryInt(7)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryInt.java:32)
- [ ] `TConfigEntryInt.TConfigEntryInt(8)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryInt.java:39)
- [ ] `TConfigEntryInt.TConfigEntryInt(6)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryInt.java:45)
- [ ] `TConfigEntryInt.getValueFromWidget(0)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryInt.java:49)
- [ ] `TConfigEntryInt.getWidget(4)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryInt.java:53)
- [ ] `TConfigEntryInt.setWidgetToDefaultValue(0)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryInt.java:59)
- [ ] `TConfigEntryInt.resetWidgetToInitialValue(0)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryInt.java:64)
- [ ] `TConfigEntryInt.dontModifyOffMaxValues(0)`: its class isn't translated (it extends `TConfigEntryValue`) (traben/tconfig/gui/entries/TConfigEntryInt.java:69)

### `TConfigEntryInt.IntSliderWidget`: it extends Minecraft's `AbstractSliderButton`

- [ ] `TConfigEntryInt.IntSliderWidget.IntSliderWidget(7)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (traben/tconfig/gui/entries/TConfigEntryInt.java:82)
- [ ] `TConfigEntryInt.IntSliderWidget.isOff(0)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (traben/tconfig/gui/entries/TConfigEntryInt.java:94)
- [ ] `TConfigEntryInt.IntSliderWidget.setValue(1)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (traben/tconfig/gui/entries/TConfigEntryInt.java:98)
- [ ] `TConfigEntryInt.IntSliderWidget.updateMessage(0)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (traben/tconfig/gui/entries/TConfigEntryInt.java:103)
- [ ] `TConfigEntryInt.IntSliderWidget.applyValue(0)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (traben/tconfig/gui/entries/TConfigEntryInt.java:114)
- [ ] `TConfigEntryInt.IntSliderWidget.snapValueToNearestIndex(0)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (traben/tconfig/gui/entries/TConfigEntryInt.java:117)
- [ ] `TConfigEntryInt.IntSliderWidget.getValueRoundedToIntBetweenMinMax(0)`: its class isn't translated (it extends Minecraft's `AbstractSliderButton`) (traben/tconfig/gui/entries/TConfigEntryInt.java:121)

### `TConfigEntryNullSafe`: it's generic

- [ ] `TConfigEntryNullSafe.TConfigEntryNullSafe(5)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryNullSafe.java:7)
- [ ] `TConfigEntryNullSafe.hasChangedFromInitial(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryNullSafe.java:11)
- [ ] `TConfigEntryNullSafe.allowNullValue(0)`: it has no body (traben/tconfig/gui/entries/TConfigEntryNullSafe.java:16)

### `TConfigEntryText`: it extends `TConfigEntry`

- [ ] `TConfigEntryText.TConfigEntryText(2)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryText.java:19)
- [ ] `TConfigEntryText.TConfigEntryText(1)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryText.java:24)
- [ ] `TConfigEntryText.fromLongOrMultilineTranslation(2)`: it uses TConfigEntry, which isn't translated (it extends `TConfigEntryForList`) (traben/tconfig/gui/entries/TConfigEntryText.java:28)
- [ ] `TConfigEntryText.fromLongOrMultilineTranslation(3)`: it uses TConfigEntry, which isn't translated (it extends `TConfigEntryForList`) (traben/tconfig/gui/entries/TConfigEntryText.java:32)
- [ ] `TConfigEntryText.getWidget(4)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryText.java:54)
- [ ] `TConfigEntryText.hasChangedFromInitial(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryText.java:60)
- [ ] `TConfigEntryText.saveValuesToConfig(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryText.java:65)
- [ ] `TConfigEntryText.setValuesToDefault(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryText.java:70)
- [ ] `TConfigEntryText.resetValuesToInitial(0)`: its class isn't translated (it extends `TConfigEntry`) (traben/tconfig/gui/entries/TConfigEntryText.java:74)

### `TConfigEntryText.TextAlignment`

- [ ] `TConfigEntryText.TextAlignment.align(1)`: it uses Minecraft's `StringWidget` (traben/tconfig/gui/entries/TConfigEntryText.java:84)

### `TConfigEntryText.TwoLines`: it extends `TConfigEntryText`

- [ ] `TConfigEntryText.TwoLines.TwoLines(2)`: its class isn't translated (it extends `TConfigEntryText`) (traben/tconfig/gui/entries/TConfigEntryText.java:91)
- [ ] `TConfigEntryText.TwoLines.TwoLines(3)`: its class isn't translated (it extends `TConfigEntryText`) (traben/tconfig/gui/entries/TConfigEntryText.java:95)
- [ ] `TConfigEntryText.TwoLines.getWidget(4)`: its class isn't translated (it extends `TConfigEntryText`) (traben/tconfig/gui/entries/TConfigEntryText.java:101)
- [ ] `TConfigEntryText.TwoLines.extractContent(5)`: its class isn't translated (it extends `TConfigEntryText`) (traben/tconfig/gui/entries/TConfigEntryText.java:108)

### `TConfigEntryValue`: it's generic

- [ ] `TConfigEntryValue.TConfigEntryValue(5)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryValue.java:12)
- [ ] `TConfigEntryValue.TConfigEntryValue(4)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryValue.java:19)
- [ ] `TConfigEntryValue.saveValuesToConfig(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryValue.java:26)
- [ ] `TConfigEntryValue.getValueFromWidget(0)`: it has no body (traben/tconfig/gui/entries/TConfigEntryValue.java:36)
- [ ] `TConfigEntryValue.setValuesToDefault(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryValue.java:38)
- [ ] `TConfigEntryValue.setWidgetToDefaultValue(0)`: it has no body (traben/tconfig/gui/entries/TConfigEntryValue.java:43)
- [ ] `TConfigEntryValue.resetValuesToInitial(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryValue.java:45)
- [ ] `TConfigEntryValue.resetWidgetToInitialValue(0)`: it has no body (traben/tconfig/gui/entries/TConfigEntryValue.java:50)
- [ ] `TConfigEntryValue.hasChangedFromInitial(0)`: its class isn't translated (it's generic) (traben/tconfig/gui/entries/TConfigEntryValue.java:52)

### `TConfigEntryListWidget`: it extends Minecraft's `AbstractSelectionList`

- [ ] `TConfigEntryListWidget.TConfigEntryListWidget(6)`: its class isn't translated (it extends Minecraft's `AbstractSelectionList`) (traben/tconfig/gui/TConfigEntryListWidget.java:18)
- [ ] `TConfigEntryListWidget.getRowWidth(0)`: its class isn't translated (it extends Minecraft's `AbstractSelectionList`) (traben/tconfig/gui/TConfigEntryListWidget.java:30)
- [ ] `TConfigEntryListWidget.scrollBarX(0)`: its class isn't translated (it extends Minecraft's `AbstractSelectionList`) (traben/tconfig/gui/TConfigEntryListWidget.java:34)
- [ ] `TConfigEntryListWidget.updateWidgetNarration(1)`: its class isn't translated (it extends Minecraft's `AbstractSelectionList`) (traben/tconfig/gui/TConfigEntryListWidget.java:38)
- [ ] `TConfigEntryListWidget.isValidClickButton(1)`: its class isn't translated (it extends Minecraft's `AbstractSelectionList`) (traben/tconfig/gui/TConfigEntryListWidget.java:41)
- [ ] `TConfigEntryListWidget.setSelected(1)`: its class isn't translated (it extends Minecraft's `AbstractSelectionList`) (traben/tconfig/gui/TConfigEntryListWidget.java:45)
- [ ] `TConfigEntryListWidget.setWidgetBackgroundToFullWidth(0)`: its class isn't translated (it extends Minecraft's `AbstractSelectionList`) (traben/tconfig/gui/TConfigEntryListWidget.java:48)
- [ ] `TConfigEntryListWidget.extractListBackground(1)`: its body couldn't be read: an assert (traben/tconfig/gui/TConfigEntryListWidget.java:52)
- [ ] `TConfigEntryListWidget.extractListSeparators(1)`: its body couldn't be read: an assert (traben/tconfig/gui/TConfigEntryListWidget.java:67)

### `TConfigEntryListWidget.TConfigEntryForList`: it extends Minecraft's `Entry`

- [ ] `TConfigEntryListWidget.TConfigEntryForList.extractContent(5)`: its class isn't translated (it extends Minecraft's `Entry`) (traben/tconfig/gui/TConfigEntryListWidget.java:86)
- [ ] `TConfigEntryListWidget.TConfigEntryForList.getWidget(4)`: it has no body (traben/tconfig/gui/TConfigEntryListWidget.java:93)
- [ ] `TConfigEntryListWidget.TConfigEntryForList.ignoreMouseAt(2)`: its class isn't translated (it extends Minecraft's `Entry`) (traben/tconfig/gui/TConfigEntryListWidget.java:95)
- [ ] `TConfigEntryListWidget.TConfigEntryForList.mouseClicked(2)`: its class isn't translated (it extends Minecraft's `Entry`) (traben/tconfig/gui/TConfigEntryListWidget.java:99)
- [ ] `TConfigEntryListWidget.TConfigEntryForList.mouseDragged(3)`: its class isn't translated (it extends Minecraft's `Entry`) (traben/tconfig/gui/TConfigEntryListWidget.java:103)
- [ ] `TConfigEntryListWidget.TConfigEntryForList.mouseReleased(1)`: its class isn't translated (it extends Minecraft's `Entry`) (traben/tconfig/gui/TConfigEntryListWidget.java:107)
- [ ] `TConfigEntryListWidget.TConfigEntryForList.setFocused(1)`: its class isn't translated (it extends Minecraft's `Entry`) (traben/tconfig/gui/TConfigEntryListWidget.java:111)

### `TConfigScreen`: it extends Minecraft's `Screen`

- [ ] `TConfigScreen.TConfigScreen(3)`: its class isn't translated (it extends Minecraft's `Screen`) (traben/tconfig/gui/TConfigScreen.java:18)
- [ ] `TConfigScreen.getBackButtonText(0)`: its class isn't translated (it extends Minecraft's `Screen`) (traben/tconfig/gui/TConfigScreen.java:24)
- [ ] `TConfigScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (traben/tconfig/gui/TConfigScreen.java:28)
- [ ] `TConfigScreen.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (traben/tconfig/gui/TConfigScreen.java:52)
- [ ] `TConfigScreen.shouldCloseOnEsc(0)`: its class isn't translated (it extends Minecraft's `Screen`) (traben/tconfig/gui/TConfigScreen.java:57)
- [ ] `TConfigScreen.onClose(0)`: its class isn't translated (it extends Minecraft's `Screen`) (traben/tconfig/gui/TConfigScreen.java:61)

### `TConfigScreenList`: it extends `TConfigScreen`

- [ ] `TConfigScreenList.TConfigScreenList(6)`: its class isn't translated (it extends `TConfigScreen`) (traben/tconfig/gui/TConfigScreenList.java:22)
- [ ] `TConfigScreenList.TConfigScreenList(5)`: its class isn't translated (it extends `TConfigScreen`) (traben/tconfig/gui/TConfigScreenList.java:40)
- [ ] `TConfigScreenList.setRenderFeature(1)`: its class isn't translated (it extends `TConfigScreen`) (traben/tconfig/gui/TConfigScreenList.java:44)
- [ ] `TConfigScreenList.extractRenderState(4)`: its class isn't translated (it extends `TConfigScreen`) (traben/tconfig/gui/TConfigScreenList.java:48)
- [ ] `TConfigScreenList.setWidgetBackgroundToFullWidth(0)`: its class isn't translated (it extends `TConfigScreen`) (traben/tconfig/gui/TConfigScreenList.java:56)
- [ ] `TConfigScreenList.init(0)`: its class isn't translated (it extends `TConfigScreen`) (traben/tconfig/gui/TConfigScreenList.java:60)
- [ ] `TConfigScreenList.updateSearch(1)`: its body couldn't be read: an assert (traben/tconfig/gui/TConfigScreenList.java:74)
- [ ] `TConfigScreenList.initList(1)`: its class isn't translated (it extends `TConfigScreen`) (traben/tconfig/gui/TConfigScreenList.java:86)

### `TConfigScreenList.Align`


### `TConfigScreenList.Renderable`: it's an interface

- [ ] `TConfigScreenList.Renderable.render(3)`: it has no body (traben/tconfig/gui/TConfigScreenList.java:120)

### `TConfigScreenMain`: it extends `TConfigScreen`

- [ ] `TConfigScreenMain.TConfigScreenMain(4)`: its class isn't translated (it extends `TConfigScreen`) (traben/tconfig/gui/TConfigScreenMain.java:24)
- [ ] `TConfigScreenMain.getBackButtonText(0)`: its class isn't translated (it extends `TConfigScreen`) (traben/tconfig/gui/TConfigScreenMain.java:33)
- [ ] `TConfigScreenMain.initConfigs(0)`: its class isn't translated (it extends `TConfigScreen`) (traben/tconfig/gui/TConfigScreenMain.java:38)
- [ ] `TConfigScreenMain.onClose(0)`: its class isn't translated (it extends `TConfigScreen`) (traben/tconfig/gui/TConfigScreenMain.java:52)
- [ ] `TConfigScreenMain.init(0)`: its class isn't translated (it extends `TConfigScreen`) (traben/tconfig/gui/TConfigScreenMain.java:62)
- [ ] `TConfigScreenMain.extractRenderState(4)`: its class isn't translated (it extends `TConfigScreen`) (traben/tconfig/gui/TConfigScreenMain.java:79)

### `TConfig`

- [ ] `TConfig.getGUIOptions(0)`: it has no body (traben/tconfig/TConfig.java:7)
- [ ] `TConfig.getModIcon(0)`: it has no body (traben/tconfig/TConfig.java:9)
- [x] `TConfig.doesGUI(0)` (traben/tconfig/TConfig.java:11)

### `TConfig.NoGUI`: it extends `TConfig`

- [ ] `TConfig.NoGUI.getGUIOptions(0)`: its class isn't translated (it extends `TConfig`) (traben/tconfig/TConfig.java:16)
- [ ] `TConfig.NoGUI.getModIcon(0)`: its class isn't translated (it extends `TConfig`) (traben/tconfig/TConfig.java:21)
- [ ] `TConfig.NoGUI.doesGUI(0)`: its class isn't translated (it extends `TConfig`) (traben/tconfig/TConfig.java:26)

### `TConfigHandler`: it's generic

- [ ] `TConfigHandler.TConfigHandler(3)`: its class isn't translated (it's generic) (traben/tconfig/TConfigHandler.java:23)
- [ ] `TConfigHandler.getConfig(0)`: its class isn't translated (it's generic) (traben/tconfig/TConfigHandler.java:31)
- [ ] `TConfigHandler.setConfig(1)`: its class isn't translated (it's generic) (traben/tconfig/TConfigHandler.java:39)
- [ ] `TConfigHandler.equals(1)`: its class isn't translated (it's generic) (traben/tconfig/TConfigHandler.java:43)
- [ ] `TConfigHandler.configEquals(1)`: its class isn't translated (it's generic) (traben/tconfig/TConfigHandler.java:55)
- [ ] `TConfigHandler.hashCode(0)`: its class isn't translated (it's generic) (traben/tconfig/TConfigHandler.java:63)
- [ ] `TConfigHandler.saveToFile(0)`: its class isn't translated (it's generic) (traben/tconfig/TConfigHandler.java:68)
- [ ] `TConfigHandler.toJson(0)`: its class isn't translated (it's generic) (traben/tconfig/TConfigHandler.java:84)
- [ ] `TConfigHandler.toJson(1)`: its class isn't translated (it's generic) (traben/tconfig/TConfigHandler.java:88)
- [ ] `TConfigHandler.loadFromFile(0)`: its class isn't translated (it's generic) (traben/tconfig/TConfigHandler.java:92)
- [ ] `TConfigHandler.loadFromFile(1)`: its class isn't translated (it's generic) (traben/tconfig/TConfigHandler.java:96)
- [ ] `TConfigHandler.fromJson(1)`: its class isn't translated (it's generic) (traben/tconfig/TConfigHandler.java:127)
- [ ] `TConfigHandler.fromJson(1)`: its class isn't translated (it's generic) (traben/tconfig/TConfigHandler.java:132)
- [ ] `TConfigHandler.copyOfConfig(0)`: its class isn't translated (it's generic) (traben/tconfig/TConfigHandler.java:137)
- [ ] `TConfigHandler.doesGUI(0)`: its class isn't translated (it's generic) (traben/tconfig/TConfigHandler.java:141)

### `TConfigLog`

- [x] `TConfigLog.LOGGER` (traben/tconfig/TConfigLog.java:7)
- [x] `TConfigLog.log(2)` (traben/tconfig/TConfigLog.java:9)
- [x] `TConfigLog.logError(2)` (traben/tconfig/TConfigLog.java:13)
- [x] `TConfigLog.logWarn(2)` (traben/tconfig/TConfigLog.java:17)
