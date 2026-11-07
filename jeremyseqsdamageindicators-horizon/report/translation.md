# Java translated to Rust

Translated 4 of 48 methods and 25 of 28 static fields to Rust; 1 of 3 Fabric event handlers; config class DamageIndicatorsConfig (7 on/off options). It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 19 | `DamageIndicatorsConfig.load(0)`: it calls DamageIndicatorsConfig.load, which isn't translated |
| Uses other libraries | 12 | `DamageIndicatorsConfig.getDirectionalIndicatorColor(0)`: it uses `Color` (java.awt) |
| Uses Minecraft classes (needs Horizon's game API) | 7 | `DamageTextRenderer.render(2)`: it uses Minecraft's `SubmitNodeCollector` |
| Other Java this translator doesn't handle yet | 2 | `DamageIndicatorsConfig.save(0)`: unknown static DamageIndicators.LOGGER |
| Uses Fabric API | 2 | `FabricClientEvents.register(0)`: it uses Fabric's `LevelRenderEvents`.COLLECT_SUBMITS |
| Calls the game where Horizon has no helper yet | 1 | `DamageHandler.onClientTick(0)`: it calls Minecraft's LocalPlayer.getHealth (no Horizon helper for it yet) |
| Uses files (java.nio / java.io) | 1 | `DamageIndicatorsConfig.reloadIfChanged(0)`: it calls `Files` (java.nio.file).getLastModifiedTime |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| dev/jeremyseq/fabric/FabricClientEvents.java:18 | `hud` | `on_hud_0`, called from the hook, without: the call to `IndicatorOverlay.render(..)`: it calls IndicatorOverlay.render, which isn't translated |
| dev/jeremyseq/fabric/FabricClientTickHandler.java:8 | `tick` | not translated: it calls ClientTickHandler.tick, which isn't translated |
| dev/jeremyseq/fabric/DamageIndicators.java:6 | `init` | not translated: unknown name `dev` |

## Every class


### `PlatformMethods`

- [x] `PlatformMethods.getCurrentTarget(0)` (architectury_inject_jeremyseqsdamageindicators_common_5caaefe61f3049e5878994c40a937c70_b58446b0181d852d654ff8c157759ad48016b6fe8fa8b22304414a21f0766539jdamageindicatorscommon263230devjar/PlatformMethods.java:4)

### `DamageIndicatorsConfig`

- [x] `DamageIndicatorsConfig.CONFIG_PATH` (dev/jeremyseq/config/DamageIndicatorsConfig.java:11)
- [x] `DamageIndicatorsConfig.INSTANCE` (dev/jeremyseq/config/DamageIndicatorsConfig.java:12)
- [x] `DamageIndicatorsConfig.lastModifiedTime` (dev/jeremyseq/config/DamageIndicatorsConfig.java:32)
- [ ] `DamageIndicatorsConfig.getDirectionalIndicatorColor(0)`: it uses `Color` (java.awt) (dev/jeremyseq/config/DamageIndicatorsConfig.java:34)
- [ ] `DamageIndicatorsConfig.parseHex(3)`: it uses `Color` (java.awt) (dev/jeremyseq/config/DamageIndicatorsConfig.java:44)
- [ ] `DamageIndicatorsConfig.getHitmarkerColor(0)`: it uses `Color` (java.awt) (dev/jeremyseq/config/DamageIndicatorsConfig.java:53)
- [ ] `DamageIndicatorsConfig.getHitmarkerCritColor(0)`: it uses `Color` (java.awt) (dev/jeremyseq/config/DamageIndicatorsConfig.java:57)
- [ ] `DamageIndicatorsConfig.getHitmarkerKillColor(0)`: it uses `Color` (java.awt) (dev/jeremyseq/config/DamageIndicatorsConfig.java:61)
- [ ] `DamageIndicatorsConfig.load(0)`: it calls DamageIndicatorsConfig.load, which isn't translated (dev/jeremyseq/config/DamageIndicatorsConfig.java:65)
- [ ] `DamageIndicatorsConfig.load(1)`: it uses `CommentedFileConfig` (com.electronwill.nightconfig.core.file) (dev/jeremyseq/config/DamageIndicatorsConfig.java:69)
- [ ] `DamageIndicatorsConfig.save(0)`: unknown static DamageIndicators.LOGGER (dev/jeremyseq/config/DamageIndicatorsConfig.java:106)
- [ ] `DamageIndicatorsConfig.writeWithComments(1)`: it uses `CommentedFileConfig` (com.electronwill.nightconfig.core.file) (dev/jeremyseq/config/DamageIndicatorsConfig.java:118)
- [ ] `DamageIndicatorsConfig.reloadIfChanged(0)`: it calls `Files` (java.nio.file).getLastModifiedTime (dev/jeremyseq/config/DamageIndicatorsConfig.java:168)

### `CritTracker`

- [x] `CritTracker.lastAttackWasCrit` (dev/jeremyseq/CritTracker.java:4)

### `DamageIndicators`: another class has the same name, in a different package

- [x] `DamageIndicators.MOD_ID` (dev/jeremyseq/DamageIndicators.java:10)
- [x] `DamageIndicators.LOGGER` (dev/jeremyseq/DamageIndicators.java:11)
- [ ] `DamageIndicators.init(0)`: it calls DamageIndicatorsConfig.load, which isn't translated (dev/jeremyseq/DamageIndicators.java:13)

### `DamageTextHandler`

- [x] `DamageTextHandler.SELF_DAMAGE_TEXT` (dev/jeremyseq/damage_text/DamageTextHandler.java:8)
- [ ] `DamageTextHandler.damageTexts`: it uses DamageText, which isn't translated (it uses Minecraft's `DamageSource`) (dev/jeremyseq/damage_text/DamageTextHandler.java:9)
- [ ] `DamageTextHandler.getDamageTexts(0)`: it uses DamageText, which isn't translated (it uses Minecraft's `DamageSource`) (dev/jeremyseq/damage_text/DamageTextHandler.java:11)

