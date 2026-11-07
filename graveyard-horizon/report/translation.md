# Java translated to Rust

Translated 156 of 2382 methods and 460 of 1274 static fields to Rust (3 of those call the game, through Horizon's helpers); 0 of 3 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 1867 | `TGAdvancementTrigger.trigger(1)`: its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) |
| Uses Minecraft classes (needs Horizon's game API) | 238 | `GravestoneBlockEntity.convertLegacyText(3)`: it uses Minecraft's `CompoundTag` |
| Other Java this translator doesn't handle yet | 76 | `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitModel(9)`: it's generic |
| Uses other libraries | 15 | `ConfigWriter.decimal(1)`: it calls `BigDecimal` (java.math).valueOf |
| Uses null | 12 | `CorruptedChampionEntry.bossMusicVolume(0)`: arithmetic on float (or null) and float |
| Uses Java collections other than List (Map, Set...) | 6 | `LegacyEntityData.modifier(2)`: it uses `UUID` (java.util) |
| Calls the game where Horizon has no helper yet | 5 | `ChronicleButton.hasStatusEffects(0)`: it calls Minecraft's LocalPlayer.getActiveEffects (no Horizon helper for it yet) |
| Uses Gson (JSON) | 3 | `ConfigReader.parseObject(2)`: it uses `JsonReader` (com.google.gson.stream) |
| Uses inheritance (extends, implements, instanceof) | 2 | `FragmentSpotHints.isFragment(1)`: `instanceof` |
| Shares one object between names (Java references) | 1 | `LichEntity.champion(0)`: it stores a second reference to an object (Rust would need it copied or shared) |
| Uses files (java.nio / java.io) | 1 | `LegacyMigration.joinLines(1)`: it creates `BufferedReader` (java.io) |

## Close, not the same

Where the Rust does a little less than the Java, because Horizon doesn't have something:

- LegacyItemData.hasData: the item's custom data is treated as absent (Horizon doesn't keep it)

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| com/lion/graveyard/fabric/GraveyardClientFabric.java:32 | `tick` | not translated: it calls AnimationPreviewClient.clientTick, which isn't translated |
| com/lion/graveyard/fabric/GraveyardClientFabric.java:32 | `tick` | not translated: it calls FragmentSpotHints.clientTick, which isn't translated |
| com/lion/graveyard/fabric/GraveyardClientFabric.java:32 | `init` | not translated: it calls GraveyardClient.init, which isn't translated |

## Every class


### `PlatformMethods`

- [x] `PlatformMethods.getCurrentTarget(0)` (architectury_inject_graveyard_common_b80d6c58b4ee4764bbf9e4837d6706ef_60a3283155ee0c308a142f8b9590e1717e056ecc29148714b8674935119ff19cgraveyard400mc263devjar/PlatformMethods.java:4)

### `TGAdvancementTrigger`: it extends Minecraft's `SimpleCriterionTrigger`

- [ ] `TGAdvancementTrigger.trigger(1)`: its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/advancements/TGAdvancementTrigger.java:13)
- [ ] `TGAdvancementTrigger.codec(0)`: its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/advancements/TGAdvancementTrigger.java:17)

### `TGAdvancementTrigger.Condition`: it implements Minecraft's `SimpleInstance`

- [ ] `TGAdvancementTrigger.Condition.CODEC`: it uses Minecraft's `Codec` (com/lion/graveyard/advancements/TGAdvancementTrigger.java:22)
- [ ] `TGAdvancementTrigger.Condition.Condition(1)`: its class isn't translated (it implements Minecraft's `SimpleInstance`) (com/lion/graveyard/advancements/TGAdvancementTrigger.java:21)
- [ ] `TGAdvancementTrigger.Condition.player(0)`: its class isn't translated (it implements Minecraft's `SimpleInstance`) (com/lion/graveyard/advancements/TGAdvancementTrigger.java:21)

### `SarcophagusLidAnimator`

- [x] `SarcophagusLidAnimator.step(0)` (com/lion/graveyard/blockentities/animation/SarcophagusLidAnimator.java:10)
- [x] `SarcophagusLidAnimator.getProgress(1)` (com/lion/graveyard/blockentities/animation/SarcophagusLidAnimator.java:20)
- [x] `SarcophagusLidAnimator.setOpen(1)` (com/lion/graveyard/blockentities/animation/SarcophagusLidAnimator.java:24)

### `BrazierBlockEntity`: it extends Minecraft's `BlockEntity`

- [ ] `BrazierBlockEntity.BrazierBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/BrazierBlockEntity.java:15)
- [ ] `BrazierBlockEntity.registerControllers(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/BrazierBlockEntity.java:19)
- [ ] `BrazierBlockEntity.getAnimatableInstanceCache(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/BrazierBlockEntity.java:22)

### `SarcophagusPart`: its constants carry values

- [ ] `SarcophagusPart.SarcophagusPart(1)`: its class isn't translated (its constants carry values) (com/lion/graveyard/blockentities/enums/SarcophagusPart.java:11)
- [ ] `SarcophagusPart.toString(0)`: its class isn't translated (its constants carry values) (com/lion/graveyard/blockentities/enums/SarcophagusPart.java:15)
- [ ] `SarcophagusPart.getSerializedName(0)`: its class isn't translated (its constants carry values) (com/lion/graveyard/blockentities/enums/SarcophagusPart.java:20)

### `GravestoneBlockEntity`: it extends Minecraft's `BlockEntity`

- [x] `GravestoneBlockEntity.LOGGER` (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:50)
- [x] `GravestoneBlockEntity.LEGACY_LINE_KEYS` (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:51)
- [ ] `GravestoneBlockEntity.GravestoneBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:57)
- [ ] `GravestoneBlockEntity.GravestoneBlockEntity(3)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:61)
- [ ] `GravestoneBlockEntity.createText(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:65)
- [ ] `GravestoneBlockEntity.getText(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:69)
- [ ] `GravestoneBlockEntity.getTextLineHeight(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:73)
- [ ] `GravestoneBlockEntity.getMaxTextWidth(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:77)
- [ ] `GravestoneBlockEntity.saveAdditional(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:81)
- [ ] `GravestoneBlockEntity.loadAdditional(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:88)
- [ ] `GravestoneBlockEntity.convertLegacyText(3)`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:102)
- [ ] `GravestoneBlockEntity.convertLegacyLines(2)`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:117)
- [ ] `GravestoneBlockEntity.convertLegacyLine(2)`: it uses Minecraft's `Tag` (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:131)
- [ ] `GravestoneBlockEntity.parseLines(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:149)
- [ ] `GravestoneBlockEntity.parseLine(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:161)
- [ ] `GravestoneBlockEntity.tryChangeText(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:173)
- [ ] `GravestoneBlockEntity.changeText(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:183)
- [ ] `GravestoneBlockEntity.getTextWithMessages(3)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:188)
- [ ] `GravestoneBlockEntity.setText(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:204)
- [ ] `GravestoneBlockEntity.setFrontText(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:208)
- [ ] `GravestoneBlockEntity.runCommandClickEvent(3)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:218)
- [ ] `GravestoneBlockEntity.createCommandSource(3)`: it uses Minecraft's `Level` (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:232)
- [ ] `GravestoneBlockEntity.getUpdatePacket(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:248)
- [ ] `GravestoneBlockEntity.getUpdateTag(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:252)
- [ ] `GravestoneBlockEntity.setEditor(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:256)
- [ ] `GravestoneBlockEntity.getEditor(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:260)
- [ ] `GravestoneBlockEntity.updateListeners(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:265)
- [ ] `GravestoneBlockEntity.isWaxed(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:270)
- [ ] `GravestoneBlockEntity.setWaxed(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:274)
- [ ] `GravestoneBlockEntity.isPlayerTooFarToEdit(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:284)
- [ ] `GravestoneBlockEntity.tick(4)`: it uses Minecraft's `Level` (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:289)
- [ ] `GravestoneBlockEntity.tryClearInvalidEditor(3)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/GravestoneBlockEntity.java:296)

### `LoreBookBlockEntity`: it extends Minecraft's `BlockEntity`

- [x] `LoreBookBlockEntity.PAGE_KEY` (com/lion/graveyard/blockentities/LoreBookBlockEntity.java:19)
- [ ] `LoreBookBlockEntity.LoreBookBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/LoreBookBlockEntity.java:23)
- [ ] `LoreBookBlockEntity.getPage(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/LoreBookBlockEntity.java:27)
- [ ] `LoreBookBlockEntity.setPage(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/LoreBookBlockEntity.java:32)
- [ ] `LoreBookBlockEntity.saveAdditional(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/LoreBookBlockEntity.java:37)
- [ ] `LoreBookBlockEntity.loadAdditional(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/LoreBookBlockEntity.java:46)
- [ ] `LoreBookBlockEntity.getUpdateTag(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/LoreBookBlockEntity.java:52)
- [ ] `LoreBookBlockEntity.getUpdatePacket(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/LoreBookBlockEntity.java:56)

### `BrazierModel`: it extends `GeoModel` (com.geckolib.model)

- [ ] `BrazierModel.getAnimationResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/blockentities/models/BrazierModel.java:10)
- [ ] `BrazierModel.getModelResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/blockentities/models/BrazierModel.java:14)
- [ ] `BrazierModel.getTextureResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/blockentities/models/BrazierModel.java:18)

### `OssuaryModel`: it extends `GeoModel` (com.geckolib.model)

- [ ] `OssuaryModel.getAnimationResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/blockentities/models/OssuaryModel.java:10)
- [ ] `OssuaryModel.getModelResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/blockentities/models/OssuaryModel.java:14)
- [ ] `OssuaryModel.getTextureResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/blockentities/models/OssuaryModel.java:18)

### `OssuaryBlockEntity`: it extends Minecraft's `BlockEntity`

- [ ] `OssuaryBlockEntity.OssuaryBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/OssuaryBlockEntity.java:30)
- [ ] `OssuaryBlockEntity.registerControllers(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/OssuaryBlockEntity.java:34)
- [ ] `OssuaryBlockEntity.getAnimatableInstanceCache(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/OssuaryBlockEntity.java:72)
- [ ] `OssuaryBlockEntity.tick(4)`: it uses Minecraft's `Level` (com/lion/graveyard/blockentities/OssuaryBlockEntity.java:76)

### `BrazierBlockEntityRenderer`: it's generic

- [ ] `BrazierBlockEntityRenderer.BrazierBlockEntityRenderer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/blockentities/renders/BrazierBlockEntityRenderer.java:13)
- [ ] `BrazierBlockEntityRenderer.getViewDistance(0)`: its class isn't translated (it's generic) (com/lion/graveyard/blockentities/renders/BrazierBlockEntityRenderer.java:17)
- [ ] `BrazierBlockEntityRenderer.getRenderType(2)`: its class isn't translated (it's generic) (com/lion/graveyard/blockentities/renders/BrazierBlockEntityRenderer.java:21)

### `GravestoneBlockEntityRenderer`: it implements Minecraft's `BlockEntityRenderer`

- [x] `GravestoneBlockEntityRenderer.RENDER_DISTANCE` (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:64)
- [ ] `GravestoneBlockEntityRenderer.GravestoneBlockEntityRenderer(1)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:68)
- [ ] `GravestoneBlockEntityRenderer.createRenderState(0)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:73)
- [ ] `GravestoneBlockEntityRenderer.extractRenderState(5)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:77)
- [ ] `GravestoneBlockEntityRenderer.submit(4)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:103)
- [ ] `GravestoneBlockEntityRenderer.submitGrave(3)`: its class isn't translated (it implements Minecraft's `BlockEntityRenderer`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:121)
- [ ] `GravestoneBlockEntityRenderer.shouldRender(2)`: it uses GravestoneBlockEntity, which isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:138)
- [ ] `GravestoneBlockEntityRenderer.getColor(1)`: it uses GravestoneBlockEntity, which isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:153)

### `GravestoneBlockEntityRenderer.CrumblingItemCollector`: it implements Minecraft's `SubmitNodeCollector`

- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.CrumblingItemCollector(2)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:170)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.wrap(2)`: it uses Minecraft's `SubmitNodeCollector` (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:175)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitItem(8)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:179)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.order(1)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:195)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitShadow(3)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:199)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitNameTag(7)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:203)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitText(10)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:207)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitTextBackground(8)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:222)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitFlame(3)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:226)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitLeash(2)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:230)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitModel(9)`: it's generic (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:234)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitCrumblingOverlay(8)`: it's generic (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:240)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitMovingBlock(3)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:246)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitBlockModel(7)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:250)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitBreakingBlockModel(4)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:256)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitShapeOutline(6)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:260)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitCustomGeometry(3)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:264)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitQuadParticleGroup(1)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:268)
- [ ] `GravestoneBlockEntityRenderer.CrumblingItemCollector.submitGizmoPrimitives(3)`: its class isn't translated (it implements Minecraft's `SubmitNodeCollector`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:272)

### `GravestoneBlockEntityRenderer.QuadModel`: it extends Minecraft's `Model`

- [ ] `GravestoneBlockEntityRenderer.QuadModel.QuadModel(2)`: its class isn't translated (it extends Minecraft's `Model`) (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:278)
- [ ] `GravestoneBlockEntityRenderer.QuadModel.createRoot(1)`: it uses Minecraft's `ModelPart` (com/lion/graveyard/blockentities/renders/GravestoneBlockEntityRenderer.java:282)

### `GravestoneBlockEntityRenderer.State`: it extends Minecraft's `BlockEntityRenderState`


### `OssuaryBlockEntityRenderer`: it's generic

- [ ] `OssuaryBlockEntityRenderer.OssuaryBlockEntityRenderer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/blockentities/renders/OssuaryBlockEntityRenderer.java:13)
- [ ] `OssuaryBlockEntityRenderer.getRenderType(2)`: its class isn't translated (it's generic) (com/lion/graveyard/blockentities/renders/OssuaryBlockEntityRenderer.java:17)

### `SarcophagusBlockEntityRenderer`: it's generic

- [ ] `SarcophagusBlockEntityRenderer.SarcophagusBlockEntityRenderer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/blockentities/renders/SarcophagusBlockEntityRenderer.java:38)
- [ ] `SarcophagusBlockEntityRenderer.createRenderState(0)`: its class isn't translated (it's generic) (com/lion/graveyard/blockentities/renders/SarcophagusBlockEntityRenderer.java:42)
- [ ] `SarcophagusBlockEntityRenderer.getViewDistance(0)`: its class isn't translated (it's generic) (com/lion/graveyard/blockentities/renders/SarcophagusBlockEntityRenderer.java:46)
- [ ] `SarcophagusBlockEntityRenderer.getRenderBoundingBox(1)`: its class isn't translated (it's generic) (com/lion/graveyard/blockentities/renders/SarcophagusBlockEntityRenderer.java:50)
- [ ] `SarcophagusBlockEntityRenderer.extractRenderState(5)`: its class isn't translated (it's generic) (com/lion/graveyard/blockentities/renders/SarcophagusBlockEntityRenderer.java:54)
- [ ] `SarcophagusBlockEntityRenderer.submit(4)`: its class isn't translated (it's generic) (com/lion/graveyard/blockentities/renders/SarcophagusBlockEntityRenderer.java:82)
- [ ] `SarcophagusBlockEntityRenderer.submitPart(5)`: its class isn't translated (it's generic) (com/lion/graveyard/blockentities/renders/SarcophagusBlockEntityRenderer.java:96)
- [ ] `SarcophagusBlockEntityRenderer.updateModel(3)`: its class isn't translated (it's generic) (com/lion/graveyard/blockentities/renders/SarcophagusBlockEntityRenderer.java:127)

### `SarcophagusBlockEntityRenderer.State`: it extends Minecraft's `BlockEntityRenderState`


### `SarcophagusBlockEntity`: it extends Minecraft's `RandomizableContainerBlockEntity`

- [ ] `SarcophagusBlockEntity.SarcophagusBlockEntity(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:71)
- [ ] `SarcophagusBlockEntity.loadAdditional(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:75)
- [ ] `SarcophagusBlockEntity.saveAdditional(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:84)
- [ ] `SarcophagusBlockEntity.getName(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:96)
- [ ] `SarcophagusBlockEntity.getCustomName(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:100)
- [ ] `SarcophagusBlockEntity.applyImplicitComponents(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:104)
- [ ] `SarcophagusBlockEntity.collectImplicitComponents(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:109)
- [ ] `SarcophagusBlockEntity.getContainerSize(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:114)
- [ ] `SarcophagusBlockEntity.getItems(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:118)
- [ ] `SarcophagusBlockEntity.setItems(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:122)
- [ ] `SarcophagusBlockEntity.getDefaultName(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:126)
- [ ] `SarcophagusBlockEntity.createMenu(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:132)
- [ ] `SarcophagusBlockEntity.startOpen(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:136)
- [ ] `SarcophagusBlockEntity.stopOpen(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:143)
- [ ] `SarcophagusBlockEntity.onScheduledTick(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:149)
- [ ] `SarcophagusBlockEntity.onInvOpenOrClose(5)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:155)
- [ ] `SarcophagusBlockEntity.playSound(4)`: it uses Minecraft's `Level` (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:160)
- [ ] `SarcophagusBlockEntity.clientTick(4)`: it uses Minecraft's `Level` (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:167)
- [ ] `SarcophagusBlockEntity.triggerEvent(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:171)
- [ ] `SarcophagusBlockEntity.getOpenNess(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:180)
- [ ] `SarcophagusBlockEntity.isCoffin(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/SarcophagusBlockEntity.java:184)

### `SmolderingDebrisBlockEntity`: it extends Minecraft's `BlockEntity`

- [ ] `SmolderingDebrisBlockEntity.SmolderingDebrisBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/lion/graveyard/blockentities/SmolderingDebrisBlockEntity.java:12)
- [ ] `SmolderingDebrisBlockEntity.clientTick(4)`: it uses Minecraft's `Level` (com/lion/graveyard/blockentities/SmolderingDebrisBlockEntity.java:16)

### `UrnBlockEntity`: it extends Minecraft's `RandomizableContainerBlockEntity`

- [ ] `UrnBlockEntity.UrnBlockEntity(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:59)
- [ ] `UrnBlockEntity.loadAdditional(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:63)
- [ ] `UrnBlockEntity.saveAdditional(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:72)
- [ ] `UrnBlockEntity.getName(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:84)
- [ ] `UrnBlockEntity.getCustomName(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:88)
- [ ] `UrnBlockEntity.applyImplicitComponents(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:92)
- [ ] `UrnBlockEntity.collectImplicitComponents(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:97)
- [ ] `UrnBlockEntity.getContainerSize(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:102)
- [ ] `UrnBlockEntity.getItems(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:106)
- [ ] `UrnBlockEntity.setItems(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:110)
- [ ] `UrnBlockEntity.getDefaultName(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:114)
- [ ] `UrnBlockEntity.createMenu(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:118)
- [ ] `UrnBlockEntity.startOpen(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:123)
- [ ] `UrnBlockEntity.stopOpen(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:130)
- [ ] `UrnBlockEntity.onScheduledTick(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:136)
- [ ] `UrnBlockEntity.onInvOpenOrClose(5)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (com/lion/graveyard/blockentities/UrnBlockEntity.java:142)
- [ ] `UrnBlockEntity.playSound(4)`: it uses Minecraft's `Level` (com/lion/graveyard/blockentities/UrnBlockEntity.java:147)

### `AltarBlock`: it extends Minecraft's `Block`

- [ ] `AltarBlock.BLOODY`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/AltarBlock.java:43)
- [ ] `AltarBlock.COMPLETED_ALTAR`: it uses Minecraft's `BlockPattern` (com/lion/graveyard/blocks/AltarBlock.java:44)
- [ ] `AltarBlock.ANY_ORDER_ALTAR`: it uses Minecraft's `BlockPattern` (com/lion/graveyard/blocks/AltarBlock.java:45)
- [ ] `AltarBlock.AltarBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/AltarBlock.java:47)
- [ ] `AltarBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/AltarBlock.java:52)
- [ ] `AltarBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/AltarBlock.java:56)
- [ ] `AltarBlock.getCompletedFramePattern(0)`: it uses Minecraft's `BlockPattern` (com/lion/graveyard/blocks/AltarBlock.java:72)
- [ ] `AltarBlock.getAnyOrderFramePattern(0)`: it uses Minecraft's `BlockPattern` (com/lion/graveyard/blocks/AltarBlock.java:97)
- [ ] `AltarBlock.findSummonFailure(3)`: it uses Minecraft's `Level` (com/lion/graveyard/blocks/AltarBlock.java:113)
- [ ] `AltarBlock.hasLivingLich(2)`: it uses Minecraft's `Level` (com/lion/graveyard/blocks/AltarBlock.java:128)
- [ ] `AltarBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/AltarBlock.java:135)

### `AltarDirectionBlock`: it extends `AltarParticleBlock`

- [ ] `AltarDirectionBlock.FACING`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/blocks/AltarDirectionBlock.java:16)
- [ ] `AltarDirectionBlock.AltarDirectionBlock(1)`: its class isn't translated (it extends `AltarParticleBlock`) (com/lion/graveyard/blocks/AltarDirectionBlock.java:18)
- [ ] `AltarDirectionBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends `AltarParticleBlock`) (com/lion/graveyard/blocks/AltarDirectionBlock.java:23)
- [ ] `AltarDirectionBlock.getStateForPlacement(1)`: its class isn't translated (it extends `AltarParticleBlock`) (com/lion/graveyard/blocks/AltarDirectionBlock.java:27)
- [ ] `AltarDirectionBlock.rotate(2)`: its class isn't translated (it extends `AltarParticleBlock`) (com/lion/graveyard/blocks/AltarDirectionBlock.java:32)

### `AltarParticleBlock`: it extends Minecraft's `Block`

- [ ] `AltarParticleBlock.AltarParticleBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/AltarParticleBlock.java:12)
- [ ] `AltarParticleBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/AltarParticleBlock.java:16)

### `BlockItemTooltip`: it's an interface

- [ ] `BlockItemTooltip.appendItemTooltip(1)`: it has no body (com/lion/graveyard/blocks/BlockItemTooltip.java:7)

### `BoneDisplayBlock`: it extends Minecraft's `Block`

- [ ] `BoneDisplayBlock.FACING`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/blocks/BoneDisplayBlock.java:22)
- [ ] `BoneDisplayBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/BoneDisplayBlock.java:23)
- [ ] `BoneDisplayBlock.BoneDisplayBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BoneDisplayBlock.java:25)
- [ ] `BoneDisplayBlock.propagatesSkylightDown(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BoneDisplayBlock.java:32)
- [ ] `BoneDisplayBlock.getShadeBrightness(3)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BoneDisplayBlock.java:36)
- [ ] `BoneDisplayBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BoneDisplayBlock.java:40)
- [ ] `BoneDisplayBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BoneDisplayBlock.java:44)
- [ ] `BoneDisplayBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BoneDisplayBlock.java:50)
- [ ] `BoneDisplayBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BoneDisplayBlock.java:54)
- [ ] `BoneDisplayBlock.isCollisionShapeFullBlock(3)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BoneDisplayBlock.java:58)

### `BrazierBlock`: it extends Minecraft's `Block`

- [ ] `BrazierBlock.LIT`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/BrazierBlock.java:46)
- [ ] `BrazierBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/BrazierBlock.java:47)
- [ ] `BrazierBlock.STATE_TO_LUMINANCE`: it uses Minecraft's `BlockState` (com/lion/graveyard/blocks/BrazierBlock.java:48)
- [ ] `BrazierBlock.SHAPE`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/BrazierBlock.java:49)
- [ ] `BrazierBlock.BrazierBlock(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BrazierBlock.java:52)
- [ ] `BrazierBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BrazierBlock.java:58)
- [ ] `BrazierBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BrazierBlock.java:67)
- [ ] `BrazierBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BrazierBlock.java:71)
- [ ] `BrazierBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BrazierBlock.java:79)
- [ ] `BrazierBlock.isPathfindable(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BrazierBlock.java:96)
- [ ] `BrazierBlock.placeLiquid(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BrazierBlock.java:100)
- [ ] `BrazierBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BrazierBlock.java:116)
- [ ] `BrazierBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BrazierBlock.java:120)
- [ ] `BrazierBlock.onProjectileHit(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BrazierBlock.java:149)
- [ ] `BrazierBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BrazierBlock.java:160)
- [ ] `BrazierBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BrazierBlock.java:165)
- [ ] `BrazierBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/BrazierBlock.java:169)
- [ ] `BrazierBlock.extinguish(4)`: it uses Minecraft's `BlockState` (com/lion/graveyard/blocks/BrazierBlock.java:173)
- [ ] `BrazierBlock.spawnSmokeParticle(3)`: it uses Minecraft's `Level` (com/lion/graveyard/blocks/BrazierBlock.java:201)
- [ ] `BrazierBlock.setLit(4)`: it uses Minecraft's `LevelAccessor` (com/lion/graveyard/blocks/BrazierBlock.java:227)

### `CandleHolderBlock`: it extends Minecraft's `Block`

- [ ] `CandleHolderBlock.SHAPE_NORTH`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/CandleHolderBlock.java:27)
- [ ] `CandleHolderBlock.SHAPE_SOUTH`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/CandleHolderBlock.java:28)
- [ ] `CandleHolderBlock.SHAPE_EAST`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/CandleHolderBlock.java:29)
- [ ] `CandleHolderBlock.SHAPE_WEST`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/CandleHolderBlock.java:30)
- [ ] `CandleHolderBlock.FACING`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/blocks/CandleHolderBlock.java:31)
- [ ] `CandleHolderBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/CandleHolderBlock.java:32)
- [ ] `CandleHolderBlock.CandleHolderBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/CandleHolderBlock.java:34)
- [ ] `CandleHolderBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/CandleHolderBlock.java:41)
- [ ] `CandleHolderBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/CandleHolderBlock.java:54)
- [ ] `CandleHolderBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/CandleHolderBlock.java:60)
- [ ] `CandleHolderBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/CandleHolderBlock.java:77)
- [ ] `CandleHolderBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/CandleHolderBlock.java:81)
- [ ] `CandleHolderBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/CandleHolderBlock.java:85)

### `DarkIronBars`: it extends Minecraft's `IronBarsBlock`

- [ ] `DarkIronBars.DarkIronBars(1)`: its class isn't translated (it extends Minecraft's `IronBarsBlock`) (com/lion/graveyard/blocks/DarkIronBars.java:7)

### `GravestoneBlock`: it extends Minecraft's `StandingSignBlock`

- [ ] `GravestoneBlock.ROTATION`: it uses Minecraft's `IntegerProperty` (com/lion/graveyard/blocks/GravestoneBlock.java:63)
- [ ] `GravestoneBlock.FACING`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/blocks/GravestoneBlock.java:64)
- [ ] `GravestoneBlock.FLOOR`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/GravestoneBlock.java:65)
- [ ] `GravestoneBlock.SHAPE_FACING_EW`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/GravestoneBlock.java:66)
- [ ] `GravestoneBlock.SHAPE_FACING_NS`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/GravestoneBlock.java:67)
- [ ] `GravestoneBlock.GravestoneBlock(2)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:70)
- [ ] `GravestoneBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:79)
- [ ] `GravestoneBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:83)
- [ ] `GravestoneBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:87)
- [ ] `GravestoneBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:91)
- [ ] `GravestoneBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:96)
- [ ] `GravestoneBlock.useDyeOnSign(3)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:155)
- [ ] `GravestoneBlock.useInkOnSign(2)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:164)
- [ ] `GravestoneBlock.useGlowInkOnSign(2)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:173)
- [ ] `GravestoneBlock.useWaxOnSign(2)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:182)
- [ ] `GravestoneBlock.isTextLiteralOrEmpty(2)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:191)
- [ ] `GravestoneBlock.openEditScreen(5)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:198)
- [ ] `GravestoneBlock.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:203)
- [ ] `GravestoneBlock.openScreen(3)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:209)
- [ ] `GravestoneBlock.isOtherPlayerEditing(2)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:217)
- [ ] `GravestoneBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:222)
- [ ] `GravestoneBlock.getDrops(2)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:229)
- [ ] `GravestoneBlock.getTexture(0)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:233)
- [ ] `GravestoneBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:237)
- [ ] `GravestoneBlock.getCollisionShape(4)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:241)
- [ ] `GravestoneBlock.isPathfindable(2)`: its class isn't translated (it extends Minecraft's `StandingSignBlock`) (com/lion/graveyard/blocks/GravestoneBlock.java:245)

### `LongDecoBlock`: it extends Minecraft's `Block`

- [ ] `LongDecoBlock.FACING`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/blocks/LongDecoBlock.java:37)
- [ ] `LongDecoBlock.PART`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/blocks/LongDecoBlock.java:38)
- [ ] `LongDecoBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/LongDecoBlock.java:39)
- [ ] `LongDecoBlock.LongDecoBlock(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/LongDecoBlock.java:42)
- [ ] `LongDecoBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/LongDecoBlock.java:51)
- [ ] `LongDecoBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/LongDecoBlock.java:55)
- [ ] `LongDecoBlock.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/LongDecoBlock.java:67)
- [ ] `LongDecoBlock.otherPartPos(2)`: it uses Minecraft's `BlockPos` (com/lion/graveyard/blocks/LongDecoBlock.java:73)
- [ ] `LongDecoBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/LongDecoBlock.java:78)
- [ ] `LongDecoBlock.getDrops(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/LongDecoBlock.java:100)
- [ ] `LongDecoBlock.playerWillDestroy(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/LongDecoBlock.java:104)
- [ ] `LongDecoBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/LongDecoBlock.java:117)
- [ ] `LongDecoBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/LongDecoBlock.java:121)
- [ ] `LongDecoBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/LongDecoBlock.java:125)
- [ ] `LongDecoBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/LongDecoBlock.java:129)

### `LoreBookBlock`: it extends Minecraft's `BaseEntityBlock`

- [ ] `LoreBookBlock.FACING`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/blocks/LoreBookBlock.java:50)
- [ ] `LoreBookBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/LoreBookBlock.java:51)
- [ ] `LoreBookBlock.VARIANT`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/blocks/LoreBookBlock.java:52)
- [ ] `LoreBookBlock.SHAPE`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/LoreBookBlock.java:53)
- [ ] `LoreBookBlock.LoreBookBlock(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/LoreBookBlock.java:55)
- [ ] `LoreBookBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/LoreBookBlock.java:63)
- [ ] `LoreBookBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/LoreBookBlock.java:67)
- [ ] `LoreBookBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/LoreBookBlock.java:72)
- [ ] `LoreBookBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/LoreBookBlock.java:76)
- [ ] `LoreBookBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/LoreBookBlock.java:80)
- [ ] `LoreBookBlock.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/LoreBookBlock.java:85)
- [ ] `LoreBookBlock.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/LoreBookBlock.java:95)
- [ ] `LoreBookBlock.getDrops(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/LoreBookBlock.java:117)
- [ ] `LoreBookBlock.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/LoreBookBlock.java:124)
- [ ] `LoreBookBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/LoreBookBlock.java:130)
- [ ] `LoreBookBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/LoreBookBlock.java:134)
- [ ] `LoreBookBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/LoreBookBlock.java:151)
- [ ] `LoreBookBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/LoreBookBlock.java:155)

### `LoreBookBlock.Variant`: its constants carry values

- [ ] `LoreBookBlock.Variant.Variant(1)`: its class isn't translated (its constants carry values) (com/lion/graveyard/blocks/LoreBookBlock.java:166)
- [ ] `LoreBookBlock.Variant.getSerializedName(0)`: its class isn't translated (its constants carry values) (com/lion/graveyard/blocks/LoreBookBlock.java:170)
- [ ] `LoreBookBlock.Variant.next(0)`: its class isn't translated (its constants carry values) (com/lion/graveyard/blocks/LoreBookBlock.java:174)

### `OminousBoneStaffFragment`: it extends Minecraft's `Block`

- [ ] `OminousBoneStaffFragment.FACING`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/blocks/OminousBoneStaffFragment.java:32)
- [ ] `OminousBoneStaffFragment.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/OminousBoneStaffFragment.java:33)
- [ ] `OminousBoneStaffFragment.OminousBoneStaffFragment(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/OminousBoneStaffFragment.java:35)
- [ ] `OminousBoneStaffFragment.animateTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/OminousBoneStaffFragment.java:42)
- [ ] `OminousBoneStaffFragment.appendItemTooltip(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/OminousBoneStaffFragment.java:57)
- [ ] `OminousBoneStaffFragment.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/OminousBoneStaffFragment.java:65)
- [ ] `OminousBoneStaffFragment.propagatesSkylightDown(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/OminousBoneStaffFragment.java:70)
- [ ] `OminousBoneStaffFragment.getShadeBrightness(3)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/OminousBoneStaffFragment.java:74)
- [ ] `OminousBoneStaffFragment.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/OminousBoneStaffFragment.java:78)
- [ ] `OminousBoneStaffFragment.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/OminousBoneStaffFragment.java:82)
- [ ] `OminousBoneStaffFragment.getFluidState(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/OminousBoneStaffFragment.java:88)
- [ ] `OminousBoneStaffFragment.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/OminousBoneStaffFragment.java:92)
- [ ] `OminousBoneStaffFragment.isCollisionShapeFullBlock(3)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/OminousBoneStaffFragment.java:96)

### `OssuaryBlock`: it extends Minecraft's `BaseEntityBlock`

- [x] `OssuaryBlock.TITLE` in game_side.rs (com/lion/graveyard/blocks/OssuaryBlock.java:41)
- [ ] `OssuaryBlock.FACING`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/blocks/OssuaryBlock.java:42)
- [ ] `OssuaryBlock.OPEN`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/OssuaryBlock.java:43)
- [ ] `OssuaryBlock.appendItemTooltip(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/OssuaryBlock.java:45)
- [ ] `OssuaryBlock.OssuaryBlock(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/OssuaryBlock.java:50)
- [ ] `OssuaryBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/OssuaryBlock.java:55)
- [ ] `OssuaryBlock.getTicker(3)`: it's generic (com/lion/graveyard/blocks/OssuaryBlock.java:59)
- [ ] `OssuaryBlock.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/OssuaryBlock.java:64)
- [ ] `OssuaryBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/OssuaryBlock.java:73)
- [ ] `OssuaryBlock.getMenuProvider(3)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/OssuaryBlock.java:87)
- [ ] `OssuaryBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/OssuaryBlock.java:94)
- [ ] `OssuaryBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/OssuaryBlock.java:98)
- [ ] `OssuaryBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/OssuaryBlock.java:102)
- [ ] `OssuaryBlock.isPathfindable(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/OssuaryBlock.java:106)
- [ ] `OssuaryBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/OssuaryBlock.java:110)
- [ ] `OssuaryBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/OssuaryBlock.java:115)

### `PillarBlock`: it extends Minecraft's `Block`

- [ ] `PillarBlock.SHAPE`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/PillarBlock.java:27)
- [ ] `PillarBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/PillarBlock.java:28)
- [ ] `PillarBlock.PillarBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/PillarBlock.java:30)
- [ ] `PillarBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/PillarBlock.java:35)
- [ ] `PillarBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/PillarBlock.java:39)
- [ ] `PillarBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/PillarBlock.java:47)
- [ ] `PillarBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/PillarBlock.java:64)
- [ ] `PillarBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/PillarBlock.java:68)
- [ ] `PillarBlock.isPathfindable(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/PillarBlock.java:72)

### `SarcophagusBlock`: it extends Minecraft's `BaseEntityBlock`

- [ ] `SarcophagusBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/SarcophagusBlock.java:61)
- [ ] `SarcophagusBlock.OPEN`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/SarcophagusBlock.java:62)
- [ ] `SarcophagusBlock.PLAYER_PLACED`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/SarcophagusBlock.java:63)
- [ ] `SarcophagusBlock.IS_COFFIN`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/SarcophagusBlock.java:64)
- [ ] `SarcophagusBlock.FACING`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/blocks/SarcophagusBlock.java:65)
- [ ] `SarcophagusBlock.DOUBLE_NORTH_SHAPE`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/SarcophagusBlock.java:66)
- [ ] `SarcophagusBlock.DOUBLE_SOUTH_SHAPE`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/SarcophagusBlock.java:67)
- [ ] `SarcophagusBlock.DOUBLE_WEST_SHAPE`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/SarcophagusBlock.java:68)
- [ ] `SarcophagusBlock.DOUBLE_EAST_SHAPE`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/SarcophagusBlock.java:69)
- [ ] `SarcophagusBlock.PART`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/blocks/SarcophagusBlock.java:70)
- [ ] `SarcophagusBlock.SarcophagusBlock(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:74)
- [ ] `SarcophagusBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:87)
- [ ] `SarcophagusBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:91)
- [ ] `SarcophagusBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:95)
- [ ] `SarcophagusBlock.getDirectionTowardsOtherPart(2)`: it uses Minecraft's `Direction` (com/lion/graveyard/blocks/SarcophagusBlock.java:112)
- [ ] `SarcophagusBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:116)
- [ ] `SarcophagusBlock.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:130)
- [ ] `SarcophagusBlock.spawnGhost(5)`: it uses Minecraft's `BlockState` (com/lion/graveyard/blocks/SarcophagusBlock.java:150)
- [ ] `SarcophagusBlock.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:183)
- [ ] `SarcophagusBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:194)
- [ ] `SarcophagusBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:198)
- [ ] `SarcophagusBlock.getSeed(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:202)
- [ ] `SarcophagusBlock.getSarcophagusPart(1)`: it uses Minecraft's `BlockType` (com/lion/graveyard/blocks/SarcophagusBlock.java:207)
- [ ] `SarcophagusBlock.playerWillDestroy(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:212)
- [ ] `SarcophagusBlock.affectNeighborsAfterRemoval(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:228)
- [ ] `SarcophagusBlock.tick(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:232)
- [ ] `SarcophagusBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:239)
- [ ] `SarcophagusBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:244)
- [ ] `SarcophagusBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:248)
- [ ] `SarcophagusBlock.isPathfindable(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:259)
- [ ] `SarcophagusBlock.getOppositePartDirection(1)`: it uses Minecraft's `Direction` (com/lion/graveyard/blocks/SarcophagusBlock.java:263)
- [ ] `SarcophagusBlock.getAnimationProgressRetriever(1)`: its body couldn't be read: an anonymous class (com/lion/graveyard/blocks/SarcophagusBlock.java:268)
- [ ] `SarcophagusBlock.getTicker(3)`: it's generic (com/lion/graveyard/blocks/SarcophagusBlock.java:284)
- [ ] `SarcophagusBlock.getLid(0)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:289)
- [ ] `SarcophagusBlock.getBase(0)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SarcophagusBlock.java:293)

### `SmolderingDebrisBlock`: it extends Minecraft's `BaseEntityBlock`

- [ ] `SmolderingDebrisBlock.SmolderingDebrisBlock(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SmolderingDebrisBlock.java:21)
- [ ] `SmolderingDebrisBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SmolderingDebrisBlock.java:25)
- [ ] `SmolderingDebrisBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SmolderingDebrisBlock.java:29)
- [ ] `SmolderingDebrisBlock.getTicker(3)`: it's generic (com/lion/graveyard/blocks/SmolderingDebrisBlock.java:33)
- [ ] `SmolderingDebrisBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/SmolderingDebrisBlock.java:40)

### `SmolderingDoorBlock`: it extends Minecraft's `DoorBlock`

- [ ] `SmolderingDoorBlock.SmolderingDoorBlock(2)`: its class isn't translated (it extends Minecraft's `DoorBlock`) (com/lion/graveyard/blocks/SmolderingDoorBlock.java:20)
- [ ] `SmolderingDoorBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `DoorBlock`) (com/lion/graveyard/blocks/SmolderingDoorBlock.java:25)
- [ ] `SmolderingDoorBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `DoorBlock`) (com/lion/graveyard/blocks/SmolderingDoorBlock.java:30)
- [ ] `SmolderingDoorBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `DoorBlock`) (com/lion/graveyard/blocks/SmolderingDoorBlock.java:35)

### `SmolderingFenceBlock`: it extends Minecraft's `FenceBlock`

- [ ] `SmolderingFenceBlock.SmolderingFenceBlock(1)`: its class isn't translated (it extends Minecraft's `FenceBlock`) (com/lion/graveyard/blocks/SmolderingFenceBlock.java:19)
- [ ] `SmolderingFenceBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `FenceBlock`) (com/lion/graveyard/blocks/SmolderingFenceBlock.java:24)
- [ ] `SmolderingFenceBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `FenceBlock`) (com/lion/graveyard/blocks/SmolderingFenceBlock.java:29)
- [ ] `SmolderingFenceBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `FenceBlock`) (com/lion/graveyard/blocks/SmolderingFenceBlock.java:34)

### `SmolderingFenceGateBlock`: it extends Minecraft's `FenceGateBlock`

- [ ] `SmolderingFenceGateBlock.SmolderingFenceGateBlock(2)`: its class isn't translated (it extends Minecraft's `FenceGateBlock`) (com/lion/graveyard/blocks/SmolderingFenceGateBlock.java:20)
- [ ] `SmolderingFenceGateBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `FenceGateBlock`) (com/lion/graveyard/blocks/SmolderingFenceGateBlock.java:25)
- [ ] `SmolderingFenceGateBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `FenceGateBlock`) (com/lion/graveyard/blocks/SmolderingFenceGateBlock.java:30)
- [ ] `SmolderingFenceGateBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `FenceGateBlock`) (com/lion/graveyard/blocks/SmolderingFenceGateBlock.java:35)

### `SmolderingPillarBlock`: it extends Minecraft's `RotatedPillarBlock`

- [ ] `SmolderingPillarBlock.SmolderingPillarBlock(1)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (com/lion/graveyard/blocks/SmolderingPillarBlock.java:22)
- [ ] `SmolderingPillarBlock.SmolderingPillarBlock(2)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (com/lion/graveyard/blocks/SmolderingPillarBlock.java:26)
- [ ] `SmolderingPillarBlock.strippedState(1)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (com/lion/graveyard/blocks/SmolderingPillarBlock.java:32)
- [ ] `SmolderingPillarBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (com/lion/graveyard/blocks/SmolderingPillarBlock.java:36)
- [ ] `SmolderingPillarBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (com/lion/graveyard/blocks/SmolderingPillarBlock.java:41)
- [ ] `SmolderingPillarBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (com/lion/graveyard/blocks/SmolderingPillarBlock.java:45)

### `SmolderingSlabBlock`: it extends Minecraft's `SlabBlock`

- [ ] `SmolderingSlabBlock.SmolderingSlabBlock(1)`: its class isn't translated (it extends Minecraft's `SlabBlock`) (com/lion/graveyard/blocks/SmolderingSlabBlock.java:21)
- [ ] `SmolderingSlabBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `SlabBlock`) (com/lion/graveyard/blocks/SmolderingSlabBlock.java:26)
- [ ] `SmolderingSlabBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `SlabBlock`) (com/lion/graveyard/blocks/SmolderingSlabBlock.java:31)
- [ ] `SmolderingSlabBlock.placeLiquid(4)`: its class isn't translated (it extends Minecraft's `SlabBlock`) (com/lion/graveyard/blocks/SmolderingSlabBlock.java:36)
- [ ] `SmolderingSlabBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `SlabBlock`) (com/lion/graveyard/blocks/SmolderingSlabBlock.java:40)

### `SmolderingStairsBlock`: it extends Minecraft's `StairBlock`

- [ ] `SmolderingStairsBlock.SmolderingStairsBlock(2)`: its class isn't translated (it extends Minecraft's `StairBlock`) (com/lion/graveyard/blocks/SmolderingStairsBlock.java:21)
- [ ] `SmolderingStairsBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `StairBlock`) (com/lion/graveyard/blocks/SmolderingStairsBlock.java:26)
- [ ] `SmolderingStairsBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `StairBlock`) (com/lion/graveyard/blocks/SmolderingStairsBlock.java:31)
- [ ] `SmolderingStairsBlock.placeLiquid(4)`: its class isn't translated (it extends Minecraft's `StairBlock`) (com/lion/graveyard/blocks/SmolderingStairsBlock.java:36)
- [ ] `SmolderingStairsBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `StairBlock`) (com/lion/graveyard/blocks/SmolderingStairsBlock.java:40)

### `SmolderingTrapdoorBlock`: it extends Minecraft's `TrapDoorBlock`

- [ ] `SmolderingTrapdoorBlock.SmolderingTrapdoorBlock(2)`: its class isn't translated (it extends Minecraft's `TrapDoorBlock`) (com/lion/graveyard/blocks/SmolderingTrapdoorBlock.java:22)
- [ ] `SmolderingTrapdoorBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `TrapDoorBlock`) (com/lion/graveyard/blocks/SmolderingTrapdoorBlock.java:27)
- [ ] `SmolderingTrapdoorBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `TrapDoorBlock`) (com/lion/graveyard/blocks/SmolderingTrapdoorBlock.java:32)
- [ ] `SmolderingTrapdoorBlock.placeLiquid(4)`: its class isn't translated (it extends Minecraft's `TrapDoorBlock`) (com/lion/graveyard/blocks/SmolderingTrapdoorBlock.java:37)
- [ ] `SmolderingTrapdoorBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `TrapDoorBlock`) (com/lion/graveyard/blocks/SmolderingTrapdoorBlock.java:41)

### `SmolderingWood`

- [ ] `SmolderingWood.SMOLDERING`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/SmolderingWood.java:27)
- [x] `SmolderingWood.ASH_CONFIG_KEY` (com/lion/graveyard/blocks/SmolderingWood.java:28)
- [x] `SmolderingWood.LIGHT_LEVEL` (com/lion/graveyard/blocks/SmolderingWood.java:29)
- [x] `SmolderingWood.ASH_REACH` (com/lion/graveyard/blocks/SmolderingWood.java:30)
- [x] `SmolderingWood.ASH_PER_CALL` (com/lion/graveyard/blocks/SmolderingWood.java:31)
- [x] `SmolderingWood.WHITE_ASH_ONE_IN` (com/lion/graveyard/blocks/SmolderingWood.java:32)
- [x] `SmolderingWood.SmolderingWood(0)` (com/lion/graveyard/blocks/SmolderingWood.java:34)
- [ ] `SmolderingWood.light(1)`: it uses Minecraft's `BlockState` (com/lion/graveyard/blocks/SmolderingWood.java:37)
- [ ] `SmolderingWood.isFlooded(1)`: it uses Minecraft's `BlockState` (com/lion/graveyard/blocks/SmolderingWood.java:41)
- [ ] `SmolderingWood.use(6)`: it uses Minecraft's `InteractionResult` (com/lion/graveyard/blocks/SmolderingWood.java:45)
- [ ] `SmolderingWood.setLit(4)`: it uses Minecraft's `Level` (com/lion/graveyard/blocks/SmolderingWood.java:98)
- [ ] `SmolderingWood.extinguish(3)`: it uses Minecraft's `Level` (com/lion/graveyard/blocks/SmolderingWood.java:109)
- [ ] `SmolderingWood.displayTick(4)`: it uses Minecraft's `BlockState` (com/lion/graveyard/blocks/SmolderingWood.java:120)
- [ ] `SmolderingWood.ash(3)`: it uses Minecraft's `Level` (com/lion/graveyard/blocks/SmolderingWood.java:157)

### `SmolderingWoodBlock`: it extends Minecraft's `Block`

- [ ] `SmolderingWoodBlock.SmolderingWoodBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/SmolderingWoodBlock.java:18)
- [ ] `SmolderingWoodBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/SmolderingWoodBlock.java:23)
- [ ] `SmolderingWoodBlock.useItemOn(7)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/SmolderingWoodBlock.java:27)
- [ ] `SmolderingWoodBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/SmolderingWoodBlock.java:31)

### `TallDeco`

- [ ] `TallDeco.HALF`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/blocks/TallDeco.java:20)
- [x] `TallDeco.TallDeco(0)` (com/lion/graveyard/blocks/TallDeco.java:22)
- [ ] `TallDeco.placementState(2)`: it uses Minecraft's `BlockState` (com/lion/graveyard/blocks/TallDeco.java:25)
- [ ] `TallDeco.placeUpperHalf(3)`: it uses Minecraft's `Level` (com/lion/graveyard/blocks/TallDeco.java:33)
- [ ] `TallDeco.canPlaceAt(3)`: it uses Minecraft's `BlockState` (com/lion/graveyard/blocks/TallDeco.java:40)
- [ ] `TallDeco.hangingPlacementState(2)`: it uses Minecraft's `BlockState` (com/lion/graveyard/blocks/TallDeco.java:49)
- [ ] `TallDeco.placeLowerHalf(3)`: it uses Minecraft's `Level` (com/lion/graveyard/blocks/TallDeco.java:57)
- [ ] `TallDeco.canHangAt(3)`: it uses Minecraft's `BlockState` (com/lion/graveyard/blocks/TallDeco.java:64)
- [ ] `TallDeco.onNeighborUpdate(5)`: it uses Minecraft's `BlockState` (com/lion/graveyard/blocks/TallDeco.java:74)
- [ ] `TallDeco.onBreak(4)`: it uses Minecraft's `Level` (com/lion/graveyard/blocks/TallDeco.java:86)
- [ ] `TallDeco.isLower(1)`: it uses Minecraft's `BlockState` (com/lion/graveyard/blocks/TallDeco.java:97)

### `TallDecoBlock`: it extends Minecraft's `Block`

- [ ] `TallDecoBlock.FACING`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/blocks/TallDecoBlock.java:37)
- [ ] `TallDecoBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/TallDecoBlock.java:38)
- [ ] `TallDecoBlock.TallDecoBlock(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TallDecoBlock.java:43)
- [ ] `TallDecoBlock.TallDecoBlock(3)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TallDecoBlock.java:47)
- [ ] `TallDecoBlock.TallDecoBlock(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TallDecoBlock.java:51)
- [ ] `TallDecoBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TallDecoBlock.java:62)
- [ ] `TallDecoBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TallDecoBlock.java:66)
- [ ] `TallDecoBlock.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TallDecoBlock.java:73)
- [ ] `TallDecoBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TallDecoBlock.java:81)
- [ ] `TallDecoBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TallDecoBlock.java:85)
- [ ] `TallDecoBlock.playerWillDestroy(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TallDecoBlock.java:103)
- [ ] `TallDecoBlock.getDrops(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TallDecoBlock.java:108)
- [ ] `TallDecoBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TallDecoBlock.java:112)
- [ ] `TallDecoBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TallDecoBlock.java:116)
- [ ] `TallDecoBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TallDecoBlock.java:120)
- [ ] `TallDecoBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TallDecoBlock.java:124)

### `TGDeepslateBlock`: it extends Minecraft's `RotatedPillarBlock`

- [ ] `TGDeepslateBlock.TGDeepslateBlock(2)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (com/lion/graveyard/blocks/TGDeepslateBlock.java:16)
- [ ] `TGDeepslateBlock.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (com/lion/graveyard/blocks/TGDeepslateBlock.java:21)

### `TGDoorBlock`: it extends Minecraft's `DoorBlock`

- [ ] `TGDoorBlock.TGDoorBlock(1)`: its class isn't translated (it extends Minecraft's `DoorBlock`) (com/lion/graveyard/blocks/TGDoorBlock.java:15)
- [ ] `TGDoorBlock.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `DoorBlock`) (com/lion/graveyard/blocks/TGDoorBlock.java:19)

### `TGGrassBlock`: it extends Minecraft's `GrassBlock`

- [ ] `TGGrassBlock.TGGrassBlock(1)`: its class isn't translated (it extends Minecraft's `GrassBlock`) (com/lion/graveyard/blocks/TGGrassBlock.java:12)
- [ ] `TGGrassBlock.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `GrassBlock`) (com/lion/graveyard/blocks/TGGrassBlock.java:16)

### `TGMossBlock`: it extends Minecraft's `BonemealableFeaturePlacerBlock`

- [x] `TGMossBlock.FOG_SHEET_RARITY` (com/lion/graveyard/blocks/TGMossBlock.java:17)
- [ ] `TGMossBlock.TGMossBlock(1)`: its class isn't translated (it extends Minecraft's `BonemealableFeaturePlacerBlock`) (com/lion/graveyard/blocks/TGMossBlock.java:19)
- [ ] `TGMossBlock.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `BonemealableFeaturePlacerBlock`) (com/lion/graveyard/blocks/TGMossBlock.java:23)
- [ ] `TGMossBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `BonemealableFeaturePlacerBlock`) (com/lion/graveyard/blocks/TGMossBlock.java:27)

### `TGStairsBlock`: it extends Minecraft's `StairBlock`

- [ ] `TGStairsBlock.TGStairsBlock(2)`: its class isn't translated (it extends Minecraft's `StairBlock`) (com/lion/graveyard/blocks/TGStairsBlock.java:8)

### `TGStoneBlock`: it extends Minecraft's `Block`

- [ ] `TGStoneBlock.TGStoneBlock(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TGStoneBlock.java:15)
- [ ] `TGStoneBlock.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/TGStoneBlock.java:20)

### `TGTrapDoorBlock`: it extends Minecraft's `TrapDoorBlock`

- [ ] `TGTrapDoorBlock.TGTrapDoorBlock(1)`: its class isn't translated (it extends Minecraft's `TrapDoorBlock`) (com/lion/graveyard/blocks/TGTrapDoorBlock.java:16)
- [ ] `TGTrapDoorBlock.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `TrapDoorBlock`) (com/lion/graveyard/blocks/TGTrapDoorBlock.java:20)

### `UrnBlock`: it extends Minecraft's `BaseEntityBlock`

- [ ] `UrnBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/UrnBlock.java:38)
- [ ] `UrnBlock.FACING`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/blocks/UrnBlock.java:39)
- [ ] `UrnBlock.OPEN`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/UrnBlock.java:40)
- [ ] `UrnBlock.LARGE_URN`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/UrnBlock.java:41)
- [ ] `UrnBlock.SMALL_URN`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/UrnBlock.java:42)
- [ ] `UrnBlock.UrnBlock(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:44)
- [ ] `UrnBlock.propagatesSkylightDown(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:52)
- [ ] `UrnBlock.getShadeBrightness(3)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:56)
- [ ] `UrnBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:60)
- [ ] `UrnBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:64)
- [ ] `UrnBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:70)
- [ ] `UrnBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:74)
- [ ] `UrnBlock.getCollisionShape(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:78)
- [ ] `UrnBlock.isCollisionShapeFullBlock(3)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:82)
- [ ] `UrnBlock.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:86)
- [ ] `UrnBlock.tick(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:99)
- [ ] `UrnBlock.isPathfindable(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:106)
- [ ] `UrnBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:110)
- [ ] `UrnBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:115)
- [ ] `UrnBlock.rotate(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:119)
- [ ] `UrnBlock.mirror(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:123)
- [ ] `UrnBlock.hasAnalogOutputSignal(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (com/lion/graveyard/blocks/UrnBlock.java:127)

### `VariantBoneDisplayBlock`: it extends `BoneDisplayBlock`

- [ ] `VariantBoneDisplayBlock.VARIANT`: it uses Minecraft's `IntegerProperty` (com/lion/graveyard/blocks/VariantBoneDisplayBlock.java:20)
- [ ] `VariantBoneDisplayBlock.VariantBoneDisplayBlock(1)`: its class isn't translated (it extends `BoneDisplayBlock`) (com/lion/graveyard/blocks/VariantBoneDisplayBlock.java:22)
- [ ] `VariantBoneDisplayBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends `BoneDisplayBlock`) (com/lion/graveyard/blocks/VariantBoneDisplayBlock.java:27)
- [ ] `VariantBoneDisplayBlock.getStateForPlacement(1)`: its class isn't translated (it extends `BoneDisplayBlock`) (com/lion/graveyard/blocks/VariantBoneDisplayBlock.java:33)
- [ ] `VariantBoneDisplayBlock.useItemOn(7)`: its class isn't translated (it extends `BoneDisplayBlock`) (com/lion/graveyard/blocks/VariantBoneDisplayBlock.java:41)

### `VaseBlock`: it extends Minecraft's `Block`

- [ ] `VaseBlock.VASES`: it uses Minecraft's `IntegerProperty` (com/lion/graveyard/blocks/VaseBlock.java:23)
- [ ] `VaseBlock.WATERLOGGED`: it uses Minecraft's `BooleanProperty` (com/lion/graveyard/blocks/VaseBlock.java:24)
- [ ] `VaseBlock.VASE_SHAPE_ONE`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/VaseBlock.java:25)
- [ ] `VaseBlock.VASE_SHAPE_TWO`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/VaseBlock.java:26)
- [ ] `VaseBlock.VASE_SHAPE_THREE`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/VaseBlock.java:27)
- [ ] `VaseBlock.VASE_SHAPE_FOUR`: it uses Minecraft's `VoxelShape` (com/lion/graveyard/blocks/VaseBlock.java:28)
- [ ] `VaseBlock.VaseBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/VaseBlock.java:30)
- [ ] `VaseBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/VaseBlock.java:35)
- [ ] `VaseBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/VaseBlock.java:39)
- [ ] `VaseBlock.getFluidState(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/VaseBlock.java:50)
- [ ] `VaseBlock.canBeReplaced(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/VaseBlock.java:54)
- [ ] `VaseBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/VaseBlock.java:60)
- [ ] `VaseBlock.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (com/lion/graveyard/blocks/VaseBlock.java:74)

### `Chronicle`

- [ ] `Chronicle.SIGNED_BOOKS`: it calls `Map` (java.util).entry (com/lion/graveyard/chronicle/Chronicle.java:24)
- [x] `Chronicle.UNSIGNED_BOOKS` (com/lion/graveyard/chronicle/Chronicle.java:35)
- [x] `Chronicle.ACT_ONE` (com/lion/graveyard/chronicle/Chronicle.java:38)
- [x] `Chronicle.ACT_TWO` (com/lion/graveyard/chronicle/Chronicle.java:57)
- [x] `Chronicle.PAGE_ID` (com/lion/graveyard/chronicle/Chronicle.java:78)
- [x] `Chronicle.Chronicle(0)` (com/lion/graveyard/chronicle/Chronicle.java:80)
- [x] `Chronicle.isValidId(1)` (com/lion/graveyard/chronicle/Chronicle.java:83)
- [x] `Chronicle.allPages(0)` (com/lion/graveyard/chronicle/Chronicle.java:87)
- [ ] `Chronicle.find(2)`: it uses Minecraft's `ServerPlayer` (com/lion/graveyard/chronicle/Chronicle.java:93)
- [ ] `Chronicle.syncAll(1)`: it uses Minecraft's `ServerPlayer` (com/lion/graveyard/chronicle/Chronicle.java:117)
- [ ] `Chronicle.pageOf(1)`: it uses Minecraft's `WrittenBookContent` (com/lion/graveyard/chronicle/Chronicle.java:121)

### `ChronicleOpener`

- [ ] `ChronicleOpener.opener`: a function kept in a variable or field (only parameters can be) (com/lion/graveyard/chronicle/ChronicleOpener.java:6)
- [x] `ChronicleOpener.ChronicleOpener(0)` (com/lion/graveyard/chronicle/ChronicleOpener.java:8)
- [ ] `ChronicleOpener.install(1)`: it uses ChronicleOpener.opener, which isn't translated (a function kept in a variable or field (only parameters can be)) (com/lion/graveyard/chronicle/ChronicleOpener.java:11)
- [ ] `ChronicleOpener.open(1)`: it uses ChronicleOpener.opener, which isn't translated (a function kept in a variable or field (only parameters can be)) (com/lion/graveyard/chronicle/ChronicleOpener.java:15)

### `ChronicleState`: it extends Minecraft's `SavedData`

- [x] `ChronicleState.LEGACY_FILE` (com/lion/graveyard/chronicle/ChronicleState.java:24)
- [x] `ChronicleState.MAX_PAGES_PER_PLAYER` (com/lion/graveyard/chronicle/ChronicleState.java:25)
- [ ] `ChronicleState.CODEC`: it uses Minecraft's `Codec` (com/lion/graveyard/chronicle/ChronicleState.java:26)
- [ ] `ChronicleState.TYPE`: it uses Minecraft's `SavedDataType` (com/lion/graveyard/chronicle/ChronicleState.java:27)
- [ ] `ChronicleState.get(1)`: it uses ChronicleState, which isn't translated (it extends Minecraft's `SavedData`) (com/lion/graveyard/chronicle/ChronicleState.java:30)
- [ ] `ChronicleState.importLegacy(1)`: it uses ChronicleState, which isn't translated (it extends Minecraft's `SavedData`) (com/lion/graveyard/chronicle/ChronicleState.java:41)
- [ ] `ChronicleState.found(1)`: its class isn't translated (it extends Minecraft's `SavedData`) (com/lion/graveyard/chronicle/ChronicleState.java:52)
- [ ] `ChronicleState.add(2)`: its class isn't translated (it extends Minecraft's `SavedData`) (com/lion/graveyard/chronicle/ChronicleState.java:56)
- [ ] `ChronicleState.remove(2)`: its class isn't translated (it extends Minecraft's `SavedData`) (com/lion/graveyard/chronicle/ChronicleState.java:74)
- [ ] `ChronicleState.clear(1)`: its class isn't translated (it extends Minecraft's `SavedData`) (com/lion/graveyard/chronicle/ChronicleState.java:84)
- [ ] `ChronicleState.toNbt(0)`: its class isn't translated (it extends Minecraft's `SavedData`) (com/lion/graveyard/chronicle/ChronicleState.java:90)
- [ ] `ChronicleState.fromNbt(1)`: it uses ChronicleState, which isn't translated (it extends Minecraft's `SavedData`) (com/lion/graveyard/chronicle/ChronicleState.java:102)

### `ChronicleButton`: it extends Minecraft's `Button`

- [ ] `ChronicleButton.ChronicleButton(1)`: its class isn't translated (it extends Minecraft's `Button`) (com/lion/graveyard/chronicle/client/ChronicleButton.java:19)
- [ ] `ChronicleButton.addTo(2)`: it uses Minecraft's `Screen` (com/lion/graveyard/chronicle/client/ChronicleButton.java:26)
- [ ] `ChronicleButton.extractContents(4)`: its class isn't translated (it extends Minecraft's `Button`) (com/lion/graveyard/chronicle/client/ChronicleButton.java:32)
- [ ] `ChronicleButton.followInventory(0)`: its class isn't translated (it extends Minecraft's `Button`) (com/lion/graveyard/chronicle/client/ChronicleButton.java:39)
- [ ] `ChronicleButton.hasStatusEffects(0)`: it calls Minecraft's LocalPlayer.getActiveEffects (no Horizon helper for it yet) (com/lion/graveyard/chronicle/client/ChronicleButton.java:49)

### `ChronicleChapters`

- [x] `ChronicleChapters.DIRECTORY` (com/lion/graveyard/chronicle/client/ChronicleChapters.java:23)
- [x] `ChronicleChapters.ChronicleChapters(0)` (com/lion/graveyard/chronicle/client/ChronicleChapters.java:25)
- [ ] `ChronicleChapters.load(0)`: it uses Minecraft's `Resource` (com/lion/graveyard/chronicle/client/ChronicleChapters.java:28)

### `ChronicleChapters.Chapter`

- [x] `ChronicleChapters.Chapter.Chapter(5)` (com/lion/graveyard/chronicle/client/ChronicleChapters.java:61)
- [x] `ChronicleChapters.Chapter.titleKey(0)` (com/lion/graveyard/chronicle/client/ChronicleChapters.java:62)
- [x] `ChronicleChapters.Chapter.id(0)` (com/lion/graveyard/chronicle/client/ChronicleChapters.java:61)
- [x] `ChronicleChapters.Chapter.act(0)` (com/lion/graveyard/chronicle/client/ChronicleChapters.java:61)
- [x] `ChronicleChapters.Chapter.order(0)` (com/lion/graveyard/chronicle/client/ChronicleChapters.java:61)
- [x] `ChronicleChapters.Chapter.source(0)` (com/lion/graveyard/chronicle/client/ChronicleChapters.java:61)
- [x] `ChronicleChapters.Chapter.pages(0)` (com/lion/graveyard/chronicle/client/ChronicleChapters.java:61)

### `ChronicleScreen`: it extends Minecraft's `Screen`

- [x] `ChronicleScreen.BUTTON_TEXTURE` (com/lion/graveyard/chronicle/client/ChronicleScreen.java:25)
- [x] `ChronicleScreen.BOOK_TEXTURE` (com/lion/graveyard/chronicle/client/ChronicleScreen.java:26)
- [x] `ChronicleScreen.BOOK_WIDTH` (com/lion/graveyard/chronicle/client/ChronicleScreen.java:27)
- [x] `ChronicleScreen.BOOK_HEIGHT` (com/lion/graveyard/chronicle/client/ChronicleScreen.java:28)
- [x] `ChronicleScreen.TEXT_WIDTH` (com/lion/graveyard/chronicle/client/ChronicleScreen.java:29)
- [x] `ChronicleScreen.TEXT_TOP` (com/lion/graveyard/chronicle/client/ChronicleScreen.java:30)
- [x] `ChronicleScreen.LINES_PER_PAGE` (com/lion/graveyard/chronicle/client/ChronicleScreen.java:31)
- [x] `ChronicleScreen.LIST_WIDTH` (com/lion/graveyard/chronicle/client/ChronicleScreen.java:32)
- [x] `ChronicleScreen.ROW_HEIGHT` (com/lion/graveyard/chronicle/client/ChronicleScreen.java:33)
- [ ] `ChronicleScreen.ChronicleScreen(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/chronicle/client/ChronicleScreen.java:49)
- [ ] `ChronicleScreen.ChronicleScreen(2)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/chronicle/client/ChronicleScreen.java:53)
- [ ] `ChronicleScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/chronicle/client/ChronicleScreen.java:61)
- [ ] `ChronicleScreen.select(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/chronicle/client/ChronicleScreen.java:79)
- [ ] `ChronicleScreen.paginate(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/chronicle/client/ChronicleScreen.java:86)
- [ ] `ChronicleScreen.turnPage(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/chronicle/client/ChronicleScreen.java:110)
- [ ] `ChronicleScreen.updateButtons(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/chronicle/client/ChronicleScreen.java:115)
- [ ] `ChronicleScreen.extractBackground(4)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/chronicle/client/ChronicleScreen.java:120)
- [ ] `ChronicleScreen.extractList(3)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/chronicle/client/ChronicleScreen.java:126)
- [ ] `ChronicleScreen.progressY(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/chronicle/client/ChronicleScreen.java:168)
- [ ] `ChronicleScreen.extractBook(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/chronicle/client/ChronicleScreen.java:172)
- [ ] `ChronicleScreen.mouseClicked(2)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/chronicle/client/ChronicleScreen.java:196)
- [ ] `ChronicleScreen.mouseScrolled(4)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/chronicle/client/ChronicleScreen.java:233)
- [ ] `ChronicleScreen.onClose(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/chronicle/client/ChronicleScreen.java:244)
- [ ] `ChronicleScreen.isPauseScreen(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/chronicle/client/ChronicleScreen.java:248)

### `ChronicleToast`: it implements Minecraft's `Toast`

- [x] `ChronicleToast.BACKGROUND` (com/lion/graveyard/chronicle/client/ChronicleToast.java:19)
- [x] `ChronicleToast.DURATION` (com/lion/graveyard/chronicle/client/ChronicleToast.java:20)
- [ ] `ChronicleToast.ChronicleToast(1)`: its class isn't translated (it implements Minecraft's `Toast`) (com/lion/graveyard/chronicle/client/ChronicleToast.java:26)
- [ ] `ChronicleToast.getWantedVisibility(0)`: its class isn't translated (it implements Minecraft's `Toast`) (com/lion/graveyard/chronicle/client/ChronicleToast.java:32)
- [ ] `ChronicleToast.update(2)`: its class isn't translated (it implements Minecraft's `Toast`) (com/lion/graveyard/chronicle/client/ChronicleToast.java:36)
- [ ] `ChronicleToast.extractRenderState(3)`: its class isn't translated (it implements Minecraft's `Toast`) (com/lion/graveyard/chronicle/client/ChronicleToast.java:40)

### `ClientChronicle`

- [x] `ClientChronicle.found` (com/lion/graveyard/chronicle/client/ClientChronicle.java:14)
- [x] `ClientChronicle.ClientChronicle(0)` (com/lion/graveyard/chronicle/client/ClientChronicle.java:16)
- [ ] `ClientChronicle.receive(2)`: it calls Graveyard.getConfig, which isn't translated (com/lion/graveyard/chronicle/client/ClientChronicle.java:19)
- [x] `ClientChronicle.found(0)` (com/lion/graveyard/chronicle/client/ClientChronicle.java:27)
- [x] `ClientChronicle.isFound(1)` (com/lion/graveyard/chronicle/client/ClientChronicle.java:31)

### `FragmentSpotHints`

- [x] `FragmentSpotHints.RANGE` (com/lion/graveyard/client/FragmentSpotHints.java:24)
- [x] `FragmentSpotHints.RANGE_UP_DOWN` (com/lion/graveyard/client/FragmentSpotHints.java:25)
- [x] `FragmentSpotHints.SCAN_INTERVAL` (com/lion/graveyard/client/FragmentSpotHints.java:26)
- [x] `FragmentSpotHints.PARTICLE_INTERVAL` (com/lion/graveyard/client/FragmentSpotHints.java:27)
- [x] `FragmentSpotHints.ROW_DISTANCE` (com/lion/graveyard/client/FragmentSpotHints.java:28)
- [x] `FragmentSpotHints.END_OFFSET` (com/lion/graveyard/client/FragmentSpotHints.java:29)
- [ ] `FragmentSpotHints.ALTARS`: it uses Minecraft's `BlockPos` (com/lion/graveyard/client/FragmentSpotHints.java:30)
- [x] `FragmentSpotHints.ticks` (com/lion/graveyard/client/FragmentSpotHints.java:31)
- [x] `FragmentSpotHints.FragmentSpotHints(0)` (com/lion/graveyard/client/FragmentSpotHints.java:33)
- [ ] `FragmentSpotHints.clientTick(1)`: it uses Minecraft's `ClientLevel` (com/lion/graveyard/client/FragmentSpotHints.java:36)
- [ ] `FragmentSpotHints.holdsFragment(1)`: it calls FragmentSpotHints.isFragment, which isn't translated (com/lion/graveyard/client/FragmentSpotHints.java:57)
- [ ] `FragmentSpotHints.isFragment(1)`: `instanceof` (com/lion/graveyard/client/FragmentSpotHints.java:61)
- [ ] `FragmentSpotHints.scanAltars(2)`: it uses Minecraft's `ClientLevel` (com/lion/graveyard/client/FragmentSpotHints.java:65)
- [ ] `FragmentSpotHints.spawnParticles(2)`: it uses Minecraft's `ClientLevel` (com/lion/graveyard/client/FragmentSpotHints.java:82)
- [ ] `FragmentSpotHints.isUsableRow(2)`: it uses Minecraft's `ClientLevel` (com/lion/graveyard/client/FragmentSpotHints.java:104)

### `GraveyardDataTickets`

- [ ] `GraveyardDataTickets.GHOUL_VARIANT`: it uses `DataTicket` (com.geckolib.constant.dataticket) (com/lion/graveyard/client/GraveyardDataTickets.java:9)
- [ ] `GraveyardDataTickets.GHOULING_VARIANT`: it uses `DataTicket` (com.geckolib.constant.dataticket) (com/lion/graveyard/client/GraveyardDataTickets.java:10)
- [ ] `GraveyardDataTickets.GHOULING_HAS_COFFIN`: it uses `DataTicket` (com.geckolib.constant.dataticket) (com/lion/graveyard/client/GraveyardDataTickets.java:11)
- [ ] `GraveyardDataTickets.GHOULING_OFFHAND`: it uses `DataTicket` (com.geckolib.constant.dataticket) (com/lion/graveyard/client/GraveyardDataTickets.java:12)
- [ ] `GraveyardDataTickets.ILLAGER_TEXTURE`: it uses `DataTicket` (com.geckolib.constant.dataticket) (com/lion/graveyard/client/GraveyardDataTickets.java:13)
- [ ] `GraveyardDataTickets.ILLAGER_LEFT_CLAW`: it uses `DataTicket` (com.geckolib.constant.dataticket) (com/lion/graveyard/client/GraveyardDataTickets.java:14)
- [ ] `GraveyardDataTickets.ILLAGER_RIGHT_CLAW`: it uses `DataTicket` (com.geckolib.constant.dataticket) (com/lion/graveyard/client/GraveyardDataTickets.java:15)
- [ ] `GraveyardDataTickets.CORVID_SPECIES`: it uses `DataTicket` (com.geckolib.constant.dataticket) (com/lion/graveyard/client/GraveyardDataTickets.java:16)
- [ ] `GraveyardDataTickets.CORVID_LOOK_WEIGHT`: it uses `DataTicket` (com.geckolib.constant.dataticket) (com/lion/graveyard/client/GraveyardDataTickets.java:17)
- [ ] `GraveyardDataTickets.LICH_PHASE`: it uses `DataTicket` (com.geckolib.constant.dataticket) (com/lion/graveyard/client/GraveyardDataTickets.java:18)
- [x] `GraveyardDataTickets.GraveyardDataTickets(0)` (com/lion/graveyard/client/GraveyardDataTickets.java:20)

### `VialChargedProperty`: it implements Minecraft's `RangeSelectItemModelProperty`

- [ ] `VialChargedProperty.MAP_CODEC`: it uses Minecraft's `MapCodec` (com/lion/graveyard/client/VialChargedProperty.java:16)
- [ ] `VialChargedProperty.VialChargedProperty(0)`: its class isn't translated (it implements Minecraft's `RangeSelectItemModelProperty`) (com/lion/graveyard/client/VialChargedProperty.java:15)
- [ ] `VialChargedProperty.get(4)`: its class isn't translated (it implements Minecraft's `RangeSelectItemModelProperty`) (com/lion/graveyard/client/VialChargedProperty.java:18)
- [ ] `VialChargedProperty.type(0)`: its class isn't translated (it implements Minecraft's `RangeSelectItemModelProperty`) (com/lion/graveyard/client/VialChargedProperty.java:23)

### `ConfigFiles`

- [x] `ConfigFiles.FILE_NAME` (com/lion/graveyard/config/ConfigFiles.java:12)
- [x] `ConfigFiles.LEGACY_FILE_NAME` (com/lion/graveyard/config/ConfigFiles.java:13)
- [x] `ConfigFiles.MAX_LISTED` (com/lion/graveyard/config/ConfigFiles.java:14)
- [x] `ConfigFiles.ConfigFiles(0)` (com/lion/graveyard/config/ConfigFiles.java:16)
- [ ] `ConfigFiles.load(2)`: it uses ConfigValues, which isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigFiles.java:19)
- [ ] `ConfigFiles.loadCurrent(4)`: it uses ConfigValues, which isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigFiles.java:42)
- [ ] `ConfigFiles.migrate(4)`: it uses ConfigValues, which isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigFiles.java:64)
- [ ] `ConfigFiles.write(4)`: it uses ConfigValues, which isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigFiles.java:78)
- [ ] `ConfigFiles.deleteTemporary(2)`: it uses ConfigLog, which isn't translated (it's an interface) (com/lion/graveyard/config/ConfigFiles.java:91)
- [x] `ConfigFiles.deleteQuietly(1)` (com/lion/graveyard/config/ConfigFiles.java:99)
- [ ] `ConfigFiles.backup(3)`: it uses ConfigLog, which isn't translated (it's an interface) (com/lion/graveyard/config/ConfigFiles.java:106)
- [x] `ConfigFiles.move(2)` (com/lion/graveyard/config/ConfigFiles.java:114)
- [ ] `ConfigFiles.reportUnreadable(3)`: it uses ConfigLog, which isn't translated (it's an interface) (com/lion/graveyard/config/ConfigFiles.java:122)
- [ ] `ConfigFiles.reportProblems(4)`: it uses Result, which isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)) (com/lion/graveyard/config/ConfigFiles.java:134)
- [ ] `ConfigFiles.listed(1)`: List.subList (com/lion/graveyard/config/ConfigFiles.java:149)
- [x] `ConfigFiles.normalizeLineEnds(1)` (com/lion/graveyard/config/ConfigFiles.java:153)

### `ConfigLog`: it's an interface

- [ ] `ConfigLog.info(1)`: it has no body (com/lion/graveyard/config/ConfigLog.java:4)
- [ ] `ConfigLog.warn(1)`: it has no body (com/lion/graveyard/config/ConfigLog.java:6)
- [ ] `ConfigLog.error(1)`: it has no body (com/lion/graveyard/config/ConfigLog.java:8)

### `ConfigReader`

- [x] `ConfigReader.ConfigReader(0)` (com/lion/graveyard/config/ConfigReader.java:14)
- [ ] `ConfigReader.read(1)`: unknown name `exception` (com/lion/graveyard/config/ConfigReader.java:17)
- [ ] `ConfigReader.parseObject(2)`: it uses `JsonReader` (com.google.gson.stream) (com/lion/graveyard/config/ConfigReader.java:32)
- [ ] `ConfigReader.hasContentAfter(1)`: it uses `JsonReader` (com.google.gson.stream) (com/lion/graveyard/config/ConfigReader.java:45)
- [ ] `ConfigReader.readSection(3)`: it uses Section, which isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigReader.java:53)
- [ ] `ConfigReader.hasChild(2)`: it uses Section, which isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigReader.java:77)
- [ ] `ConfigReader.readNested(3)`: it uses Section, which isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigReader.java:87)

### `ConfigReader.Result`: it uses ConfigValues, which isn't translated (it uses `Object`)

- [ ] `ConfigReader.Result.Result(2)`: its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)) (com/lion/graveyard/config/ConfigReader.java:102)
- [ ] `ConfigReader.Result.values(0)`: its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)) (com/lion/graveyard/config/ConfigReader.java:107)
- [ ] `ConfigReader.Result.failed(0)`: its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)) (com/lion/graveyard/config/ConfigReader.java:111)
- [ ] `ConfigReader.Result.failure(0)`: its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)) (com/lion/graveyard/config/ConfigReader.java:115)
- [ ] `ConfigReader.Result.reset(0)`: its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)) (com/lion/graveyard/config/ConfigReader.java:119)
- [ ] `ConfigReader.Result.dropped(0)`: its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)) (com/lion/graveyard/config/ConfigReader.java:123)
- [ ] `ConfigReader.Result.hasTrailingContent(0)`: its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)) (com/lion/graveyard/config/ConfigReader.java:127)
- [ ] `ConfigReader.Result.hasProblems(0)`: its class isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)) (com/lion/graveyard/config/ConfigReader.java:131)

### `ConfigRules`

- [x] `ConfigRules.ConfigRules(0)` (com/lion/graveyard/config/ConfigRules.java:6)
- [ ] `ConfigRules.apply(2)`: it uses ConfigValues, which isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigRules.java:9)
- [ ] `ConfigRules.raise(4)`: it uses `Object` (com/lion/graveyard/config/ConfigRules.java:23)

### `ConfigSchema`: it uses Section, which isn't translated (it extends `Node`)

- [x] `ConfigSchema.STRUCTURE_IDS` (com/lion/graveyard/config/ConfigSchema.java:11)
- [x] `ConfigSchema.NAMELESS_HANGED` (com/lion/graveyard/config/ConfigSchema.java:36)
- [x] `ConfigSchema.MIN_MULTIPLIER` (com/lion/graveyard/config/ConfigSchema.java:37)
- [x] `ConfigSchema.MOBS` (com/lion/graveyard/config/ConfigSchema.java:38)
- [x] `ConfigSchema.FULL_MOB_IDS` (com/lion/graveyard/config/ConfigSchema.java:55)
- [x] `ConfigSchema.ALL_MOB_IDS` (com/lion/graveyard/config/ConfigSchema.java:56)
- [ ] `ConfigSchema.INSTANCE`: it uses ConfigSchema, which isn't translated (it uses Section, which isn't translated (it extends `Node`)) (com/lion/graveyard/config/ConfigSchema.java:57)
- [ ] `ConfigSchema.ConfigSchema(0)`: its class isn't translated (it uses Section, which isn't translated (it extends `Node`)) (com/lion/graveyard/config/ConfigSchema.java:62)
- [ ] `ConfigSchema.root(0)`: its class isn't translated (it uses Section, which isn't translated (it extends `Node`)) (com/lion/graveyard/config/ConfigSchema.java:73)
- [ ] `ConfigSchema.leaves(0)`: its class isn't translated (it uses Section, which isn't translated (it extends `Node`)) (com/lion/graveyard/config/ConfigSchema.java:77)
- [ ] `ConfigSchema.leaf(1)`: its class isn't translated (it uses Section, which isn't translated (it extends `Node`)) (com/lion/graveyard/config/ConfigSchema.java:81)
- [ ] `ConfigSchema.leafPaths(0)`: its class isn't translated (it uses Section, which isn't translated (it extends `Node`)) (com/lion/graveyard/config/ConfigSchema.java:85)
- [x] `ConfigSchema.isFullMob(1)` (com/lion/graveyard/config/ConfigSchema.java:89)
- [ ] `ConfigSchema.collect(1)`: its class isn't translated (it uses Section, which isn't translated (it extends `Node`)) (com/lion/graveyard/config/ConfigSchema.java:93)
- [ ] `ConfigSchema.buildGeneral(0)`: its class isn't translated (it uses Section, which isn't translated (it extends `Node`)) (com/lion/graveyard/config/ConfigSchema.java:105)
- [ ] `ConfigSchema.buildStructures(0)`: its class isn't translated (it uses Section, which isn't translated (it extends `Node`)) (com/lion/graveyard/config/ConfigSchema.java:113)
- [ ] `ConfigSchema.buildMobs(0)`: its class isn't translated (it uses Section, which isn't translated (it extends `Node`)) (com/lion/graveyard/config/ConfigSchema.java:154)
- [ ] `ConfigSchema.buildHordes(0)`: its class isn't translated (it uses Section, which isn't translated (it extends `Node`)) (com/lion/graveyard/config/ConfigSchema.java:180)
- [ ] `ConfigSchema.buildChampion(0)`: its class isn't translated (it uses Section, which isn't translated (it extends `Node`)) (com/lion/graveyard/config/ConfigSchema.java:192)
- [ ] `ConfigSchema.buildChronicle(0)`: its class isn't translated (it uses Section, which isn't translated (it extends `Node`)) (com/lion/graveyard/config/ConfigSchema.java:236)
- [ ] `ConfigSchema.buildBehaviour(0)`: its class isn't translated (it uses Section, which isn't translated (it extends `Node`)) (com/lion/graveyard/config/ConfigSchema.java:243)
- [x] `ConfigSchema.sentence(1)` (com/lion/graveyard/config/ConfigSchema.java:266)

### `ConfigSchema.Coerced`: it uses `Object`

- [ ] `ConfigSchema.Coerced.Coerced(2)`: its class isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigSchema.java:271)
- [ ] `ConfigSchema.Coerced.value(0)`: its class isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigSchema.java:271)
- [ ] `ConfigSchema.Coerced.exact(0)`: its class isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigSchema.java:271)

### `ConfigSchema.Kind`


### `ConfigSchema.Leaf`: it extends `Node`

- [ ] `ConfigSchema.Leaf.Leaf(9)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:291)
- [ ] `ConfigSchema.Leaf.kind(0)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:311)
- [ ] `ConfigSchema.Leaf.defaultValue(0)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:315)
- [ ] `ConfigSchema.Leaf.coerce(1)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:319)
- [ ] `ConfigSchema.Leaf.coerceBool(1)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:333)
- [ ] `ConfigSchema.Leaf.coerceNumber(2)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:338)
- [ ] `ConfigSchema.Leaf.coerceFloored(1)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:365)
- [ ] `ConfigSchema.Leaf.bounded(1)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:374)
- [ ] `ConfigSchema.Leaf.box(1)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:391)
- [ ] `ConfigSchema.Leaf.coerceMultiplier(1)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:399)
- [ ] `ConfigSchema.Leaf.coerceStrings(1)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:406)
- [ ] `ConfigSchema.Leaf.invalid(0)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:424)

### `ConfigSchema.MobDefaults`

- [x] `ConfigSchema.MobDefaults.MobDefaults(6)` (com/lion/graveyard/config/ConfigSchema.java:429)
- [x] `ConfigSchema.MobDefaults.id(0)` (com/lion/graveyard/config/ConfigSchema.java:429)
- [x] `ConfigSchema.MobDefaults.enabled(0)` (com/lion/graveyard/config/ConfigSchema.java:429)
- [x] `ConfigSchema.MobDefaults.weight(0)` (com/lion/graveyard/config/ConfigSchema.java:429)
- [x] `ConfigSchema.MobDefaults.minGroup(0)` (com/lion/graveyard/config/ConfigSchema.java:429)
- [x] `ConfigSchema.MobDefaults.maxGroup(0)` (com/lion/graveyard/config/ConfigSchema.java:429)
- [x] `ConfigSchema.MobDefaults.burns(0)` (com/lion/graveyard/config/ConfigSchema.java:429)

### `ConfigSchema.Node`

- [x] `ConfigSchema.Node.Node(3)` (com/lion/graveyard/config/ConfigSchema.java:437)
- [x] `ConfigSchema.Node.name(0)` (com/lion/graveyard/config/ConfigSchema.java:443)
- [x] `ConfigSchema.Node.path(0)` (com/lion/graveyard/config/ConfigSchema.java:447)
- [x] `ConfigSchema.Node.comment(0)` (com/lion/graveyard/config/ConfigSchema.java:451)

### `ConfigSchema.Section`: it extends `Node`

- [ ] `ConfigSchema.Section.Section(4)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:460)
- [ ] `ConfigSchema.Section.children(0)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:465)
- [ ] `ConfigSchema.Section.isTable(0)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:469)
- [ ] `ConfigSchema.Section.section(2)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:473)
- [ ] `ConfigSchema.Section.table(1)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:479)
- [ ] `ConfigSchema.Section.bool(3)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:485)
- [ ] `ConfigSchema.Section.integer(5)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:489)
- [ ] `ConfigSchema.Section.real(5)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:505)
- [ ] `ConfigSchema.Section.realAtLeast(3)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:509)
- [ ] `ConfigSchema.Section.floating(5)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:513)
- [ ] `ConfigSchema.Section.floatAbove(3)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:517)
- [ ] `ConfigSchema.Section.multiplier(2)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:521)
- [ ] `ConfigSchema.Section.strings(3)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:525)
- [ ] `ConfigSchema.Section.add(1)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:531)
- [ ] `ConfigSchema.Section.childPath(1)`: its class isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigSchema.java:535)

### `ConfigValues`: it uses `Object`

- [ ] `ConfigValues.defaults(0)`: it uses ConfigValues, which isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigValues.java:12)
- [ ] `ConfigValues.copy(0)`: its class isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigValues.java:22)
- [ ] `ConfigValues.has(1)`: its class isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigValues.java:28)
- [ ] `ConfigValues.bool(1)`: its class isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigValues.java:32)
- [ ] `ConfigValues.integer(1)`: its class isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigValues.java:36)
- [ ] `ConfigValues.real(1)`: its class isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigValues.java:40)
- [ ] `ConfigValues.floating(1)`: its class isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigValues.java:44)
- [ ] `ConfigValues.strings(1)`: its class isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigValues.java:48)
- [ ] `ConfigValues.get(1)`: its class isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigValues.java:52)
- [ ] `ConfigValues.set(2)`: its class isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigValues.java:61)
- [ ] `ConfigValues.asMap(0)`: its class isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigValues.java:69)
- [ ] `ConfigValues.equals(1)`: its class isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigValues.java:73)
- [ ] `ConfigValues.hashCode(0)`: its class isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigValues.java:78)

### `ConfigWriter`

- [x] `ConfigWriter.HEADER` (com/lion/graveyard/config/ConfigWriter.java:10)
- [x] `ConfigWriter.INDENT` (com/lion/graveyard/config/ConfigWriter.java:11)
- [x] `ConfigWriter.ConfigWriter(0)` (com/lion/graveyard/config/ConfigWriter.java:13)
- [ ] `ConfigWriter.write(1)`: it uses ConfigValues, which isn't translated (it uses `Object`) (com/lion/graveyard/config/ConfigWriter.java:16)
- [ ] `ConfigWriter.writeChildren(6)`: it uses Section, which isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigWriter.java:25)
- [ ] `ConfigWriter.writeSection(8)`: it uses Section, which isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigWriter.java:42)
- [ ] `ConfigWriter.writeLeaf(6)`: it uses Leaf, which isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigWriter.java:54)
- [x] `ConfigWriter.line(3)` (com/lion/graveyard/config/ConfigWriter.java:62)
- [ ] `ConfigWriter.format(2)`: it uses Leaf, which isn't translated (it extends `Node`) (com/lion/graveyard/config/ConfigWriter.java:68)
- [ ] `ConfigWriter.decimal(1)`: it calls `BigDecimal` (java.math).valueOf (com/lion/graveyard/config/ConfigWriter.java:76)
- [ ] `ConfigWriter.list(1)`: it uses `Object` (com/lion/graveyard/config/ConfigWriter.java:81)

### `CorruptedChampionEntry`

- [x] `CorruptedChampionEntry.bossBarDarkenSky(0)` (com/lion/graveyard/config/entries/CorruptedChampionEntry.java:42)
- [x] `CorruptedChampionEntry.bossBarThickenFog(0)` (com/lion/graveyard/config/entries/CorruptedChampionEntry.java:46)
- [ ] `CorruptedChampionEntry.bossMusicVolume(0)`: arithmetic on float (or null) and float (com/lion/graveyard/config/entries/CorruptedChampionEntry.java:50)
- [ ] `CorruptedChampionEntry.spawnInvulnerableTicks(0)`: arithmetic on int and int (or null) (com/lion/graveyard/config/entries/CorruptedChampionEntry.java:54)
- [ ] `CorruptedChampionEntry.phaseInvulnerableTicks(0)`: arithmetic on int and int (or null) (com/lion/graveyard/config/entries/CorruptedChampionEntry.java:58)
- [ ] `CorruptedChampionEntry.cooldownHunt(0)`: arithmetic on int and int (or null) (com/lion/graveyard/config/entries/CorruptedChampionEntry.java:62)
- [ ] `CorruptedChampionEntry.cooldownCharge(0)`: arithmetic on int and int (or null) (com/lion/graveyard/config/entries/CorruptedChampionEntry.java:66)
- [ ] `CorruptedChampionEntry.cooldownPounce(0)`: arithmetic on int and int (or null) (com/lion/graveyard/config/entries/CorruptedChampionEntry.java:70)
- [x] `CorruptedChampionEntry.healThresholdHealth(0)` (com/lion/graveyard/config/entries/CorruptedChampionEntry.java:74)
- [ ] `CorruptedChampionEntry.skullExplosionRadius(0)`: arithmetic on float (or null) and float (com/lion/graveyard/config/entries/CorruptedChampionEntry.java:78)
- [x] `CorruptedChampionEntry.skullsDestroyBlocks(0)` (com/lion/graveyard/config/entries/CorruptedChampionEntry.java:82)
- [x] `CorruptedChampionEntry.CorruptedChampionEntry(24)` (com/lion/graveyard/config/entries/CorruptedChampionEntry.java:86)
- [x] `CorruptedChampionEntry.of(24)` (com/lion/graveyard/config/entries/CorruptedChampionEntry.java:138)

### `HordeConfigEntry`

- [x] `HordeConfigEntry.HordeConfigEntry(3)` (com/lion/graveyard/config/entries/HordeConfigEntry.java:13)
- [x] `HordeConfigEntry.HordeConfigEntry(8)` (com/lion/graveyard/config/entries/HordeConfigEntry.java:17)
- [ ] `HordeConfigEntry.chance(0)`: arithmetic on double and double (or null) (com/lion/graveyard/config/entries/HordeConfigEntry.java:37)
- [ ] `HordeConfigEntry.nearDistance(0)`: arithmetic on int and int (or null) (com/lion/graveyard/config/entries/HordeConfigEntry.java:41)
- [ ] `HordeConfigEntry.farDistance(0)`: it calls HordeConfigEntry.nearDistance, which isn't translated (com/lion/graveyard/config/entries/HordeConfigEntry.java:45)
- [x] `HordeConfigEntry.onlyAtNight(0)` (com/lion/graveyard/config/entries/HordeConfigEntry.java:49)
- [x] `HordeConfigEntry.of(8)` (com/lion/graveyard/config/entries/HordeConfigEntry.java:53)
- [x] `HordeConfigEntry.of(3)` (com/lion/graveyard/config/entries/HordeConfigEntry.java:68)

### `MobConfigEntry`

- [x] `MobConfigEntry.MobConfigEntry(9)` (com/lion/graveyard/config/entries/MobConfigEntry.java:14)
- [ ] `MobConfigEntry.health(0)`: it calls MobConfigEntry.multiplier, which isn't translated (com/lion/graveyard/config/entries/MobConfigEntry.java:36)
- [ ] `MobConfigEntry.damage(0)`: it calls MobConfigEntry.multiplier, which isn't translated (com/lion/graveyard/config/entries/MobConfigEntry.java:40)
- [ ] `MobConfigEntry.speed(0)`: it calls MobConfigEntry.multiplier, which isn't translated (com/lion/graveyard/config/entries/MobConfigEntry.java:44)
- [ ] `MobConfigEntry.multiplier(1)`: arithmetic on double and double (or null) (com/lion/graveyard/config/entries/MobConfigEntry.java:48)
- [x] `MobConfigEntry.of(9)` (com/lion/graveyard/config/entries/MobConfigEntry.java:52)
- [x] `MobConfigEntry.of(3)` (com/lion/graveyard/config/entries/MobConfigEntry.java:66)
- [x] `MobConfigEntry.of(6)` (com/lion/graveyard/config/entries/MobConfigEntry.java:70)

### `ParticleConfigEntry`

- [x] `ParticleConfigEntry.ParticleConfigEntry(1)` (com/lion/graveyard/config/entries/ParticleConfigEntry.java:7)
- [x] `ParticleConfigEntry.ParticleConfigEntry(2)` (com/lion/graveyard/config/entries/ParticleConfigEntry.java:11)
- [x] `ParticleConfigEntry.of(2)` (com/lion/graveyard/config/entries/ParticleConfigEntry.java:16)
- [x] `ParticleConfigEntry.of(1)` (com/lion/graveyard/config/entries/ParticleConfigEntry.java:20)

### `StructureConfigEntry`

- [x] `StructureConfigEntry.StructureConfigEntry(4)` (com/lion/graveyard/config/entries/StructureConfigEntry.java:9)
- [ ] `StructureConfigEntry.spawnChance(0)`: arithmetic on double and double (or null) (com/lion/graveyard/config/entries/StructureConfigEntry.java:16)
- [x] `StructureConfigEntry.lowestY(1)` (com/lion/graveyard/config/entries/StructureConfigEntry.java:20)
- [x] `StructureConfigEntry.highestY(1)` (com/lion/graveyard/config/entries/StructureConfigEntry.java:24)
- [x] `StructureConfigEntry.of(4)` (com/lion/graveyard/config/entries/StructureConfigEntry.java:28)
- [x] `StructureConfigEntry.of(0)` (com/lion/graveyard/config/entries/StructureConfigEntry.java:32)

### `GraveyardConfig`

- [x] `GraveyardConfig.CHAMPION_ID` (com/lion/graveyard/config/GraveyardConfig.java:13)
- [x] `GraveyardConfig.HORDE_ID` (com/lion/graveyard/config/GraveyardConfig.java:14)
- [x] `GraveyardConfig.FOG_ID` (com/lion/graveyard/config/GraveyardConfig.java:15)
- [x] `GraveyardConfig.BOOLEAN_KEYS` (com/lion/graveyard/config/GraveyardConfig.java:16)
- [x] `GraveyardConfig.NUMBER_KEYS` (com/lion/graveyard/config/GraveyardConfig.java:33)
- [ ] `GraveyardConfig.GraveyardConfig(1)`: it uses ConfigValues, which isn't translated (it uses `Object`) (com/lion/graveyard/config/GraveyardConfig.java:47)
- [x] `GraveyardConfig.enabled(1)` (com/lion/graveyard/config/GraveyardConfig.java:77)
- [x] `GraveyardConfig.isOverrunVariant(1)` (com/lion/graveyard/config/GraveyardConfig.java:82)
- [x] `GraveyardConfig.structureSwitch(1)` (com/lion/graveyard/config/GraveyardConfig.java:86)
- [ ] `GraveyardConfig.structureChance(1)`: it calls StructureConfigEntry.spawnChance, which isn't translated (com/lion/graveyard/config/GraveyardConfig.java:91)
- [x] `GraveyardConfig.flag(2)` (com/lion/graveyard/config/GraveyardConfig.java:98)
- [x] `GraveyardConfig.structuresOnly(0)` (com/lion/graveyard/config/GraveyardConfig.java:103)
- [x] `GraveyardConfig.mobMayAppear(1)` (com/lion/graveyard/config/GraveyardConfig.java:107)
- [x] `GraveyardConfig.number(2)` (com/lion/graveyard/config/GraveyardConfig.java:112)
- [x] `GraveyardConfig.mobEntry(1)` (com/lion/graveyard/config/GraveyardConfig.java:117)
- [x] `GraveyardConfig.fogSpawn(1)` (com/lion/graveyard/config/GraveyardConfig.java:122)
- [x] `GraveyardConfig.getStructure(1)` (com/lion/graveyard/config/GraveyardConfig.java:126)
- [x] `GraveyardConfig.getParticle(1)` (com/lion/graveyard/config/GraveyardConfig.java:131)
- [x] `GraveyardConfig.getHorde(1)` (com/lion/graveyard/config/GraveyardConfig.java:136)
- [ ] `GraveyardConfig.structure(2)`: it uses ConfigValues, which isn't translated (it uses `Object`) (com/lion/graveyard/config/GraveyardConfig.java:141)
- [ ] `GraveyardConfig.mob(2)`: it uses ConfigValues, which isn't translated (it uses `Object`) (com/lion/graveyard/config/GraveyardConfig.java:151)
- [ ] `GraveyardConfig.horde(1)`: it uses ConfigValues, which isn't translated (it uses `Object`) (com/lion/graveyard/config/GraveyardConfig.java:165)
- [ ] `GraveyardConfig.champion(1)`: it uses ConfigValues, which isn't translated (it uses `Object`) (com/lion/graveyard/config/GraveyardConfig.java:178)

### `LegacyFile`


### `LegacyFile.Champion`


### `LegacyFile.Horde`


### `LegacyFile.Mob`


### `LegacyFile.Particle`


### `LegacyFile.Structure`


### `LegacyKeys`

- [x] `LegacyKeys.BOOLEANS` (com/lion/graveyard/config/LegacyKeys.java:7)
- [x] `LegacyKeys.NUMBERS` (com/lion/graveyard/config/LegacyKeys.java:23)
- [x] `LegacyKeys.MOB_FIELDS` (com/lion/graveyard/config/LegacyKeys.java:26)
- [x] `LegacyKeys.HORDE_FIELDS` (com/lion/graveyard/config/LegacyKeys.java:36)
- [x] `LegacyKeys.CHAMPION_FIELDS` (com/lion/graveyard/config/LegacyKeys.java:46)
- [ ] `LegacyKeys.ENTRIES`: its value reads other changing statics (com/lion/graveyard/config/LegacyKeys.java:83)
- [x] `LegacyKeys.LegacyKeys(0)` (com/lion/graveyard/config/LegacyKeys.java:85)
- [ ] `LegacyKeys.entries(0)`: it uses LegacyKeys.ENTRIES, which isn't translated (its value reads other changing statics) (com/lion/graveyard/config/LegacyKeys.java:88)
- [x] `LegacyKeys.build(0)` (com/lion/graveyard/config/LegacyKeys.java:92)
- [x] `LegacyKeys.copy(3)` (com/lion/graveyard/config/LegacyKeys.java:142)

### `LegacyKeys.Entry`

- [x] `LegacyKeys.Entry.Entry(3)` (com/lion/graveyard/config/LegacyKeys.java:146)
- [x] `LegacyKeys.Entry.oldPath(0)` (com/lion/graveyard/config/LegacyKeys.java:146)
- [x] `LegacyKeys.Entry.newPath(0)` (com/lion/graveyard/config/LegacyKeys.java:146)
- [x] `LegacyKeys.Entry.rule(0)` (com/lion/graveyard/config/LegacyKeys.java:146)

### `LegacyKeys.Rule`


### `LegacyMigration`

- [x] `LegacyMigration.GSON` (com/lion/graveyard/config/LegacyMigration.java:23)
- [x] `LegacyMigration.SWITCH_ONLY_MOBS` (com/lion/graveyard/config/LegacyMigration.java:24)
- [ ] `LegacyMigration.OLD_PATH_BY_NEW_PATH`: it calls LegacyMigration.oldPathsByNewPath, which isn't translated (com/lion/graveyard/config/LegacyMigration.java:25)
- [ ] `LegacyMigration.KNOWN_PREFIXES`: it calls LegacyMigration.knownPrefixes, which isn't translated (com/lion/graveyard/config/LegacyMigration.java:26)
- [x] `LegacyMigration.LegacyMigration(0)` (com/lion/graveyard/config/LegacyMigration.java:28)
- [ ] `LegacyMigration.migrate(1)`: unknown name `exception` (com/lion/graveyard/config/LegacyMigration.java:31)
- [ ] `LegacyMigration.rejected(1)`: it uses Result, which isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)) (com/lion/graveyard/config/LegacyMigration.java:72)
- [ ] `LegacyMigration.joinLines(1)`: it creates `BufferedReader` (java.io) (com/lion/graveyard/config/LegacyMigration.java:76)
- [ ] `LegacyMigration.holdsNonFiniteNumber(1)`: `null` (Rust has no null: this needs an Option) (com/lion/graveyard/config/LegacyMigration.java:82)
- [ ] `LegacyMigration.copyValue(3)`: it uses `Object` (com/lion/graveyard/config/LegacyMigration.java:110)
- [ ] `LegacyMigration.mergeMobSwitch(3)`: it uses Result, which isn't translated (it uses ConfigValues, which isn't translated (it uses `Object`)) (com/lion/graveyard/config/LegacyMigration.java:122)
- [ ] `LegacyMigration.rawValue(2)`: it uses `Object` (com/lion/graveyard/config/LegacyMigration.java:135)
- [ ] `LegacyMigration.entryMap(2)`: it uses `Object` (com/lion/graveyard/config/LegacyMigration.java:145)
- [ ] `LegacyMigration.fieldOf(3)`: it uses `Object` (com/lion/graveyard/config/LegacyMigration.java:156)
- [ ] `LegacyMigration.toJson(1)`: it uses `Object` (com/lion/graveyard/config/LegacyMigration.java:171)
- [x] `LegacyMigration.addOnce(2)` (com/lion/graveyard/config/LegacyMigration.java:189)
- [ ] `LegacyMigration.collectUnknownKeys(2)`: it uses `JsonReader` (com.google.gson.stream) (com/lion/graveyard/config/LegacyMigration.java:195)
- [ ] `LegacyMigration.collectUnknown(3)`: it uses LegacyMigration.KNOWN_PREFIXES, which isn't translated (its value reads other changing statics) (com/lion/graveyard/config/LegacyMigration.java:204)
- [ ] `LegacyMigration.knownPrefixes(0)`: it calls LegacyKeys.entries, which isn't translated (com/lion/graveyard/config/LegacyMigration.java:215)
- [ ] `LegacyMigration.oldPathsByNewPath(0)`: it calls LegacyKeys.entries, which isn't translated (com/lion/graveyard/config/LegacyMigration.java:230)

### `Slf4jConfigLog`: it implements `ConfigLog`

- [ ] `Slf4jConfigLog.Slf4jConfigLog(1)`: its class isn't translated (it implements `ConfigLog`) (com/lion/graveyard/config/Slf4jConfigLog.java:8)
- [ ] `Slf4jConfigLog.info(1)`: its class isn't translated (it implements `ConfigLog`) (com/lion/graveyard/config/Slf4jConfigLog.java:12)
- [ ] `Slf4jConfigLog.warn(1)`: its class isn't translated (it implements `ConfigLog`) (com/lion/graveyard/config/Slf4jConfigLog.java:17)
- [ ] `Slf4jConfigLog.error(1)`: its class isn't translated (it implements `ConfigLog`) (com/lion/graveyard/config/Slf4jConfigLog.java:22)

### `AcolyteEntity`: it extends `CorruptedIllager`

- [ ] `AcolyteEntity.AcolyteEntity(2)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AcolyteEntity.java:20)
- [ ] `AcolyteEntity.populateDefaultEquipmentSlots(2)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AcolyteEntity.java:24)
- [ ] `AcolyteEntity.playAmbientSound(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AcolyteEntity.java:28)
- [ ] `AcolyteEntity.playHurtSound(1)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AcolyteEntity.java:32)
- [ ] `AcolyteEntity.die(1)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AcolyteEntity.java:36)
- [ ] `AcolyteEntity.createWitherRose(1)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AcolyteEntity.java:41)

### `ChaseTarget`

- [x] `ChaseTarget.ChaseTarget(0)` (com/lion/graveyard/entities/ai/ChaseTarget.java:9)
- [ ] `ChaseTarget.isLive(1)`: it uses Minecraft's `LivingEntity` (com/lion/graveyard/entities/ai/ChaseTarget.java:12)

### `CorvidMoveControl`: it extends Minecraft's `FlyingMoveControl`

- [x] `CorvidMoveControl.MAX_PITCH_CHANGE` (com/lion/graveyard/entities/ai/control/CorvidMoveControl.java:11)
- [x] `CorvidMoveControl.VERTICAL_DEADZONE` (com/lion/graveyard/entities/ai/control/CorvidMoveControl.java:12)
- [x] `CorvidMoveControl.VERTICAL_GAIN` (com/lion/graveyard/entities/ai/control/CorvidMoveControl.java:13)
- [x] `CorvidMoveControl.CRUISE_SPEED` (com/lion/graveyard/entities/ai/control/CorvidMoveControl.java:14)
- [x] `CorvidMoveControl.MAX_SINK` (com/lion/graveyard/entities/ai/control/CorvidMoveControl.java:15)
- [x] `CorvidMoveControl.MAX_CLIMB` (com/lion/graveyard/entities/ai/control/CorvidMoveControl.java:16)
- [x] `CorvidMoveControl.COAST_THRUST` (com/lion/graveyard/entities/ai/control/CorvidMoveControl.java:17)
- [ ] `CorvidMoveControl.CorvidMoveControl(1)`: its class isn't translated (it extends Minecraft's `FlyingMoveControl`) (com/lion/graveyard/entities/ai/control/CorvidMoveControl.java:20)
- [ ] `CorvidMoveControl.tick(0)`: its class isn't translated (it extends Minecraft's `FlyingMoveControl`) (com/lion/graveyard/entities/ai/control/CorvidMoveControl.java:25)
- [ ] `CorvidMoveControl.coast(0)`: its class isn't translated (it extends Minecraft's `FlyingMoveControl`) (com/lion/graveyard/entities/ai/control/CorvidMoveControl.java:68)

### `AbstractGraveyardMeleeGoal`: it's generic

- [ ] `AbstractGraveyardMeleeGoal.AbstractGraveyardMeleeGoal(4)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/ai/goals/AbstractGraveyardMeleeGoal.java:31)
- [ ] `AbstractGraveyardMeleeGoal.canUse(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/ai/goals/AbstractGraveyardMeleeGoal.java:39)
- [ ] `AbstractGraveyardMeleeGoal.isInReach(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/ai/goals/AbstractGraveyardMeleeGoal.java:60)
- [ ] `AbstractGraveyardMeleeGoal.isNewOrFarMoved(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/ai/goals/AbstractGraveyardMeleeGoal.java:64)
- [ ] `AbstractGraveyardMeleeGoal.rememberChecked(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/ai/goals/AbstractGraveyardMeleeGoal.java:75)
- [ ] `AbstractGraveyardMeleeGoal.canContinueToUse(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/ai/goals/AbstractGraveyardMeleeGoal.java:82)
- [ ] `AbstractGraveyardMeleeGoal.start(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/ai/goals/AbstractGraveyardMeleeGoal.java:97)
- [ ] `AbstractGraveyardMeleeGoal.stop(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/ai/goals/AbstractGraveyardMeleeGoal.java:104)
- [ ] `AbstractGraveyardMeleeGoal.requiresUpdateEveryTick(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/ai/goals/AbstractGraveyardMeleeGoal.java:114)
- [ ] `AbstractGraveyardMeleeGoal.tick(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/ai/goals/AbstractGraveyardMeleeGoal.java:118)
- [ ] `AbstractGraveyardMeleeGoal.onTargetTick(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/ai/goals/AbstractGraveyardMeleeGoal.java:155)
- [ ] `AbstractGraveyardMeleeGoal.attack(2)`: it has no body (com/lion/graveyard/entities/ai/goals/AbstractGraveyardMeleeGoal.java:158)
- [ ] `AbstractGraveyardMeleeGoal.getSquaredMaxAttackDistance(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/ai/goals/AbstractGraveyardMeleeGoal.java:160)
- [ ] `AbstractGraveyardMeleeGoal.resetCooldown(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/ai/goals/AbstractGraveyardMeleeGoal.java:164)

### `AttackWithOwnerGoal`: it extends Minecraft's `TargetGoal`

- [ ] `AttackWithOwnerGoal.AttackWithOwnerGoal(1)`: its class isn't translated (it extends Minecraft's `TargetGoal`) (com/lion/graveyard/entities/ai/goals/AttackWithOwnerGoal.java:15)
- [ ] `AttackWithOwnerGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `TargetGoal`) (com/lion/graveyard/entities/ai/goals/AttackWithOwnerGoal.java:21)
- [ ] `AttackWithOwnerGoal.start(0)`: its class isn't translated (it extends Minecraft's `TargetGoal`) (com/lion/graveyard/entities/ai/goals/AttackWithOwnerGoal.java:38)

### `CircleAnchorGoal`: it extends Minecraft's `Goal`

- [x] `CircleAnchorGoal.SPEED` (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:14)
- [x] `CircleAnchorGoal.RING_SPEED` (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:15)
- [x] `CircleAnchorGoal.MIN_RADIUS` (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:16)
- [x] `CircleAnchorGoal.RADIUS_SPREAD` (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:17)
- [x] `CircleAnchorGoal.MIN_HEIGHT` (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:18)
- [x] `CircleAnchorGoal.HEIGHT_SPREAD` (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:19)
- [x] `CircleAnchorGoal.DURATION` (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:20)
- [x] `CircleAnchorGoal.START_CHANCE` (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:21)
- [ ] `CircleAnchorGoal.CircleAnchorGoal(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:32)
- [ ] `CircleAnchorGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:37)
- [ ] `CircleAnchorGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:55)
- [ ] `CircleAnchorGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:64)
- [ ] `CircleAnchorGoal.findMaxTop(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:82)
- [ ] `CircleAnchorGoal.requiresUpdateEveryTick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:96)
- [ ] `CircleAnchorGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:100)
- [ ] `CircleAnchorGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/CircleAnchorGoal.java:122)

### `FleeGraveyardMobsGoal`: it extends Minecraft's `AvoidEntityGoal`

- [ ] `FleeGraveyardMobsGoal.FleeGraveyardMobsGoal(3)`: its class isn't translated (it extends Minecraft's `AvoidEntityGoal`) (com/lion/graveyard/entities/ai/goals/FleeGraveyardMobsGoal.java:8)

### `FlockStartleGoal`: it extends Minecraft's `Goal`

- [x] `FlockStartleGoal.SENTINEL_RANGE` (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:18)
- [x] `FlockStartleGoal.MEMBER_RANGE` (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:19)
- [x] `FlockStartleGoal.SNEAK_FACTOR` (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:20)
- [x] `FlockStartleGoal.SCAN_INTERVAL` (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:21)
- [x] `FlockStartleGoal.SCARECROW_SCAN_INTERVAL` (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:22)
- [x] `FlockStartleGoal.SCARE_FLOCK_SIZE` (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:23)
- [x] `FlockStartleGoal.SCARECROW_WITNESS_RANGE` (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:24)
- [x] `FlockStartleGoal.ESCAPE_DISTANCE` (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:25)
- [x] `FlockStartleGoal.ESCAPE_HEIGHT` (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:26)
- [x] `FlockStartleGoal.ESCAPE_SPEED` (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:27)
- [ ] `FlockStartleGoal.FlockStartleGoal(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:32)
- [ ] `FlockStartleGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:37)
- [ ] `FlockStartleGoal.watch(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:54)
- [ ] `FlockStartleGoal.watchForScarecrows(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:72)
- [ ] `FlockStartleGoal.findEscape(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:81)
- [ ] `FlockStartleGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:93)
- [ ] `FlockStartleGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:97)
- [ ] `FlockStartleGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FlockStartleGoal.java:104)

### `FlyOntoPerchGoal`: it extends Minecraft's `WaterAvoidingRandomFlyingGoal`

- [x] `FlyOntoPerchGoal.SEARCH_RADIUS` (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:24)
- [x] `FlyOntoPerchGoal.MAX_HOME_SEARCH_DISTANCE` (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:25)
- [x] `FlyOntoPerchGoal.SEARCH_HEIGHT` (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:26)
- [x] `FlyOntoPerchGoal.SEARCH_DEPTH` (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:27)
- [x] `FlyOntoPerchGoal.STAY_PERCHED_CHANCE` (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:28)
- [x] `FlyOntoPerchGoal.SOAR_CHANCE` (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:29)
- [ ] `FlyOntoPerchGoal.FlyOntoPerchGoal(2)`: its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`) (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:32)
- [ ] `FlyOntoPerchGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`) (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:37)
- [ ] `FlyOntoPerchGoal.start(0)`: its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`) (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:45)
- [ ] `FlyOntoPerchGoal.getSearchCenter(0)`: its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`) (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:54)
- [ ] `FlyOntoPerchGoal.isUsableHome(1)`: its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`) (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:59)
- [ ] `FlyOntoPerchGoal.getPosition(0)`: its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`) (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:72)
- [ ] `FlyOntoPerchGoal.getWanderFallbackTarget(0)`: its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`) (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:92)
- [ ] `FlyOntoPerchGoal.getSoarTarget(0)`: its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`) (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:114)
- [ ] `FlyOntoPerchGoal.getPerchTarget(0)`: its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`) (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:120)
- [ ] `FlyOntoPerchGoal.isField(1)`: it uses Minecraft's `BlockState` (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:165)
- [ ] `FlyOntoPerchGoal.scorePerch(2)`: its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`) (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:169)
- [ ] `FlyOntoPerchGoal.canPerchOn(2)`: its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`) (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:179)
- [ ] `FlyOntoPerchGoal.isBonePerch(2)`: its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`) (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:186)
- [ ] `FlyOntoPerchGoal.isRepelled(2)`: its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`) (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:196)
- [ ] `FlyOntoPerchGoal.isClear(2)`: its class isn't translated (it extends Minecraft's `WaterAvoidingRandomFlyingGoal`) (com/lion/graveyard/entities/ai/goals/FlyOntoPerchGoal.java:200)

### `FollowOwnerGoal`: it extends Minecraft's `Goal`

- [x] `FollowOwnerGoal.TELEPORT_DISTANCE` (com/lion/graveyard/entities/ai/goals/FollowOwnerGoal.java:19)
- [x] `FollowOwnerGoal.HORIZONTAL_RANGE` (com/lion/graveyard/entities/ai/goals/FollowOwnerGoal.java:20)
- [x] `FollowOwnerGoal.HORIZONTAL_VARIATION` (com/lion/graveyard/entities/ai/goals/FollowOwnerGoal.java:21)
- [x] `FollowOwnerGoal.VERTICAL_VARIATION` (com/lion/graveyard/entities/ai/goals/FollowOwnerGoal.java:22)
- [ ] `FollowOwnerGoal.FollowOwnerGoal(5)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FollowOwnerGoal.java:34)
- [ ] `FollowOwnerGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FollowOwnerGoal.java:48)
- [ ] `FollowOwnerGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FollowOwnerGoal.java:70)
- [ ] `FollowOwnerGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FollowOwnerGoal.java:78)
- [ ] `FollowOwnerGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FollowOwnerGoal.java:84)
- [ ] `FollowOwnerGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FollowOwnerGoal.java:90)
- [ ] `FollowOwnerGoal.tryTeleport(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FollowOwnerGoal.java:104)
- [ ] `FollowOwnerGoal.tryTeleportTo(3)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FollowOwnerGoal.java:118)
- [ ] `FollowOwnerGoal.canTeleportTo(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FollowOwnerGoal.java:132)
- [ ] `FollowOwnerGoal.getRandomInt(2)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/FollowOwnerGoal.java:147)

### `GhoulingCollectItemsGoal`: it extends Minecraft's `Goal`

- [x] `GhoulingCollectItemsGoal.COLLECT_RANGE` (com/lion/graveyard/entities/ai/goals/GhoulingCollectItemsGoal.java:11)
- [x] `GhoulingCollectItemsGoal.MAX_TICKS` (com/lion/graveyard/entities/ai/goals/GhoulingCollectItemsGoal.java:12)
- [ ] `GhoulingCollectItemsGoal.GhoulingCollectItemsGoal(2)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/GhoulingCollectItemsGoal.java:18)
- [ ] `GhoulingCollectItemsGoal.collect(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/GhoulingCollectItemsGoal.java:24)
- [ ] `GhoulingCollectItemsGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/GhoulingCollectItemsGoal.java:29)
- [ ] `GhoulingCollectItemsGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/GhoulingCollectItemsGoal.java:33)
- [ ] `GhoulingCollectItemsGoal.isValid(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/GhoulingCollectItemsGoal.java:37)
- [ ] `GhoulingCollectItemsGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/GhoulingCollectItemsGoal.java:47)
- [ ] `GhoulingCollectItemsGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/GhoulingCollectItemsGoal.java:52)

### `GhoulingMeleeAttackGoal`: it extends `AbstractGraveyardMeleeGoal`

- [ ] `GhoulingMeleeAttackGoal.GhoulingMeleeAttackGoal(3)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/GhoulingMeleeAttackGoal.java:10)
- [ ] `GhoulingMeleeAttackGoal.attack(2)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/GhoulingMeleeAttackGoal.java:14)

### `GhoulMeleeAttackGoal`: it extends `AbstractGraveyardMeleeGoal`

- [ ] `GhoulMeleeAttackGoal.GhoulMeleeAttackGoal(3)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/GhoulMeleeAttackGoal.java:9)
- [ ] `GhoulMeleeAttackGoal.attack(2)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/GhoulMeleeAttackGoal.java:13)

### `HopAwayGoal`: it extends `HopGoal`

- [x] `HopAwayGoal.PLAYER_RANGE` (com/lion/graveyard/entities/ai/goals/HopAwayGoal.java:10)
- [x] `HopAwayGoal.COOLDOWN` (com/lion/graveyard/entities/ai/goals/HopAwayGoal.java:11)
- [x] `HopAwayGoal.FIND_ATTEMPTS` (com/lion/graveyard/entities/ai/goals/HopAwayGoal.java:12)
- [x] `HopAwayGoal.MIN_DISTANCE` (com/lion/graveyard/entities/ai/goals/HopAwayGoal.java:13)
- [x] `HopAwayGoal.DISTANCE_SPREAD` (com/lion/graveyard/entities/ai/goals/HopAwayGoal.java:14)
- [x] `HopAwayGoal.ANGLE_SPREAD_DEGREES` (com/lion/graveyard/entities/ai/goals/HopAwayGoal.java:15)
- [ ] `HopAwayGoal.HopAwayGoal(1)`: its class isn't translated (it extends `HopGoal`) (com/lion/graveyard/entities/ai/goals/HopAwayGoal.java:20)
- [ ] `HopAwayGoal.canUse(0)`: its class isn't translated (it extends `HopGoal`) (com/lion/graveyard/entities/ai/goals/HopAwayGoal.java:24)
- [ ] `HopAwayGoal.findTarget(0)`: its class isn't translated (it extends `HopGoal`) (com/lion/graveyard/entities/ai/goals/HopAwayGoal.java:43)
- [ ] `HopAwayGoal.rollHopCount(0)`: its class isn't translated (it extends `HopGoal`) (com/lion/graveyard/entities/ai/goals/HopAwayGoal.java:64)
- [ ] `HopAwayGoal.stop(0)`: its class isn't translated (it extends `HopGoal`) (com/lion/graveyard/entities/ai/goals/HopAwayGoal.java:69)

### `HopGoal`: it extends Minecraft's `Goal`

- [x] `HopGoal.HOP_INTERVAL` (com/lion/graveyard/entities/ai/goals/HopGoal.java:16)
- [x] `HopGoal.HOP_UP` (com/lion/graveyard/entities/ai/goals/HopGoal.java:17)
- [x] `HopGoal.HOP_FORWARD` (com/lion/graveyard/entities/ai/goals/HopGoal.java:18)
- [x] `HopGoal.TIMEOUT` (com/lion/graveyard/entities/ai/goals/HopGoal.java:19)
- [x] `HopGoal.FIND_ATTEMPTS` (com/lion/graveyard/entities/ai/goals/HopGoal.java:20)
- [x] `HopGoal.MIN_DISTANCE` (com/lion/graveyard/entities/ai/goals/HopGoal.java:21)
- [x] `HopGoal.DISTANCE_SPREAD` (com/lion/graveyard/entities/ai/goals/HopGoal.java:22)
- [ ] `HopGoal.HopGoal(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/HopGoal.java:30)
- [ ] `HopGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/HopGoal.java:35)
- [ ] `HopGoal.findTarget(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/HopGoal.java:44)
- [ ] `HopGoal.isHopPathClear(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/HopGoal.java:58)
- [ ] `HopGoal.isColumnClear(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/HopGoal.java:68)
- [ ] `HopGoal.isEmptyShape(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/HopGoal.java:72)
- [ ] `HopGoal.isLowHalf(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/HopGoal.java:77)
- [ ] `HopGoal.getHopInterval(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/HopGoal.java:82)
- [ ] `HopGoal.rollHopCount(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/HopGoal.java:86)
- [ ] `HopGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/HopGoal.java:90)
- [ ] `HopGoal.requiresUpdateEveryTick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/HopGoal.java:98)
- [ ] `HopGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/HopGoal.java:102)
- [ ] `HopGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/HopGoal.java:130)
- [ ] `HopGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/HopGoal.java:134)

### `JoinFlockGoal`: it extends Minecraft's `Goal`

- [x] `JoinFlockGoal.SEARCH_INTERVAL` (com/lion/graveyard/entities/ai/goals/JoinFlockGoal.java:9)
- [x] `JoinFlockGoal.MOVE_INTERVAL` (com/lion/graveyard/entities/ai/goals/JoinFlockGoal.java:10)
- [ ] `JoinFlockGoal.JoinFlockGoal(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/JoinFlockGoal.java:15)
- [ ] `JoinFlockGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/JoinFlockGoal.java:21)
- [ ] `JoinFlockGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/JoinFlockGoal.java:39)
- [ ] `JoinFlockGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/JoinFlockGoal.java:43)
- [ ] `JoinFlockGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/JoinFlockGoal.java:47)

### `LichMeleeGoal`: it extends `AbstractGraveyardMeleeGoal`

- [x] `LichMeleeGoal.DAMAGE_START_IN_ANIM` (com/lion/graveyard/entities/ai/goals/LichMeleeGoal.java:13)
- [x] `LichMeleeGoal.HIT_REACH_SQUARED` (com/lion/graveyard/entities/ai/goals/LichMeleeGoal.java:14)
- [ ] `LichMeleeGoal.LichMeleeGoal(3)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/LichMeleeGoal.java:18)
- [ ] `LichMeleeGoal.canContinueToUse(0)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/LichMeleeGoal.java:23)
- [ ] `LichMeleeGoal.tick(0)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/LichMeleeGoal.java:33)
- [ ] `LichMeleeGoal.isPhaseOneHold(1)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/LichMeleeGoal.java:62)
- [ ] `LichMeleeGoal.isStraightBelowOrAbove(1)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/LichMeleeGoal.java:66)
- [ ] `LichMeleeGoal.isDirectChase(1)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/LichMeleeGoal.java:72)
- [ ] `LichMeleeGoal.onTargetTick(0)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/LichMeleeGoal.java:76)
- [ ] `LichMeleeGoal.attack(2)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/LichMeleeGoal.java:81)
- [ ] `LichMeleeGoal.getSquaredMaxAttackDistance(1)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/LichMeleeGoal.java:139)

### `NightmareMeleeAttackGoal`: it extends `AbstractGraveyardMeleeGoal`

- [ ] `NightmareMeleeAttackGoal.NightmareMeleeAttackGoal(3)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/NightmareMeleeAttackGoal.java:9)
- [ ] `NightmareMeleeAttackGoal.attack(2)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/NightmareMeleeAttackGoal.java:13)

### `PickTreasureGoal`: it extends Minecraft's `Goal`

- [x] `PickTreasureGoal.RESCAN_INTERVAL` (com/lion/graveyard/entities/ai/goals/PickTreasureGoal.java:16)
- [x] `PickTreasureGoal.SPEED` (com/lion/graveyard/entities/ai/goals/PickTreasureGoal.java:17)
- [x] `PickTreasureGoal.GIVE_UP_DISTANCE` (com/lion/graveyard/entities/ai/goals/PickTreasureGoal.java:18)
- [x] `PickTreasureGoal.MAX_DURATION` (com/lion/graveyard/entities/ai/goals/PickTreasureGoal.java:19)
- [ ] `PickTreasureGoal.PickTreasureGoal(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/PickTreasureGoal.java:25)
- [ ] `PickTreasureGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/PickTreasureGoal.java:30)
- [ ] `PickTreasureGoal.findTreasure(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/PickTreasureGoal.java:45)
- [ ] `PickTreasureGoal.requiresUpdateEveryTick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/PickTreasureGoal.java:66)
- [ ] `PickTreasureGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/PickTreasureGoal.java:70)
- [ ] `PickTreasureGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/PickTreasureGoal.java:76)
- [ ] `PickTreasureGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/PickTreasureGoal.java:83)
- [ ] `PickTreasureGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/PickTreasureGoal.java:91)

### `RevenantMeleeAttackGoal`: it extends `AbstractGraveyardMeleeGoal`

- [ ] `RevenantMeleeAttackGoal.RevenantMeleeAttackGoal(3)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/RevenantMeleeAttackGoal.java:9)
- [ ] `RevenantMeleeAttackGoal.attack(2)`: its class isn't translated (it extends `AbstractGraveyardMeleeGoal`) (com/lion/graveyard/entities/ai/goals/RevenantMeleeAttackGoal.java:13)

### `SettleGoal`: it extends `FlyOntoPerchGoal`

- [x] `SettleGoal.DEATH_LANDING_RANGE_SQ` (com/lion/graveyard/entities/ai/goals/SettleGoal.java:13)
- [x] `SettleGoal.DEATH_ANCHOR_RANGE_SQ` (com/lion/graveyard/entities/ai/goals/SettleGoal.java:14)
- [ ] `SettleGoal.SettleGoal(2)`: its class isn't translated (it extends `FlyOntoPerchGoal`) (com/lion/graveyard/entities/ai/goals/SettleGoal.java:17)
- [ ] `SettleGoal.canUse(0)`: its class isn't translated (it extends `FlyOntoPerchGoal`) (com/lion/graveyard/entities/ai/goals/SettleGoal.java:22)
- [ ] `SettleGoal.getSearchCenter(0)`: its class isn't translated (it extends `FlyOntoPerchGoal`) (com/lion/graveyard/entities/ai/goals/SettleGoal.java:27)
- [ ] `SettleGoal.canPerchOn(2)`: its class isn't translated (it extends `FlyOntoPerchGoal`) (com/lion/graveyard/entities/ai/goals/SettleGoal.java:38)
- [ ] `SettleGoal.isRepelled(2)`: its class isn't translated (it extends `FlyOntoPerchGoal`) (com/lion/graveyard/entities/ai/goals/SettleGoal.java:43)
- [ ] `SettleGoal.scorePerch(2)`: its class isn't translated (it extends `FlyOntoPerchGoal`) (com/lion/graveyard/entities/ai/goals/SettleGoal.java:48)
- [ ] `SettleGoal.isNearDeathSite(2)`: its class isn't translated (it extends `FlyOntoPerchGoal`) (com/lion/graveyard/entities/ai/goals/SettleGoal.java:53)

### `SitGoal`: it extends Minecraft's `Goal`

- [ ] `SitGoal.SitGoal(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/SitGoal.java:12)
- [ ] `SitGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/SitGoal.java:17)
- [ ] `SitGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/SitGoal.java:21)
- [ ] `SitGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/SitGoal.java:36)
- [ ] `SitGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/SitGoal.java:41)

### `StartleGoal`: it extends Minecraft's `Goal`

- [ ] `StartleGoal.StartleGoal(2)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/StartleGoal.java:19)
- [ ] `StartleGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/StartleGoal.java:25)
- [ ] `StartleGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/StartleGoal.java:44)
- [ ] `StartleGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/StartleGoal.java:48)
- [ ] `StartleGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ai/goals/StartleGoal.java:53)

### `TrackOwnerAttackerGoal`: it extends Minecraft's `TargetGoal`

- [ ] `TrackOwnerAttackerGoal.TrackOwnerAttackerGoal(1)`: its class isn't translated (it extends Minecraft's `TargetGoal`) (com/lion/graveyard/entities/ai/goals/TrackOwnerAttackerGoal.java:15)
- [ ] `TrackOwnerAttackerGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `TargetGoal`) (com/lion/graveyard/entities/ai/goals/TrackOwnerAttackerGoal.java:21)
- [ ] `TrackOwnerAttackerGoal.start(0)`: its class isn't translated (it extends Minecraft's `TargetGoal`) (com/lion/graveyard/entities/ai/goals/TrackOwnerAttackerGoal.java:38)

### `WalkGoal`: it extends `HopGoal`

- [x] `WalkGoal.WALK_SPEED` (com/lion/graveyard/entities/ai/goals/WalkGoal.java:8)
- [x] `WalkGoal.ARRIVE_DISTANCE_SQ` (com/lion/graveyard/entities/ai/goals/WalkGoal.java:9)
- [x] `WalkGoal.TIMEOUT` (com/lion/graveyard/entities/ai/goals/WalkGoal.java:10)
- [ ] `WalkGoal.WalkGoal(1)`: its class isn't translated (it extends `HopGoal`) (com/lion/graveyard/entities/ai/goals/WalkGoal.java:14)
- [ ] `WalkGoal.canUse(0)`: its class isn't translated (it extends `HopGoal`) (com/lion/graveyard/entities/ai/goals/WalkGoal.java:18)
- [ ] `WalkGoal.start(0)`: its class isn't translated (it extends `HopGoal`) (com/lion/graveyard/entities/ai/goals/WalkGoal.java:28)
- [ ] `WalkGoal.tick(0)`: its class isn't translated (it extends `HopGoal`) (com/lion/graveyard/entities/ai/goals/WalkGoal.java:36)
- [ ] `WalkGoal.canContinueToUse(0)`: its class isn't translated (it extends `HopGoal`) (com/lion/graveyard/entities/ai/goals/WalkGoal.java:62)
- [ ] `WalkGoal.stop(0)`: its class isn't translated (it extends `HopGoal`) (com/lion/graveyard/entities/ai/goals/WalkGoal.java:67)

### `AngerableGraveyardEntity`: it extends `HordeGraveyardEntity`

- [x] `AngerableGraveyardEntity.ATTACKING_SPEED_BOOST_ID` (com/lion/graveyard/entities/AngerableGraveyardEntity.java:27)
- [ ] `AngerableGraveyardEntity.ATTACKING_SPEED_BOOST`: it uses Minecraft's `AttributeModifier` (com/lion/graveyard/entities/AngerableGraveyardEntity.java:28)
- [ ] `AngerableGraveyardEntity.ANGRY`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/AngerableGraveyardEntity.java:29)
- [ ] `AngerableGraveyardEntity.PROVOKED`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/AngerableGraveyardEntity.java:30)
- [ ] `AngerableGraveyardEntity.ANGER_TIME`: it uses Minecraft's `UniformInt` (com/lion/graveyard/entities/AngerableGraveyardEntity.java:31)
- [ ] `AngerableGraveyardEntity.AngerableGraveyardEntity(3)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/AngerableGraveyardEntity.java:36)
- [ ] `AngerableGraveyardEntity.aiStep(0)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/AngerableGraveyardEntity.java:40)
- [ ] `AngerableGraveyardEntity.defineSynchedData(1)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/AngerableGraveyardEntity.java:49)
- [ ] `AngerableGraveyardEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/AngerableGraveyardEntity.java:56)
- [ ] `AngerableGraveyardEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/AngerableGraveyardEntity.java:62)
- [ ] `AngerableGraveyardEntity.readPersistentAngerSaveData(2)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/AngerableGraveyardEntity.java:68)
- [ ] `AngerableGraveyardEntity.isAngry(0)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/AngerableGraveyardEntity.java:73)
- [ ] `AngerableGraveyardEntity.setPersistentAngerEndTime(1)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/AngerableGraveyardEntity.java:77)
- [ ] `AngerableGraveyardEntity.getPersistentAngerEndTime(0)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/AngerableGraveyardEntity.java:81)
- [ ] `AngerableGraveyardEntity.getPersistentAngerTarget(0)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/AngerableGraveyardEntity.java:85)
- [ ] `AngerableGraveyardEntity.setPersistentAngerTarget(1)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/AngerableGraveyardEntity.java:90)
- [ ] `AngerableGraveyardEntity.startPersistentAngerTimer(0)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/AngerableGraveyardEntity.java:94)
- [ ] `AngerableGraveyardEntity.setTarget(1)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/AngerableGraveyardEntity.java:98)

### `AnimatedCorruptedIllager`: it extends `CorruptedIllager`

- [x] `AnimatedCorruptedIllager.ANIMATION_PREFIX` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:34)
- [x] `AnimatedCorruptedIllager.SPAWN_TICKS` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:35)
- [x] `AnimatedCorruptedIllager.DEATH_TICKS` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:36)
- [x] `AnimatedCorruptedIllager.GLITCH_TICKS` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:37)
- [x] `AnimatedCorruptedIllager.STRIKE_TICKS` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:38)
- [x] `AnimatedCorruptedIllager.BITE_TICKS` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:39)
- [x] `AnimatedCorruptedIllager.STRIKE_HIT_TICKS` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:40)
- [x] `AnimatedCorruptedIllager.BITE_HIT_TICKS` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:41)
- [x] `AnimatedCorruptedIllager.GLITCH_CHANCE` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:42)
- [x] `AnimatedCorruptedIllager.GLITCH_COOLDOWN_TICKS` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:43)
- [x] `AnimatedCorruptedIllager.SOUND_RESTART_SLACK` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:44)
- [x] `AnimatedCorruptedIllager.ATTACK_STRIKE` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:45)
- [x] `AnimatedCorruptedIllager.ATTACK_BITE` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:46)
- [x] `AnimatedCorruptedIllager.DECAY_LEVELS` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:47)
- [x] `AnimatedCorruptedIllager.LEFT_CLAW` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:48)
- [x] `AnimatedCorruptedIllager.RIGHT_CLAW` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:49)
- [x] `AnimatedCorruptedIllager.WANDER_SPEED` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:50)
- [x] `AnimatedCorruptedIllager.MOVING_LIMB_SPEED` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:51)
- [x] `AnimatedCorruptedIllager.WANDER_STUCK_TICKS` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:52)
- [ ] `AnimatedCorruptedIllager.DECAY`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:53)
- [ ] `AnimatedCorruptedIllager.SPAWN_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:54)
- [ ] `AnimatedCorruptedIllager.ATTACK_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:55)
- [ ] `AnimatedCorruptedIllager.ATTACK_KIND`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:56)
- [ ] `AnimatedCorruptedIllager.GLITCH_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:57)
- [ ] `AnimatedCorruptedIllager.CLAWS`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:58)
- [ ] `AnimatedCorruptedIllager.WANDERING`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:59)
- [ ] `AnimatedCorruptedIllager.SPAWN`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:60)
- [ ] `AnimatedCorruptedIllager.DEATH`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:61)
- [ ] `AnimatedCorruptedIllager.STRIKE`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:62)
- [ ] `AnimatedCorruptedIllager.BITE`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:63)
- [ ] `AnimatedCorruptedIllager.GLITCH`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:64)
- [ ] `AnimatedCorruptedIllager.ATTACK_RUN`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:65)
- [ ] `AnimatedCorruptedIllager.WANDER`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:66)
- [ ] `AnimatedCorruptedIllager.IDLE`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:67)
- [ ] `AnimatedCorruptedIllager.AnimatedCorruptedIllager(3)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:78)
- [ ] `AnimatedCorruptedIllager.hasSpawnAnimation(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:82)
- [ ] `AnimatedCorruptedIllager.defineSynchedData(1)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:86)
- [ ] `AnimatedCorruptedIllager.getDecay(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:98)
- [ ] `AnimatedCorruptedIllager.getWanderSpeed(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:103)
- [ ] `AnimatedCorruptedIllager.hasLeftClaw(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:108)
- [ ] `AnimatedCorruptedIllager.hasRightClaw(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:112)
- [ ] `AnimatedCorruptedIllager.getClaws(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:116)
- [ ] `AnimatedCorruptedIllager.getSpawnTimer(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:121)
- [ ] `AnimatedCorruptedIllager.finalizeSpawn(4)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:125)
- [ ] `AnimatedCorruptedIllager.addAdditionalSaveData(1)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:141)
- [ ] `AnimatedCorruptedIllager.readAdditionalSaveData(1)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:148)
- [ ] `AnimatedCorruptedIllager.tick(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:160)
- [ ] `AnimatedCorruptedIllager.currentAnimationSound(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:179)
- [ ] `AnimatedCorruptedIllager.tickAnimationSound(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:198)
- [ ] `AnimatedCorruptedIllager.onAnimationKeyframeZero(2)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:215)
- [ ] `AnimatedCorruptedIllager.startSound(1)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:228)
- [ ] `AnimatedCorruptedIllager.getDeathSound(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:240)
- [ ] `AnimatedCorruptedIllager.resolvePendingHit(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:245)
- [ ] `AnimatedCorruptedIllager.isSwimmingInWater(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:264)
- [ ] `AnimatedCorruptedIllager.tickDown(1)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:268)
- [ ] `AnimatedCorruptedIllager.aiStep(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:275)
- [ ] `AnimatedCorruptedIllager.customServerAiStep(1)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:283)
- [ ] `AnimatedCorruptedIllager.doHurtTarget(2)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:303)
- [ ] `AnimatedCorruptedIllager.isAttackPending(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:318)
- [ ] `AnimatedCorruptedIllager.isImmobile(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:323)
- [ ] `AnimatedCorruptedIllager.tickDeath(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:327)
- [ ] `AnimatedCorruptedIllager.registerControllers(1)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:335)
- [ ] `AnimatedCorruptedIllager.getAnimatableInstanceCache(0)`: its class isn't translated (it extends `CorruptedIllager`) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:356)

### `AnimatedCorruptedIllager.AnimationSound`: its constants carry values

- [ ] `AnimatedCorruptedIllager.AnimationSound.AnimationSound(4)`: its class isn't translated (its constants carry values) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:375)
- [ ] `AnimatedCorruptedIllager.AnimationSound.of(1)`: it uses AnimationSound, which isn't translated (its constants carry values) (com/lion/graveyard/entities/AnimatedCorruptedIllager.java:382)

### `AnimationClock`

- [x] `AnimationClock.RENDER_GRACE_TICKS` (com/lion/graveyard/entities/animation/AnimationClock.java:25)
- [x] `AnimationClock.TICKS_PER_SECOND` (com/lion/graveyard/entities/animation/AnimationClock.java:26)
- [x] `AnimationClock.LOOP_EPSILON` (com/lion/graveyard/entities/animation/AnimationClock.java:27)
- [ ] `AnimationClock.LAST_FRAME`: it uses Minecraft's `Entity` (com/lion/graveyard/entities/animation/AnimationClock.java:28)
- [ ] `AnimationClock.listener`: it uses Listener, which isn't translated (it's an interface) (com/lion/graveyard/entities/animation/AnimationClock.java:30)
- [x] `AnimationClock.AnimationClock(0)` (com/lion/graveyard/entities/animation/AnimationClock.java:32)
- [ ] `AnimationClock.setListener(1)`: it uses Listener, which isn't translated (it's an interface) (com/lion/graveyard/entities/animation/AnimationClock.java:35)
- [ ] `AnimationClock.isRendered(1)`: it uses Minecraft's `Entity` (com/lion/graveyard/entities/animation/AnimationClock.java:39)
- [ ] `AnimationClock.playTracked(2)`: it uses Minecraft's `Entity` (com/lion/graveyard/entities/animation/AnimationClock.java:44)
- [ ] `AnimationClock.playDetached(2)`: it uses Minecraft's `Entity` (com/lion/graveyard/entities/animation/AnimationClock.java:50)
- [ ] `AnimationClock.restart(1)`: it uses `AnimationController` (com.geckolib.animation) (com/lion/graveyard/entities/animation/AnimationClock.java:56)
- [ ] `AnimationClock.controller(3)`: it's generic (com/lion/graveyard/entities/animation/AnimationClock.java:62)
- [ ] `AnimationClock.controller(5)`: it's generic (com/lion/graveyard/entities/animation/AnimationClock.java:66)

### `AnimationClock.ClockController`: it's generic

- [ ] `AnimationClock.ClockController.ClockController(5)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationClock.java:86)
- [ ] `AnimationClock.ClockController.guard(1)`: it's generic (com/lion/graveyard/entities/animation/AnimationClock.java:95)
- [ ] `AnimationClock.ClockController.restart(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationClock.java:117)
- [ ] `AnimationClock.ClockController.resumeAfterStop(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationClock.java:121)
- [ ] `AnimationClock.ClockController.isRunningTriggeredClip(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationClock.java:130)
- [ ] `AnimationClock.ClockController.checkControllerState(4)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationClock.java:136)
- [ ] `AnimationClock.ClockController.progressExistingAnimation(4)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationClock.java:172)
- [ ] `AnimationClock.ClockController.extractControllerState(5)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationClock.java:183)
- [ ] `AnimationClock.ClockController.appendResetStage(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationClock.java:223)
- [ ] `AnimationClock.ClockController.currentAnimationStage(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationClock.java:236)
- [ ] `AnimationClock.ClockController.applyStartOffset(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationClock.java:246)

### `AnimationClock.KeyframeZeroSink`: it's an interface

- [ ] `AnimationClock.KeyframeZeroSink.keyframeZero(2)`: it has no body (com/lion/graveyard/entities/animation/AnimationClock.java:265)

### `AnimationClock.Listener`: it's an interface

- [ ] `AnimationClock.Listener.keyframeZero(3)`: it has no body (com/lion/graveyard/entities/animation/AnimationClock.java:269)
- [ ] `AnimationClock.Listener.playTracked(2)`: it has no body (com/lion/graveyard/entities/animation/AnimationClock.java:271)
- [ ] `AnimationClock.Listener.playDetached(2)`: it has no body (com/lion/graveyard/entities/animation/AnimationClock.java:273)

### `AnimationPreview`

- [x] `AnimationPreview.ALL` (com/lion/graveyard/entities/animation/AnimationPreview.java:20)
- [x] `AnimationPreview.PREFIX` (com/lion/graveyard/entities/animation/AnimationPreview.java:21)
- [x] `AnimationPreview.UNKNOWN_STEP_TICKS` (com/lion/graveyard/entities/animation/AnimationPreview.java:22)
- [ ] `AnimationPreview.LOOPS`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/animation/AnimationPreview.java:23)
- [ ] `AnimationPreview.PLAYS`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/animation/AnimationPreview.java:24)
- [ ] `AnimationPreview.STEPS`: it uses `AnimationController` (com.geckolib.animation) (com/lion/graveyard/entities/animation/AnimationPreview.java:25)
- [x] `AnimationPreview.AnimationPreview(0)` (com/lion/graveyard/entities/animation/AnimationPreview.java:27)
- [ ] `AnimationPreview.forcedAnimation(1)`: it uses Minecraft's `Entity` (com/lion/graveyard/entities/animation/AnimationPreview.java:30)
- [ ] `AnimationPreview.frame(2)`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/animation/AnimationPreview.java:45)
- [ ] `AnimationPreview.nextIndex(3)`: it uses `AnimationController` (com.geckolib.animation) (com/lion/graveyard/entities/animation/AnimationPreview.java:72)
- [ ] `AnimationPreview.isStalled(3)`: it uses `AnimationController` (com.geckolib.animation) (com/lion/graveyard/entities/animation/AnimationPreview.java:89)

### `AnimationPreview.Step`


### `AnimationPreviewClient`

- [x] `AnimationPreviewClient.FILE` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:41)
- [ ] `AnimationPreviewClient.PLAYBACKS`: it uses Minecraft's `Entity` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:42)
- [ ] `AnimationPreviewClient.TRACKED`: it uses Minecraft's `Entity` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:43)
- [ ] `AnimationPreviewClient.DIGS`: it uses Minecraft's `Entity` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:44)
- [x] `AnimationPreviewClient.DIG_SPECS` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:45)
- [ ] `AnimationPreviewClient.LISTENER`: its value couldn't be read: an anonymous class (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:57)
- [ ] `AnimationPreviewClient.cues`: it uses Cue, which isn't translated (it uses Minecraft's `SoundEvent`) (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:83)
- [ ] `AnimationPreviewClient.lastWorld`: it uses Minecraft's `ClientLevel` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:85)
- [x] `AnimationPreviewClient.AnimationPreviewClient(0)` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:87)
- [ ] `AnimationPreviewClient.clientTick(1)`: it calls AnimationClock.setListener, which isn't translated (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:90)
- [ ] `AnimationPreviewClient.startDig(3)`: it uses Minecraft's `Entity` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:124)
- [ ] `AnimationPreviewClient.startPreview(2)`: it uses Minecraft's `Entity` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:134)
- [ ] `AnimationPreviewClient.play(4)`: it uses Minecraft's `SoundInstance` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:154)
- [ ] `AnimationPreviewClient.stop(1)`: it uses Minecraft's `SoundInstance` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:160)
- [ ] `AnimationPreviewClient.load(0)`: it uses Cue, which isn't translated (it uses Minecraft's `SoundEvent`) (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:166)

### `AnimationPreviewClient.Cue`: it uses Minecraft's `SoundEvent`

- [ ] `AnimationPreviewClient.Cue.Cue(5)`: its class isn't translated (it uses Minecraft's `SoundEvent`) (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:202)
- [ ] `AnimationPreviewClient.Cue.parse(1)`: it uses Cue, which isn't translated (it uses Minecraft's `SoundEvent`) (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:203)
- [ ] `AnimationPreviewClient.Cue.event(0)`: its class isn't translated (it uses Minecraft's `SoundEvent`) (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:202)
- [ ] `AnimationPreviewClient.Cue.offsetTicks(0)`: its class isn't translated (it uses Minecraft's `SoundEvent`) (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:202)
- [ ] `AnimationPreviewClient.Cue.repeatTicks(0)`: its class isn't translated (it uses Minecraft's `SoundEvent`) (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:202)
- [ ] `AnimationPreviewClient.Cue.volume(0)`: its class isn't translated (it uses Minecraft's `SoundEvent`) (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:202)
- [ ] `AnimationPreviewClient.Cue.pitch(0)`: its class isn't translated (it uses Minecraft's `SoundEvent`) (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:202)

### `AnimationPreviewClient.Dig`

- [x] `AnimationPreviewClient.Dig.MIN_GAP` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:219)
- [x] `AnimationPreviewClient.Dig.GAP_SPREAD` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:220)
- [x] `AnimationPreviewClient.Dig.Dig(3)` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:229)
- [ ] `AnimationPreviewClient.Dig.tick(2)`: it uses Minecraft's `Entity` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:236)
- [ ] `AnimationPreviewClient.Dig.groundBelow(1)`: it uses Minecraft's `BlockState` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:287)
- [ ] `AnimationPreviewClient.Dig.playEarth(3)`: it uses Minecraft's `Entity` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:291)
- [ ] `AnimationPreviewClient.Dig.playDig(3)`: it uses Minecraft's `Entity` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:297)

### `AnimationPreviewClient.DigSpec`

- [x] `AnimationPreviewClient.DigSpec.DigSpec(3)` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:302)
- [x] `AnimationPreviewClient.DigSpec.endTick(0)` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:302)
- [x] `AnimationPreviewClient.DigSpec.strokeTicks(0)` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:302)
- [x] `AnimationPreviewClient.DigSpec.particles(0)` (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:302)

### `AnimationPreviewClient.Playback`: it uses Cue, which isn't translated (it uses Minecraft's `SoundEvent`)

- [ ] `AnimationPreviewClient.Playback.Playback(2)`: its class isn't translated (it uses Cue, which isn't translated (it uses Minecraft's `SoundEvent`)) (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:310)
- [ ] `AnimationPreviewClient.Playback.tick(2)`: its class isn't translated (it uses Cue, which isn't translated (it uses Minecraft's `SoundEvent`)) (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:319)
- [ ] `AnimationPreviewClient.Playback.stopSounds(0)`: its class isn't translated (it uses Cue, which isn't translated (it uses Minecraft's `SoundEvent`)) (com/lion/graveyard/entities/animation/AnimationPreviewClient.java:329)

### `AnimationTable`: it's generic

- [x] `AnimationTable.TRANSITION_TICKS` (com/lion/graveyard/entities/animation/AnimationTable.java:21)
- [x] `AnimationTable.MOVEMENT_THRESHOLD` (com/lion/graveyard/entities/animation/AnimationTable.java:22)
- [ ] `AnimationTable.AnimationTable(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationTable.java:35)
- [ ] `AnimationTable.create(0)`: it's generic (com/lion/graveyard/entities/animation/AnimationTable.java:38)
- [ ] `AnimationTable.when(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationTable.java:42)
- [ ] `AnimationTable.when(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationTable.java:47)
- [ ] `AnimationTable.stopWhen(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationTable.java:51)
- [ ] `AnimationTable.otherwise(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationTable.java:56)
- [ ] `AnimationTable.startAt(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationTable.java:61)
- [ ] `AnimationTable.speed(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationTable.java:67)
- [ ] `AnimationTable.transitionTicks(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationTable.java:72)
- [ ] `AnimationTable.build(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationTable.java:77)
- [ ] `AnimationTable.build(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationTable.java:81)
- [ ] `AnimationTable.animationNames(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationTable.java:116)
- [ ] `AnimationTable.addName(2)`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/animation/AnimationTable.java:127)
- [ ] `AnimationTable.isMoving(1)`: it uses `AnimationTest` (com.geckolib.animation.state) (com/lion/graveyard/entities/animation/AnimationTable.java:133)

### `AnimationTable.Entry`: it's generic

- [ ] `AnimationTable.Entry.Entry(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationTable.java:137)
- [ ] `AnimationTable.Entry.condition(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationTable.java:137)
- [ ] `AnimationTable.Entry.animation(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/animation/AnimationTable.java:137)

### `BoneReturn`

- [x] `BoneReturn.TICKS` (com/lion/graveyard/entities/animation/BoneReturn.java:19)
- [ ] `BoneReturn.RETURNS`: it uses `DataTicket` (com.geckolib.constant.dataticket) (com/lion/graveyard/entities/animation/BoneReturn.java:20)
- [x] `BoneReturn.BoneReturn(0)` (com/lion/graveyard/entities/animation/BoneReturn.java:22)
- [ ] `BoneReturn.add(2)`: it uses `GeoRenderState` (com.geckolib.renderer.base) (com/lion/graveyard/entities/animation/BoneReturn.java:25)
- [ ] `BoneReturn.apply(2)`: it uses `RenderPassInfo` (com.geckolib.renderer.base) (com/lion/graveyard/entities/animation/BoneReturn.java:32)
- [ ] `BoneReturn.easeToBind(3)`: it uses `BoneSnapshot` (com.geckolib.animation.state) (com/lion/graveyard/entities/animation/BoneReturn.java:58)
- [ ] `BoneReturn.value(5)`: it uses `BoneSnapshot` (com.geckolib.animation.state) (com/lion/graveyard/entities/animation/BoneReturn.java:76)

### `CorruptedEvoker`: it extends `CorruptedVindicator`

- [ ] `CorruptedEvoker.CorruptedEvoker(2)`: its class isn't translated (it extends `CorruptedVindicator`) (com/lion/graveyard/entities/CorruptedEvoker.java:7)

### `CorruptedIllager`: it extends `HordeGraveyardEntity`

- [x] `CorruptedIllager.CHASE_END_TICKS` (com/lion/graveyard/entities/CorruptedIllager.java:39)
- [ ] `CorruptedIllager.CHASING`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/CorruptedIllager.java:40)
- [ ] `CorruptedIllager.CorruptedIllager(3)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/CorruptedIllager.java:43)
- [ ] `CorruptedIllager.defineSynchedData(1)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/CorruptedIllager.java:47)
- [ ] `CorruptedIllager.registerGoals(0)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/CorruptedIllager.java:53)
- [ ] `CorruptedIllager.getWanderSpeed(0)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/CorruptedIllager.java:68)
- [ ] `CorruptedIllager.canAttack(1)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/CorruptedIllager.java:72)
- [ ] `CorruptedIllager.customServerAiStep(1)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/CorruptedIllager.java:76)
- [ ] `CorruptedIllager.trackChase(0)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/CorruptedIllager.java:86)
- [ ] `CorruptedIllager.isChasing(0)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/CorruptedIllager.java:95)
- [ ] `CorruptedIllager.setChasing(1)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/CorruptedIllager.java:99)
- [ ] `CorruptedIllager.canLead(0)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/CorruptedIllager.java:105)
- [ ] `CorruptedIllager.createCorruptedIllagerAttributes(0)`: it uses Minecraft's `Builder` (com/lion/graveyard/entities/CorruptedIllager.java:110)
- [ ] `CorruptedIllager.getState(0)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/CorruptedIllager.java:119)
- [ ] `CorruptedIllager.finalizeSpawn(4)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/CorruptedIllager.java:123)
- [ ] `CorruptedIllager.considersEntityAsAlly(1)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/CorruptedIllager.java:136)
- [ ] `CorruptedIllager.isModelDamaged(0)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/CorruptedIllager.java:144)
- [ ] `CorruptedIllager.isAttackPending(0)`: its class isn't translated (it extends `HordeGraveyardEntity`) (com/lion/graveyard/entities/CorruptedIllager.java:148)

### `CorruptedIllager.AttackGoal`: it extends Minecraft's `MeleeAttackGoal`

- [ ] `CorruptedIllager.AttackGoal.AttackGoal(1)`: its class isn't translated (it extends Minecraft's `MeleeAttackGoal`) (com/lion/graveyard/entities/CorruptedIllager.java:153)
- [ ] `CorruptedIllager.AttackGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `MeleeAttackGoal`) (com/lion/graveyard/entities/CorruptedIllager.java:157)
- [ ] `CorruptedIllager.AttackGoal.canPerformAttack(1)`: its class isn't translated (it extends Minecraft's `MeleeAttackGoal`) (com/lion/graveyard/entities/CorruptedIllager.java:163)

### `CorruptedIllager.State`


### `CorruptedPillager`: it extends `AnimatedCorruptedIllager`

- [ ] `CorruptedPillager.CorruptedPillager(2)`: its class isn't translated (it extends `AnimatedCorruptedIllager`) (com/lion/graveyard/entities/CorruptedPillager.java:14)
- [ ] `CorruptedPillager.createCorruptedPillagerAttributes(0)`: it uses Minecraft's `Builder` (com/lion/graveyard/entities/CorruptedPillager.java:18)
- [ ] `CorruptedPillager.hasSpawnAnimation(0)`: its class isn't translated (it extends `AnimatedCorruptedIllager`) (com/lion/graveyard/entities/CorruptedPillager.java:26)
- [ ] `CorruptedPillager.getState(0)`: its class isn't translated (it extends `AnimatedCorruptedIllager`) (com/lion/graveyard/entities/CorruptedPillager.java:31)
- [ ] `CorruptedPillager.playAmbientSound(0)`: its class isn't translated (it extends `AnimatedCorruptedIllager`) (com/lion/graveyard/entities/CorruptedPillager.java:36)
- [ ] `CorruptedPillager.playHurtSound(1)`: its class isn't translated (it extends `AnimatedCorruptedIllager`) (com/lion/graveyard/entities/CorruptedPillager.java:39)
- [ ] `CorruptedPillager.playStepSound(2)`: its class isn't translated (it extends `AnimatedCorruptedIllager`) (com/lion/graveyard/entities/CorruptedPillager.java:43)

### `CorruptedVindicator`: it extends `AnimatedCorruptedIllager`

- [ ] `CorruptedVindicator.CorruptedVindicator(2)`: its class isn't translated (it extends `AnimatedCorruptedIllager`) (com/lion/graveyard/entities/CorruptedVindicator.java:11)
- [ ] `CorruptedVindicator.CorruptedVindicator(3)`: its class isn't translated (it extends `AnimatedCorruptedIllager`) (com/lion/graveyard/entities/CorruptedVindicator.java:15)
- [ ] `CorruptedVindicator.hasSpawnAnimation(0)`: its class isn't translated (it extends `AnimatedCorruptedIllager`) (com/lion/graveyard/entities/CorruptedVindicator.java:19)
- [ ] `CorruptedVindicator.isModelDamaged(0)`: its class isn't translated (it extends `AnimatedCorruptedIllager`) (com/lion/graveyard/entities/CorruptedVindicator.java:24)
- [ ] `CorruptedVindicator.getState(0)`: its class isn't translated (it extends `AnimatedCorruptedIllager`) (com/lion/graveyard/entities/CorruptedVindicator.java:29)
- [ ] `CorruptedVindicator.playAmbientSound(0)`: its class isn't translated (it extends `AnimatedCorruptedIllager`) (com/lion/graveyard/entities/CorruptedVindicator.java:34)
- [ ] `CorruptedVindicator.playHurtSound(1)`: its class isn't translated (it extends `AnimatedCorruptedIllager`) (com/lion/graveyard/entities/CorruptedVindicator.java:37)
- [ ] `CorruptedVindicator.playStepSound(2)`: its class isn't translated (it extends `AnimatedCorruptedIllager`) (com/lion/graveyard/entities/CorruptedVindicator.java:41)

### `CorvidEntity`: it extends Minecraft's `PathfinderMob`

- [x] `CorvidEntity.WALK_MIN_SPEED_SQ` (com/lion/graveyard/entities/CorvidEntity.java:61)
- [x] `CorvidEntity.GROUND_TURN_MIN_SPEED_SQ` (com/lion/graveyard/entities/CorvidEntity.java:62)
- [x] `CorvidEntity.WALK_HOLD_TICKS` (com/lion/graveyard/entities/CorvidEntity.java:63)
- [ ] `CorvidEntity.TAKEOFF`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/CorvidEntity.java:64)
- [ ] `CorvidEntity.GLIDE`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/CorvidEntity.java:65)
- [ ] `CorvidEntity.FLY`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/CorvidEntity.java:66)
- [ ] `CorvidEntity.LAND`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/CorvidEntity.java:67)
- [ ] `CorvidEntity.HOP`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/CorvidEntity.java:68)
- [ ] `CorvidEntity.WATCH`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/CorvidEntity.java:69)
- [ ] `CorvidEntity.IDLE`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/CorvidEntity.java:70)
- [ ] `CorvidEntity.FLIGHT_STATE`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/CorvidEntity.java:71)
- [x] `CorvidEntity.GRAVESTONE_CHECK_INTERVAL` (com/lion/graveyard/entities/CorvidEntity.java:72)
- [x] `CorvidEntity.GRAVESTONE_WITNESS_RANGE` (com/lion/graveyard/entities/CorvidEntity.java:73)
- [x] `CorvidEntity.GLIDE_MIN` (com/lion/graveyard/entities/CorvidEntity.java:74)
- [x] `CorvidEntity.GLIDE_MAX` (com/lion/graveyard/entities/CorvidEntity.java:75)
- [x] `CorvidEntity.FLAP_BURST` (com/lion/graveyard/entities/CorvidEntity.java:76)
- [x] `CorvidEntity.HOME_RADIUS` (com/lion/graveyard/entities/CorvidEntity.java:77)
- [x] `CorvidEntity.DEATH_ANIMATION_DURATION` (com/lion/graveyard/entities/CorvidEntity.java:78)
- [x] `CorvidEntity.DEAD_FALL_LIMIT` (com/lion/graveyard/entities/CorvidEntity.java:79)
- [x] `CorvidEntity.IMPACT_REST_TICKS` (com/lion/graveyard/entities/CorvidEntity.java:80)
- [x] `CorvidEntity.FLAP_SOUND_INTERVAL` (com/lion/graveyard/entities/CorvidEntity.java:81)
- [x] `CorvidEntity.TAKEOFF_SOUND_COOLDOWN` (com/lion/graveyard/entities/CorvidEntity.java:82)
- [x] `CorvidEntity.FLAP_SOUND_VOLUME` (com/lion/graveyard/entities/CorvidEntity.java:83)
- [x] `CorvidEntity.TAKEOFF_SOUND_VOLUME` (com/lion/graveyard/entities/CorvidEntity.java:84)
- [x] `CorvidEntity.LAND_SOUND_VOLUME` (com/lion/graveyard/entities/CorvidEntity.java:85)
- [ ] `CorvidEntity.CorvidEntity(2)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:112)
- [ ] `CorvidEntity.createCorvidAttributes(0)`: it uses Minecraft's `Builder` (com/lion/graveyard/entities/CorvidEntity.java:120)
- [ ] `CorvidEntity.getSpecies(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:128)
- [ ] `CorvidEntity.tickHeadTurn(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:132)
- [ ] `CorvidEntity.getFlightState(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:146)
- [ ] `CorvidEntity.defineSynchedData(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:150)
- [ ] `CorvidEntity.finalizeSpawn(4)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:155)
- [ ] `CorvidEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:165)
- [ ] `CorvidEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:174)
- [ ] `CorvidEntity.getHome(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:180)
- [ ] `CorvidEntity.setHome(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:185)
- [ ] `CorvidEntity.restoreFrom(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:189)
- [ ] `CorvidEntity.isFarFromHome(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:196)
- [ ] `CorvidEntity.isPreparingTakeoff(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:206)
- [ ] `CorvidEntity.getPerchTag(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:210)
- [ ] `CorvidEntity.registerGoals(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:214)
- [ ] `CorvidEntity.createNavigation(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:222)
- [ ] `CorvidEntity.getFlyingSpeed(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:230)
- [ ] `CorvidEntity.tick(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:234)
- [ ] `CorvidEntity.aiStep(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:392)
- [ ] `CorvidEntity.makeStuckInBlock(2)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:464)
- [ ] `CorvidEntity.computeNearGround(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:473)
- [ ] `CorvidEntity.takeOff(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:486)
- [ ] `CorvidEntity.lift(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:503)
- [ ] `CorvidEntity.playWingSound(2)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:516)
- [ ] `CorvidEntity.tryPeck(2)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:520)
- [ ] `CorvidEntity.isPecking(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:537)
- [ ] `CorvidEntity.isOnFeedingGround(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:541)
- [ ] `CorvidEntity.isFlying(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:547)
- [ ] `CorvidEntity.isOnGravestone(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:551)
- [ ] `CorvidEntity.isPerched(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:556)
- [ ] `CorvidEntity.isNearGround(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:560)
- [ ] `CorvidEntity.isLanding(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:564)
- [ ] `CorvidEntity.isWatching(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:568)
- [ ] `CorvidEntity.isGliding(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:572)
- [ ] `CorvidEntity.isHopping(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:576)
- [ ] `CorvidEntity.isWalking(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:580)
- [ ] `CorvidEntity.canWalk(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:584)
- [ ] `CorvidEntity.canPeck(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:588)
- [ ] `CorvidEntity.state(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:592)
- [ ] `CorvidEntity.isTouching(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:596)
- [ ] `CorvidEntity.isOnBlock(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:601)
- [ ] `CorvidEntity.canSpawn(5)`: it uses Minecraft's `EntityType` (com/lion/graveyard/entities/CorvidEntity.java:606)
- [ ] `CorvidEntity.checkSpawnRules(2)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:612)
- [ ] `CorvidEntity.getWalkTargetValue(2)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:616)
- [ ] `CorvidEntity.causeFallDamage(3)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:620)
- [ ] `CorvidEntity.checkFallDamage(4)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:624)
- [ ] `CorvidEntity.playStepSound(2)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:627)
- [ ] `CorvidEntity.getAmbientSound(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:630)
- [ ] `CorvidEntity.getHurtSound(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:639)
- [ ] `CorvidEntity.getDeathSound(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:643)
- [ ] `CorvidEntity.getAmbientSoundInterval(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:647)
- [ ] `CorvidEntity.getSoundVolume(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:651)
- [ ] `CorvidEntity.isPushable(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:655)
- [ ] `CorvidEntity.getMaxSpawnClusterSize(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:659)
- [ ] `CorvidEntity.stepLookWeight(2)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:663)
- [ ] `CorvidEntity.tickDeath(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:674)
- [ ] `CorvidEntity.removeDeadBody(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:694)
- [ ] `CorvidEntity.die(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:699)
- [ ] `CorvidEntity.registerControllers(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:706)
- [ ] `CorvidEntity.getAnimatableInstanceCache(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/CorvidEntity.java:723)

### `CorvidEntity.FlightState`: its constants carry values

- [ ] `CorvidEntity.FlightState.FlightState(1)`: its class isn't translated (its constants carry values) (com/lion/graveyard/entities/CorvidEntity.java:738)
- [ ] `CorvidEntity.FlightState.byId(1)`: it uses FlightState, which isn't translated (its constants carry values) (com/lion/graveyard/entities/CorvidEntity.java:742)

### `CorvidEntity.Species`: its constants carry values

- [ ] `CorvidEntity.Species.Species(2)`: its class isn't translated (its constants carry values) (com/lion/graveyard/entities/CorvidEntity.java:764)
- [ ] `CorvidEntity.Species.getTexture(0)`: its class isn't translated (its constants carry values) (com/lion/graveyard/entities/CorvidEntity.java:772)
- [ ] `CorvidEntity.Species.getWalkAnimation(0)`: its class isn't translated (its constants carry values) (com/lion/graveyard/entities/CorvidEntity.java:776)
- [ ] `CorvidEntity.Species.getPeckAnimation(0)`: its class isn't translated (its constants carry values) (com/lion/graveyard/entities/CorvidEntity.java:780)
- [ ] `CorvidEntity.Species.getDeathAnimation(0)`: its class isn't translated (its constants carry values) (com/lion/graveyard/entities/CorvidEntity.java:784)
- [ ] `CorvidEntity.Species.getModel(0)`: its class isn't translated (its constants carry values) (com/lion/graveyard/entities/CorvidEntity.java:788)

### `CrowEntity`: it extends `CorvidEntity`

- [x] `CrowEntity.MAX_FLOCK` (com/lion/graveyard/entities/CrowEntity.java:25)
- [x] `CrowEntity.FLOCK_SEARCH_RADIUS` (com/lion/graveyard/entities/CrowEntity.java:26)
- [x] `CrowEntity.CLOSE_TO_LEADER_SQ` (com/lion/graveyard/entities/CrowEntity.java:27)
- [x] `CrowEntity.ALARM_TICKS` (com/lion/graveyard/entities/CrowEntity.java:28)
- [x] `CrowEntity.ALARM_SPREAD_RADIUS` (com/lion/graveyard/entities/CrowEntity.java:29)
- [x] `CrowEntity.ALARM_TICKS_PER_BLOCK` (com/lion/graveyard/entities/CrowEntity.java:30)
- [x] `CrowEntity.ANCHOR_INTERVAL` (com/lion/graveyard/entities/CrowEntity.java:31)
- [ ] `CrowEntity.CrowEntity(2)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:42)
- [ ] `CrowEntity.getSpecies(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:46)
- [ ] `CrowEntity.getPerchTag(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:51)
- [ ] `CrowEntity.getMaxSpawnClusterSize(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:56)
- [ ] `CrowEntity.registerGoals(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:61)
- [ ] `CrowEntity.tick(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:72)
- [ ] `CrowEntity.remove(1)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:92)
- [ ] `CrowEntity.hasLeader(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:97)
- [ ] `CrowEntity.getFlockHead(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:101)
- [ ] `CrowEntity.isSentinel(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:105)
- [ ] `CrowEntity.canAcceptMembers(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:109)
- [ ] `CrowEntity.isCloseToLeader(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:113)
- [ ] `CrowEntity.moveTowardLeader(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:117)
- [ ] `CrowEntity.findFlock(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:124)
- [ ] `CrowEntity.leaveFlock(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:138)
- [ ] `CrowEntity.getFlockSize(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:145)
- [ ] `CrowEntity.isAlarmed(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:149)
- [ ] `CrowEntity.raiseAlarm(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:153)
- [ ] `CrowEntity.isCircling(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:171)
- [ ] `CrowEntity.setCircling(1)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:175)
- [ ] `CrowEntity.getAnchor(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:179)
- [ ] `CrowEntity.updateAnchor(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:184)
- [ ] `CrowEntity.getAmbientSound(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:195)
- [ ] `CrowEntity.getAmbientSoundInterval(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:201)
- [ ] `CrowEntity.isGliding(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:206)
- [ ] `CrowEntity.setGliding(1)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:211)
- [ ] `CrowEntity.canPeck(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/CrowEntity.java:215)

### `FallingCorpse`: it extends Minecraft's `Monster`

- [ ] `FallingCorpse.IS_FALLING`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/FallingCorpse.java:38)
- [ ] `FallingCorpse.HAS_COLLIDED`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/FallingCorpse.java:39)
- [ ] `FallingCorpse.SINKING`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/FallingCorpse.java:40)
- [x] `FallingCorpse.LANDING_TICKS` (com/lion/graveyard/entities/FallingCorpse.java:42)
- [x] `FallingCorpse.SINK_START_TICKS` (com/lion/graveyard/entities/FallingCorpse.java:43)
- [x] `FallingCorpse.IMPACT_DELAY` (com/lion/graveyard/entities/FallingCorpse.java:44)
- [x] `FallingCorpse.LANDING_TRANSITION_TICKS` (com/lion/graveyard/entities/FallingCorpse.java:45)
- [ ] `FallingCorpse.FallingCorpse(2)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:51)
- [ ] `FallingCorpse.getRotation(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:56)
- [ ] `FallingCorpse.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:60)
- [ ] `FallingCorpse.readAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:65)
- [ ] `FallingCorpse.causeFallDamage(3)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:70)
- [ ] `FallingCorpse.hurtServer(3)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:78)
- [ ] `FallingCorpse.resetFallDistance(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:82)
- [ ] `FallingCorpse.defineSynchedData(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:87)
- [ ] `FallingCorpse.registerControllers(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:94)
- [ ] `FallingCorpse.getAnimatableInstanceCache(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:105)
- [ ] `FallingCorpse.createFallingCorpseAttributes(0)`: it uses Minecraft's `Builder` (com/lion/graveyard/entities/FallingCorpse.java:109)
- [ ] `FallingCorpse.aiStep(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:113)
- [ ] `FallingCorpse.customServerAiStep(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:139)
- [ ] `FallingCorpse.impact(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:174)
- [ ] `FallingCorpse.playerTouch(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:184)
- [ ] `FallingCorpse.isFalling(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:193)
- [ ] `FallingCorpse.setIsFalling(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:197)
- [ ] `FallingCorpse.isSinking(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:201)
- [ ] `FallingCorpse.setSinking(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:205)
- [ ] `FallingCorpse.hasCollided(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:209)
- [ ] `FallingCorpse.setHasCollided(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/FallingCorpse.java:213)

### `GhoulEntity`: it extends `AngerableGraveyardEntity`

- [x] `GhoulEntity.EMERGE_WITNESS_INTERVAL` (com/lion/graveyard/entities/GhoulEntity.java:59)
- [x] `GhoulEntity.EMERGE_WITNESS_RANGE` (com/lion/graveyard/entities/GhoulEntity.java:60)
- [x] `GhoulEntity.SLOWNESS_ID` (com/lion/graveyard/entities/GhoulEntity.java:61)
- [ ] `GhoulEntity.SLOWNESS_EFFECT`: it uses Minecraft's `AttributeModifier` (com/lion/graveyard/entities/GhoulEntity.java:62)
- [ ] `GhoulEntity.VARIANT`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulEntity.java:63)
- [ ] `GhoulEntity.ATTACK_ANIM_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulEntity.java:64)
- [ ] `GhoulEntity.RAGE_ANIM_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulEntity.java:65)
- [ ] `GhoulEntity.ANIMATION`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulEntity.java:66)
- [ ] `GhoulEntity.SPAWN_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulEntity.java:67)
- [ ] `GhoulEntity.IS_RAGING`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulEntity.java:68)
- [ ] `GhoulEntity.RUN_CRAWLING`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulEntity.java:69)
- [ ] `GhoulEntity.NAVIGATING`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulEntity.java:70)
- [ ] `GhoulEntity.RUNNING`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulEntity.java:71)
- [x] `GhoulEntity.ANIMATION_IDLE` (com/lion/graveyard/entities/GhoulEntity.java:85)
- [x] `GhoulEntity.ANIMATION_WALK` (com/lion/graveyard/entities/GhoulEntity.java:86)
- [x] `GhoulEntity.ANIMATION_RAGE` (com/lion/graveyard/entities/GhoulEntity.java:87)
- [x] `GhoulEntity.ANIMATION_DEATH` (com/lion/graveyard/entities/GhoulEntity.java:88)
- [x] `GhoulEntity.ANIMATION_RUNNING` (com/lion/graveyard/entities/GhoulEntity.java:89)
- [x] `GhoulEntity.ANIMATION_MELEE` (com/lion/graveyard/entities/GhoulEntity.java:90)
- [x] `GhoulEntity.ANIMATION_SPAWN` (com/lion/graveyard/entities/GhoulEntity.java:91)
- [x] `GhoulEntity.RUN_START_LIMB_SWING` (com/lion/graveyard/entities/GhoulEntity.java:92)
- [x] `GhoulEntity.RUN_STOP_LIMB_SWING` (com/lion/graveyard/entities/GhoulEntity.java:93)
- [x] `GhoulEntity.ATTACK_ANIM_SYNC_TICKS` (com/lion/graveyard/entities/GhoulEntity.java:95)
- [x] `GhoulEntity.SPAWN_ANIMATION_DURATION` (com/lion/graveyard/entities/GhoulEntity.java:97)
- [x] `GhoulEntity.CHASE_SOUND_COOLDOWN_TICKS` (com/lion/graveyard/entities/GhoulEntity.java:98)
- [x] `GhoulEntity.CHASE_END_TICKS` (com/lion/graveyard/entities/GhoulEntity.java:99)
- [x] `GhoulEntity.BIPED_SPEED_ID` (com/lion/graveyard/entities/GhoulEntity.java:100)
- [ ] `GhoulEntity.BIPED_SPEED_BOOST`: it uses Minecraft's `AttributeModifier` (com/lion/graveyard/entities/GhoulEntity.java:101)
- [x] `GhoulEntity.WALK_STEP_DISTANCE` (com/lion/graveyard/entities/GhoulEntity.java:102)
- [x] `GhoulEntity.CHASE_STEP_DISTANCE` (com/lion/graveyard/entities/GhoulEntity.java:103)
- [x] `GhoulEntity.DEATH_ANIMATION_DURATION` (com/lion/graveyard/entities/GhoulEntity.java:104)
- [ ] `GhoulEntity.GhoulEntity(2)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:107)
- [ ] `GhoulEntity.defineSynchedData(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:111)
- [ ] `GhoulEntity.registerGoals(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:125)
- [ ] `GhoulEntity.createGhoulAttributes(0)`: it uses Minecraft's `Builder` (com/lion/graveyard/entities/GhoulEntity.java:139)
- [ ] `GhoulEntity.getAnimationState(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:148)
- [ ] `GhoulEntity.setAnimationState(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:152)
- [ ] `GhoulEntity.getAttackAnimTimer(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:156)
- [ ] `GhoulEntity.setAttackAnimTimer(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:160)
- [ ] `GhoulEntity.getSpawnTimer(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:164)
- [ ] `GhoulEntity.setSpawnTimer(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:168)
- [ ] `GhoulEntity.getRageAnimTimer(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:172)
- [ ] `GhoulEntity.setRageAnimTimer(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:176)
- [ ] `GhoulEntity.isRaging(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:180)
- [ ] `GhoulEntity.setIsRaging(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:184)
- [ ] `GhoulEntity.getVariant(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:188)
- [ ] `GhoulEntity.setVariant(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:193)
- [ ] `GhoulEntity.trackChase(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:197)
- [ ] `GhoulEntity.customServerAiStep(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:216)
- [ ] `GhoulEntity.aiStep(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:251)
- [ ] `GhoulEntity.tick(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:279)
- [ ] `GhoulEntity.restartAttackClipOnRearm(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:307)
- [ ] `GhoulEntity.isImmobile(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:322)
- [ ] `GhoulEntity.tickDeath(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:326)
- [ ] `GhoulEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:334)
- [ ] `GhoulEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:341)
- [ ] `GhoulEntity.aggroMobs(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:348)
- [ ] `GhoulEntity.registerControllers(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:357)
- [ ] `GhoulEntity.isDying(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:377)
- [ ] `GhoulEntity.isRunningAtSpeed(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:381)
- [ ] `GhoulEntity.isRunning(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:386)
- [ ] `GhoulEntity.getAnimatableInstanceCache(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:390)
- [ ] `GhoulEntity.playAmbientSound(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:394)
- [ ] `GhoulEntity.nextStep(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:398)
- [ ] `GhoulEntity.playStepSound(2)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:402)
- [ ] `GhoulEntity.playHurtSound(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:406)
- [ ] `GhoulEntity.die(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/GhoulEntity.java:410)

### `GhoulingBindings`: it extends Minecraft's `SavedData`

- [x] `GhoulingBindings.LEGACY_DATA_FILE` (com/lion/graveyard/entities/GhoulingBindings.java:30)
- [x] `GhoulingBindings.LEGACY_FILE` (com/lion/graveyard/entities/GhoulingBindings.java:31)
- [ ] `GhoulingBindings.CODEC`: it uses Minecraft's `Codec` (com/lion/graveyard/entities/GhoulingBindings.java:32)
- [ ] `GhoulingBindings.TYPE`: it uses Minecraft's `SavedDataType` (com/lion/graveyard/entities/GhoulingBindings.java:33)
- [ ] `GhoulingBindings.get(1)`: it uses GhoulingBindings, which isn't translated (it extends Minecraft's `SavedData`) (com/lion/graveyard/entities/GhoulingBindings.java:36)
- [ ] `GhoulingBindings.get(1)`: it uses GhoulingBindings, which isn't translated (it extends Minecraft's `SavedData`) (com/lion/graveyard/entities/GhoulingBindings.java:47)
- [ ] `GhoulingBindings.update(1)`: its class isn't translated (it extends Minecraft's `SavedData`) (com/lion/graveyard/entities/GhoulingBindings.java:52)
- [ ] `GhoulingBindings.remove(1)`: its class isn't translated (it extends Minecraft's `SavedData`) (com/lion/graveyard/entities/GhoulingBindings.java:61)
- [ ] `GhoulingBindings.toNbt(0)`: its class isn't translated (it extends Minecraft's `SavedData`) (com/lion/graveyard/entities/GhoulingBindings.java:67)
- [ ] `GhoulingBindings.fromNbt(1)`: it uses GhoulingBindings, which isn't translated (it extends Minecraft's `SavedData`) (com/lion/graveyard/entities/GhoulingBindings.java:88)
- [ ] `GhoulingBindings.importLegacy(1)`: it uses GhoulingBindings, which isn't translated (it extends Minecraft's `SavedData`) (com/lion/graveyard/entities/GhoulingBindings.java:112)

### `GhoulingBindings.Binding`: it uses `UUID` (java.util)

- [ ] `GhoulingBindings.Binding.Binding(4)`: its class isn't translated (it uses `UUID` (java.util)) (com/lion/graveyard/entities/GhoulingBindings.java:142)
- [ ] `GhoulingBindings.Binding.owner(0)`: its class isn't translated (it uses `UUID` (java.util)) (com/lion/graveyard/entities/GhoulingBindings.java:142)
- [ ] `GhoulingBindings.Binding.dimension(0)`: its class isn't translated (it uses `UUID` (java.util)) (com/lion/graveyard/entities/GhoulingBindings.java:142)
- [ ] `GhoulingBindings.Binding.pos(0)`: its class isn't translated (it uses `UUID` (java.util)) (com/lion/graveyard/entities/GhoulingBindings.java:142)
- [ ] `GhoulingBindings.Binding.sitting(0)`: its class isn't translated (it uses `UUID` (java.util)) (com/lion/graveyard/entities/GhoulingBindings.java:142)

### `GhoulingEntity`: it extends `GraveyardMinionEntity`

- [x] `GhoulingEntity.BINDING_INTERVAL` (com/lion/graveyard/entities/GhoulingEntity.java:84)
- [x] `GhoulingEntity.SPAWN_ANIMATION_DURATION` (com/lion/graveyard/entities/GhoulingEntity.java:85)
- [x] `GhoulingEntity.PICKUP_INTERVAL` (com/lion/graveyard/entities/GhoulingEntity.java:86)
- [x] `GhoulingEntity.DEATH_ANIMATION_DURATION` (com/lion/graveyard/entities/GhoulingEntity.java:87)
- [ ] `GhoulingEntity.ATTACK_ANIM_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulingEntity.java:89)
- [ ] `GhoulingEntity.ANIMATION`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulingEntity.java:90)
- [ ] `GhoulingEntity.SPAWN_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulingEntity.java:91)
- [ ] `GhoulingEntity.TELEPORT_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulingEntity.java:92)
- [ ] `GhoulingEntity.STAFF`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulingEntity.java:93)
- [ ] `GhoulingEntity.COFFIN`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulingEntity.java:94)
- [ ] `GhoulingEntity.VARIANT`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GhoulingEntity.java:95)
- [x] `GhoulingEntity.ANIMATION_SPAWN` (com/lion/graveyard/entities/GhoulingEntity.java:102)
- [x] `GhoulingEntity.ANIMATION_IDLE` (com/lion/graveyard/entities/GhoulingEntity.java:103)
- [x] `GhoulingEntity.ANIMATION_MELEE` (com/lion/graveyard/entities/GhoulingEntity.java:104)
- [x] `GhoulingEntity.ANIMATION_WALK` (com/lion/graveyard/entities/GhoulingEntity.java:105)
- [x] `GhoulingEntity.ANIMATION_DEATH` (com/lion/graveyard/entities/GhoulingEntity.java:106)
- [ ] `GhoulingEntity.GhoulingEntity(2)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:116)
- [ ] `GhoulingEntity.registerGoals(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:120)
- [ ] `GhoulingEntity.createGhoulingAttributes(0)`: it uses Minecraft's `Builder` (com/lion/graveyard/entities/GhoulingEntity.java:135)
- [ ] `GhoulingEntity.defineSynchedData(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:146)
- [ ] `GhoulingEntity.customServerAiStep(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:158)
- [ ] `GhoulingEntity.aiStep(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:187)
- [ ] `GhoulingEntity.tick(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:215)
- [ ] `GhoulingEntity.restartAttackClipOnRearm(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:248)
- [ ] `GhoulingEntity.mayTake(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:263)
- [ ] `GhoulingEntity.canPickUpPassingItems(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:267)
- [ ] `GhoulingEntity.getMaxFallDistance(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:271)
- [ ] `GhoulingEntity.isImmobile(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:275)
- [ ] `GhoulingEntity.storeInCoffin(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:279)
- [ ] `GhoulingEntity.collect(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:303)
- [ ] `GhoulingEntity.isCoffinFull(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:307)
- [ ] `GhoulingEntity.getAnimationState(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:322)
- [ ] `GhoulingEntity.setAnimationState(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:326)
- [ ] `GhoulingEntity.getAttackAnimTimer(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:330)
- [ ] `GhoulingEntity.setAttackAnimTimer(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:334)
- [ ] `GhoulingEntity.getTeleportTimer(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:338)
- [ ] `GhoulingEntity.setTeleportTimer(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:342)
- [ ] `GhoulingEntity.prepareSpawn(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:346)
- [ ] `GhoulingEntity.onSummoned(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:351)
- [ ] `GhoulingEntity.playSpawnSounds(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:356)
- [ ] `GhoulingEntity.removeWhenFarAway(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:361)
- [ ] `GhoulingEntity.registerControllers(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:365)
- [ ] `GhoulingEntity.getAnimatableInstanceCache(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:385)
- [ ] `GhoulingEntity.mobInteract(2)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:389)
- [ ] `GhoulingEntity.toggleSitting(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:454)
- [ ] `GhoulingEntity.removeForStorage(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:464)
- [ ] `GhoulingEntity.tickDeath(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:486)
- [ ] `GhoulingEntity.getSpawnTimer(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:494)
- [ ] `GhoulingEntity.setSpawnTimer(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:498)
- [ ] `GhoulingEntity.getVariant(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:502)
- [ ] `GhoulingEntity.setVariant(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:506)
- [ ] `GhoulingEntity.getBondId(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:510)
- [ ] `GhoulingEntity.setBondId(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:515)
- [ ] `GhoulingEntity.acceptsStaff(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:519)
- [ ] `GhoulingEntity.getStaff(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:528)
- [ ] `GhoulingEntity.setStaff(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:532)
- [ ] `GhoulingEntity.hasCoffin(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:536)
- [ ] `GhoulingEntity.setHasCoffin(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:540)
- [ ] `GhoulingEntity.dropEquipment(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:544)
- [ ] `GhoulingEntity.ensureInventory(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:572)
- [ ] `GhoulingEntity.closeCoffinScreens(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:578)
- [ ] `GhoulingEntity.dropCustomDeathLoot(3)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:588)
- [ ] `GhoulingEntity.causeFallDamage(3)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:599)
- [ ] `GhoulingEntity.die(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:603)
- [ ] `GhoulingEntity.playAmbientSound(0)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:612)
- [ ] `GhoulingEntity.playStepSound(2)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:616)
- [ ] `GhoulingEntity.playHurtSound(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:620)
- [ ] `GhoulingEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:624)
- [ ] `GhoulingEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:643)
- [ ] `GhoulingEntity.createMenu(3)`: its class isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/entities/GhoulingEntity.java:661)

### `GhoulingEntity.CoffinInventory`: it extends Minecraft's `SimpleContainer`

- [ ] `GhoulingEntity.CoffinInventory.CoffinInventory(2)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/lion/graveyard/entities/GhoulingEntity.java:669)
- [ ] `GhoulingEntity.CoffinInventory.stillValid(1)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/lion/graveyard/entities/GhoulingEntity.java:674)

### `GhoulingEntity.GhoulingEscapeDangerGoal`: it extends Minecraft's `PanicGoal`

- [ ] `GhoulingEntity.GhoulingEscapeDangerGoal.GhoulingEscapeDangerGoal(1)`: its class isn't translated (it extends Minecraft's `PanicGoal`) (com/lion/graveyard/entities/GhoulingEntity.java:680)
- [ ] `GhoulingEntity.GhoulingEscapeDangerGoal.shouldPanic(0)`: its class isn't translated (it extends Minecraft's `PanicGoal`) (com/lion/graveyard/entities/GhoulingEntity.java:684)

### `GraveyardMinionEntity`: it extends Minecraft's `PathfinderMob`

- [ ] `GraveyardMinionEntity.OWNER_UUID`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GraveyardMinionEntity.java:28)
- [ ] `GraveyardMinionEntity.TAMEABLE_FLAGS`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/GraveyardMinionEntity.java:29)
- [ ] `GraveyardMinionEntity.GraveyardMinionEntity(2)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:32)
- [ ] `GraveyardMinionEntity.defineSynchedData(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:38)
- [ ] `GraveyardMinionEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:44)
- [ ] `GraveyardMinionEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:54)
- [ ] `GraveyardMinionEntity.getOwnerUuid(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:74)
- [ ] `GraveyardMinionEntity.setOwnerUuid(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:88)
- [ ] `GraveyardMinionEntity.setOwner(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:92)
- [ ] `GraveyardMinionEntity.isSitting(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:96)
- [ ] `GraveyardMinionEntity.setSitting(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:100)
- [ ] `GraveyardMinionEntity.isInSittingPose(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:107)
- [ ] `GraveyardMinionEntity.setInSittingPose(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:111)
- [ ] `GraveyardMinionEntity.getOwner(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:120)
- [ ] `GraveyardMinionEntity.canAttack(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:130)
- [ ] `GraveyardMinionEntity.setTarget(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:134)
- [ ] `GraveyardMinionEntity.sharesOwner(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:138)
- [ ] `GraveyardMinionEntity.isOwner(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:149)
- [ ] `GraveyardMinionEntity.considersEntityAsAlly(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:153)
- [ ] `GraveyardMinionEntity.canAttackWithOwner(2)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:162)
- [ ] `GraveyardMinionEntity.isPvpAllowed(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/GraveyardMinionEntity.java:183)

### `HordeGraveyardEntity`: it extends `HostileGraveyardEntity`

- [ ] `HordeGraveyardEntity.HordeGraveyardEntity(3)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:35)
- [ ] `HordeGraveyardEntity.registerGoals(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:39)
- [ ] `HordeGraveyardEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:45)
- [ ] `HordeGraveyardEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:60)
- [ ] `HordeGraveyardEntity.canLead(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:77)
- [ ] `HordeGraveyardEntity.finalizeSpawn(4)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:81)
- [ ] `HordeGraveyardEntity.removeWhenFarAway(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:100)
- [ ] `HordeGraveyardEntity.setPatrolTarget(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:104)
- [ ] `HordeGraveyardEntity.setPatrolLeader(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:109)
- [ ] `HordeGraveyardEntity.setHordeCaptain(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:114)
- [ ] `HordeGraveyardEntity.isPatrolLeader(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:118)
- [ ] `HordeGraveyardEntity.getPatrolTarget(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:122)
- [ ] `HordeGraveyardEntity.hasPatrolTarget(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:126)
- [ ] `HordeGraveyardEntity.setRandomPatrolTarget(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:130)
- [ ] `HordeGraveyardEntity.setPatrolling(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:135)
- [ ] `HordeGraveyardEntity.hasNoRaid(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:139)
- [ ] `HordeGraveyardEntity.isRaidCenterSet(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:143)
- [ ] `HordeGraveyardEntity.createWitherRose(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:147)

### `HordeGraveyardEntity.PatrolApproachGoal`: it extends Minecraft's `Goal`

- [ ] `HordeGraveyardEntity.PatrolApproachGoal.PatrolApproachGoal(2)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:160)
- [ ] `HordeGraveyardEntity.PatrolApproachGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:166)
- [ ] `HordeGraveyardEntity.PatrolApproachGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:174)
- [ ] `HordeGraveyardEntity.PatrolApproachGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:186)
- [ ] `HordeGraveyardEntity.PatrolApproachGoal.requiresUpdateEveryTick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:202)
- [ ] `HordeGraveyardEntity.PatrolApproachGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/HordeGraveyardEntity.java:206)

### `HordeGraveyardEntity.PatrolGoal`: it's generic

- [ ] `HordeGraveyardEntity.PatrolGoal.PatrolGoal(3)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/HordeGraveyardEntity.java:229)
- [ ] `HordeGraveyardEntity.PatrolGoal.canUse(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/HordeGraveyardEntity.java:243)
- [ ] `HordeGraveyardEntity.PatrolGoal.start(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/HordeGraveyardEntity.java:252)
- [ ] `HordeGraveyardEntity.PatrolGoal.stop(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/HordeGraveyardEntity.java:255)
- [ ] `HordeGraveyardEntity.PatrolGoal.tick(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/HordeGraveyardEntity.java:258)
- [ ] `HordeGraveyardEntity.PatrolGoal.findPatrolTargets(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/HordeGraveyardEntity.java:287)
- [ ] `HordeGraveyardEntity.PatrolGoal.wander(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/HordeGraveyardEntity.java:297)

### `GraveyardHordeSpawner`: it implements Minecraft's `CustomSpawner`

- [x] `GraveyardHordeSpawner.CAPTAIN_SPOTS` (com/lion/graveyard/entities/horde/GraveyardHordeSpawner.java:28)
- [ ] `GraveyardHordeSpawner.monstersAllowed(1)`: it uses Minecraft's `ServerLevel` (com/lion/graveyard/entities/horde/GraveyardHordeSpawner.java:31)
- [ ] `GraveyardHordeSpawner.tick(2)`: its class isn't translated (it implements Minecraft's `CustomSpawner`) (com/lion/graveyard/entities/horde/GraveyardHordeSpawner.java:35)
- [ ] `GraveyardHordeSpawner.spawnFollowers(5)`: its class isn't translated (it implements Minecraft's `CustomSpawner`) (com/lion/graveyard/entities/horde/GraveyardHordeSpawner.java:86)
- [ ] `GraveyardHordeSpawner.spawnHordeEntity(5)`: its class isn't translated (it implements Minecraft's `CustomSpawner`) (com/lion/graveyard/entities/horde/GraveyardHordeSpawner.java:95)

### `HostileGraveyardEntity`: it extends Minecraft's `Monster`

- [ ] `HostileGraveyardEntity.CAN_BURN_IN_SUNLIGHT`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/HostileGraveyardEntity.java:26)
- [ ] `HostileGraveyardEntity.HostileGraveyardEntity(3)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/HostileGraveyardEntity.java:30)
- [ ] `HostileGraveyardEntity.defineSynchedData(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/HostileGraveyardEntity.java:35)
- [ ] `HostileGraveyardEntity.isSunBurnTick(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/HostileGraveyardEntity.java:40)
- [ ] `HostileGraveyardEntity.burnsInDaylight(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/HostileGraveyardEntity.java:55)
- [ ] `HostileGraveyardEntity.canBurnInSunlight(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/HostileGraveyardEntity.java:59)
- [ ] `HostileGraveyardEntity.setCanBurnInSunlight(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/HostileGraveyardEntity.java:63)
- [ ] `HostileGraveyardEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/HostileGraveyardEntity.java:67)
- [ ] `HostileGraveyardEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/HostileGraveyardEntity.java:73)
- [ ] `HostileGraveyardEntity.canBeAffected(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/HostileGraveyardEntity.java:79)
- [ ] `HostileGraveyardEntity.aiStep(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/HostileGraveyardEntity.java:83)
- [ ] `HostileGraveyardEntity.canSpawnInDarkness(5)`: it uses Minecraft's `EntityType` (com/lion/graveyard/entities/HostileGraveyardEntity.java:112)
- [ ] `HostileGraveyardEntity.canSpawnInLight(5)`: it uses Minecraft's `EntityType` (com/lion/graveyard/entities/HostileGraveyardEntity.java:120)

### `LegacyEntityData`

- [ ] `LegacyEntityData.HAND_SLOTS`: it uses Minecraft's `EquipmentSlot` (com/lion/graveyard/entities/LegacyEntityData.java:44)
- [ ] `LegacyEntityData.ARMOR_SLOTS`: it uses Minecraft's `EquipmentSlot` (com/lion/graveyard/entities/LegacyEntityData.java:45)
- [ ] `LegacyEntityData.MODIFIER_IDS_BY_UUID`: it uses `UUID` (java.util) (com/lion/graveyard/entities/LegacyEntityData.java:46)
- [x] `LegacyEntityData.MODIFIER_IDS_BY_NAME` (com/lion/graveyard/entities/LegacyEntityData.java:88)
- [x] `LegacyEntityData.ATTRIBUTE_PREFIXES` (com/lion/graveyard/entities/LegacyEntityData.java:100)
- [x] `LegacyEntityData.OPERATIONS` (com/lion/graveyard/entities/LegacyEntityData.java:101)
- [x] `LegacyEntityData.EFFECT_IDS` (com/lion/graveyard/entities/LegacyEntityData.java:102)
- [x] `LegacyEntityData.EFFECT_RENAMES` (com/lion/graveyard/entities/LegacyEntityData.java:137)
- [x] `LegacyEntityData.LegacyEntityData(0)` (com/lion/graveyard/entities/LegacyEntityData.java:141)
- [ ] `LegacyEntityData.write(1)`: it uses Minecraft's `ValueOutput` (com/lion/graveyard/entities/LegacyEntityData.java:144)
- [ ] `LegacyEntityData.read(2)`: it uses Minecraft's `ValueInput` (com/lion/graveyard/entities/LegacyEntityData.java:148)
- [ ] `LegacyEntityData.readLegacyAngerTarget(3)`: it's generic (com/lion/graveyard/entities/LegacyEntityData.java:165)
- [ ] `LegacyEntityData.storedCustomName(2)`: it uses Minecraft's `Provider` (com/lion/graveyard/entities/LegacyEntityData.java:177)
- [ ] `LegacyEntityData.parseJson(2)`: it uses Minecraft's `Provider` (com/lion/graveyard/entities/LegacyEntityData.java:189)
- [ ] `LegacyEntityData.readLeash(2)`: it uses Minecraft's `ValueInput` (com/lion/graveyard/entities/LegacyEntityData.java:201)
- [ ] `LegacyEntityData.convertLeash(1)`: it uses Minecraft's `Tag` (com/lion/graveyard/entities/LegacyEntityData.java:210)
- [ ] `LegacyEntityData.readCustomName(2)`: it uses Minecraft's `ValueInput` (com/lion/graveyard/entities/LegacyEntityData.java:225)
- [ ] `LegacyEntityData.readEquipment(2)`: it uses Minecraft's `ValueInput` (com/lion/graveyard/entities/LegacyEntityData.java:234)
- [ ] `LegacyEntityData.applyStacks(5)`: it uses Minecraft's `ValueInput` (com/lion/graveyard/entities/LegacyEntityData.java:242)
- [ ] `LegacyEntityData.readDropChances(4)`: it uses Minecraft's `ValueInput` (com/lion/graveyard/entities/LegacyEntityData.java:259)
- [ ] `LegacyEntityData.modifier(2)`: it uses `UUID` (java.util) (com/lion/graveyard/entities/LegacyEntityData.java:272)
- [ ] `LegacyEntityData.readAttributes(2)`: it uses Minecraft's `ValueInput` (com/lion/graveyard/entities/LegacyEntityData.java:276)
- [ ] `LegacyEntityData.applyAttributes(3)`: it uses Minecraft's `ValueInput` (com/lion/graveyard/entities/LegacyEntityData.java:292)
- [ ] `LegacyEntityData.withUnprefixedIds(1)`: it uses Minecraft's `ListTag` (com/lion/graveyard/entities/LegacyEntityData.java:311)
- [ ] `LegacyEntityData.convertAttributes(1)`: it uses Minecraft's `ListTag` (com/lion/graveyard/entities/LegacyEntityData.java:330)
- [x] `LegacyEntityData.attributeId(1)` (com/lion/graveyard/entities/LegacyEntityData.java:350)
- [x] `LegacyEntityData.ensureNamespaced(1)` (com/lion/graveyard/entities/LegacyEntityData.java:366)
- [ ] `LegacyEntityData.convertModifiers(1)`: it uses Minecraft's `ListTag` (com/lion/graveyard/entities/LegacyEntityData.java:370)
- [x] `LegacyEntityData.operation(1)` (com/lion/graveyard/entities/LegacyEntityData.java:405)
- [ ] `LegacyEntityData.readUuid(1)`: it uses `UUID` (java.util) (com/lion/graveyard/entities/LegacyEntityData.java:409)
- [ ] `LegacyEntityData.readEffects(2)`: it uses Minecraft's `ValueInput` (com/lion/graveyard/entities/LegacyEntityData.java:420)
- [ ] `LegacyEntityData.convertEffect(1)`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/entities/LegacyEntityData.java:433)

### `LichEntity`: it extends Minecraft's `Monster`

- [ ] `LichEntity.HEAD_TARGET_PREDICATE`: it uses Minecraft's `TargetingConditions` (com/lion/graveyard/entities/LichEntity.java:87)
- [ ] `LichEntity.CAN_ATTACK_PREDICATE`: it uses Minecraft's `Selector` (com/lion/graveyard/entities/LichEntity.java:88)
- [x] `LichEntity.ATTACKING_SPEED_BOOST_ID` (com/lion/graveyard/entities/LichEntity.java:89)
- [x] `LichEntity.ATTACKING_DMG_BOOST_ID` (com/lion/graveyard/entities/LichEntity.java:90)
- [x] `LichEntity.CRAWL_SPEED_BOOST_ID` (com/lion/graveyard/entities/LichEntity.java:91)
- [ ] `LichEntity.ATTACKING_SPEED_BOOST`: it uses Minecraft's `AttributeModifier` (com/lion/graveyard/entities/LichEntity.java:92)
- [ ] `LichEntity.CRAWL_SPEED_BOOST`: it uses Minecraft's `AttributeModifier` (com/lion/graveyard/entities/LichEntity.java:95)
- [ ] `LichEntity.DMG_BOOST`: it uses Minecraft's `AttributeModifier` (com/lion/graveyard/entities/LichEntity.java:96)
- [x] `LichEntity.ANIMATION_SPAWN` (com/lion/graveyard/entities/LichEntity.java:117)
- [x] `LichEntity.ANIMATION_IDLE` (com/lion/graveyard/entities/LichEntity.java:118)
- [x] `LichEntity.ANIMATION_MELEE` (com/lion/graveyard/entities/LichEntity.java:119)
- [x] `LichEntity.ANIMATION_CORPSE_SPELL` (com/lion/graveyard/entities/LichEntity.java:120)
- [x] `LichEntity.ANIMATION_START_PHASE_2` (com/lion/graveyard/entities/LichEntity.java:121)
- [x] `LichEntity.ANIMATION_PHASE_2_IDLE` (com/lion/graveyard/entities/LichEntity.java:122)
- [x] `LichEntity.ANIMATION_PHASE_2_ATTACK` (com/lion/graveyard/entities/LichEntity.java:123)
- [x] `LichEntity.ANIMATION_SHOOT_SKULL` (com/lion/graveyard/entities/LichEntity.java:124)
- [x] `LichEntity.ANIMATION_START_PHASE_3` (com/lion/graveyard/entities/LichEntity.java:125)
- [x] `LichEntity.ANIMATION_PHASE_3_ATTACK` (com/lion/graveyard/entities/LichEntity.java:126)
- [x] `LichEntity.ANIMATION_STOP` (com/lion/graveyard/entities/LichEntity.java:127)
- [x] `LichEntity.ANIMATION_SUMMON` (com/lion/graveyard/entities/LichEntity.java:128)
- [x] `LichEntity.ANIMATION_CONJURE_FANG` (com/lion/graveyard/entities/LichEntity.java:129)
- [x] `LichEntity.ANIMATION_STUNNED` (com/lion/graveyard/entities/LichEntity.java:130)
- [x] `LichEntity.ANIMATION_CRAWL_IDLE` (com/lion/graveyard/entities/LichEntity.java:131)
- [x] `LichEntity.ANIMATION_PHASE_2_CHARGE` (com/lion/graveyard/entities/LichEntity.java:132)
- [x] `LichEntity.ANIMATION_CRAWL_POUNCE` (com/lion/graveyard/entities/LichEntity.java:133)
- [x] `LichEntity.ANIMATION_CRAWL_ATTACK` (com/lion/graveyard/entities/LichEntity.java:134)
- [ ] `LichEntity.INVUL_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:135)
- [ ] `LichEntity.SPAWN_DURATION`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:136)
- [ ] `LichEntity.PHASE_INVUL_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:137)
- [ ] `LichEntity.ATTACK_ANIM_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:138)
- [ ] `LichEntity.FIGHT_DURATION_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:139)
- [ ] `LichEntity.PHASE_TWO_START_ANIM_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:140)
- [ ] `LichEntity.PHASE_THREE_START_ANIM_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:141)
- [ ] `LichEntity.PHASE`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:142)
- [ ] `LichEntity.ANIMATION`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:143)
- [ ] `LichEntity.HUNT_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:144)
- [ ] `LichEntity.CAN_HUNT_START`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:145)
- [ ] `LichEntity.CAN_MOVE`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:146)
- [ ] `LichEntity.MUSIC_DELAY`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:147)
- [ ] `LichEntity.CONJURE_FANG_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:148)
- [ ] `LichEntity.HEAL_DURATION_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:149)
- [ ] `LichEntity.CORPSE_SPELL_DURATION_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:150)
- [ ] `LichEntity.LEVITATION_DURATION_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/LichEntity.java:151)
- [x] `LichEntity.MUSIC_PLAY_ID` (com/lion/graveyard/entities/LichEntity.java:152)
- [x] `LichEntity.MUSIC_STOP_ID` (com/lion/graveyard/entities/LichEntity.java:153)
- [x] `LichEntity.SPAWN_CLIP_TICKS` (com/lion/graveyard/entities/LichEntity.java:154)
- [ ] `LichEntity.CRAWL_DIMENSIONS`: it uses Minecraft's `EntityDimensions` (com/lion/graveyard/entities/LichEntity.java:166)
- [x] `LichEntity.DEATH_REMOVAL_TICKS` (com/lion/graveyard/entities/LichEntity.java:181)
- [x] `LichEntity.SUMMON_OFFSETS` (com/lion/graveyard/entities/LichEntity.java:186)
- [x] `LichEntity.HELIX_POINT_SPACING` (com/lion/graveyard/entities/LichEntity.java:187)
- [x] `LichEntity.HELIX_SWIRL_SPEED` (com/lion/graveyard/entities/LichEntity.java:188)
- [x] `LichEntity.HELIX_FLAME_RISE` (com/lion/graveyard/entities/LichEntity.java:189)
- [x] `LichEntity.HELIX_SOUL_RISE` (com/lion/graveyard/entities/LichEntity.java:190)
- [x] `LichEntity.HELIX_POINTS` (com/lion/graveyard/entities/LichEntity.java:191)
- [x] `LichEntity.HELIX_SPIN_PER_TICK` (com/lion/graveyard/entities/LichEntity.java:192)
- [x] `LichEntity.HELIX_TURNS` (com/lion/graveyard/entities/LichEntity.java:193)
- [x] `LichEntity.HELIX_RADIUS` (com/lion/graveyard/entities/LichEntity.java:194)
- [x] `LichEntity.HELIX_HEIGHT` (com/lion/graveyard/entities/LichEntity.java:195)
- [x] `LichEntity.CRAWL_STRIKE_TICKS` (com/lion/graveyard/entities/LichEntity.java:196)
- [x] `LichEntity.CRAWL_STRIKE_HIT_TICK` (com/lion/graveyard/entities/LichEntity.java:197)
- [x] `LichEntity.PHASE_TWO_STRIKE_TICKS` (com/lion/graveyard/entities/LichEntity.java:198)
- [x] `LichEntity.PHASE_TWO_STRIKE_HIT_TICK` (com/lion/graveyard/entities/LichEntity.java:199)
- [x] `LichEntity.POUNCE_AIR_FACTOR` (com/lion/graveyard/entities/LichEntity.java:200)
- [x] `LichEntity.CHARGE_TICKS` (com/lion/graveyard/entities/LichEntity.java:201)
- [x] `LichEntity.CHARGE_LUNGE_TICK` (com/lion/graveyard/entities/LichEntity.java:202)
- [x] `LichEntity.CHARGE_LUNGE_END_TICK` (com/lion/graveyard/entities/LichEntity.java:203)
- [x] `LichEntity.CHARGE_HIT_TICK` (com/lion/graveyard/entities/LichEntity.java:204)
- [x] `LichEntity.POUNCE_TICKS` (com/lion/graveyard/entities/LichEntity.java:205)
- [x] `LichEntity.POUNCE_LEAP_TICK` (com/lion/graveyard/entities/LichEntity.java:206)
- [ ] `LichEntity.champion(0)`: it stores a second reference to an object (Rust would need it copied or shared) (com/lion/graveyard/entities/LichEntity.java:208)
- [ ] `LichEntity.spawnInvulTicks(0)`: it calls LichEntity.champion, which isn't translated (com/lion/graveyard/entities/LichEntity.java:212)
- [ ] `LichEntity.phaseInvulTicks(0)`: it calls LichEntity.champion, which isn't translated (com/lion/graveyard/entities/LichEntity.java:216)
- [ ] `LichEntity.LichEntity(2)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:220)
- [ ] `LichEntity.registerControllers(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:230)
- [ ] `LichEntity.blendTicks(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:264)
- [ ] `LichEntity.isSpawning(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:274)
- [ ] `LichEntity.getAnimatableInstanceCache(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:278)
- [ ] `LichEntity.registerGoals(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:282)
- [ ] `LichEntity.createLichAttributes(0)`: it uses Minecraft's `Builder` (com/lion/graveyard/entities/LichEntity.java:296)
- [ ] `LichEntity.tick(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:307)
- [ ] `LichEntity.restartAttackClipOnRearm(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:322)
- [ ] `LichEntity.handleEntityEvent(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:337)
- [ ] `LichEntity.aiStep(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:347)
- [ ] `LichEntity.spawnHealHelix(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:480)
- [ ] `LichEntity.spawnTransitionAndDeathParticles(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:526)
- [ ] `LichEntity.spawnInHitbox(2)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:581)
- [ ] `LichEntity.spawnOutwardBurst(3)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:594)
- [ ] `LichEntity.baseTick(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:605)
- [ ] `LichEntity.customServerAiStep(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:613)
- [x] `LichEntity.isBoneBreakTick(1)` (com/lion/graveyard/entities/LichEntity.java:859)
- [ ] `LichEntity.enterStunned(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:863)
- [ ] `LichEntity.tickCharge(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:871)
- [ ] `LichEntity.startCrawlStrike(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:923)
- [ ] `LichEntity.tickCrawlStrike(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:932)
- [ ] `LichEntity.isCharging(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:949)
- [ ] `LichEntity.startPhaseTwoStrike(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:953)
- [ ] `LichEntity.tickPhaseTwoStrike(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:962)
- [ ] `LichEntity.stopRush(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:979)
- [ ] `LichEntity.endCharge(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:987)
- [ ] `LichEntity.tickPounce(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:995)
- [ ] `LichEntity.isPouncing(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1054)
- [ ] `LichEntity.getMaxHealthPerPhase(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1058)
- [ ] `LichEntity.setCustomName(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1062)
- [ ] `LichEntity.releaseAltar(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1067)
- [ ] `LichEntity.remove(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1076)
- [ ] `LichEntity.getHomePos(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1084)
- [ ] `LichEntity.isHomeKnown(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1088)
- [ ] `LichEntity.onSummoned(2)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1092)
- [ ] `LichEntity.applyInvulAndResetBossBar(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1102)
- [ ] `LichEntity.tickDeath(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1107)
- [ ] `LichEntity.getDefaultDimensions(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1119)
- [ ] `LichEntity.causeFallDamage(3)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1123)
- [ ] `LichEntity.addEffect(2)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1127)
- [ ] `LichEntity.canRide(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1131)
- [ ] `LichEntity.canTeleport(2)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1135)
- [ ] `LichEntity.startSeenByPlayer(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1139)
- [ ] `LichEntity.stopSeenByPlayer(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1146)
- [ ] `LichEntity.hurtServer(3)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1151)
- [ ] `LichEntity.respawn(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1177)
- [ ] `LichEntity.randomDisplayTick(3)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1189)
- [ ] `LichEntity.getPlayersInRange(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1213)
- [ ] `LichEntity.canMeeleAttack(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1218)
- [ ] `LichEntity.canSpellCast(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1229)
- [ ] `LichEntity.ignoreExplosion(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1240)
- [ ] `LichEntity.isSummonCell(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1244)
- [ ] `LichEntity.summonCell(2)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1252)
- [ ] `LichEntity.summonMob(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1258)
- [ ] `LichEntity.requiresCustomPersistence(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1299)
- [ ] `LichEntity.canHuntStart(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1303)
- [ ] `LichEntity.setHuntStart(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1307)
- [ ] `LichEntity.setCanMove(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1311)
- [ ] `LichEntity.canMove(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1315)
- [ ] `LichEntity.getInvulnerableTimer(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1319)
- [ ] `LichEntity.setInvulTimer(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1323)
- [ ] `LichEntity.getFightDurationTimer(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1327)
- [ ] `LichEntity.setFightDurationTimer(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1331)
- [ ] `LichEntity.getLevitationDurationTimer(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1335)
- [ ] `LichEntity.setLevitationDurationTimer(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1339)
- [ ] `LichEntity.getConjureFangTimer(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1343)
- [ ] `LichEntity.setConjureFangTimer(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1347)
- [ ] `LichEntity.getSpawnDuration(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1351)
- [ ] `LichEntity.getCorpseSpellTimer(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1356)
- [ ] `LichEntity.setCorpseSpellTimer(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1360)
- [ ] `LichEntity.getHealTimer(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1364)
- [ ] `LichEntity.setHealTimer(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1368)
- [ ] `LichEntity.getPhaseInvulnerableTimer(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1372)
- [ ] `LichEntity.setPhaseInvulTimer(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1376)
- [ ] `LichEntity.getHuntTimer(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1380)
- [ ] `LichEntity.setHuntTimer(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1384)
- [ ] `LichEntity.getAnimationState(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1388)
- [ ] `LichEntity.setAnimationState(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1392)
- [ ] `LichEntity.getPhase(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1396)
- [ ] `LichEntity.setPhase(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1400)
- [ ] `LichEntity.getStartPhaseTwoAnimTimer(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1404)
- [ ] `LichEntity.setStartPhaseTwoAnimTimer(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1408)
- [ ] `LichEntity.getStartPhaseThreeAnimTimer(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1412)
- [ ] `LichEntity.setStartPhaseThreeAnimTimer(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1416)
- [ ] `LichEntity.getAttackAnimTimer(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1420)
- [ ] `LichEntity.setAttackAnimTimer(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1424)
- [ ] `LichEntity.getHuntCooldownTicker(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1428)
- [ ] `LichEntity.setHuntCooldownTicker(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1432)
- [ ] `LichEntity.getMusicDelay(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1436)
- [ ] `LichEntity.setMusicDelay(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1440)
- [ ] `LichEntity.openFight(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1444)
- [ ] `LichEntity.isAtHome(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1466)
- [ ] `LichEntity.awakenBurst(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1470)
- [ ] `LichEntity.playSpawnSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1478)
- [ ] `LichEntity.playMeleeSwingSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1482)
- [ ] `LichEntity.playHealSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1486)
- [ ] `LichEntity.playCorpseSpellSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1490)
- [ ] `LichEntity.playStartPhaseTwoSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1494)
- [ ] `LichEntity.playStartPhaseThreeSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1498)
- [ ] `LichEntity.playStartPhaseThreeAttackSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1502)
- [ ] `LichEntity.playDeathSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1507)
- [ ] `LichEntity.playScareSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1511)
- [ ] `LichEntity.playHuntSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1515)
- [ ] `LichEntity.playShootSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1519)
- [ ] `LichEntity.playLevitationSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1523)
- [ ] `LichEntity.playAmbientSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1527)
- [ ] `LichEntity.playHurtSound(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1531)
- [ ] `LichEntity.getHurtSound(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1538)
- [ ] `LichEntity.getBossMusic(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1542)
- [ ] `LichEntity.canPlayMusic(0)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1546)
- [ ] `LichEntity.canPlayerHearMusic(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1550)
- [ ] `LichEntity.defineSynchedData(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1554)
- [ ] `LichEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1575)
- [ ] `LichEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/entities/LichEntity.java:1601)

### `LichEntity.ConjureFangsGoal`: it extends Minecraft's `Goal`

- [ ] `LichEntity.ConjureFangsGoal.ConjureFangsGoal(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1650)
- [ ] `LichEntity.ConjureFangsGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1654)
- [ ] `LichEntity.ConjureFangsGoal.requiresUpdateEveryTick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1664)
- [ ] `LichEntity.ConjureFangsGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1668)
- [ ] `LichEntity.ConjureFangsGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1676)
- [ ] `LichEntity.ConjureFangsGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1680)
- [ ] `LichEntity.ConjureFangsGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1701)
- [ ] `LichEntity.ConjureFangsGoal.conjureFangs(6)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1710)

### `LichEntity.LevitationGoal`: it extends Minecraft's `Goal`

- [ ] `LichEntity.LevitationGoal.LevitationGoal(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1743)
- [ ] `LichEntity.LevitationGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1747)
- [ ] `LichEntity.LevitationGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1756)
- [ ] `LichEntity.LevitationGoal.requiresUpdateEveryTick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1771)
- [ ] `LichEntity.LevitationGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1775)
- [ ] `LichEntity.LevitationGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1783)
- [ ] `LichEntity.LevitationGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1787)

### `LichEntity.ShootSkullGoal`: it extends Minecraft's `Goal`

- [ ] `LichEntity.ShootSkullGoal.ShootSkullGoal(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1801)
- [ ] `LichEntity.ShootSkullGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1805)
- [ ] `LichEntity.ShootSkullGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1809)
- [ ] `LichEntity.ShootSkullGoal.requiresUpdateEveryTick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1813)
- [ ] `LichEntity.ShootSkullGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1817)
- [ ] `LichEntity.ShootSkullGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1825)

### `LichEntity.SummonFallenCorpsesGoal`: it extends Minecraft's `Goal`

- [ ] `LichEntity.SummonFallenCorpsesGoal.SummonFallenCorpsesGoal(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1882)
- [ ] `LichEntity.SummonFallenCorpsesGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1886)
- [ ] `LichEntity.SummonFallenCorpsesGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1894)
- [ ] `LichEntity.SummonFallenCorpsesGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1940)
- [ ] `LichEntity.SummonFallenCorpsesGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1948)
- [ ] `LichEntity.SummonFallenCorpsesGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1952)

### `LichEntity.TeleportAndHealGoal`: it extends Minecraft's `Goal`

- [ ] `LichEntity.TeleportAndHealGoal.TeleportAndHealGoal(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1990)
- [ ] `LichEntity.TeleportAndHealGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:1994)
- [ ] `LichEntity.TeleportAndHealGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:2003)
- [ ] `LichEntity.TeleportAndHealGoal.requiresUpdateEveryTick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:2015)
- [ ] `LichEntity.TeleportAndHealGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:2019)
- [ ] `LichEntity.TeleportAndHealGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:2027)
- [ ] `LichEntity.TeleportAndHealGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/LichEntity.java:2031)

### `MagpieEntity`: it extends `CorvidEntity`

- [ ] `MagpieEntity.MagpieEntity(2)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/MagpieEntity.java:29)
- [ ] `MagpieEntity.getSpecies(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/MagpieEntity.java:34)
- [ ] `MagpieEntity.registerGoals(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/MagpieEntity.java:39)
- [ ] `MagpieEntity.isHopping(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/MagpieEntity.java:52)
- [ ] `MagpieEntity.setHopping(1)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/MagpieEntity.java:57)
- [ ] `MagpieEntity.canWalk(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/MagpieEntity.java:61)
- [ ] `MagpieEntity.isWalking(0)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/MagpieEntity.java:66)
- [ ] `MagpieEntity.setWalking(1)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/MagpieEntity.java:71)
- [ ] `MagpieEntity.wantsToPickUp(2)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/MagpieEntity.java:75)
- [ ] `MagpieEntity.pickUpItem(2)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/MagpieEntity.java:79)
- [ ] `MagpieEntity.hurtServer(3)`: its class isn't translated (it extends `CorvidEntity`) (com/lion/graveyard/entities/MagpieEntity.java:97)

### `MobSpawnGate`

- [x] `MobSpawnGate.MANAGED_MOBS` (com/lion/graveyard/entities/MobSpawnGate.java:19)
- [ ] `MobSpawnGate.AUTOMATIC`: it uses Minecraft's `EntitySpawnReason` (com/lion/graveyard/entities/MobSpawnGate.java:36)
- [ ] `MobSpawnGate.NO_MOB_SPAWNS`: it uses Minecraft's `TagKey` (com/lion/graveyard/entities/MobSpawnGate.java:45)
- [x] `MobSpawnGate.MobSpawnGate(0)` (com/lion/graveyard/entities/MobSpawnGate.java:47)
- [ ] `MobSpawnGate.inForbiddenBiome(4)`: it uses Minecraft's `LevelAccessor` (com/lion/graveyard/entities/MobSpawnGate.java:50)
- [ ] `MobSpawnGate.blocks(2)`: it uses Minecraft's `EntityType` (com/lion/graveyard/entities/MobSpawnGate.java:56)
- [ ] `MobSpawnGate.isSwitchedOff(1)`: it uses Minecraft's `EntityType` (com/lion/graveyard/entities/MobSpawnGate.java:60)

### `BaseGhoulModel`: it extends `GeoModel` (com.geckolib.model)

- [ ] `BaseGhoulModel.getModelResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/BaseGhoulModel.java:12)
- [ ] `BaseGhoulModel.getTextureResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/BaseGhoulModel.java:16)
- [ ] `BaseGhoulModel.addAdditionalStateData(3)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/BaseGhoulModel.java:20)
- [ ] `BaseGhoulModel.getAnimationResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/BaseGhoulModel.java:24)

### `CorruptedIllagerGeoModel`: it's generic

- [ ] `CorruptedIllagerGeoModel.getModelResource(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/models/CorruptedIllagerGeoModel.java:13)
- [ ] `CorruptedIllagerGeoModel.getTextureResource(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/models/CorruptedIllagerGeoModel.java:17)
- [ ] `CorruptedIllagerGeoModel.addAdditionalStateData(3)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/models/CorruptedIllagerGeoModel.java:21)
- [ ] `CorruptedIllagerGeoModel.getAnimationResource(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/models/CorruptedIllagerGeoModel.java:26)

### `CorruptedIllagerModel`: it extends Minecraft's `EntityModel`

- [ ] `CorruptedIllagerModel.CorruptedIllagerModel(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (com/lion/graveyard/entities/models/CorruptedIllagerModel.java:34)
- [ ] `CorruptedIllagerModel.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (com/lion/graveyard/entities/models/CorruptedIllagerModel.java:48)
- [ ] `CorruptedIllagerModel.setupAnim(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (com/lion/graveyard/entities/models/CorruptedIllagerModel.java:98)
- [ ] `CorruptedIllagerModel.getAttackingArm(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (com/lion/graveyard/entities/models/CorruptedIllagerModel.java:210)
- [ ] `CorruptedIllagerModel.getHat(0)`: its class isn't translated (it extends Minecraft's `EntityModel`) (com/lion/graveyard/entities/models/CorruptedIllagerModel.java:214)
- [ ] `CorruptedIllagerModel.getHead(0)`: its class isn't translated (it extends Minecraft's `EntityModel`) (com/lion/graveyard/entities/models/CorruptedIllagerModel.java:218)
- [ ] `CorruptedIllagerModel.translateToHand(3)`: its class isn't translated (it extends Minecraft's `EntityModel`) (com/lion/graveyard/entities/models/CorruptedIllagerModel.java:222)

### `CorvidModel`: it extends `GeoModel` (com.geckolib.model)

- [x] `CorvidModel.ANIMATION` (com/lion/graveyard/entities/models/CorvidModel.java:16)
- [ ] `CorvidModel.getModelResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/CorvidModel.java:18)
- [ ] `CorvidModel.getTextureResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/CorvidModel.java:22)
- [ ] `CorvidModel.getAnimationResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/CorvidModel.java:26)
- [ ] `CorvidModel.addAdditionalStateData(3)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/CorvidModel.java:30)
- [ ] `CorvidModel.isPlayingPeck(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/CorvidModel.java:37)

### `FallingCorpseModel`: it extends `GeoModel` (com.geckolib.model)

- [ ] `FallingCorpseModel.getModelResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/FallingCorpseModel.java:10)
- [ ] `FallingCorpseModel.getTextureResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/FallingCorpseModel.java:14)
- [ ] `FallingCorpseModel.getAnimationResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/FallingCorpseModel.java:18)

### `GhoulingModel`: it extends `GeoModel` (com.geckolib.model)

- [ ] `GhoulingModel.getModelResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/GhoulingModel.java:12)
- [ ] `GhoulingModel.getTextureResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/GhoulingModel.java:16)
- [ ] `GhoulingModel.addAdditionalStateData(3)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/GhoulingModel.java:20)
- [x] `GhoulingModel.textureVariant(1)` (com/lion/graveyard/entities/models/GhoulingModel.java:24)
- [ ] `GhoulingModel.getAnimationResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/GhoulingModel.java:28)

### `LichModel`: it extends `GeoModel` (com.geckolib.model)

- [ ] `LichModel.getModelResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/LichModel.java:14)
- [ ] `LichModel.getTextureResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/LichModel.java:18)
- [ ] `LichModel.getAnimationResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/LichModel.java:22)
- [ ] `LichModel.addAdditionalStateData(3)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/LichModel.java:26)

### `NamelessHangedModel`: it extends `GeoModel` (com.geckolib.model)

- [ ] `NamelessHangedModel.getModelResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/NamelessHangedModel.java:10)
- [ ] `NamelessHangedModel.getTextureResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/NamelessHangedModel.java:14)
- [ ] `NamelessHangedModel.getAnimationResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/NamelessHangedModel.java:18)

### `NightmareModel`: it extends `GeoModel` (com.geckolib.model)

- [ ] `NightmareModel.getModelResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/NightmareModel.java:10)
- [ ] `NightmareModel.getTextureResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/NightmareModel.java:14)
- [ ] `NightmareModel.getAnimationResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/NightmareModel.java:18)

### `ReaperModel`: it extends `GeoModel` (com.geckolib.model)

- [ ] `ReaperModel.getModelResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/ReaperModel.java:10)
- [ ] `ReaperModel.getTextureResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/ReaperModel.java:14)
- [ ] `ReaperModel.getAnimationResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/ReaperModel.java:18)

### `RevenantModel`: it extends `GeoModel` (com.geckolib.model)

- [ ] `RevenantModel.getModelResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/RevenantModel.java:10)
- [ ] `RevenantModel.getTextureResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/RevenantModel.java:14)
- [ ] `RevenantModel.getAnimationResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/RevenantModel.java:18)

### `TormentedVillagerModel`: it extends `GeoModel` (com.geckolib.model)

- [ ] `TormentedVillagerModel.getModelResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/TormentedVillagerModel.java:10)
- [ ] `TormentedVillagerModel.getTextureResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/TormentedVillagerModel.java:14)
- [ ] `TormentedVillagerModel.getAnimationResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/TormentedVillagerModel.java:18)

### `WraithModel`: it extends `GeoModel` (com.geckolib.model)

- [ ] `WraithModel.getModelResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/WraithModel.java:10)
- [ ] `WraithModel.getTextureResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/WraithModel.java:14)
- [ ] `WraithModel.getAnimationResource(1)`: its class isn't translated (it extends `GeoModel` (com.geckolib.model)) (com/lion/graveyard/entities/models/WraithModel.java:18)

### `NamelessHangedEntity`: it extends Minecraft's `AbstractVillager`

- [x] `NamelessHangedEntity.OFFERS_DAY_KEY` (com/lion/graveyard/entities/NamelessHangedEntity.java:50)
- [ ] `NamelessHangedEntity.NamelessHangedEntity(2)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:54)
- [ ] `NamelessHangedEntity.registerGoals(0)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:58)
- [ ] `NamelessHangedEntity.createNamelessHangedAttributes(0)`: it uses Minecraft's `Builder` (com/lion/graveyard/entities/NamelessHangedEntity.java:63)
- [ ] `NamelessHangedEntity.registerControllers(1)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:67)
- [ ] `NamelessHangedEntity.getAnimatableInstanceCache(0)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:71)
- [ ] `NamelessHangedEntity.canBeSeenAsEnemy(0)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:75)
- [ ] `NamelessHangedEntity.isPushable(0)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:79)
- [ ] `NamelessHangedEntity.doPush(1)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:83)
- [ ] `NamelessHangedEntity.hurtServer(3)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:86)
- [ ] `NamelessHangedEntity.ignoreExplosion(1)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:95)
- [ ] `NamelessHangedEntity.thunderHit(2)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:99)
- [ ] `NamelessHangedEntity.isAffectedByPotions(0)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:102)
- [ ] `NamelessHangedEntity.getBreedOffspring(2)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:106)
- [ ] `NamelessHangedEntity.showProgressBar(0)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:111)
- [ ] `NamelessHangedEntity.aiStep(0)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:115)
- [ ] `NamelessHangedEntity.tick(0)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:123)
- [ ] `NamelessHangedEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:131)
- [ ] `NamelessHangedEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:137)
- [ ] `NamelessHangedEntity.currentDay(0)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:143)
- [ ] `NamelessHangedEntity.mobInteract(2)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:147)
- [ ] `NamelessHangedEntity.updateTrades(1)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:170)
- [ ] `NamelessHangedEntity.minOffers(0)`: it calls Graveyard.getConfig, which isn't translated (com/lion/graveyard/entities/NamelessHangedEntity.java:190)
- [ ] `NamelessHangedEntity.maxOffers(0)`: it calls NamelessHangedEntity.minOffers, which isn't translated (com/lion/graveyard/entities/NamelessHangedEntity.java:194)
- [ ] `NamelessHangedEntity.villagertradingplus$rerollAll(0)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:198)
- [ ] `NamelessHangedEntity.villagertradingplus$catalogPool(1)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:205)
- [ ] `NamelessHangedEntity.villagertradingplus$catalogMaxLevel(0)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:209)
- [ ] `NamelessHangedEntity.villagertradingplus$catalogPicks(1)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:213)
- [ ] `NamelessHangedEntity.rewardTradeXp(1)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:217)
- [ ] `NamelessHangedEntity.removeWhenFarAway(1)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:224)
- [ ] `NamelessHangedEntity.playAmbientSound(0)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:228)
- [ ] `NamelessHangedEntity.getNotifyTradeSound(0)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:235)
- [ ] `NamelessHangedEntity.getTradeUpdatedSound(1)`: its class isn't translated (it extends Minecraft's `AbstractVillager`) (com/lion/graveyard/entities/NamelessHangedEntity.java:239)

### `NightmareEntity`: it extends `HostileGraveyardEntity`

- [x] `NightmareEntity.ANIMATION_IDLE` (com/lion/graveyard/entities/NightmareEntity.java:65)
- [x] `NightmareEntity.ANIMATION_WALK` (com/lion/graveyard/entities/NightmareEntity.java:66)
- [x] `NightmareEntity.ANIMATION_DEATH` (com/lion/graveyard/entities/NightmareEntity.java:67)
- [x] `NightmareEntity.ANIMATION_ATTACK` (com/lion/graveyard/entities/NightmareEntity.java:68)
- [ ] `NightmareEntity.ANIMATION`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/NightmareEntity.java:69)
- [ ] `NightmareEntity.ATTACK_ANIM_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/NightmareEntity.java:70)
- [ ] `NightmareEntity.ANGRY`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/NightmareEntity.java:74)
- [ ] `NightmareEntity.PROVOKED`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/NightmareEntity.java:75)
- [ ] `NightmareEntity.ANGER_TIME`: it uses Minecraft's `UniformInt` (com/lion/graveyard/entities/NightmareEntity.java:76)
- [ ] `NightmareEntity.NightmareEntity(2)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:81)
- [ ] `NightmareEntity.defineSynchedData(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:85)
- [ ] `NightmareEntity.getAnimationState(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:94)
- [ ] `NightmareEntity.setState(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:98)
- [ ] `NightmareEntity.registerGoals(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:102)
- [ ] `NightmareEntity.tick(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:114)
- [ ] `NightmareEntity.restartAttackClipOnRearm(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:121)
- [ ] `NightmareEntity.customServerAiStep(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:136)
- [ ] `NightmareEntity.aiStep(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:157)
- [ ] `NightmareEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:171)
- [ ] `NightmareEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:177)
- [ ] `NightmareEntity.readPersistentAngerSaveData(2)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:183)
- [ ] `NightmareEntity.createNightmareAttributes(0)`: it uses Minecraft's `Builder` (com/lion/graveyard/entities/NightmareEntity.java:188)
- [ ] `NightmareEntity.tickDeath(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:198)
- [ ] `NightmareEntity.hurtServer(3)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:206)
- [ ] `NightmareEntity.getAnimation(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:214)
- [ ] `NightmareEntity.setAnimation(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:218)
- [ ] `NightmareEntity.getAttackAnimTimer(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:222)
- [ ] `NightmareEntity.setAttackAnimTimer(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:226)
- [ ] `NightmareEntity.registerControllers(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:230)
- [ ] `NightmareEntity.getAnimatableInstanceCache(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:241)
- [ ] `NightmareEntity.playAmbientSound(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:245)
- [ ] `NightmareEntity.playHurtSound(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:249)
- [ ] `NightmareEntity.die(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:253)
- [ ] `NightmareEntity.isPlayerStaring(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:258)
- [ ] `NightmareEntity.teleportTo(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:272)
- [ ] `NightmareEntity.teleportToPosition(3)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:281)
- [ ] `NightmareEntity.teleportRandomly(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:290)
- [ ] `NightmareEntity.isAngry(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:301)
- [ ] `NightmareEntity.setPersistentAngerEndTime(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:305)
- [ ] `NightmareEntity.getPersistentAngerEndTime(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:309)
- [ ] `NightmareEntity.setProvoked(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:313)
- [ ] `NightmareEntity.getPersistentAngerTarget(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:317)
- [ ] `NightmareEntity.setPersistentAngerTarget(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:322)
- [ ] `NightmareEntity.startPersistentAngerTimer(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:326)
- [ ] `NightmareEntity.setTarget(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:330)
- [ ] `NightmareEntity.createWitherRose(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/NightmareEntity.java:343)

### `NightmareEntity.ChasePlayerGoal`: it extends Minecraft's `Goal`

- [ ] `NightmareEntity.ChasePlayerGoal.ChasePlayerGoal(1)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/NightmareEntity.java:356)
- [ ] `NightmareEntity.ChasePlayerGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/NightmareEntity.java:361)
- [ ] `NightmareEntity.ChasePlayerGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/NightmareEntity.java:371)
- [ ] `NightmareEntity.ChasePlayerGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/NightmareEntity.java:375)

### `NightmareEntity.TeleportTowardsPlayerGoal`: it extends Minecraft's `NearestAttackableTargetGoal`

- [ ] `NightmareEntity.TeleportTowardsPlayerGoal.TeleportTowardsPlayerGoal(2)`: its class isn't translated (it extends Minecraft's `NearestAttackableTargetGoal`) (com/lion/graveyard/entities/NightmareEntity.java:389)
- [ ] `NightmareEntity.TeleportTowardsPlayerGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `NearestAttackableTargetGoal`) (com/lion/graveyard/entities/NightmareEntity.java:397)
- [ ] `NightmareEntity.TeleportTowardsPlayerGoal.start(0)`: its class isn't translated (it extends Minecraft's `NearestAttackableTargetGoal`) (com/lion/graveyard/entities/NightmareEntity.java:402)
- [ ] `NightmareEntity.TeleportTowardsPlayerGoal.stop(0)`: its class isn't translated (it extends Minecraft's `NearestAttackableTargetGoal`) (com/lion/graveyard/entities/NightmareEntity.java:408)
- [ ] `NightmareEntity.TeleportTowardsPlayerGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `NearestAttackableTargetGoal`) (com/lion/graveyard/entities/NightmareEntity.java:413)
- [ ] `NightmareEntity.TeleportTowardsPlayerGoal.tick(0)`: its class isn't translated (it extends Minecraft's `NearestAttackableTargetGoal`) (com/lion/graveyard/entities/NightmareEntity.java:428)

### `SkullEntity`: it extends Minecraft's `AbstractHurtingProjectile`

- [ ] `SkullEntity.CHARGED`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/projectiles/SkullEntity.java:32)
- [x] `SkullEntity.INITIAL_SPEED` (com/lion/graveyard/entities/projectiles/SkullEntity.java:33)
- [ ] `SkullEntity.SkullEntity(2)`: its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`) (com/lion/graveyard/entities/projectiles/SkullEntity.java:35)
- [ ] `SkullEntity.SkullEntity(5)`: its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`) (com/lion/graveyard/entities/projectiles/SkullEntity.java:39)
- [ ] `SkullEntity.getInertia(0)`: its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`) (com/lion/graveyard/entities/projectiles/SkullEntity.java:44)
- [ ] `SkullEntity.getLightLevelDependentMagicValue(0)`: its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`) (com/lion/graveyard/entities/projectiles/SkullEntity.java:48)
- [ ] `SkullEntity.isOnFire(0)`: its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`) (com/lion/graveyard/entities/projectiles/SkullEntity.java:52)
- [ ] `SkullEntity.getBlockExplosionResistance(6)`: its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`) (com/lion/graveyard/entities/projectiles/SkullEntity.java:56)
- [ ] `SkullEntity.onHitEntity(1)`: its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`) (com/lion/graveyard/entities/projectiles/SkullEntity.java:60)
- [ ] `SkullEntity.tick(0)`: its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`) (com/lion/graveyard/entities/projectiles/SkullEntity.java:73)
- [ ] `SkullEntity.onHit(1)`: its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`) (com/lion/graveyard/entities/projectiles/SkullEntity.java:80)
- [ ] `SkullEntity.isPickable(0)`: its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`) (com/lion/graveyard/entities/projectiles/SkullEntity.java:92)
- [ ] `SkullEntity.hurtServer(3)`: its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`) (com/lion/graveyard/entities/projectiles/SkullEntity.java:96)
- [ ] `SkullEntity.defineSynchedData(1)`: its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`) (com/lion/graveyard/entities/projectiles/SkullEntity.java:100)
- [ ] `SkullEntity.isCharged(0)`: its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`) (com/lion/graveyard/entities/projectiles/SkullEntity.java:105)
- [ ] `SkullEntity.setCharged(1)`: its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`) (com/lion/graveyard/entities/projectiles/SkullEntity.java:109)
- [ ] `SkullEntity.shouldBurn(0)`: its class isn't translated (it extends Minecraft's `AbstractHurtingProjectile`) (com/lion/graveyard/entities/projectiles/SkullEntity.java:113)

### `ReaperEntity`: it extends `HostileGraveyardEntity`

- [ ] `ReaperEntity.ANIMATION`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/ReaperEntity.java:63)
- [ ] `ReaperEntity.VEX_FLAGS`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/ReaperEntity.java:64)
- [x] `ReaperEntity.CHARGING_FLAG` (com/lion/graveyard/entities/ReaperEntity.java:65)
- [x] `ReaperEntity.SPAWN_TICKS` (com/lion/graveyard/entities/ReaperEntity.java:66)
- [x] `ReaperEntity.ATTACK_TICKS` (com/lion/graveyard/entities/ReaperEntity.java:67)
- [x] `ReaperEntity.DEATH_TICKS` (com/lion/graveyard/entities/ReaperEntity.java:68)
- [ ] `ReaperEntity.SPAWN_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/ReaperEntity.java:69)
- [ ] `ReaperEntity.ATTACK_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/ReaperEntity.java:70)
- [ ] `ReaperEntity.ReaperEntity(2)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:75)
- [ ] `ReaperEntity.defineSynchedData(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:80)
- [ ] `ReaperEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:89)
- [ ] `ReaperEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:97)
- [ ] `ReaperEntity.registerGoals(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:107)
- [ ] `ReaperEntity.tick(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:119)
- [ ] `ReaperEntity.finalizeSpawn(4)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:131)
- [ ] `ReaperEntity.tickDown(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:139)
- [ ] `ReaperEntity.isSpawning(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:146)
- [ ] `ReaperEntity.doHurtTarget(2)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:150)
- [ ] `ReaperEntity.tickDeath(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:158)
- [ ] `ReaperEntity.getBounds(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:166)
- [ ] `ReaperEntity.areFlagsSet(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:171)
- [ ] `ReaperEntity.setVexFlag(2)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:176)
- [ ] `ReaperEntity.isCharging(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:187)
- [ ] `ReaperEntity.setCharging(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:191)
- [ ] `ReaperEntity.createReaperAttributes(0)`: it uses Minecraft's `Builder` (com/lion/graveyard/entities/ReaperEntity.java:195)
- [ ] `ReaperEntity.getAnimation(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:199)
- [ ] `ReaperEntity.setAnimation(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:203)
- [ ] `ReaperEntity.registerControllers(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:207)
- [ ] `ReaperEntity.getAnimatableInstanceCache(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:226)
- [ ] `ReaperEntity.playAmbientSound(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:230)
- [ ] `ReaperEntity.playHurtSound(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:234)
- [ ] `ReaperEntity.die(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/ReaperEntity.java:238)

### `ReaperEntity.ChargeTargetGoal`: it extends Minecraft's `Goal`

- [ ] `ReaperEntity.ChargeTargetGoal.ChargeTargetGoal(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ReaperEntity.java:244)
- [ ] `ReaperEntity.ChargeTargetGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ReaperEntity.java:248)
- [ ] `ReaperEntity.ChargeTargetGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ReaperEntity.java:254)
- [ ] `ReaperEntity.ChargeTargetGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ReaperEntity.java:261)
- [ ] `ReaperEntity.ChargeTargetGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ReaperEntity.java:269)
- [ ] `ReaperEntity.ChargeTargetGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ReaperEntity.java:273)

### `ReaperEntity.LookAtTargetGoal`: it extends Minecraft's `Goal`

- [ ] `ReaperEntity.LookAtTargetGoal.LookAtTargetGoal(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ReaperEntity.java:289)
- [ ] `ReaperEntity.LookAtTargetGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ReaperEntity.java:293)
- [ ] `ReaperEntity.LookAtTargetGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ReaperEntity.java:297)
- [ ] `ReaperEntity.LookAtTargetGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/ReaperEntity.java:301)

### `ReaperEntity.ReaperMoveControl`: it extends Minecraft's `MoveControl`

- [ ] `ReaperEntity.ReaperMoveControl.ReaperMoveControl(1)`: its class isn't translated (it extends Minecraft's `MoveControl`) (com/lion/graveyard/entities/ReaperEntity.java:323)
- [ ] `ReaperEntity.ReaperMoveControl.tick(0)`: its class isn't translated (it extends Minecraft's `MoveControl`) (com/lion/graveyard/entities/ReaperEntity.java:327)

### `ReaperEntity.TrackOwnerTargetGoal`: it extends Minecraft's `TargetGoal`

- [ ] `ReaperEntity.TrackOwnerTargetGoal.TrackOwnerTargetGoal(1)`: its class isn't translated (it extends Minecraft's `TargetGoal`) (com/lion/graveyard/entities/ReaperEntity.java:354)
- [ ] `ReaperEntity.TrackOwnerTargetGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `TargetGoal`) (com/lion/graveyard/entities/ReaperEntity.java:358)
- [ ] `ReaperEntity.TrackOwnerTargetGoal.start(0)`: its class isn't translated (it extends Minecraft's `TargetGoal`) (com/lion/graveyard/entities/ReaperEntity.java:364)

### `AcolyteRender`: it extends `CorruptedIllagerRenderer`

- [x] `AcolyteRender.TEXTURE` (com/lion/graveyard/entities/renders/AcolyteRender.java:16)
- [ ] `AcolyteRender.AcolyteRender(1)`: its body couldn't be read: an anonymous class (com/lion/graveyard/entities/renders/AcolyteRender.java:18)
- [ ] `AcolyteRender.getTextureLocation(1)`: its class isn't translated (it extends `CorruptedIllagerRenderer`) (com/lion/graveyard/entities/renders/AcolyteRender.java:31)

### `CorruptedIllagerGeoRenderer`: it's generic

- [ ] `CorruptedIllagerGeoRenderer.CorruptedIllagerGeoRenderer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/CorruptedIllagerGeoRenderer.java:16)
- [ ] `CorruptedIllagerGeoRenderer.getDeathMaxRotation(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/CorruptedIllagerGeoRenderer.java:22)
- [ ] `CorruptedIllagerGeoRenderer.applyAnimationControllers(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/CorruptedIllagerGeoRenderer.java:26)
- [ ] `CorruptedIllagerGeoRenderer.addRenderData(4)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/CorruptedIllagerGeoRenderer.java:31)
- [ ] `CorruptedIllagerGeoRenderer.adjustModelBonesForRender(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/CorruptedIllagerGeoRenderer.java:36)

### `CorruptedIllagerRenderer`: it's generic

- [ ] `CorruptedIllagerRenderer.CorruptedIllagerRenderer(3)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/CorruptedIllagerRenderer.java:16)
- [ ] `CorruptedIllagerRenderer.createRenderState(0)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/CorruptedIllagerRenderer.java:21)
- [ ] `CorruptedIllagerRenderer.extractRenderState(3)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/CorruptedIllagerRenderer.java:25)
- [ ] `CorruptedIllagerRenderer.scale(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/CorruptedIllagerRenderer.java:38)

### `CorruptedIllagerRenderState`: it extends Minecraft's `IllagerRenderState`


### `CorvidRenderer`: it's generic

- [x] `CorvidRenderer.MAX_HEAD_PITCH` (com/lion/graveyard/entities/renders/CorvidRenderer.java:17)
- [x] `CorvidRenderer.MAX_HEAD_YAW` (com/lion/graveyard/entities/renders/CorvidRenderer.java:18)
- [ ] `CorvidRenderer.CorvidRenderer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/CorvidRenderer.java:20)
- [ ] `CorvidRenderer.getDeathMaxRotation(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/CorvidRenderer.java:26)
- [ ] `CorvidRenderer.adjustModelBonesForRender(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/CorvidRenderer.java:30)

### `FallingCorpseRenderer`: it's generic

- [ ] `FallingCorpseRenderer.FallingCorpseRenderer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/FallingCorpseRenderer.java:14)
- [ ] `FallingCorpseRenderer.addRenderData(4)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/FallingCorpseRenderer.java:19)
- [ ] `FallingCorpseRenderer.getRenderType(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/FallingCorpseRenderer.java:23)

### `AcolyteEyes`: it extends Minecraft's `EyesLayer`

- [ ] `AcolyteEyes.EYES`: it uses Minecraft's `RenderType` (com/lion/graveyard/entities/renders/features/AcolyteEyes.java:15)
- [ ] `AcolyteEyes.AcolyteEyes(1)`: its class isn't translated (it extends Minecraft's `EyesLayer`) (com/lion/graveyard/entities/renders/features/AcolyteEyes.java:17)
- [ ] `AcolyteEyes.renderType(0)`: its class isn't translated (it extends Minecraft's `EyesLayer`) (com/lion/graveyard/entities/renders/features/AcolyteEyes.java:21)

### `CorvidHeldItemLayer`: it's generic

- [ ] `CorvidHeldItemLayer.CorvidHeldItemLayer(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/features/CorvidHeldItemLayer.java:21)
- [ ] `CorvidHeldItemLayer.getRelevantBones(4)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/features/CorvidHeldItemLayer.java:25)
- [ ] `CorvidHeldItemLayer.addRenderData(4)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/features/CorvidHeldItemLayer.java:32)
- [ ] `CorvidHeldItemLayer.submitItemStackRender(7)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/features/CorvidHeldItemLayer.java:36)

### `GhoulingCoffinLayer`: it's generic

- [x] `GhoulingCoffinLayer.TORSO` (com/lion/graveyard/entities/renders/features/GhoulingCoffinLayer.java:27)
- [ ] `GhoulingCoffinLayer.GhoulingCoffinLayer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/features/GhoulingCoffinLayer.java:29)
- [ ] `GhoulingCoffinLayer.getTextureResource(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/features/GhoulingCoffinLayer.java:33)
- [ ] `GhoulingCoffinLayer.getRenderType(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/features/GhoulingCoffinLayer.java:37)
- [ ] `GhoulingCoffinLayer.preRender(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/features/GhoulingCoffinLayer.java:42)
- [ ] `GhoulingCoffinLayer.addPerBoneRender(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/features/GhoulingCoffinLayer.java:48)
- [ ] `GhoulingCoffinLayer.hasCoffin(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/features/GhoulingCoffinLayer.java:55)
- [ ] `GhoulingCoffinLayer.submitOffhandItem(3)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/features/GhoulingCoffinLayer.java:59)

### `GraveyardEyesLayer`: it's generic

- [x] `GraveyardEyesLayer.FULL_LIGHT` (com/lion/graveyard/entities/renders/features/GraveyardEyesLayer.java:17)
- [x] `GraveyardEyesLayer.OPAQUE_WHITE` (com/lion/graveyard/entities/renders/features/GraveyardEyesLayer.java:18)
- [ ] `GraveyardEyesLayer.GraveyardEyesLayer(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/features/GraveyardEyesLayer.java:21)
- [ ] `GraveyardEyesLayer.GraveyardEyesLayer(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/features/GraveyardEyesLayer.java:25)
- [ ] `GraveyardEyesLayer.getTextureResource(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/features/GraveyardEyesLayer.java:30)
- [ ] `GraveyardEyesLayer.submitRenderTask(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/features/GraveyardEyesLayer.java:34)

### `SkeletonCreeperChargeFeatureRenderer`: it extends Minecraft's `EnergySwirlLayer`

- [x] `SkeletonCreeperChargeFeatureRenderer.SKIN` (com/lion/graveyard/entities/renders/features/SkeletonCreeperChargeFeatureRenderer.java:12)
- [ ] `SkeletonCreeperChargeFeatureRenderer.SkeletonCreeperChargeFeatureRenderer(2)`: its class isn't translated (it extends Minecraft's `EnergySwirlLayer`) (com/lion/graveyard/entities/renders/features/SkeletonCreeperChargeFeatureRenderer.java:15)
- [ ] `SkeletonCreeperChargeFeatureRenderer.isPowered(1)`: its class isn't translated (it extends Minecraft's `EnergySwirlLayer`) (com/lion/graveyard/entities/renders/features/SkeletonCreeperChargeFeatureRenderer.java:20)
- [ ] `SkeletonCreeperChargeFeatureRenderer.xOffset(1)`: its class isn't translated (it extends Minecraft's `EnergySwirlLayer`) (com/lion/graveyard/entities/renders/features/SkeletonCreeperChargeFeatureRenderer.java:24)
- [ ] `SkeletonCreeperChargeFeatureRenderer.getTextureLocation(0)`: its class isn't translated (it extends Minecraft's `EnergySwirlLayer`) (com/lion/graveyard/entities/renders/features/SkeletonCreeperChargeFeatureRenderer.java:28)
- [ ] `SkeletonCreeperChargeFeatureRenderer.model(0)`: its class isn't translated (it extends Minecraft's `EnergySwirlLayer`) (com/lion/graveyard/entities/renders/features/SkeletonCreeperChargeFeatureRenderer.java:32)

### `SkeletonCreeperEyes`: it extends Minecraft's `EyesLayer`

- [ ] `SkeletonCreeperEyes.EYES`: it uses Minecraft's `RenderType` (com/lion/graveyard/entities/renders/features/SkeletonCreeperEyes.java:15)
- [ ] `SkeletonCreeperEyes.SkeletonCreeperEyes(1)`: its class isn't translated (it extends Minecraft's `EyesLayer`) (com/lion/graveyard/entities/renders/features/SkeletonCreeperEyes.java:17)
- [ ] `SkeletonCreeperEyes.renderType(0)`: its class isn't translated (it extends Minecraft's `EyesLayer`) (com/lion/graveyard/entities/renders/features/SkeletonCreeperEyes.java:21)

### `GhoulingRenderer`: it's generic

- [ ] `GhoulingRenderer.GhoulingRenderer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/GhoulingRenderer.java:17)
- [ ] `GhoulingRenderer.applyAnimationControllers(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/GhoulingRenderer.java:23)
- [ ] `GhoulingRenderer.getDeathMaxRotation(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/GhoulingRenderer.java:28)
- [ ] `GhoulingRenderer.addRenderData(4)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/GhoulingRenderer.java:32)
- [ ] `GhoulingRenderer.adjustModelBonesForRender(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/GhoulingRenderer.java:37)

### `GhoulRenderer`: it's generic

- [ ] `GhoulRenderer.GhoulRenderer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/GhoulRenderer.java:18)
- [ ] `GhoulRenderer.applyAnimationControllers(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/GhoulRenderer.java:24)
- [ ] `GhoulRenderer.getDeathMaxRotation(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/GhoulRenderer.java:29)
- [ ] `GhoulRenderer.eyesTexture(1)`: it uses `GeoRenderState` (com.geckolib.renderer.base) (com/lion/graveyard/entities/renders/GhoulRenderer.java:33)

### `LichRenderer`: it's generic

- [x] `LichRenderer.EYES` (com/lion/graveyard/entities/renders/LichRenderer.java:22)
- [x] `LichRenderer.MAX_HEAD_YAW` (com/lion/graveyard/entities/renders/LichRenderer.java:23)
- [ ] `LichRenderer.LichRenderer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/LichRenderer.java:25)
- [ ] `LichRenderer.applyAnimationControllers(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/LichRenderer.java:31)
- [ ] `LichRenderer.getDeathMaxRotation(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/LichRenderer.java:36)
- [ ] `LichRenderer.adjustModelBonesForRender(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/LichRenderer.java:40)
- [ ] `LichRenderer.getRenderType(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/LichRenderer.java:48)

### `NamelessHangedRenderer`: it's generic

- [ ] `NamelessHangedRenderer.NamelessHangedRenderer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/NamelessHangedRenderer.java:13)
- [ ] `NamelessHangedRenderer.getRenderType(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/NamelessHangedRenderer.java:18)

### `NightmareRenderer`: it's generic

- [ ] `NightmareRenderer.NightmareRenderer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/NightmareRenderer.java:11)
- [ ] `NightmareRenderer.getDeathMaxRotation(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/NightmareRenderer.java:16)

### `ReaperRenderer`: it's generic

- [x] `ReaperRenderer.EYES` (com/lion/graveyard/entities/renders/ReaperRenderer.java:17)
- [ ] `ReaperRenderer.ReaperRenderer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/ReaperRenderer.java:19)
- [ ] `ReaperRenderer.applyAnimationControllers(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/ReaperRenderer.java:25)
- [ ] `ReaperRenderer.getDeathMaxRotation(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/ReaperRenderer.java:30)

### `RevenantRenderer`: it's generic

- [x] `RevenantRenderer.EYES` (com/lion/graveyard/entities/renders/RevenantRenderer.java:14)
- [ ] `RevenantRenderer.RevenantRenderer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/RevenantRenderer.java:16)
- [ ] `RevenantRenderer.getDeathMaxRotation(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/RevenantRenderer.java:22)

### `SkeletonCreeperRender`: it extends Minecraft's `MobRenderer`

- [x] `SkeletonCreeperRender.TEXTURE` (com/lion/graveyard/entities/renders/SkeletonCreeperRender.java:20)
- [ ] `SkeletonCreeperRender.SkeletonCreeperRender(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (com/lion/graveyard/entities/renders/SkeletonCreeperRender.java:22)
- [ ] `SkeletonCreeperRender.createRenderState(0)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (com/lion/graveyard/entities/renders/SkeletonCreeperRender.java:28)
- [ ] `SkeletonCreeperRender.extractRenderState(3)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (com/lion/graveyard/entities/renders/SkeletonCreeperRender.java:32)
- [ ] `SkeletonCreeperRender.scale(2)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (com/lion/graveyard/entities/renders/SkeletonCreeperRender.java:38)
- [ ] `SkeletonCreeperRender.getWhiteOverlayProgress(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (com/lion/graveyard/entities/renders/SkeletonCreeperRender.java:49)
- [ ] `SkeletonCreeperRender.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (com/lion/graveyard/entities/renders/SkeletonCreeperRender.java:54)

### `SkullEntityRenderer`: it extends Minecraft's `EntityRenderer`

- [x] `SkullEntityRenderer.TEXTURE` (com/lion/graveyard/entities/renders/SkullEntityRenderer.java:21)
- [ ] `SkullEntityRenderer.SkullEntityRenderer(1)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (com/lion/graveyard/entities/renders/SkullEntityRenderer.java:24)
- [ ] `SkullEntityRenderer.getBlockLightLevel(2)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (com/lion/graveyard/entities/renders/SkullEntityRenderer.java:29)
- [ ] `SkullEntityRenderer.createRenderState(0)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (com/lion/graveyard/entities/renders/SkullEntityRenderer.java:33)
- [ ] `SkullEntityRenderer.extractRenderState(3)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (com/lion/graveyard/entities/renders/SkullEntityRenderer.java:37)
- [ ] `SkullEntityRenderer.submit(4)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (com/lion/graveyard/entities/renders/SkullEntityRenderer.java:44)

### `TormentedVillagerRenderer`: it's generic

- [x] `TormentedVillagerRenderer.EYES` (com/lion/graveyard/entities/renders/TormentedVillagerRenderer.java:16)
- [ ] `TormentedVillagerRenderer.TormentedVillagerRenderer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/TormentedVillagerRenderer.java:18)
- [ ] `TormentedVillagerRenderer.getDeathMaxRotation(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/TormentedVillagerRenderer.java:24)
- [ ] `TormentedVillagerRenderer.getRenderType(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/TormentedVillagerRenderer.java:28)

### `WraithRenderer`: it's generic

- [x] `WraithRenderer.EYES` (com/lion/graveyard/entities/renders/WraithRenderer.java:19)
- [ ] `WraithRenderer.WraithRenderer(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/WraithRenderer.java:21)
- [ ] `WraithRenderer.applyAnimationControllers(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/WraithRenderer.java:27)
- [ ] `WraithRenderer.getDeathMaxRotation(1)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/WraithRenderer.java:32)
- [ ] `WraithRenderer.getRenderType(2)`: its class isn't translated (it's generic) (com/lion/graveyard/entities/renders/WraithRenderer.java:36)

### `RevenantEntity`: it extends `AngerableGraveyardEntity`

- [x] `RevenantEntity.SLOWNESS_ID` (com/lion/graveyard/entities/RevenantEntity.java:49)
- [ ] `RevenantEntity.SLOWNESS_EFFECT`: it uses Minecraft's `AttributeModifier` (com/lion/graveyard/entities/RevenantEntity.java:50)
- [ ] `RevenantEntity.ATTACK_ANIM_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/RevenantEntity.java:51)
- [ ] `RevenantEntity.REANIMATE_ANIM_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/RevenantEntity.java:52)
- [ ] `RevenantEntity.ANIMATION`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/RevenantEntity.java:53)
- [ ] `RevenantEntity.CAN_REANIMATE`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/RevenantEntity.java:54)
- [ ] `RevenantEntity.CHASING`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/RevenantEntity.java:55)
- [x] `RevenantEntity.CHASE_END_TICKS` (com/lion/graveyard/entities/RevenantEntity.java:56)
- [x] `RevenantEntity.ANIMATION_IDLE` (com/lion/graveyard/entities/RevenantEntity.java:64)
- [x] `RevenantEntity.ANIMATION_WALK` (com/lion/graveyard/entities/RevenantEntity.java:65)
- [x] `RevenantEntity.ANIMATION_DEATH` (com/lion/graveyard/entities/RevenantEntity.java:66)
- [x] `RevenantEntity.ANIMATION_RUNNING` (com/lion/graveyard/entities/RevenantEntity.java:67)
- [x] `RevenantEntity.ANIMATION_MELEE` (com/lion/graveyard/entities/RevenantEntity.java:68)
- [x] `RevenantEntity.ANIMATION_REANIMATE` (com/lion/graveyard/entities/RevenantEntity.java:69)
- [x] `RevenantEntity.ANIMATION_FAKE_DEATH` (com/lion/graveyard/entities/RevenantEntity.java:70)
- [ ] `RevenantEntity.RevenantEntity(2)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:77)
- [ ] `RevenantEntity.defineSynchedData(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:81)
- [ ] `RevenantEntity.registerGoals(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:91)
- [ ] `RevenantEntity.createRevenantAttributes(0)`: it uses Minecraft's `Builder` (com/lion/graveyard/entities/RevenantEntity.java:105)
- [ ] `RevenantEntity.tick(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:114)
- [ ] `RevenantEntity.restartAttackClipOnRearm(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:121)
- [ ] `RevenantEntity.customServerAiStep(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:136)
- [ ] `RevenantEntity.trackChase(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:163)
- [ ] `RevenantEntity.aiStep(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:172)
- [ ] `RevenantEntity.tickDeath(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:187)
- [ ] `RevenantEntity.hurtServer(3)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:195)
- [ ] `RevenantEntity.getAnimatableInstanceCache(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:220)
- [ ] `RevenantEntity.registerControllers(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:224)
- [ ] `RevenantEntity.isRunning(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:239)
- [ ] `RevenantEntity.playAmbientSound(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:243)
- [ ] `RevenantEntity.playHurtSound(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:247)
- [ ] `RevenantEntity.die(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:251)
- [ ] `RevenantEntity.playStepSound(2)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:256)
- [ ] `RevenantEntity.getAnimationState(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:260)
- [ ] `RevenantEntity.setAnimationState(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:264)
- [ ] `RevenantEntity.getAttackAnimTimer(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:268)
- [ ] `RevenantEntity.setAttackAnimTimer(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:272)
- [ ] `RevenantEntity.getReanimateAnimTimer(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:276)
- [ ] `RevenantEntity.setReanimateAnimTimer(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:280)
- [ ] `RevenantEntity.canReanimate(0)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:284)
- [ ] `RevenantEntity.setCanReanimate(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:288)
- [ ] `RevenantEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:292)
- [ ] `RevenantEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends `AngerableGraveyardEntity`) (com/lion/graveyard/entities/RevenantEntity.java:299)

### `SkeletonCreeper`: it extends Minecraft's `Creeper`

- [ ] `SkeletonCreeper.SkeletonCreeper(2)`: its class isn't translated (it extends Minecraft's `Creeper`) (com/lion/graveyard/entities/SkeletonCreeper.java:29)
- [ ] `SkeletonCreeper.createSkeletonCreeperAttributes(0)`: it uses Minecraft's `Builder` (com/lion/graveyard/entities/SkeletonCreeper.java:33)
- [ ] `SkeletonCreeper.canStart(0)`: its class isn't translated (it extends Minecraft's `Creeper`) (com/lion/graveyard/entities/SkeletonCreeper.java:37)
- [ ] `SkeletonCreeper.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Creeper`) (com/lion/graveyard/entities/SkeletonCreeper.java:42)
- [ ] `SkeletonCreeper.readAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Creeper`) (com/lion/graveyard/entities/SkeletonCreeper.java:47)
- [ ] `SkeletonCreeper.dropCustomDeathLoot(3)`: its class isn't translated (it extends Minecraft's `Creeper`) (com/lion/graveyard/entities/SkeletonCreeper.java:52)
- [ ] `SkeletonCreeper.explode(0)`: its class isn't translated (it extends Minecraft's `Creeper`) (com/lion/graveyard/entities/SkeletonCreeper.java:62)
- [ ] `SkeletonCreeper.spawnEffectsCloud(0)`: its class isn't translated (it extends Minecraft's `Creeper`) (com/lion/graveyard/entities/SkeletonCreeper.java:79)
- [ ] `SkeletonCreeper.aiStep(0)`: its class isn't translated (it extends Minecraft's `Creeper`) (com/lion/graveyard/entities/SkeletonCreeper.java:97)
- [x] `SkeletonCreeper.explosionRadius(1)` (com/lion/graveyard/entities/SkeletonCreeper.java:105)

### `TormentedVillagerEntity`: it extends Minecraft's `PathfinderMob`

- [ ] `TormentedVillagerEntity.MOOD`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/TormentedVillagerEntity.java:57)
- [ ] `TormentedVillagerEntity.SPAWN_TICKS_LEFT`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/TormentedVillagerEntity.java:58)
- [x] `TormentedVillagerEntity.BELL_RANGE` (com/lion/graveyard/entities/TormentedVillagerEntity.java:59)
- [x] `TormentedVillagerEntity.REST_CHANCE` (com/lion/graveyard/entities/TormentedVillagerEntity.java:60)
- [x] `TormentedVillagerEntity.STEP_DISTANCE` (com/lion/graveyard/entities/TormentedVillagerEntity.java:61)
- [x] `TormentedVillagerEntity.STEP_VOLUME` (com/lion/graveyard/entities/TormentedVillagerEntity.java:62)
- [x] `TormentedVillagerEntity.AMBIENT_SOUND_DELAY` (com/lion/graveyard/entities/TormentedVillagerEntity.java:63)
- [x] `TormentedVillagerEntity.RAGE_SOUND_TICKS` (com/lion/graveyard/entities/TormentedVillagerEntity.java:64)
- [x] `TormentedVillagerEntity.MIN_AGE_TO_FADE` (com/lion/graveyard/entities/TormentedVillagerEntity.java:65)
- [x] `TormentedVillagerEntity.REDEMPTION_WITNESS_RANGE` (com/lion/graveyard/entities/TormentedVillagerEntity.java:66)
- [x] `TormentedVillagerEntity.DAWN_WITNESS_RANGE` (com/lion/graveyard/entities/TormentedVillagerEntity.java:67)
- [x] `TormentedVillagerEntity.LIGHT_LEVEL` (com/lion/graveyard/entities/TormentedVillagerEntity.java:68)
- [x] `TormentedVillagerEntity.REDEMPTION_EXPERIENCE` (com/lion/graveyard/entities/TormentedVillagerEntity.java:69)
- [x] `TormentedVillagerEntity.LIGHT_KEY` (com/lion/graveyard/entities/TormentedVillagerEntity.java:70)
- [x] `TormentedVillagerEntity.MOOD_TRANSITION_TICKS` (com/lion/graveyard/entities/TormentedVillagerEntity.java:71)
- [ ] `TormentedVillagerEntity.TormentedVillagerEntity(2)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:76)
- [ ] `TormentedVillagerEntity.createTormentedVillagerAttributes(0)`: it uses Minecraft's `Builder` (com/lion/graveyard/entities/TormentedVillagerEntity.java:81)
- [ ] `TormentedVillagerEntity.canSpawn(5)`: it uses Minecraft's `EntityType` (com/lion/graveyard/entities/TormentedVillagerEntity.java:85)
- [ ] `TormentedVillagerEntity.onBellRung(2)`: it uses Minecraft's `ServerLevel` (com/lion/graveyard/entities/TormentedVillagerEntity.java:91)
- [ ] `TormentedVillagerEntity.defineSynchedData(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:97)
- [ ] `TormentedVillagerEntity.registerGoals(0)`: its body couldn't be read: an anonymous class (com/lion/graveyard/entities/TormentedVillagerEntity.java:103)
- [ ] `TormentedVillagerEntity.finalizeSpawn(4)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:114)
- [ ] `TormentedVillagerEntity.getMood(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:124)
- [ ] `TormentedVillagerEntity.setMood(2)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:128)
- [ ] `TormentedVillagerEntity.customServerAiStep(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:133)
- [ ] `TormentedVillagerEntity.startRandomMood(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:172)
- [ ] `TormentedVillagerEntity.hurtServer(3)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:189)
- [ ] `TormentedVillagerEntity.redeem(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:193)
- [ ] `TormentedVillagerEntity.vanish(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:200)
- [ ] `TormentedVillagerEntity.rewardRedemption(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:215)
- [ ] `TormentedVillagerEntity.releaseLegacyLight(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:222)
- [ ] `TormentedVillagerEntity.remove(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:235)
- [ ] `TormentedVillagerEntity.restoreFrom(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:243)
- [ ] `TormentedVillagerEntity.canTeleport(2)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:252)
- [ ] `TormentedVillagerEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:256)
- [ ] `TormentedVillagerEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:265)
- [ ] `TormentedVillagerEntity.fireImmune(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:271)
- [ ] `TormentedVillagerEntity.getAmbientSound(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:275)
- [ ] `TormentedVillagerEntity.getAmbientSoundInterval(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:284)
- [ ] `TormentedVillagerEntity.nextStep(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:288)
- [ ] `TormentedVillagerEntity.playStepSound(2)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:292)
- [ ] `TormentedVillagerEntity.registerControllers(1)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:297)
- [ ] `TormentedVillagerEntity.spawnClipOffset(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:321)
- [ ] `TormentedVillagerEntity.getAnimatableInstanceCache(0)`: its class isn't translated (it extends Minecraft's `PathfinderMob`) (com/lion/graveyard/entities/TormentedVillagerEntity.java:326)

### `TormentedVillagerEntity.Mood`: its constants carry values

- [ ] `TormentedVillagerEntity.Mood.WALK`: it uses `RawAnimation` (com.geckolib.animation) (com/lion/graveyard/entities/TormentedVillagerEntity.java:339)
- [ ] `TormentedVillagerEntity.Mood.animationNames(0)`: it calls Mood.values, which isn't in the mod's source (com/lion/graveyard/entities/TormentedVillagerEntity.java:343)
- [ ] `TormentedVillagerEntity.Mood.Mood(3)`: its class isn't translated (its constants carry values) (com/lion/graveyard/entities/TormentedVillagerEntity.java:353)

### `WraithEntity`: it extends `HostileGraveyardEntity`

- [x] `WraithEntity.ATTACKING_SPEED_BOOST_ID` (com/lion/graveyard/entities/WraithEntity.java:82)
- [ ] `WraithEntity.ATTACKING_SPEED_BOOST`: it uses Minecraft's `AttributeModifier` (com/lion/graveyard/entities/WraithEntity.java:83)
- [ ] `WraithEntity.ANIMATION`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/WraithEntity.java:93)
- [ ] `WraithEntity.SPAWN_TIMER`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/WraithEntity.java:94)
- [ ] `WraithEntity.CHASING`: it uses Minecraft's `EntityDataAccessor` (com/lion/graveyard/entities/WraithEntity.java:95)
- [x] `WraithEntity.SPAWN_TICKS` (com/lion/graveyard/entities/WraithEntity.java:96)
- [x] `WraithEntity.CHASE_END_TICKS` (com/lion/graveyard/entities/WraithEntity.java:97)
- [x] `WraithEntity.DEATH_TICKS` (com/lion/graveyard/entities/WraithEntity.java:98)
- [ ] `WraithEntity.WraithEntity(2)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:104)
- [ ] `WraithEntity.move(2)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:114)
- [ ] `WraithEntity.makeStuckInBlock(2)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:119)
- [ ] `WraithEntity.registerGoals(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:125)
- [ ] `WraithEntity.createNavigation(1)`: its body couldn't be read: an anonymous class (com/lion/graveyard/entities/WraithEntity.java:137)
- [ ] `WraithEntity.hurtServer(3)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:153)
- [ ] `WraithEntity.defineSynchedData(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:164)
- [ ] `WraithEntity.startSpawnAnimation(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:172)
- [ ] `WraithEntity.tick(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:176)
- [ ] `WraithEntity.addAdditionalSaveData(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:195)
- [ ] `WraithEntity.readAdditionalSaveData(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:207)
- [ ] `WraithEntity.aiStep(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:217)
- [ ] `WraithEntity.finalizeSpawn(4)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:240)
- [ ] `WraithEntity.tickDeath(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:249)
- [ ] `WraithEntity.addParticles(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:257)
- [ ] `WraithEntity.addDeathParticles(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:270)
- [ ] `WraithEntity.getTimeSinceExtinguish(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:285)
- [ ] `WraithEntity.addExtinguishCounter(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:289)
- [ ] `WraithEntity.hasHomePosition(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:293)
- [ ] `WraithEntity.isInvulnerableTo(2)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:298)
- [ ] `WraithEntity.createWraithAttributes(0)`: it uses Minecraft's `Builder` (com/lion/graveyard/entities/WraithEntity.java:310)
- [ ] `WraithEntity.trackChase(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:318)
- [ ] `WraithEntity.getAnimation(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:327)
- [ ] `WraithEntity.setAnimation(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:331)
- [ ] `WraithEntity.registerControllers(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:335)
- [ ] `WraithEntity.isSpawning(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:354)
- [ ] `WraithEntity.getAnimatableInstanceCache(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:358)
- [ ] `WraithEntity.isWithinDistance(2)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:362)
- [ ] `WraithEntity.playAmbientSound(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:366)
- [ ] `WraithEntity.playHurtSound(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:370)
- [ ] `WraithEntity.die(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:374)
- [ ] `WraithEntity.playDeathSound(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:379)
- [ ] `WraithEntity.canCollideWith(1)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:385)
- [ ] `WraithEntity.playBlockFallSound(0)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:389)
- [ ] `WraithEntity.playStepSound(2)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:392)
- [ ] `WraithEntity.causeFallDamage(3)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:395)
- [ ] `WraithEntity.checkFallDamage(4)`: its class isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:399)
- [ ] `WraithEntity.triggerAdvancement(2)`: it uses WraithEntity, which isn't translated (it extends `HostileGraveyardEntity`) (com/lion/graveyard/entities/WraithEntity.java:402)

### `WraithEntity.ExtinguishGoal`: it extends Minecraft's `Goal`

- [ ] `WraithEntity.ExtinguishGoal.ExtinguishGoal(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/WraithEntity.java:412)
- [ ] `WraithEntity.ExtinguishGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/WraithEntity.java:415)
- [ ] `WraithEntity.ExtinguishGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/WraithEntity.java:427)
- [ ] `WraithEntity.ExtinguishGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/WraithEntity.java:431)

### `WraithEntity.WraithMeleeGoal`: it extends Minecraft's `MeleeAttackGoal`

- [ ] `WraithEntity.WraithMeleeGoal.WraithMeleeGoal(3)`: its class isn't translated (it extends Minecraft's `MeleeAttackGoal`) (com/lion/graveyard/entities/WraithEntity.java:497)
- [ ] `WraithEntity.WraithMeleeGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `MeleeAttackGoal`) (com/lion/graveyard/entities/WraithEntity.java:501)
- [ ] `WraithEntity.WraithMeleeGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `MeleeAttackGoal`) (com/lion/graveyard/entities/WraithEntity.java:505)

### `WraithEntity.WraithWanderAroundGoal`: it extends Minecraft's `Goal`

- [x] `WraithEntity.WraithWanderAroundGoal.MAX_DISTANCE` (com/lion/graveyard/entities/WraithEntity.java:513)
- [ ] `WraithEntity.WraithWanderAroundGoal.WraithWanderAroundGoal(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/WraithEntity.java:515)
- [ ] `WraithEntity.WraithWanderAroundGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/WraithEntity.java:519)
- [ ] `WraithEntity.WraithWanderAroundGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/WraithEntity.java:523)
- [ ] `WraithEntity.WraithWanderAroundGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/WraithEntity.java:527)
- [ ] `WraithEntity.WraithWanderAroundGoal.getRandomLocation(0)`: its class isn't translated (it extends Minecraft's `Goal`) (com/lion/graveyard/entities/WraithEntity.java:534)

### `CommonEvents`

- [x] `CommonEvents.CommonEvents(0)` (com/lion/graveyard/event/CommonEvents.java:41)
- [ ] `CommonEvents.onEntityLoad(1)`: it uses Minecraft's `Entity` (com/lion/graveyard/event/CommonEvents.java:44)
- [ ] `CommonEvents.addFleeGoal(3)`: it uses Minecraft's `AbstractVillager` (com/lion/graveyard/event/CommonEvents.java:57)
- [ ] `CommonEvents.onDeath(1)`: it uses Minecraft's `LivingEntity` (com/lion/graveyard/event/CommonEvents.java:64)
- [ ] `CommonEvents.onUseItem(2)`: `instanceof` (com/lion/graveyard/event/CommonEvents.java:70)
- [ ] `CommonEvents.onUseBlock(3)`: it uses Minecraft's `Level` (com/lion/graveyard/event/CommonEvents.java:76)
- [ ] `CommonEvents.usesBlock(1)`: it calls Minecraft's LocalPlayer.isSpectator (no Horizon helper for it yet) (com/lion/graveyard/event/CommonEvents.java:82)
- [ ] `CommonEvents.findPage(2)`: it uses Minecraft's `ServerPlayer` (com/lion/graveyard/event/CommonEvents.java:87)
- [ ] `CommonEvents.smolderingStrippedState(1)`: it uses Minecraft's `BlockState` (com/lion/graveyard/event/CommonEvents.java:94)
- [ ] `CommonEvents.stripSmolderingPillar(4)`: it uses Minecraft's `InteractionResult` (com/lion/graveyard/event/CommonEvents.java:99)
- [ ] `CommonEvents.shieldTakesPriority(2)`: it calls Minecraft's LocalPlayer.isSecondaryUseActive (no Horizon helper for it yet) (com/lion/graveyard/event/CommonEvents.java:121)
- [ ] `CommonEvents.mayModify(4)`: it uses Minecraft's `Level` (com/lion/graveyard/event/CommonEvents.java:125)

### `GraveyardClientFabric`: it implements Fabric's `ClientModInitializer`

- [ ] `GraveyardClientFabric.onInitializeClient(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (com/lion/graveyard/fabric/GraveyardClientFabric.java:32)

### `GraveyardFabric`: it implements Fabric's `ModInitializer`

- [ ] `GraveyardFabric.onInitialize(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (com/lion/graveyard/fabric/GraveyardFabric.java:32)
- [ ] `GraveyardFabric.registerCommands(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (com/lion/graveyard/fabric/GraveyardFabric.java:42)
- [ ] `GraveyardFabric.registerServerEvents(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (com/lion/graveyard/fabric/GraveyardFabric.java:49)

### `Graveyard`

- [x] `Graveyard.MOD_ID` (com/lion/graveyard/Graveyard.java:26)
- [x] `Graveyard.LOGGER` (com/lion/graveyard/Graveyard.java:27)
- [ ] `Graveyard.CONFIG`: it creates a GraveyardConfig with a constructor that isn't translated (com/lion/graveyard/Graveyard.java:28)
- [x] `Graveyard.id(1)` (com/lion/graveyard/Graveyard.java:32)
- [ ] `Graveyard.getConfig(0)`: it uses Graveyard.CONFIG, which isn't translated (it creates a GraveyardConfig with a constructor that isn't translated) (com/lion/graveyard/Graveyard.java:36)
- [ ] `Graveyard.getLogger(0)`: it uses `Logger` (org.slf4j) (com/lion/graveyard/Graveyard.java:40)
- [ ] `Graveyard.init(0)`: it calls TGEntities.init, which isn't translated (com/lion/graveyard/Graveyard.java:44)
- [ ] `Graveyard.postInit(0)`: it calls TGBlocks.postInit, which isn't translated (com/lion/graveyard/Graveyard.java:61)
- [x] `Graveyard.createStringID(1)` (com/lion/graveyard/Graveyard.java:68)

### `GraveyardClient`

- [ ] `GraveyardClient.init(0)`: it calls TGEntityModelLayers.init, which isn't translated (com/lion/graveyard/GraveyardClient.java:15)
- [ ] `GraveyardClient.postInit(0)`: it calls TGEntityRenderers.postInit, which isn't translated (com/lion/graveyard/GraveyardClient.java:25)

### `GravestoneScreen`: it extends Minecraft's `Screen`

- [x] `GravestoneScreen.FACE_SCALE` (com/lion/graveyard/gui/GravestoneScreen.java:26)
- [x] `GravestoneScreen.TEXT_SCALE` (com/lion/graveyard/gui/GravestoneScreen.java:27)
- [x] `GravestoneScreen.SIGN_Y_OFFSET` (com/lion/graveyard/gui/GravestoneScreen.java:28)
- [ ] `GravestoneScreen.GravestoneScreen(2)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/gui/GravestoneScreen.java:36)
- [ ] `GravestoneScreen.GravestoneScreen(3)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/gui/GravestoneScreen.java:40)
- [ ] `GravestoneScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/gui/GravestoneScreen.java:50)
- [ ] `GravestoneScreen.setCurrentRowMessage(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/gui/GravestoneScreen.java:64)
- [ ] `GravestoneScreen.removed(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/gui/GravestoneScreen.java:70)
- [ ] `GravestoneScreen.isInputCaptured(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/gui/GravestoneScreen.java:79)
- [ ] `GravestoneScreen.tick(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/gui/GravestoneScreen.java:83)
- [ ] `GravestoneScreen.finishEditing(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/gui/GravestoneScreen.java:90)
- [ ] `GravestoneScreen.charTyped(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/gui/GravestoneScreen.java:95)
- [ ] `GravestoneScreen.onClose(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/gui/GravestoneScreen.java:100)
- [ ] `GravestoneScreen.keyPressed(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/gui/GravestoneScreen.java:104)
- [ ] `GravestoneScreen.extractBackground(4)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/gui/GravestoneScreen.java:120)
- [ ] `GravestoneScreen.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/gui/GravestoneScreen.java:124)
- [ ] `GravestoneScreen.extractSignBackground(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/gui/GravestoneScreen.java:136)
- [ ] `GravestoneScreen.extractSignText(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/lion/graveyard/gui/GravestoneScreen.java:143)

### `OssuaryScreen`: it extends Minecraft's `AbstractContainerScreen`

- [x] `OssuaryScreen.TEXTURE` (com/lion/graveyard/gui/OssuaryScreen.java:24)
- [x] `OssuaryScreen.SCROLLER_SPRITE` (com/lion/graveyard/gui/OssuaryScreen.java:25)
- [x] `OssuaryScreen.SCROLLER_DISABLED_SPRITE` (com/lion/graveyard/gui/OssuaryScreen.java:26)
- [x] `OssuaryScreen.RECIPE_SPRITE` (com/lion/graveyard/gui/OssuaryScreen.java:27)
- [x] `OssuaryScreen.RECIPE_SELECTED_SPRITE` (com/lion/graveyard/gui/OssuaryScreen.java:28)
- [x] `OssuaryScreen.RECIPE_HIGHLIGHTED_SPRITE` (com/lion/graveyard/gui/OssuaryScreen.java:29)
- [ ] `OssuaryScreen.INPUT_HINT`: it uses Minecraft's `ItemStack` (com/lion/graveyard/gui/OssuaryScreen.java:30)
- [x] `OssuaryScreen.INPUT_HINT_TEXT` in game_side.rs (com/lion/graveyard/gui/OssuaryScreen.java:31)
- [ ] `OssuaryScreen.OssuaryScreen(3)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/lion/graveyard/gui/OssuaryScreen.java:37)
- [ ] `OssuaryScreen.receive(1)`: it uses OssuaryRecipesSync, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/lion/graveyard/gui/OssuaryScreen.java:43)
- [ ] `OssuaryScreen.extractBackground(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/lion/graveyard/gui/OssuaryScreen.java:51)
- [ ] `OssuaryScreen.extractTooltip(3)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/lion/graveyard/gui/OssuaryScreen.java:72)
- [ ] `OssuaryScreen.extractRecipeBackground(6)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/lion/graveyard/gui/OssuaryScreen.java:96)
- [ ] `OssuaryScreen.extractRecipeIcons(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/lion/graveyard/gui/OssuaryScreen.java:113)
- [ ] `OssuaryScreen.mouseClicked(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/lion/graveyard/gui/OssuaryScreen.java:125)
- [ ] `OssuaryScreen.mouseDragged(3)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/lion/graveyard/gui/OssuaryScreen.java:155)
- [ ] `OssuaryScreen.mouseScrolled(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/lion/graveyard/gui/OssuaryScreen.java:168)
- [ ] `OssuaryScreen.shouldScroll(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/lion/graveyard/gui/OssuaryScreen.java:179)
- [ ] `OssuaryScreen.getMaxScroll(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/lion/graveyard/gui/OssuaryScreen.java:183)
- [ ] `OssuaryScreen.onInventoryChange(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerScreen`) (com/lion/graveyard/gui/OssuaryScreen.java:187)

### `OssuaryScreenHandler`: it extends Minecraft's `AbstractContainerMenu`

- [ ] `OssuaryScreenHandler.OssuaryScreenHandler(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:45)
- [ ] `OssuaryScreenHandler.OssuaryScreenHandler(3)`: its body couldn't be read: an anonymous class (com/lion/graveyard/gui/OssuaryScreenHandler.java:49)
- [ ] `OssuaryScreenHandler.getSelectedRecipe(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:106)
- [ ] `OssuaryScreenHandler.getAvailableResults(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:110)
- [ ] `OssuaryScreenHandler.getAvailableRecipeCount(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:114)
- [ ] `OssuaryScreenHandler.setAvailableResults(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:118)
- [ ] `OssuaryScreenHandler.canCraft(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:123)
- [ ] `OssuaryScreenHandler.stillValid(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:127)
- [ ] `OssuaryScreenHandler.clickMenuButton(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:131)
- [ ] `OssuaryScreenHandler.isInBounds(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:140)
- [ ] `OssuaryScreenHandler.slotsChanged(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:144)
- [ ] `OssuaryScreenHandler.updateInput(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:152)
- [ ] `OssuaryScreenHandler.findRecipes(2)`: it uses Minecraft's `RecipeHolder` (com/lion/graveyard/gui/OssuaryScreenHandler.java:164)
- [ ] `OssuaryScreenHandler.populateResult(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:178)
- [ ] `OssuaryScreenHandler.getType(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:197)
- [ ] `OssuaryScreenHandler.setContentsChangedListener(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:201)
- [ ] `OssuaryScreenHandler.canTakeItemForPickAll(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:205)
- [ ] `OssuaryScreenHandler.quickMoveStack(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:209)
- [ ] `OssuaryScreenHandler.removed(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/lion/graveyard/gui/OssuaryScreenHandler.java:255)

### `TGAdvancements`

- [ ] `TGAdvancements.KILLED_BY_BONE_DAGGER`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:15)
- [ ] `TGAdvancements.KILL_WHILE_BLINDED`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:16)
- [ ] `TGAdvancements.DIM_LIGHT`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:17)
- [ ] `TGAdvancements.KILL_HORDE`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:18)
- [ ] `TGAdvancements.SPAWN_WRAITH`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:19)
- [ ] `TGAdvancements.EQUIP_COFFIN`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:20)
- [ ] `TGAdvancements.REDEEM_TORMENTED_VILLAGER`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:21)
- [ ] `TGAdvancements.TORMENTED_VILLAGER_DAWN`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:22)
- [ ] `TGAdvancements.EXTINGUISH_SMOLDERING`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:23)
- [ ] `TGAdvancements.IGNITE_SMOLDERING`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:24)
- [ ] `TGAdvancements.SCARE_CROWS`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:25)
- [ ] `TGAdvancements.MAGPIE_LOOT`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:26)
- [ ] `TGAdvancements.RAVEN_ON_GRAVESTONE`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:27)
- [ ] `TGAdvancements.SCARECROW`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:28)
- [ ] `TGAdvancements.FIRST_CHRONICLE_PAGE`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:29)
- [ ] `TGAdvancements.CHRONICLE_ACT_ONE`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:30)
- [ ] `TGAdvancements.CHRONICLE_ACT_TWO`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:31)
- [ ] `TGAdvancements.PLACE_LORE_BOOK`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:32)
- [ ] `TGAdvancements.BONE_COLLECTOR`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:33)
- [ ] `TGAdvancements.GHOUL_EMERGES`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:34)
- [x] `TGAdvancements.REDEEM_TEN_ADVANCEMENT` (com/lion/graveyard/init/TGAdvancements.java:35)
- [x] `TGAdvancements.REDEEM_TEN_STEPS` (com/lion/graveyard/init/TGAdvancements.java:36)
- [x] `TGAdvancements.init(0)` (com/lion/graveyard/init/TGAdvancements.java:38)
- [ ] `TGAdvancements.trigger(1)`: it uses TGAdvancementTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/lion/graveyard/init/TGAdvancements.java:41)
- [ ] `TGAdvancements.triggerNearby(4)`: it uses Minecraft's `ServerLevel` (com/lion/graveyard/init/TGAdvancements.java:47)
- [ ] `TGAdvancements.grantNextStep(3)`: it uses Minecraft's `ServerPlayer` (com/lion/graveyard/init/TGAdvancements.java:55)

### `TGBiomes`


### `TGBlockEntities`

- [ ] `TGBlockEntities.GRAVESTONE_BLOCK_ENTITY`: it uses Minecraft's `BlockEntityType` (com/lion/graveyard/init/TGBlockEntities.java:15)
- [ ] `TGBlockEntities.URN_BLOCK_ENTITY`: it uses Minecraft's `BlockEntityType` (com/lion/graveyard/init/TGBlockEntities.java:36)
- [ ] `TGBlockEntities.SARCOPHAGUS_BLOCK_ENTITY`: it uses Minecraft's `BlockEntityType` (com/lion/graveyard/init/TGBlockEntities.java:93)
- [ ] `TGBlockEntities.BRAZIER_BLOCK_ENTITY`: it uses Minecraft's `BlockEntityType` (com/lion/graveyard/init/TGBlockEntities.java:115)
- [ ] `TGBlockEntities.LORE_BOOK_BLOCK_ENTITY`: it uses Minecraft's `BlockEntityType` (com/lion/graveyard/init/TGBlockEntities.java:118)
- [ ] `TGBlockEntities.OSSUARY_BLOCK_ENTITY`: it uses Minecraft's `BlockEntityType` (com/lion/graveyard/init/TGBlockEntities.java:121)
- [x] `TGBlockEntities.init(0)` (com/lion/graveyard/init/TGBlockEntities.java:125)

### `TGBlockEntityRenderers`

- [ ] `TGBlockEntityRenderers.postInit(0)`: it calls RegistryHelper.registerBlockEntityRenderer, which isn't translated (com/lion/graveyard/init/TGBlockEntityRenderers.java:10)

### `TGBlocks`

- [ ] `TGBlocks.coffins`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:39)
- [ ] `TGBlocks.TG_ROOTED_DIRT`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:40)
- [ ] `TGBlocks.TG_TUFF`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:43)
- [ ] `TGBlocks.TG_MOSS_BLOCK`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:46)
- [ ] `TGBlocks.TG_DEEPSLATE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:49)
- [ ] `TGBlocks.TG_GRASS_BLOCK`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:54)
- [ ] `TGBlocks.TG_DIRT`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:57)
- [ ] `TGBlocks.TG_COARSE_DIRT`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:60)
- [ ] `TGBlocks.TG_ANDESITE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:65)
- [ ] `TGBlocks.TG_GRANITE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:70)
- [ ] `TGBlocks.TG_DIORITE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:75)
- [ ] `TGBlocks.TG_STONE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:80)
- [ ] `TGBlocks.TG_PODZOL`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:83)
- [ ] `TGBlocks.DARK_IRON_BARS`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:86)
- [ ] `TGBlocks.SOUL_FIRE_BRAZIER`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:89)
- [ ] `TGBlocks.FIRE_BRAZIER`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:94)
- [ ] `TGBlocks.PEDESTAL`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:99)
- [ ] `TGBlocks.CANDLE_HOLDER`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:102)
- [ ] `TGBlocks.DARK_IRON_TRAPDOOR`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:105)
- [ ] `TGBlocks.DARK_IRON_DOOR`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:108)
- [ ] `TGBlocks.DARK_IRON_BLOCK`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:111)
- [ ] `TGBlocks.SKULL_WITH_RIB_CAGE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:114)
- [ ] `TGBlocks.LEANING_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:115)
- [ ] `TGBlocks.SKULL_PILE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:116)
- [ ] `TGBlocks.LYING_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:117)
- [ ] `TGBlocks.WITHER_SKULL_WITH_RIB_CAGE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:118)
- [ ] `TGBlocks.LEANING_WITHER_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:121)
- [ ] `TGBlocks.WITHER_SKULL_PILE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:124)
- [ ] `TGBlocks.LYING_WITHER_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:125)
- [ ] `TGBlocks.CREEPER_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:128)
- [ ] `TGBlocks.SKELETON_HAND`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:129)
- [ ] `TGBlocks.WITHER_SKELETON_HAND`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:130)
- [ ] `TGBlocks.BONE_REMAINS`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:133)
- [ ] `TGBlocks.WITHER_BONE_REMAINS`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:134)
- [ ] `TGBlocks.LATERALLY_LYING_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:135)
- [ ] `TGBlocks.LATERALLY_LYING_WITHER_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:138)
- [ ] `TGBlocks.SKULL_ON_PIKE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:141)
- [ ] `TGBlocks.WITHER_SKULL_ON_PIKE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:142)
- [ ] `TGBlocks.TORSO_PILE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:145)
- [ ] `TGBlocks.WITHER_TORSO_PILE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:146)
- [ ] `TGBlocks.HANGED_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:147)
- [ ] `TGBlocks.HANGED_WITHER_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:148)
- [ ] `TGBlocks.SEATED_FULL_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:151)
- [ ] `TGBlocks.SEATED_FULL_WITHER_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:154)
- [ ] `TGBlocks.SKELETON_SKULL_WITH_AXE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:157)
- [ ] `TGBlocks.WITHER_SKELETON_SKULL_WITH_AXE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:160)
- [ ] `TGBlocks.SWORD_IN_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:163)
- [ ] `TGBlocks.SWORD_IN_WITHER_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:164)
- [ ] `TGBlocks.SMALL_SKELETON_REMAINS`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:167)
- [ ] `TGBlocks.SMALL_WITHER_SKELETON_REMAINS`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:170)
- [ ] `TGBlocks.HANGED_FULL_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:173)
- [ ] `TGBlocks.HANGED_FULL_WITHER_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:178)
- [ ] `TGBlocks.LYING_FULL_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:183)
- [ ] `TGBlocks.LYING_FULL_WITHER_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:188)
- [ ] `TGBlocks.BACKSTABBED_FULL_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:193)
- [ ] `TGBlocks.BACKSTABBED_FULL_WITHER_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:198)
- [ ] `TGBlocks.SHOT_FULL_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:203)
- [ ] `TGBlocks.SHOT_FULL_WITHER_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:208)
- [ ] `TGBlocks.SIDEWAYS_FULL_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:213)
- [ ] `TGBlocks.SIDEWAYS_FULL_WITHER_SKELETON`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:218)
- [ ] `TGBlocks.BLACK_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:223)
- [ ] `TGBlocks.BROWN_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:224)
- [ ] `TGBlocks.WHITE_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:225)
- [ ] `TGBlocks.GRAY_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:226)
- [ ] `TGBlocks.LIGHT_GRAY_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:227)
- [ ] `TGBlocks.BLUE_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:228)
- [ ] `TGBlocks.CYAN_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:229)
- [ ] `TGBlocks.LIGHT_BLUE_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:230)
- [ ] `TGBlocks.RED_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:231)
- [ ] `TGBlocks.ORANGE_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:232)
- [ ] `TGBlocks.YELLOW_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:233)
- [ ] `TGBlocks.PINK_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:234)
- [ ] `TGBlocks.MAGENTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:235)
- [ ] `TGBlocks.PURPLE_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:236)
- [ ] `TGBlocks.GREEN_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:237)
- [ ] `TGBlocks.LIME_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:238)
- [ ] `TGBlocks.TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:239)
- [ ] `TGBlocks.BLACK_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:240)
- [ ] `TGBlocks.BLUE_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:241)
- [ ] `TGBlocks.BROWN_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:242)
- [ ] `TGBlocks.CYAN_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:243)
- [ ] `TGBlocks.GRAY_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:244)
- [ ] `TGBlocks.GREEN_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:245)
- [ ] `TGBlocks.LIGHT_BLUE_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:246)
- [ ] `TGBlocks.LIGHT_GRAY_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:247)
- [ ] `TGBlocks.LIME_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:248)
- [ ] `TGBlocks.MAGENTA_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:249)
- [ ] `TGBlocks.ORANGE_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:250)
- [ ] `TGBlocks.PINK_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:251)
- [ ] `TGBlocks.PURPLE_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:252)
- [ ] `TGBlocks.RED_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:253)
- [ ] `TGBlocks.WHITE_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:254)
- [ ] `TGBlocks.YELLOW_TERRACOTTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:255)
- [ ] `TGBlocks.SMALL_BLACK_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:256)
- [ ] `TGBlocks.SMALL_BROWN_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:257)
- [ ] `TGBlocks.SMALL_WHITE_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:258)
- [ ] `TGBlocks.SMALL_GRAY_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:259)
- [ ] `TGBlocks.SMALL_LIGHT_GRAY_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:260)
- [ ] `TGBlocks.SMALL_BLUE_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:261)
- [ ] `TGBlocks.SMALL_CYAN_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:262)
- [ ] `TGBlocks.SMALL_LIGHT_BLUE_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:263)
- [ ] `TGBlocks.SMALL_RED_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:264)
- [ ] `TGBlocks.SMALL_ORANGE_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:265)
- [ ] `TGBlocks.SMALL_YELLOW_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:266)
- [ ] `TGBlocks.SMALL_PINK_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:267)
- [ ] `TGBlocks.SMALL_MAGENTA_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:268)
- [ ] `TGBlocks.SMALL_PURPLE_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:269)
- [ ] `TGBlocks.SMALL_GREEN_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:270)
- [ ] `TGBlocks.SMALL_LIME_URN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:271)
- [ ] `TGBlocks.SARCOPHAGUS`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:272)
- [ ] `TGBlocks.OAK_COFFIN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:277)
- [ ] `TGBlocks.SPRUCE_COFFIN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:282)
- [ ] `TGBlocks.BIRCH_COFFIN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:287)
- [ ] `TGBlocks.DARK_OAK_COFFIN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:292)
- [ ] `TGBlocks.JUNGLE_COFFIN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:297)
- [ ] `TGBlocks.ACACIA_COFFIN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:302)
- [ ] `TGBlocks.WARPED_COFFIN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:307)
- [ ] `TGBlocks.CRIMSON_COFFIN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:312)
- [ ] `TGBlocks.MANGROVE_COFFIN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:317)
- [ ] `TGBlocks.BAMBOO_COFFIN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:322)
- [ ] `TGBlocks.CHERRY_COFFIN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:327)
- [ ] `TGBlocks.PALE_OAK_COFFIN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:332)
- [ ] `TGBlocks.POPLAR_COFFIN`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:337)
- [ ] `TGBlocks.VASE_BLOCK`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:342)
- [ ] `TGBlocks.GRAVESTONE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:343)
- [ ] `TGBlocks.COBBLESTONE_GRAVESTONE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:346)
- [ ] `TGBlocks.MOSSY_COBBLESTONE_GRAVESTONE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:349)
- [ ] `TGBlocks.DEEPSLATE_GRAVESTONE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:352)
- [ ] `TGBlocks.BLACKSTONE_GRAVESTONE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:355)
- [ ] `TGBlocks.CRACKED_BLACKSTONE_GRAVESTONE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:358)
- [ ] `TGBlocks.STONE_BRICKS_GRAVESTONE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:361)
- [ ] `TGBlocks.MOSSY_STONE_BRICKS_GRAVESTONE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:364)
- [ ] `TGBlocks.BRICKS_GRAVESTONE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:367)
- [ ] `TGBlocks.SANDSTONE_GRAVESTONE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:370)
- [ ] `TGBlocks.RED_SANDSTONE_GRAVESTONE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:373)
- [ ] `TGBlocks.GILDED_BLACKSTONE_GRAVESTONE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:376)
- [ ] `TGBlocks.QUARTZ_BRICKS_GRAVESTONE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:379)
- [ ] `TGBlocks.ALTAR`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:382)
- [ ] `TGBlocks.ALTAR_SIDE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:383)
- [ ] `TGBlocks.ALTAR_CORNER`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:386)
- [ ] `TGBlocks.ALTAR_CENTER`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:389)
- [ ] `TGBlocks.LOWER_BONE_STAFF`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:392)
- [ ] `TGBlocks.MIDDLE_BONE_STAFF`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:395)
- [ ] `TGBlocks.UPPER_BONE_STAFF`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:398)
- [ ] `TGBlocks.OSSUARY`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:401)
- [ ] `TGBlocks.LORE_BOOK`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGBlocks.java:404)
- [ ] `TGBlocks.boneDisplayProperties(0)`: it calls Minecraft's `Properties`.of (com/lion/graveyard/init/TGBlocks.java:406)
- [ ] `TGBlocks.urnProperties(0)`: it calls Minecraft's `Properties`.of (com/lion/graveyard/init/TGBlocks.java:410)
- [ ] `TGBlocks.vaseProperties(0)`: it calls Minecraft's `Properties`.of (com/lion/graveyard/init/TGBlocks.java:414)
- [ ] `TGBlocks.boneStaffFragmentProperties(0)`: it calls Minecraft's `Properties`.of (com/lion/graveyard/init/TGBlocks.java:418)
- [ ] `TGBlocks.loreBookProperties(0)`: it calls Minecraft's `Properties`.of (com/lion/graveyard/init/TGBlocks.java:422)
- [ ] `TGBlocks.gravestoneProperties(0)`: it calls Minecraft's `Properties`.of (com/lion/graveyard/init/TGBlocks.java:426)
- [x] `TGBlocks.init(0)` (com/lion/graveyard/init/TGBlocks.java:430)
- [ ] `TGBlocks.postInit(0)`: it uses TGBlocks.coffins, which isn't translated (it uses Minecraft's `Block`) (com/lion/graveyard/init/TGBlocks.java:433)

### `TGEntities`

- [ ] `TGEntities.REAPER`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:51)
- [ ] `TGEntities.ACOLYTE`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:54)
- [ ] `TGEntities.CORRUPTED_VINDICATOR`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:57)
- [ ] `TGEntities.CORRUPTED_PILLAGER`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:61)
- [ ] `TGEntities.CORRUPTED_EVOKER`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:64)
- [ ] `TGEntities.SKELETON_CREEPER`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:67)
- [ ] `TGEntities.FALLING_CORPSE`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:70)
- [ ] `TGEntities.GHOUL`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:73)
- [ ] `TGEntities.TORMENTED_VILLAGER`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:76)
- [ ] `TGEntities.REVENANT`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:79)
- [ ] `TGEntities.WRAITH`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:82)
- [ ] `TGEntities.NIGHTMARE`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:85)
- [ ] `TGEntities.GHOULING`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:88)
- [ ] `TGEntities.LICH`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:91)
- [ ] `TGEntities.SKULL`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:94)
- [ ] `TGEntities.NAMELESS_HANGED`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:98)
- [ ] `TGEntities.RAVEN`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:101)
- [ ] `TGEntities.MAGPIE`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:104)
- [ ] `TGEntities.CROW`: it uses Minecraft's `EntityType` (com/lion/graveyard/init/TGEntities.java:107)
- [ ] `TGEntities.SCALED_ATTRIBUTES`: it uses Minecraft's `Holder` (com/lion/graveyard/init/TGEntities.java:110)
- [x] `TGEntities.TGEntities(0)` (com/lion/graveyard/init/TGEntities.java:114)
- [ ] `TGEntities.init(0)`: it calls TGEntities.createMobAttributes, which isn't translated (com/lion/graveyard/init/TGEntities.java:117)
- [ ] `TGEntities.postInit(0)`: it calls TGEntities.addSpawns, which isn't translated (com/lion/graveyard/init/TGEntities.java:122)
- [ ] `TGEntities.naturalSpawns(0)`: it uses NaturalSpawn, which isn't translated (it uses Minecraft's `TagKey`) (com/lion/graveyard/init/TGEntities.java:126)
- [ ] `TGEntities.biomeTag(1)`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGEntities.java:145)
- [ ] `TGEntities.addSpawns(0)`: it calls TGEntities.naturalSpawns, which isn't translated (com/lion/graveyard/init/TGEntities.java:149)
- [ ] `TGEntities.gated(1)`: it's generic (com/lion/graveyard/init/TGEntities.java:158)
- [ ] `TGEntities.addSpawnRestrictions(0)`: it calls RegistryHelper.registerSpawnPlacement, which isn't translated (com/lion/graveyard/init/TGEntities.java:164)
- [ ] `TGEntities.scaled(2)`: it uses Minecraft's `Builder` (com/lion/graveyard/init/TGEntities.java:197)
- [ ] `TGEntities.factor(2)`: it uses Minecraft's `Holder` (com/lion/graveyard/init/TGEntities.java:214)
- [ ] `TGEntities.createMobAttributes(0)`: it calls RegistryHelper.registerEntityAttribute, which isn't translated (com/lion/graveyard/init/TGEntities.java:224)

### `TGEntities.NaturalSpawn`: it uses Minecraft's `TagKey`

- [ ] `TGEntities.NaturalSpawn.NaturalSpawn(4)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/lion/graveyard/init/TGEntities.java:245)
- [ ] `TGEntities.NaturalSpawn.weight(0)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/lion/graveyard/init/TGEntities.java:246)
- [ ] `TGEntities.NaturalSpawn.minGroup(0)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/lion/graveyard/init/TGEntities.java:250)
- [ ] `TGEntities.NaturalSpawn.maxGroup(0)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/lion/graveyard/init/TGEntities.java:254)
- [ ] `TGEntities.NaturalSpawn.entry(0)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/lion/graveyard/init/TGEntities.java:258)
- [ ] `TGEntities.NaturalSpawn.mob(0)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/lion/graveyard/init/TGEntities.java:245)
- [ ] `TGEntities.NaturalSpawn.tag(0)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/lion/graveyard/init/TGEntities.java:245)
- [ ] `TGEntities.NaturalSpawn.group(0)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/lion/graveyard/init/TGEntities.java:245)
- [ ] `TGEntities.NaturalSpawn.type(0)`: its class isn't translated (it uses Minecraft's `TagKey`) (com/lion/graveyard/init/TGEntities.java:245)

### `TGEntityModelLayers`

- [ ] `TGEntityModelLayers.CORRUPTED_ILLAGER_MODEL_LAYER`: it uses Minecraft's `ModelLayerLocation` (com/lion/graveyard/init/TGEntityModelLayers.java:12)
- [ ] `TGEntityModelLayers.init(0)`: it calls RegistryHelper.registerEntityModelLayer, which isn't translated (com/lion/graveyard/init/TGEntityModelLayers.java:14)

### `TGEntityRenderers`

- [ ] `TGEntityRenderers.postInit(0)`: it calls RegistryHelper.registerEntityRenderer, which isn't translated (com/lion/graveyard/init/TGEntityRenderers.java:21)

### `TGItemGroups`

- [ ] `TGItemGroups.ITEM_GROUP`: it uses Minecraft's `ResourceKey` (com/lion/graveyard/init/TGItemGroups.java:15)
- [ ] `TGItemGroups.CHRONICLE_GROUP`: it uses Minecraft's `ResourceKey` (com/lion/graveyard/init/TGItemGroups.java:16)
- [ ] `TGItemGroups.init(0)`: it calls RegistryHelper.registerItemGroup, which isn't translated (com/lion/graveyard/init/TGItemGroups.java:18)

### `TGItems`

- [ ] `TGItems.INCARNATED_EVIL_SONG`: it uses Minecraft's `ResourceKey` (com/lion/graveyard/init/TGItems.java:26)
- [ ] `TGItems.SARCOPHAGUS_LID`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:27)
- [ ] `TGItems.SARCOPHAGUS_BASE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:28)
- [ ] `TGItems.DARK_OAK_COFFIN_LID`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:29)
- [ ] `TGItems.DARK_OAK_COFFIN_BASE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:30)
- [ ] `TGItems.OAK_COFFIN_LID`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:31)
- [ ] `TGItems.OAK_COFFIN_BASE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:32)
- [ ] `TGItems.BIRCH_COFFIN_LID`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:33)
- [ ] `TGItems.BIRCH_COFFIN_BASE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:34)
- [ ] `TGItems.JUNGLE_COFFIN_LID`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:35)
- [ ] `TGItems.JUNGLE_COFFIN_BASE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:36)
- [ ] `TGItems.ACACIA_COFFIN_LID`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:37)
- [ ] `TGItems.ACACIA_COFFIN_BASE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:38)
- [ ] `TGItems.SPRUCE_COFFIN_LID`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:39)
- [ ] `TGItems.SPRUCE_COFFIN_BASE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:40)
- [ ] `TGItems.WARPED_COFFIN_LID`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:41)
- [ ] `TGItems.WARPED_COFFIN_BASE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:42)
- [ ] `TGItems.CRIMSON_COFFIN_LID`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:43)
- [ ] `TGItems.CRIMSON_COFFIN_BASE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:44)
- [ ] `TGItems.MANGROVE_COFFIN_LID`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:45)
- [ ] `TGItems.MANGROVE_COFFIN_BASE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:46)
- [ ] `TGItems.BAMBOO_COFFIN_LID`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:47)
- [ ] `TGItems.BAMBOO_COFFIN_BASE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:48)
- [ ] `TGItems.CHERRY_COFFIN_LID`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:49)
- [ ] `TGItems.CHERRY_COFFIN_BASE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:50)
- [ ] `TGItems.PALE_OAK_COFFIN_LID`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:51)
- [ ] `TGItems.PALE_OAK_COFFIN_BASE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:52)
- [ ] `TGItems.POPLAR_COFFIN_LID`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:53)
- [ ] `TGItems.POPLAR_COFFIN_BASE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:54)
- [ ] `TGItems.OSSUARY`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:55)
- [ ] `TGItems.INCARNATED_EVIL_MUSIC_DISC`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:58)
- [ ] `TGItems.SKELETON_CREEPER_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:61)
- [ ] `TGItems.RAVEN_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:64)
- [ ] `TGItems.MAGPIE_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:65)
- [ ] `TGItems.CROW_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:66)
- [ ] `TGItems.ACOLYTE_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:67)
- [ ] `TGItems.GHOUL_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:68)
- [ ] `TGItems.REAPER_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:69)
- [ ] `TGItems.TORMENTED_VILLAGER_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:70)
- [ ] `TGItems.REVENANT_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:73)
- [ ] `TGItems.NIGHTMARE_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:74)
- [ ] `TGItems.LICH_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:75)
- [ ] `TGItems.NAMELESS_HANGED_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:76)
- [ ] `TGItems.WRAITH_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:79)
- [ ] `TGItems.CORRUPTED_PILLAGER_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:80)
- [ ] `TGItems.CORRUPTED_VINDICATOR_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:83)
- [ ] `TGItems.CORRUPTED_EVOKER_SPAWN_EGG`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:86)
- [ ] `TGItems.BONE_DAGGER`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:89)
- [ ] `TGItems.DARK_IRON_INGOT`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:90)
- [ ] `TGItems.CORRUPTION`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:91)
- [ ] `TGItems.DIARY_PAGE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:92)
- [ ] `TGItems.RUINS_MAP`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:95)
- [ ] `TGItems.TG_ROOTED_DIRT`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:98)
- [ ] `TGItems.TG_TUFF`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:101)
- [ ] `TGItems.TG_MOSS_BLOCK`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:104)
- [ ] `TGItems.TG_DEEPSLATE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:107)
- [ ] `TGItems.TG_GRASS_BLOCK`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:110)
- [ ] `TGItems.TG_DIRT`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:113)
- [ ] `TGItems.TG_COARSE_DIRT`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:116)
- [ ] `TGItems.TG_ANDESITE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:119)
- [ ] `TGItems.TG_GRANITE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:122)
- [ ] `TGItems.TG_DIORITE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:125)
- [ ] `TGItems.TG_STONE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:128)
- [ ] `TGItems.TG_PODZOL`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:131)
- [ ] `TGItems.DARK_IRON_BARS`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:134)
- [ ] `TGItems.SOUL_FIRE_BRAZIER`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:137)
- [ ] `TGItems.FIRE_BRAZIER`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:140)
- [ ] `TGItems.PEDESTAL`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:143)
- [ ] `TGItems.CANDLE_HOLDER`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:146)
- [ ] `TGItems.DARK_IRON_TRAPDOOR`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:149)
- [ ] `TGItems.DARK_IRON_DOOR`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:152)
- [ ] `TGItems.DARK_IRON_BLOCK`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:155)
- [ ] `TGItems.SKULL_WITH_RIB_CAGE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:158)
- [ ] `TGItems.LEANING_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:161)
- [ ] `TGItems.SKULL_PILE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:164)
- [ ] `TGItems.LYING_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:167)
- [ ] `TGItems.WITHER_SKULL_WITH_RIB_CAGE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:170)
- [ ] `TGItems.LEANING_WITHER_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:173)
- [ ] `TGItems.WITHER_SKULL_PILE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:176)
- [ ] `TGItems.LYING_WITHER_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:179)
- [ ] `TGItems.CREEPER_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:182)
- [ ] `TGItems.SKELETON_HAND`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:185)
- [ ] `TGItems.WITHER_SKELETON_HAND`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:188)
- [ ] `TGItems.HANGED_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:191)
- [ ] `TGItems.HANGED_WITHER_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:194)
- [ ] `TGItems.SEATED_FULL_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:197)
- [ ] `TGItems.SEATED_FULL_WITHER_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:200)
- [ ] `TGItems.SKELETON_SKULL_WITH_AXE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:203)
- [ ] `TGItems.WITHER_SKELETON_SKULL_WITH_AXE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:206)
- [ ] `TGItems.SWORD_IN_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:211)
- [ ] `TGItems.SWORD_IN_WITHER_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:214)
- [ ] `TGItems.SMALL_SKELETON_REMAINS`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:217)
- [ ] `TGItems.SMALL_WITHER_SKELETON_REMAINS`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:220)
- [ ] `TGItems.HANGED_FULL_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:225)
- [ ] `TGItems.HANGED_FULL_WITHER_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:228)
- [ ] `TGItems.LYING_FULL_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:231)
- [ ] `TGItems.LYING_FULL_WITHER_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:234)
- [ ] `TGItems.BACKSTABBED_FULL_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:237)
- [ ] `TGItems.BACKSTABBED_FULL_WITHER_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:240)
- [ ] `TGItems.SHOT_FULL_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:245)
- [ ] `TGItems.SHOT_FULL_WITHER_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:248)
- [ ] `TGItems.SIDEWAYS_FULL_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:251)
- [ ] `TGItems.SIDEWAYS_FULL_WITHER_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:254)
- [ ] `TGItems.VASE_BLOCK`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:259)
- [ ] `TGItems.GRAVESTONE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:262)
- [ ] `TGItems.COBBLESTONE_GRAVESTONE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:265)
- [ ] `TGItems.MOSSY_COBBLESTONE_GRAVESTONE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:268)
- [ ] `TGItems.DEEPSLATE_GRAVESTONE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:271)
- [ ] `TGItems.BLACKSTONE_GRAVESTONE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:274)
- [ ] `TGItems.CRACKED_BLACKSTONE_GRAVESTONE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:277)
- [ ] `TGItems.STONE_BRICKS_GRAVESTONE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:282)
- [ ] `TGItems.MOSSY_STONE_BRICKS_GRAVESTONE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:285)
- [ ] `TGItems.BRICKS_GRAVESTONE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:290)
- [ ] `TGItems.GILDED_BLACKSTONE_GRAVESTONE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:293)
- [ ] `TGItems.SANDSTONE_GRAVESTONE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:296)
- [ ] `TGItems.RED_SANDSTONE_GRAVESTONE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:299)
- [ ] `TGItems.QUARTZ_BRICKS_GRAVESTONE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:302)
- [ ] `TGItems.BLACK_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:305)
- [ ] `TGItems.BROWN_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:308)
- [ ] `TGItems.WHITE_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:311)
- [ ] `TGItems.GRAY_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:314)
- [ ] `TGItems.LIGHT_GRAY_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:317)
- [ ] `TGItems.BLUE_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:320)
- [ ] `TGItems.CYAN_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:323)
- [ ] `TGItems.LIGHT_BLUE_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:326)
- [ ] `TGItems.RED_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:329)
- [ ] `TGItems.ORANGE_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:332)
- [ ] `TGItems.YELLOW_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:335)
- [ ] `TGItems.PINK_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:338)
- [ ] `TGItems.MAGENTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:341)
- [ ] `TGItems.PURPLE_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:344)
- [ ] `TGItems.GREEN_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:347)
- [ ] `TGItems.LIME_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:350)
- [ ] `TGItems.TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:353)
- [ ] `TGItems.BLACK_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:356)
- [ ] `TGItems.BLUE_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:359)
- [ ] `TGItems.BROWN_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:362)
- [ ] `TGItems.CYAN_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:365)
- [ ] `TGItems.GRAY_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:368)
- [ ] `TGItems.GREEN_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:371)
- [ ] `TGItems.LIGHT_BLUE_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:374)
- [ ] `TGItems.LIGHT_GRAY_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:377)
- [ ] `TGItems.LIME_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:380)
- [ ] `TGItems.MAGENTA_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:383)
- [ ] `TGItems.ORANGE_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:386)
- [ ] `TGItems.PINK_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:389)
- [ ] `TGItems.PURPLE_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:392)
- [ ] `TGItems.RED_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:395)
- [ ] `TGItems.WHITE_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:398)
- [ ] `TGItems.YELLOW_TERRACOTTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:401)
- [ ] `TGItems.SMALL_BLACK_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:404)
- [ ] `TGItems.SMALL_BROWN_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:407)
- [ ] `TGItems.SMALL_WHITE_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:410)
- [ ] `TGItems.SMALL_GRAY_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:413)
- [ ] `TGItems.SMALL_LIGHT_GRAY_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:416)
- [ ] `TGItems.SMALL_BLUE_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:419)
- [ ] `TGItems.SMALL_CYAN_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:422)
- [ ] `TGItems.SMALL_LIGHT_BLUE_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:425)
- [ ] `TGItems.SMALL_RED_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:428)
- [ ] `TGItems.SMALL_ORANGE_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:431)
- [ ] `TGItems.SMALL_YELLOW_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:434)
- [ ] `TGItems.SMALL_PINK_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:437)
- [ ] `TGItems.SMALL_MAGENTA_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:440)
- [ ] `TGItems.SMALL_PURPLE_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:443)
- [ ] `TGItems.SMALL_GREEN_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:446)
- [ ] `TGItems.SMALL_LIME_URN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:449)
- [ ] `TGItems.SARCOPHAGUS`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:452)
- [ ] `TGItems.OAK_COFFIN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:455)
- [ ] `TGItems.SPRUCE_COFFIN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:458)
- [ ] `TGItems.BIRCH_COFFIN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:461)
- [ ] `TGItems.DARK_OAK_COFFIN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:464)
- [ ] `TGItems.JUNGLE_COFFIN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:467)
- [ ] `TGItems.ACACIA_COFFIN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:470)
- [ ] `TGItems.WARPED_COFFIN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:473)
- [ ] `TGItems.CRIMSON_COFFIN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:476)
- [ ] `TGItems.MANGROVE_COFFIN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:479)
- [ ] `TGItems.BAMBOO_COFFIN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:482)
- [ ] `TGItems.CHERRY_COFFIN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:485)
- [ ] `TGItems.PALE_OAK_COFFIN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:488)
- [ ] `TGItems.POPLAR_COFFIN`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:491)
- [ ] `TGItems.VIAL_OF_BLOOD`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:494)
- [ ] `TGItems.TORSO_PILE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:495)
- [ ] `TGItems.WITHER_TORSO_PILE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:498)
- [ ] `TGItems.SKULL_ON_PIKE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:501)
- [ ] `TGItems.WITHER_SKULL_ON_PIKE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:504)
- [ ] `TGItems.BONE_REMAINS`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:507)
- [ ] `TGItems.WITHER_BONE_REMAINS`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:510)
- [ ] `TGItems.LATERALLY_LYING_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:513)
- [ ] `TGItems.LATERALLY_LYING_WITHER_SKELETON`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:516)
- [ ] `TGItems.WHITE_BONE_STAFF`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:521)
- [ ] `TGItems.BLACK_BONE_STAFF`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:524)
- [ ] `TGItems.RED_BONE_STAFF`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:527)
- [ ] `TGItems.CYAN_BONE_STAFF`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:530)
- [ ] `TGItems.PURPLE_BONE_STAFF`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:533)
- [ ] `TGItems.ALTAR`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:536)
- [ ] `TGItems.ALTAR_SIDE`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:539)
- [ ] `TGItems.ALTAR_CORNER`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:542)
- [ ] `TGItems.ALTAR_CENTER`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:545)
- [ ] `TGItems.UPPER_BONE_STAFF`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:548)
- [ ] `TGItems.MIDDLE_BONE_STAFF`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:551)
- [ ] `TGItems.LOWER_BONE_STAFF`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGItems.java:554)
- [x] `TGItems.init(0)` (com/lion/graveyard/init/TGItems.java:558)
- [ ] `TGItems.addItemsToGroup(0)`: it calls RegistryHelper.addToItemGroup, which isn't translated (com/lion/graveyard/init/TGItems.java:561)
- [ ] `TGItems.vialStages(0)`: it creates Minecraft's `ItemStack` (com/lion/graveyard/init/TGItems.java:723)

### `TGLootConditions`

- [x] `TGLootConditions.TGLootConditions(0)` (com/lion/graveyard/init/TGLootConditions.java:7)
- [ ] `TGLootConditions.init(0)`: it calls RegistryHelper.registerLootCondition, which isn't translated (com/lion/graveyard/init/TGLootConditions.java:10)

### `TGOverrunVillage`

- [ ] `TGOverrunVillage.CHARRED_PLANKS`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:35)
- [ ] `TGOverrunVillage.CHARRED_LOG`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:45)
- [ ] `TGOverrunVillage.SCORCHED_PLANKS`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:55)
- [ ] `TGOverrunVillage.SCORCHED_LOG`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:65)
- [ ] `TGOverrunVillage.SMOLDERING_DEBRIS`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:75)
- [ ] `TGOverrunVillage.CHARRED_STAIRS`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:80)
- [ ] `TGOverrunVillage.CHARRED_SLAB`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:83)
- [ ] `TGOverrunVillage.CHARRED_FENCE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:84)
- [ ] `TGOverrunVillage.CHARRED_FENCE_GATE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:87)
- [ ] `TGOverrunVillage.CHARRED_DOOR`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:90)
- [ ] `TGOverrunVillage.CHARRED_TRAPDOOR`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:95)
- [ ] `TGOverrunVillage.SCORCHED_STAIRS`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:100)
- [ ] `TGOverrunVillage.SCORCHED_SLAB`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:103)
- [ ] `TGOverrunVillage.SCORCHED_FENCE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:104)
- [ ] `TGOverrunVillage.SCORCHED_FENCE_GATE`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:107)
- [ ] `TGOverrunVillage.SCORCHED_DOOR`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:110)
- [ ] `TGOverrunVillage.SCORCHED_TRAPDOOR`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:115)
- [ ] `TGOverrunVillage.CHARRED_WOOD_BLOCK`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:120)
- [ ] `TGOverrunVillage.STRIPPED_CHARRED_LOG`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:130)
- [ ] `TGOverrunVillage.STRIPPED_CHARRED_WOOD`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:140)
- [ ] `TGOverrunVillage.SCORCHED_WOOD_BLOCK`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:150)
- [ ] `TGOverrunVillage.STRIPPED_SCORCHED_LOG`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:160)
- [ ] `TGOverrunVillage.STRIPPED_SCORCHED_WOOD`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:170)
- [ ] `TGOverrunVillage.CHARRED_WOOD`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:180)
- [ ] `TGOverrunVillage.SCORCHED_WOOD`: it uses Minecraft's `Block` (com/lion/graveyard/init/TGOverrunVillage.java:192)
- [ ] `TGOverrunVillage.CHARRED_PLANKS_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:204)
- [ ] `TGOverrunVillage.CHARRED_LOG_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:207)
- [ ] `TGOverrunVillage.SCORCHED_PLANKS_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:210)
- [ ] `TGOverrunVillage.SCORCHED_LOG_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:213)
- [ ] `TGOverrunVillage.CHARRED_WOOD_BLOCK_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:216)
- [ ] `TGOverrunVillage.STRIPPED_CHARRED_LOG_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:219)
- [ ] `TGOverrunVillage.STRIPPED_CHARRED_WOOD_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:224)
- [ ] `TGOverrunVillage.SCORCHED_WOOD_BLOCK_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:229)
- [ ] `TGOverrunVillage.STRIPPED_SCORCHED_LOG_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:232)
- [ ] `TGOverrunVillage.STRIPPED_SCORCHED_WOOD_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:237)
- [ ] `TGOverrunVillage.SMOLDERING_DEBRIS_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:242)
- [ ] `TGOverrunVillage.CHARRED_STAIRS_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:245)
- [ ] `TGOverrunVillage.CHARRED_SLAB_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:248)
- [ ] `TGOverrunVillage.CHARRED_FENCE_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:251)
- [ ] `TGOverrunVillage.CHARRED_FENCE_GATE_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:254)
- [ ] `TGOverrunVillage.CHARRED_DOOR_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:257)
- [ ] `TGOverrunVillage.CHARRED_TRAPDOOR_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:260)
- [ ] `TGOverrunVillage.SCORCHED_STAIRS_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:263)
- [ ] `TGOverrunVillage.SCORCHED_SLAB_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:266)
- [ ] `TGOverrunVillage.SCORCHED_FENCE_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:269)
- [ ] `TGOverrunVillage.SCORCHED_FENCE_GATE_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:272)
- [ ] `TGOverrunVillage.SCORCHED_DOOR_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:277)
- [ ] `TGOverrunVillage.SCORCHED_TRAPDOOR_ITEM`: it uses Minecraft's `Item` (com/lion/graveyard/init/TGOverrunVillage.java:280)
- [ ] `TGOverrunVillage.SMOLDERING_DEBRIS_BLOCK_ENTITY`: it uses Minecraft's `BlockEntityType` (com/lion/graveyard/init/TGOverrunVillage.java:283)
- [ ] `TGOverrunVillage.wood(1)`: it uses Minecraft's `MapColor` (com/lion/graveyard/init/TGOverrunVillage.java:287)
- [x] `TGOverrunVillage.init(0)` (com/lion/graveyard/init/TGOverrunVillage.java:291)
- [ ] `TGOverrunVillage.addItemsToGroup(0)`: it calls RegistryHelper.addToItemGroup, which isn't translated (com/lion/graveyard/init/TGOverrunVillage.java:294)

### `TGParticles`

- [ ] `TGParticles.GRAVEYARD_FOG_PARTICLE`: it uses Minecraft's `SimpleParticleType` (com/lion/graveyard/init/TGParticles.java:7)
- [ ] `TGParticles.GRAVEYARD_SOUL_PARTICLE`: it uses Minecraft's `SimpleParticleType` (com/lion/graveyard/init/TGParticles.java:8)
- [ ] `TGParticles.GRAVEYARD_HAND_PARTICLE`: it uses Minecraft's `SimpleParticleType` (com/lion/graveyard/init/TGParticles.java:9)
- [ ] `TGParticles.GRAVEYARD_LEFT_HAND_PARTICLE`: it uses Minecraft's `SimpleParticleType` (com/lion/graveyard/init/TGParticles.java:10)
- [ ] `TGParticles.GRAVEYARD_SOUL_BEAM_PARTICLE`: it uses Minecraft's `SimpleParticleType` (com/lion/graveyard/init/TGParticles.java:11)
- [x] `TGParticles.init(0)` (com/lion/graveyard/init/TGParticles.java:13)

### `TGProcessors`

- [x] `TGProcessors.init(0)` (com/lion/graveyard/init/TGProcessors.java:9)

### `TGRecipeTypes`

- [ ] `TGRecipeTypes.OSSUARY_CARVING`: it uses Minecraft's `RecipeType` (com/lion/graveyard/init/TGRecipeTypes.java:11)
- [ ] `TGRecipeTypes.OSSUARY_CARVING_SERIALIZER`: it uses Minecraft's `RecipeSerializer` (com/lion/graveyard/init/TGRecipeTypes.java:12)
- [x] `TGRecipeTypes.init(0)` (com/lion/graveyard/init/TGRecipeTypes.java:16)

### `TGRecipeTypes.Type`: it implements Minecraft's `RecipeType`

- [ ] `TGRecipeTypes.Type.INSTANCE`: it uses Type, which isn't translated (it implements Minecraft's `RecipeType`) (com/lion/graveyard/init/TGRecipeTypes.java:20)
- [ ] `TGRecipeTypes.Type.Type(0)`: its class isn't translated (it implements Minecraft's `RecipeType`) (com/lion/graveyard/init/TGRecipeTypes.java:22)

### `TGScreens`

- [ ] `TGScreens.OSSUARY_SCREEN_HANDLER`: it uses Minecraft's `MenuType` (com/lion/graveyard/init/TGScreens.java:9)
- [x] `TGScreens.init(0)` (com/lion/graveyard/init/TGScreens.java:11)

### `TGSounds`

- [ ] `TGSounds.LICH_SPAWN`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:9)
- [ ] `TGSounds.LICH_MELEE`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:10)
- [ ] `TGSounds.LICH_PHASE_02`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:11)
- [ ] `TGSounds.LICH_PHASE_03`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:12)
- [ ] `TGSounds.LICH_CORPSE_SPELL`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:13)
- [ ] `TGSounds.LICH_CAST_SKULL`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:14)
- [ ] `TGSounds.LICH_CAST_LEVITATION`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:15)
- [ ] `TGSounds.LICH_CAST_TELEPORT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:16)
- [ ] `TGSounds.LICH_DEATH`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:17)
- [ ] `TGSounds.LICH_HUNT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:18)
- [ ] `TGSounds.LICH_SCARE`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:19)
- [ ] `TGSounds.LICH_PHASE_03_ATTACK`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:20)
- [ ] `TGSounds.LICH_IDLE`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:21)
- [ ] `TGSounds.LICH_HURT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:22)
- [ ] `TGSounds.LICH_BONE_CRACK`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:23)
- [ ] `TGSounds.LICH_STUNNED`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:24)
- [ ] `TGSounds.NAMELESS_HANGED_BREATH`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:25)
- [ ] `TGSounds.NAMELESS_HANGED_AMBIENT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:26)
- [ ] `TGSounds.NAMELESS_HANGED_INTERACT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:27)
- [ ] `TGSounds.CORRUPTED_ILLAGER_AMBIENT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:28)
- [ ] `TGSounds.CORRUPTED_ILLAGER_HURT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:29)
- [ ] `TGSounds.CORRUPTED_ILLAGER_STRIKE`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:30)
- [ ] `TGSounds.CORRUPTED_ILLAGER_BITE`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:31)
- [ ] `TGSounds.CORRUPTED_ILLAGER_DEATH`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:32)
- [ ] `TGSounds.CORRUPTED_ILLAGER_STEP`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:33)
- [ ] `TGSounds.DIG`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:34)
- [ ] `TGSounds.CORRUPTED_ILLAGER_SPAWN`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:35)
- [ ] `TGSounds.CORRUPTED_ILLAGER_IDLE`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:36)
- [ ] `TGSounds.CORRUPTED_ILLAGER_WANDER`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:37)
- [ ] `TGSounds.CORRUPTED_ILLAGER_ATTACK_RUN`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:38)
- [ ] `TGSounds.ACOLYTE_AMBIENT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:39)
- [ ] `TGSounds.ACOLYTE_HURT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:40)
- [ ] `TGSounds.ACOLYTE_DEATH`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:41)
- [ ] `TGSounds.REAPER_AMBIENT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:42)
- [ ] `TGSounds.REAPER_HURT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:43)
- [ ] `TGSounds.REAPER_DEATH`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:44)
- [ ] `TGSounds.REAPER_CHARGE`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:45)
- [ ] `TGSounds.GHOUL_ROAR`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:46)
- [ ] `TGSounds.GHOUL_AMBIENT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:47)
- [ ] `TGSounds.GHOUL_HURT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:48)
- [ ] `TGSounds.GHOUL_DEATH`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:49)
- [ ] `TGSounds.GHOUL_STEP`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:50)
- [ ] `TGSounds.GHOULING_GROAN`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:51)
- [ ] `TGSounds.GHOULING_AMBIENT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:52)
- [ ] `TGSounds.GHOULING_HURT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:53)
- [ ] `TGSounds.GHOULING_DEATH`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:54)
- [ ] `TGSounds.GHOULING_ATTACK`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:55)
- [ ] `TGSounds.GHOULING_STEP`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:56)
- [ ] `TGSounds.GHOULING_SPAWN`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:57)
- [ ] `TGSounds.GHOULING_SOUL_DRAW`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:58)
- [ ] `TGSounds.REVENANT_AMBIENT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:59)
- [ ] `TGSounds.REVENANT_HURT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:60)
- [ ] `TGSounds.REVENANT_DEATH`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:61)
- [ ] `TGSounds.REVENANT_STEP`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:62)
- [ ] `TGSounds.NIGHTMARE_AMBIENT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:63)
- [ ] `TGSounds.NIGHTMARE_HURT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:64)
- [ ] `TGSounds.NIGHTMARE_DEATH`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:65)
- [ ] `TGSounds.WRAITH_AMBIENT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:66)
- [ ] `TGSounds.WRAITH_HURT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:67)
- [ ] `TGSounds.ALTAR_AMBIENT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:68)
- [ ] `TGSounds.COFFIN_OPEN`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:69)
- [ ] `TGSounds.COFFIN_CLOSE`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:70)
- [ ] `TGSounds.URN_OPEN`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:71)
- [ ] `TGSounds.URN_CLOSE`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:72)
- [ ] `TGSounds.OSSUARY_OPEN`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:73)
- [ ] `TGSounds.SARCOPHAGUS_USE`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:74)
- [ ] `TGSounds.BONE_PLACED`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:75)
- [ ] `TGSounds.BONE_AMBIENT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:76)
- [ ] `TGSounds.CROW_AMBIENT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:77)
- [ ] `TGSounds.CROW_ALARM`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:78)
- [ ] `TGSounds.RAVEN_AMBIENT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:79)
- [ ] `TGSounds.MAGPIE_AMBIENT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:80)
- [ ] `TGSounds.CORVID_HURT`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:81)
- [ ] `TGSounds.CORVID_DEATH`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:82)
- [ ] `TGSounds.CORVID_TAKEOFF`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:83)
- [ ] `TGSounds.CORVID_FLY`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:84)
- [ ] `TGSounds.CORVID_LAND`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:85)
- [ ] `TGSounds.TORMENTED_VILLAGER_IDLE`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:86)
- [ ] `TGSounds.TORMENTED_VILLAGER_LOST`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:87)
- [ ] `TGSounds.TORMENTED_VILLAGER_CRYING`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:88)
- [ ] `TGSounds.TORMENTED_VILLAGER_GLITCH`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:89)
- [ ] `TGSounds.TORMENTED_VILLAGER_RAGE`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:90)
- [ ] `TGSounds.TORMENTED_VILLAGER_BREAKDOWN`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:91)
- [ ] `TGSounds.TORMENTED_VILLAGER_SPAWN`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:92)
- [ ] `TGSounds.TORMENTED_VILLAGER_REDEMPTION`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:93)
- [ ] `TGSounds.VIAL_SPLASH`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:94)
- [ ] `TGSounds.LICH_THEME_01`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:95)
- [x] `TGSounds.init(0)` (com/lion/graveyard/init/TGSounds.java:97)
- [ ] `TGSounds.register(1)`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/init/TGSounds.java:100)

### `TGStructures`

- [ ] `TGStructures.TG_JIGSAW`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:8)
- [ ] `TGStructures.ALTAR`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:9)
- [ ] `TGStructures.DEAD_TREE`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:10)
- [ ] `TGStructures.CRYPT`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:11)
- [ ] `TGStructures.GIANT_MUSHROOM`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:12)
- [ ] `TGStructures.HAUNTED_HOUSE`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:13)
- [ ] `TGStructures.LARGE_GRAVEYARD`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:14)
- [ ] `TGStructures.LICH_PRISON`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:15)
- [ ] `TGStructures.MEDIUM_GRAVEYARD`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:16)
- [ ] `TGStructures.MEMORIAL_TREE`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:17)
- [ ] `TGStructures.MUSHROOM_GRAVE`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:18)
- [ ] `TGStructures.RUINS`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:19)
- [ ] `TGStructures.SMALL_DESERT_GRAVE`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:20)
- [ ] `TGStructures.SMALL_DESERT_GRAVEYARD`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:21)
- [ ] `TGStructures.SMALL_GRAVE`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:22)
- [ ] `TGStructures.SMALL_GRAVEYARD`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:23)
- [ ] `TGStructures.SMALL_MOUNTAIN_GRAVE`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:24)
- [ ] `TGStructures.SMALL_SAVANNA_GRAVE`: it uses Minecraft's `StructureType` (com/lion/graveyard/init/TGStructures.java:25)
- [x] `TGStructures.init(0)` (com/lion/graveyard/init/TGStructures.java:27)

### `TGTags`

- [ ] `TGTags.GHOUL_SPAWNS`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:12)
- [ ] `TGTags.REVENANT_SPAWNS`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:13)
- [ ] `TGTags.NIGHTMARE_SPAWNS`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:14)
- [ ] `TGTags.SKELETON_CREEPER_SPAWNS`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:15)
- [ ] `TGTags.REAPER_SPAWNS`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:16)
- [ ] `TGTags.RAVEN_SPAWNS`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:17)
- [ ] `TGTags.MAGPIE_SPAWNS`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:18)
- [ ] `TGTags.CROW_SPAWNS`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:19)
- [ ] `TGTags.NATURAL_HELPER_BLOCKS`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:20)
- [ ] `TGTags.CORVID_PERCHES`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:21)
- [ ] `TGTags.CORVID_BONES`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:22)
- [ ] `TGTags.CROW_SETTLE`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:23)
- [ ] `TGTags.CROW_LANDMARKS`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:24)
- [ ] `TGTags.CROW_REPELLENTS`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:25)
- [ ] `TGTags.SCARECROW_HEADS`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:26)
- [ ] `TGTags.SCARECROW_BODIES`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:27)
- [ ] `TGTags.MAGPIE_TREASURES`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:28)
- [ ] `TGTags.SKELETON_DECORATION`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:29)
- [ ] `TGTags.ON_RUINS_EXPLORER_MAPS`: it uses Minecraft's `TagKey` (com/lion/graveyard/init/TGTags.java:30)

### `BoneStaffItem`: it extends Minecraft's `Item`

- [x] `BoneStaffItem.FETCH_DISTANCE_SQ` (com/lion/graveyard/item/BoneStaffItem.java:79)
- [x] `BoneStaffItem.USE_COOLDOWN_TICKS` (com/lion/graveyard/item/BoneStaffItem.java:80)
- [ ] `BoneStaffItem.FETCH_TICKET`: it uses Minecraft's `TicketType` (com/lion/graveyard/item/BoneStaffItem.java:81)
- [x] `BoneStaffItem.MAX_FETCH_MISSES` (com/lion/graveyard/item/BoneStaffItem.java:82)
- [x] `BoneStaffItem.MAX_LANDING_DROP` (com/lion/graveyard/item/BoneStaffItem.java:83)
- [x] `BoneStaffItem.CALL_TIMEOUT` (com/lion/graveyard/item/BoneStaffItem.java:84)
- [x] `BoneStaffItem.ITEM_AIM_DISTANCE` (com/lion/graveyard/item/BoneStaffItem.java:85)
- [x] `BoneStaffItem.ITEM_AIM_MARGIN` (com/lion/graveyard/item/BoneStaffItem.java:86)
- [x] `BoneStaffItem.STORE_TICKS` (com/lion/graveyard/item/BoneStaffItem.java:87)
- [x] `BoneStaffItem.STORE_RANGE` (com/lion/graveyard/item/BoneStaffItem.java:88)
- [x] `BoneStaffItem.STORE_RANGE_SQ` (com/lion/graveyard/item/BoneStaffItem.java:89)
- [x] `BoneStaffItem.DRAW_SOUND_DELAY` (com/lion/graveyard/item/BoneStaffItem.java:90)
- [x] `BoneStaffItem.SIT_CLICK_TICKS` (com/lion/graveyard/item/BoneStaffItem.java:91)
- [x] `BoneStaffItem.MAX_STORED_NBT` (com/lion/graveyard/item/BoneStaffItem.java:92)
- [ ] `BoneStaffItem.DRAWING`: it uses `UUID` (java.util) (com/lion/graveyard/item/BoneStaffItem.java:93)
- [ ] `BoneStaffItem.STORE_DONE_SOUND`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/item/BoneStaffItem.java:95)
- [ ] `BoneStaffItem.BoneStaffItem(2)`: its class isn't translated (it extends Minecraft's `Item`) (com/lion/graveyard/item/BoneStaffItem.java:97)
- [ ] `BoneStaffItem.useOn(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/lion/graveyard/item/BoneStaffItem.java:102)
- [ ] `BoneStaffItem.spawnPosition(3)`: it uses Minecraft's `BlockPos` (com/lion/graveyard/item/BoneStaffItem.java:167)
- [ ] `BoneStaffItem.clipAtBlocks(3)`: it uses Minecraft's `HitResult` (com/lion/graveyard/item/BoneStaffItem.java:171)
- [ ] `BoneStaffItem.isBoundTo(2)`: it uses GhoulingEntity, which isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/item/BoneStaffItem.java:176)
- [ ] `BoneStaffItem.startDrawing(2)`: it uses GhoulingEntity, which isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/item/BoneStaffItem.java:180)
- [ ] `BoneStaffItem.stopDrawSound(2)`: it uses Minecraft's `ServerLevel` (com/lion/graveyard/item/BoneStaffItem.java:184)
- [ ] `BoneStaffItem.streamSouls(4)`: it uses Minecraft's `ServerLevel` (com/lion/graveyard/item/BoneStaffItem.java:192)
- [ ] `BoneStaffItem.release(4)`: it uses Minecraft's `ServerLevel` (com/lion/graveyard/item/BoneStaffItem.java:208)
- [ ] `BoneStaffItem.store(4)`: it uses Minecraft's `ServerLevel` (com/lion/graveyard/item/BoneStaffItem.java:232)
- [ ] `BoneStaffItem.fitsInStack(1)`: it uses GhoulingEntity, which isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/item/BoneStaffItem.java:244)
- [ ] `BoneStaffItem.getUseDuration(2)`: its class isn't translated (it extends Minecraft's `Item`) (com/lion/graveyard/item/BoneStaffItem.java:260)
- [ ] `BoneStaffItem.getUseAnimation(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/lion/graveyard/item/BoneStaffItem.java:264)
- [ ] `BoneStaffItem.onUseTick(4)`: its class isn't translated (it extends Minecraft's `Item`) (com/lion/graveyard/item/BoneStaffItem.java:268)
- [ ] `BoneStaffItem.releaseUsing(4)`: its class isn't translated (it extends Minecraft's `Item`) (com/lion/graveyard/item/BoneStaffItem.java:298)
- [ ] `BoneStaffItem.isFoil(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/lion/graveyard/item/BoneStaffItem.java:319)
- [ ] `BoneStaffItem.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `Item`) (com/lion/graveyard/item/BoneStaffItem.java:323)
- [ ] `BoneStaffItem.findLoaded(2)`: it uses Minecraft's `Entity` (com/lion/graveyard/item/BoneStaffItem.java:338)
- [ ] `BoneStaffItem.isUnbound(1)`: it calls LegacyItemData.ghoulingUuid, which isn't translated (com/lion/graveyard/item/BoneStaffItem.java:349)
- [ ] `BoneStaffItem.rebind(3)`: it uses GhoulingEntity, which isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/item/BoneStaffItem.java:353)
- [ ] `BoneStaffItem.rejectsStaff(3)`: it uses GhoulingEntity, which isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/item/BoneStaffItem.java:370)
- [ ] `BoneStaffItem.ownerAirborne(1)`: it calls Minecraft's LocalPlayer.getAbilities (no Horizon helper for it yet) (com/lion/graveyard/item/BoneStaffItem.java:385)
- [ ] `BoneStaffItem.isOwnedBy(2)`: it uses GhoulingEntity, which isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/item/BoneStaffItem.java:389)
- [ ] `BoneStaffItem.isAlive(2)`: it uses Minecraft's `ServerLevel` (com/lion/graveyard/item/BoneStaffItem.java:393)
- [ ] `BoneStaffItem.inventoryTick(4)`: its class isn't translated (it extends Minecraft's `Item`) (com/lion/graveyard/item/BoneStaffItem.java:404)
- [ ] `BoneStaffItem.call(4)`: it uses Minecraft's `ServerLevel` (com/lion/graveyard/item/BoneStaffItem.java:465)
- [ ] `BoneStaffItem.findLanding(3)`: it uses GhoulingEntity, which isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/item/BoneStaffItem.java:493)
- [ ] `BoneStaffItem.fetch(3)`: it uses GhoulingEntity, which isn't translated (it extends `GraveyardMinionEntity`) (com/lion/graveyard/item/BoneStaffItem.java:519)
- [ ] `BoneStaffItem.findItemInSight(2)`: it uses Minecraft's `ItemEntity` (com/lion/graveyard/item/BoneStaffItem.java:537)
- [ ] `BoneStaffItem.startFarDraw(4)`: it uses Minecraft's `Level` (com/lion/graveyard/item/BoneStaffItem.java:559)
- [ ] `BoneStaffItem.use(3)`: its class isn't translated (it extends Minecraft's `Item`) (com/lion/graveyard/item/BoneStaffItem.java:586)

### `DaggerItem`: it extends Minecraft's `Item`

- [ ] `DaggerItem.DaggerItem(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/lion/graveyard/item/DaggerItem.java:22)
- [ ] `DaggerItem.getDestroySpeed(2)`: its class isn't translated (it extends Minecraft's `Item`) (com/lion/graveyard/item/DaggerItem.java:26)
- [ ] `DaggerItem.hurtEnemy(3)`: its class isn't translated (it extends Minecraft's `Item`) (com/lion/graveyard/item/DaggerItem.java:39)
- [ ] `DaggerItem.fillHotbarVial(1)`: it uses TGItems.VIAL_OF_BLOOD, which isn't translated (it uses Minecraft's `Item`) (com/lion/graveyard/item/DaggerItem.java:57)
- [ ] `DaggerItem.convertHotbarBottle(1)`: it calls Minecraft's ItemStack.shrink (no Horizon helper for it yet) (com/lion/graveyard/item/DaggerItem.java:69)
- [ ] `DaggerItem.giveVial(2)`: it creates Minecraft's `ItemStack` (com/lion/graveyard/item/DaggerItem.java:80)

### `DiaryPageItem`: it extends Minecraft's `BlockItem`

- [ ] `DiaryPageItem.DiaryPageItem(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/lion/graveyard/item/DiaryPageItem.java:25)
- [ ] `DiaryPageItem.of(2)`: it creates Minecraft's `ItemStack` (com/lion/graveyard/item/DiaryPageItem.java:29)
- [ ] `DiaryPageItem.pageOf(1)`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/item/DiaryPageItem.java:35)
- [ ] `DiaryPageItem.use(3)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/lion/graveyard/item/DiaryPageItem.java:42)
- [ ] `DiaryPageItem.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/lion/graveyard/item/DiaryPageItem.java:67)

### `LegacyItemData`

- [x] `LegacyItemData.BLOOD` (com/lion/graveyard/item/LegacyItemData.java:13)
- [x] `LegacyItemData.PAGE` (com/lion/graveyard/item/LegacyItemData.java:14)
- [x] `LegacyItemData.GHOULING_UUID` (com/lion/graveyard/item/LegacyItemData.java:15)
- [x] `LegacyItemData.OWNER_UUID` (com/lion/graveyard/item/LegacyItemData.java:16)
- [x] `LegacyItemData.BOND_ID` (com/lion/graveyard/item/LegacyItemData.java:17)
- [x] `LegacyItemData.FETCH_CALL` (com/lion/graveyard/item/LegacyItemData.java:18)
- [x] `LegacyItemData.FETCH_MISSES` (com/lion/graveyard/item/LegacyItemData.java:19)
- [x] `LegacyItemData.STORED_GHOULING` (com/lion/graveyard/item/LegacyItemData.java:20)
- [ ] `LegacyItemData.NONE`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/item/LegacyItemData.java:21)
- [x] `LegacyItemData.LegacyItemData(0)` (com/lion/graveyard/item/LegacyItemData.java:23)
- [x] `LegacyItemData.hasData(1)` in game_side.rs (com/lion/graveyard/item/LegacyItemData.java:26)
- [ ] `LegacyItemData.view(1)`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/item/LegacyItemData.java:30)
- [ ] `LegacyItemData.edit(2)`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/item/LegacyItemData.java:35)
- [ ] `LegacyItemData.uuid(2)`: it uses `UUID` (java.util) (com/lion/graveyard/item/LegacyItemData.java:48)
- [ ] `LegacyItemData.ownerUuid(1)`: it uses `UUID` (java.util) (com/lion/graveyard/item/LegacyItemData.java:54)
- [ ] `LegacyItemData.ghoulingUuid(1)`: it uses `UUID` (java.util) (com/lion/graveyard/item/LegacyItemData.java:59)
- [ ] `LegacyItemData.bondId(1)`: it uses `UUID` (java.util) (com/lion/graveyard/item/LegacyItemData.java:64)
- [ ] `LegacyItemData.fetchMisses(1)`: it calls LegacyItemData.view, which isn't translated (com/lion/graveyard/item/LegacyItemData.java:69)
- [ ] `LegacyItemData.putUuid(3)`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/item/LegacyItemData.java:73)
- [ ] `LegacyItemData.isStored(1)`: it calls LegacyItemData.view, which isn't translated (com/lion/graveyard/item/LegacyItemData.java:77)
- [ ] `LegacyItemData.store(2)`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/item/LegacyItemData.java:81)
- [ ] `LegacyItemData.clearStored(1)`: it calls LegacyItemData.edit, which isn't translated (com/lion/graveyard/item/LegacyItemData.java:85)
- [ ] `LegacyItemData.clearBond(1)`: it calls LegacyItemData.edit, which isn't translated (com/lion/graveyard/item/LegacyItemData.java:89)
- [ ] `LegacyItemData.clearFetchCall(1)`: it calls LegacyItemData.edit, which isn't translated (com/lion/graveyard/item/LegacyItemData.java:98)
- [ ] `LegacyItemData.clearFetchMisses(1)`: it calls LegacyItemData.edit, which isn't translated (com/lion/graveyard/item/LegacyItemData.java:102)

### `SarcophagusItem`: it extends Minecraft's `BlockItem`

- [ ] `SarcophagusItem.SarcophagusItem(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/lion/graveyard/item/SarcophagusItem.java:10)
- [ ] `SarcophagusItem.placeBlock(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/lion/graveyard/item/SarcophagusItem.java:14)

### `TooltipBlockItem`: it extends Minecraft's `BlockItem`

- [ ] `TooltipBlockItem.TooltipBlockItem(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/lion/graveyard/item/TooltipBlockItem.java:15)
- [ ] `TooltipBlockItem.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/lion/graveyard/item/TooltipBlockItem.java:19)

### `VialOfBlood`: it extends Minecraft's `Item`

- [ ] `VialOfBlood.VialOfBlood(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/lion/graveyard/item/VialOfBlood.java:14)
- [ ] `VialOfBlood.getBlood(1)`: it calls LegacyItemData.view, which isn't translated (com/lion/graveyard/item/VialOfBlood.java:18)
- [ ] `VialOfBlood.setBlood(2)`: it calls LegacyItemData.edit, which isn't translated (com/lion/graveyard/item/VialOfBlood.java:22)
- [ ] `VialOfBlood.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `Item`) (com/lion/graveyard/item/VialOfBlood.java:32)

### `PagesInLootCondition`: it implements Minecraft's `LootItemCondition`

- [x] `PagesInLootCondition.CONFIG_KEY` (com/lion/graveyard/loot/PagesInLootCondition.java:9)
- [ ] `PagesInLootCondition.INSTANCE`: it uses PagesInLootCondition, which isn't translated (it implements Minecraft's `LootItemCondition`) (com/lion/graveyard/loot/PagesInLootCondition.java:10)
- [ ] `PagesInLootCondition.MAP_CODEC`: it uses Minecraft's `MapCodec` (com/lion/graveyard/loot/PagesInLootCondition.java:11)
- [ ] `PagesInLootCondition.PagesInLootCondition(0)`: its class isn't translated (it implements Minecraft's `LootItemCondition`) (com/lion/graveyard/loot/PagesInLootCondition.java:13)
- [ ] `PagesInLootCondition.codec(0)`: its class isn't translated (it implements Minecraft's `LootItemCondition`) (com/lion/graveyard/loot/PagesInLootCondition.java:16)
- [ ] `PagesInLootCondition.test(1)`: its class isn't translated (it implements Minecraft's `LootItemCondition`) (com/lion/graveyard/loot/PagesInLootCondition.java:20)
- [ ] `BellBlockEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:15)
- [ ] `BlockEntityTypeMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:11)
- [ ] `ClientPlayNetworkHandlerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:21)
- [ ] `HandledScreenAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:8)
- [ ] `SoundEngineMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:15)
- [ ] `CrafterBlockMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:16)
- [ ] `CraftingResultSlotMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:11)
- [ ] `CraftingScreenHandlerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:26)
- [ ] `CreeperEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:11)
- [ ] `EntityChunkUpgradeMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:15)
- [ ] `InventoryChangedCriterionMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:18)
- [ ] `LevelStorageMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:10)
- [ ] `MobEntityAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:9)
- [ ] `MobEntitySpawnMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:17)
- [ ] `MoonriseEntityChunkMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:15)
- [ ] `PotionEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:14)
- [ ] `ServerPlayNetworkHandlerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:18)
- [ ] `SinglePoolElementMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:27)
- [ ] `TreeFeatureMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/lion/graveyard/loot/PagesInLootCondition.java:17)

### `GraveyardFogParticle`: it extends Minecraft's `SingleQuadParticle`

- [x] `GraveyardFogParticle.PEAK_ALPHA_MIN` (com/lion/graveyard/particles/GraveyardFogParticle.java:22)
- [x] `GraveyardFogParticle.PEAK_ALPHA_RANGE` (com/lion/graveyard/particles/GraveyardFogParticle.java:23)
- [x] `GraveyardFogParticle.FADE_IN_END` (com/lion/graveyard/particles/GraveyardFogParticle.java:24)
- [x] `GraveyardFogParticle.FADE_OUT_START` (com/lion/graveyard/particles/GraveyardFogParticle.java:25)
- [x] `GraveyardFogParticle.GROWTH` (com/lion/graveyard/particles/GraveyardFogParticle.java:26)
- [x] `GraveyardFogParticle.MAX_TILT` (com/lion/graveyard/particles/GraveyardFogParticle.java:27)
- [x] `GraveyardFogParticle.FLAT_ROTATION` (com/lion/graveyard/particles/GraveyardFogParticle.java:28)
- [ ] `GraveyardFogParticle.GraveyardFogParticle(5)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (com/lion/graveyard/particles/GraveyardFogParticle.java:37)
- [ ] `GraveyardFogParticle.getLayer(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (com/lion/graveyard/particles/GraveyardFogParticle.java:57)
- [ ] `GraveyardFogParticle.tick(0)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (com/lion/graveyard/particles/GraveyardFogParticle.java:61)
- [ ] `GraveyardFogParticle.extract(3)`: its class isn't translated (it extends Minecraft's `SingleQuadParticle`) (com/lion/graveyard/particles/GraveyardFogParticle.java:77)

### `GraveyardFogParticle.FogFactory`: it implements Minecraft's `ParticleProvider`

- [ ] `GraveyardFogParticle.FogFactory.FogFactory(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (com/lion/graveyard/particles/GraveyardFogParticle.java:105)
- [ ] `GraveyardFogParticle.FogFactory.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (com/lion/graveyard/particles/GraveyardFogParticle.java:109)

### `GraveyardHandParticle`: it extends Minecraft's `RisingParticle`

- [ ] `GraveyardHandParticle.GraveyardHandParticle(8)`: its class isn't translated (it extends Minecraft's `RisingParticle`) (com/lion/graveyard/particles/GraveyardHandParticle.java:19)
- [ ] `GraveyardHandParticle.getLightCoords(1)`: its class isn't translated (it extends Minecraft's `RisingParticle`) (com/lion/graveyard/particles/GraveyardHandParticle.java:29)
- [ ] `GraveyardHandParticle.getLayer(0)`: its class isn't translated (it extends Minecraft's `RisingParticle`) (com/lion/graveyard/particles/GraveyardHandParticle.java:33)
- [ ] `GraveyardHandParticle.tick(0)`: its class isn't translated (it extends Minecraft's `RisingParticle`) (com/lion/graveyard/particles/GraveyardHandParticle.java:37)

### `GraveyardHandParticle.Factory`: it implements Minecraft's `ParticleProvider`

- [ ] `GraveyardHandParticle.Factory.Factory(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (com/lion/graveyard/particles/GraveyardHandParticle.java:46)
- [ ] `GraveyardHandParticle.Factory.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (com/lion/graveyard/particles/GraveyardHandParticle.java:50)

### `GraveyardLineFogParticle`: it extends Minecraft's `RisingParticle`

- [ ] `GraveyardLineFogParticle.GraveyardLineFogParticle(8)`: its class isn't translated (it extends Minecraft's `RisingParticle`) (com/lion/graveyard/particles/GraveyardLineFogParticle.java:19)
- [ ] `GraveyardLineFogParticle.getLightCoords(1)`: its class isn't translated (it extends Minecraft's `RisingParticle`) (com/lion/graveyard/particles/GraveyardLineFogParticle.java:29)
- [ ] `GraveyardLineFogParticle.getLayer(0)`: its class isn't translated (it extends Minecraft's `RisingParticle`) (com/lion/graveyard/particles/GraveyardLineFogParticle.java:33)
- [ ] `GraveyardLineFogParticle.tick(0)`: its class isn't translated (it extends Minecraft's `RisingParticle`) (com/lion/graveyard/particles/GraveyardLineFogParticle.java:37)

### `GraveyardLineFogParticle.Factory`: it implements Minecraft's `ParticleProvider`

- [ ] `GraveyardLineFogParticle.Factory.Factory(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (com/lion/graveyard/particles/GraveyardLineFogParticle.java:46)
- [ ] `GraveyardLineFogParticle.Factory.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (com/lion/graveyard/particles/GraveyardLineFogParticle.java:50)

### `GraveyardSoulParticle`: it extends Minecraft's `RisingParticle`

- [ ] `GraveyardSoulParticle.GraveyardSoulParticle(8)`: its class isn't translated (it extends Minecraft's `RisingParticle`) (com/lion/graveyard/particles/GraveyardSoulParticle.java:19)
- [ ] `GraveyardSoulParticle.getLightCoords(1)`: its class isn't translated (it extends Minecraft's `RisingParticle`) (com/lion/graveyard/particles/GraveyardSoulParticle.java:26)
- [ ] `GraveyardSoulParticle.getLayer(0)`: its class isn't translated (it extends Minecraft's `RisingParticle`) (com/lion/graveyard/particles/GraveyardSoulParticle.java:30)
- [ ] `GraveyardSoulParticle.tick(0)`: its class isn't translated (it extends Minecraft's `RisingParticle`) (com/lion/graveyard/particles/GraveyardSoulParticle.java:34)

### `GraveyardSoulParticle.Factory`: it implements Minecraft's `ParticleProvider`

- [ ] `GraveyardSoulParticle.Factory.Factory(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (com/lion/graveyard/particles/GraveyardSoulParticle.java:43)
- [ ] `GraveyardSoulParticle.Factory.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (com/lion/graveyard/particles/GraveyardSoulParticle.java:47)

### `GraveyardSoulParticle.SculkSoulFactory`: it implements Minecraft's `ParticleProvider`

- [ ] `GraveyardSoulParticle.SculkSoulFactory.SculkSoulFactory(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (com/lion/graveyard/particles/GraveyardSoulParticle.java:60)
- [ ] `GraveyardSoulParticle.SculkSoulFactory.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (com/lion/graveyard/particles/GraveyardSoulParticle.java:64)

### `BiomeModifications`

- [ ] `BiomeModifications.addMobSpawn(6)`: it uses Minecraft's `TagKey` (com/lion/graveyard/platform/BiomeModifications.java:15)
- [ ] `BiomeModifications.addPlacedFeature(3)`: it uses Minecraft's `TagKey` (com/lion/graveyard/platform/BiomeModifications.java:21)

### `ChronicleSync`: it implements Minecraft's `CustomPacketPayload`

- [ ] `ChronicleSync.TYPE`: it uses Type, which isn't translated (it implements Minecraft's `RecipeType`) (com/lion/graveyard/platform/ChronicleSync.java:20)
- [ ] `ChronicleSync.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/lion/graveyard/platform/ChronicleSync.java:21)
- [ ] `ChronicleSync.ChronicleSync(2)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/lion/graveyard/platform/ChronicleSync.java:19)
- [ ] `ChronicleSync.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/lion/graveyard/platform/ChronicleSync.java:23)
- [ ] `ChronicleSync.send(3)`: it uses Minecraft's `ServerPlayer` (com/lion/graveyard/platform/ChronicleSync.java:27)
- [ ] `ChronicleSync.write(2)`: it uses Minecraft's `FriendlyByteBuf` (com/lion/graveyard/platform/ChronicleSync.java:33)
- [ ] `ChronicleSync.read(1)`: it uses ChronicleSync, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/lion/graveyard/platform/ChronicleSync.java:44)
- [ ] `ChronicleSync.found(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/lion/graveyard/platform/ChronicleSync.java:19)
- [ ] `ChronicleSync.newlyFound(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/lion/graveyard/platform/ChronicleSync.java:19)

### `ConfigDirectory`

- [x] `ConfigDirectory.getConfigDirectory(0)` (com/lion/graveyard/platform/ConfigDirectory.java:9)

### `DevEnvironment`

- [x] `DevEnvironment.isDevelopment(0)` (com/lion/graveyard/platform/DevEnvironment.java:8)

### `BiomeModificationsImpl`

- [ ] `BiomeModificationsImpl.addMobSpawn(6)`: it uses Minecraft's `TagKey` (com/lion/graveyard/platform/fabric/BiomeModificationsImpl.java:14)
- [ ] `BiomeModificationsImpl.addPlacedFeature(3)`: it uses Minecraft's `TagKey` (com/lion/graveyard/platform/fabric/BiomeModificationsImpl.java:20)

### `ChronicleSyncImpl`

- [x] `ChronicleSyncImpl.ChronicleSyncImpl(0)` (com/lion/graveyard/platform/fabric/ChronicleSyncImpl.java:11)
- [ ] `ChronicleSyncImpl.send(3)`: it uses Minecraft's `ServerPlayer` (com/lion/graveyard/platform/fabric/ChronicleSyncImpl.java:14)

### `ConfigDirectoryImpl`

- [x] `ConfigDirectoryImpl.getConfigDirectory(0)` (com/lion/graveyard/platform/fabric/ConfigDirectoryImpl.java:7)

### `DevEnvironmentImpl`

- [x] `DevEnvironmentImpl.isDevelopment(0)` (com/lion/graveyard/platform/fabric/DevEnvironmentImpl.java:6)

### `HordeSpawner`

- [ ] `HordeSpawner.spawners`: it uses GraveyardHordeSpawner, which isn't translated (it implements Minecraft's `CustomSpawner`) (com/lion/graveyard/platform/fabric/HordeSpawner.java:15)
- [x] `HordeSpawner.loaded` (com/lion/graveyard/platform/fabric/HordeSpawner.java:16)
- [ ] `HordeSpawner.getSpawners(0)`: it uses GraveyardHordeSpawner, which isn't translated (it implements Minecraft's `CustomSpawner`) (com/lion/graveyard/platform/fabric/HordeSpawner.java:18)
- [ ] `HordeSpawner.addToSpawners(2)`: it uses GraveyardHordeSpawner, which isn't translated (it implements Minecraft's `CustomSpawner`) (com/lion/graveyard/platform/fabric/HordeSpawner.java:22)

### `HordeSpawner.OnWorldTick`: it implements Fabric's `EndLevelTick`

- [ ] `HordeSpawner.OnWorldTick.onEndTick(1)`: its class isn't translated (it implements Fabric's `EndLevelTick`) (com/lion/graveyard/platform/fabric/HordeSpawner.java:31)

### `HordeSpawner.ServerStopped`: it implements `ServerStopped`

- [ ] `HordeSpawner.ServerStopped.onServerStopped(1)`: its class isn't translated (it implements `ServerStopped`) (com/lion/graveyard/platform/fabric/HordeSpawner.java:46)

### `HordeSpawner.WorldLoad`: it implements Fabric's `Load`

- [ ] `HordeSpawner.WorldLoad.onLevelLoad(2)`: its class isn't translated (it implements Fabric's `Load`) (com/lion/graveyard/platform/fabric/HordeSpawner.java:53)

### `NamelessHangedTradeOfferResourceListener`: it extends Minecraft's `SimpleJsonResourceReloadListener`

- [ ] `NamelessHangedTradeOfferResourceListener.NamelessHangedTradeOfferResourceListener(0)`: its class isn't translated (it extends Minecraft's `SimpleJsonResourceReloadListener`) (com/lion/graveyard/platform/fabric/NamelessHangedTradeOfferResourceListener.java:17)
- [ ] `NamelessHangedTradeOfferResourceListener.getFabricId(0)`: its class isn't translated (it extends Minecraft's `SimpleJsonResourceReloadListener`) (com/lion/graveyard/platform/fabric/NamelessHangedTradeOfferResourceListener.java:21)
- [ ] `NamelessHangedTradeOfferResourceListener.prepare(2)`: its class isn't translated (it extends Minecraft's `SimpleJsonResourceReloadListener`) (com/lion/graveyard/platform/fabric/NamelessHangedTradeOfferResourceListener.java:25)
- [ ] `NamelessHangedTradeOfferResourceListener.apply(3)`: its class isn't translated (it extends Minecraft's `SimpleJsonResourceReloadListener`) (com/lion/graveyard/platform/fabric/NamelessHangedTradeOfferResourceListener.java:29)

### `RegistryHelperImpl`

- [ ] `RegistryHelperImpl.registerBlock(3)`: it's generic (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:56)
- [ ] `RegistryHelperImpl.registerItem(3)`: it's generic (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:62)
- [ ] `RegistryHelperImpl.registerItemGroup(4)`: it uses Minecraft's `ResourceKey` (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:70)
- [ ] `RegistryHelperImpl.registerItemGroupWithIcon(4)`: it uses Minecraft's `ResourceKey` (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:78)
- [ ] `RegistryHelperImpl.addToItemGroup(2)`: it uses Minecraft's `ResourceKey` (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:84)
- [ ] `RegistryHelperImpl.addStacksToItemGroup(2)`: it uses Minecraft's `ResourceKey` (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:88)
- [ ] `RegistryHelperImpl.registerSpawnEggItem(3)`: it's generic (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:92)
- [ ] `RegistryHelperImpl.registerMusicDiscItem(2)`: it's generic (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:98)
- [ ] `RegistryHelperImpl.registerBlockEntity(2)`: it's generic (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:102)
- [ ] `RegistryHelperImpl.registerStructureType(2)`: it's generic (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:107)
- [ ] `RegistryHelperImpl.registerParticleType(2)`: it uses Minecraft's `SimpleParticleType` (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:111)
- [ ] `RegistryHelperImpl.registerStructureProcessor(2)`: it uses Minecraft's `MapCodec` (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:115)
- [ ] `RegistryHelperImpl.registerCriterion(2)`: it uses Minecraft's `CriterionTrigger` (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:119)
- [ ] `RegistryHelperImpl.registerDataComponent(2)`: it's generic (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:123)
- [ ] `RegistryHelperImpl.registerSpawnPlacement(4)`: it's generic (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:128)
- [ ] `RegistryHelperImpl.registerScreenHandlerType(2)`: it uses Minecraft's `MenuType` (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:134)
- [ ] `RegistryHelperImpl.registerSoundEvent(2)`: it's generic (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:138)
- [ ] `RegistryHelperImpl.registerEntityRenderer(2)`: it's generic (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:143)
- [ ] `RegistryHelperImpl.registerBlockEntityRenderer(2)`: it's generic (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:147)
- [ ] `RegistryHelperImpl.registerEntityType(2)`: it's generic (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:153)
- [ ] `RegistryHelperImpl.registerEntityModelLayer(2)`: it uses Minecraft's `ModelLayerLocation` (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:159)
- [ ] `RegistryHelperImpl.registerEntityAttribute(2)`: it uses Minecraft's `EntityType` (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:163)
- [ ] `RegistryHelperImpl.registerRecipeType(2)`: it's generic (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:169)
- [ ] `RegistryHelperImpl.registerRecipeSerializer(2)`: it's generic (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:174)
- [ ] `RegistryHelperImpl.registerLootCondition(2)`: it's generic (com/lion/graveyard/platform/fabric/RegistryHelperImpl.java:179)

### `ServerEvents`


### `ServerEvents.DeleteOldConfigs`: it implements Fabric's `ServerStarted`

- [ ] `ServerEvents.DeleteOldConfigs.onServerStarted(1)`: its class isn't translated (it implements Fabric's `ServerStarted`) (com/lion/graveyard/platform/fabric/ServerEvents.java:12)

### `OssuaryRecipesSync`: it implements Minecraft's `CustomPacketPayload`

- [ ] `OssuaryRecipesSync.TYPE`: it uses Type, which isn't translated (it implements Minecraft's `RecipeType`) (com/lion/graveyard/platform/OssuaryRecipesSync.java:15)
- [ ] `OssuaryRecipesSync.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/lion/graveyard/platform/OssuaryRecipesSync.java:16)
- [ ] `OssuaryRecipesSync.OssuaryRecipesSync(2)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/lion/graveyard/platform/OssuaryRecipesSync.java:14)
- [ ] `OssuaryRecipesSync.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/lion/graveyard/platform/OssuaryRecipesSync.java:24)
- [ ] `OssuaryRecipesSync.send(3)`: it uses Minecraft's `ServerPlayer` (com/lion/graveyard/platform/OssuaryRecipesSync.java:28)
- [ ] `OssuaryRecipesSync.containerId(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/lion/graveyard/platform/OssuaryRecipesSync.java:14)
- [ ] `OssuaryRecipesSync.results(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/lion/graveyard/platform/OssuaryRecipesSync.java:14)

### `RegistryHelper`

- [ ] `RegistryHelper.registerBlock(3)`: it's generic (com/lion/graveyard/platform/RegistryHelper.java:45)
- [ ] `RegistryHelper.registerBlockEntity(2)`: it's generic (com/lion/graveyard/platform/RegistryHelper.java:51)
- [ ] `RegistryHelper.registerItemGroup(4)`: it uses Minecraft's `ResourceKey` (com/lion/graveyard/platform/RegistryHelper.java:57)
- [ ] `RegistryHelper.registerItem(3)`: it's generic (com/lion/graveyard/platform/RegistryHelper.java:63)
- [ ] `RegistryHelper.registerItemGroupWithIcon(4)`: it uses Minecraft's `ResourceKey` (com/lion/graveyard/platform/RegistryHelper.java:71)
- [ ] `RegistryHelper.addToItemGroup(2)`: it uses Minecraft's `ResourceKey` (com/lion/graveyard/platform/RegistryHelper.java:77)
- [ ] `RegistryHelper.addStacksToItemGroup(2)`: it uses Minecraft's `ResourceKey` (com/lion/graveyard/platform/RegistryHelper.java:83)
- [ ] `RegistryHelper.registerSpawnEggItem(3)`: it's generic (com/lion/graveyard/platform/RegistryHelper.java:89)
- [ ] `RegistryHelper.registerMusicDiscItem(2)`: it's generic (com/lion/graveyard/platform/RegistryHelper.java:97)
- [ ] `RegistryHelper.registerStructureType(2)`: it's generic (com/lion/graveyard/platform/RegistryHelper.java:103)
- [ ] `RegistryHelper.registerParticleType(2)`: it uses Minecraft's `SimpleParticleType` (com/lion/graveyard/platform/RegistryHelper.java:109)
- [ ] `RegistryHelper.registerStructureProcessor(2)`: it uses Minecraft's `MapCodec` (com/lion/graveyard/platform/RegistryHelper.java:115)
- [ ] `RegistryHelper.registerCriterion(2)`: it uses Minecraft's `CriterionTrigger` (com/lion/graveyard/platform/RegistryHelper.java:121)
- [ ] `RegistryHelper.registerScreenHandlerType(2)`: it uses Minecraft's `MenuType` (com/lion/graveyard/platform/RegistryHelper.java:127)
- [ ] `RegistryHelper.registerSoundEvent(2)`: it's generic (com/lion/graveyard/platform/RegistryHelper.java:133)
- [ ] `RegistryHelper.registerEntityType(2)`: it's generic (com/lion/graveyard/platform/RegistryHelper.java:139)
- [ ] `RegistryHelper.registerEntityAttribute(2)`: it uses Minecraft's `EntityType` (com/lion/graveyard/platform/RegistryHelper.java:145)
- [ ] `RegistryHelper.registerEntityModelLayer(2)`: it uses Minecraft's `ModelLayerLocation` (com/lion/graveyard/platform/RegistryHelper.java:153)
- [ ] `RegistryHelper.registerEntityRenderer(2)`: it's generic (com/lion/graveyard/platform/RegistryHelper.java:159)
- [ ] `RegistryHelper.registerBlockEntityRenderer(2)`: it's generic (com/lion/graveyard/platform/RegistryHelper.java:165)
- [ ] `RegistryHelper.registerRecipeType(2)`: it's generic (com/lion/graveyard/platform/RegistryHelper.java:173)
- [ ] `RegistryHelper.registerRecipeSerializer(2)`: it's generic (com/lion/graveyard/platform/RegistryHelper.java:179)
- [ ] `RegistryHelper.registerDataComponent(2)`: it's generic (com/lion/graveyard/platform/RegistryHelper.java:185)
- [ ] `RegistryHelper.registerSpawnPlacement(4)`: it's generic (com/lion/graveyard/platform/RegistryHelper.java:191)
- [ ] `RegistryHelper.registerLootCondition(2)`: it's generic (com/lion/graveyard/platform/RegistryHelper.java:199)

### `CarvingRecipe`: it implements Minecraft's `Recipe`

- [ ] `CarvingRecipe.INGREDIENT_CODEC`: it uses Minecraft's `Codec` (com/lion/graveyard/recipe/CarvingRecipe.java:32)
- [ ] `CarvingRecipe.CarvingRecipe(5)`: its class isn't translated (it implements Minecraft's `Recipe`) (com/lion/graveyard/recipe/CarvingRecipe.java:40)
- [ ] `CarvingRecipe.getType(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (com/lion/graveyard/recipe/CarvingRecipe.java:54)
- [ ] `CarvingRecipe.getSerializer(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (com/lion/graveyard/recipe/CarvingRecipe.java:58)
- [ ] `CarvingRecipe.group(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (com/lion/graveyard/recipe/CarvingRecipe.java:62)
- [ ] `CarvingRecipe.placementInfo(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (com/lion/graveyard/recipe/CarvingRecipe.java:66)
- [ ] `CarvingRecipe.recipeBookCategory(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (com/lion/graveyard/recipe/CarvingRecipe.java:74)
- [ ] `CarvingRecipe.getOutputItem(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (com/lion/graveyard/recipe/CarvingRecipe.java:78)
- [ ] `CarvingRecipe.getOutput(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (com/lion/graveyard/recipe/CarvingRecipe.java:82)
- [ ] `CarvingRecipe.assemble(1)`: its class isn't translated (it implements Minecraft's `Recipe`) (com/lion/graveyard/recipe/CarvingRecipe.java:86)
- [ ] `CarvingRecipe.showNotification(0)`: its class isn't translated (it implements Minecraft's `Recipe`) (com/lion/graveyard/recipe/CarvingRecipe.java:90)
- [ ] `CarvingRecipe.notAir(1)`: it uses Minecraft's `DataResult` (com/lion/graveyard/recipe/CarvingRecipe.java:94)
- [ ] `CarvingRecipe.serializer(1)`: it's generic (com/lion/graveyard/recipe/CarvingRecipe.java:98)

### `CarvingRecipe.LegacyIngredientCodec`: it implements Minecraft's `Codec`

- [ ] `CarvingRecipe.LegacyIngredientCodec.decode(2)`: it's generic (com/lion/graveyard/recipe/CarvingRecipe.java:121)
- [ ] `CarvingRecipe.LegacyIngredientCodec.encode(3)`: it's generic (com/lion/graveyard/recipe/CarvingRecipe.java:125)
- [ ] `CarvingRecipe.LegacyIngredientCodec.toStringForm(2)`: it's generic (com/lion/graveyard/recipe/CarvingRecipe.java:129)
- [ ] `CarvingRecipe.LegacyIngredientCodec.fromObject(2)`: it's generic (com/lion/graveyard/recipe/CarvingRecipe.java:149)

### `CarvingRecipe.RecipeFactory`: it's an interface

- [ ] `CarvingRecipe.RecipeFactory.create(3)`: it has no body (com/lion/graveyard/recipe/CarvingRecipe.java:161)

### `OssuaryRecipe`: it extends `CarvingRecipe`

- [ ] `OssuaryRecipe.OssuaryRecipe(3)`: its class isn't translated (it extends `CarvingRecipe`) (com/lion/graveyard/recipe/OssuaryRecipe.java:10)
- [ ] `OssuaryRecipe.matches(2)`: its class isn't translated (it extends `CarvingRecipe`) (com/lion/graveyard/recipe/OssuaryRecipe.java:14)
- [ ] `OssuaryRecipe.isSpecial(0)`: its class isn't translated (it extends `CarvingRecipe`) (com/lion/graveyard/recipe/OssuaryRecipe.java:18)

### `BossMusicPlayer`

- [ ] `BossMusicPlayer.music`: it uses LichBossMusic, which isn't translated (it extends Minecraft's `AbstractSoundInstance`) (com/lion/graveyard/sounds/BossMusicPlayer.java:11)
- [ ] `BossMusicPlayer.playBossMusic(1)`: it uses LichEntity, which isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/sounds/BossMusicPlayer.java:13)
- [ ] `BossMusicPlayer.stopBossMusic(1)`: it uses LichEntity, which isn't translated (it extends Minecraft's `Monster`) (com/lion/graveyard/sounds/BossMusicPlayer.java:39)

### `LichBossMusic`: it extends Minecraft's `AbstractSoundInstance`

- [ ] `LichBossMusic.LichBossMusic(3)`: its class isn't translated (it extends Minecraft's `AbstractSoundInstance`) (com/lion/graveyard/sounds/LichBossMusic.java:20)
- [ ] `LichBossMusic.canPlaySound(0)`: its class isn't translated (it extends Minecraft's `AbstractSoundInstance`) (com/lion/graveyard/sounds/LichBossMusic.java:34)
- [ ] `LichBossMusic.isStopped(0)`: its class isn't translated (it extends Minecraft's `AbstractSoundInstance`) (com/lion/graveyard/sounds/LichBossMusic.java:38)
- [ ] `LichBossMusic.setDone(0)`: its class isn't translated (it extends Minecraft's `AbstractSoundInstance`) (com/lion/graveyard/sounds/LichBossMusic.java:42)
- [ ] `LichBossMusic.isAudibleBoss(0)`: its class isn't translated (it extends Minecraft's `AbstractSoundInstance`) (com/lion/graveyard/sounds/LichBossMusic.java:46)
- [ ] `LichBossMusic.tick(0)`: its class isn't translated (it extends Minecraft's `AbstractSoundInstance`) (com/lion/graveyard/sounds/LichBossMusic.java:55)

### `NamelessHangedTrades`

- [x] `NamelessHangedTrades.PROFESSION` (com/lion/graveyard/trades/NamelessHangedTrades.java:13)
- [x] `NamelessHangedTrades.PENDING_TRADES` (com/lion/graveyard/trades/NamelessHangedTrades.java:14)
- [ ] `NamelessHangedTrades.factories`: it uses `TradeFactory` (com.lion.villagertradingplus.tradeoffers) (com/lion/graveyard/trades/NamelessHangedTrades.java:15)
- [x] `NamelessHangedTrades.NamelessHangedTrades(0)` (com/lion/graveyard/trades/NamelessHangedTrades.java:17)
- [ ] `NamelessHangedTrades.clear(0)`: it uses NamelessHangedTrades.factories, which isn't translated (it uses `TradeFactory` (com.lion.villagertradingplus.tradeoffers)) (com/lion/graveyard/trades/NamelessHangedTrades.java:20)
- [ ] `NamelessHangedTrades.factories(0)`: it uses `TradeFactory` (com.lion.villagertradingplus.tradeoffers) (com/lion/graveyard/trades/NamelessHangedTrades.java:25)
- [ ] `NamelessHangedTrades.deserializeJson(1)`: it calls Graveyard.getLogger, which isn't translated (com/lion/graveyard/trades/NamelessHangedTrades.java:42)
- [ ] `NamelessHangedTrades.parse(1)`: it uses `TradeFactory` (com.lion.villagertradingplus.tradeoffers) (com/lion/graveyard/trades/NamelessHangedTrades.java:66)

### `ChronicleCommand`

- [x] `ChronicleCommand.ChronicleCommand(0)` (com/lion/graveyard/util/ChronicleCommand.java:16)
- [ ] `ChronicleCommand.register(1)`: it uses Minecraft's `CommandDispatcher` (com/lion/graveyard/util/ChronicleCommand.java:19)

### `CreeperExplosionAccessor`

- [ ] `CreeperExplosionAccessor.explode(2)`: it uses Minecraft's `Creeper` (com/lion/graveyard/util/CreeperExplosionAccessor.java:8)

### `CrowAnchors`

- [x] `CrowAnchors.RADIUS` (com/lion/graveyard/util/CrowAnchors.java:21)
- [x] `CrowAnchors.STEP` (com/lion/graveyard/util/CrowAnchors.java:22)
- [x] `CrowAnchors.MIN_FIELD_HITS` (com/lion/graveyard/util/CrowAnchors.java:23)
- [x] `CrowAnchors.MIN_LANDMARK_HITS` (com/lion/graveyard/util/CrowAnchors.java:24)
- [x] `CrowAnchors.DEATH_MEMORY_TICKS` (com/lion/graveyard/util/CrowAnchors.java:25)
- [x] `CrowAnchors.MAX_DEATH_SITES` (com/lion/graveyard/util/CrowAnchors.java:26)
- [x] `CrowAnchors.DEATH_SEARCH_RADIUS_SQ` (com/lion/graveyard/util/CrowAnchors.java:27)
- [ ] `CrowAnchors.DEATH_SITES`: it uses Minecraft's `ResourceKey` (com/lion/graveyard/util/CrowAnchors.java:28)
- [x] `CrowAnchors.CrowAnchors(0)` (com/lion/graveyard/util/CrowAnchors.java:30)
- [ ] `CrowAnchors.recordDeath(2)`: it uses Minecraft's `Level` (com/lion/graveyard/util/CrowAnchors.java:33)
- [ ] `CrowAnchors.find(2)`: it uses Anchor, which isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/util/CrowAnchors.java:44)
- [ ] `CrowAnchors.findDeathSite(2)`: it uses Minecraft's `BlockPos` (com/lion/graveyard/util/CrowAnchors.java:112)
- [ ] `CrowAnchors.surface(3)`: it uses Minecraft's `BlockPos` (com/lion/graveyard/util/CrowAnchors.java:140)

### `CrowAnchors.Anchor`: it uses Minecraft's `BlockPos`

- [ ] `CrowAnchors.Anchor.Anchor(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/util/CrowAnchors.java:144)
- [ ] `CrowAnchors.Anchor.pos(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/util/CrowAnchors.java:144)
- [ ] `CrowAnchors.Anchor.kind(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/util/CrowAnchors.java:144)

### `CrowAnchors.DeathSite`: it uses Minecraft's `BlockPos`

- [ ] `CrowAnchors.DeathSite.DeathSite(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/util/CrowAnchors.java:147)
- [ ] `CrowAnchors.DeathSite.pos(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/util/CrowAnchors.java:147)
- [ ] `CrowAnchors.DeathSite.expiresAt(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/util/CrowAnchors.java:147)

### `CrowAnchors.Kind`


### `EntityChunkUpgrade`

- [x] `EntityChunkUpgrade.EntityChunkUpgrade(0)` (com/lion/graveyard/util/EntityChunkUpgrade.java:10)
- [ ] `EntityChunkUpgrade.upgradeWithoutGraveyardEntities(3)`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/util/EntityChunkUpgrade.java:13)
- [ ] `EntityChunkUpgrade.upgradeWithoutGraveyardEntities(4)`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/util/EntityChunkUpgrade.java:17)
- [ ] `EntityChunkUpgrade.hasModEntity(1)`: it uses Minecraft's `ListTag` (com/lion/graveyard/util/EntityChunkUpgrade.java:43)
- [ ] `EntityChunkUpgrade.isModEntity(1)`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/util/EntityChunkUpgrade.java:53)

### `GravestoneIdentifier`

- [x] `GravestoneIdentifier.POLISHED_BASALT_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:7)
- [x] `GravestoneIdentifier.COBBLESTONE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:8)
- [x] `GravestoneIdentifier.MOSSY_COBBLESTONE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:9)
- [x] `GravestoneIdentifier.DEEPSLATE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:10)
- [x] `GravestoneIdentifier.ANCHOR_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:11)
- [x] `GravestoneIdentifier.ANDESITE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:12)
- [x] `GravestoneIdentifier.BLACKSTONE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:13)
- [x] `GravestoneIdentifier.BLACKSTONE_BRICKS_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:14)
- [x] `GravestoneIdentifier.BRICKS_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:15)
- [x] `GravestoneIdentifier.CHISELED_BLACKSTONE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:16)
- [x] `GravestoneIdentifier.CHISELED_QUARTZ_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:17)
- [x] `GravestoneIdentifier.CHISELED_RED_SANDSTONE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:18)
- [x] `GravestoneIdentifier.CHISELED_SANDSTONE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:19)
- [x] `GravestoneIdentifier.CRACKED_BLACKSTONE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:20)
- [x] `GravestoneIdentifier.CRACKED_STONE_BRICKS_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:21)
- [x] `GravestoneIdentifier.DARK_PRISMARINE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:22)
- [x] `GravestoneIdentifier.DIORITE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:23)
- [x] `GravestoneIdentifier.GILDED_BLACKSTONE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:24)
- [x] `GravestoneIdentifier.GRANITE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:25)
- [x] `GravestoneIdentifier.IRON_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:26)
- [x] `GravestoneIdentifier.LODESTONE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:27)
- [x] `GravestoneIdentifier.MOSSY_STONE_BRICKS_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:28)
- [x] `GravestoneIdentifier.NETHERBRICKS_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:29)
- [x] `GravestoneIdentifier.NETHERITE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:30)
- [x] `GravestoneIdentifier.POLISHED_ANDESITE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:31)
- [x] `GravestoneIdentifier.POLISHED_DIORITE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:32)
- [x] `GravestoneIdentifier.POLISHED_BLACKSTONE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:33)
- [x] `GravestoneIdentifier.POLISHED_GRANITE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:34)
- [x] `GravestoneIdentifier.PRISMARINE_BRICKS_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:35)
- [x] `GravestoneIdentifier.QUARTZ_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:36)
- [x] `GravestoneIdentifier.QUARTZ_BRICKS_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:37)
- [x] `GravestoneIdentifier.QUARTZ_PILLAR_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:38)
- [x] `GravestoneIdentifier.RED_NETHERBRICKS_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:39)
- [x] `GravestoneIdentifier.RED_SANDSTONE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:40)
- [x] `GravestoneIdentifier.SANDSTONE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:41)
- [x] `GravestoneIdentifier.STONE_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:42)
- [x] `GravestoneIdentifier.STONE_BRICKS_GRAVESTONE_TEXTURE` (com/lion/graveyard/util/GravestoneIdentifier.java:43)

### `LegacyContainerName`

- [x] `LegacyContainerName.CUSTOM_NAME_KEY` (com/lion/graveyard/util/LegacyContainerName.java:20)
- [x] `LegacyContainerName.LegacyContainerName(0)` (com/lion/graveyard/util/LegacyContainerName.java:22)
- [ ] `LegacyContainerName.read(1)`: it uses Minecraft's `ValueInput` (com/lion/graveyard/util/LegacyContainerName.java:25)
- [x] `LegacyContainerName.looksLikeJson(1)` (com/lion/graveyard/util/LegacyContainerName.java:42)
- [ ] `LegacyContainerName.runFixer(3)`: it uses Minecraft's `Provider` (com/lion/graveyard/util/LegacyContainerName.java:46)
- [ ] `LegacyContainerName.parseJson(2)`: it uses Minecraft's `Provider` (com/lion/graveyard/util/LegacyContainerName.java:63)

### `LegacyItemStacks`

- [x] `LegacyItemStacks.LEGACY_DATA_VERSION` (com/lion/graveyard/util/LegacyItemStacks.java:19)
- [x] `LegacyItemStacks.PORT_1_21_1_DATA_VERSION` (com/lion/graveyard/util/LegacyItemStacks.java:20)
- [x] `LegacyItemStacks.UNKNOWN_VERSION` (com/lion/graveyard/util/LegacyItemStacks.java:21)
- [x] `LegacyItemStacks.DATA_VERSION_KEY` (com/lion/graveyard/util/LegacyItemStacks.java:22)
- [x] `LegacyItemStacks.EMPTY_ID` (com/lion/graveyard/util/LegacyItemStacks.java:23)
- [x] `LegacyItemStacks.LegacyItemStacks(0)` (com/lion/graveyard/util/LegacyItemStacks.java:25)
- [ ] `LegacyItemStacks.currentDataVersion(0)`: it calls Minecraft's `SharedConstants`.getCurrentVersion (com/lion/graveyard/util/LegacyItemStacks.java:28)
- [ ] `LegacyItemStacks.writeDataVersion(1)`: it uses Minecraft's `ValueOutput` (com/lion/graveyard/util/LegacyItemStacks.java:32)
- [ ] `LegacyItemStacks.readDataVersion(1)`: it uses Minecraft's `ValueInput` (com/lion/graveyard/util/LegacyItemStacks.java:36)
- [ ] `LegacyItemStacks.parse(3)`: it uses Minecraft's `Provider` (com/lion/graveyard/util/LegacyItemStacks.java:40)
- [ ] `LegacyItemStacks.read(2)`: it uses Minecraft's `ValueInput` (com/lion/graveyard/util/LegacyItemStacks.java:58)
- [ ] `LegacyItemStacks.write(3)`: it uses Minecraft's `ValueOutput` (com/lion/graveyard/util/LegacyItemStacks.java:62)
- [ ] `LegacyItemStacks.loadSlots(2)`: it uses Minecraft's `ValueInput` (com/lion/graveyard/util/LegacyItemStacks.java:67)
- [ ] `LegacyItemStacks.saveSlots(2)`: it uses Minecraft's `ValueOutput` (com/lion/graveyard/util/LegacyItemStacks.java:78)
- [ ] `LegacyItemStacks.sourceVersion(2)`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/util/LegacyItemStacks.java:83)
- [ ] `LegacyItemStacks.upgrade(2)`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/util/LegacyItemStacks.java:93)

### `LegacySavedData`

- [x] `LegacySavedData.LOGGER` (com/lion/graveyard/util/LegacySavedData.java:14)
- [x] `LegacySavedData.LegacySavedData(0)` (com/lion/graveyard/util/LegacySavedData.java:16)
- [ ] `LegacySavedData.read(2)`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/util/LegacySavedData.java:19)

### `LenientJsonFiles`

- [x] `LenientJsonFiles.LenientJsonFiles(0)` (com/lion/graveyard/util/LenientJsonFiles.java:18)
- [ ] `LenientJsonFiles.read(2)`: it uses Minecraft's `ResourceManager` (com/lion/graveyard/util/LenientJsonFiles.java:21)

### `MathUtil`

- [ ] `MathUtil.createParticleDisk(10)`: it's generic (com/lion/graveyard/util/MathUtil.java:9)
- [ ] `MathUtil.createParticleCircle(11)`: it's generic (com/lion/graveyard/util/MathUtil.java:23)
- [ ] `MathUtil.createParticleSpiral(10)`: it's generic (com/lion/graveyard/util/MathUtil.java:47)
- [ ] `MathUtil.createParticleFlare(9)`: it's generic (com/lion/graveyard/util/MathUtil.java:63)

### `NotifySounds`

- [x] `NotifySounds.NotifySounds(0)` (com/lion/graveyard/util/NotifySounds.java:11)
- [ ] `NotifySounds.play(5)`: it uses Minecraft's `SoundEvent` (com/lion/graveyard/util/NotifySounds.java:14)

### `OptionalTargetMixinPlugin`: it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)

- [ ] `OptionalTargetMixinPlugin.onLoad(1)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (com/lion/graveyard/util/OptionalTargetMixinPlugin.java:10)
- [ ] `OptionalTargetMixinPlugin.getRefMapperConfig(0)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (com/lion/graveyard/util/OptionalTargetMixinPlugin.java:13)
- [ ] `OptionalTargetMixinPlugin.shouldApplyMixin(2)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (com/lion/graveyard/util/OptionalTargetMixinPlugin.java:17)
- [ ] `OptionalTargetMixinPlugin.acceptTargets(2)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (com/lion/graveyard/util/OptionalTargetMixinPlugin.java:21)
- [ ] `OptionalTargetMixinPlugin.getMixins(0)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (com/lion/graveyard/util/OptionalTargetMixinPlugin.java:24)
- [ ] `OptionalTargetMixinPlugin.preApply(4)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (com/lion/graveyard/util/OptionalTargetMixinPlugin.java:28)
- [ ] `OptionalTargetMixinPlugin.postApply(4)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (com/lion/graveyard/util/OptionalTargetMixinPlugin.java:31)

### `Scarecrows`

- [x] `Scarecrows.RADIUS` (com/lion/graveyard/util/Scarecrows.java:19)
- [x] `Scarecrows.Scarecrows(0)` (com/lion/graveyard/util/Scarecrows.java:21)
- [ ] `Scarecrows.isNear(3)`: it uses Minecraft's `Level` (com/lion/graveyard/util/Scarecrows.java:24)
- [ ] `Scarecrows.hasBlockScarecrow(3)`: it uses Minecraft's `Level` (com/lion/graveyard/util/Scarecrows.java:30)
- [ ] `Scarecrows.isRelevant(1)`: it uses Minecraft's `BlockState` (com/lion/graveyard/util/Scarecrows.java:70)
- [ ] `Scarecrows.isWearingPumpkin(1)`: it uses Minecraft's `ArmorStand` (com/lion/graveyard/util/Scarecrows.java:74)
- [ ] `Scarecrows.isBuiltScarecrow(2)`: it uses Minecraft's `Level` (com/lion/graveyard/util/Scarecrows.java:79)

### `SpawnHordeCommand`

- [ ] `SpawnHordeCommand.register(1)`: it uses Minecraft's `CommandDispatcher` (com/lion/graveyard/util/SpawnHordeCommand.java:20)
- [ ] `SpawnHordeCommand.executeSpawn(1)`: it calls Graveyard.getConfig, which isn't translated (com/lion/graveyard/util/SpawnHordeCommand.java:27)
- [ ] `SpawnHordeCommand.spawnHordeEntity(5)`: it uses Minecraft's `ServerLevel` (com/lion/graveyard/util/SpawnHordeCommand.java:52)

### `StaffRecipeLock`

- [x] `StaffRecipeLock.StaffRecipeLock(0)` (com/lion/graveyard/util/StaffRecipeLock.java:11)
- [ ] `StaffRecipeLock.isStaffRecipe(1)`: it uses Minecraft's `RecipeHolder` (com/lion/graveyard/util/StaffRecipeLock.java:14)
- [ ] `StaffRecipeLock.isLocked(2)`: it uses Minecraft's `ServerPlayer` (com/lion/graveyard/util/StaffRecipeLock.java:19)
- [ ] `StaffRecipeLock.blocksTake(2)`: it uses Minecraft's `Container` (com/lion/graveyard/util/StaffRecipeLock.java:23)

### `PoolAliasProcessor`: it implements Minecraft's `StructureProcessor`

- [ ] `PoolAliasProcessor.CODEC`: it uses Minecraft's `MapCodec` (com/lion/graveyard/world/processors/PoolAliasProcessor.java:15)
- [ ] `PoolAliasProcessor.PoolAliasProcessor(1)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (com/lion/graveyard/world/processors/PoolAliasProcessor.java:20)
- [ ] `PoolAliasProcessor.apply(1)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (com/lion/graveyard/world/processors/PoolAliasProcessor.java:24)
- [ ] `PoolAliasProcessor.codec(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (com/lion/graveyard/world/processors/PoolAliasProcessor.java:50)

### `RemoveWaterloggedCryptProcessor`: it implements Minecraft's `StructureProcessor`

- [ ] `RemoveWaterloggedCryptProcessor.CODEC`: it uses Minecraft's `MapCodec` (com/lion/graveyard/world/processors/RemoveWaterloggedCryptProcessor.java:11)
- [ ] `RemoveWaterloggedCryptProcessor.processBlock(6)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (com/lion/graveyard/world/processors/RemoveWaterloggedCryptProcessor.java:13)
- [ ] `RemoveWaterloggedCryptProcessor.codec(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (com/lion/graveyard/world/processors/RemoveWaterloggedCryptProcessor.java:19)

### `RemoveWaterloggedProcessor`: it implements Minecraft's `StructureProcessor`

- [ ] `RemoveWaterloggedProcessor.INSTANCE`: it uses RemoveWaterloggedProcessor, which isn't translated (it implements Minecraft's `StructureProcessor`) (com/lion/graveyard/world/processors/RemoveWaterloggedProcessor.java:12)
- [ ] `RemoveWaterloggedProcessor.CODEC`: it uses Minecraft's `MapCodec` (com/lion/graveyard/world/processors/RemoveWaterloggedProcessor.java:13)
- [ ] `RemoveWaterloggedProcessor.processBlock(6)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (com/lion/graveyard/world/processors/RemoveWaterloggedProcessor.java:15)
- [ ] `RemoveWaterloggedProcessor.codec(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (com/lion/graveyard/world/processors/RemoveWaterloggedProcessor.java:22)

### `RuinProcessor`: it implements Minecraft's `StructureProcessor`

- [ ] `RuinProcessor.DEFAULT_FLOOR_REMAINS`: it uses Minecraft's `TagKey` (com/lion/graveyard/world/processors/RuinProcessor.java:72)
- [ ] `RuinProcessor.DEFAULT_WALL_REMAINS`: it uses Minecraft's `TagKey` (com/lion/graveyard/world/processors/RuinProcessor.java:73)
- [ ] `RuinProcessor.DEFAULT_LONG_REMAINS`: it uses Minecraft's `TagKey` (com/lion/graveyard/world/processors/RuinProcessor.java:74)
- [ ] `RuinProcessor.DEFAULT_LITTER`: it uses Minecraft's `TagKey` (com/lion/graveyard/world/processors/RuinProcessor.java:75)
- [ ] `RuinProcessor.RUBBLE_STONE`: it uses Minecraft's `TagKey` (com/lion/graveyard/world/processors/RuinProcessor.java:76)
- [ ] `RuinProcessor.SCORCHABLE_STONE`: it uses Minecraft's `TagKey` (com/lion/graveyard/world/processors/RuinProcessor.java:77)
- [ ] `RuinProcessor.SCORCHABLE_TERRACOTTA`: it uses Minecraft's `TagKey` (com/lion/graveyard/world/processors/RuinProcessor.java:78)
- [x] `RuinProcessor.LOG_PART` (com/lion/graveyard/world/processors/RuinProcessor.java:79)
- [x] `RuinProcessor.WOOD_PART` (com/lion/graveyard/world/processors/RuinProcessor.java:80)
- [x] `RuinProcessor.STRIPPED_LOG_PART` (com/lion/graveyard/world/processors/RuinProcessor.java:81)
- [x] `RuinProcessor.STRIPPED_WOOD_PART` (com/lion/graveyard/world/processors/RuinProcessor.java:82)
- [ ] `RuinProcessor.WOOD_PARTS`: it uses Minecraft's `TagKey` (com/lion/graveyard/world/processors/RuinProcessor.java:83)
- [ ] `RuinProcessor.CODEC`: it uses Minecraft's `MapCodec` (com/lion/graveyard/world/processors/RuinProcessor.java:92)
- [x] `RuinProcessor.LOG_RESISTANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:113)
- [x] `RuinProcessor.NOISE_CELL` (com/lion/graveyard/world/processors/RuinProcessor.java:114)
- [x] `RuinProcessor.ROOF_HOLE_CHANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:115)
- [x] `RuinProcessor.MIN_BITE_RADIUS` (com/lion/graveyard/world/processors/RuinProcessor.java:116)
- [x] `RuinProcessor.BITE_STEEPNESS` (com/lion/graveyard/world/processors/RuinProcessor.java:117)
- [x] `RuinProcessor.SMALL_HEAP_BLOCKS` (com/lion/graveyard/world/processors/RuinProcessor.java:118)
- [x] `RuinProcessor.LARGE_HEAP_BLOCKS` (com/lion/graveyard/world/processors/RuinProcessor.java:119)
- [x] `RuinProcessor.MAX_HEAP_LAYERS` (com/lion/graveyard/world/processors/RuinProcessor.java:120)
- [x] `RuinProcessor.HEAP_ROUGHNESS` (com/lion/graveyard/world/processors/RuinProcessor.java:121)
- [x] `RuinProcessor.GRAVEL_CHANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:122)
- [x] `RuinProcessor.STRAY_CHAR_CHANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:123)
- [x] `RuinProcessor.BURNT_EDGE_CHAR_CHANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:124)
- [x] `RuinProcessor.EDGE_CHAR_CHANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:125)
- [x] `RuinProcessor.VEIN_CHANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:126)
- [x] `RuinProcessor.VEIN_STRAIGHT_CHANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:127)
- [x] `RuinProcessor.MIN_VEIN_LENGTH` (com/lion/graveyard/world/processors/RuinProcessor.java:128)
- [x] `RuinProcessor.VEIN_LENGTH_SPREAD` (com/lion/graveyard/world/processors/RuinProcessor.java:129)
- [x] `RuinProcessor.LONE_SCORCH_MARKS` (com/lion/graveyard/world/processors/RuinProcessor.java:130)
- [x] `RuinProcessor.STRAY_EMBER_CHANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:131)
- [x] `RuinProcessor.MAGMA_CHANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:132)
- [x] `RuinProcessor.WALL_REMAINS_CHANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:133)
- [x] `RuinProcessor.SINGE_CHANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:134)
- [x] `RuinProcessor.SINGED_RUBBLE_CHANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:135)
- [x] `RuinProcessor.MIN_EMBER_NEIGHBORS` (com/lion/graveyard/world/processors/RuinProcessor.java:136)
- [x] `RuinProcessor.SMALL_PIECE_COLUMNS` (com/lion/graveyard/world/processors/RuinProcessor.java:137)
- [x] `RuinProcessor.LARGE_PIECE_COLUMNS` (com/lion/graveyard/world/processors/RuinProcessor.java:138)
- [x] `RuinProcessor.FEATHER_REACH` (com/lion/graveyard/world/processors/RuinProcessor.java:139)
- [x] `RuinProcessor.PATH_FRAY_CHANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:140)
- [x] `RuinProcessor.MAX_SINK` (com/lion/graveyard/world/processors/RuinProcessor.java:141)
- [x] `RuinProcessor.MAX_LONE_STACK` (com/lion/graveyard/world/processors/RuinProcessor.java:142)
- [x] `RuinProcessor.FEATHER_PATH_CHANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:143)
- [x] `RuinProcessor.FEATHER_COARSE_DIRT_CHANCE` (com/lion/graveyard/world/processors/RuinProcessor.java:144)
- [x] `RuinProcessor.GROUND_SEARCH_UP` (com/lion/graveyard/world/processors/RuinProcessor.java:145)
- [x] `RuinProcessor.GROUND_SEARCH_DOWN` (com/lion/graveyard/world/processors/RuinProcessor.java:146)
- [x] `RuinProcessor.NO_GROUND` (com/lion/graveyard/world/processors/RuinProcessor.java:147)
- [x] `RuinProcessor.SMOLDER_SEED` (com/lion/graveyard/world/processors/RuinProcessor.java:148)
- [ ] `RuinProcessor.RuinProcessor(16)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (com/lion/graveyard/world/processors/RuinProcessor.java:168)
- [ ] `RuinProcessor.villageTag(1)`: it uses Minecraft's `TagKey` (com/lion/graveyard/world/processors/RuinProcessor.java:206)
- [ ] `RuinProcessor.finalizeProcessing(6)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (com/lion/graveyard/world/processors/RuinProcessor.java:210)
- [ ] `RuinProcessor.dried(4)`: it uses Minecraft's `StructureBlockInfo` (com/lion/graveyard/world/processors/RuinProcessor.java:262)
- [ ] `RuinProcessor.leavesMud(2)`: it uses Minecraft's `BlockPos` (com/lion/graveyard/world/processors/RuinProcessor.java:298)
- [ ] `RuinProcessor.dropToGround(4)`: it uses Minecraft's `StructureBlockInfo` (com/lion/graveyard/world/processors/RuinProcessor.java:319)
- [ ] `RuinProcessor.reconnected(5)`: it uses Minecraft's `StructureBlockInfo` (com/lion/graveyard/world/processors/RuinProcessor.java:377)
- [ ] `RuinProcessor.connectsTo(3)`: it uses Minecraft's `BlockState` (com/lion/graveyard/world/processors/RuinProcessor.java:419)
- [ ] `RuinProcessor.dropAsUnit(3)`: it uses Minecraft's `StructureBlockInfo` (com/lion/graveyard/world/processors/RuinProcessor.java:426)
- [ ] `RuinProcessor.groundShift(4)`: it uses Minecraft's `ServerLevelAccessor` (com/lion/graveyard/world/processors/RuinProcessor.java:446)
- [ ] `RuinProcessor.isGround(2)`: it uses Minecraft's `ServerLevelAccessor` (com/lion/graveyard/world/processors/RuinProcessor.java:461)
- [ ] `RuinProcessor.isColumnIn(3)`: it uses Minecraft's `BoundingBox` (com/lion/graveyard/world/processors/RuinProcessor.java:466)
- [ ] `RuinProcessor.members(1)`: it uses Minecraft's `Block` (com/lion/graveyard/world/processors/RuinProcessor.java:470)
- [ ] `RuinProcessor.codec(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (com/lion/graveyard/world/processors/RuinProcessor.java:480)
- [ ] `RuinProcessor.evaluatesEntirePieceState(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (com/lion/graveyard/world/processors/RuinProcessor.java:484)

### `RuinProcessor.Bite`

- [x] `RuinProcessor.Bite.Bite(5)` (com/lion/graveyard/world/processors/RuinProcessor.java:488)
- [x] `RuinProcessor.Bite.depthAt(2)` (com/lion/graveyard/world/processors/RuinProcessor.java:489)
- [x] `RuinProcessor.Bite.x(0)` (com/lion/graveyard/world/processors/RuinProcessor.java:488)
- [x] `RuinProcessor.Bite.z(0)` (com/lion/graveyard/world/processors/RuinProcessor.java:488)
- [x] `RuinProcessor.Bite.radiusX(0)` (com/lion/graveyard/world/processors/RuinProcessor.java:488)
- [x] `RuinProcessor.Bite.radiusZ(0)` (com/lion/graveyard/world/processors/RuinProcessor.java:488)
- [x] `RuinProcessor.Bite.depth(0)` (com/lion/graveyard/world/processors/RuinProcessor.java:488)

### `RuinProcessor.DryMode`: its constants carry values

- [ ] `RuinProcessor.DryMode.DryMode(1)`: its class isn't translated (its constants carry values) (com/lion/graveyard/world/processors/RuinProcessor.java:504)
- [ ] `RuinProcessor.DryMode.getSerializedName(0)`: its class isn't translated (its constants carry values) (com/lion/graveyard/world/processors/RuinProcessor.java:508)

### `RuinProcessor.Ember`: it uses Minecraft's `BlockPos`

- [ ] `RuinProcessor.Ember.Ember(3)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:513)
- [ ] `RuinProcessor.Ember.spot(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:513)
- [ ] `RuinProcessor.Ember.replaced(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:513)
- [ ] `RuinProcessor.Ember.cap(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:513)

### `RuinProcessor.Extras`: it uses DryMode, which isn't translated (its constants carry values)

- [ ] `RuinProcessor.Extras.CODEC`: it uses Minecraft's `MapCodec` (com/lion/graveyard/world/processors/RuinProcessor.java:517)
- [ ] `RuinProcessor.Extras.Extras(3)`: its class isn't translated (it uses DryMode, which isn't translated (its constants carry values)) (com/lion/graveyard/world/processors/RuinProcessor.java:516)
- [ ] `RuinProcessor.Extras.dry(0)`: its class isn't translated (it uses DryMode, which isn't translated (its constants carry values)) (com/lion/graveyard/world/processors/RuinProcessor.java:516)
- [ ] `RuinProcessor.Extras.variant(0)`: its class isn't translated (it uses DryMode, which isn't translated (its constants carry values)) (com/lion/graveyard/world/processors/RuinProcessor.java:516)
- [ ] `RuinProcessor.Extras.smolderChance(0)`: its class isn't translated (it uses DryMode, which isn't translated (its constants carry values)) (com/lion/graveyard/world/processors/RuinProcessor.java:516)

### `RuinProcessor.Heap`

- [x] `RuinProcessor.Heap.Heap(4)` (com/lion/graveyard/world/processors/RuinProcessor.java:529)
- [x] `RuinProcessor.Heap.x(0)` (com/lion/graveyard/world/processors/RuinProcessor.java:529)
- [x] `RuinProcessor.Heap.z(0)` (com/lion/graveyard/world/processors/RuinProcessor.java:529)
- [x] `RuinProcessor.Heap.height(0)` (com/lion/graveyard/world/processors/RuinProcessor.java:529)
- [x] `RuinProcessor.Heap.radius(0)` (com/lion/graveyard/world/processors/RuinProcessor.java:529)

### `RuinProcessor.Ruin`: it uses Minecraft's `BlockPos`

- [ ] `RuinProcessor.Ruin.Ruin(7)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:562)
- [ ] `RuinProcessor.Ruin.dominantBlock(3)`: it uses Minecraft's `Block` (com/lion/graveyard/world/processors/RuinProcessor.java:608)
- [ ] `RuinProcessor.Ruin.planksOf(1)`: it uses Minecraft's `Block` (com/lion/graveyard/world/processors/RuinProcessor.java:627)
- [ ] `RuinProcessor.Ruin.slabOf(2)`: it uses Minecraft's `Block` (com/lion/graveyard/world/processors/RuinProcessor.java:637)
- [ ] `RuinProcessor.Ruin.isSmall(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:646)
- [ ] `RuinProcessor.Ruin.isLarge(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:650)
- [ ] `RuinProcessor.Ruin.result(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:654)
- [ ] `RuinProcessor.Ruin.quenchEmbers(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:674)
- [ ] `RuinProcessor.Ruin.smolderingAllowed(0)`: it calls Graveyard.getConfig, which isn't translated (com/lion/graveyard/world/processors/RuinProcessor.java:687)
- [ ] `RuinProcessor.Ruin.isCharredWood(1)`: it uses Minecraft's `BlockState` (com/lion/graveyard/world/processors/RuinProcessor.java:691)
- [ ] `RuinProcessor.Ruin.smoldering(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:695)
- [ ] `RuinProcessor.Ruin.collapse(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:711)
- [ ] `RuinProcessor.Ruin.piles(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:768)
- [ ] `RuinProcessor.Ruin.randomBite(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:772)
- [ ] `RuinProcessor.Ruin.spread(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:793)
- [ ] `RuinProcessor.Ruin.scorch(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:797)
- [ ] `RuinProcessor.Ruin.touchesFracture(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:837)
- [ ] `RuinProcessor.Ruin.growVein(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:847)
- [ ] `RuinProcessor.Ruin.singe(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:873)
- [ ] `RuinProcessor.Ruin.touchesChar(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:889)
- [ ] `RuinProcessor.Ruin.isCharred(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:900)
- [ ] `RuinProcessor.Ruin.singed(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:906)
- [ ] `RuinProcessor.Ruin.woodPart(1)`: it uses Minecraft's `BlockState` (com/lion/graveyard/world/processors/RuinProcessor.java:916)
- [ ] `RuinProcessor.Ruin.logKind(2)`: it uses Minecraft's `BlockState` (com/lion/graveyard/world/processors/RuinProcessor.java:926)
- [ ] `RuinProcessor.Ruin.indexIn(2)`: it uses Minecraft's `Block` (com/lion/graveyard/world/processors/RuinProcessor.java:938)
- [ ] `RuinProcessor.Ruin.scorchBlock(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:948)
- [ ] `RuinProcessor.Ruin.canChar(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:956)
- [ ] `RuinProcessor.Ruin.charred(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:960)
- [ ] `RuinProcessor.Ruin.charStone(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:974)
- [ ] `RuinProcessor.Ruin.settle(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:979)
- [ ] `RuinProcessor.Ruin.lostSupport(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:997)
- [ ] `RuinProcessor.Ruin.lostOtherHalf(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1012)
- [ ] `RuinProcessor.Ruin.isProtected(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1029)
- [ ] `RuinProcessor.Ruin.dropOrphanSnow(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1033)
- [ ] `RuinProcessor.Ruin.wasRemoved(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1053)
- [ ] `RuinProcessor.Ruin.pileRubble(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1057)
- [ ] `RuinProcessor.Ruin.standsOnTemplate(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1104)
- [ ] `RuinProcessor.Ruin.pile(3)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1109)
- [ ] `RuinProcessor.Ruin.isFree(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1118)
- [ ] `RuinProcessor.Ruin.templateBase(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1122)
- [ ] `RuinProcessor.Ruin.worldBase(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1138)
- [ ] `RuinProcessor.Ruin.nearestDebris(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1153)
- [ ] `RuinProcessor.Ruin.rubbleBlock(3)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1168)
- [ ] `RuinProcessor.Ruin.turned(2)`: it uses Minecraft's `BlockState` (com/lion/graveyard/world/processors/RuinProcessor.java:1186)
- [ ] `RuinProcessor.Ruin.rubbleCap(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1197)
- [ ] `RuinProcessor.Ruin.placeEmbers(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1209)
- [ ] `RuinProcessor.Ruin.heapNeighbors(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1237)
- [ ] `RuinProcessor.Ruin.disconnect(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1250)
- [ ] `RuinProcessor.Ruin.disconnectedWall(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1274)
- [ ] `RuinProcessor.Ruin.needsPost(1)`: it uses Minecraft's `BlockState` (com/lion/graveyard/world/processors/RuinProcessor.java:1287)
- [ ] `RuinProcessor.Ruin.wallSide(1)`: it uses Minecraft's `EnumProperty` (com/lion/graveyard/world/processors/RuinProcessor.java:1295)
- [ ] `RuinProcessor.Ruin.isDisconnected(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1304)
- [ ] `RuinProcessor.Ruin.drain(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1309)
- [ ] `RuinProcessor.Ruin.sinkLoneColumns(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1318)
- [ ] `RuinProcessor.Ruin.columnHeight(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1335)
- [ ] `RuinProcessor.Ruin.isLone(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1347)
- [ ] `RuinProcessor.Ruin.scatterLitter(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1357)
- [ ] `RuinProcessor.Ruin.scatterByCell(4)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1368)
- [ ] `RuinProcessor.Ruin.featherPathEdges(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1385)
- [ ] `RuinProcessor.Ruin.frayPath(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1422)
- [ ] `RuinProcessor.Ruin.isPathEdge(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1434)
- [ ] `RuinProcessor.Ruin.featherState(2)`: it uses Minecraft's `BlockState` (com/lion/graveyard/world/processors/RuinProcessor.java:1446)
- [ ] `RuinProcessor.Ruin.oriented(3)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1452)
- [ ] `RuinProcessor.Ruin.placeLongRemains(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1469)
- [ ] `RuinProcessor.Ruin.placeRemains(3)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1493)
- [ ] `RuinProcessor.Ruin.freeNeighbor(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1510)
- [ ] `RuinProcessor.Ruin.adjacentWall(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1523)
- [ ] `RuinProcessor.Ruin.floorCells(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1534)
- [ ] `RuinProcessor.Ruin.groundLevel(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1548)
- [ ] `RuinProcessor.Ruin.columnTops(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1552)
- [ ] `RuinProcessor.Ruin.put(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1562)
- [ ] `RuinProcessor.Ruin.toWorld(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1566)
- [ ] `RuinProcessor.Ruin.toTemplate(1)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/lion/graveyard/world/processors/RuinProcessor.java:1570)
- [ ] `RuinProcessor.Ruin.resistance(1)`: it uses Minecraft's `BlockState` (com/lion/graveyard/world/processors/RuinProcessor.java:1580)
- [ ] `RuinProcessor.Ruin.isRubbleMaterial(1)`: it uses Minecraft's `BlockState` (com/lion/graveyard/world/processors/RuinProcessor.java:1585)
- [ ] `RuinProcessor.Ruin.isFloor(1)`: it uses Minecraft's `BlockState` (com/lion/graveyard/world/processors/RuinProcessor.java:1589)
- [ ] `RuinProcessor.Ruin.isFullCube(1)`: it uses Minecraft's `BlockState` (com/lion/graveyard/world/processors/RuinProcessor.java:1593)
- [x] `RuinProcessor.Ruin.valueNoise(3)` (com/lion/graveyard/world/processors/RuinProcessor.java:1597)
- [x] `RuinProcessor.Ruin.hash(3)` (com/lion/graveyard/world/processors/RuinProcessor.java:1607)

### `RuinProcessor.Variant`: it uses Minecraft's `Block`

- [ ] `RuinProcessor.Variant.DEFAULT`: it uses Variant, which isn't translated (it uses Minecraft's `Block`) (com/lion/graveyard/world/processors/RuinProcessor.java:1619)
- [ ] `RuinProcessor.Variant.CODEC`: it uses Minecraft's `Codec` (com/lion/graveyard/world/processors/RuinProcessor.java:1622)
- [ ] `RuinProcessor.Variant.Variant(7)`: its class isn't translated (it uses Minecraft's `Block`) (com/lion/graveyard/world/processors/RuinProcessor.java:1616)
- [ ] `RuinProcessor.Variant.charStone(0)`: its class isn't translated (it uses Minecraft's `Block`) (com/lion/graveyard/world/processors/RuinProcessor.java:1616)
- [ ] `RuinProcessor.Variant.stoneFallback(0)`: its class isn't translated (it uses Minecraft's `Block`) (com/lion/graveyard/world/processors/RuinProcessor.java:1616)
- [ ] `RuinProcessor.Variant.stoneCap(0)`: its class isn't translated (it uses Minecraft's `Block`) (com/lion/graveyard/world/processors/RuinProcessor.java:1616)
- [ ] `RuinProcessor.Variant.looseRubble(0)`: its class isn't translated (it uses Minecraft's `Block`) (com/lion/graveyard/world/processors/RuinProcessor.java:1616)
- [ ] `RuinProcessor.Variant.scorchTerracotta(0)`: its class isn't translated (it uses Minecraft's `Block`) (com/lion/graveyard/world/processors/RuinProcessor.java:1616)
- [ ] `RuinProcessor.Variant.woodRubble(0)`: its class isn't translated (it uses Minecraft's `Block`) (com/lion/graveyard/world/processors/RuinProcessor.java:1616)
- [ ] `RuinProcessor.Variant.rubble(0)`: its class isn't translated (it uses Minecraft's `Block`) (com/lion/graveyard/world/processors/RuinProcessor.java:1616)

### `SwitchSpawnerProcessor`: it implements Minecraft's `StructureProcessor`

- [ ] `SwitchSpawnerProcessor.INSTANCE`: it uses SwitchSpawnerProcessor, which isn't translated (it implements Minecraft's `StructureProcessor`) (com/lion/graveyard/world/processors/SwitchSpawnerProcessor.java:16)
- [ ] `SwitchSpawnerProcessor.CODEC`: it uses Minecraft's `MapCodec` (com/lion/graveyard/world/processors/SwitchSpawnerProcessor.java:17)
- [ ] `SwitchSpawnerProcessor.processBlock(6)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (com/lion/graveyard/world/processors/SwitchSpawnerProcessor.java:19)
- [ ] `SwitchSpawnerProcessor.swapEntityId(1)`: it uses Minecraft's `CompoundTag` (com/lion/graveyard/world/processors/SwitchSpawnerProcessor.java:39)
- [ ] `SwitchSpawnerProcessor.codec(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (com/lion/graveyard/world/processors/SwitchSpawnerProcessor.java:48)

### `TGJigsawStructure`: it extends Minecraft's `Structure`

- [ ] `TGJigsawStructure.CODEC`: it uses Minecraft's `MapCodec` (com/lion/graveyard/world/structures/TGJigsawStructure.java:35)
- [ ] `TGJigsawStructure.TGJigsawStructure(11)`: its class isn't translated (it extends Minecraft's `Structure`) (com/lion/graveyard/world/structures/TGJigsawStructure.java:63)
- [ ] `TGJigsawStructure.findGenerationPoint(1)`: its class isn't translated (it extends Minecraft's `Structure`) (com/lion/graveyard/world/structures/TGJigsawStructure.java:90)
- [ ] `TGJigsawStructure.startPieces(2)`: its class isn't translated (it extends Minecraft's `Structure`) (com/lion/graveyard/world/structures/TGJigsawStructure.java:135)
- [ ] `TGJigsawStructure.canGenerate(5)`: it uses Minecraft's `GenerationContext` (com/lion/graveyard/world/structures/TGJigsawStructure.java:151)
- [ ] `TGJigsawStructure.isTerrainFlat(5)`: it uses Minecraft's `GenerationContext` (com/lion/graveyard/world/structures/TGJigsawStructure.java:155)
- [ ] `TGJigsawStructure.type(0)`: its class isn't translated (it extends Minecraft's `Structure`) (com/lion/graveyard/world/structures/TGJigsawStructure.java:187)
