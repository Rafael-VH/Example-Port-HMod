# Java translated to Rust

Translated 186 of 2580 methods (1 more is done another way in Horizon) and 208 of 1573 static fields to Rust; 1 of 1 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 1933 | `BNCriterion.register(0)`: it uses BNCriterion.BREW_BLUE, which isn't translated (it uses Minecraft's `PlayerTrigger`) |
| Uses other libraries | 170 | `AnchorTreeSaplingModel.model(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) |
| Uses Minecraft classes (needs Horizon's game API) | 155 | `BNBrewingStandBlockEntity.tick(4)`: it uses Minecraft's `Level` |
| Other Java this translator doesn't handle yet | 120 | `BNCriterion.register(1)`: it's generic |
| Shares one object between names (Java references) | 5 | `Config.setBoolean(4)`: it changes an object through a second name |
| Uses files (java.nio / java.io) | 4 | `Config.load(0)`: it uses `File` (java.io) |
| Uses Java collections other than List (Map, Set...) | 2 | `NetherBlocks.getModBlocks(0)`: it uses `Stream` (java.util.stream) |
| Done another way in Horizon | 1 | `BetterNetherClient.onInitializeClient(0)`: done another way in Horizon: it runs as `on_init`, from Horizon's init hook |
| Uses Fabric API | 1 | `CommandRegistry.register(0)`: it uses Fabric's `CommandRegistrationCallback`.EVENT |
| Uses Gson (JSON) | 1 | `NetherModels.Impl.toArray(1)`: it creates `JsonArray` (com.google.gson) |
| Uses lambdas or method references | 1 | `Patcher.register(0)`: a method reference |
| Uses null | 1 | `NetherModelProvider.shipsHandAuthoredModel(1)`: `null` (Rust has no null: this needs an Option) |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| org/betterx/betternether/BetterNether.java:52 | `init` | `on_init`, called from the hook, without: the call to `log.info(..)`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)); the call to `initOptions(..)`: it calls BetterNether.initOptions, which isn't translated; the call to `warnAboutLegacyConfigs(..)`: it calls BetterNether.warnAboutLegacyConfigs, which isn't translated; the call to `BlockEntitiesRegistry.register(..)`: it calls BlockEntitiesRegistry.register, which isn't translated; the call to `NetherEntities.register(..)`: it calls NetherEntities.register, which isn't translated; the call to `NetherFeatures.register(..)`: it calls NetherFeatures.register, which isn't translated; the call to `NetherBiomes.register(..)`: it calls NetherBiomes.register, which isn't translated; the call to `CommandRegistry.register(..)`: it calls CommandRegistry.register, which isn't translated; the call to `Config.save(..)`: it calls Config.save, which isn't translated; the call to `BNCriterion.register(..)`: it calls BNCriterion.register, which isn't translated; the call to `NetherVegetation.setupBonemealFeatures(..)`: it calls NetherVegetation.setupBonemealFeatures, which isn't translated; the call to `Configs.saveConfigs(..)`: it calls Configs.saveConfigs, which isn't translated; the call to `WorldConfig.registerMod(..)`: it calls WorldConfig.registerMod, which isn't in the mod's source; the call to `Patcher.register(..)`: it calls Patcher.register, which isn't translated; the call to `BECreativeTabs.register(..)`: it calls BECreativeTabs.register, which isn't translated; an `if` whose first call is `BCLib.isDatagen(..)`: it calls `BCLib` (org.betterx.bclib).isDatagen |

## Every class


### `org/betterx/betternether/world/tree/skeleton/Skeleton.java`: couldn't be read: expected a name but found `{` (line 10)


### `BNCriterion`

- [ ] `BNCriterion.BREW_BLUE_ID`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/advancements/BNCriterion.java:15)
- [ ] `BNCriterion.USED_FORGE_ID`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/advancements/BNCriterion.java:16)
- [ ] `BNCriterion.DISTURBED_WISP_ID`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/advancements/BNCriterion.java:17)
- [ ] `BNCriterion.BURNED_GLOOMSCULK_CRYSTAL_ID`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/advancements/BNCriterion.java:18)
- [ ] `BNCriterion.WISP_SHED_EXPERIENCE_ID`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/advancements/BNCriterion.java:19)
- [ ] `BNCriterion.BREW_BLUE`: it uses Minecraft's `PlayerTrigger` (org/betterx/betternether/advancements/BNCriterion.java:20)
- [ ] `BNCriterion.USED_FORGE`: it uses Minecraft's `PlayerTrigger` (org/betterx/betternether/advancements/BNCriterion.java:21)
- [ ] `BNCriterion.DISTURBED_WISP`: it uses Minecraft's `PlayerTrigger` (org/betterx/betternether/advancements/BNCriterion.java:22)
- [ ] `BNCriterion.BURNED_GLOOMSCULK_CRYSTAL`: it uses Minecraft's `PlayerTrigger` (org/betterx/betternether/advancements/BNCriterion.java:23)
- [ ] `BNCriterion.WISP_SHED_EXPERIENCE`: it uses Minecraft's `PlayerTrigger` (org/betterx/betternether/advancements/BNCriterion.java:24)
- [ ] `BNCriterion.CONVERT_BY_LIGHTNING`: it uses ConvertByLightningTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (org/betterx/betternether/advancements/BNCriterion.java:25)
- [ ] `BNCriterion.BREW_BLUE_CRITERION`: it uses Minecraft's `Criterion` (org/betterx/betternether/advancements/BNCriterion.java:26)
- [ ] `BNCriterion.USED_FORGE_ANY_CRITERION`: it uses Minecraft's `Criterion` (org/betterx/betternether/advancements/BNCriterion.java:27)
- [ ] `BNCriterion.DISTURBED_WISP_CRITERION`: it uses Minecraft's `Criterion` (org/betterx/betternether/advancements/BNCriterion.java:28)
- [ ] `BNCriterion.BURNED_GLOOMSCULK_CRYSTAL_CRITERION`: it uses Minecraft's `Criterion` (org/betterx/betternether/advancements/BNCriterion.java:29)
- [ ] `BNCriterion.WISP_SHED_EXPERIENCE_CRITERION`: it uses Minecraft's `Criterion` (org/betterx/betternether/advancements/BNCriterion.java:30)
- [ ] `BNCriterion.register(1)`: it's generic (org/betterx/betternether/advancements/BNCriterion.java:32)
- [ ] `BNCriterion.register(2)`: it's generic (org/betterx/betternether/advancements/BNCriterion.java:36)
- [ ] `BNCriterion.register(0)`: it uses BNCriterion.BREW_BLUE, which isn't translated (it uses Minecraft's `PlayerTrigger`) (org/betterx/betternether/advancements/BNCriterion.java:40)

### `BNCriterion.TriggerWithID`: it's an interface

- [ ] `BNCriterion.TriggerWithID.getId(0)`: it has no body (org/betterx/betternether/advancements/BNCriterion.java:55)

### `ConvertByLightningTrigger`: it extends Minecraft's `SimpleCriterionTrigger`

- [ ] `ConvertByLightningTrigger.ID`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/advancements/ConvertByLightningTrigger.java:24)
- [ ] `ConvertByLightningTrigger.getId(0)`: its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (org/betterx/betternether/advancements/ConvertByLightningTrigger.java:26)
- [ ] `ConvertByLightningTrigger.trigger(2)`: its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (org/betterx/betternether/advancements/ConvertByLightningTrigger.java:31)
- [ ] `ConvertByLightningTrigger.match(1)`: its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (org/betterx/betternether/advancements/ConvertByLightningTrigger.java:35)
- [ ] `ConvertByLightningTrigger.codec(0)`: its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (org/betterx/betternether/advancements/ConvertByLightningTrigger.java:40)

### `ConvertByLightningTrigger.TriggerInstance`: it implements Minecraft's `SimpleInstance`

- [ ] `ConvertByLightningTrigger.TriggerInstance.CODEC`: it uses Minecraft's `Codec` (org/betterx/betternether/advancements/ConvertByLightningTrigger.java:45)
- [ ] `ConvertByLightningTrigger.TriggerInstance.TriggerInstance(1)`: its class isn't translated (it implements Minecraft's `SimpleInstance`) (org/betterx/betternether/advancements/ConvertByLightningTrigger.java:51)
- [ ] `ConvertByLightningTrigger.TriggerInstance.matches(1)`: its class isn't translated (it implements Minecraft's `SimpleInstance`) (org/betterx/betternether/advancements/ConvertByLightningTrigger.java:55)
- [ ] `ConvertByLightningTrigger.TriggerInstance.validate(1)`: its class isn't translated (it implements Minecraft's `SimpleInstance`) (org/betterx/betternether/advancements/ConvertByLightningTrigger.java:59)
- [ ] `ConvertByLightningTrigger.TriggerInstance.player(0)`: its class isn't translated (it implements Minecraft's `SimpleInstance`) (org/betterx/betternether/advancements/ConvertByLightningTrigger.java:63)

### `BetterNether`: it implements Fabric's `ModInitializer`

- [ ] `BetterNether.C`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/BetterNether.java:38)
- [ ] `BetterNether.VANILLA_HAMMERS`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/BetterNether.java:39)
- [ ] `BetterNether.VANILLA_EXCAVATORS`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/BetterNether.java:40)
- [ ] `BetterNether.GLIMMERWILD`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/BetterNether.java:41)
- [ ] `BetterNether.VANILLA_HAMMERS_PACK`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/BetterNether.java:42)
- [ ] `BetterNether.VANILLA_EXCAVATORS_PACK`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/BetterNether.java:43)
- [ ] `BetterNether.GLIMMERWILD_PACK`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/BetterNether.java:44)
- [x] `BetterNether.lavafallParticles` (org/betterx/betternether/BetterNether.java:45)
- [x] `BetterNether.fogStart` (org/betterx/betternether/BetterNether.java:46)
- [x] `BetterNether.fogEnd` (org/betterx/betternether/BetterNether.java:47)
- [ ] `BetterNether.onDatagen(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (org/betterx/betternether/BetterNether.java:49)
- [ ] `BetterNether.onInitialize(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (org/betterx/betternether/BetterNether.java:52)
- [ ] `BetterNether.initOptions(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (org/betterx/betternether/BetterNether.java:81)
- [ ] `BetterNether.warnAboutLegacyConfigs(0)`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/BetterNether.java:87)
- [x] `BetterNether.collectDisabled(2)` (org/betterx/betternether/BetterNether.java:111)
- [x] `BetterNether.hasLavafallParticles(0)` (org/betterx/betternether/BetterNether.java:124)
- [x] `BetterNether.changeFogDensity(1)` (org/betterx/betternether/BetterNether.java:128)
- [x] `BetterNether.getFogStart(0)` (org/betterx/betternether/BetterNether.java:133)
- [x] `BetterNether.getFogEnd(0)` (org/betterx/betternether/BetterNether.java:137)

### `BlockEntityChestOfDrawers`: it extends Minecraft's `RandomizableContainerBlockEntity`

- [ ] `BlockEntityChestOfDrawers.BlockEntityChestOfDrawers(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityChestOfDrawers.java:33)
- [ ] `BlockEntityChestOfDrawers.getContainerSize(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityChestOfDrawers.java:38)
- [ ] `BlockEntityChestOfDrawers.getItems(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityChestOfDrawers.java:42)
- [ ] `BlockEntityChestOfDrawers.setItems(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityChestOfDrawers.java:46)
- [ ] `BlockEntityChestOfDrawers.getDefaultName(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityChestOfDrawers.java:50)
- [ ] `BlockEntityChestOfDrawers.createMenu(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityChestOfDrawers.java:54)
- [ ] `BlockEntityChestOfDrawers.loadAdditional(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityChestOfDrawers.java:58)
- [ ] `BlockEntityChestOfDrawers.saveAdditional(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityChestOfDrawers.java:66)
- [ ] `BlockEntityChestOfDrawers.onInvOpen(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityChestOfDrawers.java:73)
- [ ] `BlockEntityChestOfDrawers.stopOpen(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityChestOfDrawers.java:88)
- [ ] `BlockEntityChestOfDrawers.onInvOpenOrClose(0)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityChestOfDrawers.java:96)
- [ ] `BlockEntityChestOfDrawers.playSound(2)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityChestOfDrawers.java:108)
- [ ] `BlockEntityChestOfDrawers.addItemsToList(1)`: its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityChestOfDrawers.java:116)

### `BlockEntityForge`: it extends Minecraft's `AbstractFurnaceBlockEntity`

- [x] `BlockEntityForge.SPEEDUP` (org/betterx/betternether/blockentities/BlockEntityForge.java:18)
- [ ] `BlockEntityForge.BlockEntityForge(2)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityForge.java:20)
- [ ] `BlockEntityForge.getDefaultName(0)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityForge.java:24)
- [ ] `BlockEntityForge.createMenu(2)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityForge.java:28)
- [ ] `BlockEntityForge.getBurnDuration(2)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityForge.java:32)
- [ ] `BlockEntityForge.changeCookTime(1)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityForge.java:36)
- [ ] `BlockEntityForge.awardUsedRecipesAndPopExperience(1)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityForge.java:41)

### `BlockEntityFurnace`: it extends Minecraft's `AbstractFurnaceBlockEntity`

- [ ] `BlockEntityFurnace.BlockEntityFurnace(2)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityFurnace.java:14)
- [ ] `BlockEntityFurnace.getDefaultName(0)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityFurnace.java:18)
- [ ] `BlockEntityFurnace.createMenu(2)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`) (org/betterx/betternether/blockentities/BlockEntityFurnace.java:22)

### `BNBrewingStandBlockEntity`: it extends Minecraft's `BaseContainerBlockEntity`

- [x] `BNBrewingStandBlockEntity.TOP_SLOTS` (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:39)
- [x] `BNBrewingStandBlockEntity.BOTTOM_SLOTS` (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:40)
- [x] `BNBrewingStandBlockEntity.SIDE_SLOTS` (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:41)
- [ ] `BNBrewingStandBlockEntity.BNBrewingStandBlockEntity(2)`: its body couldn't be read: an anonymous class (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:50)
- [ ] `BNBrewingStandBlockEntity.getItems(0)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:81)
- [ ] `BNBrewingStandBlockEntity.setItems(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:85)
- [ ] `BNBrewingStandBlockEntity.getDefaultName(0)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:89)
- [ ] `BNBrewingStandBlockEntity.getContainerSize(0)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:93)
- [ ] `BNBrewingStandBlockEntity.isEmpty(0)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:97)
- [ ] `BNBrewingStandBlockEntity.tick(4)`: it uses Minecraft's `Level` (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:107)
- [ ] `BNBrewingStandBlockEntity.getSlotsEmpty(0)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:157)
- [ ] `BNBrewingStandBlockEntity.isBrewingReagent(2)`: it uses Minecraft's `Level` (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:169)
- [ ] `BNBrewingStandBlockEntity.canCraft(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:173)
- [ ] `BNBrewingStandBlockEntity.craft(0)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:189)
- [ ] `BNBrewingStandBlockEntity.craft(3)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:193)
- [ ] `BNBrewingStandBlockEntity.loadAdditional(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:221)
- [ ] `BNBrewingStandBlockEntity.saveAdditional(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:229)
- [ ] `BNBrewingStandBlockEntity.getItem(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:236)
- [ ] `BNBrewingStandBlockEntity.removeItem(2)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:240)
- [ ] `BNBrewingStandBlockEntity.removeItemNoUpdate(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:244)
- [ ] `BNBrewingStandBlockEntity.setItem(2)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:248)
- [ ] `BNBrewingStandBlockEntity.stillValid(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:254)
- [ ] `BNBrewingStandBlockEntity.canPlaceItem(2)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:260)
- [ ] `BNBrewingStandBlockEntity.getSlotsForFace(1)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:279)
- [ ] `BNBrewingStandBlockEntity.canPlaceItemThroughFace(3)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:287)
- [ ] `BNBrewingStandBlockEntity.canTakeItemThroughFace(3)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:291)
- [ ] `BNBrewingStandBlockEntity.clearContent(0)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:295)
- [ ] `BNBrewingStandBlockEntity.createMenu(2)`: its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`) (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java:299)

### `ChangebleCookTime`: it's an interface

- [ ] `ChangebleCookTime.changeCookTime(1)`: it has no body (org/betterx/betternether/blockentities/ChangebleCookTime.java:4)

### `BlocksHelper`

- [x] `BlocksHelper.FLAG_UPDATE_BLOCK` (org/betterx/betternether/BlocksHelper.java:25)
- [x] `BlocksHelper.FLAG_SEND_CLIENT_CHANGES` (org/betterx/betternether/BlocksHelper.java:26)
- [x] `BlocksHelper.FLAG_NO_RERENDER` (org/betterx/betternether/BlocksHelper.java:27)
- [x] `BlocksHelper.FORSE_RERENDER` (org/betterx/betternether/BlocksHelper.java:28)
- [x] `BlocksHelper.FLAG_IGNORE_OBSERVERS` (org/betterx/betternether/BlocksHelper.java:29)
- [x] `BlocksHelper.SET_SILENT` (org/betterx/betternether/BlocksHelper.java:30)
- [x] `BlocksHelper.SET_UPDATE` (org/betterx/betternether/BlocksHelper.java:31)
- [ ] `BlocksHelper.HORIZONTAL`: it uses Minecraft's `Direction` (org/betterx/betternether/BlocksHelper.java:32)
- [ ] `BlocksHelper.OFFSETS`: it uses Minecraft's `Vec3i` (org/betterx/betternether/BlocksHelper.java:33)
- [ ] `BlocksHelper.chunkBounds(2)`: it uses Minecraft's `BoundingBox` (org/betterx/betternether/BlocksHelper.java:63)
- [ ] `BlocksHelper.chunkBounds(4)`: it uses Minecraft's `BoundingBox` (org/betterx/betternether/BlocksHelper.java:69)
- [ ] `BlocksHelper.decorationBounds(2)`: it uses Minecraft's `BoundingBox` (org/betterx/betternether/BlocksHelper.java:75)
- [ ] `BlocksHelper.decorationBounds(4)`: it uses Minecraft's `BoundingBox` (org/betterx/betternether/BlocksHelper.java:81)
- [ ] `BlocksHelper.isInsideHorizontally(3)`: it uses Minecraft's `BoundingBox` (org/betterx/betternether/BlocksHelper.java:85)
- [ ] `BlocksHelper.isInsideHorizontally(2)`: it uses Minecraft's `BoundingBox` (org/betterx/betternether/BlocksHelper.java:89)
- [ ] `BlocksHelper.isNetherrack(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/BlocksHelper.java:93)
- [ ] `BlocksHelper.isSoulSand(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/BlocksHelper.java:97)
- [ ] `BlocksHelper.isNetherGround(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/BlocksHelper.java:101)
- [ ] `BlocksHelper.isSculkLike(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/BlocksHelper.java:105)
- [ ] `BlocksHelper.isNetherGroundMagma(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/BlocksHelper.java:109)
- [ ] `BlocksHelper.isNetherMycelium(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/BlocksHelper.java:113)
- [ ] `BlocksHelper.setWithUpdate(4)`: it uses Minecraft's `LevelAccessor` (org/betterx/betternether/BlocksHelper.java:117)
- [ ] `BlocksHelper.setWithUpdate(3)`: it uses Minecraft's `LevelAccessor` (org/betterx/betternether/BlocksHelper.java:123)
- [ ] `BlocksHelper.setWithoutUpdate(4)`: it uses Minecraft's `LevelAccessor` (org/betterx/betternether/BlocksHelper.java:127)
- [ ] `BlocksHelper.setWithoutUpdate(3)`: it uses Minecraft's `LevelAccessor` (org/betterx/betternether/BlocksHelper.java:133)
- [ ] `BlocksHelper.upRay(3)`: it uses Minecraft's `LevelAccessor` (org/betterx/betternether/BlocksHelper.java:137)
- [ ] `BlocksHelper.downRay(3)`: it uses Minecraft's `LevelAccessor` (org/betterx/betternether/BlocksHelper.java:147)
- [ ] `BlocksHelper.rotateHorizontal(3)`: it uses Minecraft's `BlockState` (org/betterx/betternether/BlocksHelper.java:157)
- [ ] `BlocksHelper.mirrorHorizontal(3)`: it uses Minecraft's `BlockState` (org/betterx/betternether/BlocksHelper.java:161)
- [ ] `BlocksHelper.getLengthDown(3)`: it uses Minecraft's `ServerLevel` (org/betterx/betternether/BlocksHelper.java:165)
- [ ] `BlocksHelper.isFertile(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/BlocksHelper.java:175)
- [ ] `BlocksHelper.cover(6)`: it uses Minecraft's `LevelAccessor` (org/betterx/betternether/BlocksHelper.java:179)
- [ ] `BlocksHelper.isNylium(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/BlocksHelper.java:208)
- [ ] `BlocksHelper.createLogIfFree(5)`: it uses Minecraft's `LevelAccessor` (org/betterx/betternether/BlocksHelper.java:212)

### `BaseBlockCommonSapling`: it extends Minecraft's `Block`

- [ ] `BaseBlockCommonSapling.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BaseBlockCommonSapling.java:23)
- [ ] `BaseBlockCommonSapling.BaseBlockCommonSapling(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BaseBlockCommonSapling.java:26)
- [ ] `BaseBlockCommonSapling.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BaseBlockCommonSapling.java:31)
- [ ] `BaseBlockCommonSapling.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BaseBlockCommonSapling.java:35)
- [ ] `BaseBlockCommonSapling.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BaseBlockCommonSapling.java:48)
- [ ] `BaseBlockCommonSapling.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BaseBlockCommonSapling.java:52)
- [ ] `BaseBlockCommonSapling.canGrowTerrain(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BaseBlockCommonSapling.java:56)
- [ ] `BaseBlockCommonSapling.grow(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BaseBlockCommonSapling.java:60)
- [ ] `BaseBlockCommonSapling.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BaseBlockCommonSapling.java:64)
- [ ] `BaseBlockCommonSapling.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BaseBlockCommonSapling.java:68)

### `BaseBlockMold`: it extends Minecraft's `Block`

- [ ] `BaseBlockMold.BaseBlockMold(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BaseBlockMold.java:19)
- [ ] `BaseBlockMold.getShadeBrightness(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BaseBlockMold.java:23)
- [ ] `BaseBlockMold.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BaseBlockMold.java:28)
- [ ] `BaseBlockMold.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BaseBlockMold.java:41)

### `BaseBlockNetherGrass`: it extends `BasePlantBlock` (org.betterx.bclib.blocks)

- [ ] `BaseBlockNetherGrass.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BaseBlockNetherGrass.java:20)
- [ ] `BaseBlockNetherGrass.BaseBlockNetherGrass(1)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betternether/blocks/BaseBlockNetherGrass.java:22)
- [ ] `BaseBlockNetherGrass.getShadeBrightness(3)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betternether/blocks/BaseBlockNetherGrass.java:26)
- [ ] `BaseBlockNetherGrass.getShape(4)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betternether/blocks/BaseBlockNetherGrass.java:31)
- [ ] `BaseBlockNetherGrass.updateShape(8)`: its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)) (org/betterx/betternether/blocks/BaseBlockNetherGrass.java:36)

### `BlockAgave`: it extends `BlockCommonPlant`

- [ ] `BlockAgave.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockAgave.java:21)
- [ ] `BlockAgave.BlockAgave(1)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockAgave.java:23)
- [ ] `BlockAgave.getShape(4)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockAgave.java:27)
- [ ] `BlockAgave.updateShape(8)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockAgave.java:32)
- [ ] `BlockAgave.entityInside(6)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockAgave.java:46)
- [ ] `BlockAgave.canSurvive(3)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockAgave.java:52)

### `BlockAnchorTreeSapling`: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)

- [ ] `BlockAnchorTreeSapling.HANGING`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BlockAnchorTreeSapling.java:27)
- [ ] `BlockAnchorTreeSapling.BlockAnchorTreeSapling(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockAnchorTreeSapling.java:29)
- [ ] `BlockAnchorTreeSapling.createBlockStateDefinition(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockAnchorTreeSapling.java:39)
- [ ] `BlockAnchorTreeSapling.supports(2)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockAnchorTreeSapling.java:44)
- [ ] `BlockAnchorTreeSapling.getStateForPlacement(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockAnchorTreeSapling.java:48)
- [ ] `BlockAnchorTreeSapling.canSurvive(3)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockAnchorTreeSapling.java:64)
- [ ] `BlockAnchorTreeSapling.getShape(4)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockAnchorTreeSapling.java:68)
- [ ] `BlockAnchorTreeSapling.megaFeatureEnabled(3)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockAnchorTreeSapling.java:72)
- [ ] `BlockAnchorTreeSapling.smallFeatureEnabled(2)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockAnchorTreeSapling.java:76)
- [ ] `BlockAnchorTreeSapling.mayPlaceOn(3)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockAnchorTreeSapling.java:80)

### `BlockAnchorTreeVine`: it extends Minecraft's `Block`

- [ ] `BlockAnchorTreeVine.SHAPE_SELECTION`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockAnchorTreeVine.java:29)
- [ ] `BlockAnchorTreeVine.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockAnchorTreeVine.java:30)
- [ ] `BlockAnchorTreeVine.BlockAnchorTreeVine(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockAnchorTreeVine.java:32)
- [ ] `BlockAnchorTreeVine.getLuminance(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/blocks/BlockAnchorTreeVine.java:36)
- [ ] `BlockAnchorTreeVine.getOffsetType(0)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockAnchorTreeVine.java:40)
- [ ] `BlockAnchorTreeVine.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockAnchorTreeVine.java:44)
- [ ] `BlockAnchorTreeVine.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockAnchorTreeVine.java:48)
- [ ] `BlockAnchorTreeVine.getShadeBrightness(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockAnchorTreeVine.java:53)
- [ ] `BlockAnchorTreeVine.propagatesSkylightDown(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockAnchorTreeVine.java:58)
- [ ] `BlockAnchorTreeVine.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockAnchorTreeVine.java:62)
- [ ] `BlockAnchorTreeVine.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockAnchorTreeVine.java:77)

### `BlockBarrelCactus`: it extends `BlockCommonPlant`

- [ ] `BlockBarrelCactus.EMPTY`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockBarrelCactus.java:22)
- [ ] `BlockBarrelCactus.SHAPES`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockBarrelCactus.java:23)
- [ ] `BlockBarrelCactus.BlockBarrelCactus(1)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockBarrelCactus.java:30)
- [ ] `BlockBarrelCactus.canSurvive(3)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockBarrelCactus.java:34)
- [ ] `BlockBarrelCactus.updateShape(8)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockBarrelCactus.java:39)
- [ ] `BlockBarrelCactus.entityInside(6)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockBarrelCactus.java:53)
- [ ] `BlockBarrelCactus.getShape(4)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockBarrelCactus.java:59)
- [ ] `BlockBarrelCactus.getCollisionShape(4)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockBarrelCactus.java:64)

### `BlockBlackApple`: it extends `BlockCommonPlant`

- [ ] `BlockBlackApple.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockBlackApple.java:17)
- [ ] `BlockBlackApple.BlockBlackApple(1)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockBlackApple.java:19)
- [ ] `BlockBlackApple.getShape(4)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockBlackApple.java:23)
- [ ] `BlockBlackApple.getCloneItemStack(4)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockBlackApple.java:27)
- [ ] `BlockBlackApple.canSurvive(3)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockBlackApple.java:32)

### `BlockBlackAppleSeed`: it extends `BlockCommonSapling`

- [ ] `BlockBlackAppleSeed.BlockBlackAppleSeed(1)`: its class isn't translated (it extends `BlockCommonSapling`) (org/betterx/betternether/blocks/BlockBlackAppleSeed.java:7)

### `BlockBlackBush`: it extends Minecraft's `Block`

- [ ] `BlockBlackBush.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockBlackBush.java:24)
- [ ] `BlockBlackBush.BlockBlackBush(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBlackBush.java:26)
- [ ] `BlockBlackBush.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBlackBush.java:30)
- [ ] `BlockBlackBush.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBlackBush.java:34)
- [ ] `BlockBlackBush.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBlackBush.java:38)
- [ ] `BlockBlackBush.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBlackBush.java:51)
- [ ] `BlockBlackBush.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBlackBush.java:55)
- [ ] `BlockBlackBush.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBlackBush.java:59)

### `BlockBlackVine`: it extends `BaseSimpleVineBlock` (org.betterx.bclib.blocks)

- [ ] `BlockBlackVine.BlockBlackVine(1)`: its class isn't translated (it extends `BaseSimpleVineBlock` (org.betterx.bclib.blocks)) (org/betterx/betternether/blocks/BlockBlackVine.java:8)

### `BlockBNPot`: it extends Minecraft's `Block`

- [ ] `BlockBNPot.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockBNPot.java:22)
- [ ] `BlockBNPot.BlockBNPot(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBNPot.java:24)
- [ ] `BlockBNPot.BlockBNPot(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBNPot.java:28)
- [ ] `BlockBNPot.useShapeForLightOcclusion(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBNPot.java:32)
- [ ] `BlockBNPot.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBNPot.java:36)
- [ ] `BlockBNPot.useItemOn(7)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBNPot.java:40)

### `BlockBNPot.Metal`: it extends `BlockBNPot`

- [ ] `BlockBNPot.Metal.Metal(1)`: its class isn't translated (it extends `BlockBNPot`) (org/betterx/betternether/blocks/BlockBNPot.java:62)
- [ ] `BlockBNPot.Metal.Metal(1)`: its class isn't translated (it extends `BlockBNPot`) (org/betterx/betternether/blocks/BlockBNPot.java:66)

### `BlockBNPot.Stone`: it extends `BlockBNPot`

- [ ] `BlockBNPot.Stone.Stone(1)`: its class isn't translated (it extends `BlockBNPot`) (org/betterx/betternether/blocks/BlockBNPot.java:72)
- [ ] `BlockBNPot.Stone.Stone(1)`: its class isn't translated (it extends `BlockBNPot`) (org/betterx/betternether/blocks/BlockBNPot.java:76)

### `BlockBNPot.Wood`: it extends `BlockBNPot`

- [ ] `BlockBNPot.Wood.Wood(1)`: its class isn't translated (it extends `BlockBNPot`) (org/betterx/betternether/blocks/BlockBNPot.java:82)
- [ ] `BlockBNPot.Wood.Wood(1)`: its class isn't translated (it extends `BlockBNPot`) (org/betterx/betternether/blocks/BlockBNPot.java:86)

### `BlockBoneMushroom`: it extends Minecraft's `Block`

- [ ] `BlockBoneMushroom.SHAPE_NORTH`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockBoneMushroom.java:31)
- [ ] `BlockBoneMushroom.SHAPE_SOUTH`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockBoneMushroom.java:32)
- [ ] `BlockBoneMushroom.SHAPE_WEST`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockBoneMushroom.java:33)
- [ ] `BlockBoneMushroom.SHAPE_EAST`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockBoneMushroom.java:34)
- [ ] `BlockBoneMushroom.SHAPE_UP`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockBoneMushroom.java:35)
- [ ] `BlockBoneMushroom.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockBoneMushroom.java:36)
- [ ] `BlockBoneMushroom.AGE`: it uses Minecraft's `IntegerProperty` (org/betterx/betternether/blocks/BlockBoneMushroom.java:37)
- [ ] `BlockBoneMushroom.BlockBoneMushroom(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBoneMushroom.java:39)
- [ ] `BlockBoneMushroom.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBoneMushroom.java:44)
- [ ] `BlockBoneMushroom.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBoneMushroom.java:48)
- [ ] `BlockBoneMushroom.getShadeBrightness(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBoneMushroom.java:64)
- [ ] `BlockBoneMushroom.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBoneMushroom.java:69)
- [ ] `BlockBoneMushroom.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBoneMushroom.java:80)
- [ ] `BlockBoneMushroom.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBoneMushroom.java:84)
- [ ] `BlockBoneMushroom.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBoneMushroom.java:88)
- [ ] `BlockBoneMushroom.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBoneMushroom.java:101)
- [ ] `BlockBoneMushroom.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBoneMushroom.java:109)

### `BlockBrownLargeMushroom`: it extends Minecraft's `Block`

- [ ] `BlockBrownLargeMushroom.TOP_CENTER_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:27)
- [ ] `BlockBrownLargeMushroom.TOP_EDGE_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:28)
- [ ] `BlockBrownLargeMushroom.MIDDLE_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:29)
- [ ] `BlockBrownLargeMushroom.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:30)
- [ ] `BlockBrownLargeMushroom.ROT_SIDE`: it uses BrownMushroomShape, which isn't translated (its constants carry values) (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:31)
- [ ] `BlockBrownLargeMushroom.ROT_CORNER`: it uses BrownMushroomShape, which isn't translated (its constants carry values) (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:37)
- [ ] `BlockBrownLargeMushroom.BlockBrownLargeMushroom(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:44)
- [ ] `BlockBrownLargeMushroom.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:48)
- [ ] `BlockBrownLargeMushroom.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:56)
- [ ] `BlockBrownLargeMushroom.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:60)
- [ ] `BlockBrownLargeMushroom.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:69)
- [ ] `BlockBrownLargeMushroom.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:86)
- [ ] `BlockBrownLargeMushroom.getRotationIndex(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:129)
- [ ] `BlockBrownLargeMushroom.rotOffset(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:139)
- [ ] `BlockBrownLargeMushroom.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:149)
- [ ] `BlockBrownLargeMushroom.getStateIfSame(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java:181)

### `BlockChestOfDrawers`: it extends Minecraft's `BaseEntityBlock`

- [ ] `BlockChestOfDrawers.BOUNDING_SHAPES`: it uses `EnumMap` (java.util) (org/betterx/betternether/blocks/BlockChestOfDrawers.java:38)
- [ ] `BlockChestOfDrawers.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockChestOfDrawers.java:50)
- [ ] `BlockChestOfDrawers.OPEN`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BlockChestOfDrawers.java:51)
- [ ] `BlockChestOfDrawers.BlockChestOfDrawers(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (org/betterx/betternether/blocks/BlockChestOfDrawers.java:53)
- [ ] `BlockChestOfDrawers.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (org/betterx/betternether/blocks/BlockChestOfDrawers.java:58)
- [ ] `BlockChestOfDrawers.getShape(4)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (org/betterx/betternether/blocks/BlockChestOfDrawers.java:62)
- [ ] `BlockChestOfDrawers.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (org/betterx/betternether/blocks/BlockChestOfDrawers.java:66)
- [ ] `BlockChestOfDrawers.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (org/betterx/betternether/blocks/BlockChestOfDrawers.java:70)
- [ ] `BlockChestOfDrawers.useItemOn(7)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (org/betterx/betternether/blocks/BlockChestOfDrawers.java:74)
- [ ] `BlockChestOfDrawers.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (org/betterx/betternether/blocks/BlockChestOfDrawers.java:87)
- [ ] `BlockChestOfDrawers.getDrops(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (org/betterx/betternether/blocks/BlockChestOfDrawers.java:91)
- [ ] `BlockChestOfDrawers.rotate(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (org/betterx/betternether/blocks/BlockChestOfDrawers.java:99)
- [ ] `BlockChestOfDrawers.mirror(2)`: its class isn't translated (it extends Minecraft's `BaseEntityBlock`) (org/betterx/betternether/blocks/BlockChestOfDrawers.java:103)

### `BlockCincinnasiteAnvil`: it extends Minecraft's `AnvilBlock`

- [ ] `BlockCincinnasiteAnvil.BlockCincinnasiteAnvil(1)`: its class isn't translated (it extends Minecraft's `AnvilBlock`) (org/betterx/betternether/blocks/BlockCincinnasiteAnvil.java:7)

### `BlockCincinnasiteForge`: it extends Minecraft's `AbstractFurnaceBlock`

- [ ] `BlockCincinnasiteForge.BlockCincinnasiteForge(1)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`) (org/betterx/betternether/blocks/BlockCincinnasiteForge.java:27)
- [ ] `BlockCincinnasiteForge.getLuminance(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/blocks/BlockCincinnasiteForge.java:31)
- [ ] `BlockCincinnasiteForge.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`) (org/betterx/betternether/blocks/BlockCincinnasiteForge.java:35)
- [ ] `BlockCincinnasiteForge.getTicker(3)`: it's generic (org/betterx/betternether/blocks/BlockCincinnasiteForge.java:39)
- [ ] `BlockCincinnasiteForge.openContainer(3)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`) (org/betterx/betternether/blocks/BlockCincinnasiteForge.java:43)
- [ ] `BlockCincinnasiteForge.animateTick(4)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`) (org/betterx/betternether/blocks/BlockCincinnasiteForge.java:51)

### `BlockCincinnasiteFrame`: it extends Minecraft's `Block`

- [ ] `BlockCincinnasiteFrame.BlockCincinnasiteFrame(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCincinnasiteFrame.java:13)
- [ ] `BlockCincinnasiteFrame.getShadeBrightness(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCincinnasiteFrame.java:17)
- [ ] `BlockCincinnasiteFrame.propagatesSkylightDown(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCincinnasiteFrame.java:22)
- [ ] `BlockCincinnasiteFrame.skipRendering(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCincinnasiteFrame.java:26)

### `BlockCincinnasitePedestal`: it extends Minecraft's `Block`

- [ ] `BlockCincinnasitePedestal.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockCincinnasitePedestal.java:12)
- [ ] `BlockCincinnasitePedestal.BlockCincinnasitePedestal(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCincinnasitePedestal.java:14)
- [ ] `BlockCincinnasitePedestal.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCincinnasitePedestal.java:18)

### `BlockCincinnasitPillar`: it extends Minecraft's `Block`

- [ ] `BlockCincinnasitPillar.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockCincinnasitPillar.java:16)
- [ ] `BlockCincinnasitPillar.BlockCincinnasitPillar(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCincinnasitPillar.java:18)
- [ ] `BlockCincinnasitPillar.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCincinnasitPillar.java:22)
- [ ] `BlockCincinnasitPillar.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCincinnasitPillar.java:26)

### `BlockCommonPlant`: it extends Minecraft's `Block`

- [ ] `BlockCommonPlant.AGE`: it uses Minecraft's `IntegerProperty` (org/betterx/betternether/blocks/BlockCommonPlant.java:23)
- [ ] `BlockCommonPlant.BlockCommonPlant(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCommonPlant.java:25)
- [ ] `BlockCommonPlant.getMaxAge(0)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCommonPlant.java:29)
- [ ] `BlockCommonPlant.getAgeProperty(0)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCommonPlant.java:33)
- [ ] `BlockCommonPlant.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCommonPlant.java:37)
- [ ] `BlockCommonPlant.canSurvive(3)`: it has no body (org/betterx/betternether/blocks/BlockCommonPlant.java:41)
- [ ] `BlockCommonPlant.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCommonPlant.java:43)
- [ ] `BlockCommonPlant.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCommonPlant.java:56)
- [ ] `BlockCommonPlant.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCommonPlant.java:60)
- [ ] `BlockCommonPlant.canGrowTerrain(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCommonPlant.java:69)
- [ ] `BlockCommonPlant.grow(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCommonPlant.java:78)
- [ ] `BlockCommonPlant.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCommonPlant.java:83)
- [ ] `BlockCommonPlant.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockCommonPlant.java:87)

### `BlockCommonSapling`: it extends `BaseBlockCommonSapling`

- [ ] `BlockCommonSapling.BlockCommonSapling(2)`: its class isn't translated (it extends `BaseBlockCommonSapling`) (org/betterx/betternether/blocks/BlockCommonSapling.java:12)
- [ ] `BlockCommonSapling.canSurvive(3)`: its class isn't translated (it extends `BaseBlockCommonSapling`) (org/betterx/betternether/blocks/BlockCommonSapling.java:16)

### `BlockEggPlant`: it extends `BlockCommonPlant`

- [ ] `BlockEggPlant.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockEggPlant.java:35)
- [ ] `BlockEggPlant.DESTRUCTED`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BlockEggPlant.java:36)
- [ ] `BlockEggPlant.BlockEggPlant(1)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockEggPlant.java:40)
- [ ] `BlockEggPlant.createBlockStateDefinition(1)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockEggPlant.java:47)
- [ ] `BlockEggPlant.canSurvive(3)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockEggPlant.java:53)
- [ ] `BlockEggPlant.getShape(4)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockEggPlant.java:58)
- [ ] `BlockEggPlant.animateTick(4)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockEggPlant.java:62)
- [ ] `BlockEggPlant.entityInside(6)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockEggPlant.java:77)
- [ ] `BlockEggPlant.grow(4)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockEggPlant.java:112)

### `BlockEyeball`: it extends `BlockEyeBase`

- [ ] `BlockEyeball.BlockEyeball(1)`: its class isn't translated (it extends `BlockEyeBase`) (org/betterx/betternether/blocks/BlockEyeball.java:17)
- [ ] `BlockEyeball.animateTick(4)`: its class isn't translated (it extends `BlockEyeBase`) (org/betterx/betternether/blocks/BlockEyeball.java:21)
- [ ] `BlockEyeball.randomTick(4)`: its class isn't translated (it extends `BlockEyeBase`) (org/betterx/betternether/blocks/BlockEyeball.java:31)

### `BlockEyeballSmall`: it extends `BlockEyeBase`

- [ ] `BlockEyeballSmall.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockEyeballSmall.java:17)
- [ ] `BlockEyeballSmall.BlockEyeballSmall(1)`: its class isn't translated (it extends `BlockEyeBase`) (org/betterx/betternether/blocks/BlockEyeballSmall.java:19)
- [ ] `BlockEyeballSmall.getShape(4)`: its class isn't translated (it extends `BlockEyeBase`) (org/betterx/betternether/blocks/BlockEyeballSmall.java:23)
- [ ] `BlockEyeballSmall.animateTick(4)`: its class isn't translated (it extends `BlockEyeBase`) (org/betterx/betternether/blocks/BlockEyeballSmall.java:27)
- [ ] `BlockEyeballSmall.canSuffocate(3)`: its class isn't translated (it extends `BlockEyeBase`) (org/betterx/betternether/blocks/BlockEyeballSmall.java:37)
- [ ] `BlockEyeballSmall.isSimpleFullBlock(3)`: its class isn't translated (it extends `BlockEyeBase`) (org/betterx/betternether/blocks/BlockEyeballSmall.java:41)

### `BlockEyeBase`: it extends Minecraft's `Block`

- [ ] `BlockEyeBase.BlockEyeBase(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockEyeBase.java:22)
- [ ] `BlockEyeBase.allowsSpawning(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockEyeBase.java:26)
- [ ] `BlockEyeBase.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockEyeBase.java:30)
- [ ] `BlockEyeBase.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockEyeBase.java:47)

### `BlockEyeSeed`: it extends Minecraft's `Block`

- [ ] `BlockEyeSeed.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockEyeSeed.java:24)
- [ ] `BlockEyeSeed.BlockEyeSeed(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockEyeSeed.java:26)
- [ ] `BlockEyeSeed.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockEyeSeed.java:30)
- [ ] `BlockEyeSeed.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockEyeSeed.java:34)
- [ ] `BlockEyeSeed.canGrowRandomly(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockEyeSeed.java:38)
- [ ] `BlockEyeSeed.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockEyeSeed.java:42)
- [ ] `BlockEyeSeed.grow(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockEyeSeed.java:46)
- [ ] `BlockEyeSeed.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockEyeSeed.java:50)
- [ ] `BlockEyeSeed.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockEyeSeed.java:54)
- [ ] `BlockEyeSeed.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockEyeSeed.java:58)
- [ ] `BlockEyeSeed.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockEyeSeed.java:71)

### `BlockEyeVine`: it extends `BaseVineBlock` (org.betterx.bclib.blocks)

- [ ] `BlockEyeVine.BlockEyeVine(1)`: its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)) (org/betterx/betternether/blocks/BlockEyeVine.java:18)
- [ ] `BlockEyeVine.updateShape(8)`: its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)) (org/betterx/betternether/blocks/BlockEyeVine.java:22)
- [ ] `BlockEyeVine.getCloneItemStack(4)`: its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)) (org/betterx/betternether/blocks/BlockEyeVine.java:37)

### `BlockFarmland`: it extends Minecraft's `Block`

- [ ] `BlockFarmland.BlockFarmland(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockFarmland.java:7)

### `BlockFeatherFern`: it extends `BlockCommonPlant`

- [ ] `BlockFeatherFern.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockFeatherFern.java:15)
- [ ] `BlockFeatherFern.BlockFeatherFern(1)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockFeatherFern.java:17)
- [ ] `BlockFeatherFern.getShape(4)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockFeatherFern.java:21)
- [ ] `BlockFeatherFern.canSurvive(3)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockFeatherFern.java:26)

### `BlockFireBowl`: it extends Minecraft's `Block`

- [ ] `BlockFireBowl.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockFireBowl.java:34)
- [ ] `BlockFireBowl.CULL_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockFireBowl.java:35)
- [ ] `BlockFireBowl.FIRE`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BlockFireBowl.java:41)
- [ ] `BlockFireBowl.BlockFireBowl(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockFireBowl.java:43)
- [ ] `BlockFireBowl.BlockFireBowl(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockFireBowl.java:48)
- [ ] `BlockFireBowl.getLuminance(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/blocks/BlockFireBowl.java:53)
- [ ] `BlockFireBowl.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockFireBowl.java:57)
- [ ] `BlockFireBowl.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockFireBowl.java:61)
- [ ] `BlockFireBowl.getOcclusionShape(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockFireBowl.java:65)
- [ ] `BlockFireBowl.useItemOn(7)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockFireBowl.java:69)
- [ ] `BlockFireBowl.stepOn(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockFireBowl.java:91)
- [ ] `BlockFireBowl.animateTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockFireBowl.java:97)

### `BlockFireBowl.Metal`: it extends `BlockFireBowl`

- [ ] `BlockFireBowl.Metal.Metal(1)`: its class isn't translated (it extends `BlockFireBowl`) (org/betterx/betternether/blocks/BlockFireBowl.java:120)
- [ ] `BlockFireBowl.Metal.Metal(1)`: its class isn't translated (it extends `BlockFireBowl`) (org/betterx/betternether/blocks/BlockFireBowl.java:124)

### `BlockFireBowl.Stone`: it extends `BlockFireBowl`

- [ ] `BlockFireBowl.Stone.Stone(1)`: its class isn't translated (it extends `BlockFireBowl`) (org/betterx/betternether/blocks/BlockFireBowl.java:130)
- [ ] `BlockFireBowl.Stone.Stone(1)`: its class isn't translated (it extends `BlockFireBowl`) (org/betterx/betternether/blocks/BlockFireBowl.java:134)

### `BlockFireBowl.Wood`: it extends `BlockFireBowl`

- [ ] `BlockFireBowl.Wood.Wood(1)`: its class isn't translated (it extends `BlockFireBowl`) (org/betterx/betternether/blocks/BlockFireBowl.java:140)
- [ ] `BlockFireBowl.Wood.Wood(1)`: its class isn't translated (it extends `BlockFireBowl`) (org/betterx/betternether/blocks/BlockFireBowl.java:144)

### `BlockGeyser`: it extends Minecraft's `Block`

- [ ] `BlockGeyser.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockGeyser.java:26)
- [ ] `BlockGeyser.BlockGeyser(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGeyser.java:28)
- [ ] `BlockGeyser.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGeyser.java:32)
- [ ] `BlockGeyser.animateTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGeyser.java:36)
- [ ] `BlockGeyser.stepOn(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGeyser.java:81)
- [ ] `BlockGeyser.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGeyser.java:90)
- [ ] `BlockGeyser.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGeyser.java:94)

### `BlockGiantLucis`: it extends Minecraft's `HugeMushroomBlock`

- [ ] `BlockGiantLucis.BlockGiantLucis(1)`: its class isn't translated (it extends Minecraft's `HugeMushroomBlock`) (org/betterx/betternether/blocks/BlockGiantLucis.java:7)

### `BlockGiantMold`: it extends Minecraft's `Block`

- [ ] `BlockGiantMold.TOP_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockGiantMold.java:27)
- [ ] `BlockGiantMold.MIDDLE_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockGiantMold.java:28)
- [ ] `BlockGiantMold.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockGiantMold.java:29)
- [ ] `BlockGiantMold.BlockGiantMold(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGiantMold.java:31)
- [ ] `BlockGiantMold.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGiantMold.java:35)
- [ ] `BlockGiantMold.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGiantMold.java:39)
- [ ] `BlockGiantMold.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGiantMold.java:43)
- [ ] `BlockGiantMold.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGiantMold.java:51)

### `BlockGiantMoldSapling`: it extends Minecraft's `Block`

- [ ] `BlockGiantMoldSapling.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockGiantMoldSapling.java:25)
- [x] `BlockGiantMoldSapling.BONEMEAL_SUCCESS_CHANCE` (org/betterx/betternether/blocks/BlockGiantMoldSapling.java:26)
- [ ] `BlockGiantMoldSapling.BlockGiantMoldSapling(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGiantMoldSapling.java:28)
- [ ] `BlockGiantMoldSapling.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGiantMoldSapling.java:32)
- [ ] `BlockGiantMoldSapling.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGiantMoldSapling.java:36)
- [ ] `BlockGiantMoldSapling.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGiantMoldSapling.java:40)
- [ ] `BlockGiantMoldSapling.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGiantMoldSapling.java:53)
- [ ] `BlockGiantMoldSapling.canGrowRandomly(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGiantMoldSapling.java:57)
- [ ] `BlockGiantMoldSapling.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGiantMoldSapling.java:61)
- [ ] `BlockGiantMoldSapling.grow(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGiantMoldSapling.java:65)
- [ ] `BlockGiantMoldSapling.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGiantMoldSapling.java:69)
- [ ] `BlockGiantMoldSapling.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGiantMoldSapling.java:73)

### `BlockGloomsculkCrystal`: it extends Minecraft's `Block`

- [ ] `BlockGloomsculkCrystal.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockGloomsculkCrystal.java:23)
- [ ] `BlockGloomsculkCrystal.SHAPES`: it uses `EnumMap` (java.util) (org/betterx/betternether/blocks/BlockGloomsculkCrystal.java:24)
- [ ] `BlockGloomsculkCrystal.BlockGloomsculkCrystal(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomsculkCrystal.java:26)
- [ ] `BlockGloomsculkCrystal.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomsculkCrystal.java:31)
- [ ] `BlockGloomsculkCrystal.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomsculkCrystal.java:35)
- [ ] `BlockGloomsculkCrystal.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomsculkCrystal.java:39)
- [ ] `BlockGloomsculkCrystal.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomsculkCrystal.java:43)
- [ ] `BlockGloomsculkCrystal.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomsculkCrystal.java:49)

### `BlockGloomsculkGeode`: it extends Minecraft's `Block`

- [x] `BlockGloomsculkGeode.BUD_CHANCE` (org/betterx/betternether/blocks/BlockGloomsculkGeode.java:19)
- [ ] `BlockGloomsculkGeode.BlockGloomsculkGeode(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomsculkGeode.java:21)
- [ ] `BlockGloomsculkGeode.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomsculkGeode.java:25)
- [ ] `BlockGloomsculkGeode.playerDestroy(6)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomsculkGeode.java:51)

### `BlockGloomwispVine`: it extends Minecraft's `Block`

- [x] `BlockGloomwispVine.MAX_HEIGHT` (org/betterx/betternether/blocks/BlockGloomwispVine.java:54)
- [x] `BlockGloomwispVine.MAX_NATURAL_HEIGHT` (org/betterx/betternether/blocks/BlockGloomwispVine.java:55)
- [x] `BlockGloomwispVine.GROW_CHANCE` (org/betterx/betternether/blocks/BlockGloomwispVine.java:56)
- [x] `BlockGloomwispVine.MIN_HEIGHT` (org/betterx/betternether/blocks/BlockGloomwispVine.java:57)
- [x] `BlockGloomwispVine.SMOKE_MOTES` (org/betterx/betternether/blocks/BlockGloomwispVine.java:58)
- [x] `BlockGloomwispVine.SHEAR_PARTICLES` (org/betterx/betternether/blocks/BlockGloomwispVine.java:59)
- [x] `BlockGloomwispVine.DISTURB_PARTICLES` (org/betterx/betternether/blocks/BlockGloomwispVine.java:60)
- [x] `BlockGloomwispVine.DISTURB_SPEED_SQR` (org/betterx/betternether/blocks/BlockGloomwispVine.java:61)
- [x] `BlockGloomwispVine.BOUNDARY_EPSILON` (org/betterx/betternether/blocks/BlockGloomwispVine.java:62)
- [x] `BlockGloomwispVine.CHIME_CHANCE` (org/betterx/betternether/blocks/BlockGloomwispVine.java:63)
- [x] `BlockGloomwispVine.CHIME_VOLUME` (org/betterx/betternether/blocks/BlockGloomwispVine.java:64)
- [x] `BlockGloomwispVine.CHIME_PITCH_MIN` (org/betterx/betternether/blocks/BlockGloomwispVine.java:65)
- [x] `BlockGloomwispVine.CHIME_PITCH_SPREAD` (org/betterx/betternether/blocks/BlockGloomwispVine.java:66)
- [x] `BlockGloomwispVine.BOON_VOLUME` (org/betterx/betternether/blocks/BlockGloomwispVine.java:67)
- [x] `BlockGloomwispVine.BOON_PITCH_MIN` (org/betterx/betternether/blocks/BlockGloomwispVine.java:68)
- [x] `BlockGloomwispVine.BOON_PITCH_SPREAD` (org/betterx/betternether/blocks/BlockGloomwispVine.java:69)
- [x] `BlockGloomwispVine.EXPERIENCE_CHANCE` (org/betterx/betternether/blocks/BlockGloomwispVine.java:70)
- [x] `BlockGloomwispVine.EXPERIENCE_MIN` (org/betterx/betternether/blocks/BlockGloomwispVine.java:71)
- [x] `BlockGloomwispVine.EXPERIENCE_SPREAD` (org/betterx/betternether/blocks/BlockGloomwispVine.java:72)
- [x] `BlockGloomwispVine.RECHARGE_TICKS` (org/betterx/betternether/blocks/BlockGloomwispVine.java:73)
- [x] `BlockGloomwispVine.RECHARGE_SPREAD_TICKS` (org/betterx/betternether/blocks/BlockGloomwispVine.java:74)
- [x] `BlockGloomwispVine.SCATTER_RADIUS` (org/betterx/betternether/blocks/BlockGloomwispVine.java:75)
- [x] `BlockGloomwispVine.SHED_HEIGHT` (org/betterx/betternether/blocks/BlockGloomwispVine.java:76)
- [ ] `BlockGloomwispVine.STEM_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockGloomwispVine.java:77)
- [ ] `BlockGloomwispVine.HEAD_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockGloomwispVine.java:78)
- [ ] `BlockGloomwispVine.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockGloomwispVine.java:79)
- [ ] `BlockGloomwispVine.PERSISTENT`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BlockGloomwispVine.java:80)
- [ ] `BlockGloomwispVine.ROTATION`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockGloomwispVine.java:81)
- [ ] `BlockGloomwispVine.OFFSET`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BlockGloomwispVine.java:82)
- [ ] `BlockGloomwispVine.SPENT`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BlockGloomwispVine.java:83)
- [ ] `BlockGloomwispVine.BlockGloomwispVine(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:85)
- [ ] `BlockGloomwispVine.stateAwareOffset(1)`: it uses BlockBehaviourPropertiesAccessor, which isn't translated (a Mixin class: its handlers are the injections, in the bundle's helpers) (org/betterx/betternether/blocks/BlockGloomwispVine.java:96)
- [ ] `BlockGloomwispVine.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:106)
- [ ] `BlockGloomwispVine.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:110)
- [ ] `BlockGloomwispVine.getShadeBrightness(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:118)
- [ ] `BlockGloomwispVine.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:123)
- [ ] `BlockGloomwispVine.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:129)
- [ ] `BlockGloomwispVine.shapeAt(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:142)
- [ ] `BlockGloomwispVine.useItemOn(7)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:150)
- [ ] `BlockGloomwispVine.entityInside(6)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:173)
- [ ] `BlockGloomwispVine.shedExperience(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:204)
- [ ] `BlockGloomwispVine.drain(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:231)
- [ ] `BlockGloomwispVine.tick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:240)
- [ ] `BlockGloomwispVine.base(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:246)
- [ ] `BlockGloomwispVine.head(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:256)
- [ ] `BlockGloomwispVine.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:266)
- [ ] `BlockGloomwispVine.animateTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:272)
- [ ] `BlockGloomwispVine.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockGloomwispVine.java:291)

### `BlockGloomwoodSapling`: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)

- [ ] `BlockGloomwoodSapling.BlockGloomwoodSapling(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockGloomwoodSapling.java:21)
- [ ] `BlockGloomwoodSapling.mayPlaceOn(3)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockGloomwoodSapling.java:25)
- [ ] `BlockGloomwoodSapling.isValidBonemealTarget(4)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockGloomwoodSapling.java:29)
- [ ] `BlockGloomwoodSapling.isRandomGrowthSuccess(4)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockGloomwoodSapling.java:33)

### `BlockGoldenVine`: it extends `BaseSimpleVineBlock` (org.betterx.bclib.blocks)

- [ ] `BlockGoldenVine.BlockGoldenVine(1)`: its class isn't translated (it extends `BaseSimpleVineBlock` (org.betterx.bclib.blocks)) (org/betterx/betternether/blocks/BlockGoldenVine.java:7)

### `BlockGrayMold`: it extends `BlockMold`

- [ ] `BlockGrayMold.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockGrayMold.java:17)
- [ ] `BlockGrayMold.BlockGrayMold(1)`: its class isn't translated (it extends `BlockMold`) (org/betterx/betternether/blocks/BlockGrayMold.java:19)
- [ ] `BlockGrayMold.animateTick(4)`: its class isn't translated (it extends `BlockMold`) (org/betterx/betternether/blocks/BlockGrayMold.java:23)
- [ ] `BlockGrayMold.getShape(4)`: its class isn't translated (it extends `BlockMold`) (org/betterx/betternether/blocks/BlockGrayMold.java:32)

### `BlockHookMushroom`: it extends `BaseBlockMold`

- [ ] `BlockHookMushroom.BlockHookMushroom(1)`: its class isn't translated (it extends `BaseBlockMold`) (org/betterx/betternether/blocks/BlockHookMushroom.java:10)
- [ ] `BlockHookMushroom.canSurvive(3)`: its class isn't translated (it extends `BaseBlockMold`) (org/betterx/betternether/blocks/BlockHookMushroom.java:14)

### `BlockInkBush`: it extends `BlockCommonPlant`

- [ ] `BlockInkBush.BlockInkBush(1)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockInkBush.java:16)
- [ ] `BlockInkBush.getOcclusionShape(1)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockInkBush.java:20)
- [ ] `BlockInkBush.getCloneItemStack(4)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockInkBush.java:24)
- [ ] `BlockInkBush.canSurvive(3)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockInkBush.java:29)

### `BlockInkBushSeed`: it extends `BlockCommonSapling`

- [ ] `BlockInkBushSeed.BlockInkBushSeed(1)`: its class isn't translated (it extends `BlockCommonSapling`) (org/betterx/betternether/blocks/BlockInkBushSeed.java:7)

### `BlockJellyfishMushroom`: it extends Minecraft's `Block`

- [ ] `BlockJellyfishMushroom.TOP_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockJellyfishMushroom.java:28)
- [ ] `BlockJellyfishMushroom.MIDDLE_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockJellyfishMushroom.java:29)
- [ ] `BlockJellyfishMushroom.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockJellyfishMushroom.java:30)
- [ ] `BlockJellyfishMushroom.VISUAL`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockJellyfishMushroom.java:31)
- [ ] `BlockJellyfishMushroom.BlockJellyfishMushroom(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroom.java:33)
- [ ] `BlockJellyfishMushroom.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroom.java:37)
- [ ] `BlockJellyfishMushroom.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroom.java:41)
- [ ] `BlockJellyfishMushroom.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroom.java:45)
- [ ] `BlockJellyfishMushroom.getShadeBrightness(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroom.java:50)
- [ ] `BlockJellyfishMushroom.propagatesSkylightDown(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroom.java:55)
- [ ] `BlockJellyfishMushroom.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroom.java:59)
- [ ] `BlockJellyfishMushroom.fallOn(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroom.java:82)
- [ ] `BlockJellyfishMushroom.stepOn(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroom.java:92)

### `BlockJellyfishMushroomSapling`: it extends Minecraft's `Block`

- [ ] `BlockJellyfishMushroomSapling.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockJellyfishMushroomSapling.java:25)
- [x] `BlockJellyfishMushroomSapling.BONEMEAL_SUCCESS_CHANCE` (org/betterx/betternether/blocks/BlockJellyfishMushroomSapling.java:26)
- [ ] `BlockJellyfishMushroomSapling.BlockJellyfishMushroomSapling(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroomSapling.java:28)
- [ ] `BlockJellyfishMushroomSapling.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroomSapling.java:32)
- [ ] `BlockJellyfishMushroomSapling.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroomSapling.java:36)
- [ ] `BlockJellyfishMushroomSapling.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroomSapling.java:40)
- [ ] `BlockJellyfishMushroomSapling.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroomSapling.java:53)
- [ ] `BlockJellyfishMushroomSapling.canGrowRandomly(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroomSapling.java:57)
- [ ] `BlockJellyfishMushroomSapling.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroomSapling.java:61)
- [ ] `BlockJellyfishMushroomSapling.grow(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroomSapling.java:65)
- [ ] `BlockJellyfishMushroomSapling.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroomSapling.java:69)
- [ ] `BlockJellyfishMushroomSapling.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockJellyfishMushroomSapling.java:73)

### `BlockLucisMushroom`: it extends Minecraft's `Block`

- [ ] `BlockLucisMushroom.V_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockLucisMushroom.java:25)
- [ ] `BlockLucisMushroom.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockLucisMushroom.java:26)
- [ ] `BlockLucisMushroom.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockLucisMushroom.java:27)
- [ ] `BlockLucisMushroom.BlockLucisMushroom(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisMushroom.java:29)
- [ ] `BlockLucisMushroom.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisMushroom.java:37)
- [ ] `BlockLucisMushroom.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisMushroom.java:41)
- [ ] `BlockLucisMushroom.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisMushroom.java:45)
- [ ] `BlockLucisMushroom.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisMushroom.java:49)
- [ ] `BlockLucisMushroom.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisMushroom.java:95)

### `BlockLucisSpore`: it extends Minecraft's `Block`

- [ ] `BlockLucisSpore.BOUNDING_SHAPES`: it uses `EnumMap` (java.util) (org/betterx/betternether/blocks/BlockLucisSpore.java:35)
- [ ] `BlockLucisSpore.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockLucisSpore.java:47)
- [ ] `BlockLucisSpore.BlockLucisSpore(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisSpore.java:49)
- [ ] `BlockLucisSpore.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisSpore.java:54)
- [ ] `BlockLucisSpore.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisSpore.java:58)
- [ ] `BlockLucisSpore.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisSpore.java:62)
- [ ] `BlockLucisSpore.canGrowRandomly(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisSpore.java:66)
- [ ] `BlockLucisSpore.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisSpore.java:70)
- [ ] `BlockLucisSpore.grow(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisSpore.java:74)
- [ ] `BlockLucisSpore.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisSpore.java:78)
- [ ] `BlockLucisSpore.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisSpore.java:82)
- [ ] `BlockLucisSpore.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisSpore.java:95)
- [ ] `BlockLucisSpore.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisSpore.java:102)
- [ ] `BlockLucisSpore.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisSpore.java:106)
- [ ] `BlockLucisSpore.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisSpore.java:110)
- [ ] `BlockLucisSpore.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLucisSpore.java:117)

### `BlockLumabusSeed`: it extends Minecraft's `Block`

- [ ] `BlockLumabusSeed.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockLumabusSeed.java:24)
- [ ] `BlockLumabusSeed.BlockLumabusSeed(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLumabusSeed.java:27)
- [ ] `BlockLumabusSeed.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLumabusSeed.java:32)
- [ ] `BlockLumabusSeed.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLumabusSeed.java:36)
- [ ] `BlockLumabusSeed.canGrowRandomly(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLumabusSeed.java:40)
- [ ] `BlockLumabusSeed.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLumabusSeed.java:44)
- [ ] `BlockLumabusSeed.grow(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLumabusSeed.java:48)
- [ ] `BlockLumabusSeed.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLumabusSeed.java:52)
- [ ] `BlockLumabusSeed.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLumabusSeed.java:56)
- [ ] `BlockLumabusSeed.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLumabusSeed.java:61)
- [ ] `BlockLumabusSeed.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockLumabusSeed.java:74)

### `BlockLumabusVine`: it extends `BaseVineBlock` (org.betterx.bclib.blocks)

- [ ] `BlockLumabusVine.MIDDLE_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockLumabusVine.java:21)
- [ ] `BlockLumabusVine.BOTTOM_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockLumabusVine.java:22)
- [ ] `BlockLumabusVine.BlockLumabusVine(2)`: its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)) (org/betterx/betternether/blocks/BlockLumabusVine.java:25)
- [ ] `BlockLumabusVine.getLuminance(0)`: it uses Minecraft's `BlockState` (org/betterx/betternether/blocks/BlockLumabusVine.java:30)
- [ ] `BlockLumabusVine.getShape(4)`: its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)) (org/betterx/betternether/blocks/BlockLumabusVine.java:34)
- [ ] `BlockLumabusVine.getCloneItemStack(4)`: its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)) (org/betterx/betternether/blocks/BlockLumabusVine.java:38)

### `BlockMagmaFlower`: it extends `BlockCommonPlant`

- [ ] `BlockMagmaFlower.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockMagmaFlower.java:13)
- [ ] `BlockMagmaFlower.BlockMagmaFlower(1)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockMagmaFlower.java:15)
- [ ] `BlockMagmaFlower.getShape(4)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockMagmaFlower.java:19)
- [ ] `BlockMagmaFlower.canSurvive(3)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockMagmaFlower.java:23)

### `BlockMold`: it extends `BaseBlockMold`

- [ ] `BlockMold.BlockMold(1)`: its class isn't translated (it extends `BaseBlockMold`) (org/betterx/betternether/blocks/BlockMold.java:10)
- [ ] `BlockMold.canSurvive(3)`: its class isn't translated (it extends `BaseBlockMold`) (org/betterx/betternether/blocks/BlockMold.java:14)

### `BlockMoltenGloomsculk`: it extends `BlockTerrain`

- [x] `BlockMoltenGloomsculk.CINDER_CHANCE` (org/betterx/betternether/blocks/BlockMoltenGloomsculk.java:13)
- [x] `BlockMoltenGloomsculk.SPIT_CHANCE` (org/betterx/betternether/blocks/BlockMoltenGloomsculk.java:14)
- [ ] `BlockMoltenGloomsculk.BlockMoltenGloomsculk(1)`: its class isn't translated (it extends `BlockTerrain`) (org/betterx/betternether/blocks/BlockMoltenGloomsculk.java:16)
- [ ] `BlockMoltenGloomsculk.animateTick(4)`: its class isn't translated (it extends `BlockTerrain`) (org/betterx/betternether/blocks/BlockMoltenGloomsculk.java:20)

### `BlockMoltenGloomwoodLog`: it extends Minecraft's `RotatedPillarBlock`

- [x] `BlockMoltenGloomwoodLog.EMBERS` (org/betterx/betternether/blocks/BlockMoltenGloomwoodLog.java:16)
- [ ] `BlockMoltenGloomwoodLog.BlockMoltenGloomwoodLog(1)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (org/betterx/betternether/blocks/BlockMoltenGloomwoodLog.java:18)
- [ ] `BlockMoltenGloomwoodLog.animateTick(4)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (org/betterx/betternether/blocks/BlockMoltenGloomwoodLog.java:22)

### `BlockMossCover`: it extends `BaseBlockMold`

- [ ] `BlockMossCover.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockMossCover.java:14)
- [ ] `BlockMossCover.BlockMossCover(1)`: its class isn't translated (it extends `BaseBlockMold`) (org/betterx/betternether/blocks/BlockMossCover.java:16)
- [ ] `BlockMossCover.canSurvive(3)`: its class isn't translated (it extends `BaseBlockMold`) (org/betterx/betternether/blocks/BlockMossCover.java:20)
- [ ] `BlockMossCover.getShape(4)`: its class isn't translated (it extends `BaseBlockMold`) (org/betterx/betternether/blocks/BlockMossCover.java:24)

### `BlockMushroomFir`: it extends Minecraft's `Block`

- [ ] `BlockMushroomFir.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockMushroomFir.java:26)
- [ ] `BlockMushroomFir.BOTTOM_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockMushroomFir.java:27)
- [ ] `BlockMushroomFir.MIDDLE_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockMushroomFir.java:28)
- [ ] `BlockMushroomFir.TOP_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockMushroomFir.java:29)
- [ ] `BlockMushroomFir.SIDE_BIG_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockMushroomFir.java:30)
- [ ] `BlockMushroomFir.SIDE_SMALL_N_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockMushroomFir.java:31)
- [ ] `BlockMushroomFir.SIDE_SMALL_S_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockMushroomFir.java:32)
- [ ] `BlockMushroomFir.SIDE_SMALL_E_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockMushroomFir.java:33)
- [ ] `BlockMushroomFir.SIDE_SMALL_W_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockMushroomFir.java:34)
- [ ] `BlockMushroomFir.END_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockMushroomFir.java:35)
- [ ] `BlockMushroomFir.BlockMushroomFir(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockMushroomFir.java:37)
- [ ] `BlockMushroomFir.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockMushroomFir.java:41)
- [ ] `BlockMushroomFir.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockMushroomFir.java:45)
- [ ] `BlockMushroomFir.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockMushroomFir.java:72)
- [ ] `BlockMushroomFir.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockMushroomFir.java:80)
- [ ] `BlockMushroomFir.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockMushroomFir.java:101)
- [ ] `BlockMushroomFir.tick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockMushroomFir.java:118)

### `BlockMushroomFir.MushroomFirShape`: its constants carry values

- [ ] `BlockMushroomFir.MushroomFirShape.MushroomFirShape(1)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BlockMushroomFir.java:140)
- [ ] `BlockMushroomFir.MushroomFirShape.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BlockMushroomFir.java:144)
- [ ] `BlockMushroomFir.MushroomFirShape.toString(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BlockMushroomFir.java:148)

### `BlockMushroomFirSapling`: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)

- [ ] `BlockMushroomFirSapling.BlockMushroomFirSapling(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockMushroomFirSapling.java:18)
- [ ] `BlockMushroomFirSapling.mayPlaceOn(3)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockMushroomFirSapling.java:22)
- [ ] `BlockMushroomFirSapling.isRandomGrowthSuccess(4)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockMushroomFirSapling.java:26)

### `BlockNeonEquisetum`: it extends `Growing` (org.betterx.bclib.blocks.BaseVineBlock)

- [ ] `BlockNeonEquisetum.BlockNeonEquisetum(1)`: its class isn't translated (it extends `Growing` (org.betterx.bclib.blocks.BaseVineBlock)) (org/betterx/betternether/blocks/BlockNeonEquisetum.java:7)

### `BlockNetherCactus`: it extends Minecraft's `Block`

- [ ] `BlockNetherCactus.TOP_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockNetherCactus.java:27)
- [ ] `BlockNetherCactus.SIDE_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockNetherCactus.java:28)
- [ ] `BlockNetherCactus.TOP`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BlockNetherCactus.java:29)
- [ ] `BlockNetherCactus.BlockNetherCactus(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherCactus.java:31)
- [ ] `BlockNetherCactus.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherCactus.java:36)
- [ ] `BlockNetherCactus.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherCactus.java:40)
- [ ] `BlockNetherCactus.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherCactus.java:44)
- [ ] `BlockNetherCactus.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherCactus.java:62)
- [ ] `BlockNetherCactus.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherCactus.java:67)
- [ ] `BlockNetherCactus.entityInside(6)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherCactus.java:83)

### `BlockNetherFurnace`: it extends Minecraft's `AbstractFurnaceBlock`

- [ ] `BlockNetherFurnace.BlockNetherFurnace(1)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`) (org/betterx/betternether/blocks/BlockNetherFurnace.java:28)
- [ ] `BlockNetherFurnace.getLuminance(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/blocks/BlockNetherFurnace.java:32)
- [ ] `BlockNetherFurnace.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`) (org/betterx/betternether/blocks/BlockNetherFurnace.java:36)
- [ ] `BlockNetherFurnace.getTicker(3)`: it's generic (org/betterx/betternether/blocks/BlockNetherFurnace.java:40)
- [ ] `BlockNetherFurnace.openContainer(3)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`) (org/betterx/betternether/blocks/BlockNetherFurnace.java:45)
- [ ] `BlockNetherFurnace.animateTick(4)`: its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`) (org/betterx/betternether/blocks/BlockNetherFurnace.java:53)

### `BlockNetherGrass`: it extends `BaseBlockNetherGrass`

- [ ] `BlockNetherGrass.BlockNetherGrass(1)`: its class isn't translated (it extends `BaseBlockNetherGrass`) (org/betterx/betternether/blocks/BlockNetherGrass.java:6)

### `BlockNetherGrass.BoneGrass`: it extends `BaseBlockNetherGrass`

- [ ] `BlockNetherGrass.BoneGrass.BoneGrass(1)`: its class isn't translated (it extends `BaseBlockNetherGrass`) (org/betterx/betternether/blocks/BlockNetherGrass.java:11)

### `BlockNetherGrass.JunglePlant`: it extends `BlockNetherGrass`

- [ ] `BlockNetherGrass.JunglePlant.JunglePlant(1)`: its class isn't translated (it extends `BlockNetherGrass`) (org/betterx/betternether/blocks/BlockNetherGrass.java:17)

### `BlockNetherGrass.NetherGrass`: it extends `BlockNetherGrass`

- [ ] `BlockNetherGrass.NetherGrass.NetherGrass(1)`: its class isn't translated (it extends `BlockNetherGrass`) (org/betterx/betternether/blocks/BlockNetherGrass.java:23)

### `BlockNetherGrass.SepiaBoneGrass`: it extends `BaseBlockNetherGrass`

- [ ] `BlockNetherGrass.SepiaBoneGrass.SepiaBoneGrass(1)`: its class isn't translated (it extends `BaseBlockNetherGrass`) (org/betterx/betternether/blocks/BlockNetherGrass.java:29)

### `BlockNetherGrass.SwampGrass`: it extends `BlockNetherGrass`

- [ ] `BlockNetherGrass.SwampGrass.SwampGrass(1)`: its class isn't translated (it extends `BlockNetherGrass`) (org/betterx/betternether/blocks/BlockNetherGrass.java:35)

### `BlockNetherMycelium`: it extends Minecraft's `Block`

- [ ] `BlockNetherMycelium.IS_BLUE`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BlockNetherMycelium.java:25)
- [ ] `BlockNetherMycelium.BlockNetherMycelium(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherMycelium.java:28)
- [ ] `BlockNetherMycelium.setVegetationFeature(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherMycelium.java:33)
- [ ] `BlockNetherMycelium.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherMycelium.java:37)
- [ ] `BlockNetherMycelium.animateTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherMycelium.java:41)
- [ ] `BlockNetherMycelium.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherMycelium.java:47)
- [ ] `BlockNetherMycelium.getHostBlock(0)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherMycelium.java:51)
- [ ] `BlockNetherMycelium.getCoverFeature(0)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherMycelium.java:55)

### `BlockNetherReed`: it extends Minecraft's `Block`

- [ ] `BlockNetherReed.TOP`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BlockNetherReed.java:23)
- [ ] `BlockNetherReed.BlockNetherReed(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherReed.java:25)
- [ ] `BlockNetherReed.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherReed.java:30)
- [ ] `BlockNetherReed.getShadeBrightness(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherReed.java:34)
- [ ] `BlockNetherReed.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherReed.java:39)
- [ ] `BlockNetherReed.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherReed.java:71)
- [ ] `BlockNetherReed.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockNetherReed.java:84)

### `BlockNetherSakuraLeaves`: it extends `BNLeaves`

- [x] `BlockNetherSakuraLeaves.COLOR` (org/betterx/betternether/blocks/BlockNetherSakuraLeaves.java:16)
- [ ] `BlockNetherSakuraLeaves.BlockNetherSakuraLeaves(1)`: its class isn't translated (it extends `BNLeaves`) (org/betterx/betternether/blocks/BlockNetherSakuraLeaves.java:18)
- [ ] `BlockNetherSakuraLeaves.getShadeBrightness(3)`: its class isn't translated (it extends `BNLeaves`) (org/betterx/betternether/blocks/BlockNetherSakuraLeaves.java:22)
- [ ] `BlockNetherSakuraLeaves.propagatesSkylightDown(1)`: its class isn't translated (it extends `BNLeaves`) (org/betterx/betternether/blocks/BlockNetherSakuraLeaves.java:27)
- [ ] `BlockNetherSakuraLeaves.animateTick(4)`: its class isn't translated (it extends `BNLeaves`) (org/betterx/betternether/blocks/BlockNetherSakuraLeaves.java:31)
- [ ] `BlockNetherSakuraLeaves.getColor(3)`: its class isn't translated (it extends `BNLeaves`) (org/betterx/betternether/blocks/BlockNetherSakuraLeaves.java:44)

### `BlockNetherSakuraSapling`: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)

- [ ] `BlockNetherSakuraSapling.BlockNetherSakuraSapling(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockNetherSakuraSapling.java:14)
- [ ] `BlockNetherSakuraSapling.mayPlaceOn(3)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockNetherSakuraSapling.java:18)

### `BlockObsidianGlass`: it extends Minecraft's `Block`

- [ ] `BlockObsidianGlass.BlockObsidianGlass(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockObsidianGlass.java:13)
- [ ] `BlockObsidianGlass.getShadeBrightness(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockObsidianGlass.java:17)
- [ ] `BlockObsidianGlass.propagatesSkylightDown(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockObsidianGlass.java:22)
- [ ] `BlockObsidianGlass.skipRendering(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockObsidianGlass.java:26)

### `BlockOrangeMushroom`: it extends `BlockCommonPlant`

- [ ] `BlockOrangeMushroom.SHAPES`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockOrangeMushroom.java:14)
- [ ] `BlockOrangeMushroom.BlockOrangeMushroom(1)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockOrangeMushroom.java:21)
- [ ] `BlockOrangeMushroom.canSurvive(3)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockOrangeMushroom.java:25)
- [ ] `BlockOrangeMushroom.getShape(4)`: its class isn't translated (it extends `BlockCommonPlant`) (org/betterx/betternether/blocks/BlockOrangeMushroom.java:30)

### `BlockOre`: it extends Minecraft's `DropExperienceBlock`

- [ ] `BlockOre.BlockOre(2)`: its class isn't translated (it extends Minecraft's `DropExperienceBlock`) (org/betterx/betternether/blocks/BlockOre.java:8)

### `BlockPottedPlant`: it extends Minecraft's `Block`

- [ ] `BlockPottedPlant.PLANT`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockPottedPlant.java:24)
- [ ] `BlockPottedPlant.BlockPottedPlant(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockPottedPlant.java:26)
- [ ] `BlockPottedPlant.getLuminance(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/blocks/BlockPottedPlant.java:31)
- [ ] `BlockPottedPlant.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockPottedPlant.java:41)
- [ ] `BlockPottedPlant.propagatesSkylightDown(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockPottedPlant.java:47)
- [ ] `BlockPottedPlant.getCollisionShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockPottedPlant.java:51)
- [ ] `BlockPottedPlant.getBlockSupportShape(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockPottedPlant.java:55)
- [ ] `BlockPottedPlant.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockPottedPlant.java:59)
- [ ] `BlockPottedPlant.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockPottedPlant.java:63)
- [ ] `BlockPottedPlant.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockPottedPlant.java:67)
- [ ] `BlockPottedPlant.getPlant(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/blocks/BlockPottedPlant.java:80)

### `BlockRedLargeMushroom`: it extends Minecraft's `Block`

- [ ] `BlockRedLargeMushroom.TOP_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockRedLargeMushroom.java:27)
- [ ] `BlockRedLargeMushroom.MIDDLE_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockRedLargeMushroom.java:28)
- [ ] `BlockRedLargeMushroom.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockRedLargeMushroom.java:29)
- [ ] `BlockRedLargeMushroom.BlockRedLargeMushroom(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockRedLargeMushroom.java:31)
- [ ] `BlockRedLargeMushroom.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockRedLargeMushroom.java:35)
- [ ] `BlockRedLargeMushroom.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockRedLargeMushroom.java:39)
- [ ] `BlockRedLargeMushroom.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockRedLargeMushroom.java:43)
- [ ] `BlockRedLargeMushroom.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockRedLargeMushroom.java:48)

### `BlockRedMold`: it extends `BlockMold`

- [ ] `BlockRedMold.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockRedMold.java:13)
- [ ] `BlockRedMold.BlockRedMold(1)`: its class isn't translated (it extends `BlockMold`) (org/betterx/betternether/blocks/BlockRedMold.java:15)
- [ ] `BlockRedMold.getShape(4)`: its class isn't translated (it extends `BlockMold`) (org/betterx/betternether/blocks/BlockRedMold.java:19)

### `BlockReedsBlock`: it extends `BNPillar`

- [ ] `BlockReedsBlock.BlockReedsBlock(1)`: its class isn't translated (it extends `BNPillar`) (org/betterx/betternether/blocks/BlockReedsBlock.java:6)

### `BlockRubeusCone`: it extends Minecraft's `Block`

- [ ] `BlockRubeusCone.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockRubeusCone.java:18)
- [ ] `BlockRubeusCone.BlockRubeusCone(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockRubeusCone.java:20)
- [ ] `BlockRubeusCone.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockRubeusCone.java:24)
- [ ] `BlockRubeusCone.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockRubeusCone.java:28)
- [ ] `BlockRubeusCone.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockRubeusCone.java:34)

### `BlockRubeusLeaves`: it extends Minecraft's `TintedParticleLeavesBlock`

- [ ] `BlockRubeusLeaves.BlockRubeusLeaves(1)`: its class isn't translated (it extends Minecraft's `TintedParticleLeavesBlock`) (org/betterx/betternether/blocks/BlockRubeusLeaves.java:7)

### `BlockRubeusSapling`: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)

- [ ] `BlockRubeusSapling.BlockRubeusSapling(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockRubeusSapling.java:18)
- [ ] `BlockRubeusSapling.mayPlaceOn(3)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockRubeusSapling.java:22)
- [ ] `BlockRubeusSapling.isRandomGrowthSuccess(4)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockRubeusSapling.java:26)

### `BlockSmallLantern`: it extends Minecraft's `Block`

- [ ] `BlockSmallLantern.SHAPE_NORTH`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSmallLantern.java:26)
- [ ] `BlockSmallLantern.SHAPE_SOUTH`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSmallLantern.java:27)
- [ ] `BlockSmallLantern.SHAPE_WEST`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSmallLantern.java:28)
- [ ] `BlockSmallLantern.SHAPE_EAST`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSmallLantern.java:29)
- [ ] `BlockSmallLantern.SHAPE_UP`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSmallLantern.java:30)
- [ ] `BlockSmallLantern.SHAPE_DOWN`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSmallLantern.java:31)
- [ ] `BlockSmallLantern.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockSmallLantern.java:32)
- [ ] `BlockSmallLantern.BlockSmallLantern(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSmallLantern.java:34)
- [ ] `BlockSmallLantern.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSmallLantern.java:39)
- [ ] `BlockSmallLantern.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSmallLantern.java:43)
- [ ] `BlockSmallLantern.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSmallLantern.java:61)
- [ ] `BlockSmallLantern.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSmallLantern.java:71)
- [ ] `BlockSmallLantern.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSmallLantern.java:75)
- [ ] `BlockSmallLantern.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSmallLantern.java:79)
- [ ] `BlockSmallLantern.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSmallLantern.java:92)

### `BlockSmallLantern.Metal`: it extends `BlockSmallLantern`

- [ ] `BlockSmallLantern.Metal.Metal(1)`: its class isn't translated (it extends `BlockSmallLantern`) (org/betterx/betternether/blocks/BlockSmallLantern.java:111)

### `BlockSmoker`: it extends Minecraft's `Block`

- [ ] `BlockSmoker.TOP_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSmoker.java:27)
- [ ] `BlockSmoker.MIDDLE_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSmoker.java:28)
- [ ] `BlockSmoker.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockSmoker.java:29)
- [ ] `BlockSmoker.BlockSmoker(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSmoker.java:31)
- [ ] `BlockSmoker.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSmoker.java:36)
- [ ] `BlockSmoker.animateTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSmoker.java:40)
- [ ] `BlockSmoker.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSmoker.java:47)
- [ ] `BlockSmoker.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSmoker.java:51)
- [ ] `BlockSmoker.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSmoker.java:74)

### `BlockSoulGrass`: it extends `BaseBlockNetherGrass`

- [ ] `BlockSoulGrass.BlockSoulGrass(1)`: its class isn't translated (it extends `BaseBlockNetherGrass`) (org/betterx/betternether/blocks/BlockSoulGrass.java:13)
- [ ] `BlockSoulGrass.animateTick(4)`: its class isn't translated (it extends `BaseBlockNetherGrass`) (org/betterx/betternether/blocks/BlockSoulGrass.java:17)

### `BlockSoulLily`: it extends Minecraft's `Block`

- [ ] `BlockSoulLily.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockSoulLily.java:33)
- [ ] `BlockSoulLily.SHAPE_SMALL`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSoulLily.java:34)
- [ ] `BlockSoulLily.SHAPE_MEDIUM_BOTTOM`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSoulLily.java:35)
- [ ] `BlockSoulLily.SHAPE_MEDIUM_TOP`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSoulLily.java:36)
- [ ] `BlockSoulLily.SHAPE_BIG_BOTTOM`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSoulLily.java:37)
- [ ] `BlockSoulLily.SHAPE_BIG_MIDDLE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSoulLily.java:38)
- [ ] `BlockSoulLily.SHAPE_BIG_TOP_CENTER`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSoulLily.java:39)
- [ ] `BlockSoulLily.SHAPE_BIG_TOP_SIDE_N`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSoulLily.java:40)
- [ ] `BlockSoulLily.SHAPE_BIG_TOP_SIDE_S`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSoulLily.java:41)
- [ ] `BlockSoulLily.SHAPE_BIG_TOP_SIDE_E`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSoulLily.java:42)
- [ ] `BlockSoulLily.SHAPE_BIG_TOP_SIDE_W`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSoulLily.java:43)
- [ ] `BlockSoulLily.STRUCTURE`: it uses SoulLilyFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/blocks/BlockSoulLily.java:44)
- [ ] `BlockSoulLily.ROT`: it uses SoulLilyShape, which isn't translated (its constants carry values) (org/betterx/betternether/blocks/BlockSoulLily.java:45)
- [ ] `BlockSoulLily.BlockSoulLily(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulLily.java:52)
- [ ] `BlockSoulLily.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulLily.java:57)
- [ ] `BlockSoulLily.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulLily.java:61)
- [ ] `BlockSoulLily.canGrow(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulLily.java:87)
- [ ] `BlockSoulLily.randomTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulLily.java:96)
- [ ] `BlockSoulLily.isAirSides(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulLily.java:108)
- [ ] `BlockSoulLily.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulLily.java:112)
- [ ] `BlockSoulLily.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulLily.java:123)
- [ ] `BlockSoulLily.getRotationIndex(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulLily.java:147)
- [ ] `BlockSoulLily.rotOffset(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulLily.java:157)
- [ ] `BlockSoulLily.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulLily.java:167)
- [ ] `BlockSoulLily.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulLily.java:172)
- [ ] `BlockSoulLily.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulLily.java:199)

### `BlockSoulLily.SoulLilyShape`: its constants carry values

- [ ] `BlockSoulLily.SoulLilyShape.SoulLilyShape(1)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BlockSoulLily.java:226)
- [ ] `BlockSoulLily.SoulLilyShape.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BlockSoulLily.java:230)
- [ ] `BlockSoulLily.SoulLilyShape.toString(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BlockSoulLily.java:234)

### `BlockSoulLilySapling`: it extends `BaseBlockCommonSapling`

- [ ] `BlockSoulLilySapling.BlockSoulLilySapling(1)`: its class isn't translated (it extends `BaseBlockCommonSapling`) (org/betterx/betternether/blocks/BlockSoulLilySapling.java:11)
- [ ] `BlockSoulLilySapling.canSurvive(3)`: its class isn't translated (it extends `BaseBlockCommonSapling`) (org/betterx/betternether/blocks/BlockSoulLilySapling.java:15)

### `BlockSoulSandstone`: it extends Minecraft's `Block`

- [ ] `BlockSoulSandstone.UP`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BlockSoulSandstone.java:17)
- [ ] `BlockSoulSandstone.BlockSoulSandstone(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulSandstone.java:19)
- [ ] `BlockSoulSandstone.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulSandstone.java:23)
- [ ] `BlockSoulSandstone.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulSandstone.java:27)
- [ ] `BlockSoulSandstone.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulSandstone.java:40)

### `BlockSoulVein`: it extends Minecraft's `Block`

- [ ] `BlockSoulVein.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockSoulVein.java:25)
- [ ] `BlockSoulVein.BlockSoulVein(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulVein.java:27)
- [ ] `BlockSoulVein.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulVein.java:31)
- [ ] `BlockSoulVein.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulVein.java:35)
- [ ] `BlockSoulVein.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulVein.java:39)
- [ ] `BlockSoulVein.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulVein.java:52)
- [ ] `BlockSoulVein.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulVein.java:56)
- [ ] `BlockSoulVein.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulVein.java:60)
- [ ] `BlockSoulVein.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockSoulVein.java:64)

### `BlockStalactite`: it extends Minecraft's `Block`

- [ ] `BlockStalactite.SIZE`: it uses Minecraft's `IntegerProperty` (org/betterx/betternether/blocks/BlockStalactite.java:24)
- [ ] `BlockStalactite.SHAPES`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockStalactite.java:25)
- [ ] `BlockStalactite.BlockStalactite(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalactite.java:27)
- [ ] `BlockStalactite.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalactite.java:32)
- [ ] `BlockStalactite.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalactite.java:36)
- [ ] `BlockStalactite.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalactite.java:40)
- [ ] `BlockStalactite.destroy(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalactite.java:83)
- [ ] `BlockStalactite.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalactite.java:99)
- [ ] `BlockStalactite.canPlace(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalactite.java:103)

### `BlockStalagnate`: it extends Minecraft's `Block`

- [ ] `BlockStalagnate.SELECT_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockStalagnate.java:22)
- [ ] `BlockStalagnate.COLLISION_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockStalagnate.java:23)
- [ ] `BlockStalagnate.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockStalagnate.java:24)
- [ ] `BlockStalagnate.BlockStalagnate(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalagnate.java:26)
- [ ] `BlockStalagnate.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalagnate.java:31)
- [ ] `BlockStalagnate.getCollisionShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalagnate.java:35)
- [ ] `BlockStalagnate.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalagnate.java:39)
- [ ] `BlockStalagnate.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalagnate.java:43)

### `BlockStalagnateBowl`: it extends Minecraft's `Block`

- [ ] `BlockStalagnateBowl.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockStalagnateBowl.java:20)
- [ ] `BlockStalagnateBowl.FOOD`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockStalagnateBowl.java:21)
- [ ] `BlockStalagnateBowl.BlockStalagnateBowl(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalagnateBowl.java:23)
- [ ] `BlockStalagnateBowl.BlockStalagnateBowl(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalagnateBowl.java:28)
- [ ] `BlockStalagnateBowl.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalagnateBowl.java:33)
- [ ] `BlockStalagnateBowl.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalagnateBowl.java:37)
- [ ] `BlockStalagnateBowl.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalagnateBowl.java:41)
- [ ] `BlockStalagnateBowl.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStalagnateBowl.java:46)

### `BlockStalagnateSeed`: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)

- [x] `BlockStalagnateSeed.MAX_SEARCH_LENGTH` (org/betterx/betternether/blocks/BlockStalagnateSeed.java:30)
- [x] `BlockStalagnateSeed.MIN_LENGTH` (org/betterx/betternether/blocks/BlockStalagnateSeed.java:31)
- [ ] `BlockStalagnateSeed.SHAPE_TOP`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockStalagnateSeed.java:32)
- [ ] `BlockStalagnateSeed.SHAPE_BOTTOM`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockStalagnateSeed.java:33)
- [ ] `BlockStalagnateSeed.TOP`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BlockStalagnateSeed.java:34)
- [ ] `BlockStalagnateSeed.BlockStalagnateSeed(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockStalagnateSeed.java:36)
- [ ] `BlockStalagnateSeed.createBlockStateDefinition(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockStalagnateSeed.java:46)
- [ ] `BlockStalagnateSeed.getStateForPlacement(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockStalagnateSeed.java:51)
- [ ] `BlockStalagnateSeed.getShape(4)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockStalagnateSeed.java:60)
- [ ] `BlockStalagnateSeed.growsDownward(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/blocks/BlockStalagnateSeed.java:64)
- [ ] `BlockStalagnateSeed.isBonemealSuccess(5)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockStalagnateSeed.java:68)
- [ ] `BlockStalagnateSeed.isRandomGrowthSuccess(4)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockStalagnateSeed.java:72)
- [ ] `BlockStalagnateSeed.hasRoomToGrow(3)`: it uses Minecraft's `Level` (org/betterx/betternether/blocks/BlockStalagnateSeed.java:76)
- [ ] `BlockStalagnateSeed.mayPlaceOn(3)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockStalagnateSeed.java:80)
- [ ] `BlockStalagnateSeed.canSurvive(3)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockStalagnateSeed.java:84)
- [ ] `BlockStalagnateSeed.growFeature(4)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockStalagnateSeed.java:99)

### `BlockStatueRespawner`: it extends Minecraft's `Block`

- [ ] `BlockStatueRespawner.SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockStatueRespawner.java:42)
- [ ] `BlockStatueRespawner.CULL_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockStatueRespawner.java:43)
- [ ] `BlockStatueRespawner.EFFECT`: it uses Minecraft's `DustParticleOptions` (org/betterx/betternether/blocks/BlockStatueRespawner.java:47)
- [ ] `BlockStatueRespawner.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockStatueRespawner.java:48)
- [ ] `BlockStatueRespawner.TOP`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BlockStatueRespawner.java:49)
- [ ] `BlockStatueRespawner.BlockStatueRespawner(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStatueRespawner.java:53)
- [ ] `BlockStatueRespawner.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStatueRespawner.java:65)
- [ ] `BlockStatueRespawner.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStatueRespawner.java:69)
- [ ] `BlockStatueRespawner.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStatueRespawner.java:73)
- [ ] `BlockStatueRespawner.getOcclusionShape(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStatueRespawner.java:77)
- [ ] `BlockStatueRespawner.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStatueRespawner.java:81)
- [ ] `BlockStatueRespawner.getHelpMessage(0)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStatueRespawner.java:117)
- [ ] `BlockStatueRespawner.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStatueRespawner.java:123)
- [ ] `BlockStatueRespawner.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStatueRespawner.java:132)
- [ ] `BlockStatueRespawner.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStatueRespawner.java:138)
- [ ] `BlockStatueRespawner.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStatueRespawner.java:155)
- [ ] `BlockStatueRespawner.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStatueRespawner.java:159)
- [ ] `BlockStatueRespawner.playerWillDestroy(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStatueRespawner.java:163)

### `BlockStem`: it extends Minecraft's `Block`

- [ ] `BlockStem.AXIS`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockStem.java:21)
- [ ] `BlockStem.OUTLINES`: it uses `EnumMap` (java.util) (org/betterx/betternether/blocks/BlockStem.java:22)
- [ ] `BlockStem.BlockStem(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStem.java:24)
- [ ] `BlockStem.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStem.java:29)
- [ ] `BlockStem.getCollisionShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStem.java:33)
- [ ] `BlockStem.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStem.java:37)
- [ ] `BlockStem.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStem.java:54)
- [ ] `BlockStem.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockStem.java:58)

### `BlockTerrain`: it extends Minecraft's `Block`

- [ ] `BlockTerrain.TERRAIN_SOUND`: it uses Minecraft's `SoundType` (org/betterx/betternether/blocks/BlockTerrain.java:19)
- [ ] `BlockTerrain.BlockTerrain(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockTerrain.java:29)
- [ ] `BlockTerrain.setVegetationFeature(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockTerrain.java:33)
- [ ] `BlockTerrain.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockTerrain.java:37)
- [ ] `BlockTerrain.getHostBlock(0)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockTerrain.java:41)
- [ ] `BlockTerrain.getCoverFeature(0)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockTerrain.java:45)

### `BlockVeinedSand`: it extends Minecraft's `Block`

- [ ] `BlockVeinedSand.BlockVeinedSand(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockVeinedSand.java:20)
- [ ] `BlockVeinedSand.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockVeinedSand.java:24)
- [ ] `BlockVeinedSand.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockVeinedSand.java:37)

### `BlockWartRoots`: it extends Minecraft's `Block`

- [ ] `BlockWartRoots.BlockWartRoots(0)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWartRoots.java:8)
- [ ] `BlockWartRoots.BlockWartRoots(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWartRoots.java:12)

### `BlockWartSeed`: it extends Minecraft's `Block`

- [ ] `BlockWartSeed.BOUNDING_SHAPES`: it uses `EnumMap` (java.util) (org/betterx/betternether/blocks/BlockWartSeed.java:35)
- [ ] `BlockWartSeed.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockWartSeed.java:36)
- [ ] `BlockWartSeed.BlockWartSeed(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWartSeed.java:38)
- [ ] `BlockWartSeed.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWartSeed.java:43)
- [ ] `BlockWartSeed.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWartSeed.java:47)
- [ ] `BlockWartSeed.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWartSeed.java:51)
- [ ] `BlockWartSeed.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWartSeed.java:69)
- [ ] `BlockWartSeed.isValidBonemealTarget(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWartSeed.java:75)
- [ ] `BlockWartSeed.isBonemealSuccess(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWartSeed.java:80)
- [ ] `BlockWartSeed.performBonemeal(5)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWartSeed.java:84)
- [ ] `BlockWartSeed.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWartSeed.java:88)
- [ ] `BlockWartSeed.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWartSeed.java:92)
- [ ] `BlockWartSeed.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWartSeed.java:96)

### `BlockWhisperingGourdLantern`: it extends Minecraft's `Block`

- [ ] `BlockWhisperingGourdLantern.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockWhisperingGourdLantern.java:17)
- [ ] `BlockWhisperingGourdLantern.BlockWhisperingGourdLantern(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWhisperingGourdLantern.java:19)
- [ ] `BlockWhisperingGourdLantern.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWhisperingGourdLantern.java:23)
- [ ] `BlockWhisperingGourdLantern.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWhisperingGourdLantern.java:27)
- [ ] `BlockWhisperingGourdLantern.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWhisperingGourdLantern.java:31)
- [ ] `BlockWhisperingGourdLantern.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWhisperingGourdLantern.java:35)

### `BlockWhisperingGourdVine`: it extends `Growing` (org.betterx.bclib.blocks.BaseVineBlock)

- [ ] `BlockWhisperingGourdVine.BlockWhisperingGourdVine(1)`: its class isn't translated (it extends `Growing` (org.betterx.bclib.blocks.BaseVineBlock)) (org/betterx/betternether/blocks/BlockWhisperingGourdVine.java:28)
- [ ] `BlockWhisperingGourdVine.updateShape(8)`: its class isn't translated (it extends `Growing` (org.betterx.bclib.blocks.BaseVineBlock)) (org/betterx/betternether/blocks/BlockWhisperingGourdVine.java:32)
- [ ] `BlockWhisperingGourdVine.useItemOn(7)`: its class isn't translated (it extends `Growing` (org.betterx.bclib.blocks.BaseVineBlock)) (org/betterx/betternether/blocks/BlockWhisperingGourdVine.java:51)

### `BlockWillowBranch`: it extends Minecraft's `Block`

- [ ] `BlockWillowBranch.V_SHAPE`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockWillowBranch.java:26)
- [ ] `BlockWillowBranch.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockWillowBranch.java:27)
- [ ] `BlockWillowBranch.BlockWillowBranch(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowBranch.java:29)
- [ ] `BlockWillowBranch.getLuminance(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/blocks/BlockWillowBranch.java:34)
- [ ] `BlockWillowBranch.getShadeBrightness(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowBranch.java:38)
- [ ] `BlockWillowBranch.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowBranch.java:43)
- [ ] `BlockWillowBranch.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowBranch.java:47)
- [ ] `BlockWillowBranch.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowBranch.java:51)
- [ ] `BlockWillowBranch.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowBranch.java:66)

### `BlockWillowLeaves`: it extends `BNLeaves`

- [ ] `BlockWillowLeaves.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockWillowLeaves.java:28)
- [ ] `BlockWillowLeaves.NATURAL`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BlockWillowLeaves.java:29)
- [ ] `BlockWillowLeaves.BlockWillowLeaves(1)`: its class isn't translated (it extends `BNLeaves`) (org/betterx/betternether/blocks/BlockWillowLeaves.java:31)
- [ ] `BlockWillowLeaves.createBlockStateDefinition(1)`: its class isn't translated (it extends `BNLeaves`) (org/betterx/betternether/blocks/BlockWillowLeaves.java:42)
- [ ] `BlockWillowLeaves.getStateForPlacement(1)`: its class isn't translated (it extends `BNLeaves`) (org/betterx/betternether/blocks/BlockWillowLeaves.java:47)
- [ ] `BlockWillowLeaves.rotate(2)`: its class isn't translated (it extends `BNLeaves`) (org/betterx/betternether/blocks/BlockWillowLeaves.java:51)
- [ ] `BlockWillowLeaves.mirror(2)`: its class isn't translated (it extends `BNLeaves`) (org/betterx/betternether/blocks/BlockWillowLeaves.java:55)
- [ ] `BlockWillowLeaves.updateShape(8)`: its class isn't translated (it extends `BNLeaves`) (org/betterx/betternether/blocks/BlockWillowLeaves.java:59)
- [ ] `BlockWillowLeaves.getShadeBrightness(3)`: its class isn't translated (it extends `BNLeaves`) (org/betterx/betternether/blocks/BlockWillowLeaves.java:74)
- [ ] `BlockWillowLeaves.propagatesSkylightDown(1)`: its class isn't translated (it extends `BNLeaves`) (org/betterx/betternether/blocks/BlockWillowLeaves.java:79)
- [ ] `BlockWillowLeaves.getBlockSupportShape(3)`: its class isn't translated (it extends `BNLeaves`) (org/betterx/betternether/blocks/BlockWillowLeaves.java:83)

### `BlockWillowSapling`: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)

- [ ] `BlockWillowSapling.BlockWillowSapling(1)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockWillowSapling.java:20)
- [ ] `BlockWillowSapling.megaFeatureEnabled(3)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockWillowSapling.java:29)
- [ ] `BlockWillowSapling.isRandomGrowthSuccess(4)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockWillowSapling.java:33)
- [ ] `BlockWillowSapling.mayPlaceOn(3)`: its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)) (org/betterx/betternether/blocks/BlockWillowSapling.java:37)

### `BlockWillowTorch`: it extends Minecraft's `Block`

- [ ] `BlockWillowTorch.SHAPE_NORTH`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockWillowTorch.java:25)
- [ ] `BlockWillowTorch.SHAPE_SOUTH`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockWillowTorch.java:26)
- [ ] `BlockWillowTorch.SHAPE_WEST`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockWillowTorch.java:27)
- [ ] `BlockWillowTorch.SHAPE_EAST`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockWillowTorch.java:28)
- [ ] `BlockWillowTorch.SHAPE_UP`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockWillowTorch.java:29)
- [ ] `BlockWillowTorch.SHAPE_DOWN`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockWillowTorch.java:30)
- [ ] `BlockWillowTorch.FACING`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockWillowTorch.java:31)
- [ ] `BlockWillowTorch.BlockWillowTorch(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowTorch.java:33)
- [ ] `BlockWillowTorch.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowTorch.java:38)
- [ ] `BlockWillowTorch.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowTorch.java:42)
- [ ] `BlockWillowTorch.canSurvive(3)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowTorch.java:60)
- [ ] `BlockWillowTorch.rotate(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowTorch.java:65)
- [ ] `BlockWillowTorch.mirror(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowTorch.java:69)
- [ ] `BlockWillowTorch.updateShape(8)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowTorch.java:73)
- [ ] `BlockWillowTorch.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowTorch.java:86)

### `BlockWillowTrunk`: it extends Minecraft's `Block`

- [ ] `BlockWillowTrunk.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BlockWillowTrunk.java:17)
- [ ] `BlockWillowTrunk.SHAPE_BOTTOM`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockWillowTrunk.java:18)
- [ ] `BlockWillowTrunk.SHAPE_TOP`: it uses Minecraft's `VoxelShape` (org/betterx/betternether/blocks/BlockWillowTrunk.java:19)
- [ ] `BlockWillowTrunk.BlockWillowTrunk(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowTrunk.java:21)
- [ ] `BlockWillowTrunk.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowTrunk.java:26)
- [ ] `BlockWillowTrunk.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BlockWillowTrunk.java:30)

### `BlueCryingObsidianBlock`: it extends `BNObsidianBase`

- [ ] `BlueCryingObsidianBlock.BlueCryingObsidianBlock(1)`: its class isn't translated (it extends `BNObsidianBase`) (org/betterx/betternether/blocks/BlueCryingObsidianBlock.java:13)
- [ ] `BlueCryingObsidianBlock.animateTick(4)`: its class isn't translated (it extends `BNObsidianBase`) (org/betterx/betternether/blocks/BlueCryingObsidianBlock.java:17)

### `BlueWeepingObsidianBlock`: it extends `BNObsidianBase`

- [ ] `BlueWeepingObsidianBlock.BlueWeepingObsidianBlock(1)`: its class isn't translated (it extends `BNObsidianBase`) (org/betterx/betternether/blocks/BlueWeepingObsidianBlock.java:12)
- [ ] `BlueWeepingObsidianBlock.animateTick(4)`: its class isn't translated (it extends `BNObsidianBase`) (org/betterx/betternether/blocks/BlueWeepingObsidianBlock.java:16)

### `BNBlockProperties`

- [ ] `BNBlockProperties.PILLAR_SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BNBlockProperties.java:17)
- [ ] `BNBlockProperties.BROWN_MUSHROOM_SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BNBlockProperties.java:20)
- [ ] `BNBlockProperties.WILLOW_SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BNBlockProperties.java:23)
- [ ] `BNBlockProperties.JELLY_MUSHROOM_VISUAL`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BNBlockProperties.java:24)
- [ ] `BNBlockProperties.LUCIS_SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BNBlockProperties.java:25)
- [ ] `BNBlockProperties.PLANT`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BNBlockProperties.java:26)
- [ ] `BNBlockProperties.FOOD`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/BNBlockProperties.java:27)
- [ ] `BNBlockProperties.DESTRUCTED`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BNBlockProperties.java:28)
- [ ] `BNBlockProperties.FLOOR`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BNBlockProperties.java:29)
- [ ] `BNBlockProperties.OPEN`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BNBlockProperties.java:30)
- [ ] `BNBlockProperties.FIRE`: it uses Minecraft's `BooleanProperty` (org/betterx/betternether/blocks/BNBlockProperties.java:31)

### `BNBlockProperties.BrownMushroomShape`: its constants carry values

- [ ] `BNBlockProperties.BrownMushroomShape.BrownMushroomShape(1)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:48)
- [ ] `BNBlockProperties.BrownMushroomShape.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:52)
- [ ] `BNBlockProperties.BrownMushroomShape.toString(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:56)

### `BNBlockProperties.CincinnasitPillarShape`: its constants carry values

- [ ] `BNBlockProperties.CincinnasitPillarShape.CincinnasitPillarShape(1)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:70)
- [ ] `BNBlockProperties.CincinnasitPillarShape.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:74)
- [ ] `BNBlockProperties.CincinnasitPillarShape.toString(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:78)

### `BNBlockProperties.EnumLucisShape`: its constants carry values

- [ ] `BNBlockProperties.EnumLucisShape.EnumLucisShape(1)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:91)
- [ ] `BNBlockProperties.EnumLucisShape.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:95)
- [ ] `BNBlockProperties.EnumLucisShape.toString(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:99)

### `BNBlockProperties.FoodShape`: its constants carry values

- [ ] `BNBlockProperties.FoodShape.FoodShape(1)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:114)
- [ ] `BNBlockProperties.FoodShape.setItem(1)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:118)
- [ ] `BNBlockProperties.FoodShape.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:122)
- [ ] `BNBlockProperties.FoodShape.toString(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:126)
- [ ] `BNBlockProperties.FoodShape.getItem(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:131)

### `BNBlockProperties.JellyShape`: its constants carry values

- [ ] `BNBlockProperties.JellyShape.JellyShape(1)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:143)
- [ ] `BNBlockProperties.JellyShape.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:147)
- [ ] `BNBlockProperties.JellyShape.toString(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:151)

### `BNBlockProperties.PottedPlantShape`: its constants carry values

- [ ] `BNBlockProperties.PottedPlantShape.PottedPlantShape(2)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:186)
- [ ] `BNBlockProperties.PottedPlantShape.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:191)
- [ ] `BNBlockProperties.PottedPlantShape.toString(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:195)
- [ ] `BNBlockProperties.PottedPlantShape.getItem(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:200)
- [ ] `BNBlockProperties.PottedPlantShape.getBlock(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:204)

### `BNBlockProperties.WillowBranchShape`: its constants carry values

- [ ] `BNBlockProperties.WillowBranchShape.WillowBranchShape(1)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:215)
- [ ] `BNBlockProperties.WillowBranchShape.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:219)
- [ ] `BNBlockProperties.WillowBranchShape.toString(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/BNBlockProperties.java:223)

### `BNBrewingStand`: it extends Minecraft's `BrewingStandBlock`

- [ ] `BNBrewingStand.BNBrewingStand(1)`: its class isn't translated (it extends Minecraft's `BrewingStandBlock`) (org/betterx/betternether/blocks/BNBrewingStand.java:23)
- [ ] `BNBrewingStand.getTicker(3)`: it's generic (org/betterx/betternether/blocks/BNBrewingStand.java:27)
- [ ] `BNBrewingStand.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BrewingStandBlock`) (org/betterx/betternether/blocks/BNBrewingStand.java:31)
- [ ] `BNBrewingStand.useItemOn(7)`: its class isn't translated (it extends Minecraft's `BrewingStandBlock`) (org/betterx/betternether/blocks/BNBrewingStand.java:35)
- [ ] `BNBrewingStand.affectNeighborsAfterRemoval(4)`: its class isn't translated (it extends Minecraft's `BrewingStandBlock`) (org/betterx/betternether/blocks/BNBrewingStand.java:51)

### `BNLeaves`: it extends Minecraft's `TintedParticleLeavesBlock`

- [ ] `BNLeaves.BNLeaves(1)`: its class isn't translated (it extends Minecraft's `TintedParticleLeavesBlock`) (org/betterx/betternether/blocks/BNLeaves.java:11)
- [ ] `BNLeaves.isRandomlyTicking(1)`: its class isn't translated (it extends Minecraft's `TintedParticleLeavesBlock`) (org/betterx/betternether/blocks/BNLeaves.java:15)
- [ ] `BNLeaves.randomTick(4)`: its class isn't translated (it extends Minecraft's `TintedParticleLeavesBlock`) (org/betterx/betternether/blocks/BNLeaves.java:19)
- [ ] `BNLeaves.tick(4)`: its class isn't translated (it extends Minecraft's `TintedParticleLeavesBlock`) (org/betterx/betternether/blocks/BNLeaves.java:22)

### `BNObsidian`: it extends `BNObsidianBase`

- [ ] `BNObsidian.BNObsidian(0)`: its class isn't translated (it extends `BNObsidianBase`) (org/betterx/betternether/blocks/BNObsidian.java:14)
- [ ] `BNObsidian.BNObsidian(1)`: its class isn't translated (it extends `BNObsidianBase`) (org/betterx/betternether/blocks/BNObsidian.java:18)
- [ ] `BNObsidian.BNObsidian(2)`: its class isn't translated (it extends `BNObsidianBase`) (org/betterx/betternether/blocks/BNObsidian.java:22)
- [ ] `BNObsidian.onLightningUpdate(3)`: it uses Minecraft's `Level` (org/betterx/betternether/blocks/BNObsidian.java:26)

### `BNObsidianBase`: it extends Minecraft's `Block`

- [ ] `BNObsidianBase.BNObsidianBase(0)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BNObsidianBase.java:18)
- [ ] `BNObsidianBase.BNObsidianBase(1)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BNObsidianBase.java:22)
- [ ] `BNObsidianBase.BNObsidianBase(2)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BNObsidianBase.java:26)
- [ ] `BNObsidianBase.neighborChanged(6)`: its class isn't translated (it extends Minecraft's `Block`) (org/betterx/betternether/blocks/BNObsidianBase.java:31)

### `BNPane`: it extends Minecraft's `IronBarsBlock`

- [ ] `BNPane.BNPane(1)`: its class isn't translated (it extends Minecraft's `IronBarsBlock`) (org/betterx/betternether/blocks/BNPane.java:12)
- [ ] `BNPane.skipRendering(3)`: its class isn't translated (it extends Minecraft's `IronBarsBlock`) (org/betterx/betternether/blocks/BNPane.java:16)

### `BNPillar`: it extends Minecraft's `RotatedPillarBlock`

- [ ] `BNPillar.BNPillar(1)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (org/betterx/betternether/blocks/BNPillar.java:8)
- [ ] `BNPillar.BNPillar(1)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (org/betterx/betternether/blocks/BNPillar.java:12)

### `BNWoodlikeDoor`: it extends Minecraft's `DoorBlock`

- [ ] `BNWoodlikeDoor.BNWoodlikeDoor(2)`: its class isn't translated (it extends Minecraft's `DoorBlock`) (org/betterx/betternether/blocks/BNWoodlikeDoor.java:8)

### `AnchorTreeMaterial`: it extends `NetherWoodenMaterial`

- [x] `AnchorTreeMaterial.LOG_VARIANTS` (org/betterx/betternether/blocks/complex/AnchorTreeMaterial.java:18)
- [ ] `AnchorTreeMaterial.AnchorTreeMaterial(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/AnchorTreeMaterial.java:20)
- [ ] `AnchorTreeMaterial.logModel(1)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/AnchorTreeMaterial.java:25)
- [ ] `AnchorTreeMaterial.barkModel(2)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/AnchorTreeMaterial.java:30)
- [ ] `AnchorTreeMaterial.trapdoorModel(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/AnchorTreeMaterial.java:35)
- [ ] `AnchorTreeMaterial.defineExtras(1)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/AnchorTreeMaterial.java:40)
- [ ] `AnchorTreeMaterial.getSapling(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/AnchorTreeMaterial.java:49)

### `AnchorTreeSaplingModel`

- [ ] `AnchorTreeSaplingModel.MODEL`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/complex/AnchorTreeSaplingModel.java:21)
- [x] `AnchorTreeSaplingModel.AnchorTreeSaplingModel(0)` (org/betterx/betternether/blocks/complex/AnchorTreeSaplingModel.java:24)
- [x] `AnchorTreeSaplingModel.ensureLoaded(0)` (org/betterx/betternether/blocks/complex/AnchorTreeSaplingModel.java:27)
- [ ] `AnchorTreeSaplingModel.model(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/complex/AnchorTreeSaplingModel.java:30)

### `AnchorTreeSaplingModel.ClientModel`

- [ ] `AnchorTreeSaplingModel.ClientModel.build(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/complex/AnchorTreeSaplingModel.java:36)

### `ColoredGlassMaterial`: it uses `BlockKey` (de.ambertation.wover.block.api.classify)

- [ ] `ColoredGlassMaterial.ColoredGlassMaterial(3)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/blocks/complex/ColoredGlassMaterial.java:53)
- [ ] `ColoredGlassMaterial.ColoredGlassMaterial(4)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/blocks/complex/ColoredGlassMaterial.java:57)
- [ ] `ColoredGlassMaterial.ColoredGlassMaterial(7)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/blocks/complex/ColoredGlassMaterial.java:61)
- [ ] `ColoredGlassMaterial.makeKey(1)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/blocks/complex/ColoredGlassMaterial.java:89)
- [ ] `ColoredGlassMaterial.register(1)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/blocks/complex/ColoredGlassMaterial.java:98)
- [ ] `ColoredGlassMaterial.define(3)`: its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/blocks/complex/ColoredGlassMaterial.java:102)

### `GloomwoodDarkMaterial`: it extends `NetherWoodenMaterial`

- [ ] `GloomwoodDarkMaterial.GloomwoodDarkMaterial(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/GloomwoodDarkMaterial.java:10)
- [ ] `GloomwoodDarkMaterial.logModel(1)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/GloomwoodDarkMaterial.java:15)
- [ ] `GloomwoodDarkMaterial.barkModel(2)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/GloomwoodDarkMaterial.java:21)
- [ ] `GloomwoodDarkMaterial.trapdoorModel(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/GloomwoodDarkMaterial.java:27)
- [ ] `GloomwoodDarkMaterial.hasBoat(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/GloomwoodDarkMaterial.java:32)

### `GloomwoodMaterial`: it extends `NetherWoodenMaterial`

- [ ] `GloomwoodMaterial.GloomwoodMaterial(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/GloomwoodMaterial.java:17)
- [ ] `GloomwoodMaterial.logModel(1)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/GloomwoodMaterial.java:22)
- [ ] `GloomwoodMaterial.barkModel(2)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/GloomwoodMaterial.java:28)
- [ ] `GloomwoodMaterial.trapdoorModel(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/GloomwoodMaterial.java:34)
- [ ] `GloomwoodMaterial.defineExtras(1)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/GloomwoodMaterial.java:39)
- [ ] `GloomwoodMaterial.getSapling(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/GloomwoodMaterial.java:47)

### `MushroomFirMaterial`: it extends `NetherWoodenMaterial`

- [ ] `MushroomFirMaterial.MushroomFirMaterial(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/MushroomFirMaterial.java:21)
- [ ] `MushroomFirMaterial.trapdoorModel(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/MushroomFirMaterial.java:26)
- [ ] `MushroomFirMaterial.defineExtras(1)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/MushroomFirMaterial.java:31)
- [ ] `MushroomFirMaterial.getStem(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/MushroomFirMaterial.java:45)
- [ ] `MushroomFirMaterial.getSapling(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/MushroomFirMaterial.java:49)
- [ ] `MushroomFirMaterial.getTrunk(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/MushroomFirMaterial.java:53)

### `NetherMushroomMaterial`: it extends `NetherWoodenMaterial`

- [ ] `NetherMushroomMaterial.NetherMushroomMaterial(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/NetherMushroomMaterial.java:19)
- [ ] `NetherMushroomMaterial.hasLogs(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/NetherMushroomMaterial.java:24)
- [ ] `NetherMushroomMaterial.withoutKinds(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/NetherMushroomMaterial.java:29)
- [ ] `NetherMushroomMaterial.strippedLogStandIn(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/NetherMushroomMaterial.java:34)
- [ ] `NetherMushroomMaterial.defineBeforeSet(1)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/NetherMushroomMaterial.java:39)
- [ ] `NetherMushroomMaterial.defineBlocks(1)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/NetherMushroomMaterial.java:44)
- [ ] `NetherMushroomMaterial.planksRecipe(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/NetherMushroomMaterial.java:68)
- [ ] `NetherMushroomMaterial.getStem(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/NetherMushroomMaterial.java:72)

### `NetherReedMaterial`: it extends `RoofMaterial`

- [ ] `NetherReedMaterial.NetherReedMaterial(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/NetherReedMaterial.java:18)
- [ ] `NetherReedMaterial.hasLogs(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/NetherReedMaterial.java:23)
- [ ] `NetherReedMaterial.withoutKinds(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/NetherReedMaterial.java:28)
- [ ] `NetherReedMaterial.strippedLogStandIn(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/NetherReedMaterial.java:33)
- [ ] `NetherReedMaterial.isRaft(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/NetherReedMaterial.java:38)
- [ ] `NetherReedMaterial.ladderModel(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/NetherReedMaterial.java:43)
- [ ] `NetherReedMaterial.trapdoorModel(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/NetherReedMaterial.java:48)
- [ ] `NetherReedMaterial.defineBlocks(1)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/NetherReedMaterial.java:53)
- [ ] `NetherReedMaterial.planksRecipe(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/NetherReedMaterial.java:67)
- [ ] `NetherReedMaterial.getStem(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/NetherReedMaterial.java:71)

### `NetherSakuraMaterial`: it extends `NetherWoodenMaterial`

- [ ] `NetherSakuraMaterial.NetherSakuraMaterial(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/NetherSakuraMaterial.java:15)
- [ ] `NetherSakuraMaterial.defineExtras(1)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/NetherSakuraMaterial.java:20)
- [ ] `NetherSakuraMaterial.getSapling(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/NetherSakuraMaterial.java:28)

### `NetherWoodenMaterial`: it's generic

- [ ] `NetherWoodenMaterial.NetherWoodenMaterial(3)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:61)
- [ ] `NetherWoodenMaterial.self(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:68)
- [ ] `NetherWoodenMaterial.setFurnitureCloth(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:72)
- [ ] `NetherWoodenMaterial.setFurnitureCloth(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:77)
- [ ] `NetherWoodenMaterial.init(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:82)
- [ ] `NetherWoodenMaterial.register(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:91)
- [ ] `NetherWoodenMaterial.getMaterial(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:97)
- [ ] `NetherWoodenMaterial.createMaterial(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:101)
- [ ] `NetherWoodenMaterial.defineBlocks(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:114)
- [ ] `NetherWoodenMaterial.declareExtras(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:137)
- [ ] `NetherWoodenMaterial.defineBeforeSet(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:140)
- [ ] `NetherWoodenMaterial.defineExtras(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:143)
- [ ] `NetherWoodenMaterial.withoutKinds(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:146)
- [ ] `NetherWoodenMaterial.strippedLogStandIn(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:150)
- [ ] `NetherWoodenMaterial.hasLogs(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:154)
- [ ] `NetherWoodenMaterial.trapdoorModel(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:158)
- [ ] `NetherWoodenMaterial.ladderModel(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:162)
- [ ] `NetherWoodenMaterial.logModel(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:166)
- [ ] `NetherWoodenMaterial.barkModel(2)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:171)
- [ ] `NetherWoodenMaterial.logRecipe(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:176)
- [ ] `NetherWoodenMaterial.logFactory(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:181)
- [ ] `NetherWoodenMaterial.barkFactory(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:186)
- [ ] `NetherWoodenMaterial.logAndBark(2)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:191)
- [ ] `NetherWoodenMaterial.logOrBark(3)`: it uses `BlockDefinition` (de.ambertation.wover.block.api) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:223)
- [ ] `NetherWoodenMaterial.addFurniture(4)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:234)
- [ ] `NetherWoodenMaterial.furniture(4)`: it uses `EntryConfig` (de.ambertation.wover.sets.api.classify.BlockSet) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:244)
- [ ] `NetherWoodenMaterial.key(2)`: it's generic (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:249)
- [ ] `NetherWoodenMaterial.sapling(4)`: it's generic (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:253)
- [ ] `NetherWoodenMaterial.stem(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:261)
- [ ] `NetherWoodenMaterial.trunk(4)`: it's generic (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:270)
- [ ] `NetherWoodenMaterial.woodBlock(4)`: it's generic (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:277)
- [ ] `NetherWoodenMaterial.woodBlock(5)`: it's generic (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:281)
- [ ] `NetherWoodenMaterial.dropLog(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:291)
- [ ] `NetherWoodenMaterial.logsBlockTag(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:295)
- [ ] `NetherWoodenMaterial.planks(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:299)
- [ ] `NetherWoodenMaterial.withoutItemKinds(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:303)
- [ ] `NetherWoodenMaterial.defineBoats(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:307)
- [ ] `NetherWoodenMaterial.hasBoat(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:315)
- [ ] `NetherWoodenMaterial.isRaft(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:319)
- [ ] `NetherWoodenMaterial.blocks(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:323)
- [ ] `NetherWoodenMaterial.woodType(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:327)
- [ ] `NetherWoodenMaterial.getBlock(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:331)
- [ ] `NetherWoodenMaterial.getBlock(2)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:335)
- [ ] `NetherWoodenMaterial.isTreeLog(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:339)
- [ ] `NetherWoodenMaterial.isTreeLog(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:343)
- [ ] `NetherWoodenMaterial.getPlanks(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:347)
- [ ] `NetherWoodenMaterial.getSlab(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:351)
- [ ] `NetherWoodenMaterial.getLog(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:355)
- [ ] `NetherWoodenMaterial.getBark(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:359)
- [ ] `NetherWoodenMaterial.getStrippedLog(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:363)
- [ ] `NetherWoodenMaterial.getStrippedBark(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java:367)

### `NetherWoodModels`

- [ ] `NetherWoodModels.LADDER_TEMPLATE`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/blocks/complex/NetherWoodModels.java:10)
- [ ] `NetherWoodModels.TRAPDOOR_TEMPLATE`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/blocks/complex/NetherWoodModels.java:11)
- [ ] `NetherWoodModels.TRAPDOOR_SIDE_TEMPLATE`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/blocks/complex/NetherWoodModels.java:12)
- [ ] `NetherWoodModels.FENCE_POST_TEMPLATE`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/blocks/complex/NetherWoodModels.java:13)
- [ ] `NetherWoodModels.FENCE_SIDE_TEMPLATE`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/blocks/complex/NetherWoodModels.java:14)
- [x] `NetherWoodModels.NetherWoodModels(0)` (org/betterx/betternether/blocks/complex/NetherWoodModels.java:16)
- [ ] `NetherWoodModels.model(1)`: a raw Supplier (org/betterx/betternether/blocks/complex/NetherWoodModels.java:19)
- [ ] `NetherWoodModels.external(0)`: a raw Supplier (org/betterx/betternether/blocks/complex/NetherWoodModels.java:23)
- [ ] `NetherWoodModels.ladder(0)`: a raw Supplier (org/betterx/betternether/blocks/complex/NetherWoodModels.java:27)
- [ ] `NetherWoodModels.trapdoor(0)`: a raw Supplier (org/betterx/betternether/blocks/complex/NetherWoodModels.java:31)
- [ ] `NetherWoodModels.trapdoor(1)`: a raw Supplier (org/betterx/betternether/blocks/complex/NetherWoodModels.java:35)
- [ ] `NetherWoodModels.fence(3)`: a raw Supplier (org/betterx/betternether/blocks/complex/NetherWoodModels.java:39)

### `RoofMaterial`: it's generic

- [ ] `RoofMaterial.RoofMaterial(3)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/RoofMaterial.java:15)
- [ ] `RoofMaterial.defineBlocks(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/RoofMaterial.java:19)
- [ ] `RoofMaterial.noFuel(1)`: it uses `BlockDefinition` (de.ambertation.wover.block.api) (org/betterx/betternether/blocks/complex/RoofMaterial.java:34)
- [ ] `RoofMaterial.roofRecipe(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/RoofMaterial.java:38)
- [ ] `RoofMaterial.roofRecipe(3)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/RoofMaterial.java:42)
- [ ] `RoofMaterial.roofRecipe(4)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/RoofMaterial.java:46)
- [ ] `RoofMaterial.getRoof(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/RoofMaterial.java:51)
- [ ] `RoofMaterial.getRoofStairs(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/RoofMaterial.java:55)
- [ ] `RoofMaterial.getRoofSlab(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/RoofMaterial.java:59)

### `RubeusMaterial`: it extends `NetherWoodenMaterial`

- [ ] `RubeusMaterial.RubeusMaterial(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/RubeusMaterial.java:23)
- [ ] `RubeusMaterial.logModel(1)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/RubeusMaterial.java:28)
- [ ] `RubeusMaterial.barkModel(2)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/RubeusMaterial.java:47)
- [ ] `RubeusMaterial.logFactory(1)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/RubeusMaterial.java:64)
- [ ] `RubeusMaterial.barkFactory(1)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/RubeusMaterial.java:70)
- [ ] `RubeusMaterial.defineExtras(1)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/RubeusMaterial.java:76)
- [ ] `RubeusMaterial.getCone(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/RubeusMaterial.java:86)
- [ ] `RubeusMaterial.getSapling(0)`: its class isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/blocks/complex/RubeusMaterial.java:90)

### `StalagnateMaterial`: it extends `RoofMaterial`

- [ ] `StalagnateMaterial.StalagnateMaterial(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/StalagnateMaterial.java:28)
- [ ] `StalagnateMaterial.logRecipe(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/StalagnateMaterial.java:33)
- [ ] `StalagnateMaterial.defineExtras(1)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/StalagnateMaterial.java:39)
- [ ] `StalagnateMaterial.getTrunk(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/StalagnateMaterial.java:71)
- [ ] `StalagnateMaterial.getStem(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/StalagnateMaterial.java:75)
- [ ] `StalagnateMaterial.getBowl(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/StalagnateMaterial.java:79)
- [ ] `StalagnateMaterial.getSeed(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/StalagnateMaterial.java:83)

### `VanillaFallback`: it's generic

- [ ] `VanillaFallback.VanillaFallback(3)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/VanillaFallback.java:19)
- [ ] `VanillaFallback.createMaterial(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/VanillaFallback.java:23)
- [ ] `VanillaFallback.material(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/VanillaFallback.java:30)
- [ ] `VanillaFallback.addVanillaFurniture(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/VanillaFallback.java:34)
- [ ] `VanillaFallback.burning(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/VanillaFallback.java:39)
- [ ] `VanillaFallback.planks(0)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/VanillaFallback.java:42)
- [ ] `VanillaFallback.vanilla(1)`: its class isn't translated (it's generic) (org/betterx/betternether/blocks/complex/VanillaFallback.java:47)
- [ ] `VanillaFallback.getVanillaBlock(2)`: it uses Minecraft's `Block` (org/betterx/betternether/blocks/complex/VanillaFallback.java:56)

### `VanillaNetherWood`: it extends `VanillaFallback`

- [ ] `VanillaNetherWood.VanillaNetherWood(3)`: its class isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/blocks/complex/VanillaNetherWood.java:12)
- [ ] `VanillaNetherWood.material(0)`: its class isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/blocks/complex/VanillaNetherWood.java:16)
- [ ] `VanillaNetherWood.defineBlocks(1)`: its class isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/blocks/complex/VanillaNetherWood.java:21)

### `VanillaWood`: it extends `VanillaFallback`

- [ ] `VanillaWood.VanillaWood(3)`: its class isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/blocks/complex/VanillaWood.java:11)
- [ ] `VanillaWood.defineBlocks(1)`: its class isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/blocks/complex/VanillaWood.java:15)
- [ ] `VanillaWood.burning(1)`: its class isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/blocks/complex/VanillaWood.java:20)
- [ ] `VanillaWood.hasBoat(0)`: its class isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/blocks/complex/VanillaWood.java:25)
- [ ] `VanillaWood.create(2)`: it uses VanillaWood, which isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/blocks/complex/VanillaWood.java:30)

### `WartMaterial`: it extends `RoofMaterial`

- [ ] `WartMaterial.WartMaterial(3)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/WartMaterial.java:17)
- [ ] `WartMaterial.trapdoorModel(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/WartMaterial.java:22)
- [ ] `WartMaterial.defineExtras(1)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/WartMaterial.java:27)
- [ ] `WartMaterial.getRoot(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/WartMaterial.java:38)
- [ ] `WartMaterial.getSeed(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/WartMaterial.java:42)

### `WillowMaterial`: it extends `RoofMaterial`

- [x] `WillowMaterial.LOG_VARIANTS` (org/betterx/betternether/blocks/complex/WillowMaterial.java:38)
- [ ] `WillowMaterial.WillowMaterial(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/WillowMaterial.java:41)
- [ ] `WillowMaterial.trapdoorModel(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/WillowMaterial.java:46)
- [ ] `WillowMaterial.logModel(1)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/WillowMaterial.java:51)
- [ ] `WillowMaterial.barkModel(2)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/WillowMaterial.java:61)
- [ ] `WillowMaterial.declareExtras(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/WillowMaterial.java:69)
- [ ] `WillowMaterial.defineExtras(1)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/WillowMaterial.java:75)
- [ ] `WillowMaterial.willowTrunkModel(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/complex/WillowMaterial.java:94)
- [ ] `WillowMaterial.getTrunk(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/WillowMaterial.java:111)
- [ ] `WillowMaterial.getBranch(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/WillowMaterial.java:115)
- [ ] `WillowMaterial.getSapling(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/WillowMaterial.java:119)
- [ ] `WillowMaterial.getTorch(0)`: its class isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/blocks/complex/WillowMaterial.java:123)

### `NetherLoot`

- [ ] `NetherLoot.NONE`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:52)
- [ ] `NetherLoot.DROP_SOUL_SAND`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:54)
- [ ] `NetherLoot.DROP_LOG`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:56)
- [ ] `NetherLoot.NETHER_GRASS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:61)
- [ ] `NetherLoot.GIANT_LUCIS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:63)
- [ ] `NetherLoot.ONLY_TOP_DROPS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:75)
- [ ] `NetherLoot.WHISPERING_GOURD_VINE`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:94)
- [ ] `NetherLoot.LUMABUS_VINE`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:129)
- [ ] `NetherLoot.BLACK_APPLE`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:174)
- [ ] `NetherLoot.BONE_MUSHROOM`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:186)
- [ ] `NetherLoot.FEATHER_FERN`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:199)
- [ ] `NetherLoot.MAGMA_FLOWER`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:212)
- [ ] `NetherLoot.ORANGE_MUSHROOM`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:225)
- [ ] `NetherLoot.INK_BUSH`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:238)
- [ ] `NetherLoot.EYEBALL`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:246)
- [ ] `NetherLoot.EYEBALL_SMALL`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:266)
- [ ] `NetherLoot.RED_LARGE_MUSHROOM`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:280)
- [ ] `NetherLoot.BROWN_LARGE_MUSHROOM`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:304)
- [ ] `NetherLoot.GIANT_MOLD`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:338)
- [x] `NetherLoot.MUSHROOM_FIR_SAPLING_CHANCE` (org/betterx/betternether/blocks/NetherLoot.java:374)
- [ ] `NetherLoot.MUSHROOM_FIR_TRUNK`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:375)
- [ ] `NetherLoot.STALAGNATE_TRUNK`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:407)
- [ ] `NetherLoot.PIG_STATUE_RESPAWNER`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:423)
- [ ] `NetherLoot.LUCIS_MUSHROOM`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:434)
- [ ] `NetherLoot.BARREL_CACTUS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:451)
- [ ] `NetherLoot.AGAVE`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:469)
- [ ] `NetherLoot.JELLYFISH_MUSHROOM`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:499)
- [ ] `NetherLoot.WILLOW_BRANCH`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:540)
- [ ] `NetherLoot.SOUL_LILY`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:552)
- [ ] `NetherLoot.CINCINNASITE_ANVIL`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:599)
- [ ] `NetherLoot.STALAGNATE_BOWL`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:611)
- [ ] `NetherLoot.POTTED_PLANT`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherLoot.java:628)
- [ ] `NetherLoot.none(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:646)
- [ ] `NetherLoot.dropSoulSand(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:650)
- [ ] `NetherLoot.dropLog(1)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:654)
- [x] `NetherLoot.ensureLoaded(0)` (org/betterx/betternether/blocks/NetherLoot.java:658)
- [ ] `NetherLoot.netherGrass(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:661)
- [ ] `NetherLoot.giantLucis(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:665)
- [ ] `NetherLoot.onlyTopDrops(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:669)
- [ ] `NetherLoot.whisperingGourdVine(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:673)
- [ ] `NetherLoot.lumabusVine(1)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:677)
- [ ] `NetherLoot.stateIs(4)`: it's generic (org/betterx/betternether/blocks/NetherLoot.java:681)
- [ ] `NetherLoot.stateIs(4)`: it's generic (org/betterx/betternether/blocks/NetherLoot.java:689)
- [ ] `NetherLoot.stateIs(4)`: it's generic (org/betterx/betternether/blocks/NetherLoot.java:697)
- [ ] `NetherLoot.survivingPool(1)`: it uses Builder, which isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/blocks/NetherLoot.java:705)
- [ ] `NetherLoot.ripePool(6)`: it uses `LootLookupProvider` (de.ambertation.wover.loot.api) (org/betterx/betternether/blocks/NetherLoot.java:710)
- [ ] `NetherLoot.ripePlant(6)`: it uses Minecraft's `Builder` (org/betterx/betternether/blocks/NetherLoot.java:721)
- [ ] `NetherLoot.blackApple(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:733)
- [ ] `NetherLoot.boneMushroom(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:737)
- [ ] `NetherLoot.featherFern(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:741)
- [ ] `NetherLoot.magmaFlower(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:745)
- [ ] `NetherLoot.orangeMushroom(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:749)
- [ ] `NetherLoot.inkBush(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:753)
- [ ] `NetherLoot.eyeball(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:757)
- [ ] `NetherLoot.eyeballSmall(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:761)
- [ ] `NetherLoot.redLargeMushroom(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:765)
- [ ] `NetherLoot.brownLargeMushroom(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:769)
- [ ] `NetherLoot.giantMold(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:773)
- [ ] `NetherLoot.mushroomFirTrunk(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:777)
- [ ] `NetherLoot.stalagnateTrunk(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:781)
- [ ] `NetherLoot.pigStatueRespawner(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:785)
- [ ] `NetherLoot.stateIsAnyOf(5)`: it's generic (org/betterx/betternether/blocks/NetherLoot.java:789)
- [ ] `NetherLoot.toolIsIn(2)`: it uses Minecraft's `Builder` (org/betterx/betternether/blocks/NetherLoot.java:802)
- [ ] `NetherLoot.lucisMushroom(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:806)
- [ ] `NetherLoot.barrelCactus(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:810)
- [ ] `NetherLoot.agave(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:814)
- [ ] `NetherLoot.jellyfishMushroom(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:818)
- [ ] `NetherLoot.willowBranch(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:822)
- [ ] `NetherLoot.soulLily(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:826)
- [ ] `NetherLoot.cincinnasiteAnvil(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:830)
- [ ] `NetherLoot.stalagnateBowl(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:834)
- [ ] `NetherLoot.pottedPlant(0)`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/blocks/NetherLoot.java:838)

### `NetherLoot.DropLog`: it uses Minecraft's `Block`

- [ ] `NetherLoot.DropLog.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/blocks/NetherLoot.java:843)
- [ ] `NetherLoot.DropLog.DropLog(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherLoot.java:842)
- [ ] `NetherLoot.DropLog.log(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherLoot.java:842)

### `NetherLoot.LumabusVine`: it uses Minecraft's `Block`

- [ ] `NetherLoot.LumabusVine.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/blocks/NetherLoot.java:849)
- [ ] `NetherLoot.LumabusVine.LumabusVine(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherLoot.java:848)
- [ ] `NetherLoot.LumabusVine.seed(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherLoot.java:848)

### `NetherModels`

- [ ] `NetherModels.BASALT_BRICKS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:56)
- [ ] `NetherModels.SLAB`: it uses `Template4` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:58)
- [ ] `NetherModels.SLAB_COLUMN_DOUBLE`: it uses `Template2` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:66)
- [ ] `NetherModels.SOUL_SANDSTONE_STAIRS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:74)
- [ ] `NetherModels.SOUL_SANDSTONE_SMOOTH_STAIRS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:76)
- [ ] `NetherModels.SOUL_SANDSTONE_CUT_STAIRS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:78)
- [ ] `NetherModels.REED_STAIRS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:80)
- [ ] `NetherModels.PRESSURE_PLATE`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:82)
- [ ] `NetherModels.BUTTON`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:90)
- [ ] `NetherModels.CHAIR_CINCINNASITE`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:98)
- [ ] `NetherModels.BAR_STOOL_CINCINNASITE`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:100)
- [ ] `NetherModels.TABURET_CINCINNASITE`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:102)
- [ ] `NetherModels.GRASS`: it uses `Template2` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:104)
- [ ] `NetherModels.NETHER_GRASS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:112)
- [ ] `NetherModels.TERRAIN_COVER`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:114)
- [ ] `NetherModels.CUBE_WITH_BORROWED_TOP_AND_BOTTOM`: it uses `Template2` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:122)
- [ ] `NetherModels.JUNGLE_PLANT`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:130)
- [ ] `NetherModels.QUARTZ_GLASS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:132)
- [ ] `NetherModels.OBSIDIAN_VARIANTS`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:134)
- [ ] `NetherModels.CUBE_FROM_TEXTURE`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betternether/blocks/NetherModels.java:136)
- [x] `NetherModels.ensureLoaded(0)` (org/betterx/betternether/blocks/NetherModels.java:145)
- [ ] `NetherModels.basaltBricks(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:148)
- [ ] `NetherModels.slab(4)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:152)
- [ ] `NetherModels.slabColumnDouble(2)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:156)
- [ ] `NetherModels.soulSandstoneStairs(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:160)
- [ ] `NetherModels.soulSandstoneSmoothStairs(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:164)
- [ ] `NetherModels.soulSandstoneCutStairs(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:168)
- [ ] `NetherModels.reedStairs(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:172)
- [ ] `NetherModels.pressurePlate(1)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:176)
- [ ] `NetherModels.button(1)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:180)
- [ ] `NetherModels.chairCincinnasite(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:184)
- [ ] `NetherModels.barStoolCincinnasite(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:188)
- [ ] `NetherModels.taburetCincinnasite(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:192)
- [ ] `NetherModels.grass(2)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:196)
- [ ] `NetherModels.netherGrass(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:200)
- [ ] `NetherModels.terrainCover(1)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:204)
- [ ] `NetherModels.terrainCoverVariants(3)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:208)
- [ ] `NetherModels.cubeWithBorrowedTopAndBottom(2)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:227)
- [ ] `NetherModels.junglePlant(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:231)
- [ ] `NetherModels.quartzGlass(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:235)
- [ ] `NetherModels.obsidianVariants(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:239)
- [ ] `NetherModels.debugItem(1)`: it uses `ItemModel` (de.ambertation.wover.item.api.model) (org/betterx/betternether/blocks/NetherModels.java:243)
- [ ] `NetherModels.gloomwispVineModelTrait(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:247)
- [ ] `NetherModels.lumabusVineModelTrait(1)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:290)
- [ ] `NetherModels.cubeFromTexture(1)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:310)
- [ ] `NetherModels.netherSakuraLeavesModelTrait(0)`: it uses `BlockModel` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:314)

### `NetherModels.BuildParams`

- [x] `NetherModels.BuildParams.BuildParams(1)` (org/betterx/betternether/blocks/NetherModels.java:343)
- [x] `NetherModels.BuildParams.texture(0)` (org/betterx/betternether/blocks/NetherModels.java:343)

### `NetherModels.Button`

- [ ] `NetherModels.Button.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/blocks/NetherModels.java:347)
- [x] `NetherModels.Button.Button(1)` (org/betterx/betternether/blocks/NetherModels.java:346)
- [x] `NetherModels.Button.texture(0)` (org/betterx/betternether/blocks/NetherModels.java:346)

### `NetherModels.ButtonParams`

- [x] `NetherModels.ButtonParams.ButtonParams(1)` (org/betterx/betternether/blocks/NetherModels.java:352)
- [x] `NetherModels.ButtonParams.texture(0)` (org/betterx/betternether/blocks/NetherModels.java:352)

### `NetherModels.ClientCubeModel`

- [ ] `NetherModels.ClientCubeModel.buildFrom(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:357)
- [ ] `NetherModels.ClientCubeModel.build(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:361)

### `NetherModels.CubeFromTexture`

- [ ] `NetherModels.CubeFromTexture.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/blocks/NetherModels.java:370)
- [x] `NetherModels.CubeFromTexture.CubeFromTexture(1)` (org/betterx/betternether/blocks/NetherModels.java:369)
- [x] `NetherModels.CubeFromTexture.texture(0)` (org/betterx/betternether/blocks/NetherModels.java:369)

### `NetherModels.CubeWithBorrowedTopAndBottom`: it uses Minecraft's `Block`

- [ ] `NetherModels.CubeWithBorrowedTopAndBottom.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/blocks/NetherModels.java:376)
- [ ] `NetherModels.CubeWithBorrowedTopAndBottom.CubeWithBorrowedTopAndBottom(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:375)
- [ ] `NetherModels.CubeWithBorrowedTopAndBottom.top(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:375)
- [ ] `NetherModels.CubeWithBorrowedTopAndBottom.bottom(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:375)

### `NetherModels.CubeWithBorrowedTopAndBottomParams`: it uses Minecraft's `Block`

- [ ] `NetherModels.CubeWithBorrowedTopAndBottomParams.CubeWithBorrowedTopAndBottomParams(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:385)
- [ ] `NetherModels.CubeWithBorrowedTopAndBottomParams.top(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:385)
- [ ] `NetherModels.CubeWithBorrowedTopAndBottomParams.bottom(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:385)

### `NetherModels.Grass`

- [ ] `NetherModels.Grass.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/blocks/NetherModels.java:389)
- [x] `NetherModels.Grass.Grass(2)` (org/betterx/betternether/blocks/NetherModels.java:388)
- [x] `NetherModels.Grass.name(0)` (org/betterx/betternether/blocks/NetherModels.java:388)
- [x] `NetherModels.Grass.variants(0)` (org/betterx/betternether/blocks/NetherModels.java:388)

### `NetherModels.GrassParams`

- [x] `NetherModels.GrassParams.GrassParams(2)` (org/betterx/betternether/blocks/NetherModels.java:395)
- [x] `NetherModels.GrassParams.name(0)` (org/betterx/betternether/blocks/NetherModels.java:395)
- [x] `NetherModels.GrassParams.variants(0)` (org/betterx/betternether/blocks/NetherModels.java:395)

### `NetherModels.Impl`

- [ ] `NetherModels.Impl.grassFrom(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:400)
- [ ] `NetherModels.Impl.terrainCoverFrom(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:404)
- [ ] `NetherModels.Impl.cubeWithBorrowedTopAndBottomFrom(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:408)
- [ ] `NetherModels.Impl.slabFrom(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:412)
- [ ] `NetherModels.Impl.slabColumnDoubleFrom(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:416)
- [ ] `NetherModels.Impl.pressurePlateFrom(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:420)
- [ ] `NetherModels.Impl.buttonFrom(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:424)
- [ ] `NetherModels.Impl.debugItem(1)`: it uses `ItemModel` (de.ambertation.wover.item.api.model) (org/betterx/betternether/blocks/NetherModels.java:428)
- [ ] `NetherModels.Impl.grass(2)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:432)
- [ ] `NetherModels.Impl.terrainCover(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:436)
- [ ] `NetherModels.Impl.cubeWithBorrowedTopAndBottom(2)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:440)
- [ ] `NetherModels.Impl.netherGrass(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:454)
- [ ] `NetherModels.Impl.junglePlant(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:486)
- [ ] `NetherModels.Impl.quartzGlass(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:530)
- [ ] `NetherModels.Impl.obsidianVariants(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:559)
- [ ] `NetherModels.Impl.basaltBricks(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:563)
- [ ] `NetherModels.Impl.slab(4)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:567)
- [ ] `NetherModels.Impl.slabColumnDouble(2)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:578)
- [ ] `NetherModels.Impl.soulSandstoneStairs(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:605)
- [ ] `NetherModels.Impl.soulSandstoneSmoothStairs(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:614)
- [ ] `NetherModels.Impl.soulSandstoneCutStairs(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:621)
- [ ] `NetherModels.Impl.reedStairs(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:628)
- [ ] `NetherModels.Impl.pressurePlate(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:635)
- [ ] `NetherModels.Impl.button(1)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:639)
- [ ] `NetherModels.Impl.chairCincinnasite(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:643)
- [ ] `NetherModels.Impl.barStoolCincinnasite(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:667)
- [ ] `NetherModels.Impl.taburetCincinnasite(0)`: it uses `BlockModelFactory` (de.ambertation.wover.block.api.model) (org/betterx/betternether/blocks/NetherModels.java:673)
- [ ] `NetherModels.Impl.toArray(1)`: it creates `JsonArray` (com.google.gson) (org/betterx/betternether/blocks/NetherModels.java:677)

### `NetherModels.PressurePlate`

- [ ] `NetherModels.PressurePlate.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/blocks/NetherModels.java:689)
- [x] `NetherModels.PressurePlate.PressurePlate(1)` (org/betterx/betternether/blocks/NetherModels.java:688)
- [x] `NetherModels.PressurePlate.texture(0)` (org/betterx/betternether/blocks/NetherModels.java:688)

### `NetherModels.PressurePlateParams`

- [x] `NetherModels.PressurePlateParams.PressurePlateParams(1)` (org/betterx/betternether/blocks/NetherModels.java:694)
- [x] `NetherModels.PressurePlateParams.texture(0)` (org/betterx/betternether/blocks/NetherModels.java:694)

### `NetherModels.Slab`: it uses Minecraft's `Block`

- [ ] `NetherModels.Slab.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/blocks/NetherModels.java:698)
- [ ] `NetherModels.Slab.Slab(4)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:697)
- [ ] `NetherModels.Slab.source(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:697)
- [ ] `NetherModels.Slab.top(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:697)
- [ ] `NetherModels.Slab.bottom(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:697)
- [ ] `NetherModels.Slab.side(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:697)

### `NetherModels.SlabColumnDouble`

- [ ] `NetherModels.SlabColumnDouble.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/blocks/NetherModels.java:710)
- [x] `NetherModels.SlabColumnDouble.SlabColumnDouble(2)` (org/betterx/betternether/blocks/NetherModels.java:709)
- [x] `NetherModels.SlabColumnDouble.topBottom(0)` (org/betterx/betternether/blocks/NetherModels.java:709)
- [x] `NetherModels.SlabColumnDouble.side(0)` (org/betterx/betternether/blocks/NetherModels.java:709)

### `NetherModels.SlabColumnDoubleParams`

- [x] `NetherModels.SlabColumnDoubleParams.SlabColumnDoubleParams(2)` (org/betterx/betternether/blocks/NetherModels.java:719)
- [x] `NetherModels.SlabColumnDoubleParams.topBottom(0)` (org/betterx/betternether/blocks/NetherModels.java:719)
- [x] `NetherModels.SlabColumnDoubleParams.side(0)` (org/betterx/betternether/blocks/NetherModels.java:719)

### `NetherModels.SlabParams`: it uses Minecraft's `Block`

- [ ] `NetherModels.SlabParams.SlabParams(4)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:722)
- [ ] `NetherModels.SlabParams.source(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:722)
- [ ] `NetherModels.SlabParams.top(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:722)
- [ ] `NetherModels.SlabParams.bottom(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:722)
- [ ] `NetherModels.SlabParams.side(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:722)

### `NetherModels.TerrainCover`: it uses Minecraft's `Block`

- [ ] `NetherModels.TerrainCover.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/blocks/NetherModels.java:726)
- [ ] `NetherModels.TerrainCover.TerrainCover(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:725)
- [ ] `NetherModels.TerrainCover.bottom(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:725)

### `NetherModels.TerrainCoverParams`: it uses Minecraft's `Block`

- [ ] `NetherModels.TerrainCoverParams.TerrainCoverParams(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:731)
- [ ] `NetherModels.TerrainCoverParams.bottom(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/blocks/NetherModels.java:731)

### `RedstoneOreBlock`: it extends Minecraft's `RedStoneOreBlock`

- [ ] `RedstoneOreBlock.RedstoneOreBlock(1)`: its class isn't translated (it extends Minecraft's `RedStoneOreBlock`) (org/betterx/betternether/blocks/RedstoneOreBlock.java:13)
- [ ] `RedstoneOreBlock.litBlockEmission(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/blocks/RedstoneOreBlock.java:17)

### `RubeusBark`: it extends Minecraft's `RotatedPillarBlock`

- [ ] `RubeusBark.RubeusBark(1)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (org/betterx/betternether/blocks/RubeusBark.java:13)
- [ ] `RubeusBark.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (org/betterx/betternether/blocks/RubeusBark.java:18)

### `RubeusLog`: it extends Minecraft's `RotatedPillarBlock`

- [ ] `RubeusLog.SHAPE`: it uses Minecraft's `EnumProperty` (org/betterx/betternether/blocks/RubeusLog.java:15)
- [ ] `RubeusLog.RubeusLog(1)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (org/betterx/betternether/blocks/RubeusLog.java:17)
- [ ] `RubeusLog.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `RotatedPillarBlock`) (org/betterx/betternether/blocks/RubeusLog.java:22)

### `VanillaWeepingObsidianBlock`: it extends `BNObsidianBase`

- [ ] `VanillaWeepingObsidianBlock.VanillaWeepingObsidianBlock(1)`: its class isn't translated (it extends `BNObsidianBase`) (org/betterx/betternether/blocks/VanillaWeepingObsidianBlock.java:12)
- [ ] `VanillaWeepingObsidianBlock.animateTick(4)`: its class isn't translated (it extends `BNObsidianBase`) (org/betterx/betternether/blocks/VanillaWeepingObsidianBlock.java:16)

### `WispRotation`: its constants carry values

- [ ] `WispRotation.WispRotation(1)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/WispRotation.java:15)
- [ ] `WispRotation.yRotation(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/WispRotation.java:19)
- [ ] `WispRotation.forHorizontal(1)`: it uses WispRotation, which isn't translated (its constants carry values) (org/betterx/betternether/blocks/WispRotation.java:28)
- [ ] `WispRotation.getSerializedName(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/blocks/WispRotation.java:37)

### `BN`

- [ ] `BN.id(1)`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/BN.java:6)

### `BetterNetherClient`: it implements Fabric's `ClientModInitializer`

- [ ] `BetterNetherClient.onInitializeClient(0)`: done another way in Horizon: it runs as `on_init`, from Horizon's init hook (org/betterx/betternether/client/BetterNetherClient.java:11)

### `BNModels`

- [ ] `BNModels.TEMPLATES`: it uses Minecraft's `ModelTemplate` (org/betterx/betternether/client/block/BNModels.java:31)
- [ ] `BNModels.CROP_BLOCK_MODEL_LOCATION`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/client/block/BNModels.java:32)
- [ ] `BNModels.JUNGLE_PLANT_MODEL_LOCATION`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/client/block/BNModels.java:33)
- [ ] `BNModels.GRASS_FAN_MODEL_LOCATION`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/client/block/BNModels.java:34)
- [ ] `BNModels.STAIRS_WITH_TOP_MODEL_LOCATION`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/client/block/BNModels.java:35)
- [ ] `BNModels.STAIRS_WITH_TOP_MODEL`: it uses Minecraft's `ModelTemplate` (org/betterx/betternether/client/block/BNModels.java:36)
- [ ] `BNModels.createComplex(3)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) (org/betterx/betternether/client/block/BNModels.java:40)
- [ ] `BNModels.createComplex(4)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) (org/betterx/betternether/client/block/BNModels.java:44)
- [ ] `BNModels.getCropBlockModelTemplate(1)`: it uses Minecraft's `ModelTemplate` (org/betterx/betternether/client/block/BNModels.java:70)
- [ ] `BNModels.getCropCrossBlockModelTemplate(1)`: it uses Minecraft's `ModelTemplate` (org/betterx/betternether/client/block/BNModels.java:81)
- [ ] `BNModels.provideGrassBlockModels(4)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) (org/betterx/betternether/client/block/BNModels.java:92)
- [ ] `BNModels.provideSimpleMultiStateBlock(3)`: it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model) (org/betterx/betternether/client/block/BNModels.java:119)

### `BNModels.ModelSource`: it uses VariantSupplier, which isn't translated (it's an interface)

- [ ] `BNModels.ModelSource.ModelSource(4)`: its class isn't translated (it uses VariantSupplier, which isn't translated (it's an interface)) (org/betterx/betternether/client/block/BNModels.java:139)
- [ ] `BNModels.ModelSource.of(4)`: it uses ModelSource, which isn't translated (it uses VariantSupplier, which isn't translated (it's an interface)) (org/betterx/betternether/client/block/BNModels.java:140)
- [ ] `BNModels.ModelSource.parent(0)`: its class isn't translated (it uses VariantSupplier, which isn't translated (it's an interface)) (org/betterx/betternether/client/block/BNModels.java:139)
- [ ] `BNModels.ModelSource.suffix(0)`: its class isn't translated (it uses VariantSupplier, which isn't translated (it's an interface)) (org/betterx/betternether/client/block/BNModels.java:139)
- [ ] `BNModels.ModelSource.variants(0)`: its class isn't translated (it uses VariantSupplier, which isn't translated (it's an interface)) (org/betterx/betternether/client/block/BNModels.java:139)
- [ ] `BNModels.ModelSource.textures(0)`: its class isn't translated (it uses VariantSupplier, which isn't translated (it's an interface)) (org/betterx/betternether/client/block/BNModels.java:139)

### `BNModels.TextureSource`: it uses Minecraft's `TextureSlot`

- [ ] `BNModels.TextureSource.TextureSource(2)`: its class isn't translated (it uses Minecraft's `TextureSlot`) (org/betterx/betternether/client/block/BNModels.java:145)
- [ ] `BNModels.TextureSource.of(2)`: it uses TextureSource, which isn't translated (it uses Minecraft's `TextureSlot`) (org/betterx/betternether/client/block/BNModels.java:146)
- [ ] `BNModels.TextureSource.slot(0)`: its class isn't translated (it uses Minecraft's `TextureSlot`) (org/betterx/betternether/client/block/BNModels.java:145)
- [ ] `BNModels.TextureSource.texture(0)`: its class isn't translated (it uses Minecraft's `TextureSlot`) (org/betterx/betternether/client/block/BNModels.java:145)

### `BNModels.VariantSupplier`: it's an interface

- [ ] `BNModels.VariantSupplier.apply(2)`: it has no body (org/betterx/betternether/client/block/BNModels.java:152)

### `CommandRegistry`

- [ ] `CommandRegistry.ERROR_BIOME_NOT_FOUND`: it uses Minecraft's `DynamicCommandExceptionType` (org/betterx/betternether/commands/CommandRegistry.java:65)
- [x] `CommandRegistry.MAX_SEARCH_RADIUS` (org/betterx/betternether/commands/CommandRegistry.java:68)
- [x] `CommandRegistry.SAMPLE_RESOLUTION_HORIZONTAL` (org/betterx/betternether/commands/CommandRegistry.java:69)
- [x] `CommandRegistry.SAMPLE_RESOLUTION_VERTICAL` (org/betterx/betternether/commands/CommandRegistry.java:70)
- [x] `CommandRegistry.biomeIndex` (org/betterx/betternether/commands/CommandRegistry.java:71)
- [x] `CommandRegistry.placeMapIdx` (org/betterx/betternether/commands/CommandRegistry.java:72)
- [ ] `CommandRegistry.states`: it uses Minecraft's `BlockState` (org/betterx/betternether/commands/CommandRegistry.java:73)
- [ ] `CommandRegistry.states2`: it uses Minecraft's `BlockState` (org/betterx/betternether/commands/CommandRegistry.java:85)
- [ ] `CommandRegistry.BIOME_INFO_NOISE`: it uses Minecraft's `PerlinNoise` (org/betterx/betternether/commands/CommandRegistry.java:97)
- [ ] `CommandRegistry.register(0)`: it uses Fabric's `CommandRegistrationCallback`.EVENT (org/betterx/betternether/commands/CommandRegistry.java:99)
- [ ] `CommandRegistry.register(3)`: it uses Minecraft's `CommandDispatcher` (org/betterx/betternether/commands/CommandRegistry.java:103)
- [ ] `CommandRegistry.teleportToNextBiome(1)`: its body couldn't be read: an anonymous class (org/betterx/betternether/commands/CommandRegistry.java:134)
- [ ] `CommandRegistry.placeMatchingBlocks(2)`: it uses Minecraft's `ServerPlayer` (org/betterx/betternether/commands/CommandRegistry.java:211)
- [ ] `CommandRegistry.compareBlockNames(2)`: it uses Minecraft's `Block` (org/betterx/betternether/commands/CommandRegistry.java:228)
- [ ] `CommandRegistry.findSurface(1)`: it uses Minecraft's `ServerPlayer` (org/betterx/betternether/commands/CommandRegistry.java:235)
- [ ] `CommandRegistry.testPlace(1)`: it uses Minecraft's `ServerPlayer` (org/betterx/betternether/commands/CommandRegistry.java:251)
- [ ] `CommandRegistry.placeAllBlocks(1)`: it uses Minecraft's `ServerPlayer` (org/betterx/betternether/commands/CommandRegistry.java:311)
- [ ] `CommandRegistry.placeBlockRow(5)`: it uses Minecraft's `ServerPlayer` (org/betterx/betternether/commands/CommandRegistry.java:344)
- [ ] `CommandRegistry.place(4)`: it uses Minecraft's `ServerPlayer` (org/betterx/betternether/commands/CommandRegistry.java:367)

### `PlaceCommand`

- [ ] `PlaceCommand.ERROR_STRUCTURE_INVALID`: it uses Minecraft's `DynamicCommandExceptionType` (org/betterx/betternether/commands/PlaceCommand.java:20)
- [ ] `PlaceCommand.ERROR_NBT_STRUCTURE_NOT_FOUND`: it uses Minecraft's `DynamicCommandExceptionType` (org/betterx/betternether/commands/PlaceCommand.java:23)
- [ ] `PlaceCommand.register(1)`: it uses Minecraft's `LiteralArgumentBuilder` (org/betterx/betternether/commands/PlaceCommand.java:27)
- [ ] `PlaceCommand.placeStructure(2)`: it uses Minecraft's `Registry` (org/betterx/betternether/commands/PlaceCommand.java:45)

### `ClientConfig`: it extends `ConfigFile` (de.ambertation.wunderlib.configs)

- [ ] `ClientConfig.ClientConfig(0)`: its class isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)) (org/betterx/betternether/config/ClientConfig.java:27)

### `Config`

- [ ] `Config.ALL`: it calls `Lists` (com.google.common.collect).newArrayList (org/betterx/betternether/config/Config.java:22)
- [ ] `Config.Config(1)`: it uses Config.ALL, which isn't translated (it calls `Lists` (com.google.common.collect).newArrayList) (org/betterx/betternether/config/Config.java:27)
- [ ] `Config.load(0)`: it uses `File` (java.io) (org/betterx/betternether/config/Config.java:32)
- [ ] `Config.save(0)`: it uses Config.ALL, which isn't translated (it calls `Lists` (com.google.common.collect).newArrayList) (org/betterx/betternether/config/Config.java:64)
- [ ] `Config.getFile(0)`: it uses `File` (java.io) (org/betterx/betternether/config/Config.java:89)
- [ ] `Config.getFolder(0)`: it uses `File` (java.io) (org/betterx/betternether/config/Config.java:93)
- [ ] `Config.getBoolean(3)`: it calls Config.load, which isn't translated (org/betterx/betternether/config/Config.java:97)
- [ ] `Config.setBoolean(4)`: it changes an object through a second name (org/betterx/betternether/config/Config.java:111)
- [ ] `Config.getFloat(3)`: it calls Config.load, which isn't translated (org/betterx/betternether/config/Config.java:118)
- [ ] `Config.setFloat(4)`: it changes an object through a second name (org/betterx/betternether/config/Config.java:132)
- [ ] `Config.getInt(3)`: it calls Config.load, which isn't translated (org/betterx/betternether/config/Config.java:139)
- [ ] `Config.getString(3)`: it calls Config.load, which isn't translated (org/betterx/betternether/config/Config.java:153)
- [ ] `Config.setInt(4)`: it changes an object through a second name (org/betterx/betternether/config/Config.java:167)
- [ ] `Config.setStringLoad(3)`: it changes an object through a second name (org/betterx/betternether/config/Config.java:174)
- [ ] `Config.getStringArray(3)`: it calls Config.load, which isn't translated (org/betterx/betternether/config/Config.java:179)
- [ ] `Config.toStringArray(1)`: it calls Config.load, which isn't translated (org/betterx/betternether/config/Config.java:192)
- [ ] `Config.toJsonArray(1)`: it calls Config.load, which isn't translated (org/betterx/betternether/config/Config.java:203)
- [ ] `Config.getGroup(1)`: it changes an object through a second name (org/betterx/betternether/config/Config.java:214)
- [ ] `Config.getBaseGroups(0)`: .iterator on a map's entries (org/betterx/betternether/config/Config.java:231)
- [ ] `Config.getGroupMembers(1)`: Minecraft's entry kept in a variable or field (org/betterx/betternether/config/Config.java:238)
- [x] `Config.markToSave(0)` (org/betterx/betternether/config/Config.java:244)

### `Configs`

- [ ] `Configs.CLIENT`: it uses ClientConfig, which isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)) (org/betterx/betternether/config/Configs.java:4)
- [ ] `Configs.GAME_RULES`: it uses DefaultWorldConfig, which isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)) (org/betterx/betternether/config/Configs.java:5)
- [ ] `Configs.WORLD`: it uses WorldConfig, which isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)) (org/betterx/betternether/config/Configs.java:6)
- [ ] `Configs.saveConfigs(0)`: unknown name `de` (org/betterx/betternether/config/Configs.java:8)

### `DefaultWorldConfig`: it extends `ConfigFile` (de.ambertation.wunderlib.configs)

- [ ] `DefaultWorldConfig.DAMAGE_GROUP`: it uses `Group` (de.ambertation.wunderlib.configs.AbstractConfig) (org/betterx/betternether/config/DefaultWorldConfig.java:11)
- [ ] `DefaultWorldConfig.DefaultWorldConfig(0)`: its class isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)) (org/betterx/betternether/config/DefaultWorldConfig.java:21)

### `ConfigScreen`: it extends `ConfigScreen`

- [ ] `ConfigScreen.BN_LOGO_LOCATION`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/config/screen/ConfigScreen.java:11)
- [ ] `ConfigScreen.ConfigScreen(1)`: its class isn't translated (it extends `ConfigScreen`) (org/betterx/betternether/config/screen/ConfigScreen.java:13)
- [ ] `ConfigScreen.onClose(0)`: its class isn't translated (it extends `ConfigScreen`) (org/betterx/betternether/config/screen/ConfigScreen.java:17)

### `WorldConfig`: it extends `ConfigFile` (de.ambertation.wunderlib.configs)

- [ ] `WorldConfig.WorldConfig(0)`: its class isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)) (org/betterx/betternether/config/WorldConfig.java:15)

### `ObsidianBreaker`

- [ ] `ObsidianBreaker.modifyObsidianBreakerSpeed(3)`: it uses Minecraft's `BlockState` (org/betterx/betternether/enchantments/ObsidianBreaker.java:9)

### `RubyFire`

- [ ] `RubyFire.ARMOR_SLOTS`: it uses Minecraft's `EquipmentSlot` (org/betterx/betternether/enchantments/RubyFire.java:33)
- [ ] `RubyFire.FIRE_CONVERSIONS`: it uses Minecraft's `Item` (org/betterx/betternether/enchantments/RubyFire.java:34)
- [ ] `RubyFire.convertedDrops`: it uses `ThreadLocal` (org/betterx/betternether/enchantments/RubyFire.java:35)
- [ ] `RubyFire.getDrops(5)`: it uses Minecraft's `BlockState` (org/betterx/betternether/enchantments/RubyFire.java:37)
- [ ] `RubyFire.popExperience(3)`: it uses Minecraft's `ServerLevel` (org/betterx/betternether/enchantments/RubyFire.java:102)
- [ ] `RubyFire.buildConversionTable(1)`: it uses Minecraft's `ServerLevel` (org/betterx/betternether/enchantments/RubyFire.java:116)

### `EntityFirefly`: it extends `DespawnableAnimal` (org.betterx.bclib.entity)

- [ ] `EntityFirefly.SEARCH`: it uses Minecraft's `Vec3i` (org/betterx/betternether/entity/EntityFirefly.java:58)
- [ ] `EntityFirefly.COLOR`: it uses Minecraft's `EntityDataAccessor` (org/betterx/betternether/entity/EntityFirefly.java:59)
- [ ] `EntityFirefly.EntityFirefly(2)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:62)
- [ ] `EntityFirefly.defineSynchedData(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:72)
- [ ] `EntityFirefly.createMobAttributes(0)`: it uses Minecraft's `Builder` (org/betterx/betternether/entity/EntityFirefly.java:77)
- [ ] `EntityFirefly.createNavigation(1)`: its body couldn't be read: an anonymous class (org/betterx/betternether/entity/EntityFirefly.java:85)
- [ ] `EntityFirefly.registerGoals(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:109)
- [ ] `EntityFirefly.getWalkTargetValue(2)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:119)
- [ ] `EntityFirefly.isFood(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:123)
- [ ] `EntityFirefly.isFlapping(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:127)
- [ ] `EntityFirefly.jumpInLiquid(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:131)
- [ ] `EntityFirefly.getLightLevelDependentMagicValue(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:135)
- [ ] `EntityFirefly.causeFallDamage(3)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:139)
- [ ] `EntityFirefly.checkFallDamage(4)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:143)
- [ ] `EntityFirefly.isNoGravity(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:146)
- [ ] `EntityFirefly.getColor(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:150)
- [ ] `EntityFirefly.addAdditionalSaveData(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:154)
- [ ] `EntityFirefly.readAdditionalSaveData(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:159)
- [ ] `EntityFirefly.getBreedOffspring(2)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:177)
- [ ] `EntityFirefly.omnidirectionalAirMover(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:181)
- [ ] `EntityFirefly.checkMovement(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:185)
- [ ] `EntityFirefly.getAmbientSound(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:196)
- [ ] `EntityFirefly.getSoundVolume(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:200)
- [ ] `EntityFirefly.makeColor(3)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:204)
- [ ] `EntityFirefly.getMaxSpawnClusterSize(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:248)
- [ ] `EntityFirefly.canSpawn(5)`: it uses Minecraft's `EntityType` (org/betterx/betternether/entity/EntityFirefly.java:252)
- [ ] `EntityFirefly.isPushable(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFirefly.java:273)

### `EntityFirefly.FreflyLookControl`: it extends Minecraft's `LookControl`

- [ ] `EntityFirefly.FreflyLookControl.FreflyLookControl(1)`: its class isn't translated (it extends Minecraft's `LookControl`) (org/betterx/betternether/entity/EntityFirefly.java:303)
- [ ] `EntityFirefly.FreflyLookControl.resetXRotOnTick(0)`: its class isn't translated (it extends Minecraft's `LookControl`) (org/betterx/betternether/entity/EntityFirefly.java:307)

### `EntityFirefly.MoveRandomGoal`: it extends Minecraft's `Goal`

- [ ] `EntityFirefly.MoveRandomGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:316)
- [ ] `EntityFirefly.MoveRandomGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:320)
- [ ] `EntityFirefly.MoveRandomGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:324)
- [ ] `EntityFirefly.MoveRandomGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:336)

### `EntityFirefly.MoveToFlowersGoal`: it extends Minecraft's `Goal`

- [ ] `EntityFirefly.MoveToFlowersGoal.MoveToFlowersGoal(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:343)
- [ ] `EntityFirefly.MoveToFlowersGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:347)
- [ ] `EntityFirefly.MoveToFlowersGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:351)
- [ ] `EntityFirefly.MoveToFlowersGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:355)
- [ ] `EntityFirefly.MoveToFlowersGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:365)
- [ ] `EntityFirefly.MoveToFlowersGoal.getFlowerLocation(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:373)
- [ ] `EntityFirefly.MoveToFlowersGoal.isFlower(1)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:387)
- [ ] `EntityFirefly.MoveToFlowersGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:391)

### `EntityFirefly.SittingGoal`: it extends Minecraft's `Goal`

- [ ] `EntityFirefly.SittingGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:401)
- [ ] `EntityFirefly.SittingGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:411)
- [ ] `EntityFirefly.SittingGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:415)
- [ ] `EntityFirefly.SittingGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:423)
- [ ] `EntityFirefly.SittingGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:428)

### `EntityFirefly.WanderAroundGoal`: it extends Minecraft's `Goal`

- [ ] `EntityFirefly.WanderAroundGoal.WanderAroundGoal(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:435)
- [ ] `EntityFirefly.WanderAroundGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:439)
- [ ] `EntityFirefly.WanderAroundGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:443)
- [ ] `EntityFirefly.WanderAroundGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:447)
- [ ] `EntityFirefly.WanderAroundGoal.getRandomLocation(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:459)
- [ ] `EntityFirefly.WanderAroundGoal.randomRange(1)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:490)
- [ ] `EntityFirefly.WanderAroundGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFirefly.java:495)

### `EntityFlyingPig`: it extends `DespawnableAnimal` (org.betterx.bclib.entity)

- [ ] `EntityFlyingPig.DATA_SHARED_FLAGS_ID`: it uses Minecraft's `EntityDataAccessor` (org/betterx/betternether/entity/EntityFlyingPig.java:62)
- [x] `EntityFlyingPig.BIT_ROOSTING` (org/betterx/betternether/entity/EntityFlyingPig.java:63)
- [x] `EntityFlyingPig.BIT_WARTED` (org/betterx/betternether/entity/EntityFlyingPig.java:64)
- [ ] `EntityFlyingPig.EntityFlyingPig(2)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:67)
- [ ] `EntityFlyingPig.getFlyingSpeed(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:75)
- [ ] `EntityFlyingPig.registerGoals(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:79)
- [ ] `EntityFlyingPig.createMobAttributes(0)`: it uses Builder, which isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/entity/EntityFlyingPig.java:90)
- [ ] `EntityFlyingPig.createNavigation(1)`: its body couldn't be read: an anonymous class (org/betterx/betternether/entity/EntityFlyingPig.java:100)
- [ ] `EntityFlyingPig.defineSynchedData(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:111)
- [ ] `EntityFlyingPig.addAdditionalSaveData(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:116)
- [ ] `EntityFlyingPig.readAdditionalSaveData(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:121)
- [ ] `EntityFlyingPig.isRoosting(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:126)
- [ ] `EntityFlyingPig.setRoosting(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:131)
- [ ] `EntityFlyingPig.isWarted(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:136)
- [ ] `EntityFlyingPig.setWarted(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:141)
- [ ] `EntityFlyingPig.getSoundVolume(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:146)
- [ ] `EntityFlyingPig.getVoicePitch(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:150)
- [ ] `EntityFlyingPig.classicPigSounds(0)`: it uses Minecraft's `PigSoundSet` (org/betterx/betternether/entity/EntityFlyingPig.java:154)
- [ ] `EntityFlyingPig.getHurtSound(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:158)
- [ ] `EntityFlyingPig.getDeathSound(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:162)
- [ ] `EntityFlyingPig.getAmbientSound(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:166)
- [ ] `EntityFlyingPig.isPushable(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:170)
- [ ] `EntityFlyingPig.doPush(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:174)
- [ ] `EntityFlyingPig.pushEntities(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:177)
- [ ] `EntityFlyingPig.isFlapping(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:180)
- [ ] `EntityFlyingPig.causeFallDamage(3)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:184)
- [ ] `EntityFlyingPig.checkFallDamage(4)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:188)
- [ ] `EntityFlyingPig.tickDeath(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:191)
- [ ] `EntityFlyingPig.getMaxSpawnClusterSize(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:199)
- [ ] `EntityFlyingPig.omnidirectionalAirMover(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:203)
- [ ] `EntityFlyingPig.getBreedOffspring(2)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:207)
- [ ] `EntityFlyingPig.isFood(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityFlyingPig.java:213)

### `EntityFlyingPig.FindFoodGoal`: it extends Minecraft's `Goal`

- [ ] `EntityFlyingPig.FindFoodGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:221)
- [ ] `EntityFlyingPig.FindFoodGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:225)
- [ ] `EntityFlyingPig.FindFoodGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:236)
- [ ] `EntityFlyingPig.FindFoodGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:240)
- [ ] `EntityFlyingPig.FindFoodGoal.getFood(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:265)
- [ ] `EntityFlyingPig.FindFoodGoal.hasNearFood(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:270)

### `EntityFlyingPig.RoostingGoal`: it extends Minecraft's `Goal`

- [ ] `EntityFlyingPig.RoostingGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:283)
- [ ] `EntityFlyingPig.RoostingGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:290)
- [ ] `EntityFlyingPig.RoostingGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:294)
- [ ] `EntityFlyingPig.RoostingGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:307)
- [ ] `EntityFlyingPig.RoostingGoal.getRoostingLocation(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:317)

### `EntityFlyingPig.SittingGoal`: it extends Minecraft's `Goal`

- [ ] `EntityFlyingPig.SittingGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:330)
- [ ] `EntityFlyingPig.SittingGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:334)
- [ ] `EntityFlyingPig.SittingGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:338)
- [ ] `EntityFlyingPig.SittingGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:346)
- [ ] `EntityFlyingPig.SittingGoal.tick(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:353)

### `EntityFlyingPig.WanderAroundGoal`: it extends Minecraft's `Goal`

- [ ] `EntityFlyingPig.WanderAroundGoal.WanderAroundGoal(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:360)
- [ ] `EntityFlyingPig.WanderAroundGoal.canUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:364)
- [ ] `EntityFlyingPig.WanderAroundGoal.canContinueToUse(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:368)
- [ ] `EntityFlyingPig.WanderAroundGoal.start(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:372)
- [ ] `EntityFlyingPig.WanderAroundGoal.getRandomLocation(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:390)
- [ ] `EntityFlyingPig.WanderAroundGoal.randomRange(1)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:419)
- [ ] `EntityFlyingPig.WanderAroundGoal.stop(0)`: its class isn't translated (it extends Minecraft's `Goal`) (org/betterx/betternether/entity/EntityFlyingPig.java:424)

### `EntityHydrogenJellyfish`: it extends `DespawnableAnimal` (org.betterx.bclib.entity)

- [ ] `EntityHydrogenJellyfish.EntityHydrogenJellyfish(2)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:40)
- [ ] `EntityHydrogenJellyfish.createMobAttributes(0)`: it uses Builder, which isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:47)
- [ ] `EntityHydrogenJellyfish.isFlapping(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:56)
- [ ] `EntityHydrogenJellyfish.jumpInLiquid(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:60)
- [ ] `EntityHydrogenJellyfish.isNoGravity(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:64)
- [ ] `EntityHydrogenJellyfish.isFood(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:68)
- [ ] `EntityHydrogenJellyfish.playerTouch(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:72)
- [ ] `EntityHydrogenJellyfish.customServerAiStep(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:76)
- [x] `EntityHydrogenJellyfish.lerpAngleDegrees(3)` (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:114)
- [ ] `EntityHydrogenJellyfish.getMaxSpawnClusterSize(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:118)
- [ ] `EntityHydrogenJellyfish.die(1)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:122)
- [ ] `EntityHydrogenJellyfish.getAmbientSound(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:144)
- [ ] `EntityHydrogenJellyfish.getSoundVolume(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:148)
- [ ] `EntityHydrogenJellyfish.getBreedOffspring(2)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:152)
- [ ] `EntityHydrogenJellyfish.causeFallDamage(3)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:156)
- [ ] `EntityHydrogenJellyfish.checkFallDamage(4)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:160)
- [ ] `EntityHydrogenJellyfish.hurtServer(3)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:163)
- [ ] `EntityHydrogenJellyfish.canSpawn(5)`: it uses Minecraft's `EntityType` (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:169)
- [ ] `EntityHydrogenJellyfish.omnidirectionalAirMover(0)`: its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)) (org/betterx/betternether/entity/EntityHydrogenJellyfish.java:181)

### `EntityJungleSkeleton`: it extends Minecraft's `Skeleton`

- [ ] `EntityJungleSkeleton.EntityJungleSkeleton(2)`: its class isn't translated (it extends Minecraft's `Skeleton`) (org/betterx/betternether/entity/EntityJungleSkeleton.java:21)
- [ ] `EntityJungleSkeleton.checkDespawn(0)`: its class isn't translated (it extends Minecraft's `Skeleton`) (org/betterx/betternether/entity/EntityJungleSkeleton.java:25)
- [ ] `EntityJungleSkeleton.finalizeSpawn(4)`: its class isn't translated (it extends Minecraft's `Skeleton`) (org/betterx/betternether/entity/EntityJungleSkeleton.java:33)
- [ ] `EntityJungleSkeleton.getHandItem(0)`: its class isn't translated (it extends Minecraft's `Skeleton`) (org/betterx/betternether/entity/EntityJungleSkeleton.java:58)
- [ ] `EntityJungleSkeleton.getRandomOffhandItem(0)`: its class isn't translated (it extends Minecraft's `Skeleton`) (org/betterx/betternether/entity/EntityJungleSkeleton.java:71)

### `EntityNaga`: it extends Minecraft's `Monster`

- [ ] `EntityNaga.EntityNaga(2)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntityNaga.java:33)
- [ ] `EntityNaga.registerGoals(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntityNaga.java:38)
- [ ] `EntityNaga.createMobAttributes(0)`: it uses Builder, which isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/entity/EntityNaga.java:46)
- [ ] `EntityNaga.performRangedAttack(2)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntityNaga.java:55)
- [ ] `EntityNaga.canBeAffected(1)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntityNaga.java:65)
- [ ] `EntityNaga.getAmbientSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntityNaga.java:69)
- [ ] `EntityNaga.getHurtSound(1)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntityNaga.java:73)
- [ ] `EntityNaga.getDeathSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntityNaga.java:77)
- [ ] `EntityNaga.checkDespawn(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntityNaga.java:81)
- [ ] `EntityNaga.getMaxHeadYRot(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntityNaga.java:89)
- [ ] `EntityNaga.canSpawn(5)`: it uses Minecraft's `EntityType` (org/betterx/betternether/entity/EntityNaga.java:93)

### `EntityNagaProjectile`: it extends Minecraft's `Mob`

- [x] `EntityNagaProjectile.MAX_LIFE_TIME` (org/betterx/betternether/entity/EntityNagaProjectile.java:26)
- [ ] `EntityNagaProjectile.EntityNagaProjectile(2)`: its class isn't translated (it extends Minecraft's `Mob`) (org/betterx/betternether/entity/EntityNagaProjectile.java:29)
- [ ] `EntityNagaProjectile.setParams(2)`: its class isn't translated (it extends Minecraft's `Mob`) (org/betterx/betternether/entity/EntityNagaProjectile.java:34)
- [ ] `EntityNagaProjectile.isNoGravity(0)`: its class isn't translated (it extends Minecraft's `Mob`) (org/betterx/betternether/entity/EntityNagaProjectile.java:43)
- [ ] `EntityNagaProjectile.checkFallDamage(4)`: its class isn't translated (it extends Minecraft's `Mob`) (org/betterx/betternether/entity/EntityNagaProjectile.java:47)
- [ ] `EntityNagaProjectile.travel(1)`: its class isn't translated (it extends Minecraft's `Mob`) (org/betterx/betternether/entity/EntityNagaProjectile.java:50)
- [ ] `EntityNagaProjectile.onClimbable(0)`: its class isn't translated (it extends Minecraft's `Mob`) (org/betterx/betternether/entity/EntityNagaProjectile.java:81)
- [ ] `EntityNagaProjectile.shouldRenderAtSqrDistance(1)`: its class isn't translated (it extends Minecraft's `Mob`) (org/betterx/betternether/entity/EntityNagaProjectile.java:85)
- [ ] `EntityNagaProjectile.tick(0)`: its class isn't translated (it extends Minecraft's `Mob`) (org/betterx/betternether/entity/EntityNagaProjectile.java:90)
- [ ] `EntityNagaProjectile.isSame(2)`: its class isn't translated (it extends Minecraft's `Mob`) (org/betterx/betternether/entity/EntityNagaProjectile.java:127)
- [ ] `EntityNagaProjectile.onCollision(1)`: its class isn't translated (it extends Minecraft's `Mob`) (org/betterx/betternether/entity/EntityNagaProjectile.java:131)
- [ ] `EntityNagaProjectile.effectKill(0)`: its class isn't translated (it extends Minecraft's `Mob`) (org/betterx/betternether/entity/EntityNagaProjectile.java:162)
- [ ] `EntityNagaProjectile.canBeAffected(1)`: its class isn't translated (it extends Minecraft's `Mob`) (org/betterx/betternether/entity/EntityNagaProjectile.java:179)
- [ ] `EntityNagaProjectile.isSilent(0)`: its class isn't translated (it extends Minecraft's `Mob`) (org/betterx/betternether/entity/EntityNagaProjectile.java:183)
- [ ] `EntityNagaProjectile.addAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Mob`) (org/betterx/betternether/entity/EntityNagaProjectile.java:187)
- [ ] `EntityNagaProjectile.readAdditionalSaveData(1)`: its class isn't translated (it extends Minecraft's `Mob`) (org/betterx/betternether/entity/EntityNagaProjectile.java:192)

### `EntitySkull`: it extends Minecraft's `Monster`

- [x] `EntitySkull.particleX` (org/betterx/betternether/entity/EntitySkull.java:33)
- [x] `EntitySkull.particleY` (org/betterx/betternether/entity/EntitySkull.java:34)
- [x] `EntitySkull.particleZ` (org/betterx/betternether/entity/EntitySkull.java:35)
- [ ] `EntitySkull.EntitySkull(2)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:40)
- [ ] `EntitySkull.createMobAttributes(0)`: it uses Builder, which isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/entity/EntitySkull.java:50)
- [ ] `EntitySkull.getFlyingSpeed(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:59)
- [ ] `EntitySkull.omnidirectionalAirMover(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:63)
- [ ] `EntitySkull.getAttackDamage(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:67)
- [ ] `EntitySkull.playerTouch(1)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:71)
- [ ] `EntitySkull.checkDespawn(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:89)
- [ ] `EntitySkull.tick(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:97)
- [ ] `EntitySkull.getAmbientSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:137)
- [ ] `EntitySkull.getHurtSound(1)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:141)
- [ ] `EntitySkull.getDeathSound(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:145)
- [ ] `EntitySkull.moveRandomDir(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:149)
- [ ] `EntitySkull.lookAt(1)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:171)
- [ ] `EntitySkull.updateParticlePos(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:182)
- [ ] `EntitySkull.isFlapping(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:198)
- [ ] `EntitySkull.causeFallDamage(3)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:202)
- [ ] `EntitySkull.checkFallDamage(4)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:206)
- [ ] `EntitySkull.isNoGravity(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:209)
- [ ] `EntitySkull.isPushable(0)`: its class isn't translated (it extends Minecraft's `Monster`) (org/betterx/betternether/entity/EntitySkull.java:213)
- [ ] `EntitySkull.canSpawn(5)`: it uses Minecraft's `EntityType` (org/betterx/betternether/entity/EntitySkull.java:217)

### `EntitySkull.SkullLookControl`: it extends Minecraft's `LookControl`

- [ ] `EntitySkull.SkullLookControl.SkullLookControl(1)`: its class isn't translated (it extends Minecraft's `LookControl`) (org/betterx/betternether/entity/EntitySkull.java:236)
- [ ] `EntitySkull.SkullLookControl.resetXRotOnTick(0)`: its class isn't translated (it extends Minecraft's `LookControl`) (org/betterx/betternether/entity/EntitySkull.java:240)

### `ModelEntityFirefly`: it extends Minecraft's `EntityModel`

- [x] `ModelEntityFirefly.GLOW` (org/betterx/betternether/entity/model/ModelEntityFirefly.java:19)
- [x] `ModelEntityFirefly.GLOW_PLANE` (org/betterx/betternether/entity/model/ModelEntityFirefly.java:20)
- [ ] `ModelEntityFirefly.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betternether/entity/model/ModelEntityFirefly.java:22)
- [ ] `ModelEntityFirefly.ModelEntityFirefly(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betternether/entity/model/ModelEntityFirefly.java:39)
- [ ] `ModelEntityFirefly.setupAnim(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betternether/entity/model/ModelEntityFirefly.java:46)
- [ ] `ModelEntityFirefly.syncTransform(0)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betternether/entity/model/ModelEntityFirefly.java:50)
- [ ] `ModelEntityFirefly.getGlowPart(0)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betternether/entity/model/ModelEntityFirefly.java:54)

### `ModelEntityFlyingPig`: it extends Minecraft's `EntityModel`

- [ ] `ModelEntityFlyingPig.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betternether/entity/model/ModelEntityFlyingPig.java:27)
- [ ] `ModelEntityFlyingPig.ModelEntityFlyingPig(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betternether/entity/model/ModelEntityFlyingPig.java:71)
- [ ] `ModelEntityFlyingPig.setupAnim(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betternether/entity/model/ModelEntityFlyingPig.java:84)

### `ModelEntityHydrogenJellyfish`: it extends Minecraft's `EntityModel`

- [x] `ModelEntityHydrogenJellyfish.BODY_TOP` (org/betterx/betternether/entity/model/ModelEntityHydrogenJellyfish.java:16)
- [x] `ModelEntityHydrogenJellyfish.HALF_PI` (org/betterx/betternether/entity/model/ModelEntityHydrogenJellyfish.java:17)
- [x] `ModelEntityHydrogenJellyfish.LEGS` (org/betterx/betternether/entity/model/ModelEntityHydrogenJellyfish.java:18)
- [ ] `ModelEntityHydrogenJellyfish.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betternether/entity/model/ModelEntityHydrogenJellyfish.java:25)
- [ ] `ModelEntityHydrogenJellyfish.ModelEntityHydrogenJellyfish(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betternether/entity/model/ModelEntityHydrogenJellyfish.java:94)
- [ ] `ModelEntityHydrogenJellyfish.setupAnim(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betternether/entity/model/ModelEntityHydrogenJellyfish.java:110)

### `ModelJungleSkeleton`: it extends Minecraft's `SkeletonModel`

- [x] `ModelJungleSkeleton.ANGLE45` (org/betterx/betternether/entity/model/ModelJungleSkeleton.java:20)
- [x] `ModelJungleSkeleton.ANGLE90` (org/betterx/betternether/entity/model/ModelJungleSkeleton.java:21)
- [ ] `ModelJungleSkeleton.RANDOM`: it creates Minecraft's `LegacyRandomSource` (org/betterx/betternether/entity/model/ModelJungleSkeleton.java:22)
- [x] `ModelJungleSkeleton.BOUND_MIN` (org/betterx/betternether/entity/model/ModelJungleSkeleton.java:23)
- [x] `ModelJungleSkeleton.BOUND_MAX` (org/betterx/betternether/entity/model/ModelJungleSkeleton.java:24)
- [ ] `ModelJungleSkeleton.createBodyLayer(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betternether/entity/model/ModelJungleSkeleton.java:26)
- [ ] `ModelJungleSkeleton.ModelJungleSkeleton(1)`: its class isn't translated (it extends Minecraft's `SkeletonModel`) (org/betterx/betternether/entity/model/ModelJungleSkeleton.java:51)

### `ModelNaga`: it extends Minecraft's `EntityModel`

- [x] `ModelNaga.SPIKE_COUNT` (org/betterx/betternether/entity/model/ModelNaga.java:16)
- [x] `ModelNaga.TAIL_COUNT` (org/betterx/betternether/entity/model/ModelNaga.java:17)
- [ ] `ModelNaga.getTexturedModelData(0)`: its body couldn't be read: an assert (org/betterx/betternether/entity/model/ModelNaga.java:25)
- [ ] `ModelNaga.ModelNaga(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betternether/entity/model/ModelNaga.java:86)
- [ ] `ModelNaga.setupAnim(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betternether/entity/model/ModelNaga.java:110)
- [ ] `ModelNaga.lerpAngle(3)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betternether/entity/model/ModelNaga.java:152)

### `ModelSkull`: it extends Minecraft's `EntityModel`

- [ ] `ModelSkull.getTexturedModelData(0)`: it uses Minecraft's `LayerDefinition` (org/betterx/betternether/entity/model/ModelSkull.java:19)
- [ ] `ModelSkull.ModelSkull(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betternether/entity/model/ModelSkull.java:28)
- [ ] `ModelSkull.setupAnim(1)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betternether/entity/model/ModelSkull.java:33)
- [ ] `ModelSkull.lerpAngle(3)`: its class isn't translated (it extends Minecraft's `EntityModel`) (org/betterx/betternether/entity/model/ModelSkull.java:49)

### `FireflyGlowFeatureRenderer`: it extends Minecraft's `RenderLayer`

- [x] `FireflyGlowFeatureRenderer.LIT` (org/betterx/betternether/entity/render/FireflyGlowFeatureRenderer.java:25)
- [ ] `FireflyGlowFeatureRenderer.TEXTURE`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/entity/render/FireflyGlowFeatureRenderer.java:26)
- [ ] `FireflyGlowFeatureRenderer.FireflyGlowFeatureRenderer(1)`: its class isn't translated (it extends Minecraft's `RenderLayer`) (org/betterx/betternether/entity/render/FireflyGlowFeatureRenderer.java:28)
- [ ] `FireflyGlowFeatureRenderer.submit(6)`: its class isn't translated (it extends Minecraft's `RenderLayer`) (org/betterx/betternether/entity/render/FireflyGlowFeatureRenderer.java:32)
- [ ] `FireflyGlowFeatureRenderer.submitViewAlignedGlow(4)`: its class isn't translated (it extends Minecraft's `RenderLayer`) (org/betterx/betternether/entity/render/FireflyGlowFeatureRenderer.java:49)
- [ ] `FireflyGlowFeatureRenderer.addVertex(8)`: it uses Minecraft's `Pose` (org/betterx/betternether/entity/render/FireflyGlowFeatureRenderer.java:66)

### `FireflyRenderState`: it extends Minecraft's `LivingEntityRenderState`


### `FlyingPigRenderState`: it extends Minecraft's `LivingEntityRenderState`


### `HydrogenJellyfishRenderState`: it extends Minecraft's `LivingEntityRenderState`


### `JungleSkeletonRenderState`: it extends Minecraft's `SkeletonRenderState`


### `NagaRenderState`: it extends Minecraft's `LivingEntityRenderState`


### `RenderFirefly`: it extends Minecraft's `MobRenderer`

- [ ] `RenderFirefly.TEXTURE`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/entity/render/RenderFirefly.java:16)
- [ ] `RenderFirefly.RenderFirefly(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderFirefly.java:18)
- [ ] `RenderFirefly.createRenderState(0)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderFirefly.java:23)
- [ ] `RenderFirefly.extractRenderState(3)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderFirefly.java:27)
- [ ] `RenderFirefly.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderFirefly.java:32)
- [ ] `RenderFirefly.getBlockLightLevel(2)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderFirefly.java:36)

### `RenderFlyingPig`: it extends Minecraft's `MobRenderer`

- [ ] `RenderFlyingPig.TEXTURE`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/entity/render/RenderFlyingPig.java:15)
- [ ] `RenderFlyingPig.TEXTURE_WARTED`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/entity/render/RenderFlyingPig.java:16)
- [ ] `RenderFlyingPig.RenderFlyingPig(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderFlyingPig.java:18)
- [ ] `RenderFlyingPig.createRenderState(0)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderFlyingPig.java:22)
- [ ] `RenderFlyingPig.extractRenderState(3)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderFlyingPig.java:26)
- [ ] `RenderFlyingPig.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderFlyingPig.java:32)

### `RenderHydrogenJellyfish`: it extends Minecraft's `MobRenderer`

- [ ] `RenderHydrogenJellyfish.TEXTURE`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/entity/render/RenderHydrogenJellyfish.java:16)
- [ ] `RenderHydrogenJellyfish.RenderHydrogenJellyfish(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderHydrogenJellyfish.java:18)
- [ ] `RenderHydrogenJellyfish.createRenderState(0)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderHydrogenJellyfish.java:22)
- [ ] `RenderHydrogenJellyfish.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderHydrogenJellyfish.java:26)
- [ ] `RenderHydrogenJellyfish.getBlockLightLevel(2)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderHydrogenJellyfish.java:30)

### `RenderJungleSkeleton`: it extends Minecraft's `AbstractSkeletonRenderer`

- [ ] `RenderJungleSkeleton.TEXTURE`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/entity/render/RenderJungleSkeleton.java:16)
- [ ] `RenderJungleSkeleton.RenderJungleSkeleton(1)`: its class isn't translated (it extends Minecraft's `AbstractSkeletonRenderer`) (org/betterx/betternether/entity/render/RenderJungleSkeleton.java:18)
- [ ] `RenderJungleSkeleton.createRenderState(0)`: its class isn't translated (it extends Minecraft's `AbstractSkeletonRenderer`) (org/betterx/betternether/entity/render/RenderJungleSkeleton.java:22)
- [ ] `RenderJungleSkeleton.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `AbstractSkeletonRenderer`) (org/betterx/betternether/entity/render/RenderJungleSkeleton.java:26)

### `RenderNaga`: it extends Minecraft's `MobRenderer`

- [ ] `RenderNaga.TEXTURE`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/entity/render/RenderNaga.java:15)
- [ ] `RenderNaga.RenderNaga(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderNaga.java:17)
- [ ] `RenderNaga.createRenderState(0)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderNaga.java:21)
- [ ] `RenderNaga.extractRenderState(3)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderNaga.java:25)
- [ ] `RenderNaga.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderNaga.java:33)

### `RenderNagaProjectile`: it extends Minecraft's `EntityRenderer`

- [ ] `RenderNagaProjectile.TEXTURE`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/entity/render/RenderNagaProjectile.java:24)
- [ ] `RenderNagaProjectile.LAYER`: it uses Minecraft's `RenderType` (org/betterx/betternether/entity/render/RenderNagaProjectile.java:25)
- [ ] `RenderNagaProjectile.RenderNagaProjectile(1)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (org/betterx/betternether/entity/render/RenderNagaProjectile.java:27)
- [ ] `RenderNagaProjectile.getBlockLightLevel(2)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (org/betterx/betternether/entity/render/RenderNagaProjectile.java:31)
- [ ] `RenderNagaProjectile.createRenderState(0)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (org/betterx/betternether/entity/render/RenderNagaProjectile.java:35)
- [ ] `RenderNagaProjectile.submit(4)`: its class isn't translated (it extends Minecraft's `EntityRenderer`) (org/betterx/betternether/entity/render/RenderNagaProjectile.java:39)
- [ ] `RenderNagaProjectile.vertex(7)`: it uses Minecraft's `VertexConsumer` (org/betterx/betternether/entity/render/RenderNagaProjectile.java:58)

### `RenderPhaseAccessor`

- [ ] `RenderPhaseAccessor.FIREFLY_PIPELINE`: it uses Minecraft's `RenderPipeline` (org/betterx/betternether/entity/render/RenderPhaseAccessor.java:23)
- [ ] `RenderPhaseAccessor.FIREFLY_RENDER_TYPE`: it uses Minecraft's `RenderType` (org/betterx/betternether/entity/render/RenderPhaseAccessor.java:31)
- [ ] `RenderPhaseAccessor.createFirefly(1)`: it uses Minecraft's `RenderType` (org/betterx/betternether/entity/render/RenderPhaseAccessor.java:33)
- [ ] `RenderPhaseAccessor.getFirefly(1)`: it uses Minecraft's `RenderType` (org/betterx/betternether/entity/render/RenderPhaseAccessor.java:42)
- [ ] `RenderPhaseAccessor.assignIrisProgram(1)`: it uses Minecraft's `RenderPipeline` (org/betterx/betternether/entity/render/RenderPhaseAccessor.java:46)

### `RenderSkull`: it extends Minecraft's `MobRenderer`

- [ ] `RenderSkull.TEXTURE`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/entity/render/RenderSkull.java:21)
- [ ] `RenderSkull.RenderSkull(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderSkull.java:23)
- [ ] `RenderSkull.createRenderState(0)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderSkull.java:28)
- [ ] `RenderSkull.extractRenderState(3)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderSkull.java:32)
- [ ] `RenderSkull.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderSkull.java:38)
- [ ] `RenderSkull.getBlockLightLevel(2)`: its class isn't translated (it extends Minecraft's `MobRenderer`) (org/betterx/betternether/entity/render/RenderSkull.java:42)

### `RenderSkull.GlowFeatureRenderer`: it extends Minecraft's `EyesLayer`

- [ ] `RenderSkull.GlowFeatureRenderer.SKIN`: it uses Minecraft's `RenderType` (org/betterx/betternether/entity/render/RenderSkull.java:47)
- [ ] `RenderSkull.GlowFeatureRenderer.GlowFeatureRenderer(1)`: its class isn't translated (it extends Minecraft's `EyesLayer`) (org/betterx/betternether/entity/render/RenderSkull.java:49)
- [ ] `RenderSkull.GlowFeatureRenderer.renderType(0)`: its class isn't translated (it extends Minecraft's `EyesLayer`) (org/betterx/betternether/entity/render/RenderSkull.java:53)

### `SkullRenderState`: it extends Minecraft's `LivingEntityRenderState`


### `EMIForgeRecipe`


### `EMIPlugin`


### `JeiPlugin`: it implements `IModPlugin` (mezz.jei.api)

- [ ] `JeiPlugin.UID`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/integrations/jei/JeiPlugin.java:12)
- [ ] `JeiPlugin.getPluginUid(0)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (org/betterx/betternether/integrations/jei/JeiPlugin.java:14)
- [ ] `JeiPlugin.registerRecipeCatalysts(1)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (org/betterx/betternether/integrations/jei/JeiPlugin.java:18)

### `VanillaExcavatorsIntegration`

- [ ] `VanillaExcavatorsIntegration.hasExcavators`: it uses BetterNether.VANILLA_EXCAVATORS, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/integrations/VanillaExcavatorsIntegration.java:12)
- [ ] `VanillaExcavatorsIntegration.excavatorConstructor`: it uses `Constructor` (java.lang.reflect) (org/betterx/betternether/integrations/VanillaExcavatorsIntegration.java:13)
- [ ] `VanillaExcavatorsIntegration.makeExcavator(4)`: it uses Minecraft's `ToolMaterial` (org/betterx/betternether/integrations/VanillaExcavatorsIntegration.java:15)
- [ ] `VanillaExcavatorsIntegration.hasExcavators(0)`: it uses VanillaExcavatorsIntegration.hasExcavators, which isn't translated (it uses BetterNether.VANILLA_EXCAVATORS, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api))) (org/betterx/betternether/integrations/VanillaExcavatorsIntegration.java:28)

### `VanillaHammersIntegration`

- [ ] `VanillaHammersIntegration.hasHammers`: it uses BetterNether.VANILLA_HAMMERS, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/integrations/VanillaHammersIntegration.java:12)
- [ ] `VanillaHammersIntegration.hammerConstructor`: it uses `Constructor` (java.lang.reflect) (org/betterx/betternether/integrations/VanillaHammersIntegration.java:13)
- [ ] `VanillaHammersIntegration.makeHammer(4)`: it uses Minecraft's `ToolMaterial` (org/betterx/betternether/integrations/VanillaHammersIntegration.java:15)
- [ ] `VanillaHammersIntegration.hasHammers(0)`: it uses VanillaHammersIntegration.hasHammers, which isn't translated (it uses BetterNether.VANILLA_HAMMERS, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api))) (org/betterx/betternether/integrations/VanillaHammersIntegration.java:28)

### `DiamondSet`: it extends `NetherSet`

- [ ] `DiamondSet.DiamondSet(1)`: its class isn't translated (it extends `NetherSet`) (org/betterx/betternether/items/complex/DiamondSet.java:8)
- [ ] `DiamondSet.nameForSlot(1)`: its class isn't translated (it extends `NetherSet`) (org/betterx/betternether/items/complex/DiamondSet.java:12)

### `NetherSet`: it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)

- [ ] `NetherSet.NetherSet(4)`: its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)) (org/betterx/betternether/items/complex/NetherSet.java:37)
- [ ] `NetherSet.NetherSet(5)`: its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)) (org/betterx/betternether/items/complex/NetherSet.java:41)
- [ ] `NetherSet.shearsTool(1)`: it uses Minecraft's `Tool` (org/betterx/betternether/items/complex/NetherSet.java:89)
- [ ] `NetherSet.commonToolProperties(1)`: its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)) (org/betterx/betternether/items/complex/NetherSet.java:97)
- [ ] `NetherSet.addEquipmentSetSlotCriterion(2)`: its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)) (org/betterx/betternether/items/complex/NetherSet.java:102)
- [ ] `NetherSet.addEquipmentSetSlotCriterion(2)`: its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)) (org/betterx/betternether/items/complex/NetherSet.java:106)
- [ ] `NetherSet.addArmorSetCriterion(1)`: its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)) (org/betterx/betternether/items/complex/NetherSet.java:110)
- [ ] `NetherSet.addToolSetCriterion(1)`: its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)) (org/betterx/betternether/items/complex/NetherSet.java:118)

### `ItemBlackApple`: it extends Minecraft's `Item`

- [ ] `ItemBlackApple.BLACK_APPLE`: it uses Minecraft's `FoodProperties` (org/betterx/betternether/items/ItemBlackApple.java:15)
- [ ] `ItemBlackApple.ItemBlackApple(1)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/ItemBlackApple.java:17)
- [ ] `ItemBlackApple.finishUsingItem(3)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/ItemBlackApple.java:21)

### `ItemBowlFood`: it extends Minecraft's `Item`

- [ ] `ItemBowlFood.ItemBowlFood(3)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/ItemBowlFood.java:25)
- [ ] `ItemBowlFood.useOn(1)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/ItemBowlFood.java:31)
- [ ] `ItemBowlFood.finishUsingItem(3)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/ItemBowlFood.java:53)

### `ItemBowlMedicine`: it extends Minecraft's `Item`

- [ ] `ItemBowlMedicine.ItemBowlMedicine(1)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/ItemBowlMedicine.java:12)
- [ ] `ItemBowlMedicine.finishUsingItem(3)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/ItemBowlMedicine.java:16)

### `BNArmorMaterials`

- [ ] `BNArmorMaterials.CINCINNASITE`: it uses Minecraft's `Holder` (org/betterx/betternether/items/materials/BNArmorMaterials.java:10)
- [ ] `BNArmorMaterials.NETHER_RUBY`: it uses Minecraft's `Holder` (org/betterx/betternether/items/materials/BNArmorMaterials.java:20)
- [ ] `BNArmorMaterials.FLAMING_RUBY`: it uses Minecraft's `Holder` (org/betterx/betternether/items/materials/BNArmorMaterials.java:30)

### `BNArmorTiers`

- [ ] `BNArmorTiers.CINCINNASITE`: it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betternether/items/materials/BNArmorTiers.java:11)
- [ ] `BNArmorTiers.NETHER_RUBY`: it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betternether/items/materials/BNArmorTiers.java:15)
- [ ] `BNArmorTiers.FLAMING_RUBY`: it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betternether/items/materials/BNArmorTiers.java:19)

### `BNToolMaterial`

- [ ] `BNToolMaterial.CINCINNASITE`: it uses Minecraft's `ToolMaterial` (org/betterx/betternether/items/materials/BNToolMaterial.java:10)
- [ ] `BNToolMaterial.CINCINNASITE_DIAMOND`: it uses Minecraft's `ToolMaterial` (org/betterx/betternether/items/materials/BNToolMaterial.java:13)
- [ ] `BNToolMaterial.NETHER_RUBY`: it uses Minecraft's `ToolMaterial` (org/betterx/betternether/items/materials/BNToolMaterial.java:16)
- [ ] `BNToolMaterial.FLAMING_RUBY`: it uses Minecraft's `ToolMaterial` (org/betterx/betternether/items/materials/BNToolMaterial.java:19)

### `BNToolTiers`

- [ ] `BNToolTiers.CINCINNASITE`: it uses `ToolTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betternether/items/materials/BNToolTiers.java:13)
- [ ] `BNToolTiers.CINCINNASITE_DIAMOND`: it uses `ToolTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betternether/items/materials/BNToolTiers.java:19)
- [ ] `BNToolTiers.NETHER_RUBY`: it uses `ToolTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betternether/items/materials/BNToolTiers.java:27)
- [ ] `BNToolTiers.FLAMING_RUBY`: it uses `ToolTier` (de.ambertation.wover.complex.api.equipment) (org/betterx/betternether/items/materials/BNToolTiers.java:33)

### `NetherArmor`: it extends Minecraft's `Item`

- [ ] `NetherArmor.NetherArmor(2)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/NetherArmor.java:16)
- [ ] `NetherArmor.setupItemStack(2)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/NetherArmor.java:21)

### `NetherAxe`: it extends Minecraft's `Item`

- [ ] `NetherAxe.NetherAxe(4)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/NetherAxe.java:16)
- [ ] `NetherAxe.setupItemStack(2)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/NetherAxe.java:21)

### `NetherHoe`: it extends Minecraft's `Item`

- [ ] `NetherHoe.NetherHoe(4)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/NetherHoe.java:16)
- [ ] `NetherHoe.setupItemStack(2)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/NetherHoe.java:21)

### `NetherPickaxe`: it extends Minecraft's `Item`

- [ ] `NetherPickaxe.NetherPickaxe(2)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/NetherPickaxe.java:18)
- [ ] `NetherPickaxe.setupItemStack(2)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/NetherPickaxe.java:23)

### `NetherShovel`: it extends Minecraft's `Item`

- [ ] `NetherShovel.NetherShovel(4)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/NetherShovel.java:16)
- [ ] `NetherShovel.setupItemStack(2)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/NetherShovel.java:21)

### `NetherSpear`: it extends Minecraft's `Item`

- [ ] `NetherSpear.NetherSpear(2)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/NetherSpear.java:16)
- [ ] `NetherSpear.setupItemStack(2)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/NetherSpear.java:21)

### `NetherSword`: it extends Minecraft's `Item`

- [ ] `NetherSword.NetherSword(2)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/NetherSword.java:16)
- [ ] `NetherSword.setupItemStack(2)`: its class isn't translated (it extends Minecraft's `Item`) (org/betterx/betternether/items/NetherSword.java:21)

### `LegacySpawnerFix`

- [x] `LegacySpawnerFix.ARMOR_SLOTS` (org/betterx/betternether/LegacySpawnerFix.java:14)
- [x] `LegacySpawnerFix.HAND_SLOTS` (org/betterx/betternether/LegacySpawnerFix.java:15)
- [ ] `LegacySpawnerFix.VANILLA_EFFECT_IDS`: it calls `Map` (java.util).entry (org/betterx/betternether/LegacySpawnerFix.java:16)
- [x] `LegacySpawnerFix.LegacySpawnerFix(0)` (org/betterx/betternether/LegacySpawnerFix.java:46)
- [ ] `LegacySpawnerFix.patchChunk(1)`: it uses Minecraft's `CompoundTag` (org/betterx/betternether/LegacySpawnerFix.java:49)
- [ ] `LegacySpawnerFix.patchBlockEntities(2)`: it uses Minecraft's `ListTag` (org/betterx/betternether/LegacySpawnerFix.java:56)
- [ ] `LegacySpawnerFix.patchSpawner(1)`: it uses Minecraft's `CompoundTag` (org/betterx/betternether/LegacySpawnerFix.java:68)
- [ ] `LegacySpawnerFix.spawnEntityHolders(1)`: it uses Minecraft's `CompoundTag` (org/betterx/betternether/LegacySpawnerFix.java:106)
- [ ] `LegacySpawnerFix.patchEntity(1)`: it uses Minecraft's `CompoundTag` (org/betterx/betternether/LegacySpawnerFix.java:117)
- [ ] `LegacySpawnerFix.moveEquipment(4)`: it uses Minecraft's `CompoundTag` (org/betterx/betternether/LegacySpawnerFix.java:149)
- [ ] `LegacySpawnerFix.moveDropChances(5)`: it uses Minecraft's `CompoundTag` (org/betterx/betternether/LegacySpawnerFix.java:166)
- [ ] `LegacySpawnerFix.convertItem(1)`: it uses Minecraft's `CompoundTag` (org/betterx/betternether/LegacySpawnerFix.java:183)
- [ ] `LegacySpawnerFix.convertEffect(1)`: it uses Minecraft's `CompoundTag` (org/betterx/betternether/LegacySpawnerFix.java:206)

### `BNLoot`

- [ ] `BNLoot.CITY_LOOT`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/loot/BNLoot.java:9)
- [ ] `BNLoot.CITY_LOOT_COMMON`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/loot/BNLoot.java:10)
- [ ] `BNLoot.CITY_LOOT_SURPRISE`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/loot/BNLoot.java:11)
- [ ] `BNLoot.LIBRARY_LOOT`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/loot/BNLoot.java:12)
- [ ] `BNLoot.WITHER_TOWER_LOOT`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/loot/BNLoot.java:13)
- [ ] `BNLoot.WITHER_TOWER_BONUS_LOOT`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/loot/BNLoot.java:14)
- [ ] `BNLoot.GHAST_HIVE`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/loot/BNLoot.java:17)
- [ ] `BNLoot.FIREFLY`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/loot/BNLoot.java:18)
- [ ] `BNLoot.FLYING_PIG`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/loot/BNLoot.java:19)
- [ ] `BNLoot.JUNGLE_SKELETON`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/loot/BNLoot.java:20)
- [ ] `BNLoot.NAGA`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/loot/BNLoot.java:21)
- [ ] `BNLoot.SKULL`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/loot/BNLoot.java:22)

### `MHelper`

- [x] `MHelper.PI2` (org/betterx/betternether/MHelper.java:8)
- [x] `MHelper.ALPHA` (org/betterx/betternether/MHelper.java:9)
- [ ] `MHelper.RANDOM`: it creates a ThreadLocalRandomSource, which isn't translated (it implements Minecraft's `RandomSource`) (org/betterx/betternether/MHelper.java:11)
- [x] `MHelper.color(3)` (org/betterx/betternether/MHelper.java:13)
- [ ] `MHelper.randRange(3)`: it's overloaded with the same number of parameters (org/betterx/betternether/MHelper.java:17)
- [ ] `MHelper.randRange(3)`: it's overloaded with the same number of parameters (org/betterx/betternether/MHelper.java:21)
- [x] `MHelper.setBit(3)` (org/betterx/betternether/MHelper.java:25)
- [x] `MHelper.setBitTrue(2)` (org/betterx/betternether/MHelper.java:29)
- [x] `MHelper.setBitFalse(2)` (org/betterx/betternether/MHelper.java:33)
- [x] `MHelper.getBit(2)` (org/betterx/betternether/MHelper.java:37)
- [ ] `MHelper.floor(1)`: it's overloaded with the same number of parameters (org/betterx/betternether/MHelper.java:41)
- [ ] `MHelper.wrap(2)`: it calls MHelper.floor, which isn't translated (org/betterx/betternether/MHelper.java:45)
- [ ] `MHelper.floor(1)`: it's overloaded with the same number of parameters (org/betterx/betternether/MHelper.java:49)
- [x] `MHelper.nextFloat(2)` (org/betterx/betternether/MHelper.java:53)
- [x] `MHelper.nextDouble(2)` (org/betterx/betternether/MHelper.java:57)

### `MHelper.ThreadLocalRandomSource`: it implements Minecraft's `RandomSource`

- [ ] `MHelper.ThreadLocalRandomSource.fork(0)`: its class isn't translated (it implements Minecraft's `RandomSource`) (org/betterx/betternether/MHelper.java:62)
- [ ] `MHelper.ThreadLocalRandomSource.forkPositional(0)`: its class isn't translated (it implements Minecraft's `RandomSource`) (org/betterx/betternether/MHelper.java:66)
- [ ] `MHelper.ThreadLocalRandomSource.setSeed(1)`: its class isn't translated (it implements Minecraft's `RandomSource`) (org/betterx/betternether/MHelper.java:70)
- [ ] `MHelper.ThreadLocalRandomSource.nextInt(0)`: its class isn't translated (it implements Minecraft's `RandomSource`) (org/betterx/betternether/MHelper.java:74)
- [ ] `MHelper.ThreadLocalRandomSource.nextInt(1)`: its class isn't translated (it implements Minecraft's `RandomSource`) (org/betterx/betternether/MHelper.java:78)
- [ ] `MHelper.ThreadLocalRandomSource.nextLong(0)`: its class isn't translated (it implements Minecraft's `RandomSource`) (org/betterx/betternether/MHelper.java:82)
- [ ] `MHelper.ThreadLocalRandomSource.nextBoolean(0)`: its class isn't translated (it implements Minecraft's `RandomSource`) (org/betterx/betternether/MHelper.java:86)
- [ ] `MHelper.ThreadLocalRandomSource.nextFloat(0)`: its class isn't translated (it implements Minecraft's `RandomSource`) (org/betterx/betternether/MHelper.java:90)
- [ ] `MHelper.ThreadLocalRandomSource.nextDouble(0)`: its class isn't translated (it implements Minecraft's `RandomSource`) (org/betterx/betternether/MHelper.java:94)
- [ ] `MHelper.ThreadLocalRandomSource.nextGaussian(0)`: its class isn't translated (it implements Minecraft's `RandomSource`) (org/betterx/betternether/MHelper.java:98)
- [ ] `LavaFluidMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:18)
- [ ] `MinecraftClientMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:12)
- [ ] `PlayerArmorMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:16)
- [ ] `StandArmorMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:23)
- [ ] `TexturedModelDataMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:9)
- [ ] `AbstractFurnaceBlockEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:22)
- [ ] `BlockAgeProcessorMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:20)
- [ ] `BlockBehaviourAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:9)
- [ ] `BlockBehaviourMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:19)
- [ ] `BlockBehaviourPropertiesAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:9)
- [ ] `CopperGolemAiMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:12)
- [ ] `CraftingScreenHandlerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:15)
- [ ] `DefaultAttributeRegistryMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:14)
- [ ] `DeltaFeatureMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:16)
- [ ] `FlatLevelGeneratorPresetsMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:18)
- [ ] `LightningRodBlockMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:19)
- [ ] `LiquidBlockMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:25)
- [ ] `MapMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:32)
- [ ] `MappedRegistryMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:12)
- [ ] `MushroomMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:22)
- [ ] `NetherWartMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:21)
- [ ] `PlayerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:16)
- [ ] `PiglinAiMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:20)
- [ ] `PiglinBruteMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:18)
- [ ] `PiglinMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:19)
- [ ] `PlayerEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:22)
- [ ] `PortalShapeMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (org/betterx/betternether/MHelper.java:18)

### `OpenSimplexNoise`

- [x] `OpenSimplexNoise.STRETCH_CONSTANT_2D` (org/betterx/betternether/noise/OpenSimplexNoise.java:4)
- [x] `OpenSimplexNoise.SQUISH_CONSTANT_2D` (org/betterx/betternether/noise/OpenSimplexNoise.java:5)
- [x] `OpenSimplexNoise.STRETCH_CONSTANT_3D` (org/betterx/betternether/noise/OpenSimplexNoise.java:6)
- [x] `OpenSimplexNoise.SQUISH_CONSTANT_3D` (org/betterx/betternether/noise/OpenSimplexNoise.java:7)
- [x] `OpenSimplexNoise.STRETCH_CONSTANT_4D` (org/betterx/betternether/noise/OpenSimplexNoise.java:8)
- [x] `OpenSimplexNoise.SQUISH_CONSTANT_4D` (org/betterx/betternether/noise/OpenSimplexNoise.java:9)
- [x] `OpenSimplexNoise.NORM_CONSTANT_2D` (org/betterx/betternether/noise/OpenSimplexNoise.java:10)
- [x] `OpenSimplexNoise.NORM_CONSTANT_3D` (org/betterx/betternether/noise/OpenSimplexNoise.java:11)
- [x] `OpenSimplexNoise.NORM_CONSTANT_4D` (org/betterx/betternether/noise/OpenSimplexNoise.java:12)
- [x] `OpenSimplexNoise.DEFAULT_SEED` (org/betterx/betternether/noise/OpenSimplexNoise.java:13)
- [x] `OpenSimplexNoise.gradients2D` (org/betterx/betternether/noise/OpenSimplexNoise.java:16)
- [x] `OpenSimplexNoise.gradients3D` (org/betterx/betternether/noise/OpenSimplexNoise.java:17)
- [x] `OpenSimplexNoise.gradients4D` (org/betterx/betternether/noise/OpenSimplexNoise.java:91)
- [ ] `OpenSimplexNoise.OpenSimplexNoise(0)`: its constructors are overloaded with the same number of parameters (org/betterx/betternether/noise/OpenSimplexNoise.java:350)
- [ ] `OpenSimplexNoise.OpenSimplexNoise(1)`: its constructors are overloaded with the same number of parameters (org/betterx/betternether/noise/OpenSimplexNoise.java:354)
- [ ] `OpenSimplexNoise.OpenSimplexNoise(1)`: its constructors are overloaded with the same number of parameters (org/betterx/betternether/noise/OpenSimplexNoise.java:363)
- [x] `OpenSimplexNoise.eval(2)` (org/betterx/betternether/noise/OpenSimplexNoise.java:390)
- [x] `OpenSimplexNoise.eval(3)` (org/betterx/betternether/noise/OpenSimplexNoise.java:483)
- [x] `OpenSimplexNoise.eval(4)` (org/betterx/betternether/noise/OpenSimplexNoise.java:996)
- [x] `OpenSimplexNoise.extrapolate(4)` (org/betterx/betternether/noise/OpenSimplexNoise.java:2274)
- [x] `OpenSimplexNoise.extrapolate(6)` (org/betterx/betternether/noise/OpenSimplexNoise.java:2279)
- [x] `OpenSimplexNoise.extrapolate(8)` (org/betterx/betternether/noise/OpenSimplexNoise.java:2284)
- [x] `OpenSimplexNoise.fastFloor(1)` (org/betterx/betternether/noise/OpenSimplexNoise.java:2289)

### `BNDripHangParticle`: it extends Minecraft's `DripHangParticle`

- [ ] `BNDripHangParticle.BNDripHangParticle(7)`: its class isn't translated (it extends Minecraft's `DripHangParticle`) (org/betterx/betternether/particles/BNDripHangParticle.java:10)
- [ ] `BNDripHangParticle.setup(3)`: its class isn't translated (it extends Minecraft's `DripHangParticle`) (org/betterx/betternether/particles/BNDripHangParticle.java:14)

### `BNDripLandParticle`: it extends Minecraft's `DripLandParticle`

- [ ] `BNDripLandParticle.BNDripLandParticle(6)`: its class isn't translated (it extends Minecraft's `DripLandParticle`) (org/betterx/betternether/particles/BNDripLandParticle.java:9)
- [ ] `BNDripLandParticle.setup(2)`: its class isn't translated (it extends Minecraft's `DripLandParticle`) (org/betterx/betternether/particles/BNDripLandParticle.java:13)

### `BNFallAndLandParticle`: it extends Minecraft's `FallAndLandParticle`

- [ ] `BNFallAndLandParticle.BNFallAndLandParticle(7)`: its class isn't translated (it extends Minecraft's `FallAndLandParticle`) (org/betterx/betternether/particles/BNFallAndLandParticle.java:10)
- [ ] `BNFallAndLandParticle.setup(2)`: its class isn't translated (it extends Minecraft's `FallAndLandParticle`) (org/betterx/betternether/particles/BNFallAndLandParticle.java:14)

### `BNParticleProvider`: it's generic

- [x] `BNParticleProvider.BLUE_DRIP_R` (org/betterx/betternether/particles/BNParticleProvider.java:16)
- [x] `BNParticleProvider.BLUE_DRIP_G` (org/betterx/betternether/particles/BNParticleProvider.java:17)
- [x] `BNParticleProvider.BLUE_DRIP_B` (org/betterx/betternether/particles/BNParticleProvider.java:18)
- [x] `BNParticleProvider.DRIP_R` (org/betterx/betternether/particles/BNParticleProvider.java:19)
- [x] `BNParticleProvider.DRIP_G` (org/betterx/betternether/particles/BNParticleProvider.java:20)
- [x] `BNParticleProvider.DRIP_B` (org/betterx/betternether/particles/BNParticleProvider.java:21)

### `BNParticleProvider.ObsidianTearFallProvider`: it implements Minecraft's `ParticleProvider`

- [ ] `BNParticleProvider.ObsidianTearFallProvider.ObsidianTearFallProvider(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:27)
- [ ] `BNParticleProvider.ObsidianTearFallProvider.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:31)

### `BNParticleProvider.ObsidianTearHangProvider`: it implements Minecraft's `ParticleProvider`

- [ ] `BNParticleProvider.ObsidianTearHangProvider.ObsidianTearHangProvider(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:47)
- [ ] `BNParticleProvider.ObsidianTearHangProvider.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:51)

### `BNParticleProvider.ObsidianTearLandProvider`: it implements Minecraft's `ParticleProvider`

- [ ] `BNParticleProvider.ObsidianTearLandProvider.ObsidianTearLandProvider(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:67)
- [ ] `BNParticleProvider.ObsidianTearLandProvider.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:71)

### `BNParticleProvider.ObsidianVanillaWeepFallProvider`: it implements Minecraft's `ParticleProvider`

- [ ] `BNParticleProvider.ObsidianVanillaWeepFallProvider.ObsidianVanillaWeepFallProvider(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:85)
- [ ] `BNParticleProvider.ObsidianVanillaWeepFallProvider.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:89)

### `BNParticleProvider.ObsidianVanillaWeepHangProvider`: it implements Minecraft's `ParticleProvider`

- [ ] `BNParticleProvider.ObsidianVanillaWeepHangProvider.ObsidianVanillaWeepHangProvider(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:105)
- [ ] `BNParticleProvider.ObsidianVanillaWeepHangProvider.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:109)

### `BNParticleProvider.ObsidianVanillaWeepLandProvider`: it implements Minecraft's `ParticleProvider`

- [ ] `BNParticleProvider.ObsidianVanillaWeepLandProvider.ObsidianVanillaWeepLandProvider(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:125)
- [ ] `BNParticleProvider.ObsidianVanillaWeepLandProvider.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:129)

### `BNParticleProvider.ObsidianWeepFallProvider`: it implements Minecraft's `ParticleProvider`

- [ ] `BNParticleProvider.ObsidianWeepFallProvider.ObsidianWeepFallProvider(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:143)
- [ ] `BNParticleProvider.ObsidianWeepFallProvider.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:147)

### `BNParticleProvider.ObsidianWeepHangProvider`: it implements Minecraft's `ParticleProvider`

- [ ] `BNParticleProvider.ObsidianWeepHangProvider.ObsidianWeepHangProvider(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:163)
- [ ] `BNParticleProvider.ObsidianWeepHangProvider.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:167)

### `BNParticleProvider.ObsidianWeepLandProvider`: it implements Minecraft's `ParticleProvider`

- [ ] `BNParticleProvider.ObsidianWeepLandProvider.ObsidianWeepLandProvider(1)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:183)
- [ ] `BNParticleProvider.ObsidianWeepLandProvider.createParticle(9)`: its class isn't translated (it implements Minecraft's `ParticleProvider`) (org/betterx/betternether/particles/BNParticleProvider.java:187)

### `Patcher`

- [ ] `Patcher.register(0)`: a method reference (org/betterx/betternether/Patcher.java:6)

### `Patcher_001`: it extends `Patch` (org.betterx.bclib.api.v2.datafixer)

- [x] `Patcher_001.LEAVE_IDS` (org/betterx/betternether/Patcher_001.java:12)
- [ ] `Patcher_001.Patcher_001(0)`: its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)) (org/betterx/betternether/Patcher_001.java:16)
- [ ] `Patcher_001.getIDReplacements(0)`: its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)) (org/betterx/betternether/Patcher_001.java:20)
- [ ] `Patcher_001.getChunkPatcher(0)`: its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)) (org/betterx/betternether/Patcher_001.java:178)
- [ ] `Patcher_001.patchChunk(2)`: it uses Minecraft's `CompoundTag` (org/betterx/betternether/Patcher_001.java:182)

### `Patcher_002`: it extends `Patch` (org.betterx.bclib.api.v2.datafixer)

- [ ] `Patcher_002.Patcher_002(0)`: its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)) (org/betterx/betternether/Patcher_002.java:8)
- [ ] `Patcher_002.getIDReplacements(0)`: its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)) (org/betterx/betternether/Patcher_002.java:12)

### `Patcher_003`: it extends `Patch` (org.betterx.bclib.api.v2.datafixer)

- [ ] `Patcher_003.Patcher_003(0)`: its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)) (org/betterx/betternether/Patcher_003.java:8)
- [ ] `Patcher_003.getIDReplacements(0)`: its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)) (org/betterx/betternether/Patcher_003.java:12)

### `Patcher_004`: it extends `Patch` (org.betterx.bclib.api.v2.datafixer)

- [ ] `Patcher_004.Patcher_004(0)`: its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)) (org/betterx/betternether/Patcher_004.java:12)
- [ ] `Patcher_004.getIDReplacements(0)`: its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)) (org/betterx/betternether/Patcher_004.java:16)
- [ ] `Patcher_004.getBlockStatePatcher(0)`: its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)) (org/betterx/betternether/Patcher_004.java:38)
- [ ] `Patcher_004.patchBlockState(3)`: it uses Minecraft's `ListTag` (org/betterx/betternether/Patcher_004.java:42)

### `Patcher_005`: it extends `Patch` (org.betterx.bclib.api.v2.datafixer)

- [ ] `Patcher_005.Patcher_005(0)`: its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)) (org/betterx/betternether/Patcher_005.java:9)
- [ ] `Patcher_005.getChunkPatcher(0)`: its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)) (org/betterx/betternether/Patcher_005.java:13)

### `BNPortalShape`: it uses Minecraft's `BlockGetter`

- [ ] `BNPortalShape.DIR_X`: it uses Minecraft's `Direction` (org/betterx/betternether/portals/BNPortalShape.java:19)
- [ ] `BNPortalShape.DIR_Z`: it uses Minecraft's `Direction` (org/betterx/betternether/portals/BNPortalShape.java:20)
- [x] `BNPortalShape.MAX_SEARCH_DIST` (org/betterx/betternether/portals/BNPortalShape.java:21)
- [ ] `BNPortalShape.stack`: it uses Minecraft's `BlockPos` (org/betterx/betternether/portals/BNPortalShape.java:30)
- [ ] `BNPortalShape.BNPortalShape(3)`: its class isn't translated (it uses Minecraft's `BlockGetter`) (org/betterx/betternether/portals/BNPortalShape.java:32)
- [ ] `BNPortalShape.getPortalBlocks(2)`: its class isn't translated (it uses Minecraft's `BlockGetter`) (org/betterx/betternether/portals/BNPortalShape.java:43)
- [ ] `BNPortalShape.calculateBottomLeft(1)`: its class isn't translated (it uses Minecraft's `BlockGetter`) (org/betterx/betternether/portals/BNPortalShape.java:107)
- [ ] `BNPortalShape.calculateWidth(0)`: its class isn't translated (it uses Minecraft's `BlockGetter`) (org/betterx/betternether/portals/BNPortalShape.java:111)
- [ ] `BNPortalShape.calculateHeight(0)`: its class isn't translated (it uses Minecraft's `BlockGetter`) (org/betterx/betternether/portals/BNPortalShape.java:121)
- [ ] `BNPortalShape.isFrame(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/portals/BNPortalShape.java:131)
- [ ] `BNPortalShape.isEmpty(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/portals/BNPortalShape.java:135)
- [ ] `BNPortalShape.isValid(0)`: its class isn't translated (it uses Minecraft's `BlockGetter`) (org/betterx/betternether/portals/BNPortalShape.java:139)
- [ ] `BNPortalShape.createPortalBlocks(1)`: its class isn't translated (it uses Minecraft's `BlockGetter`) (org/betterx/betternether/portals/BNPortalShape.java:143)
- [ ] `BNPortalShape.isComplete(0)`: its class isn't translated (it uses Minecraft's `BlockGetter`) (org/betterx/betternether/portals/BNPortalShape.java:148)

### `BNPortalShapeProvider`: it's an interface

- [ ] `BNPortalShapeProvider.bn_setShape(1)`: it has no body (org/betterx/betternether/portals/BNPortalShapeProvider.java:4)
- [ ] `BNPortalShapeProvider.bn_getShape(0)`: it has no body (org/betterx/betternether/portals/BNPortalShapeProvider.java:6)

### `RecipesHelper`

- [x] `RecipesHelper.SHAPE_ROOF` (org/betterx/betternether/recipes/RecipesHelper.java:13)
- [x] `RecipesHelper.SHAPE_STAIR` (org/betterx/betternether/recipes/RecipesHelper.java:14)
- [x] `RecipesHelper.SHAPE_SLAB` (org/betterx/betternether/recipes/RecipesHelper.java:15)
- [x] `RecipesHelper.SHAPE_BUTTON` (org/betterx/betternether/recipes/RecipesHelper.java:16)
- [x] `RecipesHelper.SHAPE_PLATE` (org/betterx/betternether/recipes/RecipesHelper.java:17)
- [x] `RecipesHelper.SHAPE_X2` (org/betterx/betternether/recipes/RecipesHelper.java:18)
- [x] `RecipesHelper.SHAPE_3X2` (org/betterx/betternether/recipes/RecipesHelper.java:19)
- [x] `RecipesHelper.SHAPE_COLORING` (org/betterx/betternether/recipes/RecipesHelper.java:20)
- [x] `RecipesHelper.SHAPE_ROUND` (org/betterx/betternether/recipes/RecipesHelper.java:21)
- [x] `RecipesHelper.SHAPE_FIRE_BOWL` (org/betterx/betternether/recipes/RecipesHelper.java:22)
- [ ] `RecipesHelper.RECIPES`: it uses RecipeBuilder, which isn't translated (it's an interface) (org/betterx/betternether/recipes/RecipesHelper.java:23)
- [ ] `RecipesHelper.provideRecipes(1)`: it uses `Templates` (de.ambertation.wover.recipe.api.RecipeBuilder) (org/betterx/betternether/recipes/RecipesHelper.java:25)
- [ ] `RecipesHelper.addProvider(1)`: it uses RecipeBuilder, which isn't translated (it's an interface) (org/betterx/betternether/recipes/RecipesHelper.java:30)
- [ ] `RecipesHelper.makeRoofRecipe(2)`: it uses Minecraft's `Block` (org/betterx/betternether/recipes/RecipesHelper.java:34)
- [ ] `RecipesHelper.makeStairsRecipe(2)`: it uses Minecraft's `Block` (org/betterx/betternether/recipes/RecipesHelper.java:38)
- [ ] `RecipesHelper.makeSlabRecipe(2)`: it uses Minecraft's `Block` (org/betterx/betternether/recipes/RecipesHelper.java:42)
- [ ] `RecipesHelper.makeButtonRecipe(2)`: it uses Minecraft's `Block` (org/betterx/betternether/recipes/RecipesHelper.java:46)
- [ ] `RecipesHelper.makePlateRecipe(2)`: it uses Minecraft's `Block` (org/betterx/betternether/recipes/RecipesHelper.java:50)
- [ ] `RecipesHelper.makeSimpleRecipe2(5)`: it uses Minecraft's `Block` (org/betterx/betternether/recipes/RecipesHelper.java:54)
- [ ] `RecipesHelper.makeWallRecipe(2)`: it uses Minecraft's `Block` (org/betterx/betternether/recipes/RecipesHelper.java:58)
- [ ] `RecipesHelper.makeColoringRecipe(5)`: it uses Minecraft's `Block` (org/betterx/betternether/recipes/RecipesHelper.java:62)
- [ ] `RecipesHelper.makeRoundRecipe(4)`: it uses Minecraft's `Block` (org/betterx/betternether/recipes/RecipesHelper.java:66)
- [ ] `RecipesHelper.makeFireBowlRecipe(4)`: it uses Minecraft's `Block` (org/betterx/betternether/recipes/RecipesHelper.java:70)

### `RecipesHelper.RecipeBuilder`: it's an interface


### `BlockEntitiesRegistry`

- [ ] `BlockEntitiesRegistry.CINCINNASITE_FORGE`: it uses Minecraft's `BlockEntityType` (org/betterx/betternether/registry/BlockEntitiesRegistry.java:21)
- [ ] `BlockEntitiesRegistry.NETHERRACK_FURNACE`: it uses Minecraft's `BlockEntityType` (org/betterx/betternether/registry/BlockEntitiesRegistry.java:25)
- [ ] `BlockEntitiesRegistry.CHEST_OF_DRAWERS`: it uses Minecraft's `BlockEntityType` (org/betterx/betternether/registry/BlockEntitiesRegistry.java:27)
- [ ] `BlockEntitiesRegistry.NETHER_BREWING_STAND`: it uses Minecraft's `BlockEntityType` (org/betterx/betternether/registry/BlockEntitiesRegistry.java:31)
- [ ] `BlockEntitiesRegistry.register(0)`: it calls BlockEntitiesRegistry.RegisterBlockEntity, which isn't translated (org/betterx/betternether/registry/BlockEntitiesRegistry.java:36)
- [ ] `BlockEntitiesRegistry.RegisterBlockEntity(2)`: it uses Minecraft's `BlockEntityType` (org/betterx/betternether/registry/BlockEntitiesRegistry.java:43)
- [ ] `BlockEntitiesRegistry.getFurnaces(0)`: it uses Minecraft's `Block` (org/betterx/betternether/registry/BlockEntitiesRegistry.java:47)

### `NetherBlockItemFactories`

- [ ] `NetherBlockItemFactories.STACKS_TO_16`: it uses `BlockItemFactory` (de.ambertation.wover.block.api.BlockDefinition) (org/betterx/betternether/registry/block/NetherBlockItemFactories.java:9)
- [x] `NetherBlockItemFactories.NetherBlockItemFactories(0)` (org/betterx/betternether/registry/block/NetherBlockItemFactories.java:13)
- [x] `NetherBlockItemFactories.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherBlockItemFactories.java:16)

### `NetherCropBlocks`

- [ ] `NetherCropBlocks.BLACK_APPLE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherCropBlocks.java:10)
- [ ] `NetherCropBlocks.BLACK_APPLE_SEED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherCropBlocks.java:11)
- [ ] `NetherCropBlocks.WHISPERING_GOURD`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherCropBlocks.java:12)
- [x] `NetherCropBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherCropBlocks.java:14)

### `NetherDecorBlocks`

- [ ] `NetherDecorBlocks.CINCINNASITE_FRAME`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherDecorBlocks.java:13)
- [ ] `NetherDecorBlocks.CINCINNASITE_POT`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherDecorBlocks.java:14)
- [ ] `NetherDecorBlocks.BRICK_POT`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherDecorBlocks.java:15)
- [ ] `NetherDecorBlocks.GEYSER`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherDecorBlocks.java:16)
- [ ] `NetherDecorBlocks.ROOF_TILE_NETHER_BRICKS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherDecorBlocks.java:17)
- [ ] `NetherDecorBlocks.ROOF_TILE_NETHER_BRICKS_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherDecorBlocks.java:18)
- [ ] `NetherDecorBlocks.ROOF_TILE_NETHER_BRICKS_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherDecorBlocks.java:19)
- [ ] `NetherDecorBlocks.ROOF_TILE_CINCINNASITE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherDecorBlocks.java:20)
- [ ] `NetherDecorBlocks.ROOF_TILE_CINCINNASITE_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherDecorBlocks.java:21)
- [ ] `NetherDecorBlocks.ROOF_TILE_CINCINNASITE_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherDecorBlocks.java:22)
- [x] `NetherDecorBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherDecorBlocks.java:24)

### `NetherFactories`

- [ ] `NetherFactories.AGAVE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:121)
- [ ] `NetherFactories.ANCHOR_TREE_SAPLING`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:122)
- [ ] `NetherFactories.ANCHOR_TREE_VINE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:123)
- [ ] `NetherFactories.BARREL_CACTUS`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:124)
- [ ] `NetherFactories.BLACK_APPLE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:125)
- [ ] `NetherFactories.BLACK_APPLE_SEED`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:126)
- [ ] `NetherFactories.BLACK_BUSH`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:127)
- [ ] `NetherFactories.BLACK_VINE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:128)
- [ ] `NetherFactories.BLUE_CRYING_OBSIDIAN`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:129)
- [ ] `NetherFactories.BLUE_WEEPING_OBSIDIAN`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:130)
- [ ] `NetherFactories.BN_BREWING_STAND`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:131)
- [ ] `NetherFactories.BN_PANE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:132)
- [ ] `NetherFactories.BN_PILLAR`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:133)
- [ ] `NetherFactories.BN_POT_METAL`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:134)
- [ ] `NetherFactories.BN_POT_STONE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:135)
- [ ] `NetherFactories.BONE_MUSHROOM`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:136)
- [ ] `NetherFactories.BROWN_LARGE_MUSHROOM`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:137)
- [ ] `NetherFactories.CHEST_OF_DRAWERS`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:138)
- [ ] `NetherFactories.CINCINNASITE_ANVIL`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:139)
- [ ] `NetherFactories.CINCINNASITE_FORGE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:140)
- [ ] `NetherFactories.CINCINNASITE_FRAME`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:141)
- [ ] `NetherFactories.CINCINNASITE_PEDESTAL`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:142)
- [ ] `NetherFactories.CINCINNASIT_PILLAR`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:143)
- [ ] `NetherFactories.EGG_PLANT`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:144)
- [ ] `NetherFactories.EYEBALL`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:145)
- [ ] `NetherFactories.EYEBALL_SMALL`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:146)
- [ ] `NetherFactories.EYE_SEED`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:147)
- [ ] `NetherFactories.EYE_VINE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:148)
- [ ] `NetherFactories.FARMLAND`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:149)
- [ ] `NetherFactories.FEATHER_FERN`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:150)
- [ ] `NetherFactories.GEYSER`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:151)
- [ ] `NetherFactories.GIANT_LUCIS`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:152)
- [ ] `NetherFactories.GIANT_MOLD`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:153)
- [ ] `NetherFactories.GIANT_MOLD_SAPLING`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:154)
- [ ] `NetherFactories.GLOOMSCULK_CRYSTAL`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:155)
- [ ] `NetherFactories.GLOOMSCULK_GEODE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:156)
- [ ] `NetherFactories.GLOOMWISP_VINE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:157)
- [ ] `NetherFactories.GLOOMWOOD_SAPLING`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:158)
- [ ] `NetherFactories.GOLDEN_VINE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:159)
- [ ] `NetherFactories.GRAY_MOLD`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:160)
- [ ] `NetherFactories.HOOK_MUSHROOM`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:161)
- [ ] `NetherFactories.INK_BUSH`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:162)
- [ ] `NetherFactories.INK_BUSH_SEED`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:163)
- [ ] `NetherFactories.JELLYFISH_MUSHROOM`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:164)
- [ ] `NetherFactories.JELLYFISH_MUSHROOM_SAPLING`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:165)
- [ ] `NetherFactories.LUCIS_MUSHROOM`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:168)
- [ ] `NetherFactories.LUCIS_SPORE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:169)
- [ ] `NetherFactories.LUMABUS_VINE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:170)
- [ ] `NetherFactories.GOLDEN_LUMABUS_VINE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:173)
- [ ] `NetherFactories.MAGMA_FLOWER`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:176)
- [ ] `NetherFactories.MOLTEN_GLOOMSCULK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:177)
- [ ] `NetherFactories.MOLTEN_GLOOMWOOD_LOG`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:178)
- [ ] `NetherFactories.MOSS_COVER`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:179)
- [ ] `NetherFactories.MUSHROOM_FIR`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:180)
- [ ] `NetherFactories.MUSHROOM_FIR_SAPLING`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:181)
- [ ] `NetherFactories.NEON_EQUISETUM`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:182)
- [ ] `NetherFactories.NETHER_CACTUS`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:183)
- [ ] `NetherFactories.NETHER_FURNACE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:184)
- [ ] `NetherFactories.NETHER_GRASS_BONE_GRASS`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:185)
- [ ] `NetherFactories.NETHER_GRASS_JUNGLE_PLANT`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:188)
- [ ] `NetherFactories.NETHER_GRASS_NETHER_GRASS`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:191)
- [ ] `NetherFactories.NETHER_GRASS_SEPIA_BONE_GRASS`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:194)
- [ ] `NetherFactories.NETHER_GRASS_SWAMP_GRASS`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:197)
- [ ] `NetherFactories.NETHER_MYCELIUM`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:200)
- [ ] `NetherFactories.NETHER_REED`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:201)
- [ ] `NetherFactories.NETHER_SAKURA_SAPLING`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:202)
- [ ] `NetherFactories.OBSIDIAN_GLASS`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:203)
- [ ] `NetherFactories.ORANGE_MUSHROOM`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:204)
- [ ] `NetherFactories.POTTED_PLANT`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:205)
- [ ] `NetherFactories.REDSTONE_ORE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:206)
- [ ] `NetherFactories.RED_LARGE_MUSHROOM`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:207)
- [ ] `NetherFactories.RED_MOLD`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:208)
- [ ] `NetherFactories.REEDS`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:209)
- [ ] `NetherFactories.RUBEUS_BARK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:210)
- [ ] `NetherFactories.RUBEUS_CONE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:211)
- [ ] `NetherFactories.RUBEUS_LOG`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:212)
- [ ] `NetherFactories.RUBEUS_SAPLING`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:213)
- [ ] `NetherFactories.SMALL_LANTERN_METAL`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:214)
- [ ] `NetherFactories.SMOKER`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:215)
- [ ] `NetherFactories.SOUL_GRASS`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:216)
- [ ] `NetherFactories.SOUL_LILY`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:217)
- [ ] `NetherFactories.SOUL_LILY_SAPLING`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:218)
- [ ] `NetherFactories.SOUL_SANDSTONE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:219)
- [ ] `NetherFactories.SOUL_VEIN`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:220)
- [ ] `NetherFactories.STALACTITE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:221)
- [ ] `NetherFactories.STALAGNATE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:222)
- [ ] `NetherFactories.STALAGNATE_BOWL`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:223)
- [ ] `NetherFactories.STALAGNATE_SEED`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:224)
- [ ] `NetherFactories.STATUE_RESPAWNER`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:225)
- [ ] `NetherFactories.TERRAIN`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:226)
- [ ] `NetherFactories.VANILLA_WEEPING_OBSIDIAN`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:227)
- [ ] `NetherFactories.VEINED_SAND`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:230)
- [ ] `NetherFactories.WART_ROOTS`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:231)
- [ ] `NetherFactories.WART_SEED`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:232)
- [ ] `NetherFactories.WHISPERING_GOURD_LANTERN`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:233)
- [ ] `NetherFactories.WHISPERING_GOURD_VINE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:236)
- [ ] `NetherFactories.WILLOW_BRANCH`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:237)
- [ ] `NetherFactories.WILLOW_SAPLING`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:238)
- [ ] `NetherFactories.WILLOW_TORCH`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:239)
- [ ] `NetherFactories.WILLOW_TRUNK`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:240)
- [ ] `NetherFactories.BN_LEAVES`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:241)
- [ ] `NetherFactories.WILLOW_LEAVES`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:242)
- [ ] `NetherFactories.RUBEUS_LEAVES`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:243)
- [ ] `NetherFactories.NETHER_SAKURA_LEAVES`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:244)
- [ ] `NetherFactories.BN_OBSIDIAN`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:245)
- [ ] `NetherFactories.BN_OBSIDIAN_BLUE_CRYING`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:246)
- [ ] `NetherFactories.WALL_PLANT_SHAPE_2_6`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:249)
- [ ] `NetherFactories.CINCINNASITE_ORE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:252)
- [ ] `NetherFactories.NETHER_RUBY_ORE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:253)
- [ ] `NetherFactories.NETHER_LAPIS_ORE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:254)
- [ ] `NetherFactories.CRIMSON_BUTTON`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:255)
- [ ] `NetherFactories.IRON_BUTTON`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:258)
- [ ] `NetherFactories.BONE_PLATE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:259)
- [ ] `NetherFactories.CINCINNASITE_PLATE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:262)
- [ ] `NetherFactories.CRIMSON_WOODLIKE_DOOR`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:265)
- [ ] `NetherFactories.FIRE_BOWL_METAL`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:268)
- [ ] `NetherFactories.FIRE_BOWL_STONE`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:269)
- [ ] `NetherFactories.LUMABUS_SEED`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:270)
- [ ] `NetherFactories.GOLDEN_LUMABUS_SEED`: it uses `BlockFactory` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFactories.java:273)
- [x] `NetherFactories.NetherFactories(0)` (org/betterx/betternether/registry/block/NetherFactories.java:277)
- [x] `NetherFactories.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherFactories.java:280)

### `NetherFunctionalBlocks`

- [ ] `NetherFunctionalBlocks.CINCINNASITE_PEDESTAL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFunctionalBlocks.java:18)
- [ ] `NetherFunctionalBlocks.PIG_STATUE_RESPAWNER`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFunctionalBlocks.java:19)
- [ ] `NetherFunctionalBlocks.BLACKSTONE_FURNACE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFunctionalBlocks.java:20)
- [ ] `NetherFunctionalBlocks.BASALT_FURNACE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFunctionalBlocks.java:21)
- [ ] `NetherFunctionalBlocks.NETHERRACK_FURNACE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFunctionalBlocks.java:22)
- [ ] `NetherFunctionalBlocks.CINCINNASITE_FORGE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFunctionalBlocks.java:23)
- [ ] `NetherFunctionalBlocks.NETHER_BREWING_STAND`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFunctionalBlocks.java:24)
- [ ] `NetherFunctionalBlocks.CINCINNASITE_ANVIL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFunctionalBlocks.java:25)
- [ ] `NetherFunctionalBlocks.BLACKSTONE_FURNACE_STONE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFunctionalBlocks.java:26)
- [ ] `NetherFunctionalBlocks.BASALT_FURNACE_STONE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFunctionalBlocks.java:27)
- [ ] `NetherFunctionalBlocks.NETHERRACK_FURNACE_STONE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFunctionalBlocks.java:28)
- [ ] `NetherFunctionalBlocks.workstationStone(2)`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFunctionalBlocks.java:30)
- [x] `NetherFunctionalBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherFunctionalBlocks.java:34)

### `NetherFurnitureBlocks`

- [ ] `NetherFurnitureBlocks.TRIMMED_CHEST_MATERIAL`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFurnitureBlocks.java:14)
- [ ] `NetherFurnitureBlocks.TABURET_CINCINNASITE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFurnitureBlocks.java:19)
- [ ] `NetherFurnitureBlocks.CHAIR_CINCINNASITE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFurnitureBlocks.java:20)
- [ ] `NetherFurnitureBlocks.BAR_STOOL_CINCINNASITE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFurnitureBlocks.java:23)
- [ ] `NetherFurnitureBlocks.CHEST_OF_DRAWERS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFurnitureBlocks.java:26)
- [ ] `NetherFurnitureBlocks.TRIMMED_MUSHROOM_FIR_CHEST`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherFurnitureBlocks.java:27)
- [x] `NetherFurnitureBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherFurnitureBlocks.java:29)

### `NetherGlassBlocks`

- [ ] `NetherGlassBlocks.QUARTZ_GLASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherGlassBlocks.java:11)
- [ ] `NetherGlassBlocks.QUARTZ_GLASS_FRAMED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherGlassBlocks.java:12)
- [ ] `NetherGlassBlocks.QUARTZ_GLASS_FRAMED_COLORED`: it uses ColoredGlassMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherGlassBlocks.java:13)
- [ ] `NetherGlassBlocks.QUARTZ_GLASS_PANE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherGlassBlocks.java:16)
- [ ] `NetherGlassBlocks.QUARTZ_GLASS_PANE_COLORED`: it uses ColoredGlassMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherGlassBlocks.java:17)
- [ ] `NetherGlassBlocks.QUARTZ_GLASS_FRAMED_PANE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherGlassBlocks.java:20)
- [ ] `NetherGlassBlocks.QUARTZ_GLASS_FRAMED_PANE_COLORED`: it uses ColoredGlassMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherGlassBlocks.java:21)
- [ ] `NetherGlassBlocks.QUARTZ_GLASS_COLORED`: it uses ColoredGlassMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherGlassBlocks.java:24)
- [x] `NetherGlassBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherGlassBlocks.java:26)

### `NetherKinds`

- [ ] `NetherKinds.OWN_MODEL`: a raw Supplier (org/betterx/betternether/registry/block/NetherKinds.java:35)
- [ ] `NetherKinds.STEM`: it uses StemKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherKinds.java:36)
- [ ] `NetherKinds.TRUNK`: it uses TrunkKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherKinds.java:37)
- [ ] `NetherKinds.POTTED_PLANT`: it uses PottedPlantKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherKinds.java:38)
- [x] `NetherKinds.NetherKinds(0)` (org/betterx/betternether/registry/block/NetherKinds.java:40)
- [ ] `NetherKinds.model(3)`: it uses `ResolveContext` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherKinds.java:43)

### `NetherKinds.PottedPlantKind`: it extends `Kind` (de.ambertation.wover.block.api.classify)

- [ ] `NetherKinds.PottedPlantKind.PottedPlantKind(0)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherKinds.java:53)
- [ ] `NetherKinds.PottedPlantKind.PottedPlantKind(3)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherKinds.java:61)
- [ ] `NetherKinds.PottedPlantKind.resolve(1)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherKinds.java:75)

### `NetherKinds.StemKind`: it extends `Kind` (de.ambertation.wover.block.api.classify)

- [ ] `NetherKinds.StemKind.StemKind(0)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherKinds.java:96)
- [ ] `NetherKinds.StemKind.StemKind(6)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherKinds.java:107)
- [ ] `NetherKinds.StemKind.resolve(1)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherKinds.java:131)

### `NetherKinds.TrunkKind`: it extends `Kind` (de.ambertation.wover.block.api.classify)

- [ ] `NetherKinds.TrunkKind.TrunkKind(0)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherKinds.java:155)
- [ ] `NetherKinds.TrunkKind.TrunkKind(5)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherKinds.java:165)
- [ ] `NetherKinds.TrunkKind.hasItem(0)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherKinds.java:181)
- [ ] `NetherKinds.TrunkKind.resolve(1)`: its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/block/NetherKinds.java:185)

### `NetherLeavesBlocks`

- [ ] `NetherLeavesBlocks.WILLOW_LEAVES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLeavesBlocks.java:12)
- [ ] `NetherLeavesBlocks.RUBEUS_LEAVES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLeavesBlocks.java:13)
- [ ] `NetherLeavesBlocks.ANCHOR_TREE_LEAVES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLeavesBlocks.java:14)
- [ ] `NetherLeavesBlocks.NETHER_SAKURA_LEAVES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLeavesBlocks.java:15)
- [ ] `NetherLeavesBlocks.GLOOMWOOD_LEAVES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLeavesBlocks.java:16)
- [ ] `NetherLeavesBlocks.GLOOMWOOD_BLEACHED_LEAVES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLeavesBlocks.java:17)
- [x] `NetherLeavesBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherLeavesBlocks.java:21)

### `NetherLightBlocks`

- [ ] `NetherLightBlocks.NETHERITE_FIRE_BOWL_MATERIAL`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLightBlocks.java:18)
- [ ] `NetherLightBlocks.CINCINNASITE_LANTERN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLightBlocks.java:23)
- [ ] `NetherLightBlocks.CINCINNASITE_LANTERN_SMALL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLightBlocks.java:24)
- [ ] `NetherLightBlocks.WHISPERING_GOURD_LANTERN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLightBlocks.java:27)
- [ ] `NetherLightBlocks.CINCINNASITE_FIRE_BOWL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLightBlocks.java:30)
- [ ] `NetherLightBlocks.BRICKS_FIRE_BOWL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLightBlocks.java:31)
- [ ] `NetherLightBlocks.NETHERITE_FIRE_BOWL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLightBlocks.java:32)
- [ ] `NetherLightBlocks.CINCINNASITE_FIRE_BOWL_SOUL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLightBlocks.java:33)
- [ ] `NetherLightBlocks.BRICKS_FIRE_BOWL_SOUL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLightBlocks.java:34)
- [ ] `NetherLightBlocks.NETHERITE_FIRE_BOWL_SOUL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherLightBlocks.java:35)
- [x] `NetherLightBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherLightBlocks.java:37)

### `NetherMaterials`

- [ ] `NetherMaterials.WOOD`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:22)
- [ ] `NetherMaterials.PLANT`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:28)
- [ ] `NetherMaterials.GRASS`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:30)
- [ ] `NetherMaterials.GRAY_GRASS`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:35)
- [ ] `NetherMaterials.CROP_PLANT`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:38)
- [ ] `NetherMaterials.HOE_PLANT`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:40)
- [ ] `NetherMaterials.SEED`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:43)
- [ ] `NetherMaterials.CACTUS`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:45)
- [ ] `NetherMaterials.MUSHROOM`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:51)
- [ ] `NetherMaterials.VINE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:55)
- [ ] `NetherMaterials.STURDY_VINE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:56)
- [ ] `NetherMaterials.MUSHROOM_STEM`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:58)
- [ ] `NetherMaterials.GLOOMWOOD_TRANSITION`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:68)
- [ ] `NetherMaterials.GLOOMWOOD_TRANSITION_LOG`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:75)
- [ ] `NetherMaterials.VANILLA_WOOD`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:78)
- [ ] `NetherMaterials.VANILLA_NETHER_WOOD`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:83)
- [ ] `NetherMaterials.CINCINNASITE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:86)
- [ ] `NetherMaterials.CINCINNASITE_OPEN`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:93)
- [ ] `NetherMaterials.NETHER_TERRAIN`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:96)
- [ ] `NetherMaterials.NETHER_TERRAIN_COVER`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:102)
- [ ] `NetherMaterials.NETHER_NYLIUM`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:105)
- [ ] `NetherMaterials.PORTAL_OBSIDIAN`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:108)
- [ ] `NetherMaterials.NETHER_ORE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:112)
- [ ] `NetherMaterials.QUARTZ_GLASS`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:119)
- [ ] `NetherMaterials.FRAMED_QUARTZ_GLASS`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:123)
- [ ] `NetherMaterials.FRAMED_QUARTZ_GLASS_PANE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMaterials.java:131)
- [x] `NetherMaterials.NetherMaterials(0)` (org/betterx/betternether/registry/block/NetherMaterials.java:135)
- [x] `NetherMaterials.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherMaterials.java:138)

### `NetherMetalBlocks`

- [ ] `NetherMetalBlocks.NETHER_RUBY`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:23)
- [ ] `NetherMetalBlocks.CINCINNASITE_BLOCK`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:27)
- [ ] `NetherMetalBlocks.CINCINNASITE_FORGED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:28)
- [ ] `NetherMetalBlocks.CINCINNASITE_PILLAR`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:29)
- [ ] `NetherMetalBlocks.CINCINNASITE_BRICKS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:30)
- [ ] `NetherMetalBlocks.CINCINNASITE_BRICK_PLATE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:31)
- [ ] `NetherMetalBlocks.CINCINNASITE_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:32)
- [ ] `NetherMetalBlocks.CINCINNASITE_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:33)
- [ ] `NetherMetalBlocks.CINCINNASITE_BUTTON`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:34)
- [ ] `NetherMetalBlocks.CINCINNASITE_PLATE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:35)
- [ ] `NetherMetalBlocks.CINCINNASITE_TILE_LARGE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:36)
- [ ] `NetherMetalBlocks.CINCINNASITE_TILE_SMALL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:37)
- [ ] `NetherMetalBlocks.CINCINNASITE_CARVED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:38)
- [ ] `NetherMetalBlocks.CINCINNASITE_WALL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:39)
- [ ] `NetherMetalBlocks.CINCINNASITE_BRICKS_PILLAR`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:40)
- [ ] `NetherMetalBlocks.CINCINNASITE_BARS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:41)
- [ ] `NetherMetalBlocks.CINCINNASITE_CHAIN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:42)
- [ ] `NetherMetalBlocks.NETHER_RUBY_BLOCK`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:43)
- [ ] `NetherMetalBlocks.NETHER_RUBY_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:44)
- [ ] `NetherMetalBlocks.NETHER_RUBY_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMetalBlocks.java:45)
- [x] `NetherMetalBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherMetalBlocks.java:47)

### `NetherMushroomBlocks`

- [ ] `NetherMushroomBlocks.ORANGE_MUSHROOM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMushroomBlocks.java:20)
- [ ] `NetherMushroomBlocks.RED_MOLD`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMushroomBlocks.java:21)
- [ ] `NetherMushroomBlocks.GRAY_MOLD`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMushroomBlocks.java:22)
- [ ] `NetherMushroomBlocks.LUCIS_SPORE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMushroomBlocks.java:23)
- [ ] `NetherMushroomBlocks.GIANT_LUCIS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMushroomBlocks.java:24)
- [ ] `NetherMushroomBlocks.BONE_MUSHROOM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMushroomBlocks.java:25)
- [ ] `NetherMushroomBlocks.SMOKER`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMushroomBlocks.java:26)
- [ ] `NetherMushroomBlocks.HOOK_MUSHROOM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMushroomBlocks.java:27)
- [ ] `NetherMushroomBlocks.RED_LARGE_MUSHROOM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMushroomBlocks.java:28)
- [ ] `NetherMushroomBlocks.BROWN_LARGE_MUSHROOM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMushroomBlocks.java:29)
- [ ] `NetherMushroomBlocks.LUCIS_MUSHROOM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMushroomBlocks.java:30)
- [ ] `NetherMushroomBlocks.GIANT_MOLD`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMushroomBlocks.java:31)
- [ ] `NetherMushroomBlocks.JELLYFISH_MUSHROOM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherMushroomBlocks.java:32)
- [x] `NetherMushroomBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherMushroomBlocks.java:34)

### `NetherObsidianBlocks`

- [ ] `NetherObsidianBlocks.IMMOVEABLE`: it uses `Flag` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:29)
- [ ] `NetherObsidianBlocks.OBSIDIAN_GLASS_MATERIAL`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:30)
- [ ] `NetherObsidianBlocks.OBSIDIAN_GLASS_PANE_MATERIAL`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:35)
- [ ] `NetherObsidianBlocks.BLUE_WEEPING_OBSIDIAN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:42)
- [ ] `NetherObsidianBlocks.WEEPING_OBSIDIAN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:43)
- [ ] `NetherObsidianBlocks.BLUE_CRYING_OBSIDIAN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:44)
- [ ] `NetherObsidianBlocks.OBSIDIAN_BRICKS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:45)
- [ ] `NetherObsidianBlocks.OBSIDIAN_BRICKS_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:46)
- [ ] `NetherObsidianBlocks.OBSIDIAN_BRICKS_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:47)
- [ ] `NetherObsidianBlocks.OBSIDIAN_TILE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:48)
- [ ] `NetherObsidianBlocks.OBSIDIAN_TILE_SMALL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:49)
- [ ] `NetherObsidianBlocks.OBSIDIAN_TILE_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:50)
- [ ] `NetherObsidianBlocks.OBSIDIAN_TILE_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:51)
- [ ] `NetherObsidianBlocks.OBSIDIAN_ROD_TILES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:52)
- [ ] `NetherObsidianBlocks.OBSIDIAN_GLASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:53)
- [ ] `NetherObsidianBlocks.OBSIDIAN_GLASS_PANE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:54)
- [ ] `NetherObsidianBlocks.BLUE_OBSIDIAN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:55)
- [ ] `NetherObsidianBlocks.BLUE_OBSIDIAN_BRICKS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:56)
- [ ] `NetherObsidianBlocks.BLUE_OBSIDIAN_BRICKS_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:57)
- [ ] `NetherObsidianBlocks.BLUE_OBSIDIAN_BRICKS_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:58)
- [ ] `NetherObsidianBlocks.BLUE_OBSIDIAN_TILE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:59)
- [ ] `NetherObsidianBlocks.BLUE_OBSIDIAN_TILE_SMALL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:60)
- [ ] `NetherObsidianBlocks.BLUE_OBSIDIAN_TILE_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:61)
- [ ] `NetherObsidianBlocks.BLUE_OBSIDIAN_TILE_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:62)
- [ ] `NetherObsidianBlocks.BLUE_OBSIDIAN_ROD_TILES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:63)
- [ ] `NetherObsidianBlocks.BLUE_OBSIDIAN_GLASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:64)
- [ ] `NetherObsidianBlocks.BLUE_OBSIDIAN_GLASS_PANE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:65)
- [x] `NetherObsidianBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherObsidianBlocks.java:67)

### `NetherOreBlocks`

- [ ] `NetherOreBlocks.IRON_ORE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherOreBlocks.java:14)
- [ ] `NetherOreBlocks.DIAMOND_ORE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherOreBlocks.java:18)
- [ ] `NetherOreBlocks.CINCINNASITE_ORE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherOreBlocks.java:22)
- [ ] `NetherOreBlocks.NETHER_RUBY_ORE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherOreBlocks.java:23)
- [ ] `NetherOreBlocks.NETHER_LAPIS_ORE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherOreBlocks.java:24)
- [ ] `NetherOreBlocks.NETHER_REDSTONE_ORE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherOreBlocks.java:25)
- [x] `NetherOreBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherOreBlocks.java:27)

### `NetherPlantBlocks`

- [ ] `NetherPlantBlocks.EYE_SEED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:26)
- [ ] `NetherPlantBlocks.NETHER_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:27)
- [ ] `NetherPlantBlocks.SWAMP_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:28)
- [ ] `NetherPlantBlocks.SOUL_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:29)
- [ ] `NetherPlantBlocks.JUNGLE_PLANT`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:30)
- [ ] `NetherPlantBlocks.BONE_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:31)
- [ ] `NetherPlantBlocks.PALE_GLOOMGRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:32)
- [ ] `NetherPlantBlocks.GLOOMGRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:33)
- [ ] `NetherPlantBlocks.SEPIA_BONE_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:34)
- [ ] `NetherPlantBlocks.BLACK_BUSH`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:35)
- [ ] `NetherPlantBlocks.INK_BUSH`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:36)
- [ ] `NetherPlantBlocks.INK_BUSH_SEED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:37)
- [ ] `NetherPlantBlocks.EGG_PLANT`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:38)
- [ ] `NetherPlantBlocks.MAGMA_FLOWER`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:39)
- [ ] `NetherPlantBlocks.FEATHER_FERN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:40)
- [ ] `NetherPlantBlocks.MOSS_COVER`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:41)
- [ ] `NetherPlantBlocks.NEON_EQUISETUM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:42)
- [ ] `NetherPlantBlocks.AGAVE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:43)
- [ ] `NetherPlantBlocks.BARREL_CACTUS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:44)
- [ ] `NetherPlantBlocks.NETHER_CACTUS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:45)
- [ ] `NetherPlantBlocks.JUNGLE_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:46)
- [ ] `NetherPlantBlocks.SOUL_LILY`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:47)
- [ ] `NetherPlantBlocks.EYEBALL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:48)
- [ ] `NetherPlantBlocks.EYEBALL_SMALL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:49)
- [ ] `NetherPlantBlocks.POTTED_PLANT`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherPlantBlocks.java:50)
- [x] `NetherPlantBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherPlantBlocks.java:52)

### `NetherRecipes`

- [ ] `NetherRecipes.CATEGORY`: it uses Minecraft's `Codec` (org/betterx/betternether/registry/block/NetherRecipes.java:30)
- [ ] `NetherRecipes.STALACTITE`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betternether/registry/block/NetherRecipes.java:31)
- [ ] `NetherRecipes.SIMPLE_2X2`: it uses `Template4` (de.ambertation.wover.core.api.template) (org/betterx/betternether/registry/block/NetherRecipes.java:43)
- [ ] `NetherRecipes.FURNACE`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betternether/registry/block/NetherRecipes.java:51)
- [ ] `NetherRecipes.ROOF_TILE`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betternether/registry/block/NetherRecipes.java:56)
- [ ] `NetherRecipes.FIRE_BOWL`: it uses `Template3` (de.ambertation.wover.core.api.template) (org/betterx/betternether/registry/block/NetherRecipes.java:61)
- [ ] `NetherRecipes.STEM_TO_LOG`: it uses `Template1` (de.ambertation.wover.core.api.template) (org/betterx/betternether/registry/block/NetherRecipes.java:68)
- [ ] `NetherRecipes.MOLTEN_GLOOMSCULK`: it uses `Template0` (de.ambertation.wover.core.api.template) (org/betterx/betternether/registry/block/NetherRecipes.java:85)
- [ ] `NetherRecipes.GLOOMSCULK_LAMP`: it uses `Template2` (de.ambertation.wover.core.api.template) (org/betterx/betternether/registry/block/NetherRecipes.java:96)
- [ ] `NetherRecipes.MOLTEN_GLOOMWOOD_LOG`: it uses `Template2` (de.ambertation.wover.core.api.template) (org/betterx/betternether/registry/block/NetherRecipes.java:114)
- [ ] `NetherRecipes.TRANSITION_CONVERSION`: it uses `Template2` (de.ambertation.wover.core.api.template) (org/betterx/betternether/registry/block/NetherRecipes.java:140)
- [ ] `NetherRecipes.TRIMMED_CHEST`: it uses `Template2` (de.ambertation.wover.core.api.template) (org/betterx/betternether/registry/block/NetherRecipes.java:169)
- [ ] `NetherRecipes.STEMS_TO_BLOCK`: it uses `Template4` (de.ambertation.wover.core.api.template) (org/betterx/betternether/registry/block/NetherRecipes.java:188)
- [ ] `NetherRecipes.ROOF`: it uses `Template4` (de.ambertation.wover.core.api.template) (org/betterx/betternether/registry/block/NetherRecipes.java:205)
- [x] `NetherRecipes.NetherRecipes(0)` (org/betterx/betternether/registry/block/NetherRecipes.java:226)
- [ ] `NetherRecipes.deferred(1)`: it uses `RecipeMaterial` (de.ambertation.wover.recipe.api) (org/betterx/betternether/registry/block/NetherRecipes.java:229)
- [x] `NetherRecipes.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherRecipes.java:233)
- [ ] `NetherRecipes.stalactite(1)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/registry/block/NetherRecipes.java:236)
- [ ] `NetherRecipes.simple2x2(4)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/registry/block/NetherRecipes.java:240)
- [ ] `NetherRecipes.furnace(1)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/registry/block/NetherRecipes.java:244)
- [ ] `NetherRecipes.roofTile(1)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/registry/block/NetherRecipes.java:248)
- [ ] `NetherRecipes.fireBowl(3)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/registry/block/NetherRecipes.java:252)
- [ ] `NetherRecipes.stemToLog(1)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/registry/block/NetherRecipes.java:256)
- [ ] `NetherRecipes.moltenGloomsculk(0)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/registry/block/NetherRecipes.java:260)
- [ ] `NetherRecipes.gloomsculkLamp(2)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/registry/block/NetherRecipes.java:264)
- [ ] `NetherRecipes.moltenGloomwoodLog(2)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/registry/block/NetherRecipes.java:268)
- [ ] `NetherRecipes.transitionConversion(2)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/registry/block/NetherRecipes.java:272)
- [ ] `NetherRecipes.trimmedChest(2)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/registry/block/NetherRecipes.java:276)
- [ ] `NetherRecipes.stemsToBlock(4)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/registry/block/NetherRecipes.java:280)
- [ ] `NetherRecipes.roof(4)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/betternether/registry/block/NetherRecipes.java:284)

### `NetherRecipes.FireBowl`: it uses Minecraft's `Block`

- [ ] `NetherRecipes.FireBowl.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/registry/block/NetherRecipes.java:289)
- [ ] `NetherRecipes.FireBowl.FireBowl(3)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:288)
- [ ] `NetherRecipes.FireBowl.ring(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:288)
- [ ] `NetherRecipes.FireBowl.inside(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:288)
- [ ] `NetherRecipes.FireBowl.leg(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:288)

### `NetherRecipes.Furnace`: it uses Minecraft's `Block`

- [ ] `NetherRecipes.Furnace.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/registry/block/NetherRecipes.java:300)
- [ ] `NetherRecipes.Furnace.Furnace(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:299)
- [ ] `NetherRecipes.Furnace.source(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:299)

### `NetherRecipes.GloomsculkLamp`: it uses Minecraft's `Block`

- [ ] `NetherRecipes.GloomsculkLamp.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/registry/block/NetherRecipes.java:306)
- [ ] `NetherRecipes.GloomsculkLamp.GloomsculkLamp(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:305)
- [ ] `NetherRecipes.GloomsculkLamp.geode(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:305)
- [ ] `NetherRecipes.GloomsculkLamp.crystal(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:305)

### `NetherRecipes.MoltenGloomwoodLog`: it uses Minecraft's `Block`

- [ ] `NetherRecipes.MoltenGloomwoodLog.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/registry/block/NetherRecipes.java:316)
- [ ] `NetherRecipes.MoltenGloomwoodLog.MoltenGloomwoodLog(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:315)
- [ ] `NetherRecipes.MoltenGloomwoodLog.planks(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:315)
- [ ] `NetherRecipes.MoltenGloomwoodLog.log(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:315)

### `NetherRecipes.Roof`: it uses Minecraft's `Block`

- [ ] `NetherRecipes.Roof.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/registry/block/NetherRecipes.java:326)
- [ ] `NetherRecipes.Roof.Roof(4)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:325)
- [ ] `NetherRecipes.Roof.ingredient(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:325)
- [ ] `NetherRecipes.Roof.count(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:325)
- [ ] `NetherRecipes.Roof.group(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:325)
- [ ] `NetherRecipes.Roof.shape(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:325)

### `NetherRecipes.RoofTile`: it uses Minecraft's `Block`

- [ ] `NetherRecipes.RoofTile.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/registry/block/NetherRecipes.java:338)
- [ ] `NetherRecipes.RoofTile.RoofTile(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:337)
- [ ] `NetherRecipes.RoofTile.source(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:337)

### `NetherRecipes.Simple2x2`: it uses Minecraft's `Block`

- [ ] `NetherRecipes.Simple2x2.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/registry/block/NetherRecipes.java:344)
- [ ] `NetherRecipes.Simple2x2.Simple2x2(4)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:343)
- [ ] `NetherRecipes.Simple2x2.source(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:343)
- [ ] `NetherRecipes.Simple2x2.count(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:343)
- [ ] `NetherRecipes.Simple2x2.group(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:343)
- [ ] `NetherRecipes.Simple2x2.category(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:343)

### `NetherRecipes.Stalactite`: it uses Minecraft's `Block`

- [ ] `NetherRecipes.Stalactite.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/registry/block/NetherRecipes.java:356)
- [ ] `NetherRecipes.Stalactite.Stalactite(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:355)
- [ ] `NetherRecipes.Stalactite.source(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:355)

### `NetherRecipes.StemToLog`: it uses Minecraft's `Block`

- [ ] `NetherRecipes.StemToLog.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/registry/block/NetherRecipes.java:362)
- [ ] `NetherRecipes.StemToLog.StemToLog(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:361)
- [ ] `NetherRecipes.StemToLog.log(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:361)

### `NetherRecipes.StemsToBlock`: it uses Minecraft's `Block`

- [ ] `NetherRecipes.StemsToBlock.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/registry/block/NetherRecipes.java:368)
- [ ] `NetherRecipes.StemsToBlock.StemsToBlock(4)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:367)
- [ ] `NetherRecipes.StemsToBlock.stem(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:367)
- [ ] `NetherRecipes.StemsToBlock.count(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:367)
- [ ] `NetherRecipes.StemsToBlock.shaped(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:367)
- [ ] `NetherRecipes.StemsToBlock.group(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:367)

### `NetherRecipes.TransitionConversion`: it uses Minecraft's `Block`

- [ ] `NetherRecipes.TransitionConversion.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/registry/block/NetherRecipes.java:380)
- [ ] `NetherRecipes.TransitionConversion.TransitionConversion(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:379)
- [ ] `NetherRecipes.TransitionConversion.light(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:379)
- [ ] `NetherRecipes.TransitionConversion.dark(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:379)

### `NetherRecipes.TrimmedChest`: it uses Minecraft's `Block`

- [ ] `NetherRecipes.TrimmedChest.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/registry/block/NetherRecipes.java:390)
- [ ] `NetherRecipes.TrimmedChest.TrimmedChest(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:389)
- [ ] `NetherRecipes.TrimmedChest.chest(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:389)
- [ ] `NetherRecipes.TrimmedChest.strippedLog(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/registry/block/NetherRecipes.java:389)

### `NetherSaplingBlocks`

- [ ] `NetherSaplingBlocks.GIANT_MOLD_SAPLING`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherSaplingBlocks.java:10)
- [ ] `NetherSaplingBlocks.JELLYFISH_MUSHROOM_SAPLING`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherSaplingBlocks.java:11)
- [ ] `NetherSaplingBlocks.SOUL_LILY_SAPLING`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherSaplingBlocks.java:14)
- [x] `NetherSaplingBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherSaplingBlocks.java:16)

### `NetherStoneBlocks`

- [ ] `NetherStoneBlocks.NETHER_BRICK_TILE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:30)
- [ ] `NetherStoneBlocks.NETHER_BRICK_OPEN`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:31)
- [ ] `NetherStoneBlocks.BONE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:34)
- [ ] `NetherStoneBlocks.BONE_WOOD`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:35)
- [ ] `NetherStoneBlocks.SOUL_SANDSTONE_SHAPE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:47)
- [ ] `NetherStoneBlocks.SOUL_SANDSTONE_STONE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:48)
- [ ] `NetherStoneBlocks.BASALT`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:52)
- [ ] `NetherStoneBlocks.NETHERRACK`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:53)
- [ ] `NetherStoneBlocks.BLACKSTONE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:54)
- [ ] `NetherStoneBlocks.BLACKSTONE_STALACTITE_STONE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:55)
- [ ] `NetherStoneBlocks.GLOWSTONE_STALACTITE_STONE`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:56)
- [ ] `NetherStoneBlocks.NETHER_BRICK_TILE_LARGE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:62)
- [ ] `NetherStoneBlocks.NETHER_BRICK_TILE_SMALL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:63)
- [ ] `NetherStoneBlocks.NETHER_BRICK_WALL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:64)
- [ ] `NetherStoneBlocks.NETHER_BRICK_TILE_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:65)
- [ ] `NetherStoneBlocks.NETHER_BRICK_TILE_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:66)
- [ ] `NetherStoneBlocks.BONE_BLOCK`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:67)
- [ ] `NetherStoneBlocks.BONE_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:68)
- [ ] `NetherStoneBlocks.BONE_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:69)
- [ ] `NetherStoneBlocks.BONE_BUTTON`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:70)
- [ ] `NetherStoneBlocks.BONE_PLATE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:71)
- [ ] `NetherStoneBlocks.BONE_WALL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:72)
- [ ] `NetherStoneBlocks.BONE_TILE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:73)
- [ ] `NetherStoneBlocks.BONE_REED_DOOR`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:74)
- [ ] `NetherStoneBlocks.BONE_CINCINNASITE_DOOR`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:75)
- [ ] `NetherStoneBlocks.SOUL_SANDSTONE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:76)
- [ ] `NetherStoneBlocks.SOUL_SANDSTONE_CUT`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:77)
- [ ] `NetherStoneBlocks.SOUL_SANDSTONE_CUT_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:78)
- [ ] `NetherStoneBlocks.SOUL_SANDSTONE_CUT_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:79)
- [ ] `NetherStoneBlocks.SOUL_SANDSTONE_WALL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:80)
- [ ] `NetherStoneBlocks.SOUL_SANDSTONE_SMOOTH`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:81)
- [ ] `NetherStoneBlocks.SOUL_SANDSTONE_CHISELED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:82)
- [ ] `NetherStoneBlocks.SOUL_SANDSTONE_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:83)
- [ ] `NetherStoneBlocks.SOUL_SANDSTONE_SMOOTH_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:84)
- [ ] `NetherStoneBlocks.SOUL_SANDSTONE_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:85)
- [ ] `NetherStoneBlocks.SOUL_SANDSTONE_SMOOTH_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:86)
- [ ] `NetherStoneBlocks.BASALT_BRICKS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:87)
- [ ] `NetherStoneBlocks.BASALT_BRICKS_STAIRS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:88)
- [ ] `NetherStoneBlocks.BASALT_BRICKS_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:89)
- [ ] `NetherStoneBlocks.BASALT_BRICKS_WALL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:90)
- [ ] `NetherStoneBlocks.BASALT_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:91)
- [ ] `NetherStoneBlocks.NETHERRACK_STALACTITE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:92)
- [ ] `NetherStoneBlocks.GLOWSTONE_STALACTITE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:93)
- [ ] `NetherStoneBlocks.BLACKSTONE_STALACTITE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:94)
- [ ] `NetherStoneBlocks.BASALT_STALACTITE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:95)
- [ ] `NetherStoneBlocks.BONE_STALACTITE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:96)
- [ ] `NetherStoneBlocks.NETHERRACK_SLAB`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:97)
- [ ] `NetherStoneBlocks.NETHERRACK_STAIR`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:98)
- [ ] `NetherStoneBlocks.NETHERRACK_WALLS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:99)
- [ ] `NetherStoneBlocks.stone(2)`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherStoneBlocks.java:101)
- [x] `NetherStoneBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherStoneBlocks.java:108)

### `NetherSurvivalRules`

- [ ] `NetherSurvivalRules.GLIMMERWILD_GROUND`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:11)
- [ ] `NetherSurvivalRules.NETHERRACK`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:12)
- [ ] `NetherSurvivalRules.NETHER_MYCELIUM`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:13)
- [ ] `NetherSurvivalRules.SOUL_SAND`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:14)
- [ ] `NetherSurvivalRules.NETHER_GROUND`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:15)
- [ ] `NetherSurvivalRules.NETHER_GROUND_AND_SCULK`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:24)
- [ ] `NetherSurvivalRules.SCULK_LIKE`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:25)
- [ ] `NetherSurvivalRules.NETHERRACK_NYLIUM_AND_SCULK`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:26)
- [ ] `NetherSurvivalRules.NYLIUM`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:29)
- [ ] `NetherSurvivalRules.NETHER_SAND`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:30)
- [ ] `NetherSurvivalRules.SOUL_GROUND`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:31)
- [ ] `NetherSurvivalRules.SOUL_GROUND_OR_FARMLAND`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:32)
- [ ] `NetherSurvivalRules.SOIL_OR_LOGS`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:33)
- [ ] `NetherSurvivalRules.GRAVEL`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:34)
- [ ] `NetherSurvivalRules.MAGMA_SAND_OR_SCULK`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:35)
- [ ] `NetherSurvivalRules.orGlimmerwild(1)`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:39)
- [ ] `NetherSurvivalRules.boneBlocks(0)`: it uses Minecraft's `BlockPredicate` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:43)
- [x] `NetherSurvivalRules.NetherSurvivalRules(0)` (org/betterx/betternether/registry/block/NetherSurvivalRules.java:47)

### `NetherTerrainBlocks`

- [ ] `NetherTerrainBlocks.SCULK_ROCK`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:30)
- [ ] `NetherTerrainBlocks.HOT_SCULK_ROCK`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:36)
- [ ] `NetherTerrainBlocks.NETHERRACK_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:39)
- [ ] `NetherTerrainBlocks.NETHER_MYCELIUM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:40)
- [ ] `NetherTerrainBlocks.JUNGLE_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:41)
- [ ] `NetherTerrainBlocks.MUSHROOM_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:42)
- [ ] `NetherTerrainBlocks.SEPIA_MUSHROOM_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:43)
- [ ] `NetherTerrainBlocks.SWAMPLAND_GRASS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:44)
- [ ] `NetherTerrainBlocks.FARMLAND`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:45)
- [ ] `NetherTerrainBlocks.CEILING_MUSHROOMS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:46)
- [ ] `NetherTerrainBlocks.BLEACHED_GLOOMSCULK`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:47)
- [ ] `NetherTerrainBlocks.MOLTEN_GLOOMSCULK`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:48)
- [ ] `NetherTerrainBlocks.VEINED_GLOOMSCULK`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:49)
- [ ] `NetherTerrainBlocks.GLOOMSCULK_GEODE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:50)
- [ ] `NetherTerrainBlocks.GLOOMSCULK_GEODE_CRYSTAL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:51)
- [ ] `NetherTerrainBlocks.GLOOMSCULK_LAMP`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:52)
- [ ] `NetherTerrainBlocks.GLOOMSCULK_CRYSTAL`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:53)
- [ ] `NetherTerrainBlocks.VEINED_SAND`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:54)
- [ ] `NetherTerrainBlocks.FARMLAND_MATERIAL`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:55)
- [ ] `NetherTerrainBlocks.VEINED_SAND_MATERIAL`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:71)
- [x] `NetherTerrainBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherTerrainBlocks.java:79)

### `NetherVineBlocks`

- [ ] `NetherVineBlocks.BLACK_VINE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherVineBlocks.java:16)
- [ ] `NetherVineBlocks.BLOOMING_VINE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherVineBlocks.java:17)
- [ ] `NetherVineBlocks.GOLDEN_VINE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherVineBlocks.java:18)
- [ ] `NetherVineBlocks.LUMABUS_VINE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherVineBlocks.java:19)
- [ ] `NetherVineBlocks.GOLDEN_LUMABUS_VINE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherVineBlocks.java:20)
- [ ] `NetherVineBlocks.GLOOMSCULK_VINE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherVineBlocks.java:21)
- [ ] `NetherVineBlocks.GLOOMWISP_VINE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherVineBlocks.java:22)
- [ ] `NetherVineBlocks.SOUL_VEIN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherVineBlocks.java:23)
- [ ] `NetherVineBlocks.WHISPERING_GOURD_VINE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherVineBlocks.java:24)
- [ ] `NetherVineBlocks.ANCHOR_TREE_VINE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherVineBlocks.java:25)
- [ ] `NetherVineBlocks.EYE_VINE`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherVineBlocks.java:26)
- [ ] `NetherVineBlocks.LUMABUS_SEED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherVineBlocks.java:27)
- [ ] `NetherVineBlocks.GOLDEN_LUMABUS_SEED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherVineBlocks.java:28)
- [x] `NetherVineBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherVineBlocks.java:30)

### `NetherWallPlantBlocks`

- [ ] `NetherWallPlantBlocks.WALL_MOSS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherWallPlantBlocks.java:8)
- [ ] `NetherWallPlantBlocks.WALL_MUSHROOM_BROWN`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherWallPlantBlocks.java:9)
- [ ] `NetherWallPlantBlocks.WALL_MUSHROOM_RED`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherWallPlantBlocks.java:10)
- [x] `NetherWallPlantBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherWallPlantBlocks.java:12)

### `NetherWoodBlocks`

- [ ] `NetherWoodBlocks.NETHER_REED_STEM`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:35)
- [ ] `NetherWoodBlocks.MAT_REED`: it uses NetherReedMaterial, which isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:36)
- [ ] `NetherWoodBlocks.MAT_STALAGNATE`: it uses StalagnateMaterial, which isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:37)
- [ ] `NetherWoodBlocks.MAT_WILLOW`: it uses WillowMaterial, which isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:38)
- [ ] `NetherWoodBlocks.MAT_WART`: it uses WartMaterial, which isn't translated (it extends `RoofMaterial`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:39)
- [ ] `NetherWoodBlocks.WARPED_WOOD`: it uses VanillaNetherWood, which isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:40)
- [ ] `NetherWoodBlocks.CRIMSON_WOOD`: it uses VanillaNetherWood, which isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:43)
- [ ] `NetherWoodBlocks.OAK_WOOD`: it uses VanillaWood, which isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:46)
- [ ] `NetherWoodBlocks.SPRUCE_WOOD`: it uses VanillaWood, which isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:47)
- [ ] `NetherWoodBlocks.BIRCH_WOOD`: it uses VanillaWood, which isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:48)
- [ ] `NetherWoodBlocks.JUNGLE_WOOD`: it uses VanillaWood, which isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:49)
- [ ] `NetherWoodBlocks.ACACIA_WOOD`: it uses VanillaWood, which isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:50)
- [ ] `NetherWoodBlocks.DARK_OAK_WOOD`: it uses VanillaWood, which isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:51)
- [ ] `NetherWoodBlocks.CHERRY_WOOD`: it uses VanillaWood, which isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:52)
- [ ] `NetherWoodBlocks.BAMBOO_WOOD`: it uses VanillaWood, which isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:53)
- [ ] `NetherWoodBlocks.MANGROVE_WOOD`: it uses VanillaWood, which isn't translated (it extends `VanillaFallback`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:54)
- [ ] `NetherWoodBlocks.MAT_RUBEUS`: it uses RubeusMaterial, which isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:55)
- [ ] `NetherWoodBlocks.MAT_MUSHROOM_FIR`: it uses MushroomFirMaterial, which isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:56)
- [ ] `NetherWoodBlocks.MAT_NETHER_MUSHROOM`: it uses NetherMushroomMaterial, which isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:57)
- [ ] `NetherWoodBlocks.MAT_ANCHOR_TREE`: it uses AnchorTreeMaterial, which isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:58)
- [ ] `NetherWoodBlocks.MAT_NETHER_SAKURA`: it uses NetherSakuraMaterial, which isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:59)
- [ ] `NetherWoodBlocks.MAT_GLOOMWOOD`: it uses GloomwoodMaterial, which isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:60)
- [ ] `NetherWoodBlocks.MAT_GLOOMWOOD_DARK`: it uses GloomwoodDarkMaterial, which isn't translated (it extends `NetherWoodenMaterial`) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:61)
- [ ] `NetherWoodBlocks.GLOOMWOOD_TRANSITION_STRIPPED_LOG`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:62)
- [ ] `NetherWoodBlocks.GLOOMWOOD_TRANSITION_LOG`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:65)
- [ ] `NetherWoodBlocks.GLOOMWOOD_DARK_MOLTEN_LOG`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:66)
- [ ] `NetherWoodBlocks.GLOOMWOOD_TRANSITION_PLANKS`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:69)
- [ ] `NetherWoodBlocks.NETHER_REED_STEM_MATERIAL`: it uses `MaterialInstance` (de.ambertation.wover.block.api.classify) (org/betterx/betternether/registry/block/NetherWoodBlocks.java:70)
- [x] `NetherWoodBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/block/NetherWoodBlocks.java:79)

### `EntityRenderRegistry`

- [x] `EntityRenderRegistry.DEFAULT_LAYER` (org/betterx/betternether/registry/EntityRenderRegistry.java:27)
- [ ] `EntityRenderRegistry.FIREFLY_MODEL`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betternether/registry/EntityRenderRegistry.java:28)
- [ ] `EntityRenderRegistry.NAGA_MODEL`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betternether/registry/EntityRenderRegistry.java:29)
- [ ] `EntityRenderRegistry.JUNGLE_SKELETON_MODEL`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betternether/registry/EntityRenderRegistry.java:30)
- [ ] `EntityRenderRegistry.FLYING_PIG_MODEL`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betternether/registry/EntityRenderRegistry.java:31)
- [ ] `EntityRenderRegistry.HYDROGEN_JELLYFISH_MODEL`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betternether/registry/EntityRenderRegistry.java:32)
- [ ] `EntityRenderRegistry.SKULL_MODEL`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betternether/registry/EntityRenderRegistry.java:33)
- [ ] `EntityRenderRegistry.registerMain(1)`: it uses Minecraft's `ModelLayerLocation` (org/betterx/betternether/registry/EntityRenderRegistry.java:35)
- [ ] `EntityRenderRegistry.register(0)`: it calls EntityRenderRegistry.registerRenderMob, which isn't translated (org/betterx/betternether/registry/EntityRenderRegistry.java:39)
- [ ] `EntityRenderRegistry.registerRenderMob(2)`: it uses Minecraft's `EntityType` (org/betterx/betternether/registry/EntityRenderRegistry.java:55)

### `NetherObjects`

- [ ] `NetherObjects.C`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/registry/features/configured/NetherObjects.java:11)
- [ ] `NetherObjects.PATCH_BASALT_STALACTITE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherObjects.java:12)
- [ ] `NetherObjects.PATCH_BASALT_STALAGMITE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherObjects.java:13)
- [ ] `NetherObjects.PATCH_SMOKER`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherObjects.java:14)

### `NetherOres`

- [ ] `NetherOres.C`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/registry/features/configured/NetherOres.java:10)
- [ ] `NetherOres.CINCINNASITE_ORE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherOres.java:11)
- [ ] `NetherOres.GLOOMWOOD_CEILING_DEBRIS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherOres.java:12)
- [ ] `NetherOres.NETHER_RUBY_ORE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherOres.java:13)
- [ ] `NetherOres.NETHER_RUBY_ORE_SOUL`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherOres.java:14)
- [ ] `NetherOres.NETHER_RUBY_ORE_LARGE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherOres.java:15)
- [ ] `NetherOres.NETHER_RUBY_ORE_RARE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherOres.java:16)
- [ ] `NetherOres.NETHER_LAPIS_ORE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherOres.java:17)
- [ ] `NetherOres.NETHER_REDSTONE_ORE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherOres.java:18)

### `NetherTerrain`

- [ ] `NetherTerrain.C`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/registry/features/configured/NetherTerrain.java:16)
- [ ] `NetherTerrain.LAVA_PITS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTerrain.java:17)
- [ ] `NetherTerrain.SCULK_VEIN`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTerrain.java:18)
- [ ] `NetherTerrain.sculkVein(1)`: it uses Minecraft's `MultifaceGrowthFeature` (org/betterx/betternether/registry/features/configured/NetherTerrain.java:22)

### `NetherTrees`

- [ ] `NetherTrees.C`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/registry/features/configured/NetherTrees.java:25)
- [ ] `NetherTrees.RUBEUS_TREE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:26)
- [ ] `NetherTrees.GLOOMWOOD_TREE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:29)
- [ ] `NetherTrees.GLOOMWOOD_TREE_SOLITARY`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:32)
- [ ] `NetherTrees.MUSHROOM_FIR`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:35)
- [ ] `NetherTrees.STALAGNATE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:38)
- [ ] `NetherTrees.STALAGNATE_DOWN`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:39)
- [ ] `NetherTrees.GIANT_MOLD`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:40)
- [ ] `NetherTrees.PATCH_BIG_RED_MUSHROOM`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:41)
- [ ] `NetherTrees.PATCH_BIG_BROWN_MUSHROOM`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:42)
- [ ] `NetherTrees.WART_TREE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:45)
- [ ] `NetherTrees.WILLOW_TREE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:46)
- [ ] `NetherTrees.OLD_WILLOW_TREE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:49)
- [ ] `NetherTrees.ANCHOR_TREE_BRANCH`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:52)
- [ ] `NetherTrees.ANCHOR_TREE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:55)
- [ ] `NetherTrees.ANCHOR_TREE_ROOT`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:58)
- [ ] `NetherTrees.SAKURA_TREE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherTrees.java:61)

### `NetherVegetation`

- [ ] `NetherVegetation.C`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:31)
- [ ] `NetherVegetation.BONEMEAL_NETHERRACK_MOSS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:32)
- [ ] `NetherVegetation.BONEMEAL_NETHER_MYCELIUM`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:33)
- [ ] `NetherVegetation.BONEMEAL_JUNGLE_GRASS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:34)
- [ ] `NetherVegetation.BONEMEAL_MUSHROOM_GRASS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:35)
- [ ] `NetherVegetation.BONEMEAL_SEPIA_MUSHROOM_GRASS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:38)
- [ ] `NetherVegetation.BONEMEAL_SWAMPLAND_GRASS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:41)
- [ ] `NetherVegetation.BONEMEAL_CEILING_MUSHROOMS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:42)
- [ ] `NetherVegetation.VEGETATION_MUSHROOM_FORREST`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:43)
- [ ] `NetherVegetation.JELLYFISH_MUSHROOM`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:46)
- [ ] `NetherVegetation.PATCH_JELLYFISH_MUSHROOM`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:49)
- [ ] `NetherVegetation.PATCH_BLACK_BUSH`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:50)
- [ ] `NetherVegetation.WALL_LUCIS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:51)
- [ ] `NetherVegetation.BONEMEAL_SOUL_SOIL`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:52)
- [ ] `NetherVegetation.VEGETATION_GLOOMWOOD_BLEACHED`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:53)
- [ ] `NetherVegetation.VEGETATION_GLOOMWOOD_SCULK`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:54)
- [ ] `NetherVegetation.VEGETATION_MAGMA_LAND`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:55)
- [ ] `NetherVegetation.GLOOMWISP_VINE_HEAD`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:56)
- [ ] `NetherVegetation.GLOOMWISP_VINE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:57)
- [ ] `NetherVegetation.VEGETATION_GRASSLANDS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:58)
- [ ] `NetherVegetation.VEGETATION_GRAVEL_DESERT`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:59)
- [ ] `NetherVegetation.VEGETATION_JUNGLE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:60)
- [ ] `NetherVegetation.VEGETATION_POOR_GRASSLANDS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:61)
- [ ] `NetherVegetation.VEGETATION_SOUL_PLAIN`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:64)
- [ ] `NetherVegetation.VEGETATION_WART_FOREST`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:65)
- [ ] `NetherVegetation.VEGETATION_WART_FOREST_EDGE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:66)
- [ ] `NetherVegetation.VEGETATION_SWAMPLAND`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:67)
- [ ] `NetherVegetation.VEGETATION_OLD_SWAMPLAND`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:68)
- [ ] `NetherVegetation.VEGETATION_OLD_WARPED_WOODS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:69)
- [ ] `NetherVegetation.NETHER_CACTUS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:70)
- [ ] `NetherVegetation.WALL_MUSHROOM_RED_WITH_MOSS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:71)
- [ ] `NetherVegetation.WALL_MUSHROOMS_WITH_MOSS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:72)
- [ ] `NetherVegetation.WALL_MUSHROOMS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:73)
- [ ] `NetherVegetation.WALL_JUNGLE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:74)
- [ ] `NetherVegetation.WALL_UPSIDE_DOWN`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:75)
- [ ] `NetherVegetation.NETHER_REED`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:76)
- [ ] `NetherVegetation.WART_BUSH`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:77)
- [ ] `NetherVegetation.WILLOW_BUSH`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:78)
- [ ] `NetherVegetation.RUBEUS_BUSH`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:81)
- [ ] `NetherVegetation.SAKURA_BUSH`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:84)
- [ ] `NetherVegetation.SCULK_VEGETATION`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:87)
- [ ] `NetherVegetation.HOOK_MUSHROOM`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:88)
- [ ] `NetherVegetation.MOSS_COVER`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:89)
- [ ] `NetherVegetation.setupBonemealFeatures(0)`: it uses NetherTerrainBlocks.NETHERRACK_MOSS, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/features/configured/NetherVegetation.java:91)

### `NetherVines`

- [ ] `NetherVines.C`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/registry/features/configured/NetherVines.java:15)
- [ ] `NetherVines.LUMABUS_VINE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:16)
- [ ] `NetherVines.GOLDEN_LUMABUS_VINE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:17)
- [ ] `NetherVines.EYE_VINE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:18)
- [ ] `NetherVines.PATCH_LUMABUS_VINE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:19)
- [ ] `NetherVines.PATCH_LUMABUS_VINE_GLOOM`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:20)
- [ ] `NetherVines.PATCH_GOLDEN_LUMABUS_VINE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:21)
- [ ] `NetherVines.PATCH_GOLDEN_VINE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:22)
- [ ] `NetherVines.PATCH_GOLDEN_VINE_SPARSE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:23)
- [ ] `NetherVines.PATCH_EYE_VINE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:24)
- [ ] `NetherVines.PATCH_GLOOMSCULK_VINE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:25)
- [ ] `NetherVines.PATCH_BLACK_VINE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:26)
- [ ] `NetherVines.PATCH_TWISTING_VINES`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:27)
- [ ] `NetherVines.PATCH_BLOOMING_VINE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:30)
- [ ] `NetherVines.NEON_EQUISETUM`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:31)
- [ ] `NetherVines.PATCH_NEON_EQUISETUM`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:32)
- [ ] `NetherVines.WHISPERING_GOURD_VINE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:33)
- [ ] `NetherVines.PATCH_WHISPERING_GOURD_VINE`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/configured/NetherVines.java:34)

### `NetherGlimmerwildFeatures`

- [ ] `NetherGlimmerwildFeatures.C`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:16)
- [ ] `NetherGlimmerwildFeatures.WOOL_FLOWER_MEADOW`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:17)
- [ ] `NetherGlimmerwildFeatures.MEADOW_TUFTS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:18)
- [ ] `NetherGlimmerwildFeatures.MEADOW_BLOOMS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:19)
- [ ] `NetherGlimmerwildFeatures.DESERT_PLANTS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:20)
- [ ] `NetherGlimmerwildFeatures.WOUND_PLANTS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:21)
- [ ] `NetherGlimmerwildFeatures.MUSHROOM_CAVE_PLANTS`: it uses `FeatureKey` (de.ambertation.wover.feature.api.configured) (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:22)
- [ ] `NetherGlimmerwildFeatures.PLACED_MUSHROOM_CAVE_PLANTS`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:23)
- [ ] `NetherGlimmerwildFeatures.PLACED_DESERT_PLANTS`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:25)
- [ ] `NetherGlimmerwildFeatures.PLACED_WOUND_PLANTS`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:27)
- [ ] `NetherGlimmerwildFeatures.PLACED_MEADOW_TUFTS`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:29)
- [ ] `NetherGlimmerwildFeatures.PLACED_MEADOW_BLOOMS`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:31)
- [ ] `NetherGlimmerwildFeatures.RAINBOW_DESERT`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:33)
- [ ] `NetherGlimmerwildFeatures.SHADOW_WOUND`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:34)
- [ ] `NetherGlimmerwildFeatures.MUSHROOM_CAVE`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:35)
- [x] `NetherGlimmerwildFeatures.ensureLoaded(0)` (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java:37)

### `NetherObjectsPlaced`

- [ ] `NetherObjectsPlaced.C`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:10)
- [ ] `NetherObjectsPlaced.PATCH_TERRACOTTA_CLUMP`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:11)
- [ ] `NetherObjectsPlaced.BONES`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:12)
- [ ] `NetherObjectsPlaced.JUNGLE_BONES`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:13)
- [ ] `NetherObjectsPlaced.OBSIDIAN_CRYSTAL`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:15)
- [ ] `NetherObjectsPlaced.BONE_STALAGMITE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:17)
- [ ] `NetherObjectsPlaced.STALACTITE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:19)
- [ ] `NetherObjectsPlaced.STALAGMITE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:21)
- [ ] `NetherObjectsPlaced.BASALT_STALACTITE_2`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:23)
- [ ] `NetherObjectsPlaced.BASALT_STALACTITE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:25)
- [ ] `NetherObjectsPlaced.BASALT_STALACTITE_SPARSE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:27)
- [ ] `NetherObjectsPlaced.BASALT_STALAGMITE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:29)
- [ ] `NetherObjectsPlaced.BASALT_STALAGMITE_SPARSE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:31)
- [ ] `NetherObjectsPlaced.BLACKSTONE_STALACTITE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:33)
- [ ] `NetherObjectsPlaced.BLACKSTONE_STALAGMITE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:35)
- [ ] `NetherObjectsPlaced.SMOKER`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:37)
- [ ] `NetherObjectsPlaced.SMOKER_SPARSE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:39)
- [ ] `NetherObjectsPlaced.WART_DEADWOOD`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:41)
- [ ] `NetherObjectsPlaced.SCULK_HIDDEN`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:43)
- [ ] `NetherObjectsPlaced.SCULK_TOP`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:45)
- [ ] `NetherObjectsPlaced.FOREST_LITTER`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java:47)

### `NetherOresPlaced`

- [ ] `NetherOresPlaced.C`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/registry/features/placed/NetherOresPlaced.java:11)
- [ ] `NetherOresPlaced.CINCINNASITE_ORE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherOresPlaced.java:12)
- [ ] `NetherOresPlaced.GLOOMWOOD_CEILING_DEBRIS`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherOresPlaced.java:14)
- [ ] `NetherOresPlaced.NETHER_RUBY_ORE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherOresPlaced.java:18)
- [ ] `NetherOresPlaced.NETHER_RUBY_ORE_SOUL`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherOresPlaced.java:20)
- [ ] `NetherOresPlaced.NETHER_RUBY_ORE_LARGE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherOresPlaced.java:22)
- [ ] `NetherOresPlaced.NETHER_RUBY_ORE_RARE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherOresPlaced.java:24)
- [ ] `NetherOresPlaced.NETHER_LAPIS_ORE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherOresPlaced.java:26)
- [ ] `NetherOresPlaced.NETHER_REDSTONE_ORE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherOresPlaced.java:28)

### `NetherTerrainPlaced`

- [ ] `NetherTerrainPlaced.C`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:12)
- [ ] `NetherTerrainPlaced.LAVA_PIT`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:13)
- [ ] `NetherTerrainPlaced.MAGMA_BLOBS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:17)
- [ ] `NetherTerrainPlaced.BASALT_OR_AIR`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:18)
- [ ] `NetherTerrainPlaced.MARK`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:19)
- [ ] `NetherTerrainPlaced.EXTEND_BASALT`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:20)
- [ ] `NetherTerrainPlaced.LAVA_PITS_SPARSE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:21)
- [ ] `NetherTerrainPlaced.LAVA_PITS_DENSE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:25)
- [ ] `NetherTerrainPlaced.LAVA_SWAMP`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:29)
- [ ] `NetherTerrainPlaced.LAVA_TERRACE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:31)
- [ ] `NetherTerrainPlaced.FLOODED_LAVA_PIT_SURFACE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:33)
- [ ] `NetherTerrainPlaced.FLOODED_LAVA_PIT`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:35)
- [ ] `NetherTerrainPlaced.MOLTEN_GLOOMSCULK`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:37)
- [ ] `NetherTerrainPlaced.MOLTEN_GLOOMSCULK_NEAR_LAVA`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:39)
- [ ] `NetherTerrainPlaced.GLOOMSCULK_GEODE_FLOOR`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:41)
- [ ] `NetherTerrainPlaced.GLOOMSCULK_GEODE_CEILING`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:43)
- [ ] `NetherTerrainPlaced.GLOOMSCULK_GEODE_ON_FLOOR`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:45)
- [ ] `NetherTerrainPlaced.GLOOMSCULK_CRYSTAL_FLOOR`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:47)
- [ ] `NetherTerrainPlaced.GLOOMSCULK_CRYSTAL_CEILING`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:49)
- [ ] `NetherTerrainPlaced.SCULK_VEIN_FLOOR`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:51)
- [ ] `NetherTerrainPlaced.SCULK_VEIN_WALL`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:55)
- [ ] `NetherTerrainPlaced.SCULK_VEIN_CEILING`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:59)
- [ ] `NetherTerrainPlaced.REPLACE_SOUL_SANDSTONE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java:63)

### `NetherTreesPlaced`

- [ ] `NetherTreesPlaced.C`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:12)
- [ ] `NetherTreesPlaced.RUBEUS_TREE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:13)
- [ ] `NetherTreesPlaced.GLOOMWOOD_TREE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:15)
- [ ] `NetherTreesPlaced.GLOOMWOOD_TREE_EDGE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:17)
- [ ] `NetherTreesPlaced.GLOOMWOOD_TREE_SOLITARY`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:21)
- [ ] `NetherTreesPlaced.MUSHROOM_FIR`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:23)
- [ ] `NetherTreesPlaced.BIG_RED_MUSHROOM`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:25)
- [ ] `NetherTreesPlaced.BIG_BROWN_MUSHROOM`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:27)
- [ ] `NetherTreesPlaced.WART_TREE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:29)
- [ ] `NetherTreesPlaced.WILLOW_TREE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:31)
- [ ] `NetherTreesPlaced.SAKURA_TREE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:33)
- [ ] `NetherTreesPlaced.ANCHOR_TREE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:35)
- [ ] `NetherTreesPlaced.ANCHOR_TREE_BRANCH`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:37)
- [ ] `NetherTreesPlaced.ANCHOR_TREE_ROOT`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:39)
- [ ] `NetherTreesPlaced.CRIMSON_GLOWING_TREE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:41)
- [ ] `NetherTreesPlaced.CRIMSON_PINE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:43)
- [ ] `NetherTreesPlaced.STALAGNATE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:45)
- [ ] `NetherTreesPlaced.GIANT_MOLD`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:47)
- [ ] `NetherTreesPlaced.OLD_RED_MUSHROOM`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:49)
- [ ] `NetherTreesPlaced.OLD_BROWN_MUSHROOM`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:51)
- [ ] `NetherTreesPlaced.SOUL_LILY`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:53)
- [ ] `NetherTreesPlaced.OLD_WILLOW_TREE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:55)
- [ ] `NetherTreesPlaced.ANCHOR_TREE_SPARSE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:57)
- [ ] `NetherTreesPlaced.BIG_WARPED_TREE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java:61)

### `NetherVegetationPlaced`

- [ ] `NetherVegetationPlaced.C`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:12)
- [ ] `NetherVegetationPlaced.VEGETATION_MUSHROOM_FORREST_EDGE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:13)
- [ ] `NetherVegetationPlaced.JELLYFISH_MUSHROOM`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:17)
- [ ] `NetherVegetationPlaced.PATCH_JELLYFISH_MUSHROOM`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:19)
- [ ] `NetherVegetationPlaced.JELLYFISH_MUSHROOM_DENSE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:23)
- [ ] `NetherVegetationPlaced.BLACK_BUSH`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:27)
- [ ] `NetherVegetationPlaced.BLACK_BUSH_SPARSE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:29)
- [ ] `NetherVegetationPlaced.WALL_LUCIS`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:33)
- [ ] `NetherVegetationPlaced.VEGETATION_BONE_REEF`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:37)
- [ ] `NetherVegetationPlaced.VEGETATION_SULFURIC_BONE_REEF`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:39)
- [ ] `NetherVegetationPlaced.VEGETATION_MAGMA_LAND`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:41)
- [ ] `NetherVegetationPlaced.VEGETATION_CRIMSON_GLOWING_WOODS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:43)
- [ ] `NetherVegetationPlaced.VEGETATION_GRASSLANDS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:47)
- [ ] `NetherVegetationPlaced.VEGETATION_GRAVEL_DESERT`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:49)
- [ ] `NetherVegetationPlaced.VEGETATION_JUNGLE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:51)
- [ ] `NetherVegetationPlaced.VEGETATION_MUSHROOM_FORREST`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:53)
- [ ] `NetherVegetationPlaced.VEGETATION_POOR_GRASSLANDS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:57)
- [ ] `NetherVegetationPlaced.VEGETATION_SOUL_PLAIN`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:59)
- [ ] `NetherVegetationPlaced.VEGETATION_WART_FOREST`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:61)
- [ ] `NetherVegetationPlaced.VEGETATION_WART_FOREST_EDGE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:63)
- [ ] `NetherVegetationPlaced.VEGETATION_SWAMPLAND`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:65)
- [ ] `NetherVegetationPlaced.VEGETATION_OLD_SWAMPLAND`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:67)
- [ ] `NetherVegetationPlaced.VEGETATION_OLD_WARPED_WOODS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:69)
- [ ] `NetherVegetationPlaced.NETHER_CACTUS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:71)
- [ ] `NetherVegetationPlaced.WALL_MUSHROOM_RED_WITH_MOSS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:73)
- [ ] `NetherVegetationPlaced.WALL_MUSHROOMS_WITH_MOSS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:77)
- [ ] `NetherVegetationPlaced.WALL_MUSHROOMS`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:79)
- [ ] `NetherVegetationPlaced.WALL_JUNGLE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:81)
- [ ] `NetherVegetationPlaced.WALL_UPSIDE_DOWN`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:83)
- [ ] `NetherVegetationPlaced.VEGETATION_GLOOMWOOD_BLEACHED`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:85)
- [ ] `NetherVegetationPlaced.VEGETATION_GLOOMWOOD_SCULK`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:87)
- [ ] `NetherVegetationPlaced.GLOOMWISP_VINE_HEAD`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:89)
- [ ] `NetherVegetationPlaced.GLOOMWISP_VINE`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:91)
- [ ] `NetherVegetationPlaced.NETHER_REED`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:93)
- [ ] `NetherVegetationPlaced.WART_BUSH`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:95)
- [ ] `NetherVegetationPlaced.WILLOW_BUSH`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:97)
- [ ] `NetherVegetationPlaced.RUBEUS_BUSH`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:99)
- [ ] `NetherVegetationPlaced.SAKURA_BUSH`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:101)
- [ ] `NetherVegetationPlaced.WART_CAP`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:103)
- [ ] `NetherVegetationPlaced.SCULK_VEGETATION`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:104)
- [ ] `NetherVegetationPlaced.HOOK_MUSHROOM`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:106)
- [ ] `NetherVegetationPlaced.MOSS_COVER`: it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java:108)

### `NetherVinesPlaced`

- [ ] `NetherVinesPlaced.C`: it uses `ModCore` (de.ambertation.wover.core.api) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:11)
- [ ] `NetherVinesPlaced.PLACED_LUMABUS_VINE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:12)
- [ ] `NetherVinesPlaced.PLACED_LUMABUS_VINE_GLOOM`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:13)
- [ ] `NetherVinesPlaced.PLACED_GOLDEN_LUMABUS_VINE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:14)
- [ ] `NetherVinesPlaced.PLACED_EYE_VINE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:15)
- [ ] `NetherVinesPlaced.PLACED_NEON_EQUISETUM`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:16)
- [ ] `NetherVinesPlaced.PLACED_WHISPERING_GOURD_VINE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:17)
- [ ] `NetherVinesPlaced.LUMABUS_VINE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:18)
- [ ] `NetherVinesPlaced.GOLDEN_LUMABUS_VINE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:20)
- [ ] `NetherVinesPlaced.GOLDEN_VINE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:22)
- [ ] `NetherVinesPlaced.GOLDEN_VINE_SPARSE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:24)
- [ ] `NetherVinesPlaced.EYE_VINE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:26)
- [ ] `NetherVinesPlaced.GLOOMSCULK_VINE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:28)
- [ ] `NetherVinesPlaced.LUMABUS_VINE_SPARSE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:30)
- [ ] `NetherVinesPlaced.BLACK_VINE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:34)
- [ ] `NetherVinesPlaced.BLOOMING_VINE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:36)
- [ ] `NetherVinesPlaced.TWISTING_VINES`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:38)
- [ ] `NetherVinesPlaced.NEON_EQUISETUM`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:40)
- [ ] `NetherVinesPlaced.PATCH_WHISPERING_GOURD_VINE`: it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed) (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java:42)

### `NetherEquipmentItems`

- [ ] `NetherEquipmentItems.CINCINNASITE_SET`: it uses NetherSet, which isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)) (org/betterx/betternether/registry/item/NetherEquipmentItems.java:12)
- [ ] `NetherEquipmentItems.NETHER_RUBY_SET`: it uses NetherSet, which isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)) (org/betterx/betternether/registry/item/NetherEquipmentItems.java:13)
- [ ] `NetherEquipmentItems.CINCINNASITE_DIAMOND_SET`: it uses DiamondSet, which isn't translated (it extends `NetherSet`) (org/betterx/betternether/registry/item/NetherEquipmentItems.java:14)
- [ ] `NetherEquipmentItems.FLAMING_RUBY_SET`: it uses NetherSet, which isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)) (org/betterx/betternether/registry/item/NetherEquipmentItems.java:15)
- [ ] `NetherEquipmentItems.CINCINNASITE_HAMMER`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherEquipmentItems.java:16)
- [ ] `NetherEquipmentItems.CINCINNASITE_HAMMER_DIAMOND`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherEquipmentItems.java:17)
- [ ] `NetherEquipmentItems.NETHER_RUBY_HAMMER`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherEquipmentItems.java:18)
- [ ] `NetherEquipmentItems.CINCINNASITE_EXCAVATOR`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherEquipmentItems.java:19)
- [ ] `NetherEquipmentItems.CINCINNASITE_EXCAVATOR_DIAMOND`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherEquipmentItems.java:20)
- [ ] `NetherEquipmentItems.NETHER_RUBY_EXCAVATOR`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherEquipmentItems.java:21)
- [x] `NetherEquipmentItems.ensureLoaded(0)` (org/betterx/betternether/registry/item/NetherEquipmentItems.java:23)

### `NetherFoodItems`

- [ ] `NetherFoodItems.BLACK_APPLE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherFoodItems.java:8)
- [ ] `NetherFoodItems.STALAGNATE_BOWL`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherFoodItems.java:9)
- [ ] `NetherFoodItems.STALAGNATE_BOWL_WART`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherFoodItems.java:10)
- [ ] `NetherFoodItems.STALAGNATE_BOWL_MUSHROOM`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherFoodItems.java:11)
- [ ] `NetherFoodItems.STALAGNATE_BOWL_APPLE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherFoodItems.java:12)
- [ ] `NetherFoodItems.HOOK_MUSHROOM_COOKED`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherFoodItems.java:13)
- [x] `NetherFoodItems.ensureLoaded(0)` (org/betterx/betternether/registry/item/NetherFoodItems.java:15)

### `NetherItemFactories`

- [ ] `NetherItemFactories.CINCINNASITE`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:19)
- [ ] `NetherItemFactories.CINCINNASITE_INGOT`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:20)
- [ ] `NetherItemFactories.NETHER_RUBY`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:21)
- [ ] `NetherItemFactories.GLOWSTONE_PILE`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:22)
- [ ] `NetherItemFactories.LAPIS_PILE`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:23)
- [ ] `NetherItemFactories.AGAVE_LEAF`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:24)
- [ ] `NetherItemFactories.AGAVE_MEDICINE`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:25)
- [ ] `NetherItemFactories.HERBAL_MEDICINE`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:26)
- [ ] `NetherItemFactories.BLACK_APPLE`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:27)
- [ ] `NetherItemFactories.STALAGNATE_BOWL`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:28)
- [ ] `NetherItemFactories.STALAGNATE_BOWL_WART`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:29)
- [ ] `NetherItemFactories.STALAGNATE_BOWL_MUSHROOM`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:32)
- [ ] `NetherItemFactories.STALAGNATE_BOWL_APPLE`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:35)
- [ ] `NetherItemFactories.CINCINNASITE_HAMMER`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:38)
- [ ] `NetherItemFactories.CINCINNASITE_HAMMER_DIAMOND`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:41)
- [ ] `NetherItemFactories.NETHER_RUBY_HAMMER`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:44)
- [ ] `NetherItemFactories.CINCINNASITE_EXCAVATOR`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:47)
- [ ] `NetherItemFactories.CINCINNASITE_EXCAVATOR_DIAMOND`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:50)
- [ ] `NetherItemFactories.NETHER_RUBY_EXCAVATOR`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:53)
- [x] `NetherItemFactories.NetherItemFactories(0)` (org/betterx/betternether/registry/item/NetherItemFactories.java:57)
- [ ] `NetherItemFactories.factory(2)`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherItemFactories.java:60)
- [x] `NetherItemFactories.ensureLoaded(0)` (org/betterx/betternether/registry/item/NetherItemFactories.java:64)

### `NetherResourceItems`

- [ ] `NetherResourceItems.CINCINNASITE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherResourceItems.java:8)
- [ ] `NetherResourceItems.CINCINNASITE_INGOT`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherResourceItems.java:9)
- [ ] `NetherResourceItems.NETHER_RUBY`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherResourceItems.java:10)
- [ ] `NetherResourceItems.GLOWSTONE_PILE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherResourceItems.java:11)
- [ ] `NetherResourceItems.LAPIS_PILE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherResourceItems.java:12)
- [ ] `NetherResourceItems.AGAVE_LEAF`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherResourceItems.java:13)
- [ ] `NetherResourceItems.AGAVE_MEDICINE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherResourceItems.java:14)
- [ ] `NetherResourceItems.HERBAL_MEDICINE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/item/NetherResourceItems.java:15)
- [x] `NetherResourceItems.ensureLoaded(0)` (org/betterx/betternether/registry/item/NetherResourceItems.java:17)

### `NetherBiomes`

- [ ] `NetherBiomes.BONE_REEF`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:37)
- [ ] `NetherBiomes.CRIMSON_GLOWING_WOODS`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:38)
- [ ] `NetherBiomes.CRIMSON_PINEWOOD`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:39)
- [ ] `NetherBiomes.FLOODED_DELTAS`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:40)
- [ ] `NetherBiomes.GLOOMWOOD`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:41)
- [ ] `NetherBiomes.GRAVEL_DESERT`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:42)
- [ ] `NetherBiomes.MAGMA_LAND`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:43)
- [ ] `NetherBiomes.NETHER_GRASSLANDS`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:44)
- [ ] `NetherBiomes.NETHER_JUNGLE`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:45)
- [ ] `NetherBiomes.NETHER_MUSHROOM_FORREST`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:46)
- [ ] `NetherBiomes.NETHER_MUSHROOM_FORREST_EDGE`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:47)
- [ ] `NetherBiomes.NETHER_SWAMPLAND`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:48)
- [ ] `NetherBiomes.NETHER_SWAMPLAND_TERRACES`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:49)
- [ ] `NetherBiomes.OLD_FUNGIWOODS`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:50)
- [ ] `NetherBiomes.OLD_SWAMPLAND`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:51)
- [ ] `NetherBiomes.OLD_WARPED_WOODS`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:52)
- [ ] `NetherBiomes.POOR_NETHER_GRASSLANDS`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:53)
- [ ] `NetherBiomes.SOUL_PLAIN`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:54)
- [ ] `NetherBiomes.SULFURIC_BONE_REEF`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:55)
- [ ] `NetherBiomes.UPSIDE_DOWN_FOREST`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:56)
- [ ] `NetherBiomes.UPSIDE_DOWN_FOREST_CLEARED`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:57)
- [ ] `NetherBiomes.WART_FOREST`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:58)
- [ ] `NetherBiomes.WART_FOREST_EDGE`: it uses NetherBiomeKey, which isn't translated (it's generic) (org/betterx/betternether/registry/NetherBiomes.java:59)
- [ ] `NetherBiomes.register(0)`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/registry/NetherBiomes.java:61)
- [ ] `NetherBiomes.registerNumericProviders(0)`: it uses `NumericProviderRegistry` (de.ambertation.wover.surface.api.noise).NUMERIC_PROVIDER (org/betterx/betternether/registry/NetherBiomes.java:66)

### `NetherBlocks`

- [ ] `NetherBlocks.BLOCKS_REGISTRY`: it uses `BlockRegistry` (de.ambertation.wover.block.api) (org/betterx/betternether/registry/NetherBlocks.java:31)
- [x] `NetherBlocks.NetherBlocks(0)` (org/betterx/betternether/registry/NetherBlocks.java:33)
- [ ] `NetherBlocks.getBlockRegistry(0)`: it uses `BlockRegistry` (de.ambertation.wover.block.api) (org/betterx/betternether/registry/NetherBlocks.java:36)
- [ ] `NetherBlocks.key(2)`: it's generic (org/betterx/betternether/registry/NetherBlocks.java:45)
- [ ] `NetherBlocks.getModBlocks(0)`: it uses `Stream` (java.util.stream) (org/betterx/betternether/registry/NetherBlocks.java:49)
- [ ] `NetherBlocks.getModBlockItems(0)`: it uses `Stream` (java.util.stream) (org/betterx/betternether/registry/NetherBlocks.java:53)
- [x] `NetherBlocks.ensureLoaded(0)` (org/betterx/betternether/registry/NetherBlocks.java:57)

### `NetherClassification`: it implements `WoverBlocksEntrypoint` (de.ambertation.wover.block.api.classify)

- [ ] `NetherClassification.registerClassification(0)`: its class isn't translated (it implements `WoverBlocksEntrypoint` (de.ambertation.wover.block.api.classify)) (org/betterx/betternether/registry/NetherClassification.java:18)
- [ ] `NetherClassification.init(1)`: it uses `Class` (org/betterx/betternether/registry/NetherClassification.java:37)

### `NetherEnchantments`

- [ ] `NetherEnchantments.OBSIDIAN_BREAKER`: it uses `EnchantmentKey` (de.ambertation.wover.enchantment.api) (org/betterx/betternether/registry/NetherEnchantments.java:13)
- [ ] `NetherEnchantments.RUBY_FIRE`: it uses `EnchantmentKey` (de.ambertation.wover.enchantment.api) (org/betterx/betternether/registry/NetherEnchantments.java:14)
- [ ] `NetherEnchantments.OBSIDIAN_BLOCK_BREAK_SPEED`: it uses Minecraft's `Holder` (org/betterx/betternether/registry/NetherEnchantments.java:15)
- [x] `NetherEnchantments.ensureStaticallyLoaded(0)` (org/betterx/betternether/registry/NetherEnchantments.java:20)

### `NetherEntities`

- [ ] `NetherEntities.ATTRIBUTES`: it uses Minecraft's `EntityType` (org/betterx/betternether/registry/NetherEntities.java:43)
- [ ] `NetherEntities.NETHER_ENTITIES`: it uses `BCLEntityWrapper` (org.betterx.bclib.entity) (org/betterx/betternether/registry/NetherEntities.java:44)
- [ ] `NetherEntities.SPAWN_EGGS`: it uses SpawnEgg, which isn't translated (it uses Minecraft's `EntityType`) (org/betterx/betternether/registry/NetherEntities.java:45)
- [ ] `NetherEntities.NAGA_PROJECTILE`: it uses Minecraft's `EntityType` (org/betterx/betternether/registry/NetherEntities.java:46)
- [ ] `NetherEntities.FIREFLY`: it uses `BCLEntityWrapper` (org.betterx.bclib.entity) (org/betterx/betternether/registry/NetherEntities.java:50)
- [ ] `NetherEntities.HYDROGEN_JELLYFISH`: it uses `BCLEntityWrapper` (org.betterx.bclib.entity) (org/betterx/betternether/registry/NetherEntities.java:61)
- [ ] `NetherEntities.NAGA`: it uses `BCLEntityWrapper` (org.betterx.bclib.entity) (org/betterx/betternether/registry/NetherEntities.java:72)
- [ ] `NetherEntities.FLYING_PIG`: it uses `BCLEntityWrapper` (org.betterx.bclib.entity) (org/betterx/betternether/registry/NetherEntities.java:83)
- [ ] `NetherEntities.JUNGLE_SKELETON`: it uses `BCLEntityWrapper` (org.betterx.bclib.entity) (org/betterx/betternether/registry/NetherEntities.java:94)
- [ ] `NetherEntities.SKULL`: it uses `BCLEntityWrapper` (org.betterx.bclib.entity) (org/betterx/betternether/registry/NetherEntities.java:105)
- [x] `NetherEntities.MAX_FLOAT_HEIGHT` (org/betterx/betternether/registry/NetherEntities.java:116)
- [ ] `NetherEntities.RULE_FLOAT_NOT_ABOVE_LAVA`: it uses `SpawnRule` (org.betterx.bclib.interfaces) (org/betterx/betternether/registry/NetherEntities.java:117)
- [ ] `NetherEntities.RULE_FLOAT_ABOVE_LAVA`: it uses `SpawnRule` (org.betterx.bclib.interfaces) (org/betterx/betternether/registry/NetherEntities.java:118)
- [ ] `NetherEntities.spawnEggs(0)`: it uses SpawnEgg, which isn't translated (it uses Minecraft's `EntityType`) (org/betterx/betternether/registry/NetherEntities.java:120)
- [ ] `NetherEntities.register(9)`: it's generic (org/betterx/betternether/registry/NetherEntities.java:124)
- [ ] `NetherEntities.testSpawnAboveLava(3)`: it uses Minecraft's `LevelAccessor` (org/betterx/betternether/registry/NetherEntities.java:149)
- [ ] `NetherEntities.register(0)`: it calls NetherEntities.registerEntity, which isn't translated (org/betterx/betternether/registry/NetherEntities.java:164)
- [ ] `NetherEntities.registerEntity(2)`: it uses Minecraft's `EntityType` (org/betterx/betternether/registry/NetherEntities.java:178)
- [ ] `NetherEntities.registerEntity(3)`: it uses Minecraft's `EntityType` (org/betterx/betternether/registry/NetherEntities.java:182)
- [ ] `NetherEntities.isNetherEntity(1)`: it uses Minecraft's `Entity` (org/betterx/betternether/registry/NetherEntities.java:187)

### `NetherEntities.KnownSpawnTypes`: its constants carry values

- [ ] `NetherEntities.KnownSpawnTypes.isVanilla(0)`: its class isn't translated (its constants carry values) (org/betterx/betternether/registry/NetherEntities.java:213)
- [ ] `NetherEntities.KnownSpawnTypes.addSpawn(2)`: its class isn't translated (its constants carry values) (org/betterx/betternether/registry/NetherEntities.java:217)
- [ ] `NetherEntities.KnownSpawnTypes.KnownSpawnTypes(4)`: its class isn't translated (its constants carry values) (org/betterx/betternether/registry/NetherEntities.java:228)
- [ ] `NetherEntities.KnownSpawnTypes.KnownSpawnTypes(4)`: its class isn't translated (its constants carry values) (org/betterx/betternether/registry/NetherEntities.java:236)

### `NetherEntities.SpawnEgg`: it uses Minecraft's `EntityType`

- [ ] `NetherEntities.SpawnEgg.SpawnEgg(4)`: its class isn't translated (it uses Minecraft's `EntityType`) (org/betterx/betternether/registry/NetherEntities.java:245)
- [ ] `NetherEntities.SpawnEgg.name(0)`: its class isn't translated (it uses Minecraft's `EntityType`) (org/betterx/betternether/registry/NetherEntities.java:245)
- [ ] `NetherEntities.SpawnEgg.type(0)`: its class isn't translated (it uses Minecraft's `EntityType`) (org/betterx/betternether/registry/NetherEntities.java:245)
- [ ] `NetherEntities.SpawnEgg.background(0)`: its class isn't translated (it uses Minecraft's `EntityType`) (org/betterx/betternether/registry/NetherEntities.java:245)
- [ ] `NetherEntities.SpawnEgg.dots(0)`: its class isn't translated (it uses Minecraft's `EntityType`) (org/betterx/betternether/registry/NetherEntities.java:245)

### `NetherFeatures`

- [ ] `NetherFeatures.JELLYFISH_MUSHROOM`: it uses JellyfishMushroomFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:46)
- [ ] `NetherFeatures.OBSIDIAN_CRYSTAL`: it uses CrystalFeature, which isn't translated (it extends `NetherSurfaceFeature`) (org/betterx/betternether/registry/NetherFeatures.java:49)
- [ ] `NetherFeatures.WART_BUSH`: it uses WartBushFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:50)
- [ ] `NetherFeatures.RUBEUS_TREE`: it uses RubeusTreeFeature, which isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/registry/NetherFeatures.java:51)
- [ ] `NetherFeatures.GLOOMWOOD_TREE`: it uses GloomwoodTreeFeature, which isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/registry/NetherFeatures.java:54)
- [ ] `NetherFeatures.MUSHROOM_FIR`: it uses MushroomFirFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:57)
- [ ] `NetherFeatures.BIG_BROWN_MUSHROOM`: it uses BigBrownMushroomFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:58)
- [ ] `NetherFeatures.RUBEUS_BUSH`: it uses RubeusBushFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:61)
- [ ] `NetherFeatures.LUCIS`: it uses LucisFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:62)
- [ ] `NetherFeatures.SOUL_LILY`: it uses SoulLilyFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:63)
- [ ] `NetherFeatures.WART_TREE`: it uses WartTreeFeature, which isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/registry/NetherFeatures.java:64)
- [ ] `NetherFeatures.WILLOW_BUSH`: it uses WillowBushFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:65)
- [ ] `NetherFeatures.WILLOW_TREE`: it uses WillowTreeFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:66)
- [ ] `NetherFeatures.OLD_WILLOW_TREE`: it uses OldWillowTree, which isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/registry/NetherFeatures.java:67)
- [ ] `NetherFeatures.SAKURA_TREE`: it uses NetherSakuraFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:70)
- [ ] `NetherFeatures.SAKURA_BUSH`: it uses NetherSakuraBushFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:71)
- [ ] `NetherFeatures.ANCHOR_TREE_BRANCH`: it uses AnchorTreeBranchFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:72)
- [ ] `NetherFeatures.ANCHOR_TREE`: it uses AnchorTreeFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:75)
- [ ] `NetherFeatures.ANCHOR_TREE_ROOT`: it uses AnchorTreeRootFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:76)
- [ ] `NetherFeatures.WART_CAP`: it uses WartCapFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:77)
- [ ] `NetherFeatures.TWISTING_VINES`: it uses TwistedVinesFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/registry/NetherFeatures.java:78)
- [ ] `NetherFeatures.AVOID_NETHER_CITY`: it uses Minecraft's `MapCodec` (org/betterx/betternether/registry/NetherFeatures.java:79)
- [ ] `NetherFeatures.register(3)`: it's generic (org/betterx/betternether/registry/NetherFeatures.java:83)
- [ ] `NetherFeatures.addDefaultFeatures(1)`: it uses NetherBiomeBuilder, which isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)) (org/betterx/betternether/registry/NetherFeatures.java:88)
- [ ] `NetherFeatures.addDefaultBNFeatures(1)`: it uses NetherBiomeBuilder, which isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)) (org/betterx/betternether/registry/NetherFeatures.java:92)
- [ ] `NetherFeatures.addDefaultOres(1)`: it uses NetherBiomeBuilder, which isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)) (org/betterx/betternether/registry/NetherFeatures.java:95)
- [ ] `NetherFeatures.register(0)`: it uses `WorldLifecycle` (de.ambertation.wover.events.api).SERVER_LEVEL_READY (org/betterx/betternether/registry/NetherFeatures.java:102)
- [ ] `NetherFeatures.onWorldLoad(4)`: it uses Minecraft's `ServerLevel` (org/betterx/betternether/registry/NetherFeatures.java:106)

### `NetherGameRules`

- [ ] `NetherGameRules.GLOOMWISP_DROPS_EXPERIENCE`: it uses Minecraft's `GameRule` (org/betterx/betternether/registry/NetherGameRules.java:9)
- [ ] `NetherGameRules.GENERATE_BLUE_RUINED_PORTALS`: it uses Minecraft's `GameRule` (org/betterx/betternether/registry/NetherGameRules.java:12)
- [ ] `NetherGameRules.GROW_LARGE_WILLOWS`: it uses Minecraft's `GameRule` (org/betterx/betternether/registry/NetherGameRules.java:15)
- [ ] `NetherGameRules.GROW_LARGE_ANCHOR_TREES`: it uses Minecraft's `GameRule` (org/betterx/betternether/registry/NetherGameRules.java:18)
- [x] `NetherGameRules.ensureStaticallyLoaded(0)` (org/betterx/betternether/registry/NetherGameRules.java:22)

### `NetherItems`

- [ ] `NetherItems.ITEMS_REGISTRY`: it uses `ItemRegistry` (de.ambertation.wover.item.api) (org/betterx/betternether/registry/NetherItems.java:37)
- [ ] `NetherItems.DEBUG_ITEMS`: it uses DebugItem, which isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)) (org/betterx/betternether/registry/NetherItems.java:38)
- [x] `NetherItems.NetherItems(0)` (org/betterx/betternether/registry/NetherItems.java:40)
- [ ] `NetherItems.getItemRegistry(0)`: it uses `ItemRegistry` (de.ambertation.wover.item.api) (org/betterx/betternether/registry/NetherItems.java:43)
- [ ] `NetherItems.key(2)`: it's generic (org/betterx/betternether/registry/NetherItems.java:52)
- [x] `NetherItems.defaultSettings(0)` (org/betterx/betternether/registry/NetherItems.java:56)
- [ ] `NetherItems.addDebugItem(3)`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/registry/NetherItems.java:60)
- [ ] `NetherItems.debugItems(0)`: it uses DebugItem, which isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)) (org/betterx/betternether/registry/NetherItems.java:68)
- [ ] `NetherItems.buildItem(3)`: it uses Minecraft's `CompoundTag` (org/betterx/betternether/registry/NetherItems.java:72)
- [ ] `NetherItems.fullLightRange(0)`: it uses Minecraft's `CompoundTag` (org/betterx/betternether/registry/NetherItems.java:94)
- [ ] `NetherItems.buildCitySpawnerData(0)`: it uses Minecraft's `CompoundTag` (org/betterx/betternether/registry/NetherItems.java:101)
- [x] `NetherItems.ensureLoaded(0)` (org/betterx/betternether/registry/NetherItems.java:139)

### `NetherItems.DebugItem`: it uses `ItemCreator` (de.ambertation.wover.item.api.classify)

- [ ] `NetherItems.DebugItem.DebugItem(3)`: its class isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)) (org/betterx/betternether/registry/NetherItems.java:165)
- [ ] `NetherItems.DebugItem.path(0)`: its class isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)) (org/betterx/betternether/registry/NetherItems.java:165)
- [ ] `NetherItems.DebugItem.factory(0)`: its class isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)) (org/betterx/betternether/registry/NetherItems.java:165)
- [ ] `NetherItems.DebugItem.icon(0)`: its class isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)) (org/betterx/betternether/registry/NetherItems.java:165)

### `NetherParticles`

- [ ] `NetherParticles.BLUE_DRIPPING_OBSIDIAN_TEAR`: it uses Minecraft's `SimpleParticleType` (org/betterx/betternether/registry/NetherParticles.java:10)
- [ ] `NetherParticles.BLUE_FALLING_OBSIDIAN_TEAR`: it uses Minecraft's `SimpleParticleType` (org/betterx/betternether/registry/NetherParticles.java:11)
- [ ] `NetherParticles.BLUE_LANDING_OBSIDIAN_TEAR`: it uses Minecraft's `SimpleParticleType` (org/betterx/betternether/registry/NetherParticles.java:12)
- [ ] `NetherParticles.BLUE_DRIPPING_OBSIDIAN_WEEP`: it uses Minecraft's `SimpleParticleType` (org/betterx/betternether/registry/NetherParticles.java:13)
- [ ] `NetherParticles.BLUE_FALLING_OBSIDIAN_WEEP`: it uses Minecraft's `SimpleParticleType` (org/betterx/betternether/registry/NetherParticles.java:14)
- [ ] `NetherParticles.BLUE_LANDING_OBSIDIAN_WEEP`: it uses Minecraft's `SimpleParticleType` (org/betterx/betternether/registry/NetherParticles.java:15)
- [ ] `NetherParticles.DRIPPING_OBSIDIAN_WEEP`: it uses Minecraft's `SimpleParticleType` (org/betterx/betternether/registry/NetherParticles.java:16)
- [ ] `NetherParticles.FALLING_OBSIDIAN_WEEP`: it uses Minecraft's `SimpleParticleType` (org/betterx/betternether/registry/NetherParticles.java:17)
- [ ] `NetherParticles.LANDING_OBSIDIAN_WEEP`: it uses Minecraft's `SimpleParticleType` (org/betterx/betternether/registry/NetherParticles.java:18)
- [ ] `NetherParticles.register(0)`: it uses NetherParticles.BLUE_DRIPPING_OBSIDIAN_TEAR, which isn't translated (it uses Minecraft's `SimpleParticleType`) (org/betterx/betternether/registry/NetherParticles.java:20)

### `NetherPoiTypes`

- [ ] `NetherPoiTypes.PIG_STATUE`: it uses `WoverPoiType` (de.ambertation.wover.poi.api) (org/betterx/betternether/registry/NetherPoiTypes.java:9)
- [x] `NetherPoiTypes.register(0)` (org/betterx/betternether/registry/NetherPoiTypes.java:13)

### `NetherStructurePieces`

- [ ] `NetherStructurePieces.NETHER_CITY_PIECE`: it uses Minecraft's `StructurePieceType` (org/betterx/betternether/registry/NetherStructurePieces.java:12)
- [ ] `NetherStructurePieces.CAVE_PIECE`: it uses Minecraft's `StructurePieceType` (org/betterx/betternether/registry/NetherStructurePieces.java:13)
- [ ] `NetherStructurePieces.DESTRUCTION_PIECE`: it uses Minecraft's `StructurePieceType` (org/betterx/betternether/registry/NetherStructurePieces.java:14)
- [ ] `NetherStructurePieces.ANCHOR_TREE_PIECE`: it uses Minecraft's `StructurePieceType` (org/betterx/betternether/registry/NetherStructurePieces.java:15)
- [ ] `NetherStructurePieces.LAVA_LAKE_PIECE`: it uses Minecraft's `StructurePieceType` (org/betterx/betternether/registry/NetherStructurePieces.java:16)
- [x] `NetherStructurePieces.ensureStaticLoad(0)` (org/betterx/betternether/registry/NetherStructurePieces.java:18)

### `NetherStructures`

- [ ] `NetherStructures.CITY_STRUCTURE`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betternether/registry/NetherStructures.java:20)
- [ ] `NetherStructures.PYRAMIDS`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betternether/registry/NetherStructures.java:24)
- [ ] `NetherStructures.GHAST_HIVE`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betternether/registry/NetherStructures.java:26)
- [ ] `NetherStructures.SPAWN_ALTAR_LADDER`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betternether/registry/NetherStructures.java:30)
- [ ] `NetherStructures.RESPAWN_POINTS`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betternether/registry/NetherStructures.java:35)
- [ ] `NetherStructures.PILLARS`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betternether/registry/NetherStructures.java:40)
- [ ] `NetherStructures.GARDENS`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betternether/registry/NetherStructures.java:43)
- [ ] `NetherStructures.PORTALS`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betternether/registry/NetherStructures.java:46)
- [ ] `NetherStructures.ALTARS`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betternether/registry/NetherStructures.java:49)
- [ ] `NetherStructures.JUNGLE_TEMPLES`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betternether/registry/NetherStructures.java:52)
- [ ] `NetherStructures.MEGA_LAVA_LAKE`: it uses `Simple` (de.ambertation.wover.structure.api.StructureKey) (org/betterx/betternether/registry/NetherStructures.java:56)
- [x] `NetherStructures.register(0)` (org/betterx/betternether/registry/NetherStructures.java:61)

### `NetherTags`

- [ ] `NetherTags.BETTER_NETHER_DECORATIONS`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:13)
- [ ] `NetherTags.CITY_AVOIDS`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:14)
- [ ] `NetherTags.SOUL_GROUND_ITEM`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:15)
- [ ] `NetherTags.NETHER_FARMLAND`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:16)
- [ ] `NetherTags.GLIMMERWILD_GROUND`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:17)
- [ ] `NetherTags.GLIMMERWILD_RAINBOW_SAND`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:18)
- [ ] `NetherTags.GLIMMERWILD_SOILS`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:19)
- [ ] `NetherTags.NETHER_SAND`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:20)
- [ ] `NetherTags.NETHER_NYLIUM`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:21)
- [ ] `NetherTags.BETTER_NETHER`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:22)
- [ ] `NetherTags.FIREFLY_FLOWERS`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:23)
- [ ] `NetherTags.FLAMING_RUBY_ENCHANTABLE`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:24)
- [ ] `NetherTags.FLAMING_RUBY_PRIMARY`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:25)
- [ ] `NetherTags.OBSIDIAN_BREAKER_ENCHANTABLE`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:26)
- [ ] `NetherTags.OBSIDIAN_BREAKER_PRIMARY`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:27)
- [ ] `NetherTags.OBSIDIAN_BREAKER_MINEABLE`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:28)
- [ ] `NetherTags.NETHER_PICKAXES`: it uses Minecraft's `TagKey` (org/betterx/betternether/registry/NetherTags.java:29)
- [x] `NetherTags.register(0)` (org/betterx/betternether/registry/NetherTags.java:31)

### `NetherTemplates`

- [ ] `NetherTemplates.EMPTY_SLOT_BOWL`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/registry/NetherTemplates.java:10)
- [ ] `NetherTemplates.EMPTY_SLOT_BLOCK`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/registry/NetherTemplates.java:11)
- [ ] `NetherTemplates.NETHER_BOWL_SMITHING_TEMPLATE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/NetherTemplates.java:12)
- [ ] `NetherTemplates.FLAMING_RUBY_TEMPLATE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/NetherTemplates.java:13)
- [ ] `NetherTemplates.CINCINNASITE_DIAMOND_TEMPLATE`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/NetherTemplates.java:14)
- [ ] `NetherTemplates.key(1)`: it uses `ItemKey` (de.ambertation.wover.item.api.classify) (org/betterx/betternether/registry/NetherTemplates.java:16)
- [x] `NetherTemplates.ensureStaticallyLoaded(0)` (org/betterx/betternether/registry/NetherTemplates.java:20)

### `SoundsRegistry`

- [ ] `SoundsRegistry.AMBIENT_MUSHROOM_FOREST`: it uses Minecraft's `Holder` (org/betterx/betternether/registry/SoundsRegistry.java:11)
- [ ] `SoundsRegistry.AMBIENT_GRAVEL_DESERT`: it uses Minecraft's `Holder` (org/betterx/betternether/registry/SoundsRegistry.java:12)
- [ ] `SoundsRegistry.AMBIENT_NETHER_JUNGLE`: it uses Minecraft's `Holder` (org/betterx/betternether/registry/SoundsRegistry.java:13)
- [ ] `SoundsRegistry.AMBIENT_SWAMPLAND`: it uses Minecraft's `Holder` (org/betterx/betternether/registry/SoundsRegistry.java:14)
- [ ] `SoundsRegistry.MUSIC_GLOOMWOOD`: it uses Minecraft's `Holder` (org/betterx/betternether/registry/SoundsRegistry.java:15)
- [ ] `SoundsRegistry.BLOCK_GLOOMWISP_CHIME`: it uses Minecraft's `Holder` (org/betterx/betternether/registry/SoundsRegistry.java:16)
- [ ] `SoundsRegistry.BLOCK_GLOOMWISP_BOON`: it uses Minecraft's `Holder` (org/betterx/betternether/registry/SoundsRegistry.java:17)
- [ ] `SoundsRegistry.MOB_FIREFLY_FLY`: it uses Minecraft's `Holder` (org/betterx/betternether/registry/SoundsRegistry.java:18)
- [ ] `SoundsRegistry.MOB_JELLYFISH`: it uses Minecraft's `Holder` (org/betterx/betternether/registry/SoundsRegistry.java:19)
- [ ] `SoundsRegistry.MOB_NAGA_IDLE`: it uses Minecraft's `Holder` (org/betterx/betternether/registry/SoundsRegistry.java:20)
- [ ] `SoundsRegistry.MOB_NAGA_ATTACK`: it uses Minecraft's `Holder` (org/betterx/betternether/registry/SoundsRegistry.java:21)
- [ ] `SoundsRegistry.MOB_SKULL_FLIGHT`: it uses Minecraft's `Holder` (org/betterx/betternether/registry/SoundsRegistry.java:22)
- [ ] `SoundsRegistry.register(1)`: it uses Minecraft's `SoundEvent` (org/betterx/betternether/registry/SoundsRegistry.java:24)
- [ ] `SoundsRegistry.registerHolder(1)`: it uses Minecraft's `Holder` (org/betterx/betternether/registry/SoundsRegistry.java:29)
- [x] `SoundsRegistry.ensureStaticallyLoaded(0)` (org/betterx/betternether/registry/SoundsRegistry.java:34)

### `BECreativeTabs`

- [ ] `BECreativeTabs.register(0)`: it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)) (org/betterx/betternether/tab/BECreativeTabs.java:19)
- [ ] `BECreativeTabs.isPlantLike(1)`: it uses Minecraft's `Block` (org/betterx/betternether/tab/BECreativeTabs.java:40)

### `BoneReef`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `BoneReef.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/BoneReef.java:17)
- [ ] `BoneReef.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/BoneReef.java:38)

### `CrimsonGlowingWoods`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `CrimsonGlowingWoods.NETHER_WART_BLOCK`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/CrimsonGlowingWoods.java:24)
- [ ] `CrimsonGlowingWoods.CRIMSON_NYLIUM`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/CrimsonGlowingWoods.java:25)
- [ ] `CrimsonGlowingWoods.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/CrimsonGlowingWoods.java:27)
- [ ] `CrimsonGlowingWoods.spawnWeight(1)`: it's generic (org/betterx/betternether/world/biomes/CrimsonGlowingWoods.java:49)
- [ ] `CrimsonGlowingWoods.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/CrimsonGlowingWoods.java:64)
- [ ] `CrimsonGlowingWoods.subBiomeOf(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/CrimsonGlowingWoods.java:70)

### `CrimsonPinewood`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `CrimsonPinewood.TERRAIN`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betternether/world/biomes/CrimsonPinewood.java:23)
- [ ] `CrimsonPinewood.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/CrimsonPinewood.java:25)
- [ ] `CrimsonPinewood.spawnWeight(1)`: it's generic (org/betterx/betternether/world/biomes/CrimsonPinewood.java:47)
- [ ] `CrimsonPinewood.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/CrimsonPinewood.java:62)
- [ ] `CrimsonPinewood.subBiomeOf(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/CrimsonPinewood.java:68)

### `FloodedDeltas`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `FloodedDeltas.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/FloodedDeltas.java:22)
- [ ] `FloodedDeltas.hasNetherCity(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/FloodedDeltas.java:42)
- [ ] `FloodedDeltas.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/FloodedDeltas.java:47)
- [ ] `FloodedDeltas.subBiomeOf(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/FloodedDeltas.java:60)

### `Gloomwood`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `Gloomwood.CEILING_NETHERRACK`: it uses `NoiseCondition` (de.ambertation.wover.surface.api.conditions) (org/betterx/betternether/world/biomes/Gloomwood.java:33)
- [ ] `Gloomwood.CEILING_DEEP`: it uses `NoiseCondition` (de.ambertation.wover.surface.api.conditions) (org/betterx/betternether/world/biomes/Gloomwood.java:34)
- [ ] `Gloomwood.FLOOR_SCULK`: it uses `NoiseCondition` (de.ambertation.wover.surface.api.conditions) (org/betterx/betternether/world/biomes/Gloomwood.java:35)
- [ ] `Gloomwood.FLOOR_MOLTEN`: it uses `NoiseCondition` (de.ambertation.wover.surface.api.conditions) (org/betterx/betternether/world/biomes/Gloomwood.java:36)
- [ ] `Gloomwood.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/Gloomwood.java:38)
- [ ] `Gloomwood.spawnWeight(1)`: it's generic (org/betterx/betternether/world/biomes/Gloomwood.java:78)
- [ ] `Gloomwood.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/Gloomwood.java:89)

### `GravelDesert`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `GravelDesert.GRAVEL`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/GravelDesert.java:21)
- [ ] `GravelDesert.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/GravelDesert.java:23)
- [ ] `GravelDesert.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/GravelDesert.java:41)
- [ ] `GravelDesert.spawnWeight(1)`: it's generic (org/betterx/betternether/world/biomes/GravelDesert.java:50)

### `MagmaLand`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `MagmaLand.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/MagmaLand.java:18)
- [ ] `MagmaLand.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/MagmaLand.java:38)

### `NetherGrasslands`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `NetherGrasslands.SOUL_SOIL`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/NetherGrasslands.java:24)
- [ ] `NetherGrasslands.SOUL_SAND`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/NetherGrasslands.java:25)
- [ ] `NetherGrasslands.MOSS`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/NetherGrasslands.java:26)
- [ ] `NetherGrasslands.BLUE`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/NetherGrasslands.java:27)
- [ ] `NetherGrasslands.LIGHT_BLUE`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/NetherGrasslands.java:28)
- [ ] `NetherGrasslands.CYAN`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/NetherGrasslands.java:29)
- [ ] `NetherGrasslands.GREEN`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/NetherGrasslands.java:30)
- [ ] `NetherGrasslands.LIME_GREEN`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/NetherGrasslands.java:31)
- [ ] `NetherGrasslands.YELLOW`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/NetherGrasslands.java:32)
- [ ] `NetherGrasslands.ORANGE`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/NetherGrasslands.java:33)
- [ ] `NetherGrasslands.RED`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/NetherGrasslands.java:34)
- [ ] `NetherGrasslands.PINK`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/NetherGrasslands.java:35)
- [ ] `NetherGrasslands.PURPLE`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/NetherGrasslands.java:36)
- [ ] `NetherGrasslands.MAGENTA`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/NetherGrasslands.java:37)
- [ ] `NetherGrasslands.BLACK`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/NetherGrasslands.java:38)
- [ ] `NetherGrasslands.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/NetherGrasslands.java:40)
- [ ] `NetherGrasslands.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/NetherGrasslands.java:60)
- [ ] `NetherGrasslands.spawnWeight(1)`: it's generic (org/betterx/betternether/world/biomes/NetherGrasslands.java:76)

### `NetherJungle`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `NetherJungle.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/NetherJungle.java:20)
- [ ] `NetherJungle.hasBNStructures(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/NetherJungle.java:49)
- [ ] `NetherJungle.spawnWeight(1)`: it's generic (org/betterx/betternether/world/biomes/NetherJungle.java:54)
- [ ] `NetherJungle.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/NetherJungle.java:75)

### `NetherMushroomForest`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `NetherMushroomForest.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/NetherMushroomForest.java:21)
- [ ] `NetherMushroomForest.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/NetherMushroomForest.java:46)
- [ ] `NetherMushroomForest.withEdgeBiome(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/NetherMushroomForest.java:52)

### `NetherMushroomForestEdge`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `NetherMushroomForestEdge.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/NetherMushroomForestEdge.java:20)
- [ ] `NetherMushroomForestEdge.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/NetherMushroomForestEdge.java:35)
- [ ] `NetherMushroomForestEdge.hasNetherCity(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/NetherMushroomForestEdge.java:54)

### `NetherSwampland`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `NetherSwampland.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/NetherSwampland.java:25)
- [ ] `NetherSwampland.addCustomSwamplandBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/NetherSwampland.java:51)
- [ ] `NetherSwampland.spawnWeight(1)`: it's generic (org/betterx/betternether/world/biomes/NetherSwampland.java:55)
- [ ] `NetherSwampland.hasDefaultOres(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/NetherSwampland.java:75)
- [ ] `NetherSwampland.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/NetherSwampland.java:80)

### `NetherSwamplandTerraces`: it extends `NetherSwampland`

- [ ] `NetherSwamplandTerraces.addCustomSwamplandBuildData(1)`: its class isn't translated (it extends `NetherSwampland`) (org/betterx/betternether/world/biomes/NetherSwamplandTerraces.java:10)
- [ ] `NetherSwamplandTerraces.subBiomeOf(0)`: its class isn't translated (it extends `NetherSwampland`) (org/betterx/betternether/world/biomes/NetherSwamplandTerraces.java:15)

### `OldFungiwoods`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `OldFungiwoods.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/OldFungiwoods.java:21)
- [ ] `OldFungiwoods.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/OldFungiwoods.java:46)
- [ ] `OldFungiwoods.subBiomeOf(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/OldFungiwoods.java:52)

### `OldSwampland`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `OldSwampland.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/OldSwampland.java:28)
- [ ] `OldSwampland.spawnWeight(1)`: it's generic (org/betterx/betternether/world/biomes/OldSwampland.java:56)
- [ ] `OldSwampland.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/OldSwampland.java:76)
- [ ] `OldSwampland.subBiomeOf(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/OldSwampland.java:97)

### `OldWarpedWoods`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `OldWarpedWoods.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/OldWarpedWoods.java:22)
- [ ] `OldWarpedWoods.spawnWeight(1)`: it's generic (org/betterx/betternether/world/biomes/OldWarpedWoods.java:43)
- [ ] `OldWarpedWoods.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/OldWarpedWoods.java:63)
- [ ] `OldWarpedWoods.subBiomeOf(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/OldWarpedWoods.java:69)

### `PoorNetherGrasslands`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `PoorNetherGrasslands.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/PoorNetherGrasslands.java:23)
- [ ] `PoorNetherGrasslands.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/PoorNetherGrasslands.java:41)
- [ ] `PoorNetherGrasslands.subBiomeOf(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/PoorNetherGrasslands.java:63)

### `NetherGrasslandsNumericProvider`: it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)

- [ ] `NetherGrasslandsNumericProvider.DEFAULT`: it uses NetherGrasslandsNumericProvider, which isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betternether/world/biomes/providers/NetherGrasslandsNumericProvider.java:11)
- [ ] `NetherGrasslandsNumericProvider.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/biomes/providers/NetherGrasslandsNumericProvider.java:12)
- [x] `NetherGrasslandsNumericProvider.SEED` (org/betterx/betternether/world/biomes/providers/NetherGrasslandsNumericProvider.java:13)
- [ ] `NetherGrasslandsNumericProvider.getNumber(1)`: its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betternether/world/biomes/providers/NetherGrasslandsNumericProvider.java:15)
- [ ] `NetherGrasslandsNumericProvider.pcodec(0)`: its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betternether/world/biomes/providers/NetherGrasslandsNumericProvider.java:25)

### `NetherMushroomForestEdgeNumericProvider`: it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)

- [ ] `NetherMushroomForestEdgeNumericProvider.DEFAULT`: it uses NetherMushroomForestEdgeNumericProvider, which isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betternether/world/biomes/providers/NetherMushroomForestEdgeNumericProvider.java:11)
- [ ] `NetherMushroomForestEdgeNumericProvider.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/biomes/providers/NetherMushroomForestEdgeNumericProvider.java:12)
- [x] `NetherMushroomForestEdgeNumericProvider.SEED` (org/betterx/betternether/world/biomes/providers/NetherMushroomForestEdgeNumericProvider.java:15)
- [ ] `NetherMushroomForestEdgeNumericProvider.getNumber(1)`: its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betternether/world/biomes/providers/NetherMushroomForestEdgeNumericProvider.java:17)
- [ ] `NetherMushroomForestEdgeNumericProvider.pcodec(0)`: its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)) (org/betterx/betternether/world/biomes/providers/NetherMushroomForestEdgeNumericProvider.java:22)

### `SoulPlain`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `SoulPlain.SOUL_SAND`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/SoulPlain.java:28)
- [ ] `SoulPlain.SOUL_SOIL`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/SoulPlain.java:29)
- [ ] `SoulPlain.SOUL_SANDSTONE`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/SoulPlain.java:30)
- [ ] `SoulPlain.LAVA`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/SoulPlain.java:31)
- [ ] `SoulPlain.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/SoulPlain.java:33)
- [ ] `SoulPlain.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/SoulPlain.java:53)
- [ ] `SoulPlain.subBiomeOf(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/SoulPlain.java:69)

### `SulfuricBoneReef`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `SulfuricBoneReef.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/SulfuricBoneReef.java:19)
- [ ] `SulfuricBoneReef.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/SulfuricBoneReef.java:40)
- [ ] `SulfuricBoneReef.subBiomeOf(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/SulfuricBoneReef.java:46)

### `UpsideDownForest`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `UpsideDownForest.CEILEING_MOSS`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/UpsideDownForest.java:23)
- [ ] `UpsideDownForest.NETHERRACK_MOSS`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/UpsideDownForest.java:24)
- [ ] `UpsideDownForest.NOISE_CEIL_LAYER`: it uses Minecraft's `MaterialCondition` (org/betterx/betternether/world/biomes/UpsideDownForest.java:25)
- [ ] `UpsideDownForest.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/UpsideDownForest.java:27)
- [ ] `UpsideDownForest.hasBNStructures(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/UpsideDownForest.java:57)
- [ ] `UpsideDownForest.hasBNFeatures(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/UpsideDownForest.java:62)
- [ ] `UpsideDownForest.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/UpsideDownForest.java:67)

### `UpsideDownForestCleared`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `UpsideDownForestCleared.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/UpsideDownForestCleared.java:25)
- [ ] `UpsideDownForestCleared.hasBNStructures(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/UpsideDownForestCleared.java:52)
- [ ] `UpsideDownForestCleared.hasBNFeatures(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/UpsideDownForestCleared.java:57)
- [ ] `UpsideDownForestCleared.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/UpsideDownForestCleared.java:62)
- [ ] `UpsideDownForestCleared.subBiomeOf(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/UpsideDownForestCleared.java:86)

### `NetherBiomeBuilder`: it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)

- [x] `NetherBiomeBuilder.GEN_CHANCE_FACTOR` (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java:41)
- [ ] `NetherBiomeBuilder.BEDROCK`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java:42)
- [ ] `NetherBiomeBuilder.ON_FLOOR`: it uses Minecraft's `MaterialCondition` (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java:43)
- [ ] `NetherBiomeBuilder.UNDER_FLOOR`: it uses Minecraft's `MaterialCondition` (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java:44)
- [ ] `NetherBiomeBuilder.ON_CEILING`: it uses Minecraft's `MaterialCondition` (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java:45)
- [ ] `NetherBiomeBuilder.UNDER_CEILING`: it uses Minecraft's `MaterialCondition` (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java:46)
- [ ] `NetherBiomeBuilder.BEDROCK_BOTTOM`: it uses Minecraft's `VerticalGradientCondition` (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java:47)
- [ ] `NetherBiomeBuilder.BEDROCK_TOP`: it uses Minecraft's `VerticalGradientCondition` (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java:50)
- [ ] `NetherBiomeBuilder.NetherBiomeBuilder(2)`: its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)) (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java:54)
- [ ] `NetherBiomeBuilder.addVanillaStructures(0)`: its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)) (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java:73)
- [ ] `NetherBiomeBuilder.addVanillaFeatures(0)`: its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)) (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java:77)
- [ ] `NetherBiomeBuilder.addBNStructures(0)`: its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)) (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java:93)
- [ ] `NetherBiomeBuilder.configure(1)`: its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)) (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java:98)
- [ ] `NetherBiomeBuilder.createKey(1)`: it's generic (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java:149)
- [ ] `NetherBiomeBuilder.registerBiomeData(1)`: its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)) (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java:153)

### `NetherBiomeKey`: it's generic

- [ ] `NetherBiomeKey.NetherBiomeKey(1)`: its class isn't translated (it's generic) (org/betterx/betternether/world/biomes/util/NetherBiomeKey.java:10)
- [ ] `NetherBiomeKey.bootstrap(1)`: its class isn't translated (it's generic) (org/betterx/betternether/world/biomes/util/NetherBiomeKey.java:14)
- [ ] `NetherBiomeKey.bootstrap(2)`: its class isn't translated (it's generic) (org/betterx/betternether/world/biomes/util/NetherBiomeKey.java:19)

### `WartForest`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `WartForest.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/WartForest.java:30)
- [ ] `WartForest.spawnWeight(1)`: it's generic (org/betterx/betternether/world/biomes/WartForest.java:53)
- [ ] `WartForest.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/WartForest.java:67)
- [ ] `WartForest.withEdgeBiome(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/WartForest.java:103)

### `WartForestEdge`: it extends `NetherBiomeConfig` (org.betterx.betternether.world)

- [ ] `WartForestEdge.addCustomBuildData(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/WartForestEdge.java:22)
- [ ] `WartForestEdge.surface(1)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/WartForestEdge.java:39)
- [ ] `WartForestEdge.hasNetherCity(0)`: its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)) (org/betterx/betternether/world/biomes/WartForestEdge.java:59)
- [ ] `WartForestEdge.spawnWeight(1)`: it's generic (org/betterx/betternether/world/biomes/WartForestEdge.java:64)

### `BNWorldGenerator`: it uses StructureGeneratorThreadContext, which isn't translated (it uses Minecraft's `MutableBlockPos`)

- [x] `BNWorldGenerator.onModInit(0)` (org/betterx/betternether/world/BNWorldGenerator.java:8)

### `AnchorTreeBranchFeature`: it extends `ContextFeature`

- [ ] `AnchorTreeBranchFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java:29)
- [x] `AnchorTreeBranchFeature.CURVE_X` (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java:30)
- [x] `AnchorTreeBranchFeature.CURVE_Y` (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java:31)
- [x] `AnchorTreeBranchFeature.MIDDLE_Y` (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java:32)
- [ ] `AnchorTreeBranchFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java:34)
- [ ] `AnchorTreeBranchFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java:38)
- [ ] `AnchorTreeBranchFeature.grow(6)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java:50)
- [ ] `AnchorTreeBranchFeature.safeSetIfAir(4)`: it uses Minecraft's `LevelAccessor` (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java:192)
- [ ] `AnchorTreeBranchFeature.safeSet(3)`: it uses Minecraft's `LevelAccessor` (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java:198)
- [ ] `AnchorTreeBranchFeature.updateDistances(2)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java:204)
- [ ] `AnchorTreeBranchFeature.updateSDFFrom(2)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java:229)
- [ ] `AnchorTreeBranchFeature.line(9)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java:245)
- [ ] `AnchorTreeBranchFeature.crown(7)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java:287)
- [ ] `AnchorTreeBranchFeature.canReplace(1)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java:348)
- [ ] `AnchorTreeBranchFeature.grow(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java:352)

### `AnchorTreeFeature`: it extends `ContextFeature`

- [ ] `AnchorTreeFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/AnchorTreeFeature.java:33)
- [ ] `AnchorTreeFeature.NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betternether/world/features/AnchorTreeFeature.java:34)
- [ ] `AnchorTreeFeature.wallPlants`: it uses Minecraft's `Block` (org/betterx/betternether/world/features/AnchorTreeFeature.java:35)
- [ ] `AnchorTreeFeature.FOLIAGE`: it uses AnchorTreeBranchFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeFeature.java:42)
- [x] `AnchorTreeFeature.SHAPED_FOR_DEPTH` (org/betterx/betternether/world/features/AnchorTreeFeature.java:43)
- [x] `AnchorTreeFeature.CITY_CLEARANCE` (org/betterx/betternether/world/features/AnchorTreeFeature.java:44)
- [ ] `AnchorTreeFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeFeature.java:46)
- [x] `AnchorTreeFeature.toMiddle(1)` (org/betterx/betternether/world/features/AnchorTreeFeature.java:50)
- [ ] `AnchorTreeFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeFeature.java:54)
- [ ] `AnchorTreeFeature.grow(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeFeature.java:66)
- [ ] `AnchorTreeFeature.growFoliage(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeFeature.java:72)
- [ ] `AnchorTreeFeature.canGenerate(1)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeFeature.java:115)
- [ ] `AnchorTreeFeature.grow(7)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeFeature.java:119)
- [ ] `AnchorTreeFeature.buildBigCircle(11)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeFeature.java:247)
- [ ] `AnchorTreeFeature.drawLine(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeFeature.java:276)
- [ ] `AnchorTreeFeature.lerp(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeFeature.java:297)
- [ ] `AnchorTreeFeature.lerpCos(4)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeFeature.java:304)
- [ ] `AnchorTreeFeature.lcos(1)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeFeature.java:311)
- [ ] `AnchorTreeFeature.line(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeFeature.java:315)
- [ ] `AnchorTreeFeature.cylinder(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeFeature.java:332)
- [ ] `AnchorTreeFeature.circleLinesEnds(10)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeFeature.java:353)
- [ ] `AnchorTreeFeature.makeMushroom(5)`: it uses Minecraft's `ServerLevelAccessor` (org/betterx/betternether/world/features/AnchorTreeFeature.java:424)

### `AnchorTreeRootFeature`: it extends `ContextFeature`

- [ ] `AnchorTreeRootFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/AnchorTreeRootFeature.java:27)
- [ ] `AnchorTreeRootFeature.LUCIS`: it uses LucisFeature, which isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeRootFeature.java:28)
- [ ] `AnchorTreeRootFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeRootFeature.java:30)
- [ ] `AnchorTreeRootFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeRootFeature.java:34)
- [ ] `AnchorTreeRootFeature.canReplace(1)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeRootFeature.java:168)
- [ ] `AnchorTreeRootFeature.buildLine(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeRootFeature.java:175)
- [ ] `AnchorTreeRootFeature.lerpCos(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeRootFeature.java:193)
- [ ] `AnchorTreeRootFeature.lcos(1)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeRootFeature.java:201)
- [ ] `AnchorTreeRootFeature.lineParable(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeRootFeature.java:205)
- [ ] `AnchorTreeRootFeature.sphere(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/AnchorTreeRootFeature.java:226)

### `BigBrownMushroomFeature`: it extends `ContextFeature`

- [ ] `BigBrownMushroomFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/BigBrownMushroomFeature.java:16)
- [ ] `BigBrownMushroomFeature.MIDDLE`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/BigBrownMushroomFeature.java:17)
- [ ] `BigBrownMushroomFeature.BOTTOM`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/BigBrownMushroomFeature.java:20)
- [ ] `BigBrownMushroomFeature.TOP`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/BigBrownMushroomFeature.java:23)
- [ ] `BigBrownMushroomFeature.SIDE_N`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/BigBrownMushroomFeature.java:26)
- [ ] `BigBrownMushroomFeature.SIDE_S`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/BigBrownMushroomFeature.java:29)
- [ ] `BigBrownMushroomFeature.SIDE_E`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/BigBrownMushroomFeature.java:32)
- [ ] `BigBrownMushroomFeature.SIDE_W`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/BigBrownMushroomFeature.java:35)
- [ ] `BigBrownMushroomFeature.CORNER_N`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/BigBrownMushroomFeature.java:38)
- [ ] `BigBrownMushroomFeature.CORNER_W`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/BigBrownMushroomFeature.java:41)
- [ ] `BigBrownMushroomFeature.CORNER_E`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/BigBrownMushroomFeature.java:44)
- [ ] `BigBrownMushroomFeature.CORNER_S`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/BigBrownMushroomFeature.java:47)
- [ ] `BigBrownMushroomFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/BigBrownMushroomFeature.java:51)
- [ ] `BigBrownMushroomFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/BigBrownMushroomFeature.java:55)

### `CavesFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `CavesFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/CavesFeature.java:12)
- [ ] `CavesFeature.caves`: it uses StructureCaves, which isn't translated (it implements `IStructure`) (org/betterx/betternether/world/features/CavesFeature.java:13)
- [ ] `CavesFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betternether/world/features/CavesFeature.java:15)
- [ ] `CavesFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betternether/world/features/CavesFeature.java:19)
- [ ] `CavesFeature.onLoad(1)`: it uses CavesFeature.caves, which isn't translated (it uses StructureCaves, which isn't translated (it implements `IStructure`)) (org/betterx/betternether/world/features/CavesFeature.java:26)
- [ ] `CavesFeature.isInCave(3)`: it uses CavesFeature.caves, which isn't translated (it uses StructureCaves, which isn't translated (it implements `IStructure`)) (org/betterx/betternether/world/features/CavesFeature.java:30)

### `GloomwoodTreeConfiguration`: it extends `NaturalTreeConfiguration`

- [x] `GloomwoodTreeConfiguration.SPACING` (org/betterx/betternether/world/features/configs/GloomwoodTreeConfiguration.java:10)
- [x] `GloomwoodTreeConfiguration.SOLITARY_BLEACHED_CHANCE` (org/betterx/betternether/world/features/configs/GloomwoodTreeConfiguration.java:11)
- [ ] `GloomwoodTreeConfiguration.NATURAL`: it uses GloomwoodTreeConfiguration, which isn't translated (it extends `NaturalTreeConfiguration`) (org/betterx/betternether/world/features/configs/GloomwoodTreeConfiguration.java:12)
- [ ] `GloomwoodTreeConfiguration.codecFor(1)`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/configs/GloomwoodTreeConfiguration.java:15)
- [ ] `GloomwoodTreeConfiguration.GloomwoodTreeConfiguration(3)`: its class isn't translated (it extends `NaturalTreeConfiguration`) (org/betterx/betternether/world/features/configs/GloomwoodTreeConfiguration.java:26)
- [ ] `GloomwoodTreeConfiguration.natural(0)`: it uses GloomwoodTreeConfiguration, which isn't translated (it extends `NaturalTreeConfiguration`) (org/betterx/betternether/world/features/configs/GloomwoodTreeConfiguration.java:31)
- [ ] `GloomwoodTreeConfiguration.bleachedSometimes(1)`: it uses GloomwoodTreeConfiguration, which isn't translated (it extends `NaturalTreeConfiguration`) (org/betterx/betternether/world/features/configs/GloomwoodTreeConfiguration.java:35)

### `NaturalTreeConfiguration`

- [x] `NaturalTreeConfiguration.NATURAL` (org/betterx/betternether/world/features/configs/NaturalTreeConfiguration.java:10)
- [x] `NaturalTreeConfiguration.USER` (org/betterx/betternether/world/features/configs/NaturalTreeConfiguration.java:11)
- [x] `NaturalTreeConfiguration.NATURAL_LARGE` (org/betterx/betternether/world/features/configs/NaturalTreeConfiguration.java:12)
- [x] `NaturalTreeConfiguration.USER_LARGE` (org/betterx/betternether/world/features/configs/NaturalTreeConfiguration.java:13)
- [ ] `NaturalTreeConfiguration.mapCodecFor(1)`: it's generic (org/betterx/betternether/world/features/configs/NaturalTreeConfiguration.java:18)
- [x] `NaturalTreeConfiguration.NaturalTreeConfiguration(2)` (org/betterx/betternether/world/features/configs/NaturalTreeConfiguration.java:28)
- [x] `NaturalTreeConfiguration.natural(0)` (org/betterx/betternether/world/features/configs/NaturalTreeConfiguration.java:34)
- [x] `NaturalTreeConfiguration.userGrown(0)` (org/betterx/betternether/world/features/configs/NaturalTreeConfiguration.java:38)
- [x] `NaturalTreeConfiguration.naturalLarge(0)` (org/betterx/betternether/world/features/configs/NaturalTreeConfiguration.java:42)
- [x] `NaturalTreeConfiguration.userGrownLarge(0)` (org/betterx/betternether/world/features/configs/NaturalTreeConfiguration.java:46)

### `ContextFeature`: it implements Minecraft's `Feature`

- [ ] `ContextFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betternether/world/features/ContextFeature.java:12)
- [ ] `ContextFeature.place(5)`: it has no body (org/betterx/betternether/world/features/ContextFeature.java:16)

### `CrystalFeature`: it extends `NetherSurfaceFeature`

- [ ] `CrystalFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/CrystalFeature.java:18)
- [ ] `CrystalFeature.PALETTES`: it uses Minecraft's `Block` (org/betterx/betternether/world/features/CrystalFeature.java:19)
- [x] `CrystalFeature.SQRT05` (org/betterx/betternether/world/features/CrystalFeature.java:25)
- [x] `CrystalFeature.MAX_ANGLE_X` (org/betterx/betternether/world/features/CrystalFeature.java:26)
- [x] `CrystalFeature.MAX_ANGLE_Y` (org/betterx/betternether/world/features/CrystalFeature.java:27)
- [x] `CrystalFeature.MAX_ANGLE_Z` (org/betterx/betternether/world/features/CrystalFeature.java:28)
- [ ] `CrystalFeature.NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betternether/world/features/CrystalFeature.java:29)
- [ ] `CrystalFeature.codec(0)`: its class isn't translated (it extends `NetherSurfaceFeature`) (org/betterx/betternether/world/features/CrystalFeature.java:31)
- [ ] `CrystalFeature.generate(5)`: its class isn't translated (it extends `NetherSurfaceFeature`) (org/betterx/betternether/world/features/CrystalFeature.java:35)
- [ ] `CrystalFeature.depth(4)`: its class isn't translated (it extends `NetherSurfaceFeature`) (org/betterx/betternether/world/features/CrystalFeature.java:86)
- [ ] `CrystalFeature.isInside(4)`: its class isn't translated (it extends `NetherSurfaceFeature`) (org/betterx/betternether/world/features/CrystalFeature.java:90)
- [ ] `CrystalFeature.isNotEdge(4)`: its class isn't translated (it extends `NetherSurfaceFeature`) (org/betterx/betternether/world/features/CrystalFeature.java:94)
- [ ] `CrystalFeature.dodecahedronSDF(3)`: its class isn't translated (it extends `NetherSurfaceFeature`) (org/betterx/betternether/world/features/CrystalFeature.java:103)
- [ ] `CrystalFeature.rigidNoise(2)`: its class isn't translated (it extends `NetherSurfaceFeature`) (org/betterx/betternether/world/features/CrystalFeature.java:110)
- [ ] `CrystalFeature.getState(2)`: its class isn't translated (it extends `NetherSurfaceFeature`) (org/betterx/betternether/world/features/CrystalFeature.java:115)

### `GloomwoodTreeFeature`: it extends `NonOverlappingFeature`

- [ ] `GloomwoodTreeFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:33)
- [x] `GloomwoodTreeFeature.MAX_CLEARANCE_SCAN` (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:35)
- [x] `GloomwoodTreeFeature.CROWN_MARGIN` (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:36)
- [x] `GloomwoodTreeFeature.MOLTEN_ROOT_CHANCE` (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:37)
- [x] `GloomwoodTreeFeature.ROOT_RADIUS` (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:38)
- [x] `GloomwoodTreeFeature.MIN_DEBRIS_DEPTH` (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:39)
- [x] `GloomwoodTreeFeature.MAX_DEBRIS_DEPTH` (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:40)
- [x] `GloomwoodTreeFeature.MOLTEN_COLUMN_CHANCE` (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:41)
- [ ] `GloomwoodTreeFeature.MIN_CLEARANCE`: it calls Size.values, which isn't in the mod's source (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:42)
- [ ] `GloomwoodTreeFeature.GloomwoodTreeFeature(1)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:44)
- [ ] `GloomwoodTreeFeature.bleachedChance(0)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:49)
- [ ] `GloomwoodTreeFeature.codec(0)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:53)
- [ ] `GloomwoodTreeFeature.isStructure(1)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:57)
- [ ] `GloomwoodTreeFeature.isGround(1)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:64)
- [ ] `GloomwoodTreeFeature.grow(5)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:69)
- [ ] `GloomwoodTreeFeature.moltenRoot(4)`: it uses Minecraft's `ServerLevelAccessor` (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:100)
- [ ] `GloomwoodTreeFeature.ancientDebrisRoots(4)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:118)
- [ ] `GloomwoodTreeFeature.hasRoomToGrow(2)`: it uses Minecraft's `BlockGetter` (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:142)
- [ ] `GloomwoodTreeFeature.clearanceAbove(2)`: it uses Minecraft's `BlockGetter` (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:146)
- [ ] `GloomwoodTreeFeature.grow(3)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:163)
- [ ] `GloomwoodTreeFeature.builder(3)`: it uses TreeBuilder, which isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:173)
- [ ] `GloomwoodTreeFeature.palette(4)`: it uses TreePalette, which isn't translated (it's an interface) (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:207)

### `GloomwoodTreeFeature.Size`: its constants carry values

- [ ] `GloomwoodTreeFeature.Size.Size(21)`: its class isn't translated (its constants carry values) (org/betterx/betternether/world/features/GloomwoodTreeFeature.java:249)

### `JellyfishMushroomFeature`: it extends `ContextFeature`

- [ ] `JellyfishMushroomFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/JellyfishMushroomFeature.java:20)
- [ ] `JellyfishMushroomFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/JellyfishMushroomFeature.java:22)
- [ ] `JellyfishMushroomFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/JellyfishMushroomFeature.java:26)
- [ ] `JellyfishMushroomFeature.growHere(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/JellyfishMushroomFeature.java:36)
- [ ] `JellyfishMushroomFeature.growSmall(2)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/JellyfishMushroomFeature.java:44)
- [ ] `JellyfishMushroomFeature.growMedium(2)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/JellyfishMushroomFeature.java:59)
- [ ] `JellyfishMushroomFeature.grow(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/JellyfishMushroomFeature.java:82)

### `LucisFeature`: it extends `ContextFeature`

- [ ] `LucisFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/LucisFeature.java:20)
- [ ] `LucisFeature.CENTER`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/LucisFeature.java:21)
- [ ] `LucisFeature.SIDE`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/LucisFeature.java:24)
- [ ] `LucisFeature.CORNER`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/LucisFeature.java:27)
- [ ] `LucisFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/LucisFeature.java:31)
- [ ] `LucisFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/LucisFeature.java:35)
- [ ] `LucisFeature.grow(4)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/LucisFeature.java:40)
- [ ] `LucisFeature.canReplace(1)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/LucisFeature.java:115)
- [ ] `LucisFeature.canGenerate(2)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/LucisFeature.java:119)
- [ ] `LucisFeature.grow(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/LucisFeature.java:130)

### `MushroomFirFeature`: it extends `ContextFeature`

- [ ] `MushroomFirFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/MushroomFirFeature.java:18)
- [ ] `MushroomFirFeature.SIDE_BIG_N`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/MushroomFirFeature.java:19)
- [ ] `MushroomFirFeature.SIDE_BIG_W`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/MushroomFirFeature.java:23)
- [ ] `MushroomFirFeature.SIDE_BIG_E`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/MushroomFirFeature.java:27)
- [ ] `MushroomFirFeature.SIDE_SMALL_S`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/MushroomFirFeature.java:31)
- [ ] `MushroomFirFeature.SIDE_SMALL_N`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/MushroomFirFeature.java:35)
- [ ] `MushroomFirFeature.SIDE_SMALL_W`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/MushroomFirFeature.java:39)
- [ ] `MushroomFirFeature.SIDE_SMALL_E`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/MushroomFirFeature.java:43)
- [ ] `MushroomFirFeature.END`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/MushroomFirFeature.java:47)
- [ ] `MushroomFirFeature.BLUE_MYCELIUM`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/MushroomFirFeature.java:51)
- [ ] `MushroomFirFeature.SIDE_BIG_S`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/MushroomFirFeature.java:54)
- [ ] `MushroomFirFeature.TOP`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/MushroomFirFeature.java:58)
- [ ] `MushroomFirFeature.MIDDLE`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/MushroomFirFeature.java:62)
- [ ] `MushroomFirFeature.BOTTOM`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/MushroomFirFeature.java:66)
- [ ] `MushroomFirFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/MushroomFirFeature.java:71)
- [ ] `MushroomFirFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/MushroomFirFeature.java:75)
- [ ] `MushroomFirFeature.grow(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/MushroomFirFeature.java:87)
- [ ] `MushroomFirFeature.grow(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/MushroomFirFeature.java:191)

### `NetherSakuraBushFeature`: it extends `ContextFeature`

- [ ] `NetherSakuraBushFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/NetherSakuraBushFeature.java:17)
- [ ] `NetherSakuraBushFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/NetherSakuraBushFeature.java:19)
- [ ] `NetherSakuraBushFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/NetherSakuraBushFeature.java:23)
- [ ] `NetherSakuraBushFeature.setIfAir(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/NetherSakuraBushFeature.java:93)

### `NetherSakuraFeature`: it extends `ContextFeature`

- [ ] `NetherSakuraFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/NetherSakuraFeature.java:22)
- [ ] `NetherSakuraFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/NetherSakuraFeature.java:24)
- [ ] `NetherSakuraFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/NetherSakuraFeature.java:28)
- [ ] `NetherSakuraFeature.grow(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/NetherSakuraFeature.java:33)
- [ ] `NetherSakuraFeature.updateSDFFrom(2)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/NetherSakuraFeature.java:89)
- [ ] `NetherSakuraFeature.updateDistances(2)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/NetherSakuraFeature.java:103)
- [ ] `NetherSakuraFeature.crown(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/NetherSakuraFeature.java:123)
- [ ] `NetherSakuraFeature.canReplace(1)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/NetherSakuraFeature.java:178)
- [ ] `NetherSakuraFeature.grow(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/NetherSakuraFeature.java:182)

### `NetherSurfaceFeature`: it implements Minecraft's `Feature`

- [ ] `NetherSurfaceFeature.isValidSurface(1)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betternether/world/features/NetherSurfaceFeature.java:15)
- [ ] `NetherSurfaceFeature.generate(4)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betternether/world/features/NetherSurfaceFeature.java:19)
- [ ] `NetherSurfaceFeature.generate(5)`: it has no body (org/betterx/betternether/world/features/NetherSurfaceFeature.java:23)
- [ ] `NetherSurfaceFeature.minHeight(1)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betternether/world/features/NetherSurfaceFeature.java:25)
- [ ] `NetherSurfaceFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (org/betterx/betternether/world/features/NetherSurfaceFeature.java:29)

### `NetherThreadDataStorage`

- [ ] `NetherThreadDataStorage.WORLD_GENERATOR`: it uses `ThreadLocal` (org/betterx/betternether/world/features/NetherThreadDataStorage.java:6)
- [ ] `NetherThreadDataStorage.generatorForThread(0)`: it uses BNWorldGenerator, which isn't translated (it uses StructureGeneratorThreadContext, which isn't translated (it uses Minecraft's `MutableBlockPos`)) (org/betterx/betternether/world/features/NetherThreadDataStorage.java:8)

### `NonOverlappingFeature`: it extends `ContextFeature`

- [ ] `NonOverlappingFeature.NonOverlappingFeature(1)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/NonOverlappingFeature.java:15)
- [ ] `NonOverlappingFeature.isStructure(1)`: it has no body (org/betterx/betternether/world/features/NonOverlappingFeature.java:19)
- [ ] `NonOverlappingFeature.isGround(1)`: it has no body (org/betterx/betternether/world/features/NonOverlappingFeature.java:21)
- [ ] `NonOverlappingFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/NonOverlappingFeature.java:23)
- [ ] `NonOverlappingFeature.grow(5)`: it has no body (org/betterx/betternether/world/features/NonOverlappingFeature.java:30)
- [ ] `NonOverlappingFeature.noObjNear(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/NonOverlappingFeature.java:34)
- [ ] `NonOverlappingFeature.isInside(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/NonOverlappingFeature.java:56)

### `OldWillowTree`: it extends `NonOverlappingFeature`

- [ ] `OldWillowTree.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/OldWillowTree.java:30)
- [x] `OldWillowTree.CROWN_SQUASH` (org/betterx/betternether/world/features/OldWillowTree.java:31)
- [x] `OldWillowTree.CURVE_X` (org/betterx/betternether/world/features/OldWillowTree.java:32)
- [x] `OldWillowTree.CURVE_Y` (org/betterx/betternether/world/features/OldWillowTree.java:33)
- [ ] `OldWillowTree.wallPlants`: it uses Minecraft's `Block` (org/betterx/betternether/world/features/OldWillowTree.java:34)
- [ ] `OldWillowTree.OldWillowTree(1)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/OldWillowTree.java:41)
- [ ] `OldWillowTree.codec(0)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/OldWillowTree.java:45)
- [ ] `OldWillowTree.grow(5)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/OldWillowTree.java:49)
- [ ] `OldWillowTree.place(5)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/OldWillowTree.java:143)
- [ ] `OldWillowTree.isStructure(1)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/OldWillowTree.java:149)
- [ ] `OldWillowTree.isGround(1)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/OldWillowTree.java:154)
- [ ] `OldWillowTree.line(9)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/OldWillowTree.java:159)
- [ ] `OldWillowTree.sphere(3)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/OldWillowTree.java:186)
- [ ] `OldWillowTree.crown(6)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/OldWillowTree.java:214)
- [ ] `OldWillowTree.grow(3)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/OldWillowTree.java:274)

### `PathsFeature`: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)

- [ ] `PathsFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/PathsFeature.java:12)
- [ ] `PathsFeature.paths`: it uses StructurePath, which isn't translated (it implements `IStructure`) (org/betterx/betternether/world/features/PathsFeature.java:13)
- [ ] `PathsFeature.codec(0)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betternether/world/features/PathsFeature.java:15)
- [ ] `PathsFeature.place(4)`: its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)) (org/betterx/betternether/world/features/PathsFeature.java:19)
- [ ] `PathsFeature.onLoad(1)`: it uses PathsFeature.paths, which isn't translated (it uses StructurePath, which isn't translated (it implements `IStructure`)) (org/betterx/betternether/world/features/PathsFeature.java:26)

### `RubeusBushFeature`: it extends `ContextFeature`

- [ ] `RubeusBushFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/RubeusBushFeature.java:19)
- [ ] `RubeusBushFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/RubeusBushFeature.java:21)
- [ ] `RubeusBushFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/RubeusBushFeature.java:25)
- [ ] `RubeusBushFeature.setIfAir(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/RubeusBushFeature.java:85)

### `RubeusTreeFeature`: it extends `NonOverlappingFeature`

- [ ] `RubeusTreeFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/RubeusTreeFeature.java:35)
- [x] `RubeusTreeFeature.CURVE_X` (org/betterx/betternether/world/features/RubeusTreeFeature.java:36)
- [x] `RubeusTreeFeature.CURVE_Y` (org/betterx/betternether/world/features/RubeusTreeFeature.java:37)
- [x] `RubeusTreeFeature.MIDDLE_Y` (org/betterx/betternether/world/features/RubeusTreeFeature.java:38)
- [ ] `RubeusTreeFeature.RubeusTreeFeature(1)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/RubeusTreeFeature.java:40)
- [ ] `RubeusTreeFeature.codec(0)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/RubeusTreeFeature.java:44)
- [ ] `RubeusTreeFeature.isStructure(1)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/RubeusTreeFeature.java:48)
- [ ] `RubeusTreeFeature.isGround(1)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/RubeusTreeFeature.java:53)
- [ ] `RubeusTreeFeature.place(5)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/RubeusTreeFeature.java:58)
- [ ] `RubeusTreeFeature.updateDistances(3)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/RubeusTreeFeature.java:64)
- [ ] `RubeusTreeFeature.grow(5)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/RubeusTreeFeature.java:110)
- [ ] `RubeusTreeFeature.line(9)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/RubeusTreeFeature.java:219)
- [ ] `RubeusTreeFeature.crown(7)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/RubeusTreeFeature.java:261)
- [ ] `RubeusTreeFeature.setCondition(6)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/RubeusTreeFeature.java:296)
- [ ] `RubeusTreeFeature.setIfAir(3)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/RubeusTreeFeature.java:316)
- [ ] `RubeusTreeFeature.setIfGroundOrAir(3)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/RubeusTreeFeature.java:329)
- [ ] `RubeusTreeFeature.setIfAirLeaves(3)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/RubeusTreeFeature.java:343)
- [ ] `RubeusTreeFeature.grow(3)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/RubeusTreeFeature.java:353)

### `SoulLilyFeature`: it extends `ContextFeature`

- [ ] `SoulLilyFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/SoulLilyFeature.java:17)
- [ ] `SoulLilyFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/SoulLilyFeature.java:19)
- [ ] `SoulLilyFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/SoulLilyFeature.java:23)
- [ ] `SoulLilyFeature.growTree(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/SoulLilyFeature.java:50)
- [ ] `SoulLilyFeature.growSmall(2)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/SoulLilyFeature.java:68)
- [ ] `SoulLilyFeature.growMedium(2)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/SoulLilyFeature.java:72)
- [ ] `SoulLilyFeature.growBig(2)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/SoulLilyFeature.java:89)
- [ ] `SoulLilyFeature.isAirSides(2)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/SoulLilyFeature.java:142)

### `TwistedVinesFeature`: it extends `ContextFeature`

- [ ] `TwistedVinesFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/TwistedVinesFeature.java:14)
- [ ] `TwistedVinesFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/TwistedVinesFeature.java:16)
- [ ] `TwistedVinesFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/TwistedVinesFeature.java:20)
- [ ] `TwistedVinesFeature.canPlaceAt(2)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/TwistedVinesFeature.java:56)

### `WartBushFeature`: it extends `ContextFeature`

- [ ] `WartBushFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/WartBushFeature.java:16)
- [ ] `WartBushFeature.DIRS`: it uses Minecraft's `Direction` (org/betterx/betternether/world/features/WartBushFeature.java:17)
- [ ] `WartBushFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/WartBushFeature.java:19)
- [ ] `WartBushFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/WartBushFeature.java:23)
- [ ] `WartBushFeature.setSeed(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/WartBushFeature.java:38)

### `WartCapFeature`: it extends `ContextFeature`

- [ ] `WartCapFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/WartCapFeature.java:16)
- [ ] `WartCapFeature.INSIDE`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/WartCapFeature.java:17)
- [ ] `WartCapFeature.SKIN`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/WartCapFeature.java:25)
- [ ] `WartCapFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/WartCapFeature.java:27)
- [ ] `WartCapFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/WartCapFeature.java:31)
- [ ] `WartCapFeature.isWall(2)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/WartCapFeature.java:100)

### `WartTreeFeature`: it extends `NonOverlappingFeature`

- [ ] `WartTreeFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/WartTreeFeature.java:26)
- [ ] `WartTreeFeature.WART_BLOCK`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/features/WartTreeFeature.java:27)
- [ ] `WartTreeFeature.HORIZONTAL`: it uses Minecraft's `Direction` (org/betterx/betternether/world/features/WartTreeFeature.java:28)
- [ ] `WartTreeFeature.WartTreeFeature(1)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/WartTreeFeature.java:30)
- [ ] `WartTreeFeature.codec(0)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/WartTreeFeature.java:34)
- [ ] `WartTreeFeature.grow(5)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/WartTreeFeature.java:38)
- [ ] `WartTreeFeature.PlaceRandomSeed(2)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/WartTreeFeature.java:142)
- [ ] `WartTreeFeature.isReplaceable(1)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/WartTreeFeature.java:163)
- [ ] `WartTreeFeature.isWart(1)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/WartTreeFeature.java:175)
- [ ] `WartTreeFeature.isStructure(1)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/WartTreeFeature.java:179)
- [ ] `WartTreeFeature.isGround(1)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/WartTreeFeature.java:184)
- [ ] `WartTreeFeature.grow(3)`: its class isn't translated (it extends `NonOverlappingFeature`) (org/betterx/betternether/world/features/WartTreeFeature.java:189)

### `WillowBushFeature`: it extends `ContextFeature`

- [ ] `WillowBushFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/WillowBushFeature.java:21)
- [ ] `WillowBushFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/WillowBushFeature.java:23)
- [ ] `WillowBushFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/WillowBushFeature.java:27)
- [ ] `WillowBushFeature.setIfAir(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/WillowBushFeature.java:129)

### `WillowTreeFeature`: it extends `ContextFeature`

- [ ] `WillowTreeFeature.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/features/WillowTreeFeature.java:25)
- [ ] `WillowTreeFeature.HOR`: it uses Minecraft's `Direction` (org/betterx/betternether/world/features/WillowTreeFeature.java:26)
- [ ] `WillowTreeFeature.codec(0)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/WillowTreeFeature.java:28)
- [ ] `WillowTreeFeature.place(5)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/WillowTreeFeature.java:32)
- [ ] `WillowTreeFeature.grow(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/WillowTreeFeature.java:37)
- [ ] `WillowTreeFeature.branch(7)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/WillowTreeFeature.java:85)
- [ ] `WillowTreeFeature.vine(3)`: its class isn't translated (it extends `ContextFeature`) (org/betterx/betternether/world/features/WillowTreeFeature.java:185)

### `NetherBiome`: it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)

- [ ] `NetherBiome.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/NetherBiome.java:12)
- [ ] `NetherBiome.NetherBiome(9)`: its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)) (org/betterx/betternether/world/NetherBiome.java:14)
- [ ] `NetherBiome.codec(0)`: its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)) (org/betterx/betternether/world/NetherBiome.java:28)

### `NetherBiomeConfig`

- [ ] `NetherBiomeConfig.NETHERRACK`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/NetherBiomeConfig.java:15)
- [ ] `NetherBiomeConfig.BEDROCK_BOTTOM`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/NetherBiomeConfig.java:16)
- [ ] `NetherBiomeConfig.BEDROCK_TOP`: it uses Minecraft's `MaterialRule` (org/betterx/betternether/world/NetherBiomeConfig.java:17)
- [x] `NetherBiomeConfig.NetherBiomeConfig(0)` (org/betterx/betternether/world/NetherBiomeConfig.java:19)
- [ ] `NetherBiomeConfig.subBiomeOf(0)`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/world/NetherBiomeConfig.java:22)
- [ ] `NetherBiomeConfig.withEdgeBiome(0)`: it uses Minecraft's `ResourceKey` (org/betterx/betternether/world/NetherBiomeConfig.java:26)
- [x] `NetherBiomeConfig.hasVanillaFeatures(0)` (org/betterx/betternether/world/NetherBiomeConfig.java:30)
- [x] `NetherBiomeConfig.hasVanillaOres(0)` (org/betterx/betternether/world/NetherBiomeConfig.java:34)
- [x] `NetherBiomeConfig.hasStalactites(0)` (org/betterx/betternether/world/NetherBiomeConfig.java:38)
- [x] `NetherBiomeConfig.hasDefaultOres(0)` (org/betterx/betternether/world/NetherBiomeConfig.java:42)
- [x] `NetherBiomeConfig.hasNetherCity(0)` (org/betterx/betternether/world/NetherBiomeConfig.java:46)
- [x] `NetherBiomeConfig.hasVanillaStructures(0)` (org/betterx/betternether/world/NetherBiomeConfig.java:50)
- [x] `NetherBiomeConfig.hasBNStructures(0)` (org/betterx/betternether/world/NetherBiomeConfig.java:54)
- [x] `NetherBiomeConfig.hasBNFeatures(0)` (org/betterx/betternether/world/NetherBiomeConfig.java:58)
- [ ] `NetherBiomeConfig.spawnWeight(1)`: it's generic (org/betterx/betternether/world/NetherBiomeConfig.java:62)
- [ ] `NetherBiomeConfig.addCustomBuildData(1)`: it has no body (org/betterx/betternether/world/NetherBiomeConfig.java:75)
- [ ] `NetherBiomeConfig.surface(1)`: it uses `BiomeSurfaceRuleBuilder` (de.ambertation.wover.biome.api.builder) (org/betterx/betternether/world/NetherBiomeConfig.java:77)

### `AvoidNetherCity`: it implements Minecraft's `PlacementFilter`

- [ ] `AvoidNetherCity.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/structures/city/AvoidNetherCity.java:13)
- [ ] `AvoidNetherCity.AvoidNetherCity(2)`: its class isn't translated (it implements Minecraft's `PlacementFilter`) (org/betterx/betternether/world/structures/city/AvoidNetherCity.java:23)
- [ ] `AvoidNetherCity.shouldPlace(3)`: its class isn't translated (it implements Minecraft's `PlacementFilter`) (org/betterx/betternether/world/structures/city/AvoidNetherCity.java:28)
- [ ] `AvoidNetherCity.codec(0)`: its class isn't translated (it implements Minecraft's `PlacementFilter`) (org/betterx/betternether/world/structures/city/AvoidNetherCity.java:34)

### `BoundingBox2D`

- [x] `BoundingBox2D.BoundingBox2D(4)` (org/betterx/betternether/world/structures/city/BoundingBox2D.java:12)
- [ ] `BoundingBox2D.BoundingBox2D(3)`: it uses Minecraft's `BlockPos` (org/betterx/betternether/world/structures/city/BoundingBox2D.java:19)
- [ ] `BoundingBox2D.BoundingBox2D(1)`: it uses Minecraft's `BlockPos` (org/betterx/betternether/world/structures/city/BoundingBox2D.java:26)
- [x] `BoundingBox2D.isColliding(1)` (org/betterx/betternether/world/structures/city/BoundingBox2D.java:33)
- [ ] `BoundingBox2D.offset(1)`: it uses Minecraft's `BlockPos` (org/betterx/betternether/world/structures/city/BoundingBox2D.java:39)
- [ ] `BoundingBox2D.offsetNegative(1)`: it uses Minecraft's `BlockPos` (org/betterx/betternether/world/structures/city/BoundingBox2D.java:43)
- [x] `BoundingBox2D.toString(0)` (org/betterx/betternether/world/structures/city/BoundingBox2D.java:47)
- [ ] `BoundingBox2D.rotate(1)`: it uses Minecraft's `Rotation` (org/betterx/betternether/world/structures/city/BoundingBox2D.java:52)
- [ ] `BoundingBox2D.getCenter(0)`: it uses Minecraft's `BlockPos` (org/betterx/betternether/world/structures/city/BoundingBox2D.java:67)
- [x] `BoundingBox2D.getSideX(0)` (org/betterx/betternether/world/structures/city/BoundingBox2D.java:71)
- [x] `BoundingBox2D.getSideZ(0)` (org/betterx/betternether/world/structures/city/BoundingBox2D.java:75)
- [x] `BoundingBox2D.getMinX(0)` (org/betterx/betternether/world/structures/city/BoundingBox2D.java:79)
- [x] `BoundingBox2D.getMaxX(0)` (org/betterx/betternether/world/structures/city/BoundingBox2D.java:83)
- [x] `BoundingBox2D.getMinZ(0)` (org/betterx/betternether/world/structures/city/BoundingBox2D.java:87)
- [x] `BoundingBox2D.getMaxZ(0)` (org/betterx/betternether/world/structures/city/BoundingBox2D.java:91)

### `BuildingStructureProcessor`: it implements Minecraft's `StructureProcessor`

- [ ] `BuildingStructureProcessor.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/structures/city/BuildingStructureProcessor.java:36)
- [ ] `BuildingStructureProcessor.BuildingStructureProcessor(1)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (org/betterx/betternether/world/structures/city/BuildingStructureProcessor.java:41)
- [ ] `BuildingStructureProcessor.setState(2)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (org/betterx/betternether/world/structures/city/BuildingStructureProcessor.java:45)
- [ ] `BuildingStructureProcessor.processBlock(6)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (org/betterx/betternether/world/structures/city/BuildingStructureProcessor.java:49)
- [ ] `BuildingStructureProcessor.codec(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (org/betterx/betternether/world/structures/city/BuildingStructureProcessor.java:163)

### `CityGenerator`: it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))

- [ ] `CityGenerator.CityGenerator(0)`: its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))) (org/betterx/betternether/world/structures/city/CityGenerator.java:21)
- [ ] `CityGenerator.addBuildingToList(2)`: its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))) (org/betterx/betternether/world/structures/city/CityGenerator.java:62)
- [ ] `CityGenerator.addBuildingToList(3)`: its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))) (org/betterx/betternether/world/structures/city/CityGenerator.java:66)
- [ ] `CityGenerator.placeCenterBuilding(5)`: its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))) (org/betterx/betternether/world/structures/city/CityGenerator.java:74)
- [ ] `CityGenerator.attachBuildings(3)`: its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))) (org/betterx/betternether/world/structures/city/CityGenerator.java:84)
- [ ] `CityGenerator.closeRoads(3)`: its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))) (org/betterx/betternether/world/structures/city/CityGenerator.java:131)
- [ ] `CityGenerator.generate(3)`: its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))) (org/betterx/betternether/world/structures/city/CityGenerator.java:152)
- [ ] `CityGenerator.noCollisions(1)`: its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))) (org/betterx/betternether/world/structures/city/CityGenerator.java:168)
- [ ] `CityGenerator.getBuildings(0)`: its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))) (org/betterx/betternether/world/structures/city/CityGenerator.java:178)

### `CityStructure`: it extends Minecraft's `Structure`

- [ ] `CityStructure.generator`: it uses CityGenerator, which isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))) (org/betterx/betternether/world/structures/city/CityStructure.java:43)
- [x] `CityStructure.RADIUS` (org/betterx/betternether/world/structures/city/CityStructure.java:44)
- [x] `CityStructure.DEFAULT_HEIGHT` (org/betterx/betternether/world/structures/city/CityStructure.java:45)
- [x] `CityStructure.BASTION_REACH` (org/betterx/betternether/world/structures/city/CityStructure.java:46)
- [ ] `CityStructure.CityStructure(1)`: its class isn't translated (it extends Minecraft's `Structure`) (org/betterx/betternether/world/structures/city/CityStructure.java:48)
- [ ] `CityStructure.initGenerator(0)`: it uses CityStructure.generator, which isn't translated (it uses CityGenerator, which isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)))) (org/betterx/betternether/world/structures/city/CityStructure.java:52)
- [ ] `CityStructure.findGenerationPoint(1)`: its class isn't translated (it extends Minecraft's `Structure`) (org/betterx/betternether/world/structures/city/CityStructure.java:56)
- [ ] `CityStructure.generatePieces(2)`: it uses Minecraft's `StructurePiecesBuilder` (org/betterx/betternether/world/structures/city/CityStructure.java:85)
- [ ] `CityStructure.getCenter(4)`: it uses Minecraft's `BlockPos` (org/betterx/betternether/world/structures/city/CityStructure.java:114)
- [ ] `CityStructure.buildingsWithin(3)`: it uses Minecraft's `ServerLevelAccessor` (org/betterx/betternether/world/structures/city/CityStructure.java:122)
- [ ] `CityStructure.buildingsIntersect(3)`: it uses Minecraft's `ServerLevelAccessor` (org/betterx/betternether/world/structures/city/CityStructure.java:130)
- [ ] `CityStructure.cityWithin(3)`: it uses Minecraft's `ServerLevelAccessor` (org/betterx/betternether/world/structures/city/CityStructure.java:134)
- [ ] `CityStructure.intersects(3)`: it uses Minecraft's `ServerLevelAccessor` (org/betterx/betternether/world/structures/city/CityStructure.java:142)
- [ ] `CityStructure.type(0)`: its class isn't translated (it extends Minecraft's `Structure`) (org/betterx/betternether/world/structures/city/CityStructure.java:186)

### `NearbyBastions`

- [x] `NearbyBastions.BASTION` (org/betterx/betternether/world/structures/city/NearbyBastions.java:22)
- [x] `NearbyBastions.NearbyBastions(0)` (org/betterx/betternether/world/structures/city/NearbyBastions.java:24)
- [ ] `NearbyBastions.startsNear(3)`: it uses Minecraft's `GenerationStub` (org/betterx/betternether/world/structures/city/NearbyBastions.java:27)
- [ ] `NearbyBastions.selectedAt(3)`: it uses Selected, which isn't translated (it uses Minecraft's `Holder`) (org/betterx/betternether/world/structures/city/NearbyBastions.java:59)

### `NearbyBastions.Selected`: it uses Minecraft's `Holder`

- [ ] `NearbyBastions.Selected.Selected(2)`: its class isn't translated (it uses Minecraft's `Holder`) (org/betterx/betternether/world/structures/city/NearbyBastions.java:110)
- [ ] `NearbyBastions.Selected.structure(0)`: its class isn't translated (it uses Minecraft's `Holder`) (org/betterx/betternether/world/structures/city/NearbyBastions.java:110)
- [ ] `NearbyBastions.Selected.stub(0)`: its class isn't translated (it uses Minecraft's `Holder`) (org/betterx/betternether/world/structures/city/NearbyBastions.java:110)

### `CityPalette`: it uses Minecraft's `Block`

- [ ] `CityPalette.CityPalette(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:66)
- [ ] `CityPalette.getName(0)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:70)
- [ ] `CityPalette.putBlocks(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:74)
- [ ] `CityPalette.addFoundationBlocks(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:82)
- [ ] `CityPalette.addFoundationSlabs(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:86)
- [ ] `CityPalette.addFoundationStairs(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:90)
- [ ] `CityPalette.addFoundationWalls(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:94)
- [ ] `CityPalette.addRoofBlocks(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:98)
- [ ] `CityPalette.addRoofSlabs(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:102)
- [ ] `CityPalette.addRoofStairs(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:106)
- [ ] `CityPalette.addPlanksBlocks(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:110)
- [ ] `CityPalette.addPlanksSlabs(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:114)
- [ ] `CityPalette.addPlanksStairs(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:118)
- [ ] `CityPalette.addFences(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:122)
- [ ] `CityPalette.addWalls(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:126)
- [ ] `CityPalette.addGates(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:130)
- [ ] `CityPalette.addLogs(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:134)
- [ ] `CityPalette.addBark(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:138)
- [ ] `CityPalette.addStoneBlocks(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:142)
- [ ] `CityPalette.addStoneSlabs(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:146)
- [ ] `CityPalette.addStoneStairs(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:150)
- [ ] `CityPalette.addGlowingBlocks(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:154)
- [ ] `CityPalette.addWallLights(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:158)
- [ ] `CityPalette.addCeilingLights(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:162)
- [ ] `CityPalette.addFloorLights(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:166)
- [ ] `CityPalette.addDoors(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:170)
- [ ] `CityPalette.addTrapdoors(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:174)
- [ ] `CityPalette.addStonePlates(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:178)
- [ ] `CityPalette.addWoodPlates(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:182)
- [ ] `CityPalette.addGlassBlocks(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:186)
- [ ] `CityPalette.addGlassPanes(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:190)
- [ ] `CityPalette.addPotsPanes(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:194)
- [ ] `CityPalette.getRandomBlock(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:198)
- [ ] `CityPalette.getFullState(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:212)
- [ ] `CityPalette.getSlabState(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:220)
- [ ] `CityPalette.getStairState(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:228)
- [ ] `CityPalette.copySlab(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:236)
- [ ] `CityPalette.copyStair(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:241)
- [ ] `CityPalette.copyWall(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:249)
- [ ] `CityPalette.copyFence(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:259)
- [ ] `CityPalette.copyGate(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:268)
- [ ] `CityPalette.copyPillar(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:276)
- [ ] `CityPalette.copyLanternWall(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:280)
- [ ] `CityPalette.copyLanternCeiling(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:290)
- [ ] `CityPalette.copyDoor(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:299)
- [ ] `CityPalette.copyTrapdoor(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:308)
- [ ] `CityPalette.copyPane(2)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:317)
- [ ] `CityPalette.getFoundationBlock(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:326)
- [ ] `CityPalette.getFoundationSlab(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:330)
- [ ] `CityPalette.getFoundationStair(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:334)
- [ ] `CityPalette.getFoundationWall(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:338)
- [ ] `CityPalette.getRoofBlock(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:348)
- [ ] `CityPalette.getRoofSlab(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:352)
- [ ] `CityPalette.getRoofStair(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:356)
- [ ] `CityPalette.getPlanksBlock(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:360)
- [ ] `CityPalette.getPlanksSlab(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:364)
- [ ] `CityPalette.getPlanksStair(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:368)
- [ ] `CityPalette.getFence(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:372)
- [ ] `CityPalette.getWall(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:380)
- [ ] `CityPalette.getGate(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:388)
- [ ] `CityPalette.getLog(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:396)
- [ ] `CityPalette.getBark(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:404)
- [ ] `CityPalette.getStoneBlock(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:412)
- [ ] `CityPalette.getStoneSlab(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:416)
- [ ] `CityPalette.getStoneStair(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:420)
- [ ] `CityPalette.getGlowingBlock(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:424)
- [ ] `CityPalette.getWallLight(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:428)
- [ ] `CityPalette.getCeilingLight(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:438)
- [ ] `CityPalette.getFloorLight(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:442)
- [ ] `CityPalette.getDoor(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:446)
- [ ] `CityPalette.getTrapdoor(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:450)
- [ ] `CityPalette.getWoodenPlate(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:454)
- [ ] `CityPalette.getStonePlate(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:458)
- [ ] `CityPalette.getGlassBlock(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:462)
- [ ] `CityPalette.getGlassPane(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:466)
- [ ] `CityPalette.getPot(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:470)
- [ ] `CityPalette.getPlant(1)`: its class isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/CityPalette.java:474)

### `Palettes`

- [ ] `Palettes.REGISTRY`: it uses CityPalette, which isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/Palettes.java:17)
- [ ] `Palettes.PALETTES`: it uses CityPalette, which isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/Palettes.java:18)
- [ ] `Palettes.EMPTY`: it uses CityPalette, which isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/Palettes.java:19)
- [ ] `Palettes.RED`: it uses CityPalette, which isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/Palettes.java:20)
- [ ] `Palettes.register(1)`: it uses CityPalette, which isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/Palettes.java:61)
- [ ] `Palettes.getPalette(1)`: it uses CityPalette, which isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/Palettes.java:67)
- [ ] `Palettes.getRandom(1)`: it uses CityPalette, which isn't translated (it uses Minecraft's `Block`) (org/betterx/betternether/world/structures/city/palette/Palettes.java:72)

### `StructureAvoidingProcessor`: it implements Minecraft's `StructureProcessor`

- [ ] `StructureAvoidingProcessor.AVOIDED`: it uses Minecraft's `TagKey` (org/betterx/betternether/world/structures/city/StructureAvoidingProcessor.java:24)
- [ ] `StructureAvoidingProcessor.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/structures/city/StructureAvoidingProcessor.java:25)
- [ ] `StructureAvoidingProcessor.StructureAvoidingProcessor(1)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (org/betterx/betternether/world/structures/city/StructureAvoidingProcessor.java:28)
- [ ] `StructureAvoidingProcessor.collect(3)`: it uses Minecraft's `BoundingBox` (org/betterx/betternether/world/structures/city/StructureAvoidingProcessor.java:32)
- [ ] `StructureAvoidingProcessor.isAvoided(2)`: it uses Minecraft's `Registry` (org/betterx/betternether/world/structures/city/StructureAvoidingProcessor.java:48)
- [ ] `StructureAvoidingProcessor.isEmpty(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (org/betterx/betternether/world/structures/city/StructureAvoidingProcessor.java:52)
- [ ] `StructureAvoidingProcessor.claimed(1)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (org/betterx/betternether/world/structures/city/StructureAvoidingProcessor.java:56)
- [ ] `StructureAvoidingProcessor.processBlock(6)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (org/betterx/betternether/world/structures/city/StructureAvoidingProcessor.java:66)
- [ ] `StructureAvoidingProcessor.codec(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (org/betterx/betternether/world/structures/city/StructureAvoidingProcessor.java:72)

### `StructureCityBuilding`: it extends `StructureNBT` (de.ambertation.wover.structure.api)

- [ ] `StructureCityBuilding.AIR`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:23)
- [ ] `StructureCityBuilding.StructureCityBuilding(1)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:32)
- [ ] `StructureCityBuilding.StructureCityBuilding(2)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:37)
- [ ] `StructureCityBuilding.StructureCityBuilding(2)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:43)
- [ ] `StructureCityBuilding.init(0)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:48)
- [ ] `StructureCityBuilding.asBlockPos(1)`: it uses Minecraft's `BlockPos` (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:66)
- [ ] `StructureCityBuilding.getDir(1)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:70)
- [ ] `StructureCityBuilding.getBoungingBox(0)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:81)
- [ ] `StructureCityBuilding.mirrorRotation(1)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:85)
- [ ] `StructureCityBuilding.placeInChunk(4)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:94)
- [ ] `StructureCityBuilding.getEnds(0)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:105)
- [ ] `StructureCityBuilding.getEndsCount(0)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:109)
- [ ] `StructureCityBuilding.getOffsettedPos(1)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:113)
- [ ] `StructureCityBuilding.getPos(1)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:117)
- [ ] `StructureCityBuilding.getRotated(1)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:121)
- [ ] `StructureCityBuilding.getRandomRotated(1)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:151)
- [ ] `StructureCityBuilding.clone(0)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:155)
- [ ] `StructureCityBuilding.rotated(2)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:159)
- [ ] `StructureCityBuilding.getYOffset(0)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:168)
- [ ] `StructureCityBuilding.getRotation(0)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:172)
- [ ] `StructureCityBuilding.getMirror(0)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:176)
- [ ] `StructureCityBuilding.setMirror(1)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:180)
- [ ] `StructureCityBuilding.getBoundingBox(1)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:185)
- [ ] `StructureCityBuilding.setRotation(1)`: its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)) (org/betterx/betternether/world/structures/city/StructureCityBuilding.java:189)

### `IStructure`: it's an interface

- [ ] `IStructure.generate(5)`: it has no body (org/betterx/betternether/world/structures/IStructure.java:8)

### `MegaLavaLakeStructure`: it extends Minecraft's `Structure`

- [x] `MegaLavaLakeStructure.MIN_RADIUS` (org/betterx/betternether/world/structures/lake/MegaLavaLakeStructure.java:23)
- [x] `MegaLavaLakeStructure.MAX_RADIUS` (org/betterx/betternether/world/structures/lake/MegaLavaLakeStructure.java:24)
- [x] `MegaLavaLakeStructure.RIM_PROBE_REACH` (org/betterx/betternether/world/structures/lake/MegaLavaLakeStructure.java:25)
- [x] `MegaLavaLakeStructure.MIN_DEPTH` (org/betterx/betternether/world/structures/lake/MegaLavaLakeStructure.java:26)
- [x] `MegaLavaLakeStructure.MAX_DEPTH` (org/betterx/betternether/world/structures/lake/MegaLavaLakeStructure.java:27)
- [x] `MegaLavaLakeStructure.MAX_RIM_DROP` (org/betterx/betternether/world/structures/lake/MegaLavaLakeStructure.java:28)
- [x] `MegaLavaLakeStructure.REQUIRED_HEADROOM` (org/betterx/betternether/world/structures/lake/MegaLavaLakeStructure.java:29)
- [ ] `MegaLavaLakeStructure.MegaLavaLakeStructure(1)`: its class isn't translated (it extends Minecraft's `Structure`) (org/betterx/betternether/world/structures/lake/MegaLavaLakeStructure.java:31)
- [ ] `MegaLavaLakeStructure.type(0)`: its class isn't translated (it extends Minecraft's `Structure`) (org/betterx/betternether/world/structures/lake/MegaLavaLakeStructure.java:35)
- [ ] `MegaLavaLakeStructure.findGenerationPoint(1)`: its class isn't translated (it extends Minecraft's `Structure`) (org/betterx/betternether/world/structures/lake/MegaLavaLakeStructure.java:39)
- [ ] `MegaLavaLakeStructure.rimColumns(3)`: it uses Minecraft's `BlockPos` (org/betterx/betternether/world/structures/lake/MegaLavaLakeStructure.java:73)

### `CavePiece`: it extends `CustomPiece`

- [ ] `CavePiece.LAVA`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/structures/piece/CavePiece.java:21)
- [ ] `CavePiece.NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betternether/world/structures/piece/CavePiece.java:22)
- [ ] `CavePiece.CavePiece(4)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/CavePiece.java:27)
- [ ] `CavePiece.CavePiece(2)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/CavePiece.java:34)
- [ ] `CavePiece.addAdditionalSaveData(2)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/CavePiece.java:41)
- [ ] `CavePiece.postProcess(7)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/CavePiece.java:46)
- [ ] `CavePiece.makeBoundingBox(2)`: it uses Minecraft's `BoundingBox` (org/betterx/betternether/world/structures/piece/CavePiece.java:82)

### `CityPiece`: it extends `CustomPiece`

- [ ] `CityPiece.CityPiece(4)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/CityPiece.java:32)
- [ ] `CityPiece.CityPiece(2)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/CityPiece.java:41)
- [ ] `CityPiece.addAdditionalSaveData(2)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/CityPiece.java:52)
- [ ] `CityPiece.postProcess(7)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/CityPiece.java:61)

### `CustomPiece`: it extends Minecraft's `StructurePiece`

- [ ] `CustomPiece.CustomPiece(3)`: its class isn't translated (it extends Minecraft's `StructurePiece`) (org/betterx/betternether/world/structures/piece/CustomPiece.java:9)
- [ ] `CustomPiece.CustomPiece(2)`: its class isn't translated (it extends Minecraft's `StructurePiece`) (org/betterx/betternether/world/structures/piece/CustomPiece.java:13)

### `DestructionPiece`: it extends `CustomPiece`

- [ ] `DestructionPiece.DestructionPiece(2)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/DestructionPiece.java:25)
- [ ] `DestructionPiece.DestructionPiece(2)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/DestructionPiece.java:37)
- [ ] `DestructionPiece.addAdditionalSaveData(2)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/DestructionPiece.java:45)
- [ ] `DestructionPiece.postProcess(7)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/DestructionPiece.java:50)
- [ ] `DestructionPiece.makeBoundingBox(0)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/DestructionPiece.java:93)

### `LavaLakePiece`: it extends `CustomPiece`

- [ ] `LavaLakePiece.LAVA`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:31)
- [ ] `LavaLakePiece.MAGMA`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:32)
- [ ] `LavaLakePiece.NETHERRACK`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:33)
- [x] `LavaLakePiece.SHORE_WIDTH` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:34)
- [x] `LavaLakePiece.APRON` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:35)
- [x] `LavaLakePiece.APRON_FILL_DEPTH` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:36)
- [x] `LavaLakePiece.BED_SHELL` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:37)
- [x] `LavaLakePiece.HEADROOM` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:38)
- [x] `LavaLakePiece.OUTLINE_WOBBLE` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:39)
- [x] `LavaLakePiece.WISP_ONE_IN` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:40)
- [x] `LavaLakePiece.MIN_WISP_HEIGHT` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:41)
- [x] `LavaLakePiece.MAX_WISP_HEIGHT` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:42)
- [x] `LavaLakePiece.MOLTEN_AT_WATER` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:43)
- [x] `LavaLakePiece.MOLTEN_FADE` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:44)
- [x] `LavaLakePiece.SCULK_PATCH_THRESHOLD` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:45)
- [x] `LavaLakePiece.MAX_FLOWER_SHARE` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:46)
- [ ] `LavaLakePiece.LavaLakePiece(4)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:54)
- [ ] `LavaLakePiece.LavaLakePiece(5)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:58)
- [ ] `LavaLakePiece.LavaLakePiece(2)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:68)
- [ ] `LavaLakePiece.addAdditionalSaveData(2)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:78)
- [ ] `LavaLakePiece.lobes(2)`: it calls Minecraft's `RandomSource`.create (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:85)
- [ ] `LavaLakePiece.makeBoundingBox(3)`: it uses Minecraft's `BoundingBox` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:103)
- [ ] `LavaLakePiece.postProcess(7)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:116)
- [ ] `LavaLakePiece.shapeAt(2)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:139)
- [ ] `LavaLakePiece.overburden(4)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:157)
- [ ] `LavaLakePiece.isOpen(1)`: it uses Minecraft's `BlockState` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:170)
- [ ] `LavaLakePiece.carveBowl(5)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:174)
- [ ] `LavaLakePiece.clearAbove(4)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:201)
- [ ] `LavaLakePiece.buildShore(6)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:213)
- [ ] `LavaLakePiece.fillDownToGround(5)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:243)
- [ ] `LavaLakePiece.findSurface(6)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:272)
- [ ] `LavaLakePiece.shoreGround(3)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:284)
- [ ] `LavaLakePiece.vegetate(5)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:295)
- [ ] `LavaLakePiece.placeWisp(3)`: its class isn't translated (it extends `CustomPiece`) (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:318)

### `LavaLakePiece.Lobe`

- [x] `LavaLakePiece.Lobe.Lobe(3)` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:340)
- [x] `LavaLakePiece.Lobe.reach(0)` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:341)
- [x] `LavaLakePiece.Lobe.offsetX(0)` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:340)
- [x] `LavaLakePiece.Lobe.offsetZ(0)` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:340)
- [x] `LavaLakePiece.Lobe.radius(0)` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:340)

### `LavaLakePiece.Shape`

- [x] `LavaLakePiece.Shape.Shape(2)` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:346)
- [x] `LavaLakePiece.Shape.t(0)` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:346)
- [x] `LavaLakePiece.Shape.dOut(0)` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java:346)

### `LegacyStructureAnchorTree`: it implements `IStructure` (org.betterx.betternether.world.structures)

- [ ] `LegacyStructureAnchorTree.NOISE`: it creates a OpenSimplexNoise with a constructor that isn't translated (org/betterx/betternether/world/structures/plants/LegacyStructureAnchorTree.java:30)
- [ ] `LegacyStructureAnchorTree.generate(5)`: its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)) (org/betterx/betternether/world/structures/plants/LegacyStructureAnchorTree.java:32)
- [ ] `LegacyStructureAnchorTree.canGenerate(1)`: its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)) (org/betterx/betternether/world/structures/plants/LegacyStructureAnchorTree.java:39)
- [ ] `LegacyStructureAnchorTree.grow(6)`: its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)) (org/betterx/betternether/world/structures/plants/LegacyStructureAnchorTree.java:43)
- [ ] `LegacyStructureAnchorTree.buildBigCircle(8)`: its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)) (org/betterx/betternether/world/structures/plants/LegacyStructureAnchorTree.java:145)
- [ ] `LegacyStructureAnchorTree.buildLine(3)`: its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)) (org/betterx/betternether/world/structures/plants/LegacyStructureAnchorTree.java:162)
- [ ] `LegacyStructureAnchorTree.lerp(3)`: its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)) (org/betterx/betternether/world/structures/plants/LegacyStructureAnchorTree.java:183)
- [ ] `LegacyStructureAnchorTree.lerpCos(4)`: its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)) (org/betterx/betternether/world/structures/plants/LegacyStructureAnchorTree.java:190)
- [ ] `LegacyStructureAnchorTree.lcos(1)`: its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)) (org/betterx/betternether/world/structures/plants/LegacyStructureAnchorTree.java:197)
- [ ] `LegacyStructureAnchorTree.line(5)`: its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)) (org/betterx/betternether/world/structures/plants/LegacyStructureAnchorTree.java:201)
- [ ] `LegacyStructureAnchorTree.cylinder(3)`: its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)) (org/betterx/betternether/world/structures/plants/LegacyStructureAnchorTree.java:218)
- [ ] `LegacyStructureAnchorTree.circleLinesEnds(6)`: its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)) (org/betterx/betternether/world/structures/plants/LegacyStructureAnchorTree.java:239)
- [ ] `LegacyStructureAnchorTree.makeMushroom(4)`: it uses Minecraft's `ServerLevelAccessor` (org/betterx/betternether/world/structures/plants/LegacyStructureAnchorTree.java:257)

### `StructureCaves`: it implements `IStructure`

- [ ] `StructureCaves.StructureCaves(1)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructureCaves.java:18)
- [ ] `StructureCaves.generate(5)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructureCaves.java:26)
- [ ] `StructureCaves.getHeight(2)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructureCaves.java:72)
- [ ] `StructureCaves.getRigid(2)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructureCaves.java:76)
- [ ] `StructureCaves.mix(3)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructureCaves.java:81)
- [ ] `StructureCaves.opSmoothUnion(3)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructureCaves.java:85)
- [ ] `StructureCaves.isInCave(4)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructureCaves.java:91)

### `StructureGeneratorThreadContext`: it uses Minecraft's `MutableBlockPos`

- [ ] `StructureGeneratorThreadContext.clear(0)`: its class isn't translated (it uses Minecraft's `MutableBlockPos`) (org/betterx/betternether/world/structures/StructureGeneratorThreadContext.java:21)

### `StructurePath`: it implements `IStructure`

- [ ] `StructurePath.StructurePath(1)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructurePath.java:27)
- [ ] `StructurePath.generate(5)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructurePath.java:35)
- [ ] `StructurePath.getHeight(2)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructurePath.java:66)
- [ ] `StructurePath.getRigid(2)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructurePath.java:70)
- [ ] `StructurePath.isHeightValid(2)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructurePath.java:76)
- [ ] `StructurePath.makeLantern(2)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructurePath.java:81)
- [ ] `StructurePath.getRoadMaterial(3)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructurePath.java:108)
- [ ] `StructurePath.getSlabMaterial(3)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructurePath.java:112)
- [ ] `StructurePath.needsSlab(2)`: its class isn't translated (it implements `IStructure`) (org/betterx/betternether/world/structures/StructurePath.java:116)

### `Altars`: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)

- [ ] `Altars.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/structures/templates/Altars.java:15)
- [ ] `Altars.Altars(4)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/Altars.java:17)
- [ ] `Altars.Altars(1)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/Altars.java:23)
- [ ] `Altars.type(0)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/Altars.java:49)

### `Gardens`: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)

- [ ] `Gardens.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/structures/templates/Gardens.java:15)
- [ ] `Gardens.Gardens(4)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/Gardens.java:17)
- [ ] `Gardens.Gardens(1)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/Gardens.java:23)
- [ ] `Gardens.type(0)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/Gardens.java:34)

### `GhastHive`: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)

- [ ] `GhastHive.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/structures/templates/GhastHive.java:15)
- [ ] `GhastHive.GhastHive(4)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/GhastHive.java:17)
- [ ] `GhastHive.GhastHive(1)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/GhastHive.java:23)
- [ ] `GhastHive.type(0)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/GhastHive.java:29)

### `JungleTemples`: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)

- [ ] `JungleTemples.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/structures/templates/JungleTemples.java:15)
- [ ] `JungleTemples.JungleTemples(4)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/JungleTemples.java:17)
- [ ] `JungleTemples.JungleTemples(1)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/JungleTemples.java:23)
- [ ] `JungleTemples.type(0)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/JungleTemples.java:39)

### `Pillars`: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)

- [ ] `Pillars.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/structures/templates/Pillars.java:15)
- [ ] `Pillars.Pillars(4)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/Pillars.java:17)
- [ ] `Pillars.Pillars(1)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/Pillars.java:23)
- [ ] `Pillars.type(0)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/Pillars.java:45)

### `Portals`: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)

- [ ] `Portals.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/structures/templates/Portals.java:15)
- [ ] `Portals.Portals(4)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/Portals.java:17)
- [ ] `Portals.Portals(1)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/Portals.java:23)
- [ ] `Portals.type(0)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/Portals.java:34)

### `Pyramids`: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)

- [ ] `Pyramids.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/structures/templates/Pyramids.java:15)
- [ ] `Pyramids.Pyramids(4)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/Pyramids.java:17)
- [ ] `Pyramids.Pyramids(1)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/Pyramids.java:23)
- [ ] `Pyramids.type(0)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/Pyramids.java:41)

### `RespawnPoints`: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)

- [ ] `RespawnPoints.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/structures/templates/RespawnPoints.java:15)
- [ ] `RespawnPoints.RespawnPoints(4)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/RespawnPoints.java:17)
- [ ] `RespawnPoints.RespawnPoints(1)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/RespawnPoints.java:23)
- [ ] `RespawnPoints.type(0)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/RespawnPoints.java:41)

### `SpawnAltarLadder`: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)

- [ ] `SpawnAltarLadder.CODEC`: it uses Minecraft's `MapCodec` (org/betterx/betternether/world/structures/templates/SpawnAltarLadder.java:15)
- [ ] `SpawnAltarLadder.SpawnAltarLadder(4)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/SpawnAltarLadder.java:17)
- [ ] `SpawnAltarLadder.SpawnAltarLadder(1)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/SpawnAltarLadder.java:23)
- [ ] `SpawnAltarLadder.type(0)`: its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)) (org/betterx/betternether/world/structures/templates/SpawnAltarLadder.java:32)

### `LooseWood`

- [x] `LooseWood.LooseWood(0)` (org/betterx/betternether/world/tree/build/LooseWood.java:4)
- [ ] `LooseWood.prune(1)`: `++` inside an expression (org/betterx/betternether/world/tree/build/LooseWood.java:7)
- [ ] `LooseWood.lowestWoodLayer(1)`: it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/LooseWood.java:51)
- [ ] `LooseWood.sweepUnattached(2)`: it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/LooseWood.java:65)

### `TreeBuilder`: it uses TrunkShape, which isn't translated (it's an interface)

- [x] `TreeBuilder.DEBUG` (org/betterx/betternether/world/tree/build/TreeBuilder.java:35)
- [x] `TreeBuilder.BARK_MARGIN` (org/betterx/betternether/world/tree/build/TreeBuilder.java:36)
- [ ] `TreeBuilder.TreeBuilder(1)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:38)
- [ ] `TreeBuilder.create(0)`: it uses Builder, which isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:51)
- [ ] `TreeBuilder.build(2)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:55)
- [ ] `TreeBuilder.build(3)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:59)
- [ ] `TreeBuilder.settleColumnTops(1)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:103)
- [ ] `TreeBuilder.world(4)`: it uses Minecraft's `BlockPos` (org/betterx/betternether/world/tree/build/TreeBuilder.java:120)
- [ ] `TreeBuilder.commit(3)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:124)
- [ ] `TreeBuilder.filterFor(2)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:142)
- [ ] `TreeBuilder.grow(3)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:155)
- [ ] `TreeBuilder.allocate(1)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:160)
- [ ] `TreeBuilder.rasterizeWood(5)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:181)
- [ ] `TreeBuilder.rasterizeFoliage(5)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:207)
- [ ] `TreeBuilder.orderedAnchors(2)`: it uses `Skeleton` (org.betterx.betternether.world.tree.skeleton) (org/betterx/betternether/world/tree/build/TreeBuilder.java:232)
- [ ] `TreeBuilder.primaryAnchor(1)`: it uses `Skeleton` (org.betterx.betternether.world.tree.skeleton) (org/betterx/betternether/world/tree/build/TreeBuilder.java:243)

### `TreeBuilder.Bounds`

- [x] `TreeBuilder.Bounds.include(2)` (org/betterx/betternether/world/tree/build/TreeBuilder.java:264)

### `TreeBuilder.Builder`: it uses TrunkShape, which isn't translated (it's an interface)

- [ ] `TreeBuilder.Builder.trunk(1)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:302)
- [ ] `TreeBuilder.Builder.canopy(1)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:307)
- [ ] `TreeBuilder.Builder.palette(1)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:312)
- [ ] `TreeBuilder.Builder.bark(1)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:317)
- [ ] `TreeBuilder.Builder.repair(1)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:322)
- [ ] `TreeBuilder.Builder.decayBudget(1)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:327)
- [ ] `TreeBuilder.Builder.maxCarves(1)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:332)
- [ ] `TreeBuilder.Builder.pruneLooseWood(1)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:337)
- [ ] `TreeBuilder.Builder.replaceable(1)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:342)
- [ ] `TreeBuilder.Builder.writeFlags(1)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:347)
- [ ] `TreeBuilder.Builder.build(0)`: its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/build/TreeBuilder.java:352)

### `TreeBuilder.CellFilter`: it's an interface

- [ ] `TreeBuilder.CellFilter.canOccupy(3)`: it has no body (org/betterx/betternether/world/tree/build/TreeBuilder.java:369)

### `TreeVoxels`: it uses Minecraft's `BlockState`

- [x] `TreeVoxels.EMPTY` (org/betterx/betternether/world/tree/build/TreeVoxels.java:6)
- [x] `TreeVoxels.LOG` (org/betterx/betternether/world/tree/build/TreeVoxels.java:7)
- [x] `TreeVoxels.LEAF` (org/betterx/betternether/world/tree/build/TreeVoxels.java:8)
- [x] `TreeVoxels.FACE_X` (org/betterx/betternether/world/tree/build/TreeVoxels.java:17)
- [x] `TreeVoxels.FACE_Y` (org/betterx/betternether/world/tree/build/TreeVoxels.java:18)
- [x] `TreeVoxels.FACE_Z` (org/betterx/betternether/world/tree/build/TreeVoxels.java:19)
- [ ] `TreeVoxels.TreeVoxels(6)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:21)
- [ ] `TreeVoxels.minX(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:33)
- [ ] `TreeVoxels.minY(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:37)
- [ ] `TreeVoxels.minZ(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:41)
- [ ] `TreeVoxels.sizeX(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:45)
- [ ] `TreeVoxels.sizeY(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:49)
- [ ] `TreeVoxels.sizeZ(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:53)
- [ ] `TreeVoxels.cellCount(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:57)
- [ ] `TreeVoxels.inBounds(3)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:61)
- [ ] `TreeVoxels.index(3)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:65)
- [ ] `TreeVoxels.localX(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:69)
- [ ] `TreeVoxels.localY(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:73)
- [ ] `TreeVoxels.localZ(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:77)
- [ ] `TreeVoxels.kindAt(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:81)
- [ ] `TreeVoxels.kindAt(3)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:85)
- [ ] `TreeVoxels.stateAt(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:89)
- [ ] `TreeVoxels.set(3)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:93)
- [ ] `TreeVoxels.setLog(4)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:100)
- [ ] `TreeVoxels.setLeafIfEmpty(4)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:108)
- [ ] `TreeVoxels.clear(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:116)
- [ ] `TreeVoxels.isBuried(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:123)
- [ ] `TreeVoxels.countWoodBreakingFoliage(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:142)
- [ ] `TreeVoxels.count(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:171)
- [ ] `TreeVoxels.forEach(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/build/TreeVoxels.java:183)

### `TreeVoxels.CellVisitor`: it's an interface

- [ ] `TreeVoxels.CellVisitor.visit(5)`: it has no body (org/betterx/betternether/world/tree/build/TreeVoxels.java:193)

### `CanopyShape`: it's an interface

- [ ] `CanopyShape.volumeAt(3)`: it has no body (org/betterx/betternether/world/tree/canopy/CanopyShape.java:9)
- [ ] `CanopyShape.reach(0)`: it has no body (org/betterx/betternether/world/tree/canopy/CanopyShape.java:11)

### `GhostCanopy`: it implements `CanopyShape`

- [ ] `GhostCanopy.GhostCanopy(1)`: its class isn't translated (it implements `CanopyShape`) (org/betterx/betternether/world/tree/canopy/GhostCanopy.java:23)
- [x] `GhostCanopy.builder(0)` (org/betterx/betternether/world/tree/canopy/GhostCanopy.java:36)
- [ ] `GhostCanopy.volumeAt(3)`: its class isn't translated (it implements `CanopyShape`) (org/betterx/betternether/world/tree/canopy/GhostCanopy.java:40)
- [ ] `GhostCanopy.reach(0)`: its class isn't translated (it implements `CanopyShape`) (org/betterx/betternether/world/tree/canopy/GhostCanopy.java:65)

### `GhostCanopy.Builder`

- [ ] `GhostCanopy.Builder.radius(1)`: `this` used as a value (org/betterx/betternether/world/tree/canopy/GhostCanopy.java:84)
- [ ] `GhostCanopy.Builder.bodyHeight(1)`: `this` used as a value (org/betterx/betternether/world/tree/canopy/GhostCanopy.java:89)
- [ ] `GhostCanopy.Builder.domeHeight(1)`: `this` used as a value (org/betterx/betternether/world/tree/canopy/GhostCanopy.java:94)
- [ ] `GhostCanopy.Builder.skirt(3)`: `this` used as a value (org/betterx/betternether/world/tree/canopy/GhostCanopy.java:99)
- [ ] `GhostCanopy.Builder.smoothing(1)`: `this` used as a value (org/betterx/betternether/world/tree/canopy/GhostCanopy.java:106)
- [ ] `GhostCanopy.Builder.hemWobble(1)`: `this` used as a value (org/betterx/betternether/world/tree/canopy/GhostCanopy.java:111)
- [ ] `GhostCanopy.Builder.minRadius(1)`: `this` used as a value (org/betterx/betternether/world/tree/canopy/GhostCanopy.java:116)
- [ ] `GhostCanopy.Builder.verticalOffset(1)`: `this` used as a value (org/betterx/betternether/world/tree/canopy/GhostCanopy.java:121)
- [ ] `GhostCanopy.Builder.build(0)`: it uses GhostCanopy, which isn't translated (it implements `CanopyShape`) (org/betterx/betternether/world/tree/canopy/GhostCanopy.java:126)

### `DecayRepair`


### `LeafDecay`

- [x] `LeafDecay.VANILLA_BUDGET` (org/betterx/betternether/world/tree/decay/LeafDecay.java:11)
- [x] `LeafDecay.UNREACHED` (org/betterx/betternether/world/tree/decay/LeafDecay.java:12)
- [x] `LeafDecay.FACE_X` (org/betterx/betternether/world/tree/decay/LeafDecay.java:13)
- [x] `LeafDecay.FACE_Y` (org/betterx/betternether/world/tree/decay/LeafDecay.java:14)
- [x] `LeafDecay.FACE_Z` (org/betterx/betternether/world/tree/decay/LeafDecay.java:15)
- [x] `LeafDecay.LeafDecay(0)` (org/betterx/betternether/world/tree/decay/LeafDecay.java:17)
- [ ] `LeafDecay.solve(2)`: it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/decay/LeafDecay.java:20)
- [ ] `LeafDecay.solve(5)`: it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/decay/LeafDecay.java:24)
- [ ] `LeafDecay.sweep(1)`: it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/decay/LeafDecay.java:49)
- [ ] `LeafDecay.worstReachableLeaf(4)`: it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/decay/LeafDecay.java:86)
- [ ] `LeafDecay.carveTowardsWood(5)`: it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/decay/LeafDecay.java:100)
- [ ] `LeafDecay.stepTowardsWood(3)`: it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/decay/LeafDecay.java:142)
- [ ] `LeafDecay.finish(5)`: it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/decay/LeafDecay.java:158)

### `LeafDecay.BranchFactory`: it's an interface

- [ ] `LeafDecay.BranchFactory.at(3)`: it has no body (org/betterx/betternether/world/tree/decay/LeafDecay.java:196)

### `LeafDecay.Result`

- [x] `LeafDecay.Result.Result(4)` (org/betterx/betternether/world/tree/decay/LeafDecay.java:199)
- [x] `LeafDecay.Result.isClean(0)` (org/betterx/betternether/world/tree/decay/LeafDecay.java:200)
- [x] `LeafDecay.Result.leavesKept(0)` (org/betterx/betternether/world/tree/decay/LeafDecay.java:199)
- [x] `LeafDecay.Result.leavesPruned(0)` (org/betterx/betternether/world/tree/decay/LeafDecay.java:199)
- [x] `LeafDecay.Result.branchCells(0)` (org/betterx/betternether/world/tree/decay/LeafDecay.java:199)
- [x] `LeafDecay.Result.leavesPersist(0)` (org/betterx/betternether/world/tree/decay/LeafDecay.java:199)

### `Crackle`

- [x] `Crackle.Crackle(0)` (org/betterx/betternether/world/tree/math/Crackle.java:6)
- [ ] `Crackle.bark(3)`: it uses Displacement, which isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Crackle.java:9)
- [ ] `Crackle.wobble(3)`: it uses Displacement, which isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Crackle.java:17)

### `Spline`

- [x] `Spline.Spline(0)` (org/betterx/betternether/world/tree/math/Spline.java:10)
- [x] `Spline.line(3)` (org/betterx/betternether/world/tree/math/Spline.java:13)
- [x] `Spline.smooth(2)` (org/betterx/betternether/world/tree/math/Spline.java:25)
- [x] `Spline.catmullRom(5)` (org/betterx/betternether/world/tree/math/Spline.java:47)
- [x] `Spline.catmullRom(7)` (org/betterx/betternether/world/tree/math/Spline.java:55)
- [x] `Spline.jitter(3)` (org/betterx/betternether/world/tree/math/Spline.java:59)

### `Volume`: it's an interface

- [ ] `Volume.distance(3)`: it has no body (org/betterx/betternether/world/tree/math/Volume.java:7)
- [ ] `Volume.contains(3)`: its class isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Volume.java:9)
- [ ] `Volume.union(1)`: its class isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Volume.java:13)
- [ ] `Volume.intersect(1)`: its class isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Volume.java:17)
- [ ] `Volume.subtract(1)`: its class isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Volume.java:21)
- [ ] `Volume.smoothUnion(2)`: its class isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Volume.java:25)
- [ ] `Volume.translate(3)`: its class isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Volume.java:34)
- [ ] `Volume.scale(3)`: its class isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Volume.java:38)
- [ ] `Volume.expand(1)`: its class isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Volume.java:43)
- [ ] `Volume.displace(1)`: its class isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Volume.java:47)

### `Volume.Displacement`: it's an interface

- [ ] `Volume.Displacement.at(3)`: it has no body (org/betterx/betternether/world/tree/math/Volume.java:53)

### `Volumes`

- [x] `Volumes.Volumes(0)` (org/betterx/betternether/world/tree/math/Volumes.java:6)
- [ ] `Volumes.sphere(1)`: it uses Volume, which isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Volumes.java:9)
- [ ] `Volumes.ellipsoid(3)`: it uses Volume, which isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Volumes.java:13)
- [ ] `Volumes.cylinder(2)`: it uses Volume, which isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Volumes.java:31)
- [ ] `Volumes.capsule(4)`: it uses Volume, which isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Volumes.java:40)
- [ ] `Volumes.box(3)`: it uses Volume, which isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Volumes.java:58)
- [ ] `Volumes.below(1)`: it uses Volume, which isn't translated (it's an interface) (org/betterx/betternether/world/tree/math/Volumes.java:70)

### `LeafMarks`: it implements `LeafSource`

- [ ] `LeafMarks.LeafMarks(1)`: its class isn't translated (it implements `LeafSource`) (org/betterx/betternether/world/tree/palette/LeafMarks.java:14)
- [ ] `LeafMarks.of(1)`: it uses Builder, which isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/palette/LeafMarks.java:19)
- [ ] `LeafMarks.at(1)`: its class isn't translated (it implements `LeafSource`) (org/betterx/betternether/world/tree/palette/LeafMarks.java:23)

### `LeafMarks.Builder`: it uses Minecraft's `BlockState`

- [ ] `LeafMarks.Builder.Builder(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/palette/LeafMarks.java:42)
- [ ] `LeafMarks.Builder.mark(2)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/palette/LeafMarks.java:46)
- [ ] `LeafMarks.Builder.mark(3)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/palette/LeafMarks.java:50)
- [ ] `LeafMarks.Builder.markOnPrimaryLobe(2)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/palette/LeafMarks.java:55)
- [ ] `LeafMarks.Builder.eyes(6)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/palette/LeafMarks.java:60)
- [ ] `LeafMarks.Builder.eyesOnPrimaryLobe(6)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/palette/LeafMarks.java:64)
- [ ] `LeafMarks.Builder.addEyes(7)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/palette/LeafMarks.java:68)
- [ ] `LeafMarks.Builder.build(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/palette/LeafMarks.java:83)

### `LeafMarks.Mark`: it uses Volume, which isn't translated (it's an interface)

- [ ] `LeafMarks.Mark.Mark(4)`: its class isn't translated (it uses Volume, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/palette/LeafMarks.java:88)
- [ ] `LeafMarks.Mark.region(0)`: its class isn't translated (it uses Volume, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/palette/LeafMarks.java:88)
- [ ] `LeafMarks.Mark.block(0)`: its class isn't translated (it uses Volume, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/palette/LeafMarks.java:88)
- [ ] `LeafMarks.Mark.chance(0)`: its class isn't translated (it uses Volume, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/palette/LeafMarks.java:88)
- [ ] `LeafMarks.Mark.primaryOnly(0)`: its class isn't translated (it uses Volume, which isn't translated (it's an interface)) (org/betterx/betternether/world/tree/palette/LeafMarks.java:88)

### `LogBands`: it implements `LogSource`

- [x] `LogBands.NOISE_SCALE` (org/betterx/betternether/world/tree/palette/LogBands.java:13)
- [ ] `LogBands.LogBands(1)`: its class isn't translated (it implements `LogSource`) (org/betterx/betternether/world/tree/palette/LogBands.java:20)
- [ ] `LogBands.builder(0)`: it uses Builder, which isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)) (org/betterx/betternether/world/tree/palette/LogBands.java:28)
- [ ] `LogBands.at(1)`: its class isn't translated (it implements `LogSource`) (org/betterx/betternether/world/tree/palette/LogBands.java:32)
- [ ] `LogBands.topOfColumn(1)`: its class isn't translated (it implements `LogSource`) (org/betterx/betternether/world/tree/palette/LogBands.java:75)
- [ ] `LogBands.sameAxis(2)`: its class isn't translated (it implements `LogSource`) (org/betterx/betternether/world/tree/palette/LogBands.java:90)
- [ ] `LogBands.seamLayer(3)`: its class isn't translated (it implements `LogSource`) (org/betterx/betternether/world/tree/palette/LogBands.java:96)
- [ ] `LogBands.choice(3)`: its class isn't translated (it implements `LogSource`) (org/betterx/betternether/world/tree/palette/LogBands.java:102)

### `LogBands.Band`: it uses Minecraft's `BlockState`

- [ ] `LogBands.Band.Band(3)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/palette/LogBands.java:112)
- [ ] `LogBands.Band.upTo(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/palette/LogBands.java:112)
- [ ] `LogBands.Band.block(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/palette/LogBands.java:112)
- [ ] `LogBands.Band.seam(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (org/betterx/betternether/world/tree/palette/LogBands.java:112)

### `LogBands.Builder`: it uses Band, which isn't translated (it uses Minecraft's `BlockState`)

- [ ] `LogBands.Builder.band(2)`: its class isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)) (org/betterx/betternether/world/tree/palette/LogBands.java:122)
- [ ] `LogBands.Builder.seam(2)`: its class isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)) (org/betterx/betternether/world/tree/palette/LogBands.java:127)
- [ ] `LogBands.Builder.top(1)`: its class isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)) (org/betterx/betternether/world/tree/palette/LogBands.java:132)
- [ ] `LogBands.Builder.jitter(2)`: its class isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)) (org/betterx/betternether/world/tree/palette/LogBands.java:137)
- [ ] `LogBands.Builder.seamSeed(1)`: its class isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)) (org/betterx/betternether/world/tree/palette/LogBands.java:143)
- [ ] `LogBands.Builder.applyAxis(1)`: its class isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)) (org/betterx/betternether/world/tree/palette/LogBands.java:148)
- [ ] `LogBands.Builder.build(0)`: its class isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)) (org/betterx/betternether/world/tree/palette/LogBands.java:153)

### `TreePalette`: it's an interface

- [ ] `TreePalette.log(1)`: it has no body (org/betterx/betternether/world/tree/palette/TreePalette.java:10)
- [ ] `TreePalette.leaf(1)`: it has no body (org/betterx/betternether/world/tree/palette/TreePalette.java:12)
- [ ] `TreePalette.topOfColumn(1)`: its class isn't translated (it's an interface) (org/betterx/betternether/world/tree/palette/TreePalette.java:14)
- [ ] `TreePalette.of(2)`: its body couldn't be read: an anonymous class (org/betterx/betternether/world/tree/palette/TreePalette.java:18)

### `TreePalette.LeafQuery`: it uses Minecraft's `BlockPos`

- [ ] `TreePalette.LeafQuery.LeafQuery(6)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/palette/TreePalette.java:37)
- [ ] `TreePalette.LeafQuery.world(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/palette/TreePalette.java:37)
- [ ] `TreePalette.LeafQuery.local(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/palette/TreePalette.java:37)
- [ ] `TreePalette.LeafQuery.anchor(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/palette/TreePalette.java:37)
- [ ] `TreePalette.LeafQuery.toAnchor(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/palette/TreePalette.java:37)
- [ ] `TreePalette.LeafQuery.primary(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/palette/TreePalette.java:37)
- [ ] `TreePalette.LeafQuery.random(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/palette/TreePalette.java:37)

### `TreePalette.LeafSource`: it's an interface

- [ ] `TreePalette.LeafSource.at(1)`: it has no body (org/betterx/betternether/world/tree/palette/TreePalette.java:42)

### `TreePalette.LogQuery`: it uses Minecraft's `BlockPos`

- [ ] `TreePalette.LogQuery.LogQuery(6)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/palette/TreePalette.java:45)
- [ ] `TreePalette.LogQuery.world(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/palette/TreePalette.java:45)
- [ ] `TreePalette.LogQuery.local(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/palette/TreePalette.java:45)
- [ ] `TreePalette.LogQuery.heightAbove(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/palette/TreePalette.java:45)
- [ ] `TreePalette.LogQuery.axis(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/palette/TreePalette.java:45)
- [ ] `TreePalette.LogQuery.carved(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/palette/TreePalette.java:45)
- [ ] `TreePalette.LogQuery.random(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/palette/TreePalette.java:45)

### `TreePalette.LogSource`: it's an interface

- [ ] `TreePalette.LogSource.at(1)`: it has no body (org/betterx/betternether/world/tree/palette/TreePalette.java:50)
- [ ] `TreePalette.LogSource.topOfColumn(1)`: its class isn't translated (it's an interface) (org/betterx/betternether/world/tree/palette/TreePalette.java:52)

### `Anchor`

- [x] `Anchor.Anchor(2)` (org/betterx/betternether/world/tree/skeleton/Anchor.java:5)
- [x] `Anchor.Anchor(1)` (org/betterx/betternether/world/tree/skeleton/Anchor.java:6)
- [x] `Anchor.position(0)` (org/betterx/betternether/world/tree/skeleton/Anchor.java:5)
- [x] `Anchor.scale(0)` (org/betterx/betternether/world/tree/skeleton/Anchor.java:5)

### `ForkingTrunk`: it implements `TrunkShape`

- [ ] `ForkingTrunk.ForkingTrunk(1)`: its class isn't translated (it implements `TrunkShape`) (org/betterx/betternether/world/tree/skeleton/ForkingTrunk.java:24)
- [x] `ForkingTrunk.builder(0)` (org/betterx/betternether/world/tree/skeleton/ForkingTrunk.java:38)
- [ ] `ForkingTrunk.build(2)`: its class isn't translated (it implements `TrunkShape`) (org/betterx/betternether/world/tree/skeleton/ForkingTrunk.java:42)
- [ ] `ForkingTrunk.anchorsAlong(1)`: its class isn't translated (it implements `TrunkShape`) (org/betterx/betternether/world/tree/skeleton/ForkingTrunk.java:78)

### `ForkingTrunk.Builder`

- [ ] `ForkingTrunk.Builder.height(2)`: `this` used as a value (org/betterx/betternether/world/tree/skeleton/ForkingTrunk.java:103)
- [ ] `ForkingTrunk.Builder.forkAt(1)`: `this` used as a value (org/betterx/betternether/world/tree/skeleton/ForkingTrunk.java:109)
- [ ] `ForkingTrunk.Builder.arms(2)`: `this` used as a value (org/betterx/betternether/world/tree/skeleton/ForkingTrunk.java:114)
- [ ] `ForkingTrunk.Builder.spread(1)`: `this` used as a value (org/betterx/betternether/world/tree/skeleton/ForkingTrunk.java:120)
- [ ] `ForkingTrunk.Builder.lean(1)`: `this` used as a value (org/betterx/betternether/world/tree/skeleton/ForkingTrunk.java:125)
- [ ] `ForkingTrunk.Builder.jitter(1)`: `this` used as a value (org/betterx/betternether/world/tree/skeleton/ForkingTrunk.java:130)
- [ ] `ForkingTrunk.Builder.radius(2)`: `this` used as a value (org/betterx/betternether/world/tree/skeleton/ForkingTrunk.java:135)
- [ ] `ForkingTrunk.Builder.anchorsPerArm(1)`: `this` used as a value (org/betterx/betternether/world/tree/skeleton/ForkingTrunk.java:141)
- [ ] `ForkingTrunk.Builder.build(0)`: it uses ForkingTrunk, which isn't translated (it implements `TrunkShape`) (org/betterx/betternether/world/tree/skeleton/ForkingTrunk.java:146)

### `Segment`

- [x] `Segment.Segment(4)` (org/betterx/betternether/world/tree/skeleton/Segment.java:8)
- [ ] `Segment.toVolume(0)`: it uses Volume, which isn't translated (it's an interface) (org/betterx/betternether/world/tree/skeleton/Segment.java:9)
- [x] `Segment.maxRadius(0)` (org/betterx/betternether/world/tree/skeleton/Segment.java:13)
- [ ] `Segment.axis(0)`: it uses Minecraft's `Axis` (org/betterx/betternether/world/tree/skeleton/Segment.java:17)
- [x] `Segment.from(0)` (org/betterx/betternether/world/tree/skeleton/Segment.java:8)
- [x] `Segment.to(0)` (org/betterx/betternether/world/tree/skeleton/Segment.java:8)
- [x] `Segment.radiusFrom(0)` (org/betterx/betternether/world/tree/skeleton/Segment.java:8)
- [x] `Segment.radiusTo(0)` (org/betterx/betternether/world/tree/skeleton/Segment.java:8)

### `TrunkShape`: it's an interface

- [ ] `TrunkShape.build(2)`: it has no body (org/betterx/betternether/world/tree/skeleton/TrunkShape.java:8)

### `TreeSpace`: it uses Minecraft's `BlockPos`

- [ ] `TreeSpace.TreeSpace(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/TreeSpace.java:8)
- [ ] `TreeSpace.of(2)`: it uses TreeSpace, which isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/TreeSpace.java:9)
- [ ] `TreeSpace.unbounded(1)`: it uses TreeSpace, which isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/TreeSpace.java:13)
- [ ] `TreeSpace.fitRadius(3)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/TreeSpace.java:17)
- [ ] `TreeSpace.fitSegment(3)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/TreeSpace.java:21)
- [ ] `TreeSpace.canWrite(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/TreeSpace.java:25)
- [ ] `TreeSpace.origin(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/TreeSpace.java:8)
- [ ] `TreeSpace.zone(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (org/betterx/betternether/world/tree/TreeSpace.java:8)

### `NetherAdvancementDataProvider`: it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)

- [ ] `NetherAdvancementDataProvider.NetherAdvancementDataProvider(2)`: its class isn't translated (it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)) (org/betterx/datagen/betternether/advancements/NetherAdvancementDataProvider.java:40)
- [ ] `NetherAdvancementDataProvider.bootstrap(1)`: its class isn't translated (it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)) (org/betterx/datagen/betternether/advancements/NetherAdvancementDataProvider.java:44)

### `BetterNetherDatagen`: it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api)

- [ ] `BetterNetherDatagen.onInitializeProviders(1)`: its class isn't translated (it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betternether/BetterNetherDatagen.java:46)
- [ ] `BetterNetherDatagen.onBuildRegistry(1)`: its class isn't translated (it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betternether/BetterNetherDatagen.java:93)
- [ ] `BetterNetherDatagen.modCore(0)`: its class isn't translated (it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betternether/BetterNetherDatagen.java:97)

### `NetherBlockDefinitions`: it implements `BlockDefinitionsEntrypoint` (de.ambertation.wover.block.api.classify)

- [ ] `NetherBlockDefinitions.defineBlocks(1)`: its class isn't translated (it implements `BlockDefinitionsEntrypoint` (de.ambertation.wover.block.api.classify)) (org/betterx/datagen/betternether/blocks/NetherBlockDefinitions.java:13)

### `NetherCropBlockDefinitions`

- [x] `NetherCropBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherCropBlockDefinitions.java:23)
- [x] `NetherCropBlockDefinitions.NetherCropBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherCropBlockDefinitions.java:25)
- [ ] `NetherCropBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherCropBlockDefinitions.java:28)
- [ ] `NetherCropBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherCropBlockDefinitions.java:32)

### `NetherDecorBlockDefinitions`

- [x] `NetherDecorBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherDecorBlockDefinitions.java:28)
- [ ] `NetherDecorBlockDefinitions.POT_TAGS`: it uses Minecraft's `TagKey` (org/betterx/datagen/betternether/blocks/NetherDecorBlockDefinitions.java:29)
- [ ] `NetherDecorBlockDefinitions.CINCINNASITE_POT_TAGS`: it uses Minecraft's `TagKey` (org/betterx/datagen/betternether/blocks/NetherDecorBlockDefinitions.java:30)
- [x] `NetherDecorBlockDefinitions.NetherDecorBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherDecorBlockDefinitions.java:32)
- [ ] `NetherDecorBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherDecorBlockDefinitions.java:35)
- [ ] `NetherDecorBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherDecorBlockDefinitions.java:39)

### `NetherFunctionalBlockDefinitions`

- [x] `NetherFunctionalBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherFunctionalBlockDefinitions.java:30)
- [ ] `NetherFunctionalBlockDefinitions.ANVIL_TAGS`: it uses Minecraft's `TagKey` (org/betterx/datagen/betternether/blocks/NetherFunctionalBlockDefinitions.java:31)
- [x] `NetherFunctionalBlockDefinitions.NetherFunctionalBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherFunctionalBlockDefinitions.java:33)
- [ ] `NetherFunctionalBlockDefinitions.furnace(4)`: it's generic (org/betterx/datagen/betternether/blocks/NetherFunctionalBlockDefinitions.java:36)
- [ ] `NetherFunctionalBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherFunctionalBlockDefinitions.java:44)
- [ ] `NetherFunctionalBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherFunctionalBlockDefinitions.java:48)

### `NetherFurnitureBlockDefinitions`

- [x] `NetherFurnitureBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherFurnitureBlockDefinitions.java:22)
- [ ] `NetherFurnitureBlockDefinitions.NO_LOOT`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/datagen/betternether/blocks/NetherFurnitureBlockDefinitions.java:23)
- [x] `NetherFurnitureBlockDefinitions.NetherFurnitureBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherFurnitureBlockDefinitions.java:25)
- [ ] `NetherFurnitureBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherFurnitureBlockDefinitions.java:28)
- [ ] `NetherFurnitureBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherFurnitureBlockDefinitions.java:32)

### `NetherGlassBlockDefinitions`

- [x] `NetherGlassBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherGlassBlockDefinitions.java:15)
- [x] `NetherGlassBlockDefinitions.NetherGlassBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherGlassBlockDefinitions.java:17)
- [ ] `NetherGlassBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherGlassBlockDefinitions.java:20)
- [ ] `NetherGlassBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherGlassBlockDefinitions.java:24)

### `NetherLeavesBlockDefinitions`

- [x] `NetherLeavesBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherLeavesBlockDefinitions.java:23)
- [x] `NetherLeavesBlockDefinitions.NetherLeavesBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherLeavesBlockDefinitions.java:25)
- [ ] `NetherLeavesBlockDefinitions.leaves(5)`: it's generic (org/betterx/datagen/betternether/blocks/NetherLeavesBlockDefinitions.java:28)
- [ ] `NetherLeavesBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherLeavesBlockDefinitions.java:35)
- [ ] `NetherLeavesBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherLeavesBlockDefinitions.java:39)

### `NetherLightBlockDefinitions`

- [x] `NetherLightBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherLightBlockDefinitions.java:32)
- [x] `NetherLightBlockDefinitions.NetherLightBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherLightBlockDefinitions.java:34)
- [ ] `NetherLightBlockDefinitions.fireBowl(7)`: it's generic (org/betterx/datagen/betternether/blocks/NetherLightBlockDefinitions.java:37)
- [ ] `NetherLightBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherLightBlockDefinitions.java:53)
- [ ] `NetherLightBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherLightBlockDefinitions.java:57)

### `NetherMetalBlockDefinitions`

- [x] `NetherMetalBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherMetalBlockDefinitions.java:19)
- [ ] `NetherMetalBlockDefinitions.NO_RECIPE`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/datagen/betternether/blocks/NetherMetalBlockDefinitions.java:20)
- [x] `NetherMetalBlockDefinitions.NetherMetalBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherMetalBlockDefinitions.java:22)
- [ ] `NetherMetalBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherMetalBlockDefinitions.java:25)
- [ ] `NetherMetalBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherMetalBlockDefinitions.java:29)

### `NetherMushroomBlockDefinitions`

- [x] `NetherMushroomBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherMushroomBlockDefinitions.java:27)
- [x] `NetherMushroomBlockDefinitions.NetherMushroomBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherMushroomBlockDefinitions.java:29)
- [ ] `NetherMushroomBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherMushroomBlockDefinitions.java:32)
- [ ] `NetherMushroomBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherMushroomBlockDefinitions.java:36)

### `NetherObsidianBlockDefinitions`

- [x] `NetherObsidianBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherObsidianBlockDefinitions.java:16)
- [x] `NetherObsidianBlockDefinitions.NetherObsidianBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherObsidianBlockDefinitions.java:18)
- [ ] `NetherObsidianBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherObsidianBlockDefinitions.java:21)
- [ ] `NetherObsidianBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherObsidianBlockDefinitions.java:25)

### `NetherOreBlockDefinitions`

- [x] `NetherOreBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherOreBlockDefinitions.java:22)
- [x] `NetherOreBlockDefinitions.NetherOreBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherOreBlockDefinitions.java:24)
- [ ] `NetherOreBlockDefinitions.ore(7)`: it's generic (org/betterx/datagen/betternether/blocks/NetherOreBlockDefinitions.java:27)
- [ ] `NetherOreBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherOreBlockDefinitions.java:38)

### `NetherPlantBlockDefinitions`

- [x] `NetherPlantBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherPlantBlockDefinitions.java:46)
- [ ] `NetherPlantBlockDefinitions.GLIMMERWILD_GRASSES`: it uses `BlockKey` (de.ambertation.wover.block.api.classify) (org/betterx/datagen/betternether/blocks/NetherPlantBlockDefinitions.java:47)
- [x] `NetherPlantBlockDefinitions.NetherPlantBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherPlantBlockDefinitions.java:56)
- [ ] `NetherPlantBlockDefinitions.grass(6)`: it's generic (org/betterx/datagen/betternether/blocks/NetherPlantBlockDefinitions.java:59)
- [ ] `NetherPlantBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherPlantBlockDefinitions.java:71)
- [ ] `NetherPlantBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherPlantBlockDefinitions.java:75)

### `NetherSaplingBlockDefinitions`

- [x] `NetherSaplingBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherSaplingBlockDefinitions.java:24)
- [x] `NetherSaplingBlockDefinitions.NetherSaplingBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherSaplingBlockDefinitions.java:26)
- [ ] `NetherSaplingBlockDefinitions.sapling(5)`: it's generic (org/betterx/datagen/betternether/blocks/NetherSaplingBlockDefinitions.java:29)
- [ ] `NetherSaplingBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherSaplingBlockDefinitions.java:40)
- [ ] `NetherSaplingBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherSaplingBlockDefinitions.java:44)

### `NetherStoneBlockDefinitions`

- [x] `NetherStoneBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherStoneBlockDefinitions.java:36)
- [ ] `NetherStoneBlockDefinitions.NO_RECIPE`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/datagen/betternether/blocks/NetherStoneBlockDefinitions.java:37)
- [ ] `NetherStoneBlockDefinitions.SOUL_SANDSTONE_TERRAIN_TAGS`: it uses Minecraft's `TagKey` (org/betterx/datagen/betternether/blocks/NetherStoneBlockDefinitions.java:38)
- [x] `NetherStoneBlockDefinitions.NetherStoneBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherStoneBlockDefinitions.java:42)
- [ ] `NetherStoneBlockDefinitions.stalactite(4)`: it uses `BlockDefinition` (de.ambertation.wover.block.api) (org/betterx/datagen/betternether/blocks/NetherStoneBlockDefinitions.java:45)
- [ ] `NetherStoneBlockDefinitions.simple2x2(3)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/datagen/betternether/blocks/NetherStoneBlockDefinitions.java:53)
- [ ] `NetherStoneBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherStoneBlockDefinitions.java:57)
- [ ] `NetherStoneBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherStoneBlockDefinitions.java:61)

### `NetherTerrainBlockDefinitions`

- [x] `NetherTerrainBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherTerrainBlockDefinitions.java:42)
- [x] `NetherTerrainBlockDefinitions.FUEL_DESCRIPTION` (org/betterx/datagen/betternether/blocks/NetherTerrainBlockDefinitions.java:43)
- [ ] `NetherTerrainBlockDefinitions.BLEACHED_GLOOMSCULK_TAGS`: it uses Minecraft's `TagKey` (org/betterx/datagen/betternether/blocks/NetherTerrainBlockDefinitions.java:44)
- [ ] `NetherTerrainBlockDefinitions.MOLTEN_GLOOMSCULK_TAGS`: it uses Minecraft's `TagKey` (org/betterx/datagen/betternether/blocks/NetherTerrainBlockDefinitions.java:47)
- [ ] `NetherTerrainBlockDefinitions.VEINED_GLOOMSCULK_TAGS`: it uses Minecraft's `TagKey` (org/betterx/datagen/betternether/blocks/NetherTerrainBlockDefinitions.java:48)
- [ ] `NetherTerrainBlockDefinitions.MYCELIUM_TAGS`: it uses Minecraft's `TagKey` (org/betterx/datagen/betternether/blocks/NetherTerrainBlockDefinitions.java:51)
- [x] `NetherTerrainBlockDefinitions.NetherTerrainBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherTerrainBlockDefinitions.java:55)
- [ ] `NetherTerrainBlockDefinitions.terrain(3)`: it uses `BlockDefinition` (de.ambertation.wover.block.api) (org/betterx/datagen/betternether/blocks/NetherTerrainBlockDefinitions.java:58)
- [ ] `NetherTerrainBlockDefinitions.crystal(2)`: it uses `BlockDefinition` (de.ambertation.wover.block.api) (org/betterx/datagen/betternether/blocks/NetherTerrainBlockDefinitions.java:65)
- [ ] `NetherTerrainBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherTerrainBlockDefinitions.java:75)
- [ ] `NetherTerrainBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherTerrainBlockDefinitions.java:79)

### `NetherVineBlockDefinitions`

- [x] `NetherVineBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherVineBlockDefinitions.java:34)
- [ ] `NetherVineBlockDefinitions.NO_LOOT`: it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/datagen/betternether/blocks/NetherVineBlockDefinitions.java:35)
- [x] `NetherVineBlockDefinitions.NetherVineBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherVineBlockDefinitions.java:37)
- [ ] `NetherVineBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherVineBlockDefinitions.java:40)
- [ ] `NetherVineBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherVineBlockDefinitions.java:44)

### `NetherWallPlantBlockDefinitions`

- [x] `NetherWallPlantBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherWallPlantBlockDefinitions.java:22)
- [x] `NetherWallPlantBlockDefinitions.NetherWallPlantBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherWallPlantBlockDefinitions.java:24)
- [ ] `NetherWallPlantBlockDefinitions.wallPlant(3)`: it uses `BlockDefinition` (de.ambertation.wover.block.api) (org/betterx/datagen/betternether/blocks/NetherWallPlantBlockDefinitions.java:27)
- [ ] `NetherWallPlantBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherWallPlantBlockDefinitions.java:37)
- [ ] `NetherWallPlantBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherWallPlantBlockDefinitions.java:41)

### `NetherWoodBlockDefinitions`

- [x] `NetherWoodBlockDefinitions.defined` (org/betterx/datagen/betternether/blocks/NetherWoodBlockDefinitions.java:28)
- [x] `NetherWoodBlockDefinitions.NetherWoodBlockDefinitions(0)` (org/betterx/datagen/betternether/blocks/NetherWoodBlockDefinitions.java:30)
- [ ] `NetherWoodBlockDefinitions.transitionConversion(2)`: it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify) (org/betterx/datagen/betternether/blocks/NetherWoodBlockDefinitions.java:33)
- [ ] `NetherWoodBlockDefinitions.model(1)`: a raw Supplier (org/betterx/datagen/betternether/blocks/NetherWoodBlockDefinitions.java:37)
- [ ] `NetherWoodBlockDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/blocks/NetherWoodBlockDefinitions.java:41)

### `NetherEnchantmentProvider`: it extends `WoverEnchantmentProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `NetherEnchantmentProvider.NetherEnchantmentProvider(1)`: its class isn't translated (it extends `WoverEnchantmentProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/enchantments/NetherEnchantmentProvider.java:37)
- [ ] `NetherEnchantmentProvider.bootstrap(1)`: its class isn't translated (it extends `WoverEnchantmentProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/enchantments/NetherEnchantmentProvider.java:41)

### `NetherEnchantmentTagProvider`: it extends `ForEnchantments` (de.ambertation.wover.datagen.api.WoverTagProvider)

- [ ] `NetherEnchantmentTagProvider.NetherEnchantmentTagProvider(1)`: its class isn't translated (it extends `ForEnchantments` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betternether/enchantments/NetherEnchantmentTagProvider.java:12)
- [ ] `NetherEnchantmentTagProvider.prepareTags(1)`: its class isn't translated (it extends `ForEnchantments` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betternether/enchantments/NetherEnchantmentTagProvider.java:16)

### `NetherEntityTypeTagProvider`: it extends `ForEntityTypes` (de.ambertation.wover.datagen.api.WoverTagProvider)

- [ ] `NetherEntityTypeTagProvider.NetherEntityTypeTagProvider(1)`: its class isn't translated (it extends `ForEntityTypes` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betternether/entity/NetherEntityTypeTagProvider.java:11)
- [ ] `NetherEntityTypeTagProvider.prepareTags(1)`: its class isn't translated (it extends `ForEntityTypes` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betternether/entity/NetherEntityTypeTagProvider.java:15)

### `GlimmerwildBiomeModificationProvider`: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)

- [ ] `GlimmerwildBiomeModificationProvider.GlimmerwildBiomeModificationProvider(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betternether/glimmerwild/GlimmerwildBiomeModificationProvider.java:12)
- [ ] `GlimmerwildBiomeModificationProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betternether/glimmerwild/GlimmerwildBiomeModificationProvider.java:16)

### `GlimmerwildBlockTagProvider`: it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)

- [ ] `GlimmerwildBlockTagProvider.GlimmerwildBlockTagProvider(1)`: its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betternether/glimmerwild/GlimmerwildBlockTagProvider.java:16)
- [ ] `GlimmerwildBlockTagProvider.shouldAdd(1)`: its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betternether/glimmerwild/GlimmerwildBlockTagProvider.java:20)
- [ ] `GlimmerwildBlockTagProvider.prepareTags(1)`: its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betternether/glimmerwild/GlimmerwildBlockTagProvider.java:24)

### `GlimmerwildFeatureProvider`: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [ ] `GlimmerwildFeatureProvider.GlimmerwildFeatureProvider(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/glimmerwild/GlimmerwildFeatureProvider.java:25)
- [ ] `GlimmerwildFeatureProvider.bootstrapFeatures(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/glimmerwild/GlimmerwildFeatureProvider.java:29)
- [ ] `GlimmerwildFeatureProvider.bootstrapPlaced(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/glimmerwild/GlimmerwildFeatureProvider.java:99)

### `NetherDebugItemDefinitions`

- [x] `NetherDebugItemDefinitions.NetherDebugItemDefinitions(0)` (org/betterx/datagen/betternether/items/NetherDebugItemDefinitions.java:12)
- [ ] `NetherDebugItemDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/items/NetherDebugItemDefinitions.java:15)

### `NetherEquipmentItemDefinitions`

- [x] `NetherEquipmentItemDefinitions.defined` (org/betterx/datagen/betternether/items/NetherEquipmentItemDefinitions.java:9)
- [x] `NetherEquipmentItemDefinitions.NetherEquipmentItemDefinitions(0)` (org/betterx/datagen/betternether/items/NetherEquipmentItemDefinitions.java:11)
- [ ] `NetherEquipmentItemDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/items/NetherEquipmentItemDefinitions.java:14)

### `NetherFoodItemDefinitions`

- [x] `NetherFoodItemDefinitions.defined` (org/betterx/datagen/betternether/items/NetherFoodItemDefinitions.java:8)
- [x] `NetherFoodItemDefinitions.NetherFoodItemDefinitions(0)` (org/betterx/datagen/betternether/items/NetherFoodItemDefinitions.java:10)
- [ ] `NetherFoodItemDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/items/NetherFoodItemDefinitions.java:13)

### `NetherItemDefinitions`

- [x] `NetherItemDefinitions.NetherItemDefinitions(0)` (org/betterx/datagen/betternether/items/NetherItemDefinitions.java:24)
- [ ] `NetherItemDefinitions.registerIngredient(4)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/items/NetherItemDefinitions.java:27)
- [ ] `NetherItemDefinitions.registerTool(3)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/items/NetherItemDefinitions.java:32)
- [ ] `NetherItemDefinitions.registerCompatTool(4)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/items/NetherItemDefinitions.java:36)
- [ ] `NetherItemDefinitions.registerFood(3)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/items/NetherItemDefinitions.java:43)
- [ ] `NetherItemDefinitions.registerFood(4)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/items/NetherItemDefinitions.java:47)
- [ ] `NetherItemDefinitions.registerMedicine(5)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/items/NetherItemDefinitions.java:51)
- [ ] `NetherItemDefinitions.makeEgg(5)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/items/NetherItemDefinitions.java:60)

### `NetherResourceItemDefinitions`

- [x] `NetherResourceItemDefinitions.defined` (org/betterx/datagen/betternether/items/NetherResourceItemDefinitions.java:9)
- [x] `NetherResourceItemDefinitions.NetherResourceItemDefinitions(0)` (org/betterx/datagen/betternether/items/NetherResourceItemDefinitions.java:11)
- [ ] `NetherResourceItemDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/items/NetherResourceItemDefinitions.java:14)

### `NetherSpawnEggDefinitions`

- [x] `NetherSpawnEggDefinitions.defined` (org/betterx/datagen/betternether/items/NetherSpawnEggDefinitions.java:9)
- [x] `NetherSpawnEggDefinitions.NetherSpawnEggDefinitions(0)` (org/betterx/datagen/betternether/items/NetherSpawnEggDefinitions.java:11)
- [ ] `NetherSpawnEggDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/items/NetherSpawnEggDefinitions.java:14)

### `NetherTemplatesDefinitions`

- [x] `NetherTemplatesDefinitions.defined` (org/betterx/datagen/betternether/items/NetherTemplatesDefinitions.java:13)
- [x] `NetherTemplatesDefinitions.NetherTemplatesDefinitions(0)` (org/betterx/datagen/betternether/items/NetherTemplatesDefinitions.java:15)
- [ ] `NetherTemplatesDefinitions.define(1)`: it uses `DefinitionContext` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/items/NetherTemplatesDefinitions.java:18)
- [ ] `NetherTemplatesDefinitions.smithingTemplate(2)`: it uses `ItemDefinition` (de.ambertation.wover.item.api.classify) (org/betterx/datagen/betternether/items/NetherTemplatesDefinitions.java:36)

### `NetherBlockTagDataProvider`: it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)

- [ ] `NetherBlockTagDataProvider.NetherBlockTagDataProvider(1)`: its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betternether/NetherBlockTagDataProvider.java:21)
- [ ] `NetherBlockTagDataProvider.prepareTags(1)`: its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betternether/NetherBlockTagDataProvider.java:25)

### `NetherItemTagDataProvider`: it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider)

- [ ] `NetherItemTagDataProvider.NetherItemTagDataProvider(1)`: its class isn't translated (it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betternether/NetherItemTagDataProvider.java:18)
- [ ] `NetherItemTagDataProvider.prepareTags(1)`: its class isn't translated (it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betternether/NetherItemTagDataProvider.java:22)

### `NetherModelProvider`: it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `NetherModelProvider.HAND_AUTHORED_ROOT`: it calls NetherModelProvider.resolveHandAuthoredRoot, which isn't translated (org/betterx/datagen/betternether/NetherModelProvider.java:36)
- [ ] `NetherModelProvider.resolveHandAuthoredRoot(0)`: it calls `Path` (java.nio.file).of (org/betterx/datagen/betternether/NetherModelProvider.java:38)
- [ ] `NetherModelProvider.shipsHandAuthoredModel(1)`: `null` (Rust has no null: this needs an Option) (org/betterx/datagen/betternether/NetherModelProvider.java:43)
- [ ] `NetherModelProvider.bootstrapItemModels(1)`: its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/NetherModelProvider.java:53)
- [ ] `NetherModelProvider.generateTrimmableArmor(3)`: its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/NetherModelProvider.java:74)
- [ ] `NetherModelProvider.generateTrimmableArmor(4)`: its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/NetherModelProvider.java:90)
- [ ] `NetherModelProvider.bootstrapBlockStateModels(1)`: its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/NetherModelProvider.java:108)
- [ ] `NetherModelProvider.NetherModelProvider(1)`: its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/NetherModelProvider.java:116)

### `FlatLevelPresetsDataProvider`: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)

- [ ] `FlatLevelPresetsDataProvider.BN_FLAT`: it uses Minecraft's `ResourceKey` (org/betterx/datagen/betternether/presets/FlatLevelPresetsDataProvider.java:17)
- [ ] `FlatLevelPresetsDataProvider.FlatLevelPresetsDataProvider(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betternether/presets/FlatLevelPresetsDataProvider.java:19)
- [ ] `FlatLevelPresetsDataProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betternether/presets/FlatLevelPresetsDataProvider.java:23)

### `NetherBlockRecipesProvider`: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `NetherBlockRecipesProvider.NetherBlockRecipesProvider(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherBlockRecipesProvider.java:38)
- [ ] `NetherBlockRecipesProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherBlockRecipesProvider.java:42)
- [ ] `NetherBlockRecipesProvider.registerSmithing(1)`: it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder) (org/betterx/datagen/betternether/recipes/NetherBlockRecipesProvider.java:384)
- [ ] `NetherBlockRecipesProvider.registerSmelting(1)`: it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder) (org/betterx/datagen/betternether/recipes/NetherBlockRecipesProvider.java:399)
- [ ] `NetherBlockRecipesProvider.registerBlasting(1)`: it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder) (org/betterx/datagen/betternether/recipes/NetherBlockRecipesProvider.java:428)
- [ ] `NetherBlockRecipesProvider.registerStoneCutting(1)`: it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder) (org/betterx/datagen/betternether/recipes/NetherBlockRecipesProvider.java:436)
- [ ] `NetherBlockRecipesProvider.withTemplates(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherBlockRecipesProvider.java:837)

### `NetherBrewingRecipes`: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `NetherBrewingRecipes.NetherBrewingRecipes(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherBrewingRecipes.java:28)
- [ ] `NetherBrewingRecipes.bootstrap(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherBrewingRecipes.java:32)
- [ ] `NetherBrewingRecipes.brewingRecipe(5)`: it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder) (org/betterx/datagen/betternether/recipes/NetherBrewingRecipes.java:49)
- [ ] `NetherBrewingRecipes.potionOutput(1)`: it uses Minecraft's `ItemStackTemplate` (org/betterx/datagen/betternether/recipes/NetherBrewingRecipes.java:53)

### `NetherChestLootTableProvider`: it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `NetherChestLootTableProvider.NetherChestLootTableProvider(1)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:36)
- [ ] `NetherChestLootTableProvider.boostrap(2)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:40)
- [ ] `NetherChestLootTableProvider.surpriseItem(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:103)
- [ ] `NetherChestLootTableProvider.ghastHive(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:113)
- [ ] `NetherChestLootTableProvider.books(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:157)
- [ ] `NetherChestLootTableProvider.workstations(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:171)
- [ ] `NetherChestLootTableProvider.armor(4)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:179)
- [ ] `NetherChestLootTableProvider.simpleTemplates(2)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:222)
- [ ] `NetherChestLootTableProvider.flamingTemplate(2)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:237)
- [ ] `NetherChestLootTableProvider.allTemplates(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:244)
- [ ] `NetherChestLootTableProvider.bonusSwords(2)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:253)
- [ ] `NetherChestLootTableProvider.simpleSwords(1)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:270)
- [ ] `NetherChestLootTableProvider.hoes(1)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:298)
- [ ] `NetherChestLootTableProvider.enchantedTools(1)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:326)
- [ ] `NetherChestLootTableProvider.netherObsidian(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:383)
- [ ] `NetherChestLootTableProvider.netherFiller(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:422)
- [ ] `NetherChestLootTableProvider.superBonusOreLoot(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:443)
- [ ] `NetherChestLootTableProvider.bonusOreLoot(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:475)
- [ ] `NetherChestLootTableProvider.simpleOreLoot(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:495)
- [ ] `NetherChestLootTableProvider.simpleCityLoot(0)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java:503)

### `NetherCraftingRecipes`: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `NetherCraftingRecipes.NetherCraftingRecipes(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherCraftingRecipes.java:18)
- [ ] `NetherCraftingRecipes.bootstrap(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherCraftingRecipes.java:22)

### `NetherEntityLootTableProvider`: it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `NetherEntityLootTableProvider.NetherEntityLootTableProvider(1)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherEntityLootTableProvider.java:29)
- [ ] `NetherEntityLootTableProvider.boostrap(2)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherEntityLootTableProvider.java:33)
- [ ] `NetherEntityLootTableProvider.enchantments(1)`: it uses Minecraft's `HolderGetter` (org/betterx/datagen/betternether/recipes/NetherEntityLootTableProvider.java:55)
- [ ] `NetherEntityLootTableProvider.nagaPlayerKill(1)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherEntityLootTableProvider.java:59)
- [ ] `NetherEntityLootTableProvider.killLoot(4)`: its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherEntityLootTableProvider.java:71)

### `NetherItemRecipeProvider`: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `NetherItemRecipeProvider.NetherItemRecipeProvider(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherItemRecipeProvider.java:28)
- [ ] `NetherItemRecipeProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherItemRecipeProvider.java:32)
- [ ] `NetherItemRecipeProvider.registerShapeLess(1)`: it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder) (org/betterx/datagen/betternether/recipes/NetherItemRecipeProvider.java:191)
- [ ] `NetherItemRecipeProvider.registerSmelting(1)`: it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder) (org/betterx/datagen/betternether/recipes/NetherItemRecipeProvider.java:230)

### `NetherLootAdditionProvider`: it extends `WoverLootAdditionProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `NetherLootAdditionProvider.NetherLootAdditionProvider(1)`: its class isn't translated (it extends `WoverLootAdditionProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherLootAdditionProvider.java:27)
- [ ] `NetherLootAdditionProvider.bootstrap(2)`: its class isn't translated (it extends `WoverLootAdditionProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/NetherLootAdditionProvider.java:31)

### `VanillaExcavatorsRecipes`: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `VanillaExcavatorsRecipes.VanillaExcavatorsRecipes(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/VanillaExcavatorsRecipes.java:22)
- [ ] `VanillaExcavatorsRecipes.makeExcavatorRecipe(4)`: it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder) (org/betterx/datagen/betternether/recipes/VanillaExcavatorsRecipes.java:26)
- [ ] `VanillaExcavatorsRecipes.bootstrap(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/VanillaExcavatorsRecipes.java:37)

### `VanillaHammersRecipes`: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)

- [ ] `VanillaHammersRecipes.VanillaHammersRecipes(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/VanillaHammersRecipes.java:23)
- [ ] `VanillaHammersRecipes.makeHammerRecipe(4)`: it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder) (org/betterx/datagen/betternether/recipes/VanillaHammersRecipes.java:27)
- [ ] `VanillaHammersRecipes.bootstrap(1)`: its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)) (org/betterx/datagen/betternether/recipes/VanillaHammersRecipes.java:38)

### `ObjectFeatureDataProvider`: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [ ] `ObjectFeatureDataProvider.ObjectFeatureDataProvider(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/ObjectFeatureDataProvider.java:38)
- [ ] `ObjectFeatureDataProvider.bootstrapFeatures(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/ObjectFeatureDataProvider.java:42)
- [ ] `ObjectFeatureDataProvider.bootstrapPlaced(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/ObjectFeatureDataProvider.java:74)

### `OreFeatureDataProvider`: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [ ] `OreFeatureDataProvider.OreFeatureDataProvider(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/OreFeatureDataProvider.java:20)
- [ ] `OreFeatureDataProvider.bootstrapFeatures(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/OreFeatureDataProvider.java:24)
- [ ] `OreFeatureDataProvider.bootstrapPlaced(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/OreFeatureDataProvider.java:68)

### `PlacedTreeFeatureDataProvider`: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [x] `PlacedTreeFeatureDataProvider.GROVE_SCALE_XZ` (org/betterx/datagen/betternether/worldgen/features/PlacedTreeFeatureDataProvider.java:28)
- [x] `PlacedTreeFeatureDataProvider.GROVE_SCALE_Y` (org/betterx/datagen/betternether/worldgen/features/PlacedTreeFeatureDataProvider.java:29)
- [x] `PlacedTreeFeatureDataProvider.GROVE_CORE` (org/betterx/datagen/betternether/worldgen/features/PlacedTreeFeatureDataProvider.java:30)
- [x] `PlacedTreeFeatureDataProvider.GROVE_EDGE` (org/betterx/datagen/betternether/worldgen/features/PlacedTreeFeatureDataProvider.java:31)
- [ ] `PlacedTreeFeatureDataProvider.PlacedTreeFeatureDataProvider(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/PlacedTreeFeatureDataProvider.java:33)
- [ ] `PlacedTreeFeatureDataProvider.bootstrapFeatures(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/PlacedTreeFeatureDataProvider.java:37)
- [ ] `PlacedTreeFeatureDataProvider.bootstrapPlaced(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/PlacedTreeFeatureDataProvider.java:89)

### `TerrainFeatureDataProvider`: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [ ] `TerrainFeatureDataProvider.TerrainFeatureDataProvider(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/TerrainFeatureDataProvider.java:31)
- [ ] `TerrainFeatureDataProvider.bootstrapFeatures(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/TerrainFeatureDataProvider.java:35)
- [ ] `TerrainFeatureDataProvider.bootstrapPlaced(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/TerrainFeatureDataProvider.java:42)

### `VegetationFeatureDataProvider`: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [ ] `VegetationFeatureDataProvider.VegetationFeatureDataProvider(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/VegetationFeatureDataProvider.java:53)
- [ ] `VegetationFeatureDataProvider.bootstrapFeatures(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/VegetationFeatureDataProvider.java:57)
- [ ] `VegetationFeatureDataProvider.bootstrapPlaced(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/VegetationFeatureDataProvider.java:409)

### `VineFeatureDataProvider`: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [ ] `VineFeatureDataProvider.VineFeatureDataProvider(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/VineFeatureDataProvider.java:35)
- [ ] `VineFeatureDataProvider.bootstrapFeatures(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/VineFeatureDataProvider.java:39)
- [ ] `VineFeatureDataProvider.bootstrapPlaced(1)`: its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/features/VineFeatureDataProvider.java:202)

### `NetherBiomeModificationProvider`: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)

- [ ] `NetherBiomeModificationProvider.NetherBiomeModificationProvider(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betternether/worldgen/NetherBiomeModificationProvider.java:19)
- [ ] `NetherBiomeModificationProvider.bootstrap(1)`: its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)) (org/betterx/datagen/betternether/worldgen/NetherBiomeModificationProvider.java:23)

### `NetherBiomesProvider`: it extends `WoverBiomeProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [ ] `NetherBiomesProvider.NetherBiomesProvider(1)`: its class isn't translated (it extends `WoverBiomeProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/NetherBiomesProvider.java:34)
- [ ] `NetherBiomesProvider.bootstrap(1)`: its class isn't translated (it extends `WoverBiomeProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/NetherBiomesProvider.java:38)
- [ ] `NetherBiomesProvider.register(3)`: it's generic (org/betterx/datagen/betternether/worldgen/NetherBiomesProvider.java:64)

### `NetherBiomeTagProvider`: it extends `ForBiomes` (de.ambertation.wover.datagen.api.WoverTagProvider)

- [ ] `NetherBiomeTagProvider.WAYSTONES_BLACKSTONE_WAYSTONE`: it uses Minecraft's `TagKey` (org/betterx/datagen/betternether/worldgen/NetherBiomeTagProvider.java:14)
- [ ] `NetherBiomeTagProvider.NetherBiomeTagProvider(1)`: its class isn't translated (it extends `ForBiomes` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betternether/worldgen/NetherBiomeTagProvider.java:18)
- [ ] `NetherBiomeTagProvider.prepareTags(1)`: its class isn't translated (it extends `ForBiomes` (de.ambertation.wover.datagen.api.WoverTagProvider)) (org/betterx/datagen/betternether/worldgen/NetherBiomeTagProvider.java:22)

### `StructureDataProvider`: it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)

- [x] `StructureDataProvider.CITY_SPACING` (org/betterx/datagen/betternether/worldgen/StructureDataProvider.java:19)
- [x] `StructureDataProvider.MEGA_LAVA_LAKE_SPACING` (org/betterx/datagen/betternether/worldgen/StructureDataProvider.java:20)
- [ ] `StructureDataProvider.StructureDataProvider(1)`: its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/StructureDataProvider.java:22)
- [ ] `StructureDataProvider.bootstrapSturctures(1)`: its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/StructureDataProvider.java:26)
- [ ] `StructureDataProvider.bootstrapSets(1)`: its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/StructureDataProvider.java:40)
- [ ] `StructureDataProvider.bootstrapPools(1)`: its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/StructureDataProvider.java:53)
- [ ] `StructureDataProvider.bootstrapProcessors(1)`: its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/StructureDataProvider.java:56)
- [ ] `StructureDataProvider.prepareBiomeTags(1)`: its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)) (org/betterx/datagen/betternether/worldgen/StructureDataProvider.java:59)