### `DamageTextHandler.DamageText`: it uses Minecraft's `DamageSource`

- [ ] `DamageTextHandler.DamageText.DamageText(5)`: its class isn't translated (it uses Minecraft's `DamageSource`) (dev/jeremyseq/damage_text/DamageTextHandler.java:24)
- [ ] `DamageTextHandler.DamageText.getX(0)`: its class isn't translated (it uses Minecraft's `DamageSource`) (dev/jeremyseq/damage_text/DamageTextHandler.java:33)
- [ ] `DamageTextHandler.DamageText.getY(0)`: its class isn't translated (it uses Minecraft's `DamageSource`) (dev/jeremyseq/damage_text/DamageTextHandler.java:37)
- [ ] `DamageTextHandler.DamageText.getZ(0)`: its class isn't translated (it uses Minecraft's `DamageSource`) (dev/jeremyseq/damage_text/DamageTextHandler.java:41)
- [ ] `DamageTextHandler.DamageText.getDamage(0)`: its class isn't translated (it uses Minecraft's `DamageSource`) (dev/jeremyseq/damage_text/DamageTextHandler.java:45)
- [ ] `DamageTextHandler.DamageText.getSource(0)`: its class isn't translated (it uses Minecraft's `DamageSource`) (dev/jeremyseq/damage_text/DamageTextHandler.java:49)
- [ ] `DamageTextHandler.DamageText.getTimestamp(0)`: its class isn't translated (it uses Minecraft's `DamageSource`) (dev/jeremyseq/damage_text/DamageTextHandler.java:53)
- [ ] `DamageTextHandler.DamageText.getDuration(0)`: its class isn't translated (it uses Minecraft's `DamageSource`) (dev/jeremyseq/damage_text/DamageTextHandler.java:57)

### `DamageTextRenderer`

