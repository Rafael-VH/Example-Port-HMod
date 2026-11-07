# Java translated to Rust

Translated 37 of 1955 methods (1 more is done another way in Horizon) and 209 of 549 static fields to Rust (26 of those call the game, through Horizon's helpers); 1 of 4 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 1603 | `ActionTypeTrigger.trigger(2)`: its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) |
| Uses Minecraft classes (needs Horizon's game API) | 126 | `BackpackBlockEntity.canPlaceSleepingBag(2)`: it uses Minecraft's `BlockPos` |
| Other Java this translator doesn't handle yet | 110 | `HoseEvent.Drink.Callback.onDrink(1)`: it has no body |
| Uses Fabric API | 32 | `AttachmentUtils.registerJoinEquip(0)`: it uses Fabric's `ServerPlayerEvents`.JOIN |
| Calls the game where Horizon has no helper yet | 15 | `BackpackScreen.displayTanksUpgradeWarning(1)`: it calls Minecraft's LocalPlayer.level (no Horizon helper for it yet) |
| Uses other libraries | 13 | `BackpackDynamicModel.DynamicBlockStateModel.addAll(2)`: it uses `Builder` (com.google.common.collect.ImmutableMap) |
| Uses inheritance (extends, implements, instanceof) | 8 | `ShapedBackpackRecipe.getProperColor(1)`: `instanceof` |
| Uses files (java.nio / java.io) | 6 | `BackpackManager.getBackpackFile(3)`: it uses `File` (java.io) |
| Uses Java collections other than List (Map, Set...) | 2 | `SortSelector.getSortTypeComparator(2)`: it uses `Comparator` (java.util) |
| Done another way in Horizon | 1 | `TravelersBackpackClient.onInitializeClient(0)`: done another way in Horizon: it runs as `on_init`, from Horizon's init hook |
| Shares one object between names (Java references) | 1 | `UpgradeSlot.UpgradeSlot(6)`: it stores a second reference to an object (Rust would need it copied or shared) |
| Uses lambdas or method references | 1 | `TravelersBackpackClient.registerCustomModels(0)`: a lambda |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| com/tiviacz/travelersbackpack/init/ModNetwork.java:23 | `join` | not translated: it uses TravelersBackpackConfig.CLIENT, which isn't translated (it uses Client, which isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec))) |
| com/tiviacz/travelersbackpack/TravelersBackpackClient.java:286 | `hud` | not translated: it calls HudOverlay.renderOverlay, which isn't translated |
| com/tiviacz/travelersbackpack/TravelersBackpackClient.java:309 | `command` | not translated: its command tree: a command registration that isn't a lambda |
| com/tiviacz/travelersbackpack/TravelersBackpack.java:61 | `init` | `on_init`, called from the hook, without: the call to `ModItemGroups.registerItemGroup(..)`: it calls ModItemGroups.registerItemGroup, which isn't translated; the call to `ModBlocks.init(..)`: it calls ModBlocks.init, which isn't translated; the call to `ModItems.init(..)`: it calls ModItems.init, which isn't translated; the call to `ModItems.registerItemFluidStorage(..)`: it calls ModItems.registerItemFluidStorage, which isn't translated; the call to `ModBlockEntityTypes.init(..)`: it calls ModBlockEntityTypes.init, which isn't translated; the call to `ModBlockEntityTypes.initSidedStorage(..)`: it calls ModBlockEntityTypes.initSidedStorage, which isn't translated; the call to `ModDataComponents.init(..)`: it calls ModDataComponents.init, which isn't translated; the call to `ModScreenHandlerTypes.init(..)`: it calls ModScreenHandlerTypes.init, which isn't translated; the call to `ModRecipeSerializers.init(..)`: it calls ModRecipeSerializers.init, which isn't translated; the call to `ModNetwork.initServer(..)`: it calls ModNetwork.initServer, which isn't translated; the call to `ModCommands.registerCommands(..)`: it calls ModCommands.registerCommands, which isn't translated; the call to `ModAdvancements.init(..)`: it calls ModAdvancements.init, which isn't translated; the call to `LootHandler.registerListeners(..)`: it calls LootHandler.registerListeners, which isn't translated; the call to `RightClickHandler.registerListeners(..)`: it calls RightClickHandler.registerListeners, which isn't translated; the call to `SleepHandler.registerListener(..)`: it calls SleepHandler.registerListener, which isn't translated; the call to `registerDeathHandler(..)`: it calls TravelersBackpack.registerDeathHandler, which isn't translated; the call to `TravelersBackpackBlock.registerDispenserBehaviour(..)`: it calls TravelersBackpackBlock.registerDispenserBehaviour, which isn't translated; the call to `TickHandler.register(..)`: it calls TickHandler.register, which isn't translated; the call to `RecipeSynchronization.synchronizeRecipeSerializer(..)`: it uses ModRecipeSerializers.BACKPACK_SHAPED, which isn't translated (it uses Minecraft's `RecipeSerializer`); the call to `RecipeSynchronization.synchronizeRecipeSerializer(..)`: it uses ModRecipeSerializers.BACKPACK_UPGRADE, which isn't translated (it uses Minecraft's `RecipeSerializer`); the call to `ModItemGroups.addItemGroup(..)`: it calls ModItemGroups.addItemGroup, which isn't translated; the local `fabricLoader`: it uses Fabric's `FabricLoader`; an assignment: unknown name `fabricLoader`; an assignment: unknown name `fabricLoader`; an assignment: unknown name `fabricLoader`; the call to `CraftingTweaksCompat.registerCraftingTweaksAddition(..)`: it calls CraftingTweaksCompat.registerCraftingTweaksAddition, which isn't translated; the call to `BackpackTrinket.init(..)`: it calls BackpackTrinket.init, which isn't translated; an assignment: unknown name `fabricLoader`; an assignment: unknown name `fabricLoader`; an assignment: unknown name `fabricLoader`; the call to `UniversalGravesCompat.register(..)`: it calls UniversalGravesCompat.register, which isn't translated; an assignment: unknown name `fabricLoader`; the call to `PneumonoGravestonesCompat.register(..)`: it calls PneumonoGravestonesCompat.register, which isn't translated; an assignment: unknown name `fabricLoader`; an assignment: unknown name `fabricLoader`; an assignment: unknown name `fabricLoader`; the call to `VinURLNetwork.register(..)`: it calls VinURLNetwork.register, which isn't translated; an assignment: unknown name `fabricLoader`; an assignment: unknown name `fabricLoader`; the call to `Supporters.fetchSupporters(..)`: it calls Supporters.fetchSupporters, which isn't translated; the call to `EffectFluidRegistry.initEffects(..)`: it calls EffectFluidRegistry.initEffects, which isn't translated; the call to `AttachmentUtils.registerJoinEquip(..)`: it calls AttachmentUtils.registerJoinEquip, which isn't translated |

## Every class


### `ActionTypeTrigger`: it extends Minecraft's `SimpleCriterionTrigger`

- [x] `ActionTypeTrigger.UNDYE_BACKPACK` (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:13)
- [x] `ActionTypeTrigger.CHANGE_SLEEPING_BAG` (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:14)
- [x] `ActionTypeTrigger.USE_SLEEPING_BAG` (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:15)
- [x] `ActionTypeTrigger.REVERT_CUSTOM_BACKPACK` (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:16)
- [x] `ActionTypeTrigger.SWAP_TOOLS` (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:17)
- [x] `ActionTypeTrigger.HOSE_SUCK` (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:18)
- [x] `ActionTypeTrigger.HOSE_SPILL` (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:19)
- [x] `ActionTypeTrigger.HOSE_DRINK` (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:20)
- [x] `ActionTypeTrigger.HOSE_SPILL_POTION` (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:21)
- [x] `ActionTypeTrigger.HOSE_DRINK_POTION` (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:22)
- [ ] `ActionTypeTrigger.trigger(2)`: its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:24)
- [ ] `ActionTypeTrigger.codec(0)`: its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:28)

### `ActionTypeTrigger.TriggerInstance`: it implements Minecraft's `SimpleInstance`

- [ ] `ActionTypeTrigger.TriggerInstance.CODEC`: it uses Minecraft's `Codec` (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:33)
- [ ] `ActionTypeTrigger.TriggerInstance.TriggerInstance(2)`: its class isn't translated (it implements Minecraft's `SimpleInstance`) (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:32)
- [ ] `ActionTypeTrigger.TriggerInstance.test(1)`: its class isn't translated (it implements Minecraft's `SimpleInstance`) (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:41)
- [ ] `ActionTypeTrigger.TriggerInstance.player(0)`: its class isn't translated (it implements Minecraft's `SimpleInstance`) (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:32)
- [ ] `ActionTypeTrigger.TriggerInstance.action(0)`: its class isn't translated (it implements Minecraft's `SimpleInstance`) (com/tiviacz/travelersbackpack/advancements/ActionTypeTrigger.java:32)

### `HoseEvent`: it uses Minecraft's `Player`

- [ ] `HoseEvent.HoseEvent(3)`: its class isn't translated (it uses Minecraft's `Player`) (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:17)
- [ ] `HoseEvent.getPlayer(0)`: its class isn't translated (it uses Minecraft's `Player`) (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:23)
- [ ] `HoseEvent.getLevel(0)`: its class isn't translated (it uses Minecraft's `Player`) (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:27)
- [ ] `HoseEvent.getFluid(0)`: its class isn't translated (it uses Minecraft's `Player`) (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:31)
- [ ] `HoseEvent.isCanceled(0)`: its class isn't translated (it uses Minecraft's `Player`) (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:35)
- [ ] `HoseEvent.cancel(0)`: its class isn't translated (it uses Minecraft's `Player`) (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:39)

### `HoseEvent.Drink`: it extends `HoseEvent`

- [ ] `HoseEvent.Drink.EVENT`: it uses Fabric's `Event` (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:45)
- [ ] `HoseEvent.Drink.Drink(4)`: its class isn't translated (it extends `HoseEvent`) (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:51)
- [ ] `HoseEvent.Drink.getAmount(0)`: its class isn't translated (it extends `HoseEvent`) (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:56)
- [ ] `HoseEvent.Drink.setAmount(1)`: its class isn't translated (it extends `HoseEvent`) (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:60)

### `HoseEvent.Drink.Callback`: it's an interface

- [ ] `HoseEvent.Drink.Callback.onDrink(1)`: it has no body (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:66)

### `HoseEvent.PickUp`: it extends `HoseEvent`

- [ ] `HoseEvent.PickUp.EVENT`: it uses Fabric's `Event` (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:73)
- [ ] `HoseEvent.PickUp.PickUp(5)`: its class isn't translated (it extends `HoseEvent`) (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:79)
- [ ] `HoseEvent.PickUp.getPos(0)`: its class isn't translated (it extends `HoseEvent`) (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:85)
- [ ] `HoseEvent.PickUp.getState(0)`: its class isn't translated (it extends `HoseEvent`) (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:89)
- [ ] `HoseEvent.PickUp.setFluid(1)`: its class isn't translated (it extends `HoseEvent`) (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:93)

### `HoseEvent.PickUp.Callback`: it's an interface

- [ ] `HoseEvent.PickUp.Callback.onPickUp(1)`: it has no body (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:99)

### `HoseEvent.Spill`: it extends `HoseEvent`

- [ ] `HoseEvent.Spill.EVENT`: it uses Fabric's `Event` (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:105)
- [ ] `HoseEvent.Spill.Spill(4)`: its class isn't translated (it extends `HoseEvent`) (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:111)
- [ ] `HoseEvent.Spill.getPos(0)`: its class isn't translated (it extends `HoseEvent`) (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:116)

### `HoseEvent.Spill.Callback`: it's an interface

- [ ] `HoseEvent.Spill.Callback.onSpill(1)`: it has no body (com/tiviacz/travelersbackpack/api/event/HoseEvent.java:122)

### `EffectFluid`: it uses Minecraft's `Fluid`

- [ ] `EffectFluid.EffectFluid(3)`: its class isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/api/fluids/EffectFluid.java:17)
- [ ] `EffectFluid.EffectFluid(3)`: its class isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/api/fluids/EffectFluid.java:21)
- [ ] `EffectFluid.EffectFluid(4)`: its class isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/api/fluids/EffectFluid.java:31)
- [ ] `EffectFluid.getUniqueId(0)`: its class isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/api/fluids/EffectFluid.java:42)
- [ ] `EffectFluid.setEffectID(1)`: its class isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/api/fluids/EffectFluid.java:46)
- [ ] `EffectFluid.getEffectID(0)`: its class isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/api/fluids/EffectFluid.java:50)
- [ ] `EffectFluid.affectDrinker(3)`: it has no body (com/tiviacz/travelersbackpack/api/fluids/EffectFluid.java:54)
- [ ] `EffectFluid.canExecuteEffect(3)`: it has no body (com/tiviacz/travelersbackpack/api/fluids/EffectFluid.java:56)

### `AttachmentUtils`

- [ ] `AttachmentUtils.LOAD_ALL`: a function kept in a variable or field (only parameters can be) (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:28)
- [ ] `AttachmentUtils.NO_ITEMS`: a function kept in a variable or field (only parameters can be) (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:29)
- [ ] `AttachmentUtils.STORAGE_ONLY`: a function kept in a variable or field (only parameters can be) (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:30)
- [ ] `AttachmentUtils.UPGRADES_ONLY`: a function kept in a variable or field (only parameters can be) (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:31)
- [ ] `AttachmentUtils.TOOLS_ONLY`: a function kept in a variable or field (only parameters can be) (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:32)
- [ ] `AttachmentUtils.getAttachment(1)`: it uses BackpackAttachment, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:34)
- [ ] `AttachmentUtils.registerJoinEquip(0)`: it uses Fabric's `ServerPlayerEvents`.JOIN (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:38)
- [ ] `AttachmentUtils.synchronise(1)`: it calls AttachmentUtils.getAttachment, which isn't translated (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:56)
- [ ] `AttachmentUtils.isWearingBackpack(1)`: it calls TravelersBackpack.enableIntegration, which isn't translated (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:60)
- [ ] `AttachmentUtils.getWearingBackpack(1)`: it calls TravelersBackpack.enableIntegration, which isn't translated (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:70)
- [ ] `AttachmentUtils.equipBackpack(2)`: it calls AttachmentUtils.getAttachment, which isn't translated (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:87)
- [ ] `AttachmentUtils.getBackpackWrapper(2)`: it's overloaded with the same number of parameters (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:103)
- [ ] `AttachmentUtils.getBackpackWrapper(3)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:108)
- [ ] `AttachmentUtils.getBackpackWrapperArtificial(1)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:117)
- [ ] `AttachmentUtils.getBackpackWrapper(1)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:122)
- [ ] `AttachmentUtils.getBackpackWrapper(2)`: it's overloaded with the same number of parameters (com/tiviacz/travelersbackpack/attachment/AttachmentUtils.java:127)

### `BackpackAttachment`: it uses Minecraft's `ItemStack`

- [ ] `BackpackAttachment.CODEC`: it uses Minecraft's `Codec` (com/tiviacz/travelersbackpack/attachment/BackpackAttachment.java:17)
- [ ] `BackpackAttachment.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/attachment/BackpackAttachment.java:18)
- [ ] `BackpackAttachment.BackpackAttachment(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/BackpackAttachment.java:23)
- [ ] `BackpackAttachment.hasBackpack(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/BackpackAttachment.java:27)
- [ ] `BackpackAttachment.getBackpack(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/BackpackAttachment.java:31)
- [ ] `BackpackAttachment.equipBackpack(2)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/BackpackAttachment.java:35)
- [ ] `BackpackAttachment.updateBackpack(2)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/BackpackAttachment.java:45)
- [ ] `BackpackAttachment.applyComponents(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/BackpackAttachment.java:54)
- [ ] `BackpackAttachment.removeWearable(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/BackpackAttachment.java:61)
- [ ] `BackpackAttachment.removeWrapper(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/BackpackAttachment.java:65)
- [ ] `BackpackAttachment.remove(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/BackpackAttachment.java:71)
- [ ] `BackpackAttachment.getWrapper(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/BackpackAttachment.java:79)
- [ ] `BackpackAttachment.synchronise(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/BackpackAttachment.java:83)
- [ ] `BackpackAttachment.synchronise(2)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/attachment/BackpackAttachment.java:90)

### `BackpackBlockEntity`: it extends Minecraft's `BlockEntity`

- [x] `BackpackBlockEntity.BACKPACK` (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:53)
- [x] `BackpackBlockEntity.SLEEPING_BAG` (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:54)
- [x] `BackpackBlockEntity.SETTINGS_USER` (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:55)
- [ ] `BackpackBlockEntity.BackpackBlockEntity(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:57)
- [ ] `BackpackBlockEntity.getWrapper(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:61)
- [ ] `BackpackBlockEntity.removeWrapper(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:65)
- [ ] `BackpackBlockEntity.saveAdditional(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:69)
- [ ] `BackpackBlockEntity.loadAdditional(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:75)
- [ ] `BackpackBlockEntity.setBackpack(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:82)
- [ ] `BackpackBlockEntity.setBackpackFromNbt(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:105)
- [ ] `BackpackBlockEntity.setLevel(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:109)
- [ ] `BackpackBlockEntity.writeBackpack(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:116)
- [ ] `BackpackBlockEntity.preRemoveSideEffects(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:123)
- [ ] `BackpackBlockEntity.getBlockDirection(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:130)
- [ ] `BackpackBlockEntity.isSleepingBagDeployed(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:138)
- [ ] `BackpackBlockEntity.setSleepingBagDeployed(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:142)
- [ ] `BackpackBlockEntity.canPlaceSleepingBag(2)`: it uses Minecraft's `BlockPos` (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:148)
- [ ] `BackpackBlockEntity.deploySleepingBag(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:152)
- [ ] `BackpackBlockEntity.removeSleepingBag(2)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:192)
- [ ] `BackpackBlockEntity.isThereSleepingBag(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:215)
- [ ] `BackpackBlockEntity.getProperSleepingBag(1)`: it uses Minecraft's `BlockState` (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:225)
- [ ] `BackpackBlockEntity.collectImplicitComponents(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:247)
- [ ] `BackpackBlockEntity.toItemStack(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:252)
- [ ] `BackpackBlockEntity.notifyBlockUpdate(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:257)
- [ ] `BackpackBlockEntity.tick(4)`: it uses Minecraft's `Level` (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:264)
- [ ] `BackpackBlockEntity.getDisplayName(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:268)
- [ ] `BackpackBlockEntity.getDefaultName(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:272)
- [ ] `BackpackBlockEntity.setSettingsUser(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:276)
- [ ] `BackpackBlockEntity.getSettingsUser(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:281)
- [ ] `BackpackBlockEntity.removeSettingsUser(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:285)
- [ ] `BackpackBlockEntity.canOpenSettings(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:290)
- [ ] `BackpackBlockEntity.getUpdatePacket(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:298)
- [ ] `BackpackBlockEntity.getUpdateTag(1)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:303)
- [ ] `BackpackBlockEntity.createMenu(3)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:309)
- [ ] `BackpackBlockEntity.openBackpack(3)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:320)
- [ ] `BackpackBlockEntity.openSettings(3)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:337)
- [ ] `BackpackBlockEntity.openBackpackFromCommand(3)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:344)
- [ ] `BackpackBlockEntity.shouldCloseCurrentScreen(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:356)
- [ ] `BackpackBlockEntity.saveSettingsExtraData(1)`: it uses SettingsScreenData, which isn't translated (it uses Minecraft's `BlockPos`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:360)
- [ ] `BackpackBlockEntity.saveExtraData(2)`: it uses BlockEntityScreenData, which isn't translated (it uses Minecraft's `BlockPos`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:364)
- [ ] `BackpackBlockEntity.getRenderData(0)`: its class isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:368)

### `BackpackBlockEntity.BackpackRenderData`: it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`)

- [ ] `BackpackBlockEntity.BackpackRenderData.BackpackRenderData(4)`: its class isn't translated (it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`)) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:375)
- [ ] `BackpackBlockEntity.BackpackRenderData.info(0)`: its class isn't translated (it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`)) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:375)
- [ ] `BackpackBlockEntity.BackpackRenderData.dyeColor(0)`: its class isn't translated (it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`)) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:375)
- [ ] `BackpackBlockEntity.BackpackRenderData.isSleepingBagDeployed(0)`: its class isn't translated (it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`)) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:375)
- [ ] `BackpackBlockEntity.BackpackRenderData.sleepingBagColor(0)`: its class isn't translated (it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`)) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:375)

### `BackpackBlockEntity.ExtendedScreen`: it's generic

- [ ] `BackpackBlockEntity.ExtendedScreen.ExtendedScreen(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:378)
- [ ] `BackpackBlockEntity.ExtendedScreen.shouldCloseCurrentScreen(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:379)
- [ ] `BackpackBlockEntity.ExtendedScreen.getScreenOpeningData(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:383)
- [ ] `BackpackBlockEntity.ExtendedScreen.getDisplayName(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:387)
- [ ] `BackpackBlockEntity.ExtendedScreen.createMenu(3)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:391)
- [ ] `BackpackBlockEntity.ExtendedScreen.containerSupplier(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:378)
- [ ] `BackpackBlockEntity.ExtendedScreen.screenOpeningData(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/blockentity/BackpackBlockEntity.java:378)

### `SleepingBagBlock`: it extends Minecraft's `BedBlock`

- [ ] `SleepingBagBlock.PART`: it uses Minecraft's `EnumProperty` (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:51)
- [ ] `SleepingBagBlock.OCCUPIED`: it uses Minecraft's `BooleanProperty` (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:52)
- [ ] `SleepingBagBlock.CAN_DROP`: it uses Minecraft's `BooleanProperty` (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:53)
- [ ] `SleepingBagBlock.SLEEPING_BAG`: it uses Minecraft's `VoxelShape` (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:54)
- [ ] `SleepingBagBlock.SLEEPING_BAG_NORTH`: it uses Minecraft's `VoxelShape` (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:55)
- [ ] `SleepingBagBlock.SLEEPING_BAG_EAST`: it uses Minecraft's `VoxelShape` (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:58)
- [ ] `SleepingBagBlock.SLEEPING_BAG_SOUTH`: it uses Minecraft's `VoxelShape` (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:61)
- [ ] `SleepingBagBlock.SLEEPING_BAG_WEST`: it uses Minecraft's `VoxelShape` (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:64)
- [ ] `SleepingBagBlock.SleepingBagBlock(2)`: its class isn't translated (it extends Minecraft's `BedBlock`) (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:68)
- [ ] `SleepingBagBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `BedBlock`) (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:87)
- [ ] `SleepingBagBlock.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `BedBlock`) (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:106)
- [ ] `SleepingBagBlock.destroyOnUse(4)`: its class isn't translated (it extends Minecraft's `BedBlock`) (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:150)
- [ ] `SleepingBagBlock.kickVillagerOutOfBed(2)`: its class isn't translated (it extends Minecraft's `BedBlock`) (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:166)
- [ ] `SleepingBagBlock.updateShape(8)`: its class isn't translated (it extends Minecraft's `BedBlock`) (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:176)
- [ ] `SleepingBagBlock.getNeighbourDirection(2)`: it uses Minecraft's `Direction` (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:195)
- [ ] `SleepingBagBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `BedBlock`) (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:199)
- [ ] `SleepingBagBlock.playerWillDestroy(4)`: its class isn't translated (it extends Minecraft's `BedBlock`) (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:203)
- [ ] `SleepingBagBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `BedBlock`) (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:215)
- [ ] `SleepingBagBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `BedBlock`) (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:223)
- [ ] `SleepingBagBlock.setPlacedBy(5)`: its class isn't translated (it extends Minecraft's `BedBlock`) (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:227)
- [ ] `SleepingBagBlock.getSeed(2)`: its class isn't translated (it extends Minecraft's `BedBlock`) (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:237)
- [ ] `SleepingBagBlock.getDrops(2)`: its class isn't translated (it extends Minecraft's `BedBlock`) (com/tiviacz/travelersbackpack/block/SleepingBagBlock.java:242)

### `TravelersBackpackBlock`: it extends Minecraft's `Block`

- [ ] `TravelersBackpackBlock.FACING`: it uses Minecraft's `EnumProperty` (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:70)
- [ ] `TravelersBackpackBlock.ALL_DIRECTIONS`: it uses Minecraft's `Direction` (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:71)
- [ ] `TravelersBackpackBlock.BACKPACK_TANKS_SHAPE_NORTH`: it uses Minecraft's `VoxelShape` (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:72)
- [ ] `TravelersBackpackBlock.BACKPACK_TANKS_SHAPE_EAST`: it uses Minecraft's `VoxelShape` (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:84)
- [ ] `TravelersBackpackBlock.BACKPACK_TANKS_SHAPE_SOUTH`: it uses Minecraft's `VoxelShape` (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:96)
- [ ] `TravelersBackpackBlock.BACKPACK_TANKS_SHAPE_WEST`: it uses Minecraft's `VoxelShape` (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:108)
- [ ] `TravelersBackpackBlock.BACKPACK_SHAPE_NORTH`: it uses Minecraft's `VoxelShape` (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:120)
- [ ] `TravelersBackpackBlock.BACKPACK_SHAPE_EAST`: it uses Minecraft's `VoxelShape` (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:130)
- [ ] `TravelersBackpackBlock.BACKPACK_SHAPE_SOUTH`: it uses Minecraft's `VoxelShape` (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:140)
- [ ] `TravelersBackpackBlock.BACKPACK_SHAPE_WEST`: it uses Minecraft's `VoxelShape` (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:150)
- [ ] `TravelersBackpackBlock.TravelersBackpackBlock(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:161)
- [ ] `TravelersBackpackBlock.getRenderShape(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:166)
- [ ] `TravelersBackpackBlock.getShape(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:170)
- [ ] `TravelersBackpackBlock.useWithoutItem(5)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:183)
- [ ] `TravelersBackpackBlock.onExplosionHit(5)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:192)
- [ ] `TravelersBackpackBlock.playerWillDestroy(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:195)
- [ ] `TravelersBackpackBlock.getStateForPlacement(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:214)
- [ ] `TravelersBackpackBlock.createBlockStateDefinition(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:218)
- [ ] `TravelersBackpackBlock.entityInside(6)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:222)
- [ ] `TravelersBackpackBlock.hasAnalogOutputSignal(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:233)
- [ ] `TravelersBackpackBlock.getAnalogOutputSignal(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:237)
- [ ] `TravelersBackpackBlock.getCloneItemStack(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:256)
- [ ] `TravelersBackpackBlock.newBlockEntity(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:265)
- [ ] `TravelersBackpackBlock.getTicker(3)`: it's generic (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:269)
- [ ] `TravelersBackpackBlock.animateTick(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:274)
- [ ] `TravelersBackpackBlock.getSignal(4)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:308)
- [ ] `TravelersBackpackBlock.isSignalSource(1)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:316)
- [ ] `TravelersBackpackBlock.onPlace(5)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:320)
- [ ] `TravelersBackpackBlock.neighborChanged(6)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:328)
- [ ] `TravelersBackpackBlock.tryAbsorbWater(2)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:336)
- [ ] `TravelersBackpackBlock.removeWaterBreadthFirstSearch(3)`: its class isn't translated (it extends Minecraft's `Block`) (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:361)
- [ ] `TravelersBackpackBlock.registerDispenserBehaviour(0)`: it uses Minecraft's `BuiltInRegistries`.ITEM (com/tiviacz/travelersbackpack/block/TravelersBackpackBlock.java:418)

### `BackpackDynamicModel`: it implements Minecraft's `UnbakedModel`

- [ ] `BackpackDynamicModel.BackpackDynamicModel(1)`: its class isn't translated (it implements Minecraft's `UnbakedModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:66)
- [ ] `BackpackDynamicModel.bakeBlockStateModel(3)`: its class isn't translated (it implements Minecraft's `UnbakedModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:70)
- [ ] `BackpackDynamicModel.getTextureSlots(3)`: its class isn't translated (it implements Minecraft's `UnbakedModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:85)
- [ ] `BackpackDynamicModel.resolveDependencies(1)`: its class isn't translated (it implements Minecraft's `UnbakedModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:98)

### `BackpackDynamicModel.DynamicBlockStateModel`: it implements Minecraft's `BlockStateModel`

- [ ] `BackpackDynamicModel.DynamicBlockStateModel.DynamicBlockStateModel(3)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:117)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.emitQuads(6)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:123)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.rebakeSleepingBag(2)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:137)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.addExtras(1)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:184)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.getVector(3)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:210)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.rotate(2)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:216)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.createQuad(10)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:222)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.collectParts(2)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:263)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.collectParts(1)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:266)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.addBaseBackpack(1)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:275)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.addTanks(1)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:284)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.addFluids(2)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:291)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.addFluid(5)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:307)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.addSleepingBag(1)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:424)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.addAll(2)`: it uses `Builder` (com.google.common.collect.ImmutableMap) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:439)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.getQuads(0)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:455)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.particleMaterial(0)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:471)
- [ ] `BackpackDynamicModel.DynamicBlockStateModel.materialFlags(0)`: its class isn't translated (it implements Minecraft's `BlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:475)

### `BackpackDynamicModel.Loader`: it implements Fabric's `UnbakedModelDeserializer`

- [ ] `BackpackDynamicModel.Loader.INSTANCE`: it uses Loader, which isn't translated (it implements Fabric's `UnbakedModelDeserializer`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:481)
- [ ] `BackpackDynamicModel.Loader.deserialize(2)`: its class isn't translated (it implements Fabric's `UnbakedModelDeserializer`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:483)
- [ ] `BackpackDynamicModel.Loader.addPartModel(3)`: its class isn't translated (it implements Fabric's `UnbakedModelDeserializer`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:509)

### `BackpackDynamicModel.Loader.PartModel`: it implements Minecraft's `UnbakedModel`

- [ ] `BackpackDynamicModel.Loader.PartModel.PartModel(2)`: its class isn't translated (it implements Minecraft's `UnbakedModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:518)
- [ ] `BackpackDynamicModel.Loader.PartModel.ambientOcclusion(0)`: its class isn't translated (it implements Minecraft's `UnbakedModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:519)
- [ ] `BackpackDynamicModel.Loader.PartModel.transforms(0)`: its class isn't translated (it implements Minecraft's `UnbakedModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:523)
- [ ] `BackpackDynamicModel.Loader.PartModel.textureSlots(0)`: its class isn't translated (it implements Minecraft's `UnbakedModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:527)
- [ ] `BackpackDynamicModel.Loader.PartModel.parent(0)`: its class isn't translated (it implements Minecraft's `UnbakedModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:518)
- [ ] `BackpackDynamicModel.Loader.PartModel.textures(0)`: its class isn't translated (it implements Minecraft's `UnbakedModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:518)

### `BackpackDynamicModel.ModelParts`


### `BackpackDynamicModel.UnbakedBlockStateModel`: it implements Fabric's `CustomUnbakedBlockStateModel`

- [ ] `BackpackDynamicModel.UnbakedBlockStateModel.CODEC`: it uses Minecraft's `MapCodec` (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:549)
- [x] `BackpackDynamicModel.UnbakedBlockStateModel.ID` (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:553)
- [ ] `BackpackDynamicModel.UnbakedBlockStateModel.UnbakedBlockStateModel(1)`: its class isn't translated (it implements Fabric's `CustomUnbakedBlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:548)
- [ ] `BackpackDynamicModel.UnbakedBlockStateModel.bake(1)`: its class isn't translated (it implements Fabric's `CustomUnbakedBlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:555)
- [ ] `BackpackDynamicModel.UnbakedBlockStateModel.resolveDependencies(1)`: its class isn't translated (it implements Fabric's `CustomUnbakedBlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:564)
- [ ] `BackpackDynamicModel.UnbakedBlockStateModel.codec(0)`: its class isn't translated (it implements Fabric's `CustomUnbakedBlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:568)
- [ ] `BackpackDynamicModel.UnbakedBlockStateModel.variant(0)`: its class isn't translated (it implements Fabric's `CustomUnbakedBlockStateModel`) (com/tiviacz/travelersbackpack/client/model/BackpackDynamicModel.java:548)

### `BackpackItemModel`: it implements Minecraft's `ItemModel`

- [ ] `BackpackItemModel.ITEM_TRANSFORMS`: it uses Minecraft's `ItemTransforms` (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java:44)
- [ ] `BackpackItemModel.createItemTransforms(0)`: it uses Minecraft's `ItemTransforms` (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java:49)
- [ ] `BackpackItemModel.BackpackItemModel(2)`: its class isn't translated (it implements Minecraft's `ItemModel`) (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java:63)
- [ ] `BackpackItemModel.update(7)`: its class isn't translated (it implements Minecraft's `ItemModel`) (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java:69)
- [ ] `BackpackItemModel.setProperties(2)`: its class isn't translated (it implements Minecraft's `ItemModel`) (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java:111)

### `BackpackItemModel.SpecialRenderer`: it implements Minecraft's `NoDataSpecialModelRenderer`

- [ ] `BackpackItemModel.SpecialRenderer.setModelRenderParameters(2)`: its class isn't translated (it implements Minecraft's `NoDataSpecialModelRenderer`) (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java:142)
- [ ] `BackpackItemModel.SpecialRenderer.submit(6)`: its class isn't translated (it implements Minecraft's `NoDataSpecialModelRenderer`) (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java:147)
- [ ] `BackpackItemModel.SpecialRenderer.getExtents(1)`: its class isn't translated (it implements Minecraft's `NoDataSpecialModelRenderer`) (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java:160)

### `BackpackItemModel.Unbaked`: it implements `Unbaked`

- [ ] `BackpackItemModel.Unbaked.MAP_CODEC`: it uses Minecraft's `MapCodec` (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java:165)
- [ ] `BackpackItemModel.Unbaked.Unbaked(2)`: its class isn't translated (it implements `Unbaked`) (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java:164)
- [ ] `BackpackItemModel.Unbaked.type(0)`: its class isn't translated (it implements `Unbaked`) (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java:173)
- [ ] `BackpackItemModel.Unbaked.bake(2)`: its class isn't translated (it implements `Unbaked`) (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java:177)
- [ ] `BackpackItemModel.Unbaked.resolveDependencies(1)`: its class isn't translated (it implements `Unbaked`) (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java:186)
- [ ] `BackpackItemModel.Unbaked.base(0)`: its class isn't translated (it implements `Unbaked`) (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java:164)
- [ ] `BackpackItemModel.Unbaked.tintSources(0)`: its class isn't translated (it implements `Unbaked`) (com/tiviacz/travelersbackpack/client/model/BackpackItemModel.java:164)

### `BackpackModel`: it extends `BackpackModelPart`

- [ ] `BackpackModel.render(6)`: its class isn't translated (it extends `BackpackModelPart`) (com/tiviacz/travelersbackpack/client/model/BackpackModel.java:17)

### `BackpackModelPart`

- [ ] `BackpackModelPart.copyFrom(1)`: it uses Minecraft's `ModelPart` (com/tiviacz/travelersbackpack/client/model/BackpackModelPart.java:18)
- [ ] `BackpackModelPart.translateAndRotate(1)`: PoseStack.rotate (com/tiviacz/travelersbackpack/client/model/BackpackModelPart.java:30)

### `StackModelPart`: it extends `BackpackModelPart`

- [ ] `StackModelPart.prepare(1)`: its class isn't translated (it extends `BackpackModelPart`) (com/tiviacz/travelersbackpack/client/model/StackModelPart.java:23)
- [ ] `StackModelPart.render(5)`: its class isn't translated (it extends `BackpackModelPart`) (com/tiviacz/travelersbackpack/client/model/StackModelPart.java:29)
- [ ] `StackModelPart.render(5)`: its class isn't translated (it extends `BackpackModelPart`) (com/tiviacz/travelersbackpack/client/model/StackModelPart.java:35)
- [ ] `StackModelPart.getDisplayContext(1)`: its class isn't translated (it extends `BackpackModelPart`) (com/tiviacz/travelersbackpack/client/model/StackModelPart.java:72)

### `StarModelReloadListener`: it implements Fabric's `SimpleSynchronousResourceReloadListener`

- [ ] `StarModelReloadListener.INSTANCE`: it uses StarModelReloadListener, which isn't translated (it implements Fabric's `SimpleSynchronousResourceReloadListener`) (com/tiviacz/travelersbackpack/client/model/StarModelReloadListener.java:14)
- [x] `StarModelReloadListener.ID` (com/tiviacz/travelersbackpack/client/model/StarModelReloadListener.java:15)
- [ ] `StarModelReloadListener.getStarModel(0)`: its class isn't translated (it implements Fabric's `SimpleSynchronousResourceReloadListener`) (com/tiviacz/travelersbackpack/client/model/StarModelReloadListener.java:18)
- [ ] `StarModelReloadListener.onResourceManagerReload(1)`: its class isn't translated (it implements Fabric's `SimpleSynchronousResourceReloadListener`) (com/tiviacz/travelersbackpack/client/model/StarModelReloadListener.java:22)
- [ ] `StarModelReloadListener.getFabricId(0)`: its class isn't translated (it implements Fabric's `SimpleSynchronousResourceReloadListener`) (com/tiviacz/travelersbackpack/client/model/StarModelReloadListener.java:26)
- [ ] `StarModelReloadListener.getFabricDependencies(0)`: its class isn't translated (it implements Fabric's `SimpleSynchronousResourceReloadListener`) (com/tiviacz/travelersbackpack/client/model/StarModelReloadListener.java:30)

### `SupporterBadgeModel`

- [x] `SupporterBadgeModel.IDENTITY_MATRIX4FC` (com/tiviacz/travelersbackpack/client/model/SupporterBadgeModel.java:28)
- [ ] `SupporterBadgeModel.copyFrom(1)`: it uses Minecraft's `ModelPart` (com/tiviacz/travelersbackpack/client/model/SupporterBadgeModel.java:30)
- [ ] `SupporterBadgeModel.translateAndRotate(1)`: PoseStack.rotate (com/tiviacz/travelersbackpack/client/model/SupporterBadgeModel.java:42)
- [ ] `SupporterBadgeModel.render(3)`: it uses Minecraft's `SubmitNodeCollector` (com/tiviacz/travelersbackpack/client/model/SupporterBadgeModel.java:53)
- [ ] `SupporterBadgeModel.renderModel(4)`: it uses Minecraft's `SubmitNodeCollector` (com/tiviacz/travelersbackpack/client/model/SupporterBadgeModel.java:64)

### `BackpackEntityLayer`: it extends Minecraft's `RenderLayer`

- [ ] `BackpackEntityLayer.BackpackEntityLayer(1)`: its class isn't translated (it extends Minecraft's `RenderLayer`) (com/tiviacz/travelersbackpack/client/renderer/BackpackEntityLayer.java:21)
- [ ] `BackpackEntityLayer.submit(6)`: its class isn't translated (it extends Minecraft's `RenderLayer`) (com/tiviacz/travelersbackpack/client/renderer/BackpackEntityLayer.java:25)

### `BackpackLayer`: it extends Minecraft's `RenderLayer`

- [ ] `BackpackLayer.BACKPACK_MODEL`: it uses BackpackModel, which isn't translated (it extends `BackpackModelPart`) (com/tiviacz/travelersbackpack/client/renderer/BackpackLayer.java:27)
- [ ] `BackpackLayer.BackpackLayer(1)`: its class isn't translated (it extends Minecraft's `RenderLayer`) (com/tiviacz/travelersbackpack/client/renderer/BackpackLayer.java:31)
- [ ] `BackpackLayer.submit(6)`: its class isn't translated (it extends Minecraft's `RenderLayer`) (com/tiviacz/travelersbackpack/client/renderer/BackpackLayer.java:35)
- [ ] `BackpackLayer.renderBackpackLayer(8)`: it uses Minecraft's `HumanoidModel` (com/tiviacz/travelersbackpack/client/renderer/BackpackLayer.java:48)
- [ ] `BackpackLayer.alignModel(4)`: it uses Minecraft's `HumanoidModel` (com/tiviacz/travelersbackpack/client/renderer/BackpackLayer.java:73)

### `HoseSpecialRenderer`: it implements Minecraft's `SelectItemModelProperty`

- [ ] `HoseSpecialRenderer.TYPE`: it uses Minecraft's `Type` (com/tiviacz/travelersbackpack/client/renderer/HoseSpecialRenderer.java:21)
- [ ] `HoseSpecialRenderer.HoseSpecialRenderer(0)`: its class isn't translated (it implements Minecraft's `SelectItemModelProperty`) (com/tiviacz/travelersbackpack/client/renderer/HoseSpecialRenderer.java:20)
- [ ] `HoseSpecialRenderer.register(0)`: it uses Minecraft's `SelectItemModelProperties`.ID_MAPPER (com/tiviacz/travelersbackpack/client/renderer/HoseSpecialRenderer.java:23)
- [ ] `HoseSpecialRenderer.get(5)`: its class isn't translated (it implements Minecraft's `SelectItemModelProperty`) (com/tiviacz/travelersbackpack/client/renderer/HoseSpecialRenderer.java:27)
- [ ] `HoseSpecialRenderer.valueCodec(0)`: its class isn't translated (it implements Minecraft's `SelectItemModelProperty`) (com/tiviacz/travelersbackpack/client/renderer/HoseSpecialRenderer.java:32)
- [ ] `HoseSpecialRenderer.type(0)`: its class isn't translated (it implements Minecraft's `SelectItemModelProperty`) (com/tiviacz/travelersbackpack/client/renderer/HoseSpecialRenderer.java:36)

### `AbstractBackpackScreen`: it's generic

- [x] `AbstractBackpackScreen.BACKGROUND_11` (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:26)
- [x] `AbstractBackpackScreen.BACKGROUND_9` (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:27)
- [x] `AbstractBackpackScreen.SLOTS` (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:28)
- [x] `AbstractBackpackScreen.TANKS` (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:29)
- [x] `AbstractBackpackScreen.ICONS` (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:30)
- [x] `AbstractBackpackScreen.TABS` (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:31)
- [x] `AbstractBackpackScreen.TOP_BAR_OFFSET` (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:32)
- [x] `AbstractBackpackScreen.HEIGHT_WITHOUT_STORAGE` (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:40)
- [ ] `AbstractBackpackScreen.AbstractBackpackScreen(3)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:45)
- [ ] `AbstractBackpackScreen.getWrapper(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:51)
- [ ] `AbstractBackpackScreen.getScreenPlayer(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:55)
- [ ] `AbstractBackpackScreen.getScreenFont(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:59)
- [ ] `AbstractBackpackScreen.getHoveredSlot(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:63)
- [ ] `AbstractBackpackScreen.updateDimensions(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:67)
- [ ] `AbstractBackpackScreen.updateBackpackSlotsPosition(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:80)
- [ ] `AbstractBackpackScreen.updatePlayerSlotsPosition(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:118)
- [ ] `AbstractBackpackScreen.calculateSlotHeight(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:134)
- [ ] `AbstractBackpackScreen.calculateSlotHeight(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:138)
- [ ] `AbstractBackpackScreen.getRows(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:144)
- [ ] `AbstractBackpackScreen.getSlotsInRow(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:148)
- [ ] `AbstractBackpackScreen.setScrollAmount(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:152)
- [ ] `AbstractBackpackScreen.getMaxScrollAmount(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:156)
- [ ] `AbstractBackpackScreen.renderInventoryBackground(6)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:160)
- [ ] `AbstractBackpackScreen.renderSlots(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:178)
- [ ] `AbstractBackpackScreen.renderLockedBackpackSlot(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:202)
- [ ] `AbstractBackpackScreen.extractBackground(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:212)
- [ ] `AbstractBackpackScreen.renderScreen(6)`: it has no body (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:221)
- [ ] `AbstractBackpackScreen.drawUnsortableSlots(1)`: it has no body (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:223)
- [ ] `AbstractBackpackScreen.drawMemorySlots(1)`: it has no body (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:225)
- [ ] `AbstractBackpackScreen.mouseScrolled(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:227)
- [ ] `AbstractBackpackScreen.mouseDragged(3)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:237)
- [ ] `AbstractBackpackScreen.mouseClicked(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:247)
- [ ] `AbstractBackpackScreen.sendDataToServer(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:256)
- [ ] `AbstractBackpackScreen.playUIClickSound(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:259)
- [ ] `AbstractBackpackScreen.getGuiLeft(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:274)
- [ ] `AbstractBackpackScreen.getGuiTop(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:278)
- [ ] `AbstractBackpackScreen.getImageWidth(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:282)
- [ ] `AbstractBackpackScreen.getImageHeight(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/AbstractBackpackScreen.java:286)

### `BackpackScreen`: it extends `AbstractBackpackScreen`

- [ ] `BackpackScreen.BackpackScreen(3)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:66)
- [ ] `BackpackScreen.init(0)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:74)
- [ ] `BackpackScreen.resize(2)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:80)
- [ ] `BackpackScreen.containerTick(0)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:88)
- [ ] `BackpackScreen.recalculate(0)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:95)
- [ ] `BackpackScreen.getMiddleBar(0)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:128)
- [ ] `BackpackScreen.getWidthAdditions(0)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:132)
- [ ] `BackpackScreen.updateScreen(1)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:145)
- [ ] `BackpackScreen.renderScreen(6)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:155)
- [ ] `BackpackScreen.renderUpgradeSlots(3)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:215)
- [ ] `BackpackScreen.initializeUpgradeSlots(0)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:221)
- [ ] `BackpackScreen.addRenderableWidget(1)`: it's generic (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:234)
- [ ] `BackpackScreen.initWidgets(0)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:242)
- [ ] `BackpackScreen.initButtons(0)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:278)
- [ ] `BackpackScreen.extractRenderState(4)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:324)
- [ ] `BackpackScreen.extractTooltip(3)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:332)
- [ ] `BackpackScreen.drawUnsortableSlots(1)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:343)
- [ ] `BackpackScreen.drawMemorySlots(1)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:365)
- [ ] `BackpackScreen.hasClickedOutside(4)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:392)
- [ ] `BackpackScreen.mouseClicked(2)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:406)
- [ ] `BackpackScreen.mouseScrolled(4)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:422)
- [ ] `BackpackScreen.keyPressed(1)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:436)
- [ ] `BackpackScreen.addCompatWidget(1)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:455)
- [ ] `BackpackScreen.removeCompatWidget(1)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:459)
- [ ] `BackpackScreen.displayTanksUpgradeWarning(1)`: it calls Minecraft's LocalPlayer.level (no Horizon helper for it yet) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:463)
- [ ] `BackpackScreen.renderFluidWarningTooltip(3)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:469)
- [ ] `BackpackScreen.createFluidWarning(2)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackScreen.java:492)

### `BackpackSettingsScreen`: it extends `AbstractBackpackScreen`

- [ ] `BackpackSettingsScreen.BackpackSettingsScreen(3)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:47)
- [ ] `BackpackSettingsScreen.sendDataToServer(0)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:60)
- [ ] `BackpackSettingsScreen.init(0)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:66)
- [ ] `BackpackSettingsScreen.resize(2)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:71)
- [ ] `BackpackSettingsScreen.recalculate(0)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:81)
- [ ] `BackpackSettingsScreen.renderScreen(6)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:108)
- [ ] `BackpackSettingsScreen.initWidgets(0)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:124)
- [ ] `BackpackSettingsScreen.closeTabs(1)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:153)
- [ ] `BackpackSettingsScreen.updateWidgetsPosition(1)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:160)
- [ ] `BackpackSettingsScreen.extractRenderState(4)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:178)
- [ ] `BackpackSettingsScreen.extractTooltip(3)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:183)
- [ ] `BackpackSettingsScreen.drawUnsortableSlots(1)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:188)
- [ ] `BackpackSettingsScreen.drawMemorySlots(1)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:227)
- [ ] `BackpackSettingsScreen.slotClicked(4)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:304)
- [ ] `BackpackSettingsScreen.mouseDragged(3)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:310)
- [ ] `BackpackSettingsScreen.selectSlots(2)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:317)
- [ ] `BackpackSettingsScreen.selectSlotsIndex(4)`: it uses Minecraft's `Slot` (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:343)
- [ ] `BackpackSettingsScreen.onClose(0)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:359)
- [ ] `BackpackSettingsScreen.keyPressed(1)`: its class isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/client/screens/BackpackSettingsScreen.java:364)

### `AbilitySliderButton`: it extends `Button`

- [ ] `AbilitySliderButton.AbilitySliderButton(3)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/AbilitySliderButton.java:21)
- [ ] `AbilitySliderButton.render(4)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/AbilitySliderButton.java:26)
- [ ] `AbilitySliderButton.drawButton(4)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/AbilitySliderButton.java:35)
- [ ] `AbilitySliderButton.renderTooltip(3)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/AbilitySliderButton.java:43)
- [ ] `AbilitySliderButton.mouseClicked(2)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/AbilitySliderButton.java:74)

### `Button`: it implements `IButton`

- [ ] `Button.Button(5)`: its class isn't translated (it implements `IButton`) (com/tiviacz/travelersbackpack/client/screens/buttons/Button.java:18)
- [ ] `Button.drawButton(8)`: its class isn't translated (it implements `IButton`) (com/tiviacz/travelersbackpack/client/screens/buttons/Button.java:26)
- [ ] `Button.inButton(1)`: its class isn't translated (it implements `IButton`) (com/tiviacz/travelersbackpack/client/screens/buttons/Button.java:55)
- [ ] `Button.inButton(2)`: its class isn't translated (it implements `IButton`) (com/tiviacz/travelersbackpack/client/screens/buttons/Button.java:59)
- [ ] `Button.isWithinBounds(4)`: its class isn't translated (it implements `IButton`) (com/tiviacz/travelersbackpack/client/screens/buttons/Button.java:65)
- [ ] `Button.isWithinBounds(3)`: its class isn't translated (it implements `IButton`) (com/tiviacz/travelersbackpack/client/screens/buttons/Button.java:72)

### `EquipButton`: it extends `Button`

- [ ] `EquipButton.EquipButton(2)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/EquipButton.java:13)
- [ ] `EquipButton.render(4)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/EquipButton.java:18)
- [ ] `EquipButton.renderTooltip(3)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/EquipButton.java:23)
- [ ] `EquipButton.mouseClicked(2)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/EquipButton.java:34)

### `IButton`: it's an interface

- [ ] `IButton.render(4)`: it has no body (com/tiviacz/travelersbackpack/client/screens/buttons/IButton.java:7)
- [ ] `IButton.renderTooltip(3)`: it has no body (com/tiviacz/travelersbackpack/client/screens/buttons/IButton.java:9)
- [ ] `IButton.mouseClicked(2)`: it has no body (com/tiviacz/travelersbackpack/client/screens/buttons/IButton.java:11)

### `MoreButton`: it extends `Button`

- [ ] `MoreButton.MoreButton(1)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/MoreButton.java:10)
- [ ] `MoreButton.render(4)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/MoreButton.java:14)
- [ ] `MoreButton.renderTooltip(3)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/MoreButton.java:19)
- [ ] `MoreButton.mouseClicked(2)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/MoreButton.java:26)

### `SleepingBagButton`: it extends `Button`

- [ ] `SleepingBagButton.SleepingBagButton(3)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/SleepingBagButton.java:16)
- [ ] `SleepingBagButton.render(4)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/SleepingBagButton.java:21)
- [ ] `SleepingBagButton.renderTooltip(3)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/SleepingBagButton.java:26)
- [ ] `SleepingBagButton.mouseClicked(2)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/SleepingBagButton.java:36)

### `UnequipButton`: it extends `Button`

- [ ] `UnequipButton.UnequipButton(1)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/UnequipButton.java:13)
- [ ] `UnequipButton.render(4)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/UnequipButton.java:17)
- [ ] `UnequipButton.renderTooltip(3)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/UnequipButton.java:24)
- [ ] `UnequipButton.mouseClicked(2)`: its class isn't translated (it extends `Button`) (com/tiviacz/travelersbackpack/client/screens/buttons/UnequipButton.java:31)

### `HudOverlay`

- [x] `HudOverlay.OVERLAY` (com/tiviacz/travelersbackpack/client/screens/HudOverlay.java:21)
- [ ] `HudOverlay.renderOverlay(2)`: it uses TravelersBackpackConfig.CLIENT, which isn't translated (it uses Client, which isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec))) (com/tiviacz/travelersbackpack/client/screens/HudOverlay.java:23)
- [ ] `HudOverlay.drawGuiTank(6)`: it uses FluidTank, which isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/client/screens/HudOverlay.java:58)

### `RadialToolsOverlay`

- [x] `RadialToolsOverlay.TOOLS_OVERLAY` (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java:26)
- [x] `RadialToolsOverlay.ICON_SIZE` (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java:27)
- [x] `RadialToolsOverlay.ITEM_RING_RADIUS` (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java:28)
- [x] `RadialToolsOverlay.DEADZONE_RADIUS` (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java:29)
- [x] `RadialToolsOverlay.ADD_NEW` (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java:30)
- [x] `RadialToolsOverlay.drawCrosshair` (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java:31)
- [ ] `RadialToolsOverlay.renderRadial(11)`: it uses Minecraft's `NonNullList` (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java:33)
- [ ] `RadialToolsOverlay.renderRadialItems(11)`: it uses Minecraft's `NonNullList` (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java:78)
- [ ] `RadialToolsOverlay.buildSegToSlot(3)`: it uses Minecraft's `NonNullList` (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java:196)
- [ ] `RadialToolsOverlay.getTooltipFromItem(2)`: it calls Minecraft's ItemStack.getTooltipLines (no Horizon helper for it yet) (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java:237)
- [x] `RadialToolsOverlay.getHoveredIndex(5)` (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java:241)
- [ ] `RadialToolsOverlay.renderCenteredText(4)`: it calls Minecraft's GuiGraphics.enableScissor (no Horizon helper for it yet) (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java:261)
- [ ] `RadialToolsOverlay.renderPlusButton(5)`: it calls Minecraft's GuiGraphics.pose (no Horizon helper for it yet) (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java:273)
- [ ] `RadialToolsOverlay.renderCenteredItem(6)`: it calls Minecraft's GuiGraphics.pose (no Horizon helper for it yet) (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java:292)
- [x] `RadialToolsOverlay.normalize0To2Pi(1)` (com/tiviacz/travelersbackpack/client/screens/RadialToolsOverlay.java:301)

### `ToolsScreen`: it extends Minecraft's `Screen`

- [x] `ToolsScreen.REF_W` (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:31)
- [x] `ToolsScreen.REF_H` (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:32)
- [ ] `ToolsScreen.hoseMenu`: it uses Minecraft's `NonNullList` (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:36)
- [ ] `ToolsScreen.ToolsScreen(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:38)
- [ ] `ToolsScreen.getOpenProgress(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:42)
- [ ] `ToolsScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:52)
- [ ] `ToolsScreen.extractBackground(4)`: its class isn't translated (it extends Minecraft's `Screen`) (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:60)
- [ ] `ToolsScreen.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:63)
- [ ] `ToolsScreen.getScaledWindow(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:99)
- [ ] `ToolsScreen.selectHoseAction(2)`: its class isn't translated (it extends Minecraft's `Screen`) (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:114)
- [x] `ToolsScreen.getNextModeMessage(2)` in game_side.rs (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:140)
- [ ] `ToolsScreen.getNonEmptyTools(1)`: it uses Minecraft's `NonNullList` (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:152)
- [ ] `ToolsScreen.getHoseMenu(0)`: it uses Minecraft's `NonNullList` (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:164)
- [ ] `ToolsScreen.isPauseScreen(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:183)
- [ ] `ToolsScreen.mouseScrolled(4)`: its class isn't translated (it extends Minecraft's `Screen`) (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:187)
- [ ] `ToolsScreen.mouseClicked(2)`: its class isn't translated (it extends Minecraft's `Screen`) (com/tiviacz/travelersbackpack/client/screens/ToolsScreen.java:197)

### `BackpackTooltipComponent`: it implements Minecraft's `TooltipComponent`

- [ ] `BackpackTooltipComponent.BackpackTooltipComponent(1)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/BackpackTooltipComponent.java:21)
- [ ] `BackpackTooltipComponent.BackpackTooltipComponent(2)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/BackpackTooltipComponent.java:25)
- [ ] `BackpackTooltipComponent.isHoveredWithItem(0)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/BackpackTooltipComponent.java:30)
- [ ] `BackpackTooltipComponent.loadComponentData(1)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/BackpackTooltipComponent.java:34)
- [ ] `BackpackTooltipComponent.loadFluidStacks(1)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/BackpackTooltipComponent.java:42)
- [ ] `BackpackTooltipComponent.loadStorage(1)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/BackpackTooltipComponent.java:50)
- [ ] `BackpackTooltipComponent.loadUpgrades(1)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/BackpackTooltipComponent.java:56)
- [ ] `BackpackTooltipComponent.mergeStacks(1)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/BackpackTooltipComponent.java:62)
- [ ] `BackpackTooltipComponent.loadTools(1)`: its class isn't translated (it implements Minecraft's `TooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/BackpackTooltipComponent.java:117)

### `ClientBackpackTooltipComponent`: it implements Minecraft's `ClientTooltipComponent`

- [ ] `ClientBackpackTooltipComponent.ClientBackpackTooltipComponent(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/ClientBackpackTooltipComponent.java:20)
- [ ] `ClientBackpackTooltipComponent.show(0)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/ClientBackpackTooltipComponent.java:24)
- [ ] `ClientBackpackTooltipComponent.getHeight(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/ClientBackpackTooltipComponent.java:28)
- [ ] `ClientBackpackTooltipComponent.getWidth(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/ClientBackpackTooltipComponent.java:58)
- [ ] `ClientBackpackTooltipComponent.extractText(4)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/ClientBackpackTooltipComponent.java:82)
- [ ] `ClientBackpackTooltipComponent.extractImage(6)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/ClientBackpackTooltipComponent.java:119)
- [ ] `ClientBackpackTooltipComponent.renderItem(5)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/ClientBackpackTooltipComponent.java:179)
- [ ] `ClientBackpackTooltipComponent.getFluidTankTooltip(1)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/ClientBackpackTooltipComponent.java:184)
- [ ] `ClientBackpackTooltipComponent.renderFluidTankTooltip(5)`: its class isn't translated (it implements Minecraft's `ClientTooltipComponent`) (com/tiviacz/travelersbackpack/client/screens/tooltip/ClientBackpackTooltipComponent.java:191)

### `UpgradeSlot`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)

- [x] `UpgradeSlot.SLOT_SIZE` (com/tiviacz/travelersbackpack/client/screens/UpgradeSlot.java:11)
- [x] `UpgradeSlot.ICON_SIZE` (com/tiviacz/travelersbackpack/client/screens/UpgradeSlot.java:12)
- [ ] `UpgradeSlot.UpgradeSlot(6)`: it stores a second reference to an object (Rust would need it copied or shared) (com/tiviacz/travelersbackpack/client/screens/UpgradeSlot.java:20)
- [ ] `UpgradeSlot.render(3)`: it uses Minecraft's `GuiGraphicsExtractor` (com/tiviacz/travelersbackpack/client/screens/UpgradeSlot.java:29)
- [ ] `UpgradeSlot.getIndex(0)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/client/screens/UpgradeSlot.java:44)
- [ ] `UpgradeSlot.isHidden(0)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/client/screens/UpgradeSlot.java:48)
- [ ] `UpgradeSlot.getUpgradeSlotSizeAndPos(0)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/client/screens/UpgradeSlot.java:52)

### `FilterUpgradeWidgetBase`: it's generic

- [ ] `FilterUpgradeWidgetBase.FilterUpgradeWidgetBase(5)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:28)
- [ ] `FilterUpgradeWidgetBase.addFilterButton(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:36)
- [ ] `FilterUpgradeWidgetBase.hideButton(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:46)
- [ ] `FilterUpgradeWidgetBase.hasButton(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:52)
- [ ] `FilterUpgradeWidgetBase.getFilterButton(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:56)
- [ ] `FilterUpgradeWidgetBase.renderBg(5)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:64)
- [ ] `FilterUpgradeWidgetBase.extractRenderState(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:103)
- [ ] `FilterUpgradeWidgetBase.renderTooltip(3)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:153)
- [ ] `FilterUpgradeWidgetBase.updateAddableTags(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:161)
- [ ] `FilterUpgradeWidgetBase.isAdding(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:166)
- [ ] `FilterUpgradeWidgetBase.getSelectedTagIndex(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:170)
- [ ] `FilterUpgradeWidgetBase.setNextSelectedTagIndex(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:174)
- [ ] `FilterUpgradeWidgetBase.getTags(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:182)
- [ ] `FilterUpgradeWidgetBase.addTag(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:186)
- [ ] `FilterUpgradeWidgetBase.removeTag(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:201)
- [ ] `FilterUpgradeWidgetBase.getTooltip(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:214)
- [ ] `FilterUpgradeWidgetBase.calculateTagBoxY(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:234)
- [ ] `FilterUpgradeWidgetBase.getTrimmedText(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:238)
- [ ] `FilterUpgradeWidgetBase.isMouseOverTags(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:258)
- [ ] `FilterUpgradeWidgetBase.mouseScrolled(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:262)
- [ ] `FilterUpgradeWidgetBase.mouseClicked(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/FilterUpgradeWidgetBase.java:271)

### `InventoryScroll`: it extends `ScrollPanel`

- [ ] `InventoryScroll.InventoryScroll(6)`: its class isn't translated (it extends `ScrollPanel`) (com/tiviacz/travelersbackpack/client/screens/widgets/InventoryScroll.java:12)
- [ ] `InventoryScroll.getScrollAmount(0)`: its class isn't translated (it extends `ScrollPanel`) (com/tiviacz/travelersbackpack/client/screens/widgets/InventoryScroll.java:17)
- [ ] `InventoryScroll.getContentHeight(0)`: its class isn't translated (it extends `ScrollPanel`) (com/tiviacz/travelersbackpack/client/screens/widgets/InventoryScroll.java:22)
- [ ] `InventoryScroll.mouseScrolled(4)`: its class isn't translated (it extends `ScrollPanel`) (com/tiviacz/travelersbackpack/client/screens/widgets/InventoryScroll.java:27)
- [ ] `InventoryScroll.mouseDragged(3)`: its class isn't translated (it extends `ScrollPanel`) (com/tiviacz/travelersbackpack/client/screens/widgets/InventoryScroll.java:35)
- [ ] `InventoryScroll.mouseClicked(2)`: its class isn't translated (it extends `ScrollPanel`) (com/tiviacz/travelersbackpack/client/screens/widgets/InventoryScroll.java:43)
- [ ] `InventoryScroll.narrationPriority(0)`: its class isn't translated (it extends `ScrollPanel`) (com/tiviacz/travelersbackpack/client/screens/widgets/InventoryScroll.java:48)
- [ ] `InventoryScroll.updateNarration(1)`: its class isn't translated (it extends `ScrollPanel`) (com/tiviacz/travelersbackpack/client/screens/widgets/InventoryScroll.java:52)
- [ ] `InventoryScroll.setScrollDistance(1)`: its class isn't translated (it extends `ScrollPanel`) (com/tiviacz/travelersbackpack/client/screens/widgets/InventoryScroll.java:55)

### `ScrollPanel`: it extends Minecraft's `AbstractContainerEventHandler`

- [ ] `ScrollPanel.ScrollPanel(6)`: its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`) (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:32)
- [ ] `ScrollPanel.ScrollPanel(7)`: its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`) (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:36)
- [ ] `ScrollPanel.ScrollPanel(10)`: its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`) (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:40)
- [ ] `ScrollPanel.getContentHeight(0)`: it has no body (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:56)
- [ ] `ScrollPanel.clickPanel(3)`: its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`) (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:58)
- [ ] `ScrollPanel.getMaxScroll(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`) (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:62)
- [ ] `ScrollPanel.applyScrollLimits(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`) (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:66)
- [ ] `ScrollPanel.mouseScrolled(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`) (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:81)
- [ ] `ScrollPanel.getScrollAmount(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`) (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:91)
- [ ] `ScrollPanel.isMouseOver(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`) (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:95)
- [ ] `ScrollPanel.mouseClicked(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`) (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:99)
- [ ] `ScrollPanel.mouseReleased(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`) (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:115)
- [ ] `ScrollPanel.getBarHeight(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`) (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:125)
- [ ] `ScrollPanel.mouseDragged(3)`: its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`) (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:138)
- [ ] `ScrollPanel.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`) (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:150)
- [ ] `ScrollPanel.children(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerEventHandler`) (com/tiviacz/travelersbackpack/client/screens/widgets/ScrollPanel.java:169)

### `SettingsWidget`: it extends `WidgetBase`

- [ ] `SettingsWidget.SettingsWidget(3)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/SettingsWidget.java:19)
- [ ] `SettingsWidget.renderBg(5)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/SettingsWidget.java:24)
- [ ] `SettingsWidget.getSettingsUser(0)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/SettingsWidget.java:67)
- [ ] `SettingsWidget.renderTooltip(3)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/SettingsWidget.java:78)
- [ ] `SettingsWidget.mouseClicked(2)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/SettingsWidget.java:103)

### `MemoryWidget`: it extends `SettingsWidgetBase`

- [x] `MemoryWidget.IGNORE_MODE_TOOLTIPS` in game_side.rs (com/tiviacz/travelersbackpack/client/screens/widgets/settings/MemoryWidget.java:28)
- [ ] `MemoryWidget.MemoryWidget(3)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/MemoryWidget.java:32)
- [ ] `MemoryWidget.isTabOpened(0)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/MemoryWidget.java:37)
- [ ] `MemoryWidget.sendDataToServer(0)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/MemoryWidget.java:41)
- [ ] `MemoryWidget.contains(2)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/MemoryWidget.java:56)
- [ ] `MemoryWidget.renderBg(5)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/MemoryWidget.java:66)
- [ ] `MemoryWidget.renderTooltip(3)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/MemoryWidget.java:134)
- [ ] `MemoryWidget.isMouseOverMatchComponentsButton(2)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/MemoryWidget.java:147)
- [ ] `MemoryWidget.mouseClicked(2)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/MemoryWidget.java:151)

### `SettingsWidgetBase`: it extends `WidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)

- [ ] `SettingsWidgetBase.SettingsWidgetBase(3)`: its class isn't translated (it extends `WidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/SettingsWidgetBase.java:11)
- [ ] `SettingsWidgetBase.updatePos(2)`: its class isn't translated (it extends `WidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/SettingsWidgetBase.java:16)
- [ ] `SettingsWidgetBase.isMouseOver(2)`: its class isn't translated (it extends `WidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/SettingsWidgetBase.java:20)
- [ ] `SettingsWidgetBase.getWidgetSizeAndPos(0)`: its class isn't translated (it extends `WidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/SettingsWidgetBase.java:28)

### `SupporterBadgeWidget`: it extends `SettingsWidgetBase`

- [ ] `SupporterBadgeWidget.SupporterBadgeWidget(2)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/SupporterBadgeWidget.java:21)
- [ ] `SupporterBadgeWidget.sendDataToServer(0)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/SupporterBadgeWidget.java:25)
- [ ] `SupporterBadgeWidget.renderBg(5)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/SupporterBadgeWidget.java:32)
- [ ] `SupporterBadgeWidget.renderTooltip(3)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/SupporterBadgeWidget.java:52)
- [ ] `SupporterBadgeWidget.mouseClicked(2)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/SupporterBadgeWidget.java:62)

### `UnsortablesWidget`: it extends `SettingsWidgetBase`

- [ ] `UnsortablesWidget.UnsortablesWidget(2)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/UnsortablesWidget.java:25)
- [ ] `UnsortablesWidget.isTabOpened(0)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/UnsortablesWidget.java:29)
- [ ] `UnsortablesWidget.sendDataToServer(0)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/UnsortablesWidget.java:33)
- [ ] `UnsortablesWidget.renderBg(5)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/UnsortablesWidget.java:42)
- [ ] `UnsortablesWidget.renderTooltip(3)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/UnsortablesWidget.java:150)
- [ ] `UnsortablesWidget.isMouseOverSelectAllButton(2)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/UnsortablesWidget.java:167)
- [ ] `UnsortablesWidget.isMouseOverRemoveAllButton(2)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/UnsortablesWidget.java:171)
- [ ] `UnsortablesWidget.mouseClicked(2)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/UnsortablesWidget.java:175)
- [ ] `UnsortablesWidget.getWidgetSizeAndPos(0)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/UnsortablesWidget.java:207)

### `VisibilityWidget`: it extends `SettingsWidgetBase`

- [ ] `VisibilityWidget.VisibilityWidget(2)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/VisibilityWidget.java:16)
- [ ] `VisibilityWidget.sendDataToServer(0)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/VisibilityWidget.java:20)
- [ ] `VisibilityWidget.renderBg(5)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/VisibilityWidget.java:25)
- [ ] `VisibilityWidget.renderTooltip(3)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/VisibilityWidget.java:45)
- [ ] `VisibilityWidget.mouseClicked(2)`: its class isn't translated (it extends `SettingsWidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/settings/VisibilityWidget.java:52)

### `SortingButtons`: it extends `WidgetBase`

- [ ] `SortingButtons.SortingButtons(4)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/SortingButtons.java:16)
- [ ] `SortingButtons.extractRenderState(4)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/SortingButtons.java:20)
- [ ] `SortingButtons.renderTooltip(3)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/SortingButtons.java:40)
- [ ] `SortingButtons.mouseClicked(2)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/SortingButtons.java:69)
- [ ] `SortingButtons.isButtonHovered(3)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/SortingButtons.java:91)
- [ ] `SortingButtons.isButtonHovered(3)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/SortingButtons.java:95)
- [ ] `SortingButtons.isButtonHovered(4)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/SortingButtons.java:102)

### `SortingButtons.Buttons`


### `ToolSlotsWidget`: it extends `WidgetBase`

- [ ] `ToolSlotsWidget.ToolSlotsWidget(3)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/ToolSlotsWidget.java:14)
- [ ] `ToolSlotsWidget.extractRenderState(4)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/ToolSlotsWidget.java:19)
- [ ] `ToolSlotsWidget.mouseClicked(2)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/ToolSlotsWidget.java:35)
- [ ] `ToolSlotsWidget.renderTooltip(3)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/ToolSlotsWidget.java:45)
- [ ] `ToolSlotsWidget.renderToolsAddition(4)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/ToolSlotsWidget.java:56)
- [ ] `ToolSlotsWidget.getAdditionSizeAndPos(0)`: its class isn't translated (it extends `WidgetBase`) (com/tiviacz/travelersbackpack/client/screens/widgets/ToolSlotsWidget.java:66)

### `UpgradeWidgetBase`: it's generic

- [ ] `UpgradeWidgetBase.UpgradeWidgetBase(5)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:29)
- [ ] `UpgradeWidgetBase.getUpgrade(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:41)
- [ ] `UpgradeWidgetBase.renderBg(5)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:45)
- [ ] `UpgradeWidgetBase.extractRenderState(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:148)
- [ ] `UpgradeWidgetBase.renderTooltip(3)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:156)
- [ ] `UpgradeWidgetBase.getAdditionalTooltips(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:174)
- [ ] `UpgradeWidgetBase.mouseClicked(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:177)
- [ ] `UpgradeWidgetBase.isMouseOver(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:204)
- [ ] `UpgradeWidgetBase.isMouseOverRemoveButton(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:218)
- [ ] `UpgradeWidgetBase.isMouseOverEnableButton(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:222)
- [ ] `UpgradeWidgetBase.renderMatchContentsSlotOverlay(5)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:226)
- [ ] `UpgradeWidgetBase.renderRemoveButton(3)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:240)
- [ ] `UpgradeWidgetBase.removeButtonMouseClicked(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:257)
- [ ] `UpgradeWidgetBase.removeUpgrade(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:261)
- [ ] `UpgradeWidgetBase.renderEnableButton(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:271)
- [ ] `UpgradeWidgetBase.renderEnableButtonTooltip(3)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:323)
- [ ] `UpgradeWidgetBase.enableButtonMouseClicked(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:333)
- [ ] `UpgradeWidgetBase.hasButtons(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:347)
- [ ] `UpgradeWidgetBase.isBackpackOwner(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:351)
- [ ] `UpgradeWidgetBase.isTabOpened(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:355)
- [ ] `UpgradeWidgetBase.getWidgetSizeAndPos(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/UpgradeWidgetBase.java:359)

### `WidgetBase`: it's generic

- [ ] `WidgetBase.WidgetBase(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:21)
- [ ] `WidgetBase.getPos(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:28)
- [ ] `WidgetBase.extractRenderState(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:32)
- [ ] `WidgetBase.renderBg(5)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:35)
- [ ] `WidgetBase.renderUnderTooltip(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:38)
- [ ] `WidgetBase.renderTooltip(3)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:41)
- [ ] `WidgetBase.renderAboveBg(6)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:44)
- [ ] `WidgetBase.renderOnTop(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:47)
- [ ] `WidgetBase.isMouseOver(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:50)
- [ ] `WidgetBase.setFocused(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:54)
- [ ] `WidgetBase.isFocused(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:57)
- [ ] `WidgetBase.isMouseOverIcon(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:61)
- [ ] `WidgetBase.isMouseOverIcon(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:65)
- [ ] `WidgetBase.in(6)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:69)
- [ ] `WidgetBase.isWithinBounds(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:73)
- [ ] `WidgetBase.isWithinBounds(3)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:80)
- [ ] `WidgetBase.narrationPriority(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:84)
- [ ] `WidgetBase.updateNarration(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:88)
- [ ] `WidgetBase.getWidgetSizeAndPos(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetBase.java:91)

### `WidgetElement`

- [x] `WidgetElement.WidgetElement(2)` (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetElement.java:5)
- [x] `WidgetElement.pos(0)` (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetElement.java:5)
- [x] `WidgetElement.size(0)` (com/tiviacz/travelersbackpack/client/screens/widgets/WidgetElement.java:5)

### `AccessCommand`

- [ ] `AccessCommand.AccessCommand(3)`: it uses Minecraft's `CommandDispatcher` (com/tiviacz/travelersbackpack/commands/AccessCommand.java:22)
- [ ] `AccessCommand.openTargetBlockEntity(2)`: it uses Minecraft's `CommandSourceStack` (com/tiviacz/travelersbackpack/commands/AccessCommand.java:39)
- [ ] `AccessCommand.openTargetInventory(2)`: it uses Minecraft's `CommandSourceStack` (com/tiviacz/travelersbackpack/commands/AccessCommand.java:50)

### `BackpackIconCommands`

- [ ] `BackpackIconCommands.BackpackIconCommands(2)`: it uses Minecraft's `CommandDispatcher` (com/tiviacz/travelersbackpack/commands/BackpackIconCommands.java:12)
- [ ] `BackpackIconCommands.hideIcon(1)`: it uses Fabric's `FabricClientCommandSource` (com/tiviacz/travelersbackpack/commands/BackpackIconCommands.java:20)
- [ ] `BackpackIconCommands.showIcon(0)`: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec) (com/tiviacz/travelersbackpack/commands/BackpackIconCommands.java:27)

### `ClearCommand`

- [ ] `ClearCommand.ClearCommand(3)`: it uses Minecraft's `CommandDispatcher` (com/tiviacz/travelersbackpack/commands/ClearCommand.java:20)
- [ ] `ClearCommand.removeBackpack(2)`: it uses Minecraft's `ServerPlayer` (com/tiviacz/travelersbackpack/commands/ClearCommand.java:42)
- [ ] `ClearCommand.clearBackpack(2)`: it uses Minecraft's `ServerPlayer` (com/tiviacz/travelersbackpack/commands/ClearCommand.java:66)

### `RestoreCommand`

- [ ] `RestoreCommand.SUGGESTION_PROVIDER`: it uses Minecraft's `SuggestionProvider` (com/tiviacz/travelersbackpack/commands/RestoreCommand.java:26)
- [ ] `RestoreCommand.RestoreCommand(3)`: it uses Minecraft's `CommandDispatcher` (com/tiviacz/travelersbackpack/commands/RestoreCommand.java:43)
- [ ] `RestoreCommand.restoreBackpack(3)`: it uses Minecraft's `CommandSourceStack` (com/tiviacz/travelersbackpack/commands/RestoreCommand.java:66)

### `UnpackCommand`

- [ ] `UnpackCommand.UnpackCommand(3)`: it uses Minecraft's `CommandDispatcher` (com/tiviacz/travelersbackpack/commands/UnpackCommand.java:26)
- [ ] `UnpackCommand.unpackTargetBlockEntity(2)`: it uses Minecraft's `CommandSourceStack` (com/tiviacz/travelersbackpack/commands/UnpackCommand.java:45)
- [ ] `UnpackCommand.unpackTargetInventory(2)`: it uses Minecraft's `CommandSourceStack` (com/tiviacz/travelersbackpack/commands/UnpackCommand.java:65)
- [ ] `UnpackCommand.collectItems(1)`: it uses Minecraft's `NonNullList` (com/tiviacz/travelersbackpack/commands/UnpackCommand.java:99)
- [ ] `UnpackCommand.collectItems(1)`: it uses Minecraft's `NonNullList` (com/tiviacz/travelersbackpack/commands/UnpackCommand.java:107)

### `BackpackAbilities`: it uses Minecraft's `AttributeModifier`

- [ ] `BackpackAbilities.ABILITIES`: it uses BackpackAbilities, which isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:70)
- [x] `BackpackAbilities.ALLOWED_ABILITIES` in game_side.rs (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:96)
- [ ] `BackpackAbilities.BACKPACK_EFFECTS`: it uses `Multimap` (com.google.common.collect) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:97)
- [x] `BackpackAbilities.COOLDOWNS` in game_side.rs (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:98)
- [ ] `BackpackAbilities.ITEM_ABILITIES_LIST`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:99)
- [ ] `BackpackAbilities.ITEM_ABILITIES_REMOVAL_LIST`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:130)
- [ ] `BackpackAbilities.BLOCK_ABILITIES_LIST`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:142)
- [ ] `BackpackAbilities.CUSTOM_DESCRIPTIONS`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:151)
- [ ] `BackpackAbilities.abilityTick(2)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:176)
- [ ] `BackpackAbilities.abilityTickBlock(1)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:314)
- [ ] `BackpackAbilities.abilityRemoval(2)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:326)
- [ ] `BackpackAbilities.animateTick(5)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:361)
- [ ] `BackpackAbilities.getAttributeAbilityMultimap(1)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:374)
- [ ] `BackpackAbilities.attributeAbility(4)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:406)
- [ ] `BackpackAbilities.armorAbilityRemovals(1)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:417)
- [ ] `BackpackAbilities.lapisAbility(1)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:429)
- [ ] `BackpackAbilities.bookshelfAbility(2)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:444)
- [ ] `BackpackAbilities.spongeAbility(1)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:481)
- [ ] `BackpackAbilities.cakeAbility(2)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:508)
- [ ] `BackpackAbilities.chickenAbility(3)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:548)
- [ ] `BackpackAbilities.cactusAbilityEquipped(2)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:577)
- [ ] `BackpackAbilities.cactusAbilityBlockEntity(2)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:624)
- [ ] `BackpackAbilities.melonAbility(1)`: it uses BackpackBlockEntity, which isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:654)
- [ ] `BackpackAbilities.pumpkinAbility(2)`: it uses `CallbackInfoReturnable` (org.spongepowered.asm.mixin.injection.callback) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:665)
- [ ] `BackpackAbilities.creeperAbility(1)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:672)
- [ ] `BackpackAbilities.blazeAbility(1)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:707)
- [ ] `BackpackAbilities.blazeAbility(3)`: it uses Minecraft's `EntityHitResult` (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:717)
- [ ] `BackpackAbilities.ghastAbility(3)`: it uses Minecraft's `Ghast` (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:734)
- [ ] `BackpackAbilities.spiderAbility(1)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:740)
- [ ] `BackpackAbilities.witherAbilityTick(1)`: it uses BackpackAbilities.ABILITIES, which isn't translated (it uses BackpackAbilities, which isn't translated (it uses Minecraft's `AttributeModifier`)) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:766)
- [ ] `BackpackAbilities.witherAbility(2)`: it uses Minecraft's `Entity` (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:772)
- [ ] `BackpackAbilities.wardenAbility(2)`: it uses Minecraft's `Entity` (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:778)
- [ ] `BackpackAbilities.beeAbility(2)`: it uses Minecraft's `Entity` (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:784)
- [ ] `BackpackAbilities.ocelotAbility(1)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:798)
- [ ] `BackpackAbilities.cowAbility(2)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:808)
- [ ] `BackpackAbilities.removeAllNegativeEffects(2)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:824)
- [ ] `BackpackAbilities.isUnderRain(2)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:843)
- [ ] `BackpackAbilities.isAbilityEnabledInConfig(1)`: it uses TravelersBackpackConfig.serverSpec, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:847)
- [ ] `BackpackAbilities.checkBackpack(2)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:853)
- [ ] `BackpackAbilities.addTimedMobEffect(8)`: its class isn't translated (it uses Minecraft's `AttributeModifier`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:862)
- [ ] `BackpackAbilities.sendParticlesPacket(3)`: it uses Minecraft's `ParticleOptions` (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:872)
- [x] `BackpackAbilities.isOnList(2)` in game_side.rs (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:883)
- [ ] `BackpackAbilities.hasCooldown(1)`: it calls Minecraft's ItemStack.getOrDefault (no Horizon helper for it yet) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:887)
- [ ] `BackpackAbilities.setCooldown(2)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:891)
- [x] `BackpackAbilities.getAllowedAbilities(0)` in game_side.rs (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:898)
- [x] `BackpackAbilities.getCooldowns(0)` in game_side.rs (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:902)
- [ ] `BackpackAbilities.getBackpackEffects(0)`: it uses `Multimap` (com.google.common.collect) (com/tiviacz/travelersbackpack/common/BackpackAbilities.java:906)

### `BackpackManager`

- [ ] `BackpackManager.BACKPACKS`: it uses Minecraft's `LevelResource` (com/tiviacz/travelersbackpack/common/BackpackManager.java:19)
- [ ] `BackpackManager.addBackpack(2)`: it uses Minecraft's `ServerPlayer` (com/tiviacz/travelersbackpack/common/BackpackManager.java:21)
- [ ] `BackpackManager.readBackpack(3)`: it uses Minecraft's `ServerLevel` (com/tiviacz/travelersbackpack/common/BackpackManager.java:40)
- [ ] `BackpackManager.getBackpack(2)`: it uses Minecraft's `ServerLevel` (com/tiviacz/travelersbackpack/common/BackpackManager.java:53)
- [ ] `BackpackManager.getBackpackFile(3)`: it uses `File` (java.io) (com/tiviacz/travelersbackpack/common/BackpackManager.java:73)
- [ ] `BackpackManager.getBackpackFile(2)`: it uses `File` (java.io) (com/tiviacz/travelersbackpack/common/BackpackManager.java:77)
- [ ] `BackpackManager.getPlayerBackpackFolder(1)`: it uses `File` (java.io) (com/tiviacz/travelersbackpack/common/BackpackManager.java:81)
- [ ] `BackpackManager.getPlayerBackpackFolder(2)`: it uses `File` (java.io) (com/tiviacz/travelersbackpack/common/BackpackManager.java:85)
- [ ] `BackpackManager.getBackpackFolder(1)`: it uses `File` (java.io) (com/tiviacz/travelersbackpack/common/BackpackManager.java:89)
- [ ] `BackpackManager.getWorldFolder(2)`: it uses `File` (java.io) (com/tiviacz/travelersbackpack/common/BackpackManager.java:93)

### `BackpackUpgradeRecipe`: it extends Minecraft's `SimpleSmithingRecipe`

- [ ] `BackpackUpgradeRecipe.MAP_CODEC`: it uses Minecraft's `MapCodec` (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipe.java:30)
- [ ] `BackpackUpgradeRecipe.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipe.java:40)
- [ ] `BackpackUpgradeRecipe.SERIALIZER`: it uses Minecraft's `RecipeSerializer` (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipe.java:53)
- [ ] `BackpackUpgradeRecipe.BackpackUpgradeRecipe(5)`: its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipe.java:59)
- [ ] `BackpackUpgradeRecipe.assemble(1)`: its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipe.java:67)
- [ ] `BackpackUpgradeRecipe.upgradeInventory(2)`: its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipe.java:81)
- [ ] `BackpackUpgradeRecipe.getUpgradedTanksCapacity(2)`: its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipe.java:91)
- [ ] `BackpackUpgradeRecipe.templateIngredient(0)`: its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipe.java:100)
- [ ] `BackpackUpgradeRecipe.baseIngredient(0)`: its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipe.java:104)
- [ ] `BackpackUpgradeRecipe.additionIngredient(0)`: its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipe.java:108)
- [ ] `BackpackUpgradeRecipe.getSerializer(0)`: its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipe.java:112)
- [ ] `BackpackUpgradeRecipe.createPlacementInfo(0)`: its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipe.java:116)
- [ ] `BackpackUpgradeRecipe.display(0)`: its class isn't translated (it extends Minecraft's `SimpleSmithingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipe.java:120)

### `BackpackUpgradeRecipeBuilder`: it uses Minecraft's `Ingredient`

- [ ] `BackpackUpgradeRecipeBuilder.BackpackUpgradeRecipeBuilder(5)`: its class isn't translated (it uses Minecraft's `Ingredient`) (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipeBuilder.java:25)
- [ ] `BackpackUpgradeRecipeBuilder.backpackUpgrade(5)`: it uses BackpackUpgradeRecipeBuilder, which isn't translated (it uses Minecraft's `Ingredient`) (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipeBuilder.java:33)
- [ ] `BackpackUpgradeRecipeBuilder.unlocks(2)`: its class isn't translated (it uses Minecraft's `Ingredient`) (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipeBuilder.java:37)
- [ ] `BackpackUpgradeRecipeBuilder.save(2)`: its class isn't translated (it uses Minecraft's `Ingredient`) (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipeBuilder.java:42)
- [ ] `BackpackUpgradeRecipeBuilder.save(2)`: its class isn't translated (it uses Minecraft's `Ingredient`) (com/tiviacz/travelersbackpack/common/recipes/BackpackUpgradeRecipeBuilder.java:46)

### `ShapedBackpackRecipe`: it extends Minecraft's `NormalCraftingRecipe`

- [ ] `ShapedBackpackRecipe.MAP_CODEC`: it uses Minecraft's `MapCodec` (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipe.java:46)
- [ ] `ShapedBackpackRecipe.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipe.java:55)
- [ ] `ShapedBackpackRecipe.SERIALIZER`: it uses Minecraft's `RecipeSerializer` (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipe.java:66)
- [ ] `ShapedBackpackRecipe.ShapedBackpackRecipe(4)`: its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipe.java:70)
- [ ] `ShapedBackpackRecipe.assemble(1)`: its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipe.java:76)
- [ ] `ShapedBackpackRecipe.getProperColor(1)`: `instanceof` (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipe.java:120)
- [ ] `ShapedBackpackRecipe.getIngredients(0)`: its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipe.java:128)
- [ ] `ShapedBackpackRecipe.createPlacementInfo(0)`: its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipe.java:133)
- [ ] `ShapedBackpackRecipe.matches(2)`: its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipe.java:137)
- [ ] `ShapedBackpackRecipe.getWidth(0)`: its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipe.java:141)
- [ ] `ShapedBackpackRecipe.getHeight(0)`: its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipe.java:145)
- [ ] `ShapedBackpackRecipe.display(0)`: its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipe.java:149)
- [ ] `ShapedBackpackRecipe.getSerializer(0)`: its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipe.java:161)
- [ ] `ShapedBackpackRecipe.getType(0)`: its class isn't translated (it extends Minecraft's `NormalCraftingRecipe`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipe.java:165)

### `ShapedBackpackRecipeBuilder`: it implements Minecraft's `RecipeBuilder`

- [ ] `ShapedBackpackRecipeBuilder.ShapedBackpackRecipeBuilder(3)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipeBuilder.java:34)
- [ ] `ShapedBackpackRecipeBuilder.ShapedBackpackRecipeBuilder(4)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipeBuilder.java:40)
- [ ] `ShapedBackpackRecipeBuilder.shaped(3)`: it uses ShapedBackpackRecipeBuilder, which isn't translated (it implements Minecraft's `RecipeBuilder`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipeBuilder.java:44)
- [ ] `ShapedBackpackRecipeBuilder.shaped(4)`: it uses ShapedBackpackRecipeBuilder, which isn't translated (it implements Minecraft's `RecipeBuilder`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipeBuilder.java:48)
- [ ] `ShapedBackpackRecipeBuilder.define(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipeBuilder.java:52)
- [ ] `ShapedBackpackRecipeBuilder.define(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipeBuilder.java:56)
- [ ] `ShapedBackpackRecipeBuilder.define(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipeBuilder.java:60)
- [ ] `ShapedBackpackRecipeBuilder.pattern(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipeBuilder.java:73)
- [ ] `ShapedBackpackRecipeBuilder.unlockedBy(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipeBuilder.java:82)
- [ ] `ShapedBackpackRecipeBuilder.group(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipeBuilder.java:87)
- [ ] `ShapedBackpackRecipeBuilder.showNotification(1)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipeBuilder.java:92)
- [ ] `ShapedBackpackRecipeBuilder.defaultId(0)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipeBuilder.java:97)
- [ ] `ShapedBackpackRecipeBuilder.save(2)`: its class isn't translated (it implements Minecraft's `RecipeBuilder`) (com/tiviacz/travelersbackpack/common/recipes/ShapedBackpackRecipeBuilder.java:101)

### `ServerActions`

- [x] `ServerActions.TAB_OPEN` (com/tiviacz/travelersbackpack/common/ServerActions.java:59)
- [x] `ServerActions.UPGRADE_ENABLED` (com/tiviacz/travelersbackpack/common/ServerActions.java:60)
- [x] `ServerActions.SHIFT_CLICK_TO_BACKPACK` (com/tiviacz/travelersbackpack/common/ServerActions.java:61)
- [x] `ServerActions.PLAY_RECORD` (com/tiviacz/travelersbackpack/common/ServerActions.java:62)
- [x] `ServerActions.SLOT` (com/tiviacz/travelersbackpack/common/ServerActions.java:63)
- [x] `ServerActions.TANK` (com/tiviacz/travelersbackpack/common/ServerActions.java:64)
- [ ] `ServerActions.swapTool(3)`: it uses TravelersBackpackConfig.SERVER, which isn't translated (it uses Server, which isn't translated (it uses BackpackSettings, which isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)))) (com/tiviacz/travelersbackpack/common/ServerActions.java:66)
- [ ] `ServerActions.equipBackpack(2)`: it calls ServerActions.handleEquipBackpack, which isn't translated (com/tiviacz/travelersbackpack/common/ServerActions.java:118)
- [ ] `ServerActions.swapBackpack(1)`: it uses Minecraft's `Level` (com/tiviacz/travelersbackpack/common/ServerActions.java:126)
- [ ] `ServerActions.runAbilitiesRemoval(1)`: it uses BackpackAbilities.ABILITIES, which isn't translated (it uses BackpackAbilities, which isn't translated (it uses Minecraft's `AttributeModifier`)) (com/tiviacz/travelersbackpack/common/ServerActions.java:148)
- [ ] `ServerActions.equipBackpack(1)`: it uses Minecraft's `Level` (com/tiviacz/travelersbackpack/common/ServerActions.java:152)
- [ ] `ServerActions.handleEquipBackpack(1)`: it calls ServerActions.equipBackpack, which isn't translated (com/tiviacz/travelersbackpack/common/ServerActions.java:175)
- [ ] `ServerActions.unequipBackpack(1)`: it uses Minecraft's `Level` (com/tiviacz/travelersbackpack/common/ServerActions.java:181)
- [ ] `ServerActions.handleUnequipBackpack(1)`: it calls ServerActions.unequipBackpack, which isn't translated (com/tiviacz/travelersbackpack/common/ServerActions.java:213)
- [ ] `ServerActions.playEquippingSound(1)`: it calls Minecraft's LocalPlayer.level (no Horizon helper for it yet) (com/tiviacz/travelersbackpack/common/ServerActions.java:219)
- [ ] `ServerActions.openBackpackFromSlot(3)`: it uses Minecraft's `ServerPlayer` (com/tiviacz/travelersbackpack/common/ServerActions.java:231)
- [ ] `ServerActions.openBackpackSettings(3)`: it uses Minecraft's `ServerPlayer` (com/tiviacz/travelersbackpack/common/ServerActions.java:242)
- [ ] `ServerActions.modifyUpgradeTab(5)`: it uses Minecraft's `ServerPlayer` (com/tiviacz/travelersbackpack/common/ServerActions.java:270)
- [ ] `ServerActions.modifyUpgradeTab(4)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/common/ServerActions.java:295)
- [ ] `ServerActions.getPacketType(1)`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/common/ServerActions.java:311)
- [ ] `ServerActions.removeBackpackUpgrade(2)`: it uses Minecraft's `ServerPlayer` (com/tiviacz/travelersbackpack/common/ServerActions.java:321)
- [ ] `ServerActions.switchAbilitySlider(2)`: it uses Minecraft's `ServerPlayer` (com/tiviacz/travelersbackpack/common/ServerActions.java:339)
- [ ] `ServerActions.showToolSlots(2)`: it uses Minecraft's `ServerPlayer` (com/tiviacz/travelersbackpack/common/ServerActions.java:357)
- [ ] `ServerActions.sortBackpack(3)`: it uses Minecraft's `ServerPlayer` (com/tiviacz/travelersbackpack/common/ServerActions.java:363)
- [ ] `ServerActions.toggleVisibility(1)`: `instanceof` (com/tiviacz/travelersbackpack/common/ServerActions.java:369)
- [ ] `ServerActions.toggleButtonsVisibility(1)`: `instanceof` (com/tiviacz/travelersbackpack/common/ServerActions.java:376)
- [ ] `ServerActions.toggleSleepingBag(4)`: it uses Minecraft's `BlockPos` (com/tiviacz/travelersbackpack/common/ServerActions.java:383)
- [ ] `ServerActions.placeAndUseSleepingBag(6)`: it uses Minecraft's `BlockPos` (com/tiviacz/travelersbackpack/common/ServerActions.java:455)
- [ ] `ServerActions.setStack(4)`: `instanceof` (com/tiviacz/travelersbackpack/common/ServerActions.java:496)
- [ ] `ServerActions.throwPotion(4)`: it uses Minecraft's `Level` (com/tiviacz/travelersbackpack/common/ServerActions.java:526)
- [ ] `ServerActions.setFluidEffect(3)`: it uses Minecraft's `Level` (com/tiviacz/travelersbackpack/common/ServerActions.java:546)
- [ ] `ServerActions.switchHoseMode(2)`: `instanceof HoseItem` on an item (no single item id for it) (com/tiviacz/travelersbackpack/common/ServerActions.java:556)
- [ ] `ServerActions.toggleHoseTank(2)`: `instanceof HoseItem` on an item (no single item id for it) (com/tiviacz/travelersbackpack/common/ServerActions.java:569)

### `ComfortsCompat`

- [ ] `ComfortsCompat.getComfortsSleepingBagColor(1)`: it uses Minecraft's `DyeColor`.RED (com/tiviacz/travelersbackpack/compat/comforts/ComfortsCompat.java:7)

### `BackpackCraftingGridAddition`: it implements `ICraftingTweaks`

- [ ] `BackpackCraftingGridAddition.registerCraftingTweaksAddition(0)`: it calls CraftingWidget.setCraftingTweaksAddition, which isn't translated (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridAddition.java:18)
- [ ] `BackpackCraftingGridAddition.addButton(1)`: its class isn't translated (it implements `ICraftingTweaks`) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridAddition.java:22)
- [ ] `BackpackCraftingGridAddition.onCraftingSlotsHidden(0)`: its class isn't translated (it implements `ICraftingTweaks`) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridAddition.java:27)
- [ ] `BackpackCraftingGridAddition.onCraftingSlotsDisplayed(0)`: its class isn't translated (it implements `ICraftingTweaks`) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridAddition.java:35)
- [ ] `BackpackCraftingGridAddition.setScreen(1)`: its class isn't translated (it implements `ICraftingTweaks`) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridAddition.java:60)
- [ ] `BackpackCraftingGridAddition.getButtonX(1)`: its class isn't translated (it implements `ICraftingTweaks`) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridAddition.java:65)
- [ ] `BackpackCraftingGridAddition.getButtonY(2)`: its class isn't translated (it implements `ICraftingTweaks`) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridAddition.java:69)

### `BackpackCraftingGridProvider`: it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)

- [ ] `BackpackCraftingGridProvider.getModId(0)`: its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java:29)
- [ ] `BackpackCraftingGridProvider.requiresServerSide(0)`: its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java:33)
- [ ] `BackpackCraftingGridProvider.handles(1)`: its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java:37)
- [ ] `BackpackCraftingGridProvider.buildCraftingGrids(2)`: its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java:41)
- [ ] `BackpackCraftingGridProvider.clearGrid(4)`: its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java:53)
- [ ] `BackpackCraftingGridProvider.ignoresSlotId(1)`: its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java:82)
- [ ] `BackpackCraftingGridProvider.rotateSlotId(2)`: its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java:86)
- [ ] `BackpackCraftingGridProvider.rotateGrid(4)`: its class isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java:136)

### `BackpackCraftingGridProvider.BackpackBalanceGridHandler`: it implements `GridBalanceHandler` (net.blay09.mods.craftingtweaks.api)

- [ ] `BackpackCraftingGridProvider.BackpackBalanceGridHandler.balanceGrid(3)`: its class isn't translated (it implements `GridBalanceHandler` (net.blay09.mods.craftingtweaks.api)) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java:165)
- [ ] `BackpackCraftingGridProvider.BackpackBalanceGridHandler.spreadGrid(3)`: its class isn't translated (it implements `GridBalanceHandler` (net.blay09.mods.craftingtweaks.api)) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java:228)

### `BackpackCraftingGridProvider.BackpackTransferGridHandler`: it implements `GridTransferHandler` (net.blay09.mods.craftingtweaks.api)

- [ ] `BackpackCraftingGridProvider.BackpackTransferGridHandler.putIntoGrid(5)`: its class isn't translated (it implements `GridTransferHandler` (net.blay09.mods.craftingtweaks.api)) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java:281)
- [ ] `BackpackCraftingGridProvider.BackpackTransferGridHandler.transferIntoGrid(4)`: its class isn't translated (it implements `GridTransferHandler` (net.blay09.mods.craftingtweaks.api)) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java:308)
- [ ] `BackpackCraftingGridProvider.BackpackTransferGridHandler.canTransferFrom(4)`: its class isn't translated (it implements `GridTransferHandler` (net.blay09.mods.craftingtweaks.api)) (com/tiviacz/travelersbackpack/compat/craftingtweaks/BackpackCraftingGridProvider.java:352)

### `CraftingTweaksCompat`

- [ ] `CraftingTweaksCompat.registerCraftingTweaksAddition(0)`: it creates a BackpackCraftingGridProvider, which isn't translated (it implements `CraftingGridProvider` (net.blay09.mods.craftingtweaks.api)) (com/tiviacz/travelersbackpack/compat/craftingtweaks/CraftingTweaksCompat.java:6)
- [ ] `CraftingTweaksCompat.registerCraftingTweaksAdditionClient(0)`: it calls BackpackCraftingGridAddition.registerCraftingTweaksAddition, which isn't translated (com/tiviacz/travelersbackpack/compat/craftingtweaks/CraftingTweaksCompat.java:10)

### `ICraftingTweaks`: it's an interface

- [ ] `ICraftingTweaks.onCraftingSlotsHidden(0)`: it has no body (com/tiviacz/travelersbackpack/compat/craftingtweaks/ICraftingTweaks.java:20)
- [ ] `ICraftingTweaks.onCraftingSlotsDisplayed(0)`: it has no body (com/tiviacz/travelersbackpack/compat/craftingtweaks/ICraftingTweaks.java:22)
- [ ] `ICraftingTweaks.setScreen(1)`: it has no body (com/tiviacz/travelersbackpack/compat/craftingtweaks/ICraftingTweaks.java:24)

### `EntityModelFeaturesCompat`

- [ ] `EntityModelFeaturesCompat.alignModel(4)`: it uses Minecraft's `HumanoidModel` (com/tiviacz/travelersbackpack/compat/emf/EntityModelFeaturesCompat.java:11)

### `BackpackUpgradeCategoryExtension`: it implements `ISmithingCategoryExtension` (mezz.jei.api.recipe.category.extensions.vanilla.smithing)

- [ ] `BackpackUpgradeCategoryExtension.setTemplate(2)`: it's generic (com/tiviacz/travelersbackpack/compat/jei/BackpackUpgradeCategoryExtension.java:17)
- [ ] `BackpackUpgradeCategoryExtension.setBase(2)`: it's generic (com/tiviacz/travelersbackpack/compat/jei/BackpackUpgradeCategoryExtension.java:22)
- [ ] `BackpackUpgradeCategoryExtension.setAddition(2)`: it's generic (com/tiviacz/travelersbackpack/compat/jei/BackpackUpgradeCategoryExtension.java:27)
- [ ] `BackpackUpgradeCategoryExtension.setOutput(2)`: it's generic (com/tiviacz/travelersbackpack/compat/jei/BackpackUpgradeCategoryExtension.java:32)

### `BlockEntityTransferHandler`: it extends `BasicRecipeTransferHandler` (mezz.jei.library.transfer)

- [ ] `BlockEntityTransferHandler.BlockEntityTransferHandler(4)`: its class isn't translated (it extends `BasicRecipeTransferHandler` (mezz.jei.library.transfer)) (com/tiviacz/travelersbackpack/compat/jei/BlockEntityTransferHandler.java:19)
- [ ] `BlockEntityTransferHandler.transferRecipe(6)`: its class isn't translated (it extends `BasicRecipeTransferHandler` (mezz.jei.library.transfer)) (com/tiviacz/travelersbackpack/compat/jei/BlockEntityTransferHandler.java:28)

### `BlockEntityTransferInfo`: it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)

- [ ] `BlockEntityTransferInfo.getContainerClass(0)`: its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)) (com/tiviacz/travelersbackpack/compat/jei/BlockEntityTransferInfo.java:18)
- [ ] `BlockEntityTransferInfo.getMenuType(0)`: its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)) (com/tiviacz/travelersbackpack/compat/jei/BlockEntityTransferInfo.java:22)
- [ ] `BlockEntityTransferInfo.getRecipeType(0)`: its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)) (com/tiviacz/travelersbackpack/compat/jei/BlockEntityTransferInfo.java:26)
- [ ] `BlockEntityTransferInfo.canHandle(2)`: its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)) (com/tiviacz/travelersbackpack/compat/jei/BlockEntityTransferInfo.java:30)
- [ ] `BlockEntityTransferInfo.getRecipeSlots(2)`: its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)) (com/tiviacz/travelersbackpack/compat/jei/BlockEntityTransferInfo.java:34)
- [ ] `BlockEntityTransferInfo.getInventorySlots(2)`: its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)) (com/tiviacz/travelersbackpack/compat/jei/BlockEntityTransferInfo.java:45)

### `ItemTransferHandler`: it extends `BasicRecipeTransferHandler` (mezz.jei.library.transfer)

- [ ] `ItemTransferHandler.ItemTransferHandler(4)`: its class isn't translated (it extends `BasicRecipeTransferHandler` (mezz.jei.library.transfer)) (com/tiviacz/travelersbackpack/compat/jei/ItemTransferHandler.java:19)
- [ ] `ItemTransferHandler.transferRecipe(6)`: its class isn't translated (it extends `BasicRecipeTransferHandler` (mezz.jei.library.transfer)) (com/tiviacz/travelersbackpack/compat/jei/ItemTransferHandler.java:28)

### `ItemTransferInfo`: it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)

- [ ] `ItemTransferInfo.getContainerClass(0)`: its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)) (com/tiviacz/travelersbackpack/compat/jei/ItemTransferInfo.java:19)
- [ ] `ItemTransferInfo.getMenuType(0)`: its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)) (com/tiviacz/travelersbackpack/compat/jei/ItemTransferInfo.java:23)
- [ ] `ItemTransferInfo.getRecipeType(0)`: its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)) (com/tiviacz/travelersbackpack/compat/jei/ItemTransferInfo.java:27)
- [ ] `ItemTransferInfo.canHandle(2)`: its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)) (com/tiviacz/travelersbackpack/compat/jei/ItemTransferInfo.java:31)
- [ ] `ItemTransferInfo.getRecipeSlots(2)`: its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)) (com/tiviacz/travelersbackpack/compat/jei/ItemTransferInfo.java:35)
- [ ] `ItemTransferInfo.getInventorySlots(2)`: its class isn't translated (it implements `IRecipeTransferInfo` (mezz.jei.api.recipe.transfer)) (com/tiviacz/travelersbackpack/compat/jei/ItemTransferInfo.java:46)

### `JeiGhostIngredientHandler`: it implements `IGhostIngredientHandler` (mezz.jei.api.gui.handlers)

- [ ] `JeiGhostIngredientHandler.getTargetsTyped(3)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/compat/jei/JeiGhostIngredientHandler.java:16)
- [ ] `JeiGhostIngredientHandler.onComplete(0)`: its class isn't translated (it implements `IGhostIngredientHandler` (mezz.jei.api.gui.handlers)) (com/tiviacz/travelersbackpack/compat/jei/JeiGhostIngredientHandler.java:40)

### `TravelersBackpackPlugin`: it implements `IModPlugin` (mezz.jei.api)

- [ ] `TravelersBackpackPlugin.registerRecipeTransferHandlers(1)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (com/tiviacz/travelersbackpack/compat/jei/TravelersBackpackPlugin.java:24)
- [ ] `TravelersBackpackPlugin.registerVanillaCategoryExtensions(1)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (com/tiviacz/travelersbackpack/compat/jei/TravelersBackpackPlugin.java:39)
- [ ] `TravelersBackpackPlugin.registerGuiHandlers(1)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/compat/jei/TravelersBackpackPlugin.java:44)
- [ ] `TravelersBackpackPlugin.getPluginUid(0)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (com/tiviacz/travelersbackpack/compat/jei/TravelersBackpackPlugin.java:76)

### `BackpackDynamicLight`: it implements `DynamicLightsInitializer` (dev.lambdaurora.lambdynlights.api)

- [ ] `BackpackDynamicLight.TYPE`: it uses `Type` (dev.lambdaurora.lambdynlights.api.entity.luminance.EntityLuminance) (com/tiviacz/travelersbackpack/compat/lambdynamiclight/BackpackDynamicLight.java:12)
- [ ] `BackpackDynamicLight.onInitializeDynamicLights(1)`: its class isn't translated (it implements `DynamicLightsInitializer` (dev.lambdaurora.lambdynlights.api)) (com/tiviacz/travelersbackpack/compat/lambdynamiclight/BackpackDynamicLight.java:16)

### `BackpackEntityLuminance`: it implements `EntityLuminance` (dev.lambdaurora.lambdynlights.api.entity.luminance)

- [ ] `BackpackEntityLuminance.INSTANCE`: it uses BackpackEntityLuminance, which isn't translated (it implements `EntityLuminance` (dev.lambdaurora.lambdynlights.api.entity.luminance)) (com/tiviacz/travelersbackpack/compat/lambdynamiclight/BackpackEntityLuminance.java:15)
- [ ] `BackpackEntityLuminance.BackpackEntityLuminance(0)`: its class isn't translated (it implements `EntityLuminance` (dev.lambdaurora.lambdynlights.api.entity.luminance)) (com/tiviacz/travelersbackpack/compat/lambdynamiclight/BackpackEntityLuminance.java:17)
- [ ] `BackpackEntityLuminance.type(0)`: its class isn't translated (it implements `EntityLuminance` (dev.lambdaurora.lambdynlights.api.entity.luminance)) (com/tiviacz/travelersbackpack/compat/lambdynamiclight/BackpackEntityLuminance.java:20)
- [ ] `BackpackEntityLuminance.getLuminance(2)`: its class isn't translated (it implements `EntityLuminance` (dev.lambdaurora.lambdynlights.api.entity.luminance)) (com/tiviacz/travelersbackpack/compat/lambdynamiclight/BackpackEntityLuminance.java:24)

### `BackpackDataType`: it extends `GravestoneDataType` (net.pneumono.gravestones.api)

- [x] `BackpackDataType.KEY` (com/tiviacz/travelersbackpack/compat/pneumonogravestones/BackpackDataType.java:18)
- [ ] `BackpackDataType.writeData(3)`: its class isn't translated (it extends `GravestoneDataType` (net.pneumono.gravestones.api)) (com/tiviacz/travelersbackpack/compat/pneumonogravestones/BackpackDataType.java:20)
- [ ] `BackpackDataType.onBreak(5)`: its class isn't translated (it extends `GravestoneDataType` (net.pneumono.gravestones.api)) (com/tiviacz/travelersbackpack/compat/pneumonogravestones/BackpackDataType.java:32)
- [ ] `BackpackDataType.onCollect(6)`: its class isn't translated (it extends `GravestoneDataType` (net.pneumono.gravestones.api)) (com/tiviacz/travelersbackpack/compat/pneumonogravestones/BackpackDataType.java:37)

### `PneumonoGravestonesCompat`

- [ ] `PneumonoGravestonesCompat.register(0)`: it calls `Gravestones` (net.pneumono.gravestones).id (com/tiviacz/travelersbackpack/compat/pneumonogravestones/PneumonoGravestonesCompat.java:7)

### `BackpackLayout`: it implements `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout)

- [x] `BackpackLayout.DEFAULT_SNAP_ID` (com/tiviacz/travelersbackpack/compat/trashslot/BackpackLayout.java:20)
- [ ] `BackpackLayout.DEFAULT_RECT_ID`: it uses `ScreenBoundsProvider` (net.blay09.mods.trashslot.api.layout).SCREEN_ID (com/tiviacz/travelersbackpack/compat/trashslot/BackpackLayout.java:21)
- [x] `BackpackLayout.EXTENDED_TOP_RECT_ID` (com/tiviacz/travelersbackpack/compat/trashslot/BackpackLayout.java:22)
- [x] `BackpackLayout.EXTENDED_BOTTOM_RECT_ID` (com/tiviacz/travelersbackpack/compat/trashslot/BackpackLayout.java:23)
- [ ] `BackpackLayout.EXTENDED_TOP_RECT`: it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/compat/trashslot/BackpackLayout.java:24)
- [ ] `BackpackLayout.EXTENDED_BOTTOM_RECT`: it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`) (com/tiviacz/travelersbackpack/compat/trashslot/BackpackLayout.java:27)
- [ ] `BackpackLayout.getBackpackExtendedLayout(0)`: it uses `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout) (com/tiviacz/travelersbackpack/compat/trashslot/BackpackLayout.java:34)
- [ ] `BackpackLayout.getDefaultLayout(0)`: it uses `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout) (com/tiviacz/travelersbackpack/compat/trashslot/BackpackLayout.java:38)
- [ ] `BackpackLayout.getAvailability(0)`: its class isn't translated (it implements `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout)) (com/tiviacz/travelersbackpack/compat/trashslot/BackpackLayout.java:42)
- [ ] `BackpackLayout.getSnaps(1)`: its class isn't translated (it implements `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout)) (com/tiviacz/travelersbackpack/compat/trashslot/BackpackLayout.java:46)
- [ ] `BackpackLayout.getSnap(2)`: its class isn't translated (it implements `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout)) (com/tiviacz/travelersbackpack/compat/trashslot/BackpackLayout.java:52)
- [ ] `BackpackLayout.getDefaultSnap(1)`: its class isn't translated (it implements `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout)) (com/tiviacz/travelersbackpack/compat/trashslot/BackpackLayout.java:58)
- [ ] `BackpackLayout.getBounds(2)`: its class isn't translated (it implements `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout)) (com/tiviacz/travelersbackpack/compat/trashslot/BackpackLayout.java:62)
- [ ] `BackpackLayout.getAllBounds(1)`: its class isn't translated (it implements `TrashContainerLayout` (net.blay09.mods.trashslot.api.layout)) (com/tiviacz/travelersbackpack/compat/trashslot/BackpackLayout.java:76)

### `TrashSlotCompat`

- [ ] `TrashSlotCompat.register(0)`: it uses `RegisterTrashSlotContainerLayoutsEvent` (net.blay09.mods.trashslot.api.event).EVENT (com/tiviacz/travelersbackpack/compat/trashslot/TrashSlotCompat.java:8)

### `BackpackTrinket`: it implements `TrinketCallback` (eu.pb4.trinkets.api.callback)

- [ ] `BackpackTrinket.init(0)`: it uses Minecraft's `BuiltInRegistries`.ITEM (com/tiviacz/travelersbackpack/compat/trinkets/BackpackTrinket.java:14)
- [ ] `BackpackTrinket.canEquip(3)`: its class isn't translated (it implements `TrinketCallback` (eu.pb4.trinkets.api.callback)) (com/tiviacz/travelersbackpack/compat/trinkets/BackpackTrinket.java:21)
- [ ] `BackpackTrinket.canEquipFromUse(2)`: its class isn't translated (it implements `TrinketCallback` (eu.pb4.trinkets.api.callback)) (com/tiviacz/travelersbackpack/compat/trinkets/BackpackTrinket.java:25)
- [ ] `BackpackTrinket.tick(3)`: its class isn't translated (it implements `TrinketCallback` (eu.pb4.trinkets.api.callback)) (com/tiviacz/travelersbackpack/compat/trinkets/BackpackTrinket.java:29)

### `BackpackTrinketRenderer`: it implements `TrinketRenderer` (eu.pb4.trinkets.api.client)

- [ ] `BackpackTrinketRenderer.initClient(0)`: it uses Minecraft's `BuiltInRegistries`.ITEM (com/tiviacz/travelersbackpack/compat/trinkets/BackpackTrinketRenderer.java:22)
- [ ] `BackpackTrinketRenderer.submit(9)`: its class isn't translated (it implements `TrinketRenderer` (eu.pb4.trinkets.api.client)) (com/tiviacz/travelersbackpack/compat/trinkets/BackpackTrinketRenderer.java:29)

### `UniversalGravesCompat`: it implements `GraveInventoryMask` (eu.pb4.graves.grave)

- [ ] `UniversalGravesCompat.INSTANCE`: it uses `GraveInventoryMask` (eu.pb4.graves.grave) (com/tiviacz/travelersbackpack/compat/universalgraves/UniversalGravesCompat.java:18)
- [ ] `UniversalGravesCompat.register(0)`: it uses UniversalGravesCompat.INSTANCE, which isn't translated (it uses `GraveInventoryMask` (eu.pb4.graves.grave)) (com/tiviacz/travelersbackpack/compat/universalgraves/UniversalGravesCompat.java:20)
- [ ] `UniversalGravesCompat.addToGrave(2)`: its class isn't translated (it implements `GraveInventoryMask` (eu.pb4.graves.grave)) (com/tiviacz/travelersbackpack/compat/universalgraves/UniversalGravesCompat.java:24)
- [ ] `UniversalGravesCompat.moveToPlayerExactly(4)`: its class isn't translated (it implements `GraveInventoryMask` (eu.pb4.graves.grave)) (com/tiviacz/travelersbackpack/compat/universalgraves/UniversalGravesCompat.java:36)
- [ ] `UniversalGravesCompat.moveToPlayerClosest(4)`: its class isn't translated (it implements `GraveInventoryMask` (eu.pb4.graves.grave)) (com/tiviacz/travelersbackpack/compat/universalgraves/UniversalGravesCompat.java:58)

### `ServerboundVinURLStartPacket`: it implements Minecraft's `CustomPacketPayload`

- [ ] `ServerboundVinURLStartPacket.TYPE`: it uses Minecraft's `Type` (com/tiviacz/travelersbackpack/compat/vinurl/ServerboundVinURLStartPacket.java:15)
- [ ] `ServerboundVinURLStartPacket.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/compat/vinurl/ServerboundVinURLStartPacket.java:16)
- [ ] `ServerboundVinURLStartPacket.ServerboundVinURLStartPacket(1)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/compat/vinurl/ServerboundVinURLStartPacket.java:14)
- [ ] `ServerboundVinURLStartPacket.handle(2)`: it uses ServerboundVinURLStartPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/compat/vinurl/ServerboundVinURLStartPacket.java:20)
- [ ] `ServerboundVinURLStartPacket.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/compat/vinurl/ServerboundVinURLStartPacket.java:28)
- [ ] `ServerboundVinURLStartPacket.stack(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/compat/vinurl/ServerboundVinURLStartPacket.java:14)

### `ServerboundVinURLStopPacket`: it implements Minecraft's `CustomPacketPayload`

- [ ] `ServerboundVinURLStopPacket.TYPE`: it uses Minecraft's `Type` (com/tiviacz/travelersbackpack/compat/vinurl/ServerboundVinURLStopPacket.java:16)
- [ ] `ServerboundVinURLStopPacket.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/compat/vinurl/ServerboundVinURLStopPacket.java:17)
- [ ] `ServerboundVinURLStopPacket.ServerboundVinURLStopPacket(2)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/compat/vinurl/ServerboundVinURLStopPacket.java:15)
- [ ] `ServerboundVinURLStopPacket.handle(2)`: it uses ServerboundVinURLStopPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/compat/vinurl/ServerboundVinURLStopPacket.java:21)
- [ ] `ServerboundVinURLStopPacket.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/compat/vinurl/ServerboundVinURLStopPacket.java:29)
- [ ] `ServerboundVinURLStopPacket.stack(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/compat/vinurl/ServerboundVinURLStopPacket.java:15)
- [ ] `ServerboundVinURLStopPacket.cancelable(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/compat/vinurl/ServerboundVinURLStopPacket.java:15)

### `VinURLNetwork`

- [ ] `VinURLNetwork.register(0)`: it calls Fabric's `PayloadTypeRegistry`.serverboundPlay (com/tiviacz/travelersbackpack/compat/vinurl/VinURLNetwork.java:7)

### `Fluids`: it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`)

- [ ] `Fluids.CODEC`: it uses Minecraft's `Codec` (com/tiviacz/travelersbackpack/component/Fluids.java:10)
- [ ] `Fluids.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/component/Fluids.java:17)
- [ ] `Fluids.Fluids(2)`: its class isn't translated (it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`)) (com/tiviacz/travelersbackpack/component/Fluids.java:9)
- [ ] `Fluids.empty(0)`: it uses Fluids, which isn't translated (it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`)) (com/tiviacz/travelersbackpack/component/Fluids.java:21)
- [ ] `Fluids.leftFluidStack(0)`: its class isn't translated (it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`)) (com/tiviacz/travelersbackpack/component/Fluids.java:9)
- [ ] `Fluids.rightFluidStack(0)`: its class isn't translated (it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`)) (com/tiviacz/travelersbackpack/component/Fluids.java:9)

### `RenderInfo`: it uses Minecraft's `CompoundTag`

- [ ] `RenderInfo.EMPTY`: it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`) (com/tiviacz/travelersbackpack/component/RenderInfo.java:13)
- [ ] `RenderInfo.CODEC`: it uses Minecraft's `Codec` (com/tiviacz/travelersbackpack/component/RenderInfo.java:14)
- [ ] `RenderInfo.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/component/RenderInfo.java:17)
- [x] `RenderInfo.LEFT_TANK` (com/tiviacz/travelersbackpack/component/RenderInfo.java:20)
- [x] `RenderInfo.RIGHT_TANK` (com/tiviacz/travelersbackpack/component/RenderInfo.java:21)
- [x] `RenderInfo.CAPACITY` (com/tiviacz/travelersbackpack/component/RenderInfo.java:22)
- [x] `RenderInfo.LANTERN` (com/tiviacz/travelersbackpack/component/RenderInfo.java:23)
- [ ] `RenderInfo.RenderInfo(1)`: its class isn't translated (it uses Minecraft's `CompoundTag`) (com/tiviacz/travelersbackpack/component/RenderInfo.java:12)
- [ ] `RenderInfo.isEmpty(0)`: its class isn't translated (it uses Minecraft's `CompoundTag`) (com/tiviacz/travelersbackpack/component/RenderInfo.java:25)
- [ ] `RenderInfo.hasTanks(0)`: its class isn't translated (it uses Minecraft's `CompoundTag`) (com/tiviacz/travelersbackpack/component/RenderInfo.java:29)
- [ ] `RenderInfo.hasLantern(0)`: its class isn't translated (it uses Minecraft's `CompoundTag`) (com/tiviacz/travelersbackpack/component/RenderInfo.java:33)
- [ ] `RenderInfo.getLeftFluidStack(0)`: its class isn't translated (it uses Minecraft's `CompoundTag`) (com/tiviacz/travelersbackpack/component/RenderInfo.java:37)
- [ ] `RenderInfo.getRightFluidStack(0)`: its class isn't translated (it uses Minecraft's `CompoundTag`) (com/tiviacz/travelersbackpack/component/RenderInfo.java:43)
- [ ] `RenderInfo.updateCapacity(1)`: its class isn't translated (it uses Minecraft's `CompoundTag`) (com/tiviacz/travelersbackpack/component/RenderInfo.java:49)
- [ ] `RenderInfo.getCapacity(0)`: its class isn't translated (it uses Minecraft's `CompoundTag`) (com/tiviacz/travelersbackpack/component/RenderInfo.java:55)
- [ ] `RenderInfo.createCreativeTabInfo(0)`: it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`) (com/tiviacz/travelersbackpack/component/RenderInfo.java:59)
- [ ] `RenderInfo.equals(1)`: its body couldn't be read: expected `;` but found `(` (line 69) (com/tiviacz/travelersbackpack/component/RenderInfo.java:67)
- [ ] `RenderInfo.hashCode(0)`: its class isn't translated (it uses Minecraft's `CompoundTag`) (com/tiviacz/travelersbackpack/component/RenderInfo.java:72)
- [ ] `RenderInfo.compoundTag(0)`: its class isn't translated (it uses Minecraft's `CompoundTag`) (com/tiviacz/travelersbackpack/component/RenderInfo.java:12)

### `Slots`: it uses Minecraft's `Pair`

- [ ] `Slots.EMPTY`: it uses Slots, which isn't translated (it uses Minecraft's `Pair`) (com/tiviacz/travelersbackpack/component/Slots.java:13)
- [ ] `Slots.CODEC`: it uses Minecraft's `Codec` (com/tiviacz/travelersbackpack/component/Slots.java:14)
- [ ] `Slots.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/component/Slots.java:25)
- [ ] `Slots.Slots(2)`: its class isn't translated (it uses Minecraft's `Pair`) (com/tiviacz/travelersbackpack/component/Slots.java:12)
- [ ] `Slots.updateUnsortables(2)`: it uses Slots, which isn't translated (it uses Minecraft's `Pair`) (com/tiviacz/travelersbackpack/component/Slots.java:36)
- [ ] `Slots.updateMemory(2)`: it uses Slots, which isn't translated (it uses Minecraft's `Pair`) (com/tiviacz/travelersbackpack/component/Slots.java:40)
- [ ] `Slots.equals(1)`: its body couldn't be read: expected `;` but found `(` (line 49) (com/tiviacz/travelersbackpack/component/Slots.java:44)
- [ ] `Slots.memoryMatch(1)`: its class isn't translated (it uses Minecraft's `Pair`) (com/tiviacz/travelersbackpack/component/Slots.java:55)
- [ ] `Slots.hashCode(0)`: its class isn't translated (it uses Minecraft's `Pair`) (com/tiviacz/travelersbackpack/component/Slots.java:81)
- [ ] `Slots.hashSlots(2)`: it uses Minecraft's `Pair` (com/tiviacz/travelersbackpack/component/Slots.java:86)
- [ ] `Slots.unsortables(0)`: its class isn't translated (it uses Minecraft's `Pair`) (com/tiviacz/travelersbackpack/component/Slots.java:12)
- [ ] `Slots.memory(0)`: its class isn't translated (it uses Minecraft's `Pair`) (com/tiviacz/travelersbackpack/component/Slots.java:12)

### `BackpackEffect`: it uses Minecraft's `Holder`

- [ ] `BackpackEffect.BackpackEffect(4)`: its class isn't translated (it uses Minecraft's `Holder`) (com/tiviacz/travelersbackpack/config/BackpackEffect.java:6)
- [ ] `BackpackEffect.effect(0)`: its class isn't translated (it uses Minecraft's `Holder`) (com/tiviacz/travelersbackpack/config/BackpackEffect.java:6)
- [ ] `BackpackEffect.minDuration(0)`: its class isn't translated (it uses Minecraft's `Holder`) (com/tiviacz/travelersbackpack/config/BackpackEffect.java:6)
- [ ] `BackpackEffect.maxDuration(0)`: its class isn't translated (it uses Minecraft's `Holder`) (com/tiviacz/travelersbackpack/config/BackpackEffect.java:6)
- [ ] `BackpackEffect.amplifier(0)`: its class isn't translated (it uses Minecraft's `Holder`) (com/tiviacz/travelersbackpack/config/BackpackEffect.java:6)

### `Cooldown`

- [x] `Cooldown.Cooldown(2)` (com/tiviacz/travelersbackpack/config/Cooldown.java:3)
- [x] `Cooldown.minCooldown(0)` (com/tiviacz/travelersbackpack/config/Cooldown.java:3)
- [x] `Cooldown.maxCooldown(0)` (com/tiviacz/travelersbackpack/config/Cooldown.java:3)

### `TravelersBackpackConfig`

- [ ] `TravelersBackpackConfig.serverSpec`: it uses `ModConfigSpec` (net.neoforged.neoforge.common) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:29)
- [ ] `TravelersBackpackConfig.SERVER`: it uses Server, which isn't translated (it uses BackpackSettings, which isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec))) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:30)
- [ ] `TravelersBackpackConfig.commonSpec`: it uses `ModConfigSpec` (net.neoforged.neoforge.common) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:31)
- [ ] `TravelersBackpackConfig.COMMON`: it uses Common, which isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:32)
- [ ] `TravelersBackpackConfig.clientSpec`: it uses `ModConfigSpec` (net.neoforged.neoforge.common) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:33)
- [ ] `TravelersBackpackConfig.CLIENT`: it uses Client, which isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:34)

### `TravelersBackpackConfig.Client`: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)

- [ ] `TravelersBackpackConfig.Client.Client(1)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:57)

### `TravelersBackpackConfig.Client.Overlay`: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)

- [ ] `TravelersBackpackConfig.Client.Overlay.Overlay(6)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:79)

### `TravelersBackpackConfig.Client.ToolsOverlay`: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)

- [ ] `TravelersBackpackConfig.Client.ToolsOverlay.ToolsOverlay(3)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:98)

### `TravelersBackpackConfig.Common`: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)

- [ ] `TravelersBackpackConfig.Common.Common(1)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:123)

### `TravelersBackpackConfig.Server`: it uses BackpackSettings, which isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec))

- [x] `TravelersBackpackConfig.Server.REGISTRY_NAME_MATCHER` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:132)
- [ ] `TravelersBackpackConfig.Server.Server(1)`: it uses `Builder` (net.neoforged.neoforge.common.ModConfigSpec) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:139)
- [ ] `TravelersBackpackConfig.Server.loadItemsFromConfig(2)`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:149)
- [ ] `TravelersBackpackConfig.Server.loadEntityTypesFromConfig(2)`: it uses Minecraft's `EntityType` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:160)
- [ ] `TravelersBackpackConfig.Server.loadBackpackEffectsFromConfig(2)`: it uses `Multimap` (com.google.common.collect) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:175)
- [ ] `TravelersBackpackConfig.Server.loadCooldownsFromConfig(2)`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:210)
- [ ] `TravelersBackpackConfig.Server.reload(0)`: it uses TravelersBackpackConfig.serverSpec, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:239)

### `TravelersBackpackConfig.Server.BackpackAbilities`: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)

- [x] `TravelersBackpackConfig.Server.BackpackAbilities.REGISTRY_NAME_MATCHER` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:284)
- [x] `TravelersBackpackConfig.Server.BackpackAbilities.EFFECT_ABILITY_MATCHER` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:285)
- [x] `TravelersBackpackConfig.Server.BackpackAbilities.COOLDOWNS_MATCHER` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:286)
- [ ] `TravelersBackpackConfig.Server.BackpackAbilities.BackpackAbilities(2)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:293)
- [ ] `TravelersBackpackConfig.Server.BackpackAbilities.getAllowedAbilities(0)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:319)
- [ ] `TravelersBackpackConfig.Server.BackpackAbilities.getCooldowns(0)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:354)
- [ ] `TravelersBackpackConfig.Server.BackpackAbilities.getBackpackEffects(0)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:364)

### `TravelersBackpackConfig.Server.BackpackSettings`: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)

- [ ] `TravelersBackpackConfig.Server.BackpackSettings.BackpackSettings(2)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:405)

### `TravelersBackpackConfig.Server.BackpackSettings.Tier`

- [x] `TravelersBackpackConfig.Server.BackpackSettings.Tier.Tier(3)` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:462)
- [x] `TravelersBackpackConfig.Server.BackpackSettings.Tier.inventorySlotCount(0)` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:462)
- [x] `TravelersBackpackConfig.Server.BackpackSettings.Tier.toolSlotCount(0)` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:462)
- [x] `TravelersBackpackConfig.Server.BackpackSettings.Tier.tankCapacity(0)` (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:462)

### `TravelersBackpackConfig.Server.BackpackSettings.TierConfig`: it uses `IntValue` (net.neoforged.neoforge.common.ModConfigSpec)

- [ ] `TravelersBackpackConfig.Server.BackpackSettings.TierConfig.TierConfig(6)`: its class isn't translated (it uses `IntValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:471)

### `TravelersBackpackConfig.Server.BackpackUpgrades`: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)

- [ ] `TravelersBackpackConfig.Server.BackpackUpgrades.BackpackUpgrades(2)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:509)

### `TravelersBackpackConfig.Server.BackpackUpgrades.FeedingUpgradeSettings`: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)

- [ ] `TravelersBackpackConfig.Server.BackpackUpgrades.FeedingUpgradeSettings.FeedingUpgradeSettings(2)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:535)

### `TravelersBackpackConfig.Server.BackpackUpgrades.FilterUpgradeSettings`: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)

- [ ] `TravelersBackpackConfig.Server.BackpackUpgrades.FilterUpgradeSettings.FilterUpgradeSettings(3)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:550)

### `TravelersBackpackConfig.Server.BackpackUpgrades.MagnetUpgradeSettings`: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)

- [ ] `TravelersBackpackConfig.Server.BackpackUpgrades.MagnetUpgradeSettings.MagnetUpgradeSettings(2)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:566)

### `TravelersBackpackConfig.Server.BackpackUpgrades.RefillUpgradeSettings`: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)

- [ ] `TravelersBackpackConfig.Server.BackpackUpgrades.RefillUpgradeSettings.RefillUpgradeSettings(2)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:583)

### `TravelersBackpackConfig.Server.SlownessDebuff`: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)

- [ ] `TravelersBackpackConfig.Server.SlownessDebuff.SlownessDebuff(2)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:599)

### `TravelersBackpackConfig.Server.World`: it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)

- [ ] `TravelersBackpackConfig.Server.World.World(2)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:618)
- [ ] `TravelersBackpackConfig.Server.World.getPossibleOverworldEntityTypes(0)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:652)
- [ ] `TravelersBackpackConfig.Server.World.getPossibleNetherEntityTypes(0)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:660)
- [ ] `TravelersBackpackConfig.Server.World.getOverworldBackpacksList(0)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:667)
- [ ] `TravelersBackpackConfig.Server.World.getNetherBackpacksList(0)`: its class isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/tiviacz/travelersbackpack/config/TravelersBackpackConfig.java:703)

### `ModBlockLootTables`: it extends Fabric's `FabricBlockLootSubProvider`

- [ ] `ModBlockLootTables.ModBlockLootTables(2)`: its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`) (com/tiviacz/travelersbackpack/datagen/ModBlockLootTables.java:24)
- [ ] `ModBlockLootTables.generate(0)`: its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`) (com/tiviacz/travelersbackpack/datagen/ModBlockLootTables.java:28)
- [ ] `ModBlockLootTables.createBackpackDrop(1)`: its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`) (com/tiviacz/travelersbackpack/datagen/ModBlockLootTables.java:51)
- [ ] `ModBlockLootTables.createSleepingBagDrop(1)`: its class isn't translated (it extends Fabric's `FabricBlockLootSubProvider`) (com/tiviacz/travelersbackpack/datagen/ModBlockLootTables.java:86)

### `ModDataGeneration`: it implements Fabric's `DataGeneratorEntrypoint`

- [ ] `ModDataGeneration.onInitializeDataGenerator(1)`: its class isn't translated (it implements Fabric's `DataGeneratorEntrypoint`) (com/tiviacz/travelersbackpack/datagen/ModDataGeneration.java:8)

### `ModRecipeProvider`: it extends Fabric's `FabricRecipeProvider`

- [ ] `ModRecipeProvider.BACKPACKS`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/datagen/ModRecipeProvider.java:40)
- [ ] `ModRecipeProvider.ModRecipeProvider(2)`: its class isn't translated (it extends Fabric's `FabricRecipeProvider`) (com/tiviacz/travelersbackpack/datagen/ModRecipeProvider.java:87)
- [ ] `ModRecipeProvider.createRecipeProvider(3)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/datagen/ModRecipeProvider.java:91)
- [ ] `ModRecipeProvider.getName(0)`: its class isn't translated (it extends Fabric's `FabricRecipeProvider`) (com/tiviacz/travelersbackpack/datagen/ModRecipeProvider.java:779)

### `BackpackItemEntity`: it extends Minecraft's `ItemEntity`

- [ ] `BackpackItemEntity.BackpackItemEntity(2)`: its class isn't translated (it extends Minecraft's `ItemEntity`) (com/tiviacz/travelersbackpack/entity/BackpackItemEntity.java:20)
- [ ] `BackpackItemEntity.tick(0)`: its class isn't translated (it extends Minecraft's `ItemEntity`) (com/tiviacz/travelersbackpack/entity/BackpackItemEntity.java:32)
- [ ] `BackpackItemEntity.isInWater(0)`: its class isn't translated (it extends Minecraft's `ItemEntity`) (com/tiviacz/travelersbackpack/entity/BackpackItemEntity.java:72)
- [ ] `BackpackItemEntity.fireImmune(0)`: its class isn't translated (it extends Minecraft's `ItemEntity`) (com/tiviacz/travelersbackpack/entity/BackpackItemEntity.java:78)
- [ ] `BackpackItemEntity.ignoreExplosion(1)`: its class isn't translated (it extends Minecraft's `ItemEntity`) (com/tiviacz/travelersbackpack/entity/BackpackItemEntity.java:82)
- [ ] `BackpackItemEntity.isInvulnerable(0)`: its class isn't translated (it extends Minecraft's `ItemEntity`) (com/tiviacz/travelersbackpack/entity/BackpackItemEntity.java:86)
- [ ] `BackpackItemEntity.onBelowWorld(0)`: its class isn't translated (it extends Minecraft's `ItemEntity`) (com/tiviacz/travelersbackpack/entity/BackpackItemEntity.java:90)

### `EffectFluidRegistry`

- [ ] `EffectFluidRegistry.EFFECT_REGISTRY`: it uses `BiMap` (com.google.common.collect) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:28)
- [ ] `EffectFluidRegistry.WATER_EFFECT`: it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:29)
- [ ] `EffectFluidRegistry.LAVA_EFFECT`: it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:30)
- [ ] `EffectFluidRegistry.POTION_EFFECT`: it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:31)
- [ ] `EffectFluidRegistry.MILK_EFFECT`: it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:32)
- [ ] `EffectFluidRegistry.TAN_POTION_EFFECT`: it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:33)
- [ ] `EffectFluidRegistry.TAN_WATER_CANTEEN_EFFECT`: it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:34)
- [ ] `EffectFluidRegistry.TAN_WATER_EFFECT`: it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:35)
- [x] `EffectFluidRegistry.effectIDCounter` (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:36)
- [ ] `EffectFluidRegistry.initEffects(0)`: it uses EffectFluidRegistry.EFFECT_REGISTRY, which isn't translated (it uses `BiMap` (com.google.common.collect)) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:38)
- [ ] `EffectFluidRegistry.registerFluidEffect(1)`: it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:48)
- [ ] `EffectFluidRegistry.getFluidName(1)`: it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:70)
- [ ] `EffectFluidRegistry.getRegisteredFluidEffects(0)`: it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:79)
- [ ] `EffectFluidRegistry.getHighestFluidEffectAmount(1)`: it uses Minecraft's `Fluid` (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:83)
- [ ] `EffectFluidRegistry.getEffectsForFluid(1)`: it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:95)
- [ ] `EffectFluidRegistry.hasEffects(1)`: it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:107)
- [ ] `EffectFluidRegistry.hasExecutableEffects(3)`: it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:112)
- [ ] `EffectFluidRegistry.getExecutableEffects(3)`: it uses EffectFluid, which isn't translated (it uses Minecraft's `Fluid`) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:117)
- [ ] `EffectFluidRegistry.executeEffects(3)`: it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:129)
- [x] `EffectFluidRegistry.canInitialize(1)` (com/tiviacz/travelersbackpack/fluids/EffectFluidRegistry.java:137)

### `LavaEffect`: it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)

- [ ] `LavaEffect.LavaEffect(0)`: its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)) (com/tiviacz/travelersbackpack/fluids/effects/LavaEffect.java:13)
- [ ] `LavaEffect.affectDrinker(3)`: its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)) (com/tiviacz/travelersbackpack/fluids/effects/LavaEffect.java:17)
- [ ] `LavaEffect.canExecuteEffect(3)`: its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)) (com/tiviacz/travelersbackpack/fluids/effects/LavaEffect.java:28)

### `MilkEffect`: it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)

- [ ] `MilkEffect.MilkEffect(0)`: its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)) (com/tiviacz/travelersbackpack/fluids/effects/MilkEffect.java:10)
- [ ] `MilkEffect.affectDrinker(3)`: its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)) (com/tiviacz/travelersbackpack/fluids/effects/MilkEffect.java:14)
- [ ] `MilkEffect.canExecuteEffect(3)`: its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)) (com/tiviacz/travelersbackpack/fluids/effects/MilkEffect.java:21)

### `PotionEffect`: it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)

- [ ] `PotionEffect.PotionEffect(2)`: its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)) (com/tiviacz/travelersbackpack/fluids/effects/PotionEffect.java:16)
- [ ] `PotionEffect.PotionEffect(3)`: its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)) (com/tiviacz/travelersbackpack/fluids/effects/PotionEffect.java:20)
- [ ] `PotionEffect.affectDrinker(3)`: its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)) (com/tiviacz/travelersbackpack/fluids/effects/PotionEffect.java:24)
- [ ] `PotionEffect.canExecuteEffect(3)`: its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)) (com/tiviacz/travelersbackpack/fluids/effects/PotionEffect.java:38)

### `WaterEffect`: it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)

- [ ] `WaterEffect.WaterEffect(0)`: its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)) (com/tiviacz/travelersbackpack/fluids/effects/WaterEffect.java:15)
- [ ] `WaterEffect.affectDrinker(3)`: its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)) (com/tiviacz/travelersbackpack/fluids/effects/WaterEffect.java:19)
- [ ] `WaterEffect.canExecuteEffect(3)`: its class isn't translated (it extends `EffectFluid` (com.tiviacz.travelersbackpack.api.fluids)) (com/tiviacz/travelersbackpack/fluids/effects/WaterEffect.java:32)

### `PotionFluid`: it extends Minecraft's `FlowingFluid`

- [ ] `PotionFluid.getFlowing(0)`: its class isn't translated (it extends Minecraft's `FlowingFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:29)
- [ ] `PotionFluid.getSource(0)`: its class isn't translated (it extends Minecraft's `FlowingFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:33)
- [ ] `PotionFluid.getBucket(0)`: its class isn't translated (it extends Minecraft's `FlowingFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:37)
- [ ] `PotionFluid.getDripParticle(0)`: its class isn't translated (it extends Minecraft's `FlowingFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:41)
- [ ] `PotionFluid.beforeDestroyingBlock(3)`: its class isn't translated (it extends Minecraft's `FlowingFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:46)
- [ ] `PotionFluid.getSlopeFindDistance(1)`: its class isn't translated (it extends Minecraft's `FlowingFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:51)
- [ ] `PotionFluid.createLegacyBlock(1)`: its class isn't translated (it extends Minecraft's `FlowingFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:55)
- [ ] `PotionFluid.isSame(1)`: its class isn't translated (it extends Minecraft's `FlowingFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:59)
- [ ] `PotionFluid.getDropOff(1)`: its class isn't translated (it extends Minecraft's `FlowingFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:63)
- [ ] `PotionFluid.getTickDelay(1)`: its class isn't translated (it extends Minecraft's `FlowingFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:67)
- [ ] `PotionFluid.canBeReplacedWith(5)`: its class isn't translated (it extends Minecraft's `FlowingFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:71)
- [ ] `PotionFluid.getExplosionResistance(0)`: its class isn't translated (it extends Minecraft's `FlowingFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:75)
- [ ] `PotionFluid.getPickupSound(0)`: its class isn't translated (it extends Minecraft's `FlowingFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:79)

### `PotionFluid.Flowing`: it extends `PotionFluid`

- [ ] `PotionFluid.Flowing.createFluidStateDefinition(1)`: its class isn't translated (it extends `PotionFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:84)
- [ ] `PotionFluid.Flowing.canConvertToSource(1)`: its class isn't translated (it extends `PotionFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:89)
- [ ] `PotionFluid.Flowing.isSource(1)`: its class isn't translated (it extends `PotionFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:93)
- [ ] `PotionFluid.Flowing.getAmount(1)`: its class isn't translated (it extends `PotionFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:97)

### `PotionFluid.Still`: it extends `PotionFluid`

- [ ] `PotionFluid.Still.isSource(1)`: its class isn't translated (it extends `PotionFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:103)
- [ ] `PotionFluid.Still.canConvertToSource(1)`: its class isn't translated (it extends `PotionFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:107)
- [ ] `PotionFluid.Still.getAmount(1)`: its class isn't translated (it extends `PotionFluid`) (com/tiviacz/travelersbackpack/fluids/PotionFluid.java:111)

### `PotionFluidVariantAttributeHandler`: it implements Fabric's `FluidVariantAttributeHandler`

- [ ] `PotionFluidVariantAttributeHandler.getName(1)`: its class isn't translated (it implements Fabric's `FluidVariantAttributeHandler`) (com/tiviacz/travelersbackpack/fluids/potion/PotionFluidVariantAttributeHandler.java:11)
- [ ] `PotionFluidVariantAttributeHandler.getTranslationKey(1)`: its class isn't translated (it implements Fabric's `FluidVariantAttributeHandler`) (com/tiviacz/travelersbackpack/fluids/potion/PotionFluidVariantAttributeHandler.java:15)

### `PotionFluidVariantRenderHandler`: it implements Fabric's `FluidVariantRenderHandler`

- [x] `PotionFluidVariantRenderHandler.EMPTY_COLOR` (com/tiviacz/travelersbackpack/fluids/potion/PotionFluidVariantRenderHandler.java:15)
- [ ] `PotionFluidVariantRenderHandler.getColor(3)`: its class isn't translated (it implements Fabric's `FluidVariantRenderHandler`) (com/tiviacz/travelersbackpack/fluids/potion/PotionFluidVariantRenderHandler.java:17)

### `DeathHandler`

- [ ] `DeathHandler.registerListeners(0)`: it uses Fabric's `ServerLivingEntityEvents`.ALLOW_DEATH (com/tiviacz/travelersbackpack/handlers/DeathHandler.java:27)

### `EntityItemHandler`

- [x] `EntityItemHandler.registerListeners(0)` (com/tiviacz/travelersbackpack/handlers/EntityItemHandler.java:4)

### `KeybindHandler`

- [x] `KeybindHandler.TRAVELERS_BACKPACK_PHASE` (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java:37)
- [ ] `KeybindHandler.CATEGORY`: done another way in Horizon: the mod's keys in Options > Mods, read through Horizon's key hook (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java:38)
- [ ] `KeybindHandler.OPEN_BACKPACK`: done another way in Horizon: the mod's keys in Options > Mods, read through Horizon's key hook (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java:39)
- [ ] `KeybindHandler.SORT_BACKPACK`: done another way in Horizon: the mod's keys in Options > Mods, read through Horizon's key hook (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java:40)
- [ ] `KeybindHandler.ABILITY`: done another way in Horizon: the mod's keys in Options > Mods, read through Horizon's key hook (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java:41)
- [ ] `KeybindHandler.SWAP_TOOL`: done another way in Horizon: the mod's keys in Options > Mods, read through Horizon's key hook (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java:42)
- [ ] `KeybindHandler.TOGGLE_UPGRADE_0`: done another way in Horizon: the mod's keys in Options > Mods, read through Horizon's key hook (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java:43)
- [ ] `KeybindHandler.TOGGLE_UPGRADE_1`: done another way in Horizon: the mod's keys in Options > Mods, read through Horizon's key hook (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java:44)
- [ ] `KeybindHandler.TOGGLE_UPGRADE_2`: done another way in Horizon: the mod's keys in Options > Mods, read through Horizon's key hook (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java:45)
- [ ] `KeybindHandler.TOGGLE_UPGRADE_3`: done another way in Horizon: the mod's keys in Options > Mods, read through Horizon's key hook (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java:46)
- [ ] `KeybindHandler.TOGGLE_UPGRADE_KEYS`: done another way in Horizon: the mod's keys in Options > Mods, read through Horizon's key hook (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java:47)
- [ ] `KeybindHandler.initKeybinds(0)`: it calls Fabric's `KeyMappingHelper`.registerKeyMapping (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java:49)
- [ ] `KeybindHandler.registerListener(0)`: it uses Fabric's `ClientTickEvents`.START_CLIENT_TICK (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java:60)
- [ ] `KeybindHandler.isKeyDown(1)`: Minecraft's KeyMapping kept in a variable or field (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java:141)
- [ ] `KeybindHandler.sdlButtonMask(1)`: it uses `SDLMouse` (org.lwjgl.sdl).SDL_BUTTON_LMASK (com/tiviacz/travelersbackpack/handlers/KeybindHandler.java:153)

### `LootHandler`

- [ ] `LootHandler.registerListeners(0)`: it uses TravelersBackpackConfig.COMMON, which isn't translated (it uses Common, which isn't translated (it uses `BooleanValue` (net.neoforged.neoforge.common.ModConfigSpec))) (com/tiviacz/travelersbackpack/handlers/LootHandler.java:22)
- [ ] `LootHandler.addLootPool(3)`: it uses Minecraft's `Builder` (com/tiviacz/travelersbackpack/handlers/LootHandler.java:84)
- [ ] `LootHandler.addRandomDyedBackpackLoot(2)`: it uses Minecraft's `Builder` (com/tiviacz/travelersbackpack/handlers/LootHandler.java:88)
- [ ] `LootHandler.addBackpackWithTanksLoot(3)`: it uses Minecraft's `Builder` (com/tiviacz/travelersbackpack/handlers/LootHandler.java:112)

### `RightClickHandler`

- [ ] `RightClickHandler.UPGRADES`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/handlers/RightClickHandler.java:53)
- [ ] `RightClickHandler.registerListeners(0)`: it uses Fabric's `UseBlockCallback`.EVENT (com/tiviacz/travelersbackpack/handlers/RightClickHandler.java:57)
- [ ] `RightClickHandler.getUpgrades(1)`: it uses Minecraft's `NonNullList` (com/tiviacz/travelersbackpack/handlers/RightClickHandler.java:286)

### `ScreenRenderHandler`

- [ ] `ScreenRenderHandler.renderAboveContents(4)`: it uses Minecraft's `AbstractContainerScreen` (com/tiviacz/travelersbackpack/handlers/ScreenRenderHandler.java:37)
- [ ] `ScreenRenderHandler.registerScreenEvents(0)`: it uses Fabric's `ScreenEvents`.AFTER_INIT (com/tiviacz/travelersbackpack/handlers/ScreenRenderHandler.java:141)

### `SleepHandler`

- [ ] `SleepHandler.registerListener(0)`: it uses Fabric's `EntitySleepEvents`.ALLOW_SETTING_SPAWN (com/tiviacz/travelersbackpack/handlers/SleepHandler.java:16)

### `TickHandler`

- [x] `TickHandler.nextSupportersFetch` (com/tiviacz/travelersbackpack/handlers/TickHandler.java:8)
- [ ] `TickHandler.register(0)`: it uses Fabric's `ServerTickEvents`.END_SERVER_TICK (com/tiviacz/travelersbackpack/handlers/TickHandler.java:10)

### `ModAdvancements`

- [ ] `ModAdvancements.ACTION_TRIGGER`: it uses ActionTypeTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`) (com/tiviacz/travelersbackpack/init/ModAdvancements.java:9)
- [ ] `ModAdvancements.init(0)`: it uses ModAdvancements.ACTION_TRIGGER, which isn't translated (it uses ActionTypeTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`)) (com/tiviacz/travelersbackpack/init/ModAdvancements.java:11)

### `ModAttachmentTypes`

- [ ] `ModAttachmentTypes.TRAVELERS_BACKPACK`: it uses Fabric's `AttachmentType` (com/tiviacz/travelersbackpack/init/ModAttachmentTypes.java:11)
- [x] `ModAttachmentTypes.init(0)` (com/tiviacz/travelersbackpack/init/ModAttachmentTypes.java:16)

### `ModBlockEntityTypes`

- [ ] `ModBlockEntityTypes.BACKPACK`: it uses Minecraft's `BlockEntityType` (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:34)
- [ ] `ModBlockEntityTypes.init(0)`: it uses ModBlockEntityTypes.BACKPACK, which isn't translated (it uses Minecraft's `BlockEntityType`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:36)
- [ ] `ModBlockEntityTypes.initSidedStorage(0)`: it uses Fabric's `FluidStorage`.SIDED (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:93)
- [ ] `ModBlockEntityTypes.getProperTank(2)`: it uses Fabric's `SingleVariantStorage` (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:98)
- [ ] `ModBlockEntityTypes.getProperInventory(2)`: it uses Fabric's `Storage` (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:148)

### `ModBlockEntityTypes.BackpackStorage`: it extends Fabric's `SnapshotParticipant`

- [ ] `ModBlockEntityTypes.BackpackStorage.BackpackStorage(1)`: its class isn't translated (it extends Fabric's `SnapshotParticipant`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:158)
- [ ] `ModBlockEntityTypes.BackpackStorage.insert(3)`: its class isn't translated (it extends Fabric's `SnapshotParticipant`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:163)
- [ ] `ModBlockEntityTypes.BackpackStorage.extract(3)`: its class isn't translated (it extends Fabric's `SnapshotParticipant`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:183)
- [ ] `ModBlockEntityTypes.BackpackStorage.getSlotCount(0)`: its class isn't translated (it extends Fabric's `SnapshotParticipant`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:203)
- [ ] `ModBlockEntityTypes.BackpackStorage.getSlot(1)`: its class isn't translated (it extends Fabric's `SnapshotParticipant`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:207)
- [ ] `ModBlockEntityTypes.BackpackStorage.iterator(0)`: its class isn't translated (it extends Fabric's `SnapshotParticipant`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:215)
- [ ] `ModBlockEntityTypes.BackpackStorage.createSnapshot(0)`: its class isn't translated (it extends Fabric's `SnapshotParticipant`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:219)
- [ ] `ModBlockEntityTypes.BackpackStorage.readSnapshot(1)`: its class isn't translated (it extends Fabric's `SnapshotParticipant`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:229)

### `ModBlockEntityTypes.BackpackStorage.BackpackStorageView`: it implements Fabric's `SingleSlotStorage`

- [ ] `ModBlockEntityTypes.BackpackStorage.BackpackStorageView.BackpackStorageView(1)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:238)
- [ ] `ModBlockEntityTypes.BackpackStorage.BackpackStorageView.insert(3)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:242)
- [ ] `ModBlockEntityTypes.BackpackStorage.BackpackStorageView.extract(3)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:251)
- [ ] `ModBlockEntityTypes.BackpackStorage.BackpackStorageView.isResourceBlank(0)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:265)
- [ ] `ModBlockEntityTypes.BackpackStorage.BackpackStorageView.getResource(0)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:269)
- [ ] `ModBlockEntityTypes.BackpackStorage.BackpackStorageView.getAmount(0)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:273)
- [ ] `ModBlockEntityTypes.BackpackStorage.BackpackStorageView.getCapacity(0)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/init/ModBlockEntityTypes.java:277)

### `ModBlocks`

- [ ] `ModBlocks.STANDARD_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:17)
- [ ] `ModBlocks.NETHERITE_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:18)
- [ ] `ModBlocks.DIAMOND_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:19)
- [ ] `ModBlocks.GOLD_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:20)
- [ ] `ModBlocks.EMERALD_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:21)
- [ ] `ModBlocks.IRON_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:22)
- [ ] `ModBlocks.LAPIS_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:23)
- [ ] `ModBlocks.REDSTONE_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:24)
- [ ] `ModBlocks.COAL_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:25)
- [ ] `ModBlocks.QUARTZ_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:26)
- [ ] `ModBlocks.BOOKSHELF_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:27)
- [ ] `ModBlocks.END_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:28)
- [ ] `ModBlocks.NETHER_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:29)
- [ ] `ModBlocks.SANDSTONE_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:30)
- [ ] `ModBlocks.SNOW_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:31)
- [ ] `ModBlocks.SPONGE_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:32)
- [ ] `ModBlocks.CAKE_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:33)
- [ ] `ModBlocks.CACTUS_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:34)
- [ ] `ModBlocks.HAY_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:35)
- [ ] `ModBlocks.MELON_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:36)
- [ ] `ModBlocks.PUMPKIN_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:37)
- [ ] `ModBlocks.CREEPER_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:38)
- [ ] `ModBlocks.DRAGON_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:39)
- [ ] `ModBlocks.ENDERMAN_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:40)
- [ ] `ModBlocks.BLAZE_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:41)
- [ ] `ModBlocks.GHAST_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:42)
- [ ] `ModBlocks.MAGMA_CUBE_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:43)
- [ ] `ModBlocks.SKELETON_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:44)
- [ ] `ModBlocks.SPIDER_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:45)
- [ ] `ModBlocks.WITHER_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:46)
- [ ] `ModBlocks.WARDEN_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:47)
- [ ] `ModBlocks.BAT_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:48)
- [ ] `ModBlocks.BEE_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:49)
- [ ] `ModBlocks.WOLF_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:50)
- [ ] `ModBlocks.FOX_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:51)
- [ ] `ModBlocks.OCELOT_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:52)
- [ ] `ModBlocks.HORSE_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:53)
- [ ] `ModBlocks.COW_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:54)
- [ ] `ModBlocks.PIG_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:55)
- [ ] `ModBlocks.SHEEP_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:56)
- [ ] `ModBlocks.CHICKEN_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:57)
- [ ] `ModBlocks.SQUID_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:58)
- [ ] `ModBlocks.VILLAGER_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:59)
- [ ] `ModBlocks.IRON_GOLEM_TRAVELERS_BACKPACK`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:60)
- [ ] `ModBlocks.WHITE_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:61)
- [ ] `ModBlocks.ORANGE_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:62)
- [ ] `ModBlocks.MAGENTA_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:63)
- [ ] `ModBlocks.LIGHT_BLUE_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:64)
- [ ] `ModBlocks.YELLOW_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:65)
- [ ] `ModBlocks.LIME_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:66)
- [ ] `ModBlocks.PINK_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:67)
- [ ] `ModBlocks.GRAY_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:68)
- [ ] `ModBlocks.LIGHT_GRAY_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:69)
- [ ] `ModBlocks.CYAN_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:70)
- [ ] `ModBlocks.PURPLE_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:71)
- [ ] `ModBlocks.BLUE_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:72)
- [ ] `ModBlocks.BROWN_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:73)
- [ ] `ModBlocks.GREEN_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:74)
- [ ] `ModBlocks.RED_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:75)
- [ ] `ModBlocks.BLACK_SLEEPING_BAG`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModBlocks.java:76)
- [ ] `ModBlocks.init(0)`: it uses ModBlocks.STANDARD_TRAVELERS_BACKPACK, which isn't translated (it uses Minecraft's `Block`) (com/tiviacz/travelersbackpack/init/ModBlocks.java:78)
- [x] `ModBlocks.resourceLocation(1)` (com/tiviacz/travelersbackpack/init/ModBlocks.java:387)
- [ ] `ModBlocks.resourceKey(1)`: it uses Minecraft's `ResourceKey` (com/tiviacz/travelersbackpack/init/ModBlocks.java:391)

### `ModCommands`

- [ ] `ModCommands.registerCommands(0)`: it uses Fabric's `CommandRegistrationCallback`.EVENT (com/tiviacz/travelersbackpack/init/ModCommands.java:10)

### `ModDataComponents`

- [ ] `ModDataComponents.TIER`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:16)
- [ ] `ModDataComponents.STORAGE_SLOTS`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:17)
- [ ] `ModDataComponents.UPGRADE_SLOTS`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:21)
- [ ] `ModDataComponents.TOOL_SLOTS`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:25)
- [ ] `ModDataComponents.FLUIDS`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:26)
- [ ] `ModDataComponents.TAB_OPEN`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:27)
- [ ] `ModDataComponents.FILTER_SETTINGS`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:28)
- [ ] `ModDataComponents.FILTER_TAGS`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:32)
- [ ] `ModDataComponents.UPGRADE_ENABLED`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:36)
- [ ] `ModDataComponents.SHIFT_CLICK_TO_BACKPACK`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:40)
- [ ] `ModDataComponents.SHOW_TOOL_SLOTS`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:44)
- [ ] `ModDataComponents.SHOW_MORE_BUTTONS`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:48)
- [ ] `ModDataComponents.IS_PLAYING`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:52)
- [ ] `ModDataComponents.COOLDOWN`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:56)
- [ ] `ModDataComponents.STARTER_UPGRADES`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:57)
- [ ] `ModDataComponents.RENDER_INFO`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:61)
- [ ] `ModDataComponents.BACKPACK_CONTAINER`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:65)
- [ ] `ModDataComponents.UPGRADES`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:69)
- [ ] `ModDataComponents.TOOLS_CONTAINER`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:73)
- [ ] `ModDataComponents.SLEEPING_BAG_COLOR`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:77)
- [ ] `ModDataComponents.SORT_TYPE`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:81)
- [ ] `ModDataComponents.ABILITY_ENABLED`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:82)
- [ ] `ModDataComponents.IS_VISIBLE`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:86)
- [ ] `ModDataComponents.SLOTS`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:90)
- [ ] `ModDataComponents.UPGRADE_TICK_INTERVAL`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:91)
- [ ] `ModDataComponents.HOSE_MODES`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:95)
- [ ] `ModDataComponents.BURN_FINISH_TIME`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:99)
- [ ] `ModDataComponents.COOKING_FINISH_TIME`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:103)
- [ ] `ModDataComponents.BURN_TOTAL_TIME`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:107)
- [ ] `ModDataComponents.COOKING_TOTAL_TIME`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:111)
- [ ] `ModDataComponents.SPEED_MULTIPLIER`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/init/ModDataComponents.java:115)
- [ ] `ModDataComponents.init(0)`: it uses Minecraft's `BuiltInRegistries`.DATA_COMPONENT_TYPE (com/tiviacz/travelersbackpack/init/ModDataComponents.java:120)

### `ModFluids`

- [ ] `ModFluids.POTION_STILL`: it uses Minecraft's `FlowingFluid` (com/tiviacz/travelersbackpack/init/ModFluids.java:10)
- [ ] `ModFluids.POTION_FLOWING`: it uses Minecraft's `FlowingFluid` (com/tiviacz/travelersbackpack/init/ModFluids.java:11)
- [ ] `ModFluids.register(2)`: it uses Minecraft's `FlowingFluid` (com/tiviacz/travelersbackpack/init/ModFluids.java:13)

### `ModItemGroups`

- [ ] `ModItemGroups.TRAVELERS_BACKPACK`: it uses Minecraft's `ResourceKey` (com/tiviacz/travelersbackpack/init/ModItemGroups.java:21)
- [ ] `ModItemGroups.registerItemGroup(0)`: it uses Minecraft's `BuiltInRegistries`.CREATIVE_MODE_TAB (com/tiviacz/travelersbackpack/init/ModItemGroups.java:25)
- [ ] `ModItemGroups.createTabStack(0)`: it creates Minecraft's `ItemStack` (com/tiviacz/travelersbackpack/init/ModItemGroups.java:33)
- [ ] `ModItemGroups.addItemGroup(0)`: it uses ModItemGroups.TRAVELERS_BACKPACK, which isn't translated (it uses Minecraft's `ResourceKey`) (com/tiviacz/travelersbackpack/init/ModItemGroups.java:39)
- [ ] `ModItemGroups.createTieredBackpack(1)`: it creates Minecraft's `ItemStack` (com/tiviacz/travelersbackpack/init/ModItemGroups.java:128)
- [ ] `ModItemGroups.withTanks(1)`: it uses Minecraft's `Block` (com/tiviacz/travelersbackpack/init/ModItemGroups.java:135)

### `ModItems`

- [ ] `ModItems.STANDARD_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:41)
- [ ] `ModItems.NETHERITE_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:42)
- [ ] `ModItems.DIAMOND_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:43)
- [ ] `ModItems.GOLD_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:44)
- [ ] `ModItems.EMERALD_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:45)
- [ ] `ModItems.IRON_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:46)
- [ ] `ModItems.LAPIS_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:47)
- [ ] `ModItems.REDSTONE_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:48)
- [ ] `ModItems.COAL_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:49)
- [ ] `ModItems.QUARTZ_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:50)
- [ ] `ModItems.BOOKSHELF_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:51)
- [ ] `ModItems.END_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:52)
- [ ] `ModItems.NETHER_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:53)
- [ ] `ModItems.SANDSTONE_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:54)
- [ ] `ModItems.SNOW_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:55)
- [ ] `ModItems.SPONGE_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:56)
- [ ] `ModItems.CAKE_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:57)
- [ ] `ModItems.CACTUS_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:58)
- [ ] `ModItems.HAY_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:59)
- [ ] `ModItems.MELON_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:60)
- [ ] `ModItems.PUMPKIN_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:61)
- [ ] `ModItems.CREEPER_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:62)
- [ ] `ModItems.DRAGON_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:63)
- [ ] `ModItems.ENDERMAN_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:64)
- [ ] `ModItems.BLAZE_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:65)
- [ ] `ModItems.GHAST_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:66)
- [ ] `ModItems.MAGMA_CUBE_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:67)
- [ ] `ModItems.SKELETON_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:68)
- [ ] `ModItems.SPIDER_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:69)
- [ ] `ModItems.WITHER_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:70)
- [ ] `ModItems.WARDEN_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:71)
- [ ] `ModItems.BAT_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:72)
- [ ] `ModItems.BEE_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:73)
- [ ] `ModItems.WOLF_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:74)
- [ ] `ModItems.FOX_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:75)
- [ ] `ModItems.OCELOT_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:76)
- [ ] `ModItems.HORSE_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:77)
- [ ] `ModItems.COW_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:78)
- [ ] `ModItems.PIG_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:79)
- [ ] `ModItems.SHEEP_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:80)
- [ ] `ModItems.CHICKEN_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:81)
- [ ] `ModItems.SQUID_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:82)
- [ ] `ModItems.VILLAGER_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:83)
- [ ] `ModItems.IRON_GOLEM_TRAVELERS_BACKPACK`: it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/init/ModItems.java:84)
- [ ] `ModItems.WHITE_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:85)
- [ ] `ModItems.ORANGE_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:86)
- [ ] `ModItems.MAGENTA_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:87)
- [ ] `ModItems.LIGHT_BLUE_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:88)
- [ ] `ModItems.YELLOW_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:89)
- [ ] `ModItems.LIME_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:90)
- [ ] `ModItems.PINK_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:91)
- [ ] `ModItems.GRAY_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:92)
- [ ] `ModItems.LIGHT_GRAY_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:93)
- [ ] `ModItems.CYAN_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:94)
- [ ] `ModItems.PURPLE_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:95)
- [ ] `ModItems.BLUE_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:96)
- [ ] `ModItems.BROWN_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:97)
- [ ] `ModItems.GREEN_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:98)
- [ ] `ModItems.RED_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:99)
- [ ] `ModItems.BLACK_SLEEPING_BAG`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:100)
- [ ] `ModItems.BACKPACK_TANK`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:101)
- [ ] `ModItems.HOSE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:102)
- [ ] `ModItems.HOSE_NOZZLE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:103)
- [ ] `ModItems.BLANK_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:104)
- [ ] `ModItems.IRON_TIER_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:105)
- [ ] `ModItems.GOLD_TIER_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:106)
- [ ] `ModItems.DIAMOND_TIER_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:107)
- [ ] `ModItems.NETHERITE_TIER_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:108)
- [ ] `ModItems.TANKS_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:109)
- [ ] `ModItems.CRAFTING_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:110)
- [ ] `ModItems.FURNACE_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:111)
- [ ] `ModItems.SMOKER_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:112)
- [ ] `ModItems.BLAST_FURNACE_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:113)
- [ ] `ModItems.PICKUP_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:114)
- [ ] `ModItems.MAGNET_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:115)
- [ ] `ModItems.JUKEBOX_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:116)
- [ ] `ModItems.VOID_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:117)
- [ ] `ModItems.FEEDING_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:118)
- [ ] `ModItems.REFILL_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:119)
- [ ] `ModItems.LANTERN_UPGRADE`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/init/ModItems.java:120)
- [ ] `ModItems.BACKPACK_ITEM_ENTITY`: it uses Minecraft's `EntityType` (com/tiviacz/travelersbackpack/init/ModItems.java:121)
- [x] `ModItems.COMPATIBLE_OVERWORLD_BACKPACK_ENTRIES` in game_side.rs (com/tiviacz/travelersbackpack/init/ModItems.java:122)
- [x] `ModItems.COMPATIBLE_NETHER_BACKPACK_ENTRIES` in game_side.rs (com/tiviacz/travelersbackpack/init/ModItems.java:123)
- [ ] `ModItems.init(0)`: it uses ModItems.STANDARD_TRAVELERS_BACKPACK, which isn't translated (it uses TravelersBackpackItem, which isn't translated (it extends Minecraft's `BlockItem`)) (com/tiviacz/travelersbackpack/init/ModItems.java:125)
- [x] `ModItems.resourceLocation(1)` (com/tiviacz/travelersbackpack/init/ModItems.java:537)
- [ ] `ModItems.resourceKey(1)`: it uses Minecraft's `ResourceKey` (com/tiviacz/travelersbackpack/init/ModItems.java:541)
- [ ] `ModItems.registerItemFluidStorage(0)`: it uses Fabric's `FluidStorage`.ITEM (com/tiviacz/travelersbackpack/init/ModItems.java:545)

### `ModNetwork`

- [ ] `ModNetwork.initClient(0)`: it uses ClientboundSyncAttachmentPacket.TYPE, which isn't translated (it uses Minecraft's `Type`) (com/tiviacz/travelersbackpack/init/ModNetwork.java:23)
- [ ] `ModNetwork.initServer(0)`: it calls Fabric's `PayloadTypeRegistry`.clientboundPlay (com/tiviacz/travelersbackpack/init/ModNetwork.java:36)

### `ModRecipeSerializers`

- [ ] `ModRecipeSerializers.BACKPACK_SHAPED`: it uses Minecraft's `RecipeSerializer` (com/tiviacz/travelersbackpack/init/ModRecipeSerializers.java:11)
- [ ] `ModRecipeSerializers.BACKPACK_UPGRADE`: it uses Minecraft's `RecipeSerializer` (com/tiviacz/travelersbackpack/init/ModRecipeSerializers.java:12)
- [ ] `ModRecipeSerializers.init(0)`: it uses ModRecipeSerializers.BACKPACK_SHAPED, which isn't translated (it uses Minecraft's `RecipeSerializer`) (com/tiviacz/travelersbackpack/init/ModRecipeSerializers.java:14)

### `ModScreenHandlerTypes`

- [ ] `ModScreenHandlerTypes.BACKPACK_MENU`: it uses Fabric's `ExtendedMenuType` (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:16)
- [ ] `ModScreenHandlerTypes.BACKPACK_BLOCK_MENU`: it uses Fabric's `ExtendedMenuType` (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:19)
- [ ] `ModScreenHandlerTypes.BACKPACK_SETTINGS_MENU`: it uses Fabric's `ExtendedMenuType` (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:22)
- [ ] `ModScreenHandlerTypes.init(0)`: it uses Minecraft's `BuiltInRegistries`.MENU (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:26)

### `ModScreenHandlerTypes.BlockEntityScreenData`: it uses Minecraft's `BlockPos`

- [ ] `ModScreenHandlerTypes.BlockEntityScreenData.PACKET_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:33)
- [ ] `ModScreenHandlerTypes.BlockEntityScreenData.BlockEntityScreenData(2)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:32)
- [ ] `ModScreenHandlerTypes.BlockEntityScreenData.entityId(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:32)
- [ ] `ModScreenHandlerTypes.BlockEntityScreenData.pos(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:32)

### `ModScreenHandlerTypes.ItemScreenData`

- [ ] `ModScreenHandlerTypes.ItemScreenData.PACKET_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:43)
- [x] `ModScreenHandlerTypes.ItemScreenData.ItemScreenData(2)` (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:42)
- [x] `ModScreenHandlerTypes.ItemScreenData.screenID(0)` (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:42)
- [x] `ModScreenHandlerTypes.ItemScreenData.entityID(0)` (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:42)

### `ModScreenHandlerTypes.SettingsScreenData`: it uses Minecraft's `BlockPos`

- [ ] `ModScreenHandlerTypes.SettingsScreenData.PACKET_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:53)
- [ ] `ModScreenHandlerTypes.SettingsScreenData.SettingsScreenData(4)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:52)
- [ ] `ModScreenHandlerTypes.SettingsScreenData.isBlockEntity(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:52)
- [ ] `ModScreenHandlerTypes.SettingsScreenData.screenId(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:52)
- [ ] `ModScreenHandlerTypes.SettingsScreenData.pos(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:52)
- [ ] `ModScreenHandlerTypes.SettingsScreenData.index(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (com/tiviacz/travelersbackpack/init/ModScreenHandlerTypes.java:52)

### `ModTags`

- [ ] `ModTags.BLACKLISTED_ITEMS`: it uses Minecraft's `TagKey` (com/tiviacz/travelersbackpack/init/ModTags.java:9)
- [ ] `ModTags.ACCEPTABLE_TOOLS`: it uses Minecraft's `TagKey` (com/tiviacz/travelersbackpack/init/ModTags.java:12)
- [ ] `ModTags.CUSTOM_TRAVELERS_BACKPACK`: it uses Minecraft's `TagKey` (com/tiviacz/travelersbackpack/init/ModTags.java:13)
- [ ] `ModTags.SLEEPING_BAGS`: it uses Minecraft's `TagKey` (com/tiviacz/travelersbackpack/init/ModTags.java:16)
- [ ] `ModTags.BACKPACK_UPGRADES`: it uses Minecraft's `TagKey` (com/tiviacz/travelersbackpack/init/ModTags.java:17)

### `BackpackContainer`: it uses Minecraft's `ItemStack`

- [ ] `BackpackContainer.BackpackContainer(4)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackContainer.java:17)
- [ ] `BackpackContainer.saveExtraData(2)`: it's overloaded with the same number of parameters (com/tiviacz/travelersbackpack/inventory/BackpackContainer.java:18)
- [ ] `BackpackContainer.saveExtraData(2)`: it's overloaded with the same number of parameters (com/tiviacz/travelersbackpack/inventory/BackpackContainer.java:22)
- [ ] `BackpackContainer.openBackpack(3)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/BackpackContainer.java:26)
- [ ] `BackpackContainer.openBackpack(4)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/BackpackContainer.java:49)
- [ ] `BackpackContainer.openAnotherPlayerBackpack(4)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/BackpackContainer.java:76)
- [ ] `BackpackContainer.synchroniseToOpener(2)`: it uses Minecraft's `ServerPlayer` (com/tiviacz/travelersbackpack/inventory/BackpackContainer.java:100)
- [ ] `BackpackContainer.stack(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackContainer.java:17)
- [ ] `BackpackContainer.player(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackContainer.java:17)
- [ ] `BackpackContainer.screenID(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackContainer.java:17)
- [ ] `BackpackContainer.index(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackContainer.java:17)

### `BackpackSettingsContainer`: it implements Fabric's `ExtendedMenuProvider`

- [ ] `BackpackSettingsContainer.BackpackSettingsContainer(4)`: its class isn't translated (it implements Fabric's `ExtendedMenuProvider`) (com/tiviacz/travelersbackpack/inventory/BackpackSettingsContainer.java:16)
- [ ] `BackpackSettingsContainer.getScreenOpeningData(1)`: its class isn't translated (it implements Fabric's `ExtendedMenuProvider`) (com/tiviacz/travelersbackpack/inventory/BackpackSettingsContainer.java:18)
- [ ] `BackpackSettingsContainer.getDisplayName(0)`: its class isn't translated (it implements Fabric's `ExtendedMenuProvider`) (com/tiviacz/travelersbackpack/inventory/BackpackSettingsContainer.java:22)
- [ ] `BackpackSettingsContainer.shouldCloseCurrentScreen(0)`: its class isn't translated (it implements Fabric's `ExtendedMenuProvider`) (com/tiviacz/travelersbackpack/inventory/BackpackSettingsContainer.java:26)
- [ ] `BackpackSettingsContainer.createMenu(3)`: its class isn't translated (it implements Fabric's `ExtendedMenuProvider`) (com/tiviacz/travelersbackpack/inventory/BackpackSettingsContainer.java:30)
- [ ] `BackpackSettingsContainer.openSettings(4)`: it uses Minecraft's `ServerPlayer` (com/tiviacz/travelersbackpack/inventory/BackpackSettingsContainer.java:37)
- [ ] `BackpackSettingsContainer.stack(0)`: its class isn't translated (it implements Fabric's `ExtendedMenuProvider`) (com/tiviacz/travelersbackpack/inventory/BackpackSettingsContainer.java:16)
- [ ] `BackpackSettingsContainer.player(0)`: its class isn't translated (it implements Fabric's `ExtendedMenuProvider`) (com/tiviacz/travelersbackpack/inventory/BackpackSettingsContainer.java:16)
- [ ] `BackpackSettingsContainer.screenID(0)`: its class isn't translated (it implements Fabric's `ExtendedMenuProvider`) (com/tiviacz/travelersbackpack/inventory/BackpackSettingsContainer.java:16)
- [ ] `BackpackSettingsContainer.index(0)`: its class isn't translated (it implements Fabric's `ExtendedMenuProvider`) (com/tiviacz/travelersbackpack/inventory/BackpackSettingsContainer.java:16)

### `BackpackWrapper`: it uses Minecraft's `ItemStack`

- [ ] `BackpackWrapper.DUMMY`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:60)
- [x] `BackpackWrapper.STORAGE_ID` (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:77)
- [x] `BackpackWrapper.UPGRADES_ID` (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:78)
- [x] `BackpackWrapper.TOOLS_ID` (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:79)
- [ ] `BackpackWrapper.BackpackWrapper(5)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:81)
- [ ] `BackpackWrapper.BackpackWrapper(4)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:86)
- [ ] `BackpackWrapper.BackpackWrapper(5)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:90)
- [ ] `BackpackWrapper.fromStack(1)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:126)
- [ ] `BackpackWrapper.setBackpackStack(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:130)
- [ ] `BackpackWrapper.getBackpackStack(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:137)
- [ ] `BackpackWrapper.getBackpackSlotIndex(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:141)
- [ ] `BackpackWrapper.setBackpackOwner(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:145)
- [ ] `BackpackWrapper.getBackpackOwner(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:149)
- [ ] `BackpackWrapper.getPlayersUsing(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:154)
- [ ] `BackpackWrapper.addUser(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:158)
- [ ] `BackpackWrapper.setLevel(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:164)
- [ ] `BackpackWrapper.getLevel(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:168)
- [ ] `BackpackWrapper.getServerLevel(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:172)
- [ ] `BackpackWrapper.loadHandler(4)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:176)
- [ ] `BackpackWrapper.loadHandlers(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:187)
- [ ] `BackpackWrapper.loadStorage(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:201)
- [ ] `BackpackWrapper.loadUpgrades(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:205)
- [ ] `BackpackWrapper.loadTools(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:210)
- [ ] `BackpackWrapper.loadAdditionally(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:214)
- [ ] `BackpackWrapper.setStarterUpgrade(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:232)
- [ ] `BackpackWrapper.getStorageSize(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:249)
- [ ] `BackpackWrapper.getUpgradesSize(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:253)
- [ ] `BackpackWrapper.getToolSize(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:257)
- [ ] `BackpackWrapper.getStorageForInputOutput(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:261)
- [ ] `BackpackWrapper.getStorageForInputOutput(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:265)
- [ ] `BackpackWrapper.getStorage(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:269)
- [ ] `BackpackWrapper.getUpgrades(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:273)
- [ ] `BackpackWrapper.getTools(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:277)
- [ ] `BackpackWrapper.getUpgradeManager(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:281)
- [ ] `BackpackWrapper.getRegistryAccess(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:285)
- [ ] `BackpackWrapper.getUpgrade(1)`: it's generic (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:296)
- [ ] `BackpackWrapper.getUnsortableSlots(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:300)
- [ ] `BackpackWrapper.getMemorySlots(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:304)
- [ ] `BackpackWrapper.getScreenID(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:308)
- [ ] `BackpackWrapper.getBackpackScreenTitle(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:312)
- [ ] `BackpackWrapper.setUnsortableSlots(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:318)
- [ ] `BackpackWrapper.setMemorySlots(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:323)
- [ ] `BackpackWrapper.setDataAndSync(2)`: it's generic (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:328)
- [ ] `BackpackWrapper.setData(2)`: it's generic (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:333)
- [ ] `BackpackWrapper.showToolSlots(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:341)
- [ ] `BackpackWrapper.showMoreButtons(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:345)
- [ ] `BackpackWrapper.tanksVisible(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:349)
- [ ] `BackpackWrapper.getBackpackTankCapacity(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:355)
- [ ] `BackpackWrapper.setBackpackPos(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:359)
- [ ] `BackpackWrapper.getBackpackPos(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:363)
- [ ] `BackpackWrapper.setBackpackTankCapacity(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:367)
- [ ] `BackpackWrapper.getRenderInfo(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:372)
- [ ] `BackpackWrapper.setRenderInfo(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:376)
- [ ] `BackpackWrapper.updateRenderInfo(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:380)
- [ ] `BackpackWrapper.isAbilityEnabled(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:388)
- [ ] `BackpackWrapper.getSortType(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:393)
- [ ] `BackpackWrapper.setNextSortType(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:398)
- [ ] `BackpackWrapper.getSleepingBagColor(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:403)
- [ ] `BackpackWrapper.setSleepingBagColor(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:407)
- [ ] `BackpackWrapper.isOwner(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:411)
- [ ] `BackpackWrapper.isDyed(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:415)
- [ ] `BackpackWrapper.getDyeColor(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:419)
- [ ] `BackpackWrapper.getCooldown(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:423)
- [ ] `BackpackWrapper.setCooldown(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:427)
- [ ] `BackpackWrapper.decreaseCooldown(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:431)
- [ ] `BackpackWrapper.canUpgradeTick(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:438)
- [ ] `BackpackWrapper.hasTickingUpgrade(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:442)
- [ ] `BackpackWrapper.getUpgradeTickInterval(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:446)
- [ ] `BackpackWrapper.setUpgradeTickInterval(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:450)
- [ ] `BackpackWrapper.removeUpgradeTickInterval(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:454)
- [ ] `BackpackWrapper.isExtended(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:458)
- [ ] `BackpackWrapper.getSlotsInRow(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:462)
- [ ] `BackpackWrapper.getRows(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:466)
- [ ] `BackpackWrapper.sendDataToClients(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:470)
- [ ] `BackpackWrapper.getMemorizedSlot(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:521)
- [ ] `BackpackWrapper.createHandler(2)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:525)
- [ ] `BackpackWrapper.setSlotChanged(3)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:544)
- [ ] `BackpackWrapper.applyLowestTickInterval(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:572)
- [ ] `BackpackWrapper.updateMinimalTickInterval(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:587)
- [ ] `BackpackWrapper.createUpgradeHandler(2)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:621)
- [ ] `BackpackWrapper.isSizeInitialized(1)`: ItemStack.has with something other than a DataComponents constant (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:676)
- [ ] `BackpackWrapper.initializeSize(1)`: ItemStack.has with something other than a DataComponents constant (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:680)
- [ ] `BackpackWrapper.initializeSleepingBag(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:699)
- [ ] `BackpackWrapper.requestMenuAndScreenUpdate(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:707)
- [ ] `BackpackWrapper.requestMenuAndScreenUpdate(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:712)
- [ ] `BackpackWrapper.requestMenuUpdate(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:717)
- [ ] `BackpackWrapper.requestMenuUpdate(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:726)
- [ ] `BackpackWrapper.requestScreenUpdate(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:735)
- [ ] `BackpackWrapper.tickForBlockEntity(1)`: it uses BackpackBlockEntity, which isn't translated (it extends Minecraft's `BlockEntity`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:743)
- [ ] `BackpackWrapper.getBackpackWrapper(3)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:778)
- [ ] `BackpackWrapper.tick(3)`: it calls TravelersBackpack.enableIntegration, which isn't translated (com/tiviacz/travelersbackpack/inventory/BackpackWrapper.java:797)

### `CommonFluid`

- [ ] `CommonFluid.empty(0)`: it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/CommonFluid.java:7)
- [ ] `CommonFluid.getFluidName(1)`: it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/CommonFluid.java:11)

### `FluidTank`: it extends Fabric's `SingleVariantStorage`

- [ ] `FluidTank.FluidTank(1)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:18)
- [ ] `FluidTank.FluidTank(2)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:22)
- [ ] `FluidTank.setCapacity(1)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:27)
- [ ] `FluidTank.setValidator(1)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:32)
- [ ] `FluidTank.isFluidValid(1)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:40)
- [ ] `FluidTank.getBlankVariant(0)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:44)
- [ ] `FluidTank.getCapacity(1)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:48)
- [ ] `FluidTank.getCapacity(0)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:52)
- [ ] `FluidTank.getViewCapacity(0)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:56)
- [ ] `FluidTank.getFluid(0)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:60)
- [ ] `FluidTank.getFluidAmount(0)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:64)
- [ ] `FluidTank.readFromNBT(2)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:68)
- [ ] `FluidTank.writeToNBT(2)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:73)
- [ ] `FluidTank.getTanks(0)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:81)
- [ ] `FluidTank.getFluidInTank(1)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:85)
- [ ] `FluidTank.getTankCapacity(1)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:89)
- [ ] `FluidTank.isFluidValid(2)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:93)
- [ ] `FluidTank.fill(2)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:97)
- [ ] `FluidTank.fill(3)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:101)
- [ ] `FluidTank.drain(2)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:156)
- [ ] `FluidTank.drain(2)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:162)
- [ ] `FluidTank.drain(3)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:166)
- [ ] `FluidTank.insert(3)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:193)
- [ ] `FluidTank.extract(3)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:207)
- [ ] `FluidTank.onContentsChanged(0)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:221)
- [ ] `FluidTank.saveToStack(0)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:224)
- [ ] `FluidTank.setFluid(1)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:228)
- [ ] `FluidTank.isEmpty(0)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:232)
- [ ] `FluidTank.getSpace(0)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:236)
- [ ] `FluidTank.isResourceBlank(0)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:240)
- [ ] `FluidTank.getResource(0)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:244)
- [ ] `FluidTank.getAmount(0)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:248)
- [ ] `FluidTank.createSnapshot(0)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:252)
- [ ] `FluidTank.readSnapshot(1)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:256)
- [ ] `FluidTank.toString(0)`: its class isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/FluidTank.java:260)

### `FluidVariantWrapper`: it uses Fabric's `FluidVariant`

- [ ] `FluidVariantWrapper.CODEC`: it uses Minecraft's `Codec` (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:16)
- [ ] `FluidVariantWrapper.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:23)
- [ ] `FluidVariantWrapper.FluidVariantWrapper(2)`: its class isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:15)
- [ ] `FluidVariantWrapper.parse(1)`: it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:27)
- [ ] `FluidVariantWrapper.parseOptional(1)`: it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:31)
- [ ] `FluidVariantWrapper.save(0)`: its class isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:35)
- [ ] `FluidVariantWrapper.saveOptional(0)`: its class isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:40)
- [ ] `FluidVariantWrapper.isEmpty(0)`: its class isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:44)
- [ ] `FluidVariantWrapper.getAmount(0)`: its class isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:48)
- [ ] `FluidVariantWrapper.getViewAmount(0)`: its class isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:52)
- [ ] `FluidVariantWrapper.blank(0)`: it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:56)
- [ ] `FluidVariantWrapper.copyWithAmount(1)`: its class isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:60)
- [ ] `FluidVariantWrapper.copy(0)`: its class isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:74)
- [ ] `FluidVariantWrapper.setAmount(1)`: its class isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:78)
- [ ] `FluidVariantWrapper.grow(1)`: its class isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:82)
- [ ] `FluidVariantWrapper.shrink(1)`: its class isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:86)
- [ ] `FluidVariantWrapper.fluidVariant(0)`: its class isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:15)
- [ ] `FluidVariantWrapper.amount(0)`: its class isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/inventory/FluidVariantWrapper.java:15)

### `IItemHandlerModifiable`: it's an interface

- [ ] `IItemHandlerModifiable.getSlots(0)`: it has no body (com/tiviacz/travelersbackpack/inventory/handler/IItemHandlerModifiable.java:6)
- [ ] `IItemHandlerModifiable.getStackInSlot(1)`: it has no body (com/tiviacz/travelersbackpack/inventory/handler/IItemHandlerModifiable.java:8)
- [ ] `IItemHandlerModifiable.insertItem(3)`: it has no body (com/tiviacz/travelersbackpack/inventory/handler/IItemHandlerModifiable.java:10)
- [ ] `IItemHandlerModifiable.extractItem(3)`: it has no body (com/tiviacz/travelersbackpack/inventory/handler/IItemHandlerModifiable.java:12)
- [ ] `IItemHandlerModifiable.getSlotLimit(1)`: it has no body (com/tiviacz/travelersbackpack/inventory/handler/IItemHandlerModifiable.java:14)
- [ ] `IItemHandlerModifiable.isItemValid(2)`: it has no body (com/tiviacz/travelersbackpack/inventory/handler/IItemHandlerModifiable.java:16)
- [ ] `IItemHandlerModifiable.setStackInSlot(2)`: it has no body (com/tiviacz/travelersbackpack/inventory/handler/IItemHandlerModifiable.java:18)

### `ItemStackHandler`: it extends Minecraft's `SimpleContainer`

- [ ] `ItemStackHandler.ItemStackHandler(0)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:16)
- [ ] `ItemStackHandler.ItemStackHandler(1)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:20)
- [ ] `ItemStackHandler.ItemStackHandler(1)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:25)
- [ ] `ItemStackHandler.setSize(1)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:30)
- [ ] `ItemStackHandler.setStackInSlot(2)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:34)
- [ ] `ItemStackHandler.getSlots(0)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:41)
- [ ] `ItemStackHandler.getStackInSlot(1)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:46)
- [ ] `ItemStackHandler.insertItem(3)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:52)
- [ ] `ItemStackHandler.extractItem(3)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:91)
- [ ] `ItemStackHandler.getSlotLimit(1)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:122)
- [ ] `ItemStackHandler.getStackLimit(2)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:127)
- [ ] `ItemStackHandler.isItemValid(2)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:131)
- [ ] `ItemStackHandler.serialize(1)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:136)
- [ ] `ItemStackHandler.deserialize(1)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:149)
- [ ] `ItemStackHandler.validateSlotIndex(1)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:158)
- [ ] `ItemStackHandler.onLoad(0)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:164)
- [ ] `ItemStackHandler.onContentsChanged(1)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:167)
- [ ] `ItemStackHandler.getContainerSize(0)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:170)
- [ ] `ItemStackHandler.isEmpty(0)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:174)
- [ ] `ItemStackHandler.getItem(1)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:178)
- [ ] `ItemStackHandler.removeItem(2)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:182)
- [ ] `ItemStackHandler.removeItemNoUpdate(1)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:186)
- [ ] `ItemStackHandler.setItem(2)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:190)
- [ ] `ItemStackHandler.setChanged(0)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:194)
- [ ] `ItemStackHandler.stillValid(1)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:197)
- [ ] `ItemStackHandler.clearContent(0)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:201)
- [ ] `ItemStackHandler.canPlaceItem(2)`: its class isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/handler/ItemStackHandler.java:205)

### `StorageAccessWrapper`: it extends `ItemStackHandler`

- [ ] `StorageAccessWrapper.StorageAccessWrapper(3)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:18)
- [ ] `StorageAccessWrapper.StorageAccessWrapper(2)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:23)
- [ ] `StorageAccessWrapper.setStackInSlot(2)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:28)
- [ ] `StorageAccessWrapper.getSlots(0)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:33)
- [ ] `StorageAccessWrapper.getStackInSlot(1)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:38)
- [ ] `StorageAccessWrapper.insertItem(3)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:44)
- [ ] `StorageAccessWrapper.matchesStack(2)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:68)
- [ ] `StorageAccessWrapper.tryVoiding(1)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:76)
- [ ] `StorageAccessWrapper.extractItem(3)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:80)
- [ ] `StorageAccessWrapper.getSlotLimit(1)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:86)
- [ ] `StorageAccessWrapper.isItemValid(2)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:91)
- [ ] `StorageAccessWrapper.canTakeItem(3)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:96)
- [ ] `StorageAccessWrapper.canPlaceItem(2)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:100)
- [ ] `StorageAccessWrapper.getContainerSize(0)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:121)
- [ ] `StorageAccessWrapper.isEmpty(0)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:126)
- [ ] `StorageAccessWrapper.getItem(1)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:131)
- [ ] `StorageAccessWrapper.removeItem(2)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:136)
- [ ] `StorageAccessWrapper.removeItemNoUpdate(1)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:141)
- [ ] `StorageAccessWrapper.setItem(2)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:146)
- [ ] `StorageAccessWrapper.setChanged(0)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:151)
- [ ] `StorageAccessWrapper.stillValid(1)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:155)
- [ ] `StorageAccessWrapper.clearContent(0)`: its class isn't translated (it extends `ItemStackHandler`) (com/tiviacz/travelersbackpack/inventory/handler/StorageAccessWrapper.java:160)

### `InventoryActions`

- [ ] `InventoryActions.transferContainerTank(4)`: it uses TanksUpgrade, which isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/InventoryActions.java:32)
- [ ] `InventoryActions.transferFluid(6)`: its body couldn't be read: a labelled statement (com/tiviacz/travelersbackpack/inventory/InventoryActions.java:110)
- [ ] `InventoryActions.transferPotion(8)`: it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/InventoryActions.java:183)
- [ ] `InventoryActions.isSameFluid(2)`: it uses Fabric's `Storage` (com/tiviacz/travelersbackpack/inventory/InventoryActions.java:217)
- [ ] `InventoryActions.playFluidSound(4)`: it uses Minecraft's `SoundEvent` (com/tiviacz/travelersbackpack/inventory/InventoryActions.java:230)

### `AbstractBackpackMenu`: it extends Minecraft's `AbstractContainerMenu`

- [ ] `AbstractBackpackMenu.AbstractBackpackMenu(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/tiviacz/travelersbackpack/inventory/menu/AbstractBackpackMenu.java:21)
- [ ] `AbstractBackpackMenu.getWrapper(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/tiviacz/travelersbackpack/inventory/menu/AbstractBackpackMenu.java:28)
- [ ] `AbstractBackpackMenu.getPlayerInventory(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/tiviacz/travelersbackpack/inventory/menu/AbstractBackpackMenu.java:32)
- [ ] `AbstractBackpackMenu.addBackpackStorageSlots(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerMenu`) (com/tiviacz/travelersbackpack/inventory/menu/AbstractBackpackMenu.java:36)

### `BackpackBaseMenu`: it extends `AbstractBackpackMenu`

- [ ] `BackpackBaseMenu.BackpackBaseMenu(4)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:65)
- [ ] `BackpackBaseMenu.addSlots(0)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:70)
- [ ] `BackpackBaseMenu.rebuildSlots(0)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:91)
- [ ] `BackpackBaseMenu.addModifiableSlots(0)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:99)
- [ ] `BackpackBaseMenu.rebuildModifiableSlots(0)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:114)
- [ ] `BackpackBaseMenu.updateModifiableSlotsPosition(1)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:131)
- [ ] `BackpackBaseMenu.updateSlotsPosition(0)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:140)
- [ ] `BackpackBaseMenu.updateBackpackUpgradeSlots(0)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:184)
- [ ] `BackpackBaseMenu.updateUpgradeSlotsPosition(1)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:225)
- [ ] `BackpackBaseMenu.addSlot(1)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:250)
- [ ] `BackpackBaseMenu.addBackpackToolSlots(1)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:258)
- [ ] `BackpackBaseMenu.addBackpackUpgradeSlots(1)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:264)
- [ ] `BackpackBaseMenu.addPlayerInventoryAndHotbar(2)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:306)
- [ ] `BackpackBaseMenu.addUpgradeListeners(0)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:323)
- [ ] `BackpackBaseMenu.addUpgradeSlots(1)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:329)
- [ ] `BackpackBaseMenu.doClick(4)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:354)
- [ ] `BackpackBaseMenu.canCraft(2)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:364)
- [ ] `BackpackBaseMenu.slotsChanged(1)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:371)
- [ ] `BackpackBaseMenu.sendAllDataToRemote(0)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:376)
- [ ] `BackpackBaseMenu.canTakeItemForPickAll(2)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:381)
- [ ] `BackpackBaseMenu.quickMoveStack(2)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:389)
- [ ] `BackpackBaseMenu.checkMemorySlots(1)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:454)
- [ ] `BackpackBaseMenu.moveItemStackTo(4)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:474)
- [ ] `BackpackBaseMenu.handleShiftCraft(3)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:554)
- [ ] `BackpackBaseMenu.resetStackedContents(1)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:596)
- [ ] `BackpackBaseMenu.slotChangedCraftingGrid(3)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:607)
- [ ] `BackpackBaseMenu.removed(1)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:646)
- [ ] `BackpackBaseMenu.clearSlotsAndPlaySound(4)`: it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:658)
- [ ] `BackpackBaseMenu.clearSlot(4)`: it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:673)
- [ ] `BackpackBaseMenu.playSound(1)`: it calls Minecraft's LocalPlayer.level (no Horizon helper for it yet) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:694)
- [ ] `BackpackBaseMenu.stillValid(1)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBaseMenu.java:706)

### `BackpackBlockEntityMenu`: it extends `BackpackBaseMenu`

- [ ] `BackpackBlockEntityMenu.BackpackBlockEntityMenu(3)`: its class isn't translated (it extends `BackpackBaseMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBlockEntityMenu.java:19)
- [ ] `BackpackBlockEntityMenu.BackpackBlockEntityMenu(4)`: its class isn't translated (it extends `BackpackBaseMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBlockEntityMenu.java:23)
- [ ] `BackpackBlockEntityMenu.isInfiniteAccess(1)`: it uses BlockEntityScreenData, which isn't translated (it uses Minecraft's `BlockPos`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBlockEntityMenu.java:29)
- [ ] `BackpackBlockEntityMenu.getBlockEntity(2)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBlockEntityMenu.java:33)
- [ ] `BackpackBlockEntityMenu.stillValid(1)`: its class isn't translated (it extends `BackpackBaseMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBlockEntityMenu.java:47)
- [ ] `BackpackBlockEntityMenu.removed(1)`: its class isn't translated (it extends `BackpackBaseMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackBlockEntityMenu.java:57)

### `BackpackItemMenu`: it extends `BackpackBaseMenu`

- [ ] `BackpackItemMenu.BackpackItemMenu(3)`: its class isn't translated (it extends `BackpackBaseMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackItemMenu.java:17)
- [ ] `BackpackItemMenu.BackpackItemMenu(3)`: its class isn't translated (it extends `BackpackBaseMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackItemMenu.java:21)
- [ ] `BackpackItemMenu.createWrapper(2)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackItemMenu.java:26)
- [ ] `BackpackItemMenu.clicked(4)`: its class isn't translated (it extends `BackpackBaseMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackItemMenu.java:47)
- [ ] `BackpackItemMenu.addPlayerInventoryAndHotbar(2)`: its class isn't translated (it extends `BackpackBaseMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackItemMenu.java:59)
- [ ] `BackpackItemMenu.removed(1)`: its class isn't translated (it extends `BackpackBaseMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackItemMenu.java:87)
- [ ] `BackpackItemMenu.stillValid(1)`: its class isn't translated (it extends `BackpackBaseMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackItemMenu.java:93)

### `BackpackSettingsMenu`: it extends `AbstractBackpackMenu`

- [ ] `BackpackSettingsMenu.BackpackSettingsMenu(3)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackSettingsMenu.java:26)
- [ ] `BackpackSettingsMenu.BackpackSettingsMenu(3)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackSettingsMenu.java:30)
- [ ] `BackpackSettingsMenu.BackpackSettingsMenu(4)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackSettingsMenu.java:41)
- [ ] `BackpackSettingsMenu.updateSlots(0)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackSettingsMenu.java:46)
- [ ] `BackpackSettingsMenu.addSlots(0)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackSettingsMenu.java:53)
- [ ] `BackpackSettingsMenu.addPlayerInventoryAndHotbar(2)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackSettingsMenu.java:61)
- [ ] `BackpackSettingsMenu.clicked(4)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackSettingsMenu.java:100)
- [ ] `BackpackSettingsMenu.canTakeItemForPickAll(2)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackSettingsMenu.java:103)
- [ ] `BackpackSettingsMenu.quickMoveStack(2)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackSettingsMenu.java:107)
- [ ] `BackpackSettingsMenu.removed(1)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackSettingsMenu.java:111)
- [ ] `BackpackSettingsMenu.stillValid(1)`: its class isn't translated (it extends `AbstractBackpackMenu`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackSettingsMenu.java:126)
- [ ] `BackpackSettingsMenu.createWrapper(2)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackSettingsMenu.java:137)
- [ ] `BackpackSettingsMenu.getWrapper(2)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackSettingsMenu.java:144)
- [ ] `BackpackSettingsMenu.getBlockEntity(2)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/menu/BackpackSettingsMenu.java:158)

### `BackpackSlotItemHandler`: it extends `SlotItemHandler`

- [x] `BackpackSlotItemHandler.BLACKLISTED_ITEMS` in game_side.rs (com/tiviacz/travelersbackpack/inventory/menu/slot/BackpackSlotItemHandler.java:14)
- [ ] `BackpackSlotItemHandler.BackpackSlotItemHandler(4)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/BackpackSlotItemHandler.java:16)
- [ ] `BackpackSlotItemHandler.isItemValid(1)`: List.contains on objects (com/tiviacz/travelersbackpack/inventory/menu/slot/BackpackSlotItemHandler.java:20)
- [ ] `BackpackSlotItemHandler.onTake(2)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/BackpackSlotItemHandler.java:31)
- [ ] `BackpackSlotItemHandler.mayPlace(1)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/BackpackSlotItemHandler.java:36)
- [ ] `BackpackSlotItemHandler.mayPickup(1)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/BackpackSlotItemHandler.java:41)

### `CraftingSlot`: it extends Minecraft's `Slot`

- [ ] `CraftingSlot.CraftingSlot(4)`: its class isn't translated (it extends Minecraft's `Slot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/CraftingSlot.java:7)

### `DisabledSlot`: it extends Minecraft's `Slot`

- [ ] `DisabledSlot.DisabledSlot(4)`: its class isn't translated (it extends Minecraft's `Slot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/DisabledSlot.java:8)
- [ ] `DisabledSlot.mayPickup(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/DisabledSlot.java:12)

### `FilterSlotItemHandler`: it extends `SlotItemHandler`

- [ ] `FilterSlotItemHandler.FilterSlotItemHandler(6)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/FilterSlotItemHandler.java:13)
- [ ] `FilterSlotItemHandler.isActive(0)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/FilterSlotItemHandler.java:19)
- [ ] `FilterSlotItemHandler.mayPlace(1)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/FilterSlotItemHandler.java:23)
- [ ] `FilterSlotItemHandler.isFake(0)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/FilterSlotItemHandler.java:28)
- [ ] `FilterSlotItemHandler.mayPickup(1)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/FilterSlotItemHandler.java:32)
- [ ] `FilterSlotItemHandler.tryRemove(3)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/FilterSlotItemHandler.java:37)
- [ ] `FilterSlotItemHandler.safeInsert(2)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/FilterSlotItemHandler.java:46)

### `FluidSlotItemHandler`: it extends `UpgradeSlotItemHandler`

- [ ] `FluidSlotItemHandler.FluidSlotItemHandler(7)`: its class isn't translated (it extends `UpgradeSlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/FluidSlotItemHandler.java:16)
- [ ] `FluidSlotItemHandler.setByPlayer(2)`: its class isn't translated (it extends `UpgradeSlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/FluidSlotItemHandler.java:24)
- [ ] `FluidSlotItemHandler.mayPickup(1)`: its class isn't translated (it extends `UpgradeSlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/FluidSlotItemHandler.java:32)

### `ResultSlotExt`: it extends Minecraft's `ResultSlot`

- [ ] `ResultSlotExt.ResultSlotExt(7)`: its class isn't translated (it extends Minecraft's `ResultSlot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/ResultSlotExt.java:21)
- [ ] `ResultSlotExt.mayPickup(1)`: its class isn't translated (it extends Minecraft's `ResultSlot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/ResultSlotExt.java:29)
- [ ] `ResultSlotExt.isActive(0)`: its class isn't translated (it extends Minecraft's `ResultSlot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/ResultSlotExt.java:33)
- [ ] `ResultSlotExt.remove(1)`: its class isn't translated (it extends Minecraft's `ResultSlot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/ResultSlotExt.java:37)
- [ ] `ResultSlotExt.onSwapCraft(1)`: its class isn't translated (it extends Minecraft's `ResultSlot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/ResultSlotExt.java:45)
- [ ] `ResultSlotExt.set(1)`: its class isn't translated (it extends Minecraft's `ResultSlot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/ResultSlotExt.java:50)
- [ ] `ResultSlotExt.checkTakeAchievements(1)`: its class isn't translated (it extends Minecraft's `ResultSlot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/ResultSlotExt.java:53)
- [ ] `ResultSlotExt.onTake(2)`: its class isn't translated (it extends Minecraft's `ResultSlot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/ResultSlotExt.java:68)
- [ ] `ResultSlotExt.safeTake(3)`: its class isn't translated (it extends Minecraft's `ResultSlot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/ResultSlotExt.java:103)
- [ ] `ResultSlotExt.getItem(0)`: its class isn't translated (it extends Minecraft's `ResultSlot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/ResultSlotExt.java:107)

### `SlotItemHandler`: it extends Minecraft's `Slot`

- [ ] `SlotItemHandler.emptyInventory`: it uses Minecraft's `Container` (com/tiviacz/travelersbackpack/inventory/menu/slot/SlotItemHandler.java:11)
- [ ] `SlotItemHandler.SlotItemHandler(4)`: its class isn't translated (it extends Minecraft's `Slot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/SlotItemHandler.java:15)
- [ ] `SlotItemHandler.mayPlace(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/SlotItemHandler.java:21)
- [ ] `SlotItemHandler.getItem(0)`: its class isn't translated (it extends Minecraft's `Slot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/SlotItemHandler.java:25)
- [ ] `SlotItemHandler.set(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/SlotItemHandler.java:29)
- [ ] `SlotItemHandler.initialize(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/SlotItemHandler.java:34)
- [ ] `SlotItemHandler.onQuickCraft(2)`: its class isn't translated (it extends Minecraft's `Slot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/SlotItemHandler.java:39)
- [ ] `SlotItemHandler.getMaxStackSize(0)`: its class isn't translated (it extends Minecraft's `Slot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/SlotItemHandler.java:42)
- [ ] `SlotItemHandler.getMaxStackSize(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/SlotItemHandler.java:46)
- [ ] `SlotItemHandler.mayPickup(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/SlotItemHandler.java:65)
- [ ] `SlotItemHandler.remove(1)`: its class isn't translated (it extends Minecraft's `Slot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/SlotItemHandler.java:69)
- [ ] `SlotItemHandler.getItemHandler(0)`: its class isn't translated (it extends Minecraft's `Slot`) (com/tiviacz/travelersbackpack/inventory/menu/slot/SlotItemHandler.java:73)

### `ToolSlotItemHandler`: it extends `SlotItemHandler`

- [x] `ToolSlotItemHandler.TOOL_SLOTS_ACCEPTABLE_ITEMS` in game_side.rs (com/tiviacz/travelersbackpack/inventory/menu/slot/ToolSlotItemHandler.java:26)
- [ ] `ToolSlotItemHandler.ToolSlotItemHandler(4)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/ToolSlotItemHandler.java:28)
- [ ] `ToolSlotItemHandler.isActive(0)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/ToolSlotItemHandler.java:33)
- [ ] `ToolSlotItemHandler.mayPlace(1)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/ToolSlotItemHandler.java:37)
- [ ] `ToolSlotItemHandler.mayPickup(1)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/ToolSlotItemHandler.java:42)
- [ ] `ToolSlotItemHandler.isValid(1)`: `instanceof HoseItem` on an item (no single item id for it) (com/tiviacz/travelersbackpack/inventory/menu/slot/ToolSlotItemHandler.java:47)
- [ ] `ToolSlotItemHandler.setChanged(0)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/ToolSlotItemHandler.java:91)

### `UpgradeLockableSlotItemHandler`: it extends `SlotItemHandler`

- [ ] `UpgradeLockableSlotItemHandler.UpgradeLockableSlotItemHandler(5)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/UpgradeLockableSlotItemHandler.java:18)
- [ ] `UpgradeLockableSlotItemHandler.setLocked(1)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/UpgradeLockableSlotItemHandler.java:26)
- [ ] `UpgradeLockableSlotItemHandler.setHidden(1)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/UpgradeLockableSlotItemHandler.java:30)
- [ ] `UpgradeLockableSlotItemHandler.mayPlace(1)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/UpgradeLockableSlotItemHandler.java:34)
- [ ] `UpgradeLockableSlotItemHandler.mayPickup(1)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/UpgradeLockableSlotItemHandler.java:45)
- [ ] `UpgradeLockableSlotItemHandler.isActive(0)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/UpgradeLockableSlotItemHandler.java:50)
- [ ] `UpgradeLockableSlotItemHandler.setByPlayer(2)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/UpgradeLockableSlotItemHandler.java:54)
- [ ] `UpgradeLockableSlotItemHandler.setChanged(0)`: its class isn't translated (it extends `SlotItemHandler`) (com/tiviacz/travelersbackpack/inventory/menu/slot/UpgradeLockableSlotItemHandler.java:62)

### `UpgradeSlotItemHandler`: it's generic

- [ ] `UpgradeSlotItemHandler.UpgradeSlotItemHandler(5)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/menu/slot/UpgradeSlotItemHandler.java:11)
- [ ] `UpgradeSlotItemHandler.getUpgradeParent(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/menu/slot/UpgradeSlotItemHandler.java:16)
- [ ] `UpgradeSlotItemHandler.shiftClickToBackpack(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/menu/slot/UpgradeSlotItemHandler.java:20)

### `ContainerSorter`

- [x] `ContainerSorter.SORT_BACKPACK` (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:15)
- [x] `ContainerSorter.QUICK_STACK` (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:16)
- [x] `ContainerSorter.TRANSFER_TO_BACKPACK` (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:17)
- [x] `ContainerSorter.TRANSFER_TO_PLAYER` (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:18)
- [ ] `ContainerSorter.selectSort(4)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:20)
- [ ] `ContainerSorter.sortBackpack(4)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:32)
- [ ] `ContainerSorter.quickStackToBackpackNoSort(3)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:62)
- [ ] `ContainerSorter.transferToBackpackNoSort(3)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:95)
- [ ] `ContainerSorter.transferToPlayer(2)`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:152)
- [ ] `ContainerSorter.addStackWithMerge(2)`: it calls Minecraft's ItemStack.isStackable (no Horizon helper for it yet) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:177)
- [ ] `ContainerSorter.combineStacks(2)`: it calls Minecraft's ItemStack.getMaxStackSize (no Horizon helper for it yet) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:195)
- [ ] `ContainerSorter.canMergeItems(2)`: it calls Minecraft's ItemStack.isStackable (no Horizon helper for it yet) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:206)

### `ContainerSorter.CustomWrapper`: it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)

- [ ] `ContainerSorter.CustomWrapper.CustomWrapper(2)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:225)
- [ ] `ContainerSorter.CustomWrapper.CustomWrapper(3)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:229)
- [ ] `ContainerSorter.CustomWrapper.setStackInSlot(2)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:235)
- [ ] `ContainerSorter.CustomWrapper.getSlots(0)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:240)
- [ ] `ContainerSorter.CustomWrapper.getStackInSlot(1)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:245)
- [ ] `ContainerSorter.CustomWrapper.insertItem(3)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:251)
- [ ] `ContainerSorter.CustomWrapper.extractItem(3)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:270)
- [ ] `ContainerSorter.CustomWrapper.getSlotLimit(1)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:276)
- [ ] `ContainerSorter.CustomWrapper.isItemValid(2)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/ContainerSorter.java:281)

### `InvWrapper`: it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)

- [ ] `InvWrapper.InvWrapper(1)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/InvWrapper.java:10)
- [ ] `InvWrapper.equals(1)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/InvWrapper.java:14)
- [ ] `InvWrapper.hashCode(0)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/InvWrapper.java:26)
- [ ] `InvWrapper.getSlots(0)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/InvWrapper.java:31)
- [ ] `InvWrapper.getStackInSlot(1)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/InvWrapper.java:36)
- [ ] `InvWrapper.insertItem(3)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/InvWrapper.java:41)
- [ ] `InvWrapper.extractItem(3)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/InvWrapper.java:111)
- [ ] `InvWrapper.setStackInSlot(2)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/InvWrapper.java:138)
- [ ] `InvWrapper.getSlotLimit(1)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/InvWrapper.java:143)
- [ ] `InvWrapper.isItemValid(2)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/InvWrapper.java:148)
- [ ] `InvWrapper.getInv(0)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/sorter/InvWrapper.java:153)

### `SortSelector`

- [ ] `SortSelector.getSortTypeComparator(2)`: it uses `Comparator` (java.util) (com/tiviacz/travelersbackpack/inventory/sorter/SortSelector.java:20)
- [ ] `SortSelector.getCountTypeComparator(1)`: it uses `Comparator` (java.util) (com/tiviacz/travelersbackpack/inventory/sorter/SortSelector.java:28)
- [x] `SortSelector.calculateCount(1)` in game_side.rs (com/tiviacz/travelersbackpack/inventory/sorter/SortSelector.java:54)
- [ ] `SortSelector.getStringForCategorySort(1)`: it uses Minecraft's `BuiltInRegistries`.ITEM (com/tiviacz/travelersbackpack/inventory/sorter/SortSelector.java:68)
- [ ] `SortSelector.getStringForNameSort(1)`: it uses Minecraft's `BuiltInRegistries`.ITEM (com/tiviacz/travelersbackpack/inventory/sorter/SortSelector.java:73)
- [ ] `SortSelector.specialCases(2)`: it calls SortSelector.stackSize, which isn't translated (com/tiviacz/travelersbackpack/inventory/sorter/SortSelector.java:79)
- [ ] `SortSelector.stackSize(2)`: String.format with `%04d` (com/tiviacz/travelersbackpack/inventory/sorter/SortSelector.java:92)
- [ ] `SortSelector.enchantedBookNameCase(3)`: it uses Minecraft's `DataComponentType` (com/tiviacz/travelersbackpack/inventory/sorter/SortSelector.java:97)
- [x] `SortSelector.toolDurabilityCase(2)` in game_side.rs (com/tiviacz/travelersbackpack/inventory/sorter/SortSelector.java:115)

### `SortSelector.SortType`

- [ ] `SortSelector.SortType.next(0)`: it calls SortType.values, which isn't in the mod's source (com/tiviacz/travelersbackpack/inventory/sorter/SortSelector.java:124)

### `Tiers`

- [x] `Tiers.LEATHER` (com/tiviacz/travelersbackpack/inventory/Tiers.java:10)
- [x] `Tiers.IRON` (com/tiviacz/travelersbackpack/inventory/Tiers.java:11)
- [x] `Tiers.GOLD` (com/tiviacz/travelersbackpack/inventory/Tiers.java:12)
- [x] `Tiers.DIAMOND` (com/tiviacz/travelersbackpack/inventory/Tiers.java:13)
- [x] `Tiers.NETHERITE` (com/tiviacz/travelersbackpack/inventory/Tiers.java:14)
- [ ] `Tiers.of(1)`: it's overloaded with the same number of parameters (com/tiviacz/travelersbackpack/inventory/Tiers.java:16)
- [ ] `Tiers.of(1)`: it's overloaded with the same number of parameters (com/tiviacz/travelersbackpack/inventory/Tiers.java:27)

### `Tiers.Tier`

- [x] `Tiers.Tier.Tier(5)` (com/tiviacz/travelersbackpack/inventory/Tiers.java:45)
- [x] `Tiers.Tier.getName(0)` (com/tiviacz/travelersbackpack/inventory/Tiers.java:53)
- [ ] `Tiers.Tier.getLocalizedName(0)`: it uses Minecraft's `Component` (com/tiviacz/travelersbackpack/inventory/Tiers.java:57)
- [ ] `Tiers.Tier.getStorageSlots(0)`: it uses TravelersBackpackConfig.serverSpec, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)) (com/tiviacz/travelersbackpack/inventory/Tiers.java:61)
- [ ] `Tiers.Tier.getUpgradeSlots(0)`: it uses TravelersBackpackConfig.serverSpec, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)) (com/tiviacz/travelersbackpack/inventory/Tiers.java:77)
- [ ] `Tiers.Tier.getToolSlots(0)`: it uses TravelersBackpackConfig.serverSpec, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)) (com/tiviacz/travelersbackpack/inventory/Tiers.java:93)
- [ ] `Tiers.Tier.getTankCapacityPerRow(0)`: it uses TravelersBackpackConfig.serverSpec, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)) (com/tiviacz/travelersbackpack/inventory/Tiers.java:109)
- [ ] `Tiers.Tier.getNextTier(0)`: `this` used as a value (com/tiviacz/travelersbackpack/inventory/Tiers.java:127)
- [ ] `Tiers.Tier.getOrdinal(0)`: `this` used as a value (com/tiviacz/travelersbackpack/inventory/Tiers.java:139)
- [ ] `Tiers.Tier.getTierUpgradeIngredient(0)`: it uses Minecraft's `Item` (com/tiviacz/travelersbackpack/inventory/Tiers.java:153)

### `UpgradeManager`: it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)

- [ ] `UpgradeManager.UpgradeManager(1)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/inventory/UpgradeManager.java:23)
- [ ] `UpgradeManager.getWrapper(0)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/inventory/UpgradeManager.java:32)
- [ ] `UpgradeManager.getUpgradesHandler(0)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/inventory/UpgradeManager.java:36)
- [ ] `UpgradeManager.hasUpgradeInSlot(1)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/inventory/UpgradeManager.java:40)
- [ ] `UpgradeManager.getUpgrade(1)`: it's generic (com/tiviacz/travelersbackpack/inventory/UpgradeManager.java:44)
- [ ] `UpgradeManager.canAddUpgrade(1)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/inventory/UpgradeManager.java:48)
- [ ] `UpgradeManager.invalidateUpgrade(1)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/inventory/UpgradeManager.java:52)
- [ ] `UpgradeManager.initializeUpgrades(0)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/inventory/UpgradeManager.java:67)
- [ ] `UpgradeManager.detectedChange(2)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/inventory/UpgradeManager.java:73)
- [ ] `UpgradeManager.applyUpgrade(1)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/inventory/UpgradeManager.java:110)
- [ ] `UpgradeManager.getTabStatus(1)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/inventory/UpgradeManager.java:124)
- [ ] `UpgradeManager.isTagSelector(2)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/inventory/UpgradeManager.java:128)
- [ ] `UpgradeManager.isTagSelector(1)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/inventory/UpgradeManager.java:132)
- [ ] `UpgradeManager.hasTickingUpgrade(0)`: its class isn't translated (it uses BackpackWrapper, which isn't translated (it uses Minecraft's `ItemStack`)) (com/tiviacz/travelersbackpack/inventory/UpgradeManager.java:136)

### `CraftingContainerImproved`: it implements Minecraft's `CraftingContainer`

- [ ] `CraftingContainerImproved.CraftingContainerImproved(2)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:17)
- [ ] `CraftingContainerImproved.getContainerSize(0)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:22)
- [ ] `CraftingContainerImproved.getStackList(0)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:26)
- [ ] `CraftingContainerImproved.isEmpty(0)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:36)
- [ ] `CraftingContainerImproved.getItem(1)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:46)
- [ ] `CraftingContainerImproved.removeItemNoUpdate(1)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:50)
- [ ] `CraftingContainerImproved.removeItemShiftClick(2)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:54)
- [ ] `CraftingContainerImproved.removeItem(2)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:63)
- [ ] `CraftingContainerImproved.setItem(2)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:72)
- [ ] `CraftingContainerImproved.setChanged(0)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:79)
- [ ] `CraftingContainerImproved.stillValid(1)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:82)
- [ ] `CraftingContainerImproved.clearContent(0)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:86)
- [ ] `CraftingContainerImproved.getHeight(0)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:92)
- [ ] `CraftingContainerImproved.getWidth(0)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:96)
- [ ] `CraftingContainerImproved.getItems(0)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:100)
- [ ] `CraftingContainerImproved.fillStackedContents(1)`: its class isn't translated (it implements Minecraft's `CraftingContainer`) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingContainerImproved.java:104)

### `CraftingUpgrade`: it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)

- [ ] `CraftingUpgrade.CraftingUpgrade(3)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingUpgrade.java:36)
- [ ] `CraftingUpgrade.createWidget(3)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingUpgrade.java:41)
- [ ] `CraftingUpgrade.initializeContainers(2)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingUpgrade.java:47)
- [ ] `CraftingUpgrade.onUpgradeRemoved(2)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingUpgrade.java:53)
- [ ] `CraftingUpgrade.getUpgradeSlotsPosition(2)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingUpgrade.java:65)
- [ ] `CraftingUpgrade.getUpgradeSlots(4)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingUpgrade.java:79)
- [ ] `CraftingUpgrade.setSlotChanged(3)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingUpgrade.java:104)
- [ ] `CraftingUpgrade.createHandler(1)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingUpgrade.java:112)

### `CraftingWidget`: it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)

- [ ] `CraftingWidget.craftingTweaksAddition`: it uses ICraftingTweaks, which isn't translated (it's an interface) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingWidget.java:17)
- [ ] `CraftingWidget.CraftingWidget(3)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingWidget.java:19)
- [ ] `CraftingWidget.renderBg(5)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingWidget.java:28)
- [ ] `CraftingWidget.renderTooltip(3)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingWidget.java:68)
- [ ] `CraftingWidget.mouseClicked(2)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingWidget.java:74)
- [ ] `CraftingWidget.setCraftingTweaksAddition(1)`: it uses ICraftingTweaks, which isn't translated (it's an interface) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingWidget.java:91)
- [ ] `CraftingWidget.getCraftingTweaksAddition(0)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingWidget.java:95)
- [ ] `CraftingWidget.isCraftingTweaksAdditionEnabled(0)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/crafting/CraftingWidget.java:99)

### `FeedingFilterSettings`: it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)

- [x] `FeedingFilterSettings.ALLOW_MODE` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:22)
- [x] `FeedingFilterSettings.HUNGER_MODE` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:23)
- [x] `FeedingFilterSettings.IGNORE_EFFECT_MODE` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:24)
- [x] `FeedingFilterSettings.ALLOW` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:25)
- [x] `FeedingFilterSettings.BLOCK` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:26)
- [x] `FeedingFilterSettings.ALWAYS_EAT` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:27)
- [x] `FeedingFilterSettings.HALF_NUTRITION` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:28)
- [x] `FeedingFilterSettings.FULL_NUTRITION` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:29)
- [x] `FeedingFilterSettings.BLOCK_BAD_EFFECTS` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:30)
- [x] `FeedingFilterSettings.ALLOW_BAD_EFFECTS` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:31)
- [ ] `FeedingFilterSettings.FeedingFilterSettings(3)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:33)
- [ ] `FeedingFilterSettings.matchesFilter(2)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:37)
- [ ] `FeedingFilterSettings.compareHungerLevel(2)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:53)
- [ ] `FeedingFilterSettings.checkHarmfulEffects(1)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:63)
- [ ] `FeedingFilterSettings.checkHarmfulEffect(1)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:67)
- [ ] `FeedingFilterSettings.getNutritionDifference(2)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:85)
- [ ] `FeedingFilterSettings.getHalfOfStackHunger(1)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingFilterSettings.java:96)

### `FeedingUpgrade`: it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)

- [x] `FeedingUpgrade.STILL_HUNGRY_COOLDOWN` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:39)
- [x] `FeedingUpgrade.FEEDING_RANGE` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:40)
- [ ] `FeedingUpgrade.FeedingUpgrade(3)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:43)
- [ ] `FeedingUpgrade.hasTagSelector(0)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:55)
- [ ] `FeedingUpgrade.getFilter(0)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:60)
- [ ] `FeedingUpgrade.createFilterSettings(3)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:65)
- [ ] `FeedingUpgrade.canEat(2)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:71)
- [ ] `FeedingUpgrade.createWidget(3)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:75)
- [ ] `FeedingUpgrade.createFilter(2)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:81)
- [ ] `FeedingUpgrade.getTickRate(0)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:99)
- [ ] `FeedingUpgrade.tick(4)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:104)
- [ ] `FeedingUpgrade.feedPlayerAndGetHungry(2)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:133)
- [ ] `FeedingUpgrade.tryFeedingFoodFromStorage(3)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:143)
- [ ] `FeedingUpgrade.tryFeedingStack(6)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:148)
- [ ] `FeedingUpgrade.isEdible(2)`: it uses Minecraft's `LivingEntity` (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:179)
- [ ] `FeedingUpgrade.spawnHeartParticles(2)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingUpgrade.java:188)

### `FeedingWidget`: it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)

- [x] `FeedingWidget.WHITELIST_TOOLTIPS` in game_side.rs (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingWidget.java:13)
- [x] `FeedingWidget.HUNGER_MODE_TOOLTIPS` in game_side.rs (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingWidget.java:16)
- [x] `FeedingWidget.IGNORE_MODE_TOOLTIPS` in game_side.rs (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingWidget.java:21)
- [ ] `FeedingWidget.FeedingWidget(3)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingWidget.java:25)
- [ ] `FeedingWidget.renderTooltip(3)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/feeding/FeedingWidget.java:41)

### `FilterSettingsBase`: it uses Minecraft's `ItemStack`

- [ ] `FilterSettingsBase.FilterSettingsBase(4)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterSettingsBase.java:24)
- [ ] `FilterSettingsBase.FilterSettingsBase(5)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterSettingsBase.java:28)
- [ ] `FilterSettingsBase.matchesFilter(2)`: it has no body (com/tiviacz/travelersbackpack/inventory/upgrades/FilterSettingsBase.java:37)
- [ ] `FilterSettingsBase.getSettings(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterSettingsBase.java:39)
- [ ] `FilterSettingsBase.isTagFilter(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterSettingsBase.java:43)
- [ ] `FilterSettingsBase.compareModId(2)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterSettingsBase.java:47)
- [ ] `FilterSettingsBase.streamStorageContents(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterSettingsBase.java:51)
- [ ] `FilterSettingsBase.reloadItemTags(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterSettingsBase.java:63)
- [ ] `FilterSettingsBase.updateFilter(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterSettingsBase.java:75)
- [ ] `FilterSettingsBase.updateFilterTags(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterSettingsBase.java:81)
- [ ] `FilterSettingsBase.updateSettings(1)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterSettingsBase.java:88)
- [ ] `FilterSettingsBase.getFilterItems(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterSettingsBase.java:92)
- [ ] `FilterSettingsBase.getFilterTags(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterSettingsBase.java:96)
- [ ] `FilterSettingsBase.getAddableTags(0)`: its class isn't translated (it uses Minecraft's `ItemStack`) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterSettingsBase.java:100)

### `FilterUpgradeBase`: it's generic

- [ ] `FilterUpgradeBase.FilterUpgradeBase(7)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java:26)
- [ ] `FilterUpgradeBase.getFilterSettings(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java:42)
- [ ] `FilterUpgradeBase.getFilter(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java:46)
- [ ] `FilterUpgradeBase.getFilterSlotCount(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java:51)
- [ ] `FilterUpgradeBase.getSlotsInRow(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java:56)
- [ ] `FilterUpgradeBase.updateSettings(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java:61)
- [ ] `FilterUpgradeBase.hasTagSelector(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java:66)
- [ ] `FilterUpgradeBase.isTagSelector(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java:70)
- [ ] `FilterUpgradeBase.getFirstFilterStack(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java:74)
- [ ] `FilterUpgradeBase.addChangeListener(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java:78)
- [ ] `FilterUpgradeBase.getUpgradeSlotsPosition(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java:84)
- [ ] `FilterUpgradeBase.getUpgradeSlots(4)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java:100)
- [ ] `FilterUpgradeBase.getTabSize(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java:128)
- [ ] `FilterUpgradeBase.createFilterSettings(3)`: it has no body (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java:147)
- [ ] `FilterUpgradeBase.createFilter(2)`: it has no body (com/tiviacz/travelersbackpack/inventory/upgrades/FilterUpgradeBase.java:149)

### `ButtonStates`

- [ ] `ButtonStates.ALLOW`: its value couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/filter/ButtonStates.java:7)
- [ ] `ButtonStates.ALLOW_FEEDING`: its value couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/filter/ButtonStates.java:14)
- [ ] `ButtonStates.OBJECT_TYPE`: its value couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/filter/ButtonStates.java:20)
- [ ] `ButtonStates.IGNORE_MODE`: its value couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/filter/ButtonStates.java:27)
- [ ] `ButtonStates.HUNGER_MODE`: its value couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/filter/ButtonStates.java:33)
- [ ] `ButtonStates.IGNORE_EFFECT_MODE`: its value couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/filter/ButtonStates.java:40)

### `ButtonStates.ButtonState`

- [x] `ButtonStates.ButtonState.ButtonState(1)` (com/tiviacz/travelersbackpack/inventory/upgrades/filter/ButtonStates.java:50)
- [ ] `ButtonStates.ButtonState.getByIndex(1)`: .toArray on a map's keys (com/tiviacz/travelersbackpack/inventory/upgrades/filter/ButtonStates.java:54)
- [x] `ButtonStates.ButtonState.getStatesCount(0)` (com/tiviacz/travelersbackpack/inventory/upgrades/filter/ButtonStates.java:58)
- [ ] `ButtonStates.ButtonState.getButtonIcon(1)`: it calls ButtonState.getByIndex, which isn't translated (com/tiviacz/travelersbackpack/inventory/upgrades/filter/ButtonStates.java:62)

### `FilterButton`: it's generic

- [ ] `FilterButton.FilterButton(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/filter/FilterButton.java:17)
- [ ] `FilterButton.getButtonState(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/filter/FilterButton.java:25)
- [ ] `FilterButton.getCurrentState(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/filter/FilterButton.java:29)
- [ ] `FilterButton.nextState(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/filter/FilterButton.java:33)
- [ ] `FilterButton.setHidden(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/filter/FilterButton.java:37)
- [ ] `FilterButton.renderButton(3)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/filter/FilterButton.java:41)
- [ ] `FilterButton.mouseClicked(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/filter/FilterButton.java:61)
- [ ] `FilterButton.isMouseOver(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/filter/FilterButton.java:70)

### `FilterHandler`: it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)

- [ ] `FilterHandler.FilterHandler(2)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/upgrades/filter/FilterHandler.java:8)
- [ ] `FilterHandler.isItemValid(2)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/upgrades/filter/FilterHandler.java:18)
- [ ] `FilterHandler.getSlotLimit(1)`: its class isn't translated (it extends `ItemStackHandler` (com.tiviacz.travelersbackpack.inventory.handler)) (com/tiviacz/travelersbackpack/inventory/upgrades/filter/FilterHandler.java:23)

### `IFilter`: it's an interface

- [ ] `IFilter.getFilter(0)`: it has no body (com/tiviacz/travelersbackpack/inventory/upgrades/filter/IFilter.java:6)
- [ ] `IFilter.updateSettings(0)`: it has no body (com/tiviacz/travelersbackpack/inventory/upgrades/filter/IFilter.java:8)

### `IFilterSlots`: it's an interface

- [ ] `IFilterSlots.getFilterSlotCount(0)`: it has no body (com/tiviacz/travelersbackpack/inventory/upgrades/filter/IFilterSlots.java:4)
- [ ] `IFilterSlots.getSlotsInRow(0)`: it has no body (com/tiviacz/travelersbackpack/inventory/upgrades/filter/IFilterSlots.java:6)
- [ ] `IFilterSlots.getRows(0)`: its class isn't translated (it's an interface) (com/tiviacz/travelersbackpack/inventory/upgrades/filter/IFilterSlots.java:8)
- [ ] `IFilterSlots.getSlotsInRow(1)`: its class isn't translated (it's an interface) (com/tiviacz/travelersbackpack/inventory/upgrades/filter/IFilterSlots.java:12)

### `IEnable`: it's an interface

- [ ] `IEnable.isEnabled(1)`: its class isn't translated (it's an interface) (com/tiviacz/travelersbackpack/inventory/upgrades/IEnable.java:6)
- [ ] `IEnable.setEnabled(1)`: its class isn't translated (it's an interface) (com/tiviacz/travelersbackpack/inventory/upgrades/IEnable.java:10)

### `IMoveSelector`: it's an interface

- [ ] `IMoveSelector.shiftClickToBackpack(1)`: its class isn't translated (it's an interface) (com/tiviacz/travelersbackpack/inventory/upgrades/IMoveSelector.java:7)

### `ITickableUpgrade`: it's an interface

- [ ] `ITickableUpgrade.tick(4)`: it has no body (com/tiviacz/travelersbackpack/inventory/upgrades/ITickableUpgrade.java:9)
- [ ] `ITickableUpgrade.getTickRate(0)`: it has no body (com/tiviacz/travelersbackpack/inventory/upgrades/ITickableUpgrade.java:11)

### `IUpgrade`: it's an interface

- [ ] `IUpgrade.remove(0)`: its class isn't translated (it's an interface) (com/tiviacz/travelersbackpack/inventory/upgrades/IUpgrade.java:17)
- [ ] `IUpgrade.onUpgradeRemoved(1)`: its class isn't translated (it's an interface) (com/tiviacz/travelersbackpack/inventory/upgrades/IUpgrade.java:20)
- [ ] `IUpgrade.onUpgradeRemoved(2)`: its class isn't translated (it's an interface) (com/tiviacz/travelersbackpack/inventory/upgrades/IUpgrade.java:23)
- [ ] `IUpgrade.createWidget(3)`: it has no body (com/tiviacz/travelersbackpack/inventory/upgrades/IUpgrade.java:27)
- [ ] `IUpgrade.getUpgradeSlotsPosition(2)`: it has no body (com/tiviacz/travelersbackpack/inventory/upgrades/IUpgrade.java:30)
- [ ] `IUpgrade.getUpgradeSlots(4)`: it has no body (com/tiviacz/travelersbackpack/inventory/upgrades/IUpgrade.java:32)
- [ ] `IUpgrade.initializeContainers(2)`: its class isn't translated (it's an interface) (com/tiviacz/travelersbackpack/inventory/upgrades/IUpgrade.java:34)
- [ ] `IUpgrade.hasTab(0)`: its class isn't translated (it's an interface) (com/tiviacz/travelersbackpack/inventory/upgrades/IUpgrade.java:37)
- [ ] `IUpgrade.getTabSize(0)`: it has no body (com/tiviacz/travelersbackpack/inventory/upgrades/IUpgrade.java:41)
- [ ] `IUpgrade.isTabOpened(0)`: it has no body (com/tiviacz/travelersbackpack/inventory/upgrades/IUpgrade.java:43)

### `JukeboxUpgrade`: it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)

- [ ] `JukeboxUpgrade.JukeboxUpgrade(3)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxUpgrade.java:29)
- [ ] `JukeboxUpgrade.onUpgradeRemoved(1)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxUpgrade.java:34)
- [ ] `JukeboxUpgrade.createWidget(3)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxUpgrade.java:39)
- [ ] `JukeboxUpgrade.getUpgradeSlotsPosition(2)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxUpgrade.java:45)
- [ ] `JukeboxUpgrade.getUpgradeSlots(4)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxUpgrade.java:52)
- [ ] `JukeboxUpgrade.isPlayingRecord(0)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxUpgrade.java:69)
- [ ] `JukeboxUpgrade.canPlayRecord(0)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxUpgrade.java:73)
- [ ] `JukeboxUpgrade.setSlotChanged(3)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxUpgrade.java:77)
- [ ] `JukeboxUpgrade.createHandler(1)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxUpgrade.java:85)

### `JukeboxWidget`: it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)

- [ ] `JukeboxWidget.JukeboxWidget(3)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxWidget.java:36)
- [ ] `JukeboxWidget.extractRenderState(4)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxWidget.java:40)
- [ ] `JukeboxWidget.renderBg(5)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxWidget.java:76)
- [ ] `JukeboxWidget.mouseClicked(2)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxWidget.java:84)
- [ ] `JukeboxWidget.isMouseOverPlayButton(2)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxWidget.java:118)
- [ ] `JukeboxWidget.isMouseOverStopButton(2)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxWidget.java:122)
- [ ] `JukeboxWidget.getFromDisk(1)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxWidget.java:126)
- [ ] `JukeboxWidget.playDiscToPlayer(2)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxWidget.java:131)
- [ ] `JukeboxWidget.stopDisc(1)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxWidget.java:145)

### `JukeboxWidget.MovingSound`: it extends Minecraft's `AbstractTickableSoundInstance`

- [ ] `JukeboxWidget.MovingSound.MovingSound(2)`: its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxWidget.java:155)
- [ ] `JukeboxWidget.MovingSound.tick(0)`: its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxWidget.java:163)
- [ ] `JukeboxWidget.MovingSound.shouldStopPlaying(1)`: its class isn't translated (it extends Minecraft's `AbstractTickableSoundInstance`) (com/tiviacz/travelersbackpack/inventory/upgrades/jukebox/JukeboxWidget.java:177)

### `LanternUpgrade`: it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)

- [x] `LanternUpgrade.LANTERN` (com/tiviacz/travelersbackpack/inventory/upgrades/lantern/LanternUpgrade.java:20)
- [ ] `LanternUpgrade.LanternUpgrade(2)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/lantern/LanternUpgrade.java:22)
- [ ] `LanternUpgrade.createWidget(3)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/lantern/LanternUpgrade.java:27)
- [ ] `LanternUpgrade.setEnabled(1)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/lantern/LanternUpgrade.java:35)
- [ ] `LanternUpgrade.writeToRenderData(2)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/lantern/LanternUpgrade.java:40)
- [ ] `LanternUpgrade.onUpgradeRemoved(1)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/lantern/LanternUpgrade.java:48)
- [ ] `LanternUpgrade.hasTab(0)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/lantern/LanternUpgrade.java:54)
- [ ] `LanternUpgrade.getUpgradeSlotsPosition(2)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/lantern/LanternUpgrade.java:59)
- [ ] `LanternUpgrade.getUpgradeSlots(4)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/lantern/LanternUpgrade.java:64)

### `LanternWidget`: it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)

- [ ] `LanternWidget.LanternWidget(5)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/lantern/LanternWidget.java:11)
- [ ] `LanternWidget.getAdditionalTooltips(1)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/lantern/LanternWidget.java:15)

### `MagnetFilterSettings`: it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)

- [x] `MagnetFilterSettings.ALLOW_MODE` (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java:14)
- [x] `MagnetFilterSettings.OBJECT_CATEGORY` (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java:15)
- [x] `MagnetFilterSettings.IGNORE_MODE` (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java:16)
- [x] `MagnetFilterSettings.ALLOW` (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java:17)
- [x] `MagnetFilterSettings.BLOCK` (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java:18)
- [x] `MagnetFilterSettings.ITEM` (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java:19)
- [x] `MagnetFilterSettings.MOD_ID` (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java:20)
- [x] `MagnetFilterSettings.TAG_ID` (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java:21)
- [x] `MagnetFilterSettings.MATCH_COMPONENTS` (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java:22)
- [x] `MagnetFilterSettings.IGNORE_COMPONENTS` (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java:23)
- [ ] `MagnetFilterSettings.MagnetFilterSettings(4)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java:25)
- [ ] `MagnetFilterSettings.matchesFilter(2)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java:29)
- [ ] `MagnetFilterSettings.isTagFilter(0)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java:60)
- [ ] `MagnetFilterSettings.compare(2)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java:65)
- [ ] `MagnetFilterSettings.compareItemStack(2)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetFilterSettings.java:69)

### `MagnetUpgrade`: it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)

- [ ] `MagnetUpgrade.MagnetUpgrade(4)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetUpgrade.java:31)
- [ ] `MagnetUpgrade.createFilterSettings(3)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetUpgrade.java:43)
- [ ] `MagnetUpgrade.createWidget(3)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetUpgrade.java:52)
- [ ] `MagnetUpgrade.getTickRate(0)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetUpgrade.java:58)
- [ ] `MagnetUpgrade.getPullRange(0)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetUpgrade.java:63)
- [ ] `MagnetUpgrade.tick(4)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetUpgrade.java:67)
- [ ] `MagnetUpgrade.teleportNearbyItems(3)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetUpgrade.java:84)
- [ ] `MagnetUpgrade.createFilter(2)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetUpgrade.java:110)

### `MagnetWidget`: it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)

- [x] `MagnetWidget.WHITELIST_TOOLTIPS` in game_side.rs (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetWidget.java:13)
- [x] `MagnetWidget.OBJECT_TOOLTIPS` in game_side.rs (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetWidget.java:16)
- [x] `MagnetWidget.IGNORE_MODE_TOOLTIPS` in game_side.rs (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetWidget.java:21)
- [ ] `MagnetWidget.MagnetWidget(3)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetWidget.java:25)
- [ ] `MagnetWidget.renderTooltip(3)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/magnet/MagnetWidget.java:41)

### `AutoPickupFilterSettings`: it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)

- [x] `AutoPickupFilterSettings.ALLOW_MODE` (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:15)
- [x] `AutoPickupFilterSettings.OBJECT_CATEGORY` (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:16)
- [x] `AutoPickupFilterSettings.IGNORE_MODE` (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:17)
- [x] `AutoPickupFilterSettings.ALLOW` (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:18)
- [x] `AutoPickupFilterSettings.BLOCK` (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:19)
- [x] `AutoPickupFilterSettings.MATCH_CONTENTS` (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:20)
- [x] `AutoPickupFilterSettings.ITEM` (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:21)
- [x] `AutoPickupFilterSettings.MOD_ID` (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:22)
- [x] `AutoPickupFilterSettings.TAG_ID` (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:23)
- [x] `AutoPickupFilterSettings.MATCH_COMPONENTS` (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:24)
- [x] `AutoPickupFilterSettings.IGNORE_COMPONENTS` (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:25)
- [ ] `AutoPickupFilterSettings.AutoPickupFilterSettings(4)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:27)
- [ ] `AutoPickupFilterSettings.matchesFilter(2)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:31)
- [ ] `AutoPickupFilterSettings.isTagFilter(0)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:66)
- [ ] `AutoPickupFilterSettings.compare(2)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:71)
- [ ] `AutoPickupFilterSettings.compareItemStack(2)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupFilterSettings.java:75)

### `AutoPickupUpgrade`: it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)

- [x] `AutoPickupUpgrade.PICKUP_RANGE` (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupUpgrade.java:31)
- [ ] `AutoPickupUpgrade.AutoPickupUpgrade(4)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupUpgrade.java:33)
- [ ] `AutoPickupUpgrade.canPickup(1)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupUpgrade.java:45)
- [ ] `AutoPickupUpgrade.createFilterSettings(3)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupUpgrade.java:49)
- [ ] `AutoPickupUpgrade.createWidget(3)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupUpgrade.java:58)
- [ ] `AutoPickupUpgrade.getTickRate(0)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupUpgrade.java:64)
- [ ] `AutoPickupUpgrade.tick(4)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupUpgrade.java:69)
- [ ] `AutoPickupUpgrade.tryPickup(3)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupUpgrade.java:84)
- [ ] `AutoPickupUpgrade.createFilter(2)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupUpgrade.java:99)

### `AutoPickupWidget`: it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)

- [x] `AutoPickupWidget.WHITELIST_TOOLTIPS` in game_side.rs (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupWidget.java:14)
- [x] `AutoPickupWidget.OBJECT_TOOLTIPS` in game_side.rs (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupWidget.java:19)
- [x] `AutoPickupWidget.IGNORE_MODE_TOOLTIPS` in game_side.rs (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupWidget.java:24)
- [ ] `AutoPickupWidget.AutoPickupWidget(3)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupWidget.java:28)
- [ ] `AutoPickupWidget.renderBg(5)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupWidget.java:44)
- [ ] `AutoPickupWidget.renderTooltip(3)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/pickup/AutoPickupWidget.java:52)

### `Point`

- [x] `Point.Point(2)` (com/tiviacz/travelersbackpack/inventory/upgrades/Point.java:3)
- [x] `Point.x(0)` (com/tiviacz/travelersbackpack/inventory/upgrades/Point.java:3)
- [x] `Point.y(0)` (com/tiviacz/travelersbackpack/inventory/upgrades/Point.java:3)

### `RefillUpgrade`: it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)

- [x] `RefillUpgrade.REFILL_RANGE` (com/tiviacz/travelersbackpack/inventory/upgrades/refill/RefillUpgrade.java:39)
- [ ] `RefillUpgrade.RefillUpgrade(3)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/refill/RefillUpgrade.java:42)
- [ ] `RefillUpgrade.getFilterSlotCount(0)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/refill/RefillUpgrade.java:47)
- [ ] `RefillUpgrade.getSlotsInRow(0)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/refill/RefillUpgrade.java:52)
- [ ] `RefillUpgrade.createWidget(3)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/refill/RefillUpgrade.java:57)
- [ ] `RefillUpgrade.getUpgradeSlotsPosition(2)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/refill/RefillUpgrade.java:65)
- [ ] `RefillUpgrade.getUpgradeSlots(4)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/refill/RefillUpgrade.java:78)
- [ ] `RefillUpgrade.tick(4)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/refill/RefillUpgrade.java:96)
- [ ] `RefillUpgrade.tryRefillItems(1)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/refill/RefillUpgrade.java:113)
- [ ] `RefillUpgrade.refill(3)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/refill/RefillUpgrade.java:121)
- [ ] `RefillUpgrade.countAndSupply(3)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/refill/RefillUpgrade.java:153)
- [ ] `RefillUpgrade.getMissingCount(2)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/refill/RefillUpgrade.java:172)
- [ ] `RefillUpgrade.getTickRate(0)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/refill/RefillUpgrade.java:176)
- [ ] `RefillUpgrade.createFilter(2)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/refill/RefillUpgrade.java:181)

### `ResultArrowElement`: it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`)

- [ ] `ResultArrowElement.ResultArrowElement(3)`: its class isn't translated (it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`)) (com/tiviacz/travelersbackpack/inventory/upgrades/ResultArrowElement.java:17)
- [ ] `ResultArrowElement.renderBg(5)`: its class isn't translated (it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`)) (com/tiviacz/travelersbackpack/inventory/upgrades/ResultArrowElement.java:23)
- [ ] `ResultArrowElement.renderTooltip(3)`: its class isn't translated (it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`)) (com/tiviacz/travelersbackpack/inventory/upgrades/ResultArrowElement.java:55)
- [ ] `ResultArrowElement.mouseClicked(1)`: its class isn't translated (it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`)) (com/tiviacz/travelersbackpack/inventory/upgrades/ResultArrowElement.java:71)
- [ ] `ResultArrowElement.isMouseOverShiftClickButton(2)`: its class isn't translated (it uses BackpackScreen, which isn't translated (it extends `AbstractBackpackScreen`)) (com/tiviacz/travelersbackpack/inventory/upgrades/ResultArrowElement.java:88)

### `AbstractSmeltingUpgrade`: it's generic

- [x] `AbstractSmeltingUpgrade.SLOT_INPUT` (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:56)
- [x] `AbstractSmeltingUpgrade.SLOT_FUEL` (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:57)
- [x] `AbstractSmeltingUpgrade.SLOT_RESULT` (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:58)
- [ ] `AbstractSmeltingUpgrade.AbstractSmeltingUpgrade(5)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:68)
- [ ] `AbstractSmeltingUpgrade.createWidget(3)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:84)
- [ ] `AbstractSmeltingUpgrade.onUpgradeRemoved(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:92)
- [ ] `AbstractSmeltingUpgrade.getUpgradeSlotsPosition(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:101)
- [ ] `AbstractSmeltingUpgrade.getUpgradeSlots(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:110)
- [ ] `AbstractSmeltingUpgrade.setEnabled(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:119)
- [ ] `AbstractSmeltingUpgrade.syncClient(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:129)
- [ ] `AbstractSmeltingUpgrade.tick(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:149)
- [ ] `AbstractSmeltingUpgrade.getTickRate(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:174)
- [ ] `AbstractSmeltingUpgrade.isFuel(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:179)
- [ ] `AbstractSmeltingUpgrade.isBurning(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:183)
- [ ] `AbstractSmeltingUpgrade.isCooking(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:187)
- [ ] `AbstractSmeltingUpgrade.hasFuel(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:191)
- [ ] `AbstractSmeltingUpgrade.tickSmelting(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:195)
- [ ] `AbstractSmeltingUpgrade.checkCooking(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:224)
- [ ] `AbstractSmeltingUpgrade.startCooking(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:242)
- [ ] `AbstractSmeltingUpgrade.startBurning(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:248)
- [ ] `AbstractSmeltingUpgrade.canBurn(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:257)
- [ ] `AbstractSmeltingUpgrade.finishCooking(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:280)
- [ ] `AbstractSmeltingUpgrade.shrinkFuelSlot(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:313)
- [ ] `AbstractSmeltingUpgrade.finishBurning(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:323)
- [ ] `AbstractSmeltingUpgrade.setStack(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:334)
- [ ] `AbstractSmeltingUpgrade.getStack(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:338)
- [ ] `AbstractSmeltingUpgrade.getSpeedMultiplier(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:342)
- [ ] `AbstractSmeltingUpgrade.getTotalCookTime(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:346)
- [ ] `AbstractSmeltingUpgrade.getBurnDuration(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:351)
- [ ] `AbstractSmeltingUpgrade.getLootContextBlockState(0)`: it has no body (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:357)
- [ ] `AbstractSmeltingUpgrade.getLootContext(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:359)
- [ ] `AbstractSmeltingUpgrade.getBurnTotalTime(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:372)
- [ ] `AbstractSmeltingUpgrade.getBurnFinishTime(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:376)
- [ ] `AbstractSmeltingUpgrade.getCookingTotalTime(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:380)
- [ ] `AbstractSmeltingUpgrade.getCookingFinishTime(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:384)
- [ ] `AbstractSmeltingUpgrade.getSpeedMultiplier(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:388)
- [ ] `AbstractSmeltingUpgrade.setBurnTotalTime(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:392)
- [ ] `AbstractSmeltingUpgrade.setBurnFinishTime(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:396)
- [ ] `AbstractSmeltingUpgrade.setCookingTotalTime(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:400)
- [ ] `AbstractSmeltingUpgrade.setCookingFinishTime(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:404)
- [ ] `AbstractSmeltingUpgrade.setSpeedMultiplier(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:408)
- [ ] `AbstractSmeltingUpgrade.stopCooking(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:412)
- [ ] `AbstractSmeltingUpgrade.stopBurning(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:417)
- [ ] `AbstractSmeltingUpgrade.setSlotChanged(3)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:422)
- [ ] `AbstractSmeltingUpgrade.setStackData(2)`: it's generic (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:430)
- [ ] `AbstractSmeltingUpgrade.createHandler(1)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingUpgrade.java:441)

### `AbstractSmeltingWidget`: it's generic

- [ ] `AbstractSmeltingWidget.AbstractSmeltingWidget(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingWidget.java:16)
- [ ] `AbstractSmeltingWidget.renderBg(5)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingWidget.java:21)
- [ ] `AbstractSmeltingWidget.extractRenderState(4)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingWidget.java:27)
- [ ] `AbstractSmeltingWidget.renderTooltip(3)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingWidget.java:50)
- [ ] `AbstractSmeltingWidget.mouseClicked(2)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/AbstractSmeltingWidget.java:56)

### `BlastFurnaceUpgrade`: it extends `AbstractSmeltingUpgrade`

- [ ] `BlastFurnaceUpgrade.BlastFurnaceUpgrade(3)`: its class isn't translated (it extends `AbstractSmeltingUpgrade`) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/BlastFurnaceUpgrade.java:11)
- [ ] `BlastFurnaceUpgrade.getLootContextBlockState(0)`: its class isn't translated (it extends `AbstractSmeltingUpgrade`) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/BlastFurnaceUpgrade.java:15)

### `FurnaceUpgrade`: it extends `AbstractSmeltingUpgrade`

- [ ] `FurnaceUpgrade.FurnaceUpgrade(3)`: its class isn't translated (it extends `AbstractSmeltingUpgrade`) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/FurnaceUpgrade.java:11)
- [ ] `FurnaceUpgrade.getLootContextBlockState(0)`: its class isn't translated (it extends `AbstractSmeltingUpgrade`) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/FurnaceUpgrade.java:15)

### `SmokerUpgrade`: it extends `AbstractSmeltingUpgrade`

- [ ] `SmokerUpgrade.SmokerUpgrade(3)`: its class isn't translated (it extends `AbstractSmeltingUpgrade`) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/SmokerUpgrade.java:11)
- [ ] `SmokerUpgrade.getLootContextBlockState(0)`: its class isn't translated (it extends `AbstractSmeltingUpgrade`) (com/tiviacz/travelersbackpack/inventory/upgrades/smelting/SmokerUpgrade.java:15)

### `FluidStorageItemWrapper`: it implements Fabric's `SingleSlotStorage`

- [ ] `FluidStorageItemWrapper.FluidStorageItemWrapper(2)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/FluidStorageItemWrapper.java:17)
- [ ] `FluidStorageItemWrapper.insert(3)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/FluidStorageItemWrapper.java:22)
- [ ] `FluidStorageItemWrapper.extract(3)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/FluidStorageItemWrapper.java:42)
- [ ] `FluidStorageItemWrapper.getTanksUpgrade(1)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/FluidStorageItemWrapper.java:62)
- [ ] `FluidStorageItemWrapper.isResourceBlank(0)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/FluidStorageItemWrapper.java:68)
- [ ] `FluidStorageItemWrapper.getResource(0)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/FluidStorageItemWrapper.java:72)
- [ ] `FluidStorageItemWrapper.getAmount(0)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/FluidStorageItemWrapper.java:76)
- [ ] `FluidStorageItemWrapper.getCapacity(0)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/FluidStorageItemWrapper.java:80)
- [ ] `FluidStorageItemWrapper.getTank(1)`: its class isn't translated (it implements Fabric's `SingleSlotStorage`) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/FluidStorageItemWrapper.java:84)

### `TankActions`

- [ ] `TankActions.fillTank(2)`: it uses Minecraft's `ServerPlayer` (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TankActions.java:26)
- [ ] `TankActions.tryEmptyPotion(3)`: it uses FluidTank, which isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TankActions.java:98)
- [ ] `TankActions.tryFillPotion(4)`: it uses FluidTank, which isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TankActions.java:115)

### `TanksUpgrade`: it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)

- [ ] `TanksUpgrade.TanksUpgrade(3)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java:40)
- [ ] `TanksUpgrade.getLeftTank(0)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java:47)
- [ ] `TanksUpgrade.getRightTank(0)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java:51)
- [ ] `TanksUpgrade.getFluidSlotsHandler(0)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java:55)
- [ ] `TanksUpgrade.setTanksCapacity(0)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java:59)
- [ ] `TanksUpgrade.setFluids(1)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java:64)
- [ ] `TanksUpgrade.syncClients(1)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java:69)
- [ ] `TanksUpgrade.createFluidHandler(1)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java:82)
- [ ] `TanksUpgrade.writeToRenderData(1)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java:95)
- [ ] `TanksUpgrade.remove(0)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java:101)
- [ ] `TanksUpgrade.createWidget(3)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java:110)
- [ ] `TanksUpgrade.getUpgradeSlotsPosition(2)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java:116)
- [ ] `TanksUpgrade.getUpgradeSlots(4)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java:126)
- [ ] `TanksUpgrade.onUpgradeRemoved(2)`: its class isn't translated (it extends `UpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java:136)
- [ ] `TanksUpgrade.createTemporaryHandler(0)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TanksUpgrade.java:141)

### `TankWidget`: it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)

- [ ] `TankWidget.TankWidget(3)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TankWidget.java:40)
- [ ] `TankWidget.renderTooltip(3)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TankWidget.java:49)
- [ ] `TankWidget.renderAboveBg(6)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TankWidget.java:61)
- [ ] `TankWidget.getTankHeight(1)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TankWidget.java:72)
- [ ] `TankWidget.mouseClicked(2)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TankWidget.java:76)
- [ ] `TankWidget.isMouseOver(2)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TankWidget.java:89)
- [ ] `TankWidget.isValid(1)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TankWidget.java:97)
- [ ] `TankWidget.renderTank(8)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TankWidget.java:101)
- [ ] `TankWidget.getTankTooltip(1)`: it uses FluidTank, which isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TankWidget.java:126)
- [ ] `TankWidget.inTank(3)`: its class isn't translated (it extends `UpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/tanks/TankWidget.java:166)

### `UpgradeBase`: it's generic

- [ ] `UpgradeBase.UpgradeBase(3)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/UpgradeBase.java:15)
- [ ] `UpgradeBase.getUpgradeManager(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/UpgradeBase.java:21)
- [ ] `UpgradeBase.getDataHolderSlot(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/UpgradeBase.java:25)
- [ ] `UpgradeBase.isTabOpened(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/UpgradeBase.java:29)
- [ ] `UpgradeBase.getDataHolderStack(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/UpgradeBase.java:34)
- [ ] `UpgradeBase.getTabSize(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/UpgradeBase.java:38)
- [ ] `UpgradeBase.updateDataHolderUnchecked(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/UpgradeBase.java:56)
- [ ] `UpgradeBase.updateDataHolderUnchecked(2)`: it's generic (com/tiviacz/travelersbackpack/inventory/upgrades/UpgradeBase.java:64)
- [ ] `UpgradeBase.setCooldown(1)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/UpgradeBase.java:72)
- [ ] `UpgradeBase.getCooldown(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/UpgradeBase.java:78)
- [ ] `UpgradeBase.hasCooldown(0)`: its class isn't translated (it's generic) (com/tiviacz/travelersbackpack/inventory/upgrades/UpgradeBase.java:82)

### `VoidFilterSettings`: it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)

- [x] `VoidFilterSettings.ALLOW_MODE` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:14)
- [x] `VoidFilterSettings.OBJECT_CATEGORY` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:15)
- [x] `VoidFilterSettings.IGNORE_MODE` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:16)
- [x] `VoidFilterSettings.ALLOW` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:17)
- [x] `VoidFilterSettings.BLOCK` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:18)
- [x] `VoidFilterSettings.MATCH_CONTENTS` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:19)
- [x] `VoidFilterSettings.ITEM` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:20)
- [x] `VoidFilterSettings.MOD_ID` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:21)
- [x] `VoidFilterSettings.TAG_ID` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:22)
- [x] `VoidFilterSettings.MATCH_COMPONENTS` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:23)
- [x] `VoidFilterSettings.IGNORE_COMPONENTS` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:24)
- [ ] `VoidFilterSettings.VoidFilterSettings(4)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:26)
- [ ] `VoidFilterSettings.matchesFilter(2)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:30)
- [ ] `VoidFilterSettings.isTagFilter(0)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:61)
- [ ] `VoidFilterSettings.compare(2)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:66)
- [ ] `VoidFilterSettings.compareItemStack(2)`: its class isn't translated (it extends `FilterSettingsBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidFilterSettings.java:70)

### `VoidUpgrade`: it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)

- [ ] `VoidUpgrade.VoidUpgrade(4)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidUpgrade.java:20)
- [ ] `VoidUpgrade.getFilter(0)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidUpgrade.java:32)
- [ ] `VoidUpgrade.createFilterSettings(3)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidUpgrade.java:37)
- [ ] `VoidUpgrade.canVoid(1)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidUpgrade.java:46)
- [ ] `VoidUpgrade.createWidget(3)`: its class isn't translated (it extends `FilterUpgradeBase` (com.tiviacz.travelersbackpack.inventory.upgrades)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidUpgrade.java:50)
- [ ] `VoidUpgrade.createFilter(2)`: its body couldn't be read: an anonymous class (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidUpgrade.java:56)

### `VoidWidget`: it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)

- [x] `VoidWidget.TRASH_UPGRADE` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:31)
- [x] `VoidWidget.RED_HIGHLIGHT_COLOR` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:32)
- [x] `VoidWidget.RED_ARROW_COLOR` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:33)
- [x] `VoidWidget.HIGHLIGHT_TANK_COLOR` (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:34)
- [x] `VoidWidget.WHITELIST_TOOLTIPS` in game_side.rs (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:41)
- [x] `VoidWidget.OBJECT_TOOLTIPS` in game_side.rs (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:46)
- [x] `VoidWidget.IGNORE_MODE_TOOLTIPS` in game_side.rs (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:51)
- [ ] `VoidWidget.VoidWidget(3)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:55)
- [ ] `VoidWidget.renderBg(5)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:69)
- [ ] `VoidWidget.renderUnderTooltip(4)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:89)
- [ ] `VoidWidget.renderTooltip(3)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:100)
- [ ] `VoidWidget.renderOnTop(4)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:144)
- [ ] `VoidWidget.isMouseOver(2)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:155)
- [ ] `VoidWidget.isWithinTrashBinSlot(2)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:160)
- [ ] `VoidWidget.mouseDragged(3)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:164)
- [ ] `VoidWidget.mouseClicked(2)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:168)
- [ ] `VoidWidget.isHoveringWithTrashBin(0)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:189)
- [ ] `VoidWidget.select(1)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:193)
- [ ] `VoidWidget.selectTank(5)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:215)
- [ ] `VoidWidget.canSelectSlot(1)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:231)
- [ ] `VoidWidget.selectSlots(2)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:235)
- [ ] `VoidWidget.drawRedTankHighlight(6)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:250)
- [ ] `VoidWidget.drawRedHighlight(2)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:263)
- [ ] `VoidWidget.renderRedDownArrow(3)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:273)
- [ ] `VoidWidget.renderTrashBinAnimation(4)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:280)
- [ ] `VoidWidget.renderTrashBin(5)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:293)
- [ ] `VoidWidget.tickAnimation(0)`: its class isn't translated (it extends `FilterUpgradeWidgetBase` (com.tiviacz.travelersbackpack.client.screens.widgets)) (com/tiviacz/travelersbackpack/inventory/upgrades/voiding/VoidWidget.java:303)

### `BackpackTankItem`: it extends Minecraft's `Item`

- [ ] `BackpackTankItem.BackpackTankItem(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/BackpackTankItem.java:9)
- [ ] `BackpackTankItem.isEnabled(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/BackpackTankItem.java:13)

### `HoseItem`: it extends Minecraft's `Item`

- [x] `HoseItem.NO_ASSIGN` (com/tiviacz/travelersbackpack/item/HoseItem.java:70)
- [x] `HoseItem.SUCK_MODE` (com/tiviacz/travelersbackpack/item/HoseItem.java:71)
- [x] `HoseItem.SPILL_MODE` (com/tiviacz/travelersbackpack/item/HoseItem.java:72)
- [x] `HoseItem.DRINK_MODE` (com/tiviacz/travelersbackpack/item/HoseItem.java:73)
- [ ] `HoseItem.HoseItem(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:75)
- [ ] `HoseItem.getUseAnimation(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:79)
- [ ] `HoseItem.getUseDuration(2)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:83)
- [ ] `HoseItem.onUseTick(4)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:87)
- [ ] `HoseItem.shouldEmitDrinkingSounds(1)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:93)
- [ ] `HoseItem.consumeTicks(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:100)
- [ ] `HoseItem.consumeSeconds(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:104)
- [ ] `HoseItem.emitDrinkingSound(2)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:108)
- [ ] `HoseItem.use(3)`: its body couldn't be read: a labelled statement (com/tiviacz/travelersbackpack/item/HoseItem.java:112)
- [ ] `HoseItem.drink(4)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:295)
- [ ] `HoseItem.spillPotion(3)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:304)
- [ ] `HoseItem.emptyContents(5)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:330)
- [ ] `HoseItem.playEmptySound(4)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:376)
- [ ] `HoseItem.triggerAdvancement(2)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:381)
- [ ] `HoseItem.finishUsingItem(3)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:387)
- [ ] `HoseItem.getHoseMode(1)`: ItemStack.has with something other than a DataComponents constant (com/tiviacz/travelersbackpack/item/HoseItem.java:415)
- [ ] `HoseItem.getHoseTank(1)`: it calls Minecraft's ItemStack.getOrDefault (no Horizon helper for it yet) (com/tiviacz/travelersbackpack/item/HoseItem.java:419)
- [ ] `HoseItem.getSelectedFluidTank(2)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:425)
- [ ] `HoseItem.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/HoseItem.java:429)

### `SleepingBagItem`: it extends Minecraft's `BlockItem`

- [ ] `SleepingBagItem.SleepingBagItem(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/item/SleepingBagItem.java:17)
- [ ] `SleepingBagItem.isEnabled(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/item/SleepingBagItem.java:21)
- [ ] `SleepingBagItem.getDefaultColor(0)`: it uses TravelersBackpackConfig.serverSpec, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)) (com/tiviacz/travelersbackpack/item/SleepingBagItem.java:27)
- [ ] `SleepingBagItem.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/item/SleepingBagItem.java:35)

### `TravelersBackpackItem`: it extends Minecraft's `BlockItem`

- [ ] `TravelersBackpackItem.TravelersBackpackItem(2)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/item/TravelersBackpackItem.java:63)
- [ ] `TravelersBackpackItem.use(3)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/item/TravelersBackpackItem.java:67)
- [ ] `TravelersBackpackItem.useOn(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/item/TravelersBackpackItem.java:84)
- [ ] `TravelersBackpackItem.place(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/item/TravelersBackpackItem.java:89)
- [ ] `TravelersBackpackItem.isCreative(1)`: it calls Minecraft's LocalPlayer.level (no Horizon helper for it yet) (com/tiviacz/travelersbackpack/item/TravelersBackpackItem.java:137)
- [ ] `TravelersBackpackItem.overrideStackedOnOther(4)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/item/TravelersBackpackItem.java:141)
- [ ] `TravelersBackpackItem.overrideOtherStackedOnMe(6)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/item/TravelersBackpackItem.java:164)
- [ ] `TravelersBackpackItem.add(4)`: it calls BackpackSlotItemHandler.isItemValid, which isn't translated (com/tiviacz/travelersbackpack/item/TravelersBackpackItem.java:182)
- [ ] `TravelersBackpackItem.playInsertSound(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/item/TravelersBackpackItem.java:193)
- [ ] `TravelersBackpackItem.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/item/TravelersBackpackItem.java:197)
- [ ] `TravelersBackpackItem.addAttributeModifierTooltip(3)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/item/TravelersBackpackItem.java:283)
- [ ] `TravelersBackpackItem.getTooltipImage(1)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/item/TravelersBackpackItem.java:330)
- [ ] `TravelersBackpackItem.canFitInsideContainerItems(0)`: its class isn't translated (it extends Minecraft's `BlockItem`) (com/tiviacz/travelersbackpack/item/TravelersBackpackItem.java:334)

### `BlastFurnaceUpgradeItem`: it extends `UpgradeItem`

- [ ] `BlastFurnaceUpgradeItem.BlastFurnaceUpgradeItem(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/BlastFurnaceUpgradeItem.java:18)
- [ ] `BlastFurnaceUpgradeItem.isEnabled(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/BlastFurnaceUpgradeItem.java:22)
- [ ] `BlastFurnaceUpgradeItem.isTickingUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/BlastFurnaceUpgradeItem.java:28)
- [ ] `BlastFurnaceUpgradeItem.getUpgradeClass(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/BlastFurnaceUpgradeItem.java:33)
- [ ] `BlastFurnaceUpgradeItem.getUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/BlastFurnaceUpgradeItem.java:38)

### `CraftingUpgradeItem`: it extends `UpgradeItem`

- [ ] `CraftingUpgradeItem.CraftingUpgradeItem(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/CraftingUpgradeItem.java:18)
- [ ] `CraftingUpgradeItem.isEnabled(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/CraftingUpgradeItem.java:22)
- [ ] `CraftingUpgradeItem.requiresEquippedBackpack(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/CraftingUpgradeItem.java:28)
- [ ] `CraftingUpgradeItem.getUpgradeClass(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/CraftingUpgradeItem.java:33)
- [ ] `CraftingUpgradeItem.getUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/CraftingUpgradeItem.java:38)

### `FeedingUpgradeItem`: it extends `UpgradeItem`

- [ ] `FeedingUpgradeItem.FeedingUpgradeItem(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/FeedingUpgradeItem.java:18)
- [ ] `FeedingUpgradeItem.isEnabled(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/FeedingUpgradeItem.java:22)
- [ ] `FeedingUpgradeItem.isTickingUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/FeedingUpgradeItem.java:28)
- [ ] `FeedingUpgradeItem.getUpgradeClass(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/FeedingUpgradeItem.java:33)
- [ ] `FeedingUpgradeItem.getUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/FeedingUpgradeItem.java:38)

### `FurnaceUpgradeItem`: it extends `UpgradeItem`

- [ ] `FurnaceUpgradeItem.FurnaceUpgradeItem(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/FurnaceUpgradeItem.java:18)
- [ ] `FurnaceUpgradeItem.isEnabled(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/FurnaceUpgradeItem.java:22)
- [ ] `FurnaceUpgradeItem.isTickingUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/FurnaceUpgradeItem.java:28)
- [ ] `FurnaceUpgradeItem.getUpgradeClass(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/FurnaceUpgradeItem.java:33)
- [ ] `FurnaceUpgradeItem.getUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/FurnaceUpgradeItem.java:38)

### `JukeboxUpgradeItem`: it extends `UpgradeItem`

- [ ] `JukeboxUpgradeItem.JukeboxUpgradeItem(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/JukeboxUpgradeItem.java:18)
- [ ] `JukeboxUpgradeItem.isEnabled(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/JukeboxUpgradeItem.java:22)
- [ ] `JukeboxUpgradeItem.hasBlockFunctionality(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/JukeboxUpgradeItem.java:28)
- [ ] `JukeboxUpgradeItem.getUpgradeClass(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/JukeboxUpgradeItem.java:33)
- [ ] `JukeboxUpgradeItem.getUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/JukeboxUpgradeItem.java:38)

### `LanternUpgradeItem`: it extends `UpgradeItem`

- [ ] `LanternUpgradeItem.LanternUpgradeItem(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/LanternUpgradeItem.java:22)
- [ ] `LanternUpgradeItem.isEnabled(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/LanternUpgradeItem.java:26)
- [ ] `LanternUpgradeItem.appendHoverText(5)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/LanternUpgradeItem.java:32)
- [x] `LanternUpgradeItem.addStatusTooltip(1)` in game_side.rs (com/tiviacz/travelersbackpack/item/upgrade/LanternUpgradeItem.java:41)
- [ ] `LanternUpgradeItem.hasBlockFunctionality(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/LanternUpgradeItem.java:49)
- [ ] `LanternUpgradeItem.getUpgradeClass(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/LanternUpgradeItem.java:54)
- [ ] `LanternUpgradeItem.getUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/LanternUpgradeItem.java:59)

### `MagnetUpgradeItem`: it extends `UpgradeItem`

- [ ] `MagnetUpgradeItem.MagnetUpgradeItem(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/MagnetUpgradeItem.java:29)
- [ ] `MagnetUpgradeItem.isEnabled(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/MagnetUpgradeItem.java:33)
- [ ] `MagnetUpgradeItem.appendHoverText(5)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/MagnetUpgradeItem.java:39)
- [ ] `MagnetUpgradeItem.isTickingUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/MagnetUpgradeItem.java:54)
- [ ] `MagnetUpgradeItem.getUpgradeClass(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/MagnetUpgradeItem.java:59)
- [ ] `MagnetUpgradeItem.getUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/MagnetUpgradeItem.java:64)

### `PickupUpgradeItem`: it extends `UpgradeItem`

- [ ] `PickupUpgradeItem.PickupUpgradeItem(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/PickupUpgradeItem.java:21)
- [ ] `PickupUpgradeItem.isEnabled(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/PickupUpgradeItem.java:25)
- [ ] `PickupUpgradeItem.getUpgradeClass(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/PickupUpgradeItem.java:31)
- [ ] `PickupUpgradeItem.getUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/PickupUpgradeItem.java:36)

### `RefillUpgradeItem`: it extends `UpgradeItem`

- [ ] `RefillUpgradeItem.RefillUpgradeItem(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/RefillUpgradeItem.java:18)
- [ ] `RefillUpgradeItem.isEnabled(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/RefillUpgradeItem.java:22)
- [ ] `RefillUpgradeItem.isTickingUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/RefillUpgradeItem.java:28)
- [ ] `RefillUpgradeItem.getUpgradeClass(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/RefillUpgradeItem.java:33)
- [ ] `RefillUpgradeItem.getUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/RefillUpgradeItem.java:38)

### `SmokerUpgradeItem`: it extends `UpgradeItem`

- [ ] `SmokerUpgradeItem.SmokerUpgradeItem(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/SmokerUpgradeItem.java:18)
- [ ] `SmokerUpgradeItem.isEnabled(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/SmokerUpgradeItem.java:22)
- [ ] `SmokerUpgradeItem.isTickingUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/SmokerUpgradeItem.java:28)
- [ ] `SmokerUpgradeItem.getUpgradeClass(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/SmokerUpgradeItem.java:33)
- [ ] `SmokerUpgradeItem.getUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/SmokerUpgradeItem.java:38)

### `TanksUpgradeItem`: it extends `UpgradeItem`

- [ ] `TanksUpgradeItem.TanksUpgradeItem(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/TanksUpgradeItem.java:28)
- [ ] `TanksUpgradeItem.isEnabled(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/TanksUpgradeItem.java:32)
- [ ] `TanksUpgradeItem.canBePutInBackpack(2)`: ItemStack.has with something other than a DataComponents constant (com/tiviacz/travelersbackpack/item/upgrade/TanksUpgradeItem.java:38)
- [ ] `TanksUpgradeItem.getLeftFluidStack(1)`: it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/item/upgrade/TanksUpgradeItem.java:49)
- [ ] `TanksUpgradeItem.getRightFluidStack(1)`: it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/item/upgrade/TanksUpgradeItem.java:58)
- [ ] `TanksUpgradeItem.writeToRenderData(0)`: it uses RenderInfo, which isn't translated (it uses Minecraft's `CompoundTag`) (com/tiviacz/travelersbackpack/item/upgrade/TanksUpgradeItem.java:67)
- [ ] `TanksUpgradeItem.writeToRenderData(1)`: it uses Minecraft's `CompoundTag` (com/tiviacz/travelersbackpack/item/upgrade/TanksUpgradeItem.java:74)
- [ ] `TanksUpgradeItem.appendHoverText(5)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/TanksUpgradeItem.java:79)
- [ ] `TanksUpgradeItem.requiresEquippedBackpack(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/TanksUpgradeItem.java:107)
- [ ] `TanksUpgradeItem.getUpgradeClass(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/TanksUpgradeItem.java:112)
- [ ] `TanksUpgradeItem.getUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/TanksUpgradeItem.java:117)

### `TierUpgrade`: it extends Minecraft's `Item`

- [ ] `TierUpgrade.TierUpgrade(2)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/upgrade/TierUpgrade.java:19)
- [ ] `TierUpgrade.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/upgrade/TierUpgrade.java:24)

### `TierUpgrade.Upgrade`


### `UpgradeItem`: it extends Minecraft's `Item`

- [ ] `UpgradeItem.UpgradeItem(2)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/upgrade/UpgradeItem.java:22)
- [ ] `UpgradeItem.appendHoverText(5)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/upgrade/UpgradeItem.java:29)
- [ ] `UpgradeItem.isTickingUpgrade(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/upgrade/UpgradeItem.java:41)
- [ ] `UpgradeItem.requiresEquippedBackpack(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/upgrade/UpgradeItem.java:45)
- [ ] `UpgradeItem.hasBlockFunctionality(0)`: its class isn't translated (it extends Minecraft's `Item`) (com/tiviacz/travelersbackpack/item/upgrade/UpgradeItem.java:49)
- [ ] `UpgradeItem.getUpgradeClass(0)`: it has no body (com/tiviacz/travelersbackpack/item/upgrade/UpgradeItem.java:53)
- [ ] `UpgradeItem.getUpgrade(0)`: it has no body (com/tiviacz/travelersbackpack/item/upgrade/UpgradeItem.java:55)

### `VoidUpgradeItem`: it extends `UpgradeItem`

- [ ] `VoidUpgradeItem.VoidUpgradeItem(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:21)
- [ ] `VoidUpgradeItem.isEnabled(1)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:25)
- [ ] `VoidUpgradeItem.requiresEquippedBackpack(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:31)
- [ ] `VoidUpgradeItem.getUpgradeClass(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:36)
- [ ] `VoidUpgradeItem.getUpgrade(0)`: its class isn't translated (it extends `UpgradeItem`) (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:41)
- [ ] `EndermanEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:15)
- [ ] `ExperienceOrbEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:15)
- [ ] `GhastEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:16)
- [ ] `LootTableMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:24)
- [ ] `SmallFireballEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:16)
- [ ] `AbstractContainerScreenMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:12)
- [ ] `EntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:15)
- [ ] `ItemEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:29)
- [ ] `HudMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:15)
- [ ] `HumanoidRenderStateMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:11)
- [ ] `HumoanoidMobRenderer`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:24)
- [ ] `ItemStackMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:16)
- [ ] `LevelResourceMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:8)
- [ ] `LivingEntityPriorityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:15)
- [ ] `LocalPlayerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:19)
- [ ] `MobEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:32)
- [ ] `PlayerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:34)
- [ ] `ShulkerBoxBlockEntityMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/tiviacz/travelersbackpack/item/upgrade/VoidUpgradeItem.java:13)

### `ClientboundSendMessagePacket`: it implements Minecraft's `CustomPacketPayload`

- [x] `ClientboundSendMessagePacket.ID` (com/tiviacz/travelersbackpack/network/ClientboundSendMessagePacket.java:16)
- [ ] `ClientboundSendMessagePacket.TYPE`: it uses Minecraft's `Type` (com/tiviacz/travelersbackpack/network/ClientboundSendMessagePacket.java:17)
- [ ] `ClientboundSendMessagePacket.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/network/ClientboundSendMessagePacket.java:18)
- [ ] `ClientboundSendMessagePacket.ClientboundSendMessagePacket(2)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSendMessagePacket.java:15)
- [ ] `ClientboundSendMessagePacket.handle(2)`: it uses ClientboundSendMessagePacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSendMessagePacket.java:22)
- [ ] `ClientboundSendMessagePacket.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSendMessagePacket.java:42)
- [ ] `ClientboundSendMessagePacket.drop(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSendMessagePacket.java:15)
- [ ] `ClientboundSendMessagePacket.pos(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSendMessagePacket.java:15)

### `ClientboundSyncAttachmentPacket`: it implements Minecraft's `CustomPacketPayload`

- [x] `ClientboundSyncAttachmentPacket.ID` (com/tiviacz/travelersbackpack/network/ClientboundSyncAttachmentPacket.java:21)
- [ ] `ClientboundSyncAttachmentPacket.TYPE`: it uses Minecraft's `Type` (com/tiviacz/travelersbackpack/network/ClientboundSyncAttachmentPacket.java:22)
- [ ] `ClientboundSyncAttachmentPacket.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/network/ClientboundSyncAttachmentPacket.java:23)
- [ ] `ClientboundSyncAttachmentPacket.ClientboundSyncAttachmentPacket(2)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncAttachmentPacket.java:33)
- [ ] `ClientboundSyncAttachmentPacket.ClientboundSyncAttachmentPacket(3)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncAttachmentPacket.java:37)
- [ ] `ClientboundSyncAttachmentPacket.handle(2)`: it uses ClientboundSyncAttachmentPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncAttachmentPacket.java:69)
- [ ] `ClientboundSyncAttachmentPacket.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncAttachmentPacket.java:82)
- [ ] `ClientboundSyncAttachmentPacket.entityID(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncAttachmentPacket.java:20)
- [ ] `ClientboundSyncAttachmentPacket.backpack(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncAttachmentPacket.java:20)
- [ ] `ClientboundSyncAttachmentPacket.removeData(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncAttachmentPacket.java:20)

### `ClientboundSyncComponentsPacket`: it implements Minecraft's `CustomPacketPayload`

- [x] `ClientboundSyncComponentsPacket.ID` (com/tiviacz/travelersbackpack/network/ClientboundSyncComponentsPacket.java:16)
- [ ] `ClientboundSyncComponentsPacket.TYPE`: it uses Minecraft's `Type` (com/tiviacz/travelersbackpack/network/ClientboundSyncComponentsPacket.java:17)
- [ ] `ClientboundSyncComponentsPacket.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/network/ClientboundSyncComponentsPacket.java:18)
- [ ] `ClientboundSyncComponentsPacket.ClientboundSyncComponentsPacket(2)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncComponentsPacket.java:15)
- [ ] `ClientboundSyncComponentsPacket.handle(2)`: it uses ClientboundSyncComponentsPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncComponentsPacket.java:26)
- [ ] `ClientboundSyncComponentsPacket.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncComponentsPacket.java:33)
- [ ] `ClientboundSyncComponentsPacket.entityID(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncComponentsPacket.java:15)
- [ ] `ClientboundSyncComponentsPacket.map(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncComponentsPacket.java:15)

### `ClientboundSyncItemStackPacket`: it implements Minecraft's `CustomPacketPayload`

- [x] `ClientboundSyncItemStackPacket.ID` (com/tiviacz/travelersbackpack/network/ClientboundSyncItemStackPacket.java:20)
- [ ] `ClientboundSyncItemStackPacket.TYPE`: it uses Minecraft's `Type` (com/tiviacz/travelersbackpack/network/ClientboundSyncItemStackPacket.java:21)
- [ ] `ClientboundSyncItemStackPacket.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/network/ClientboundSyncItemStackPacket.java:22)
- [ ] `ClientboundSyncItemStackPacket.ClientboundSyncItemStackPacket(4)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncItemStackPacket.java:19)
- [ ] `ClientboundSyncItemStackPacket.handle(2)`: it uses ClientboundSyncItemStackPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncItemStackPacket.java:34)
- [ ] `ClientboundSyncItemStackPacket.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncItemStackPacket.java:54)
- [ ] `ClientboundSyncItemStackPacket.entityId(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncItemStackPacket.java:19)
- [ ] `ClientboundSyncItemStackPacket.slot(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncItemStackPacket.java:19)
- [ ] `ClientboundSyncItemStackPacket.itemInstance(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncItemStackPacket.java:19)
- [ ] `ClientboundSyncItemStackPacket.map(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundSyncItemStackPacket.java:19)

### `ClientboundUpdateRecipePacket`: it implements Minecraft's `CustomPacketPayload`

- [x] `ClientboundUpdateRecipePacket.ID` (com/tiviacz/travelersbackpack/network/ClientboundUpdateRecipePacket.java:16)
- [ ] `ClientboundUpdateRecipePacket.TYPE`: it uses Minecraft's `Type` (com/tiviacz/travelersbackpack/network/ClientboundUpdateRecipePacket.java:17)
- [ ] `ClientboundUpdateRecipePacket.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/network/ClientboundUpdateRecipePacket.java:18)
- [ ] `ClientboundUpdateRecipePacket.ClientboundUpdateRecipePacket(1)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundUpdateRecipePacket.java:15)
- [ ] `ClientboundUpdateRecipePacket.handle(2)`: it uses ClientboundUpdateRecipePacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundUpdateRecipePacket.java:22)
- [ ] `ClientboundUpdateRecipePacket.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundUpdateRecipePacket.java:37)
- [ ] `ClientboundUpdateRecipePacket.output(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ClientboundUpdateRecipePacket.java:15)

### `ServerboundActionTagPacket`: it implements Minecraft's `CustomPacketPayload`

- [ ] `ServerboundActionTagPacket.TYPE`: it uses Minecraft's `Type` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:23)
- [ ] `ServerboundActionTagPacket.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:24)
- [x] `ServerboundActionTagPacket.UPGRADE_TAB` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:27)
- [x] `ServerboundActionTagPacket.OPEN_SCREEN` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:28)
- [x] `ServerboundActionTagPacket.OPEN_BACKPACK` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:29)
- [x] `ServerboundActionTagPacket.SORTER` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:30)
- [x] `ServerboundActionTagPacket.SLEEPING_BAG` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:31)
- [x] `ServerboundActionTagPacket.FILL_TANK` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:32)
- [x] `ServerboundActionTagPacket.SWAP_TOOL` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:33)
- [x] `ServerboundActionTagPacket.TOGGLE_BUTTONS_VISIBILITY` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:34)
- [x] `ServerboundActionTagPacket.SHOW_TOOL_SLOTS` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:35)
- [x] `ServerboundActionTagPacket.REMOVE_UPGRADE` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:36)
- [x] `ServerboundActionTagPacket.OPEN_SETTINGS` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:37)
- [x] `ServerboundActionTagPacket.SWITCH_HOSE_MODE` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:38)
- [x] `ServerboundActionTagPacket.SWITCH_HOSE_TANK` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:39)
- [x] `ServerboundActionTagPacket.TOGGLE_VISIBILITY` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:40)
- [x] `ServerboundActionTagPacket.ABILITY_SLIDER` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:41)
- [x] `ServerboundActionTagPacket.EQUIP_BACKPACK` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:42)
- [x] `ServerboundActionTagPacket.SET_STACK` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:43)
- [ ] `ServerboundActionTagPacket.ServerboundActionTagPacket(1)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:22)
- [ ] `ServerboundActionTagPacket.handle(2)`: it uses ServerboundActionTagPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:45)
- [ ] `ServerboundActionTagPacket.create(2)`: it uses `Object` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:138)
- [ ] `ServerboundActionTagPacket.createPacketTag(2)`: it uses Minecraft's `CompoundTag` (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:142)
- [ ] `ServerboundActionTagPacket.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:166)
- [ ] `ServerboundActionTagPacket.actionTag(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundActionTagPacket.java:22)

### `ServerboundFilterSettingsPacket`: it implements Minecraft's `CustomPacketPayload`

- [ ] `ServerboundFilterSettingsPacket.TYPE`: it uses Minecraft's `Type` (com/tiviacz/travelersbackpack/network/ServerboundFilterSettingsPacket.java:21)
- [ ] `ServerboundFilterSettingsPacket.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/network/ServerboundFilterSettingsPacket.java:22)
- [ ] `ServerboundFilterSettingsPacket.ServerboundFilterSettingsPacket(2)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundFilterSettingsPacket.java:20)
- [ ] `ServerboundFilterSettingsPacket.handle(2)`: it uses ServerboundFilterSettingsPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundFilterSettingsPacket.java:30)
- [ ] `ServerboundFilterSettingsPacket.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundFilterSettingsPacket.java:53)
- [ ] `ServerboundFilterSettingsPacket.slot(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundFilterSettingsPacket.java:20)
- [ ] `ServerboundFilterSettingsPacket.settings(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundFilterSettingsPacket.java:20)

### `ServerboundFilterTagsPacket`: it implements Minecraft's `CustomPacketPayload`

- [ ] `ServerboundFilterTagsPacket.TYPE`: it uses Minecraft's `Type` (com/tiviacz/travelersbackpack/network/ServerboundFilterTagsPacket.java:21)
- [ ] `ServerboundFilterTagsPacket.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/network/ServerboundFilterTagsPacket.java:22)
- [ ] `ServerboundFilterTagsPacket.ServerboundFilterTagsPacket(2)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundFilterTagsPacket.java:20)
- [ ] `ServerboundFilterTagsPacket.handle(2)`: it uses ServerboundFilterTagsPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundFilterTagsPacket.java:30)
- [ ] `ServerboundFilterTagsPacket.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundFilterTagsPacket.java:53)
- [ ] `ServerboundFilterTagsPacket.slot(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundFilterTagsPacket.java:20)
- [ ] `ServerboundFilterTagsPacket.tags(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundFilterTagsPacket.java:20)

### `ServerboundRetrieveBackpackPacket`: it implements Minecraft's `CustomPacketPayload`

- [ ] `ServerboundRetrieveBackpackPacket.TYPE`: it uses Minecraft's `Type` (com/tiviacz/travelersbackpack/network/ServerboundRetrieveBackpackPacket.java:16)
- [ ] `ServerboundRetrieveBackpackPacket.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/network/ServerboundRetrieveBackpackPacket.java:17)
- [ ] `ServerboundRetrieveBackpackPacket.ServerboundRetrieveBackpackPacket(1)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundRetrieveBackpackPacket.java:15)
- [ ] `ServerboundRetrieveBackpackPacket.handle(2)`: it uses ServerboundRetrieveBackpackPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundRetrieveBackpackPacket.java:21)
- [ ] `ServerboundRetrieveBackpackPacket.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundRetrieveBackpackPacket.java:40)
- [ ] `ServerboundRetrieveBackpackPacket.backpackHolder(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundRetrieveBackpackPacket.java:15)

### `ServerboundSlotPacket`: it implements Minecraft's `CustomPacketPayload`

- [ ] `ServerboundSlotPacket.TYPE`: it uses Minecraft's `Type` (com/tiviacz/travelersbackpack/network/ServerboundSlotPacket.java:21)
- [ ] `ServerboundSlotPacket.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/network/ServerboundSlotPacket.java:22)
- [x] `ServerboundSlotPacket.UNSORTABLES` (com/tiviacz/travelersbackpack/network/ServerboundSlotPacket.java:31)
- [x] `ServerboundSlotPacket.MEMORY` (com/tiviacz/travelersbackpack/network/ServerboundSlotPacket.java:32)
- [ ] `ServerboundSlotPacket.ServerboundSlotPacket(3)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundSlotPacket.java:20)
- [ ] `ServerboundSlotPacket.handle(2)`: it uses ServerboundSlotPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundSlotPacket.java:34)
- [ ] `ServerboundSlotPacket.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundSlotPacket.java:75)
- [ ] `ServerboundSlotPacket.selectType(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundSlotPacket.java:20)
- [ ] `ServerboundSlotPacket.unsortables(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundSlotPacket.java:20)
- [ ] `ServerboundSlotPacket.memorizedSlots(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/ServerboundSlotPacket.java:20)

### `SupporterBadgePacket`


### `SupporterBadgePacket.Clientbound`: it implements Minecraft's `CustomPacketPayload`

- [ ] `SupporterBadgePacket.Clientbound.TYPE`: it uses Minecraft's `Type` (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java:16)
- [ ] `SupporterBadgePacket.Clientbound.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java:19)
- [ ] `SupporterBadgePacket.Clientbound.Clientbound(2)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java:15)
- [ ] `SupporterBadgePacket.Clientbound.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java:27)
- [ ] `SupporterBadgePacket.Clientbound.handle(2)`: it uses Clientbound, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java:31)
- [ ] `SupporterBadgePacket.Clientbound.isEnabledForPlayer(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java:15)
- [ ] `SupporterBadgePacket.Clientbound.playerName(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java:15)

### `SupporterBadgePacket.Serverbound`: it implements Minecraft's `CustomPacketPayload`

- [ ] `SupporterBadgePacket.Serverbound.TYPE`: it uses Minecraft's `Type` (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java:45)
- [ ] `SupporterBadgePacket.Serverbound.STREAM_CODEC`: it uses Minecraft's `StreamCodec` (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java:48)
- [ ] `SupporterBadgePacket.Serverbound.Serverbound(1)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java:44)
- [ ] `SupporterBadgePacket.Serverbound.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java:52)
- [ ] `SupporterBadgePacket.Serverbound.handle(2)`: it uses Serverbound, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java:56)
- [ ] `SupporterBadgePacket.Serverbound.isEnabledForPlayer(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (com/tiviacz/travelersbackpack/network/SupporterBadgePacket.java:44)

### `TravelersBackpack`: it implements Fabric's `ModInitializer`

- [x] `TravelersBackpack.MODID` (com/tiviacz/travelersbackpack/TravelersBackpack.java:45)
- [x] `TravelersBackpack.LOGGER` (com/tiviacz/travelersbackpack/TravelersBackpack.java:46)
- [ ] `TravelersBackpack.currentServer`: it uses Minecraft's `MinecraftServer` (com/tiviacz/travelersbackpack/TravelersBackpack.java:47)
- [x] `TravelersBackpack.accessoriesLoaded` (com/tiviacz/travelersbackpack/TravelersBackpack.java:48)
- [x] `TravelersBackpack.trinketsLoaded` (com/tiviacz/travelersbackpack/TravelersBackpack.java:49)
- [x] `TravelersBackpack.craftingTweaksLoaded` (com/tiviacz/travelersbackpack/TravelersBackpack.java:50)
- [x] `TravelersBackpack.toughasnailsLoaded` (com/tiviacz/travelersbackpack/TravelersBackpack.java:51)
- [x] `TravelersBackpack.comfortsLoaded` (com/tiviacz/travelersbackpack/TravelersBackpack.java:52)
- [x] `TravelersBackpack.universalGravesLoaded` (com/tiviacz/travelersbackpack/TravelersBackpack.java:53)
- [x] `TravelersBackpack.pneumonoGravestonesLoaded` (com/tiviacz/travelersbackpack/TravelersBackpack.java:54)
- [x] `TravelersBackpack.vinurlLoaded` (com/tiviacz/travelersbackpack/TravelersBackpack.java:55)
- [x] `TravelersBackpack.polymorphLoaded` (com/tiviacz/travelersbackpack/TravelersBackpack.java:56)
- [x] `TravelersBackpack.trashSlotLoaded` (com/tiviacz/travelersbackpack/TravelersBackpack.java:57)
- [x] `TravelersBackpack.lambDynamicLightsLoaded` (com/tiviacz/travelersbackpack/TravelersBackpack.java:58)
- [x] `TravelersBackpack.mouseTweaksLoaded` (com/tiviacz/travelersbackpack/TravelersBackpack.java:59)
- [ ] `TravelersBackpack.onInitialize(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (com/tiviacz/travelersbackpack/TravelersBackpack.java:61)
- [ ] `TravelersBackpack.registerDeathHandler(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (com/tiviacz/travelersbackpack/TravelersBackpack.java:135)
- [ ] `TravelersBackpack.getCurrentServer(0)`: it uses Minecraft's `MinecraftServer` (com/tiviacz/travelersbackpack/TravelersBackpack.java:139)
- [ ] `TravelersBackpack.enableIntegration(0)`: it calls TravelersBackpack.enableTrinkets, which isn't translated (com/tiviacz/travelersbackpack/TravelersBackpack.java:144)
- [ ] `TravelersBackpack.enableAccessories(0)`: it uses TravelersBackpackConfig.serverSpec, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)) (com/tiviacz/travelersbackpack/TravelersBackpack.java:148)
- [ ] `TravelersBackpack.enableTrinkets(0)`: it calls TravelersBackpack.enableAccessories, which isn't translated (com/tiviacz/travelersbackpack/TravelersBackpack.java:154)
- [x] `TravelersBackpack.isAnyGraveModInstalled(0)` (com/tiviacz/travelersbackpack/TravelersBackpack.java:161)

### `TravelersBackpackClient`: it implements Fabric's `ClientModInitializer`

- [x] `TravelersBackpackClient.entityModelFeaturesLoaded` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:92)
- [x] `TravelersBackpackClient.STAR_MODEL` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:93)
- [ ] `TravelersBackpackClient.STAR_MODEL_KEY`: it uses Fabric's `ExtraModelKey` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:94)
- [x] `TravelersBackpackClient.BACKPACK_BASE` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:95)
- [ ] `TravelersBackpackClient.BACKPACK_BASE_KEY`: it uses Fabric's `ExtraModelKey` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:96)
- [x] `TravelersBackpackClient.BACKPACK_BASE_DYED` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:97)
- [ ] `TravelersBackpackClient.BACKPACK_BASE_DYED_KEY`: it uses Fabric's `ExtraModelKey` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:98)
- [x] `TravelersBackpackClient.BACKPACK_EXTRAS` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:99)
- [ ] `TravelersBackpackClient.BACKPACK_EXTRAS_KEY`: it uses Fabric's `ExtraModelKey` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:100)
- [x] `TravelersBackpackClient.BACKPACK_FOX_NOSE` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:101)
- [ ] `TravelersBackpackClient.BACKPACK_FOX_NOSE_KEY`: it uses Fabric's `ExtraModelKey` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:102)
- [x] `TravelersBackpackClient.BACKPACK_OCELOT_NOSE` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:103)
- [ ] `TravelersBackpackClient.BACKPACK_OCELOT_NOSE_KEY`: it uses Fabric's `ExtraModelKey` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:104)
- [x] `TravelersBackpackClient.BACKPACK_PIG_NOSE` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:105)
- [ ] `TravelersBackpackClient.BACKPACK_PIG_NOSE_KEY`: it uses Fabric's `ExtraModelKey` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:106)
- [x] `TravelersBackpackClient.BACKPACK_SLEEPING_BAG` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:107)
- [ ] `TravelersBackpackClient.BACKPACK_SLEEPING_BAG_KEY`: it uses Fabric's `ExtraModelKey` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:108)
- [x] `TravelersBackpackClient.BACKPACK_SLEEPING_BAG_EXTRAS` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:109)
- [ ] `TravelersBackpackClient.BACKPACK_SLEEPING_BAG_EXTRAS_KEY`: it uses Fabric's `ExtraModelKey` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:110)
- [x] `TravelersBackpackClient.BACKPACK_TANKS` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:111)
- [ ] `TravelersBackpackClient.BACKPACK_TANKS_KEY`: it uses Fabric's `ExtraModelKey` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:112)
- [x] `TravelersBackpackClient.BACKPACK_VILLAGER_NOSE` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:113)
- [ ] `TravelersBackpackClient.BACKPACK_VILLAGER_NOSE_KEY`: it uses Fabric's `ExtraModelKey` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:114)
- [x] `TravelersBackpackClient.BACKPACK_WARDEN_HORNS` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:115)
- [ ] `TravelersBackpackClient.BACKPACK_WARDEN_HORNS_KEY`: it uses Fabric's `ExtraModelKey` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:116)
- [x] `TravelersBackpackClient.BACKPACK_WOLF_NOSE` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:117)
- [ ] `TravelersBackpackClient.BACKPACK_WOLF_NOSE_KEY`: it uses Fabric's `ExtraModelKey` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:118)
- [ ] `TravelersBackpackClient.POTION_MODEL`: it uses Unbaked, which isn't translated (it implements `Unbaked`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:119)
- [ ] `TravelersBackpackClient.onInitializeClient(0)`: done another way in Horizon: it runs as `on_init`, from Horizon's init hook (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:126)
- [ ] `TravelersBackpackClient.registerCustomModels(0)`: a lambda (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:166)
- [ ] `TravelersBackpackClient.registerBlockColorProvider(0)`: it creates a BackpackBlockTintSource, which isn't translated (it implements Minecraft's `BlockTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:185)
- [ ] `TravelersBackpackClient.registerItemColorProvider(0)`: it uses Minecraft's `ItemTintSources`.ID_MAPPER (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:248)
- [ ] `TravelersBackpackClient.registerBackpackItemEntityRenderer(0)`: it uses ModItems.BACKPACK_ITEM_ENTITY, which isn't translated (it uses Minecraft's `EntityType`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:261)
- [ ] `TravelersBackpackClient.registerSpecialRenderers(0)`: it calls HoseSpecialRenderer.register, which isn't translated (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:265)
- [ ] `TravelersBackpackClient.registerFeatureRenderers(0)`: it uses Fabric's `LivingEntityRenderLayerRegistrationCallback`.EVENT (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:269)
- [ ] `TravelersBackpackClient.registerHudOverlay(0)`: it uses Fabric's `VanillaHudElements`.HOTBAR (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:286)
- [ ] `TravelersBackpackClient.setupFluidRendering(0)`: it uses ModFluids.POTION_STILL, which isn't translated (it uses Minecraft's `FlowingFluid`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:292)
- [ ] `TravelersBackpackClient.registerTooltipComponent(0)`: it uses Fabric's `ClientTooltipComponentCallback`.EVENT (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:300)
- [ ] `TravelersBackpackClient.registerClientCommands(0)`: it uses Fabric's `ClientCommandRegistrationCallback`.EVENT (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:309)

### `TravelersBackpackClient.BackpackTintSources`


### `TravelersBackpackClient.BackpackTintSources.BackpackBlockTintSource`: it implements Minecraft's `BlockTintSource`

- [ ] `TravelersBackpackClient.BackpackTintSources.BackpackBlockTintSource.color(1)`: its class isn't translated (it implements Minecraft's `BlockTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:315)
- [ ] `TravelersBackpackClient.BackpackTintSources.BackpackBlockTintSource.colorInWorld(3)`: its class isn't translated (it implements Minecraft's `BlockTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:319)

### `TravelersBackpackClient.BackpackTintSources.BackpackItemTintSource`: it implements Minecraft's `ItemTintSource`

- [ ] `TravelersBackpackClient.BackpackTintSources.BackpackItemTintSource.MAP_CODEC`: it uses Minecraft's `MapCodec` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:327)
- [ ] `TravelersBackpackClient.BackpackTintSources.BackpackItemTintSource.BackpackItemTintSource(1)`: its class isn't translated (it implements Minecraft's `ItemTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:326)
- [ ] `TravelersBackpackClient.BackpackTintSources.BackpackItemTintSource.calculate(3)`: its class isn't translated (it implements Minecraft's `ItemTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:334)
- [ ] `TravelersBackpackClient.BackpackTintSources.BackpackItemTintSource.type(0)`: its class isn't translated (it implements Minecraft's `ItemTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:339)
- [ ] `TravelersBackpackClient.BackpackTintSources.BackpackItemTintSource.defaultColor(0)`: its class isn't translated (it implements Minecraft's `ItemTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:326)

### `TravelersBackpackClient.BackpackTintSources.LeftFluidBlockTintSource`: it implements Minecraft's `BlockTintSource`

- [ ] `TravelersBackpackClient.BackpackTintSources.LeftFluidBlockTintSource.color(1)`: its class isn't translated (it implements Minecraft's `BlockTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:345)
- [ ] `TravelersBackpackClient.BackpackTintSources.LeftFluidBlockTintSource.colorInWorld(3)`: its class isn't translated (it implements Minecraft's `BlockTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:349)

### `TravelersBackpackClient.BackpackTintSources.LeftFluidItemTintSource`: it implements Minecraft's `ItemTintSource`

- [ ] `TravelersBackpackClient.BackpackTintSources.LeftFluidItemTintSource.MAP_CODEC`: it uses Minecraft's `MapCodec` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:360)
- [ ] `TravelersBackpackClient.BackpackTintSources.LeftFluidItemTintSource.LeftFluidItemTintSource(1)`: its class isn't translated (it implements Minecraft's `ItemTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:359)
- [ ] `TravelersBackpackClient.BackpackTintSources.LeftFluidItemTintSource.calculate(3)`: its class isn't translated (it implements Minecraft's `ItemTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:367)
- [ ] `TravelersBackpackClient.BackpackTintSources.LeftFluidItemTintSource.type(0)`: its class isn't translated (it implements Minecraft's `ItemTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:377)
- [ ] `TravelersBackpackClient.BackpackTintSources.LeftFluidItemTintSource.defaultColor(0)`: its class isn't translated (it implements Minecraft's `ItemTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:359)

### `TravelersBackpackClient.BackpackTintSources.RightFluidBlockTintSource`: it implements Minecraft's `BlockTintSource`

- [ ] `TravelersBackpackClient.BackpackTintSources.RightFluidBlockTintSource.color(1)`: its class isn't translated (it implements Minecraft's `BlockTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:383)
- [ ] `TravelersBackpackClient.BackpackTintSources.RightFluidBlockTintSource.colorInWorld(3)`: its class isn't translated (it implements Minecraft's `BlockTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:387)

### `TravelersBackpackClient.BackpackTintSources.RightFluidItemTintSource`: it implements Minecraft's `ItemTintSource`

- [ ] `TravelersBackpackClient.BackpackTintSources.RightFluidItemTintSource.MAP_CODEC`: it uses Minecraft's `MapCodec` (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:398)
- [ ] `TravelersBackpackClient.BackpackTintSources.RightFluidItemTintSource.RightFluidItemTintSource(1)`: its class isn't translated (it implements Minecraft's `ItemTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:397)
- [ ] `TravelersBackpackClient.BackpackTintSources.RightFluidItemTintSource.calculate(3)`: its class isn't translated (it implements Minecraft's `ItemTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:405)
- [ ] `TravelersBackpackClient.BackpackTintSources.RightFluidItemTintSource.type(0)`: its class isn't translated (it implements Minecraft's `ItemTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:415)
- [ ] `TravelersBackpackClient.BackpackTintSources.RightFluidItemTintSource.defaultColor(0)`: its class isn't translated (it implements Minecraft's `ItemTintSource`) (com/tiviacz/travelersbackpack/TravelersBackpackClient.java:397)

### `TravelersBackpackMixinPlugin`: it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)

- [ ] `TravelersBackpackMixinPlugin.onLoad(1)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (com/tiviacz/travelersbackpack/TravelersBackpackMixinPlugin.java:11)
- [ ] `TravelersBackpackMixinPlugin.getRefMapperConfig(0)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (com/tiviacz/travelersbackpack/TravelersBackpackMixinPlugin.java:14)
- [ ] `TravelersBackpackMixinPlugin.shouldApplyMixin(2)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (com/tiviacz/travelersbackpack/TravelersBackpackMixinPlugin.java:18)
- [ ] `TravelersBackpackMixinPlugin.acceptTargets(2)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (com/tiviacz/travelersbackpack/TravelersBackpackMixinPlugin.java:24)
- [ ] `TravelersBackpackMixinPlugin.getMixins(0)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (com/tiviacz/travelersbackpack/TravelersBackpackMixinPlugin.java:27)
- [ ] `TravelersBackpackMixinPlugin.preApply(4)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (com/tiviacz/travelersbackpack/TravelersBackpackMixinPlugin.java:31)
- [ ] `TravelersBackpackMixinPlugin.postApply(4)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (com/tiviacz/travelersbackpack/TravelersBackpackMixinPlugin.java:34)

### `BackpackDeathHelper`

- [ ] `BackpackDeathHelper.onPlayerDrops(3)`: it uses Minecraft's `Level` (com/tiviacz/travelersbackpack/util/BackpackDeathHelper.java:23)
- [ ] `BackpackDeathHelper.placeBackpack(4)`: it uses Minecraft's `Level` (com/tiviacz/travelersbackpack/util/BackpackDeathHelper.java:44)
- [ ] `BackpackDeathHelper.placeBackpackInTheWorld(5)`: it uses Minecraft's `Level` (com/tiviacz/travelersbackpack/util/BackpackDeathHelper.java:91)
- [ ] `BackpackDeathHelper.tryPlace(3)`: it uses Minecraft's `Level` (com/tiviacz/travelersbackpack/util/BackpackDeathHelper.java:105)
- [ ] `BackpackDeathHelper.getTicker(3)`: it's generic (com/tiviacz/travelersbackpack/util/BackpackDeathHelper.java:125)
- [ ] `BackpackDeathHelper.getNearestEmptyChunkCoordinatesSpiral(9)`: it uses Minecraft's `BlockPos` (com/tiviacz/travelersbackpack/util/BackpackDeathHelper.java:132)
- [ ] `BackpackDeathHelper.isTopSolid(3)`: it uses Minecraft's `Level` (com/tiviacz/travelersbackpack/util/BackpackDeathHelper.java:202)
- [ ] `BackpackDeathHelper.checkCoordsForBackpack(6)`: it uses Minecraft's `BlockPos` (com/tiviacz/travelersbackpack/util/BackpackDeathHelper.java:206)
- [ ] `BackpackDeathHelper.areCoordinatesTheSame(2)`: it uses Minecraft's `BlockPos` (com/tiviacz/travelersbackpack/util/BackpackDeathHelper.java:217)
- [ ] `BackpackDeathHelper.findBlock3D(7)`: it uses Minecraft's `BlockPos` (com/tiviacz/travelersbackpack/util/BackpackDeathHelper.java:221)

### `ClientRegistryHelper`

- [ ] `ClientRegistryHelper.getRegistryAccess(0)`: it uses Minecraft's `RegistryAccess` (com/tiviacz/travelersbackpack/util/ClientRegistryHelper.java:9)

### `ContainerContentsHelper`

- [ ] `ContainerContentsHelper.getItems(2)`: it uses Minecraft's `NonNullList` (com/tiviacz/travelersbackpack/util/ContainerContentsHelper.java:8)
- [ ] `ContainerContentsHelper.updateStack(4)`: it uses Minecraft's `ItemContainerContents` (com/tiviacz/travelersbackpack/util/ContainerContentsHelper.java:15)

### `CooldownHelper`

- [ ] `CooldownHelper.createCooldown(2)`: it calls Minecraft's `RandomSource`.create (com/tiviacz/travelersbackpack/util/CooldownHelper.java:6)
- [x] `CooldownHelper.seconds(1)` (com/tiviacz/travelersbackpack/util/CooldownHelper.java:11)

### `FluidStackHelper`

- [ ] `FluidStackHelper.getFluidEmptySound(1)`: it uses Minecraft's `SoundEvent` (com/tiviacz/travelersbackpack/util/FluidStackHelper.java:20)
- [ ] `FluidStackHelper.getFluidFillSound(1)`: it uses Minecraft's `SoundEvent` (com/tiviacz/travelersbackpack/util/FluidStackHelper.java:29)
- [ ] `FluidStackHelper.setPotionFluidVariant(2)`: it uses Fabric's `FluidVariant` (com/tiviacz/travelersbackpack/util/FluidStackHelper.java:38)
- [ ] `FluidStackHelper.getPotionTypeFromFluidVariant(1)`: it uses Minecraft's `PotionContents` (com/tiviacz/travelersbackpack/util/FluidStackHelper.java:56)
- [ ] `FluidStackHelper.getItemStackFromFluidStack(1)`: it uses Fabric's `FluidVariant` (com/tiviacz/travelersbackpack/util/FluidStackHelper.java:60)
- [ ] `FluidStackHelper.getSplashItemStackFromFluidStack(1)`: it uses Fabric's `FluidVariant` (com/tiviacz/travelersbackpack/util/FluidStackHelper.java:64)
- [ ] `FluidStackHelper.getLingeringItemStackFromFluidStack(1)`: it uses Fabric's `FluidVariant` (com/tiviacz/travelersbackpack/util/FluidStackHelper.java:68)
- [ ] `FluidStackHelper.createPotionStack(2)`: it uses Minecraft's `PotionContents` (com/tiviacz/travelersbackpack/util/FluidStackHelper.java:72)

### `FluidTypeHelper`

- [x] `FluidTypeHelper.BUCKET_EMPTY` (com/tiviacz/travelersbackpack/util/FluidTypeHelper.java:9)
- [x] `FluidTypeHelper.BUCKET_FILL` (com/tiviacz/travelersbackpack/util/FluidTypeHelper.java:10)
- [ ] `FluidTypeHelper.getFluidVariantName(1)`: it uses Fabric's `FluidVariant` (com/tiviacz/travelersbackpack/util/FluidTypeHelper.java:12)
- [ ] `FluidTypeHelper.getSound(2)`: it uses Minecraft's `SoundEvent` (com/tiviacz/travelersbackpack/util/FluidTypeHelper.java:16)

### `FluidUtil`

- [ ] `FluidUtil.getFluidStorageAtCursor(2)`: it uses Fabric's `Storage` (com/tiviacz/travelersbackpack/util/FluidUtil.java:18)
- [ ] `FluidUtil.getFluidStorageAtSlot(1)`: it uses Fabric's `Storage` (com/tiviacz/travelersbackpack/util/FluidUtil.java:23)
- [ ] `FluidUtil.hasFluidStorageConstant(1)`: it uses Fabric's `Storage` (com/tiviacz/travelersbackpack/util/FluidUtil.java:28)
- [ ] `FluidUtil.getFluidStorageConstant(1)`: it uses Fabric's `Storage` (com/tiviacz/travelersbackpack/util/FluidUtil.java:33)
- [ ] `FluidUtil.hasFluid(2)`: it uses Minecraft's `AbstractContainerMenu` (com/tiviacz/travelersbackpack/util/FluidUtil.java:38)
- [ ] `FluidUtil.hasFluid(1)`: it uses Fabric's `Storage` (com/tiviacz/travelersbackpack/util/FluidUtil.java:42)
- [ ] `FluidUtil.tryEmptyContainerAtCursor(4)`: its body couldn't be read: a labelled statement (com/tiviacz/travelersbackpack/util/FluidUtil.java:46)
- [ ] `FluidUtil.tryFillContainerAtCursor(4)`: its body couldn't be read: a labelled statement (com/tiviacz/travelersbackpack/util/FluidUtil.java:98)
- [ ] `FluidUtil.tryFillBucketAtCursor(4)`: its body couldn't be read: a labelled statement (com/tiviacz/travelersbackpack/util/FluidUtil.java:150)
- [ ] `FluidUtil.tryFillContainerAtSlot(5)`: it uses FluidTank, which isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/util/FluidUtil.java:202)
- [ ] `FluidUtil.isSameVariant(2)`: it uses Fabric's `FluidVariant` (com/tiviacz/travelersbackpack/util/FluidUtil.java:217)

### `HumanoidRenderStateBackpackInject`: it's an interface

- [ ] `HumanoidRenderStateBackpackInject.setBackpackStack(1)`: it has no body (com/tiviacz/travelersbackpack/util/HumanoidRenderStateBackpackInject.java:6)
- [ ] `HumanoidRenderStateBackpackInject.getBackpackStack(0)`: it has no body (com/tiviacz/travelersbackpack/util/HumanoidRenderStateBackpackInject.java:8)
- [ ] `HumanoidRenderStateBackpackInject.setChestItem(1)`: it has no body (com/tiviacz/travelersbackpack/util/HumanoidRenderStateBackpackInject.java:10)
- [ ] `HumanoidRenderStateBackpackInject.setName(1)`: it has no body (com/tiviacz/travelersbackpack/util/HumanoidRenderStateBackpackInject.java:12)
- [ ] `HumanoidRenderStateBackpackInject.getName(0)`: it has no body (com/tiviacz/travelersbackpack/util/HumanoidRenderStateBackpackInject.java:14)

### `InventoryHelper`

- [ ] `InventoryHelper.removeItem(3)`: it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/util/InventoryHelper.java:10)
- [ ] `InventoryHelper.removeItemShiftClick(3)`: it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/util/InventoryHelper.java:22)
- [ ] `InventoryHelper.takeItem(2)`: it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/util/InventoryHelper.java:33)
- [ ] `InventoryHelper.isEmpty(1)`: it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/util/InventoryHelper.java:37)
- [ ] `InventoryHelper.iteratePlayerInv(2)`: Minecraft's Inventory kept in a variable or field (com/tiviacz/travelersbackpack/util/InventoryHelper.java:47)
- [ ] `InventoryHelper.iterateHandler(2)`: it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/util/InventoryHelper.java:54)
- [ ] `InventoryHelper.iterate(2)`: it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/util/InventoryHelper.java:61)
- [ ] `InventoryHelper.addItemStackToHandler(3)`: it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/util/InventoryHelper.java:72)
- [ ] `InventoryHelper.extractFromBackpack(4)`: it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/util/InventoryHelper.java:76)
- [ ] `InventoryHelper.insertItemStacked(3)`: it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/util/InventoryHelper.java:86)
- [ ] `InventoryHelper.insertItem(3)`: it uses ItemStackHandler, which isn't translated (it extends Minecraft's `SimpleContainer`) (com/tiviacz/travelersbackpack/util/InventoryHelper.java:121)

### `ItemStackUtils`

- [ ] `ItemStackUtils.isSameItemSameTags(2)`: `instanceof HoseItem` on an item (no single item id for it) (com/tiviacz/travelersbackpack/util/ItemStackUtils.java:17)
- [ ] `ItemStackUtils.isSameItemSameComponents(2)`: it calls ItemStackUtils.checkComponentsIgnoreDamage, which isn't translated (com/tiviacz/travelersbackpack/util/ItemStackUtils.java:21)
- [ ] `ItemStackUtils.checkComponentsIgnoreDamage(2)`: it uses Minecraft's `DataComponentMap` (com/tiviacz/travelersbackpack/util/ItemStackUtils.java:27)
- [ ] `ItemStackUtils.createDataComponentMap(2)`: it uses Minecraft's `DataComponentMap` (com/tiviacz/travelersbackpack/util/ItemStackUtils.java:33)
- [ ] `ItemStackUtils.reduceSize(1)`: ItemStack.has with something other than a DataComponents constant (com/tiviacz/travelersbackpack/util/ItemStackUtils.java:47)

### `KeyHelper`

- [ ] `KeyHelper.isShiftPressed(0)`: it calls Minecraft's `InputConstants`.isKeyDown (com/tiviacz/travelersbackpack/util/KeyHelper.java:6)
- [ ] `KeyHelper.isCtrlPressed(0)`: it calls Minecraft's `InputConstants`.isKeyDown (com/tiviacz/travelersbackpack/util/KeyHelper.java:10)

### `LogHelper`

- [ ] `LogHelper.log(3)`: it uses `Level` (org.apache.logging.log4j) (com/tiviacz/travelersbackpack/util/LogHelper.java:7)
- [ ] `LogHelper.info(2)`: it uses `Object` (com/tiviacz/travelersbackpack/util/LogHelper.java:11)
- [ ] `LogHelper.error(2)`: it uses `Object` (com/tiviacz/travelersbackpack/util/LogHelper.java:15)

### `PacketDistributor`

- [ ] `PacketDistributor.sendToServer(1)`: it uses Minecraft's `CustomPacketPayload` (com/tiviacz/travelersbackpack/util/PacketDistributor.java:12)
- [ ] `PacketDistributor.sendToPlayer(2)`: it uses Minecraft's `ServerPlayer` (com/tiviacz/travelersbackpack/util/PacketDistributor.java:16)
- [ ] `PacketDistributor.sendToPlayersTrackingEntityAndSelf(2)`: it uses Minecraft's `CustomPacketPayload` (com/tiviacz/travelersbackpack/util/PacketDistributor.java:20)
- [ ] `PacketDistributor.sendToAllPlayers(2)`: it uses Minecraft's `CustomPacketPayload` (com/tiviacz/travelersbackpack/util/PacketDistributor.java:28)

### `Reference`

- [x] `Reference.ITEM_SCREEN_ID` (com/tiviacz/travelersbackpack/util/Reference.java:8)
- [x] `Reference.WEARABLE_SCREEN_ID` (com/tiviacz/travelersbackpack/util/Reference.java:9)
- [x] `Reference.BLOCK_ENTITY_SCREEN_ID` (com/tiviacz/travelersbackpack/util/Reference.java:10)
- [x] `Reference.NO_SPACE` (com/tiviacz/travelersbackpack/util/Reference.java:11)
- [x] `Reference.DEPLOY` (com/tiviacz/travelersbackpack/util/Reference.java:12)
- [ ] `Reference.ALLOWED_TYPE_ENTRIES`: it uses Minecraft's `EntityType` (com/tiviacz/travelersbackpack/util/Reference.java:13)

### `RegistryHelper`

- [ ] `RegistryHelper.getRegistryAccess(0)`: it uses Minecraft's `RegistryAccess` (com/tiviacz/travelersbackpack/util/RegistryHelper.java:11)

### `RenderHelper`

- [x] `RenderHelper.SLOT_HIGHLIGHT_BACK_SPRITE` (com/tiviacz/travelersbackpack/util/RenderHelper.java:15)
- [x] `RenderHelper.SLOT_HIGHLIGHT_FRONT_SPRITE` (com/tiviacz/travelersbackpack/util/RenderHelper.java:16)
- [ ] `RenderHelper.renderScreenTank(7)`: it uses FluidTank, which isn't translated (it extends Fabric's `SingleVariantStorage`) (com/tiviacz/travelersbackpack/util/RenderHelper.java:18)
- [ ] `RenderHelper.renderScreenTank(9)`: it uses FluidVariantWrapper, which isn't translated (it uses Fabric's `FluidVariant`) (com/tiviacz/travelersbackpack/util/RenderHelper.java:22)
- [ ] `RenderHelper.renderSlotHighlightBack(3)`: it calls Minecraft's GuiGraphics.blitSprite (no Horizon helper for it yet) (com/tiviacz/travelersbackpack/util/RenderHelper.java:58)
- [ ] `RenderHelper.renderSlotHighlightFront(3)`: it calls Minecraft's GuiGraphics.blitSprite (no Horizon helper for it yet) (com/tiviacz/travelersbackpack/util/RenderHelper.java:62)

### `Supporters`

- [x] `Supporters.SUPPORTERS_REFERENCE` (com/tiviacz/travelersbackpack/util/Supporters.java:14)
- [x] `Supporters.SUPPORTERS` (com/tiviacz/travelersbackpack/util/Supporters.java:15)
- [ ] `Supporters.fetchSupporters(0)`: a future's callback (it runs on the game thread) (com/tiviacz/travelersbackpack/util/Supporters.java:17)
- [ ] `Supporters.updateSupporters(0)`: a future's callback (it runs on the game thread) (com/tiviacz/travelersbackpack/util/Supporters.java:30)
- [ ] `Supporters.getGistFileAsync(0)`: it calls `HttpRequest` (java.net.http).newBuilder (com/tiviacz/travelersbackpack/util/Supporters.java:41)

### `TextUtils`

- [ ] `TextUtils.getTranslatedSplittedText(2)`: it uses Minecraft's `ChatFormatting` (com/tiviacz/travelersbackpack/util/TextUtils.java:13)
- [x] `TextUtils.getConvertedTime(1)` (com/tiviacz/travelersbackpack/util/TextUtils.java:25)