- [x] `DamageTextRenderer.FULL_BRIGHT` (dev/jeremyseq/damage_text/DamageTextRenderer.java:22)
- [ ] `DamageTextRenderer.render(2)`: it uses Minecraft's `SubmitNodeCollector` (dev/jeremyseq/damage_text/DamageTextRenderer.java:24)
- [ ] `DamageTextRenderer.hasLineOfSight(5)`: it uses Minecraft's `LivingEntity` (dev/jeremyseq/damage_text/DamageTextRenderer.java:69)
- [ ] `DamageTextRenderer.getDamageColor(2)`: it uses `Color` (java.awt) (dev/jeremyseq/damage_text/DamageTextRenderer.java:75)

### `HealthBarRenderer`

- [x] `HealthBarRenderer.FULL_BRIGHT` (dev/jeremyseq/damage_text/HealthBarRenderer.java:24)
- [ ] `HealthBarRenderer.render(2)`: it uses Minecraft's `SubmitNodeCollector` (dev/jeremyseq/damage_text/HealthBarRenderer.java:26)
- [ ] `HealthBarRenderer.hasLineOfSight(6)`: it uses Minecraft's `LivingEntity` (dev/jeremyseq/damage_text/HealthBarRenderer.java:44)
- [ ] `HealthBarRenderer.renderHealthBar(3)`: it uses Minecraft's `SubmitNodeCollector` (dev/jeremyseq/damage_text/HealthBarRenderer.java:50)

### `ClientTickHandler`

- [ ] `ClientTickHandler.LISTENERS`: a function kept in a variable or field (only parameters can be) (dev/jeremyseq/events/ClientTickHandler.java:7)
- [ ] `ClientTickHandler.register(1)`: it uses ClientTickHandler.LISTENERS, which isn't translated (a function kept in a variable or field (only parameters can be)) (dev/jeremyseq/events/ClientTickHandler.java:9)
- [ ] `ClientTickHandler.tick(0)`: it uses ClientTickHandler.LISTENERS, which isn't translated (a function kept in a variable or field (only parameters can be)) (dev/jeremyseq/events/ClientTickHandler.java:13)

### `ConfigReloader`

- [x] `ConfigReloader.reloadCheckCounter` (dev/jeremyseq/events/ConfigReloader.java:6)
- [ ] `ConfigReloader.register(0)`: it calls ClientTickHandler.register, which isn't translated (dev/jeremyseq/events/ConfigReloader.java:8)
- [ ] `ConfigReloader.onClientTick(0)`: it calls DamageIndicatorsConfig.reloadIfChanged, which isn't translated (dev/jeremyseq/events/ConfigReloader.java:12)

### `DamageHandler`

- [x] `DamageHandler.lastHealth` (dev/jeremyseq/events/DamageHandler.java:20)
- [x] `DamageHandler.lastHealthForOtherEntities` (dev/jeremyseq/events/DamageHandler.java:21)
- [ ] `DamageHandler.register(0)`: it calls ClientTickHandler.register, which isn't translated (dev/jeremyseq/events/DamageHandler.java:23)
- [ ] `DamageHandler.onClientTick(0)`: it calls Minecraft's LocalPlayer.getHealth (no Horizon helper for it yet) (dev/jeremyseq/events/DamageHandler.java:27)

### `DamageIndicators`: another class has the same name, in a different package

- [ ] `DamageIndicators.onInitialize(0)`: its class isn't translated (another class has the same name, in a different package) (dev/jeremyseq/fabric/DamageIndicators.java:6)

### `DamageIndicatorsClient`: it implements Fabric's `ClientModInitializer`

- [ ] `DamageIndicatorsClient.onInitializeClient(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (dev/jeremyseq/fabric/DamageIndicatorsClient.java:6)

### `FabricClientEvents`

- [ ] `FabricClientEvents.register(0)`: it uses Fabric's `LevelRenderEvents`.COLLECT_SUBMITS (dev/jeremyseq/fabric/FabricClientEvents.java:18)

### `FabricClientTickHandler`

- [ ] `FabricClientTickHandler.register(0)`: it uses Fabric's `ClientTickEvents`.END_CLIENT_TICK (dev/jeremyseq/fabric/FabricClientTickHandler.java:8)
- [ ] `PlayerAttackMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/jeremyseq/fabric/FabricClientTickHandler.java:13)

### `DirectionalIndicatorImages`

- [x] `DirectionalIndicatorImages.FOLDER` (dev/jeremyseq/overlays/DirectionalIndicatorImages.java:14)
- [x] `DirectionalIndicatorImages.IMAGES` (dev/jeremyseq/overlays/DirectionalIndicatorImages.java:15)
- [ ] `DirectionalIndicatorImages.reload(1)`: it uses Minecraft's `ResourceManager` (dev/jeremyseq/overlays/DirectionalIndicatorImages.java:17)
- [x] `DirectionalIndicatorImages.get(1)` (dev/jeremyseq/overlays/DirectionalIndicatorImages.java:43)
- [x] `DirectionalIndicatorImages.getAll(0)` (dev/jeremyseq/overlays/DirectionalIndicatorImages.java:47)

### `IndicatorOverlay`

- [x] `IndicatorOverlay.BLOOD_OVERLAY` (dev/jeremyseq/overlays/IndicatorOverlay.java:14)
- [x] `IndicatorOverlay.SUBPIXEL` (dev/jeremyseq/overlays/IndicatorOverlay.java:15)
- [ ] `IndicatorOverlay.damageSource`: it uses Minecraft's `Vec3` (dev/jeremyseq/overlays/IndicatorOverlay.java:16)
- [x] `IndicatorOverlay.showDirectional` (dev/jeremyseq/overlays/IndicatorOverlay.java:17)
- [x] `IndicatorOverlay.counter` (dev/jeremyseq/overlays/IndicatorOverlay.java:18)
- [x] `IndicatorOverlay.showHitmarker` (dev/jeremyseq/overlays/IndicatorOverlay.java:19)
- [x] `IndicatorOverlay.hitmarkerCounter` (dev/jeremyseq/overlays/IndicatorOverlay.java:20)
- [x] `IndicatorOverlay.wasKill` (dev/jeremyseq/overlays/IndicatorOverlay.java:21)
- [x] `IndicatorOverlay.wasCrit` (dev/jeremyseq/overlays/IndicatorOverlay.java:22)
- [x] `IndicatorOverlay.showBlood` (dev/jeremyseq/overlays/IndicatorOverlay.java:23)
- [x] `IndicatorOverlay.bloodAlpha` (dev/jeremyseq/overlays/IndicatorOverlay.java:24)
- [x] `IndicatorOverlay.BLOOD_FADE_SPEED` (dev/jeremyseq/overlays/IndicatorOverlay.java:25)
- [ ] `IndicatorOverlay.triggerOverlay(1)`: it uses IndicatorOverlay.damageSource, which isn't translated (it uses Minecraft's `Vec3`) (dev/jeremyseq/overlays/IndicatorOverlay.java:27)
- [x] `IndicatorOverlay.triggerHitmarker(2)` (dev/jeremyseq/overlays/IndicatorOverlay.java:35)
- [ ] `IndicatorOverlay.render(4)`: it reads .hud of a Gui (dev/jeremyseq/overlays/IndicatorOverlay.java:42)
- [ ] `IndicatorOverlay.calculateFinalAngle(3)`: it uses Minecraft's `Vec2` (dev/jeremyseq/overlays/IndicatorOverlay.java:140)
- [ ] `IndicatorOverlay.drawX(7)`: it uses `Color` (java.awt) (dev/jeremyseq/overlays/IndicatorOverlay.java:148)
- [ ] `IndicatorOverlay.drawRotatedBar(8)`: it uses `Color` (java.awt) (dev/jeremyseq/overlays/IndicatorOverlay.java:153)
- [ ] `IndicatorOverlay.fillFloat(6)`: it uses `Color` (java.awt) (dev/jeremyseq/overlays/IndicatorOverlay.java:165)
- [ ] `IndicatorOverlay.withAlpha(2)`: it uses `Color` (java.awt) (dev/jeremyseq/overlays/IndicatorOverlay.java:172)
