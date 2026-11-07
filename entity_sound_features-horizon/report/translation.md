# Java translated to Rust

Translated 4 of 64 methods (1 more is done another way in Horizon) and 5 of 11 static fields to Rust; 0 of 1 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 46 | `ESF.init(0)`: it uses ESF.configHandler, which isn't translated (it uses `TConfigHandler` (traben.tconfig)) |
| Uses other libraries | 6 | `ESF.res(1)`: it calls `ETFUtils2` (traben.entity_texture_features.utils).res |
| Uses Minecraft classes (needs Horizon's game API) | 5 | `ESF.getConfigScreen(1)`: it uses Minecraft's `Screen` |
| Other Java this translator doesn't handle yet | 2 | `ESFVariantSupplier.parseSoundEventVariant(4)`: its body couldn't be read: an anonymous class |
| Done another way in Horizon | 1 | `ESFModMenu.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| traben/entity_sound_features/ESFInit.java:9 | `init` | not translated: it calls ESF.init, which isn't translated |

## Every class


### `ESF`

- [x] `ESF.MOD_ID` (traben/entity_sound_features/ESF.java:20)
- [x] `ESF.LOGGER` (traben/entity_sound_features/ESF.java:21)
- [ ] `ESF.configHandler`: it uses `TConfigHandler` (traben.tconfig) (traben/entity_sound_features/ESF.java:22)
- [ ] `ESF.res(1)`: it calls `ETFUtils2` (traben.entity_texture_features.utils).res (traben/entity_sound_features/ESF.java:24)
- [ ] `ESF.res(2)`: it calls `ETFUtils2` (traben.entity_texture_features.utils).res (traben/entity_sound_features/ESF.java:29)
- [ ] `ESF.config(0)`: it uses `TConfigHandler` (traben.tconfig) (traben/entity_sound_features/ESF.java:34)
- [ ] `ESF.init(0)`: it uses ESF.configHandler, which isn't translated (it uses `TConfigHandler` (traben.tconfig)) (traben/entity_sound_features/ESF.java:42)
- [ ] `ESF.getConfigScreen(1)`: it uses Minecraft's `Screen` (traben/entity_sound_features/ESF.java:57)
- [ ] `ESF.getConfigScreen(2)`: it uses Minecraft's `Screen` (traben/entity_sound_features/ESF.java:61)
- [x] `ESF.log(1)` (traben/entity_sound_features/ESF.java:65)
- [x] `ESF.logError(1)` (traben/entity_sound_features/ESF.java:69)
- [x] `ESF.logWarn(1)` (traben/entity_sound_features/ESF.java:73)

### `ESFConfig`: it extends `TConfig` (traben.tconfig)

- [ ] `ESFConfig.isEntityAllowedToModifySounds(1)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_sound_features/ESFConfig.java:28)
- [ ] `ESFConfig.getGUIOptions(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_sound_features/ESFConfig.java:32)
- [ ] `ESFConfig.getEntitySettings(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_sound_features/ESFConfig.java:67)
- [ ] `ESFConfig.addEntitySettings(2)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_sound_features/ESFConfig.java:92)
- [ ] `ESFConfig.getModIcon(0)`: its class isn't translated (it extends `TConfig` (traben.tconfig)) (traben/entity_sound_features/ESFConfig.java:110)

### `ESFConfig.AnnounceMode`: its constants carry values

- [ ] `ESFConfig.AnnounceMode.all(0)`: its class isn't translated (its constants carry values) (traben/entity_sound_features/ESFConfig.java:127)
- [ ] `ESFConfig.AnnounceMode.AnnounceMode(1)`: its class isn't translated (its constants carry values) (traben/entity_sound_features/ESFConfig.java:131)
- [ ] `ESFConfig.AnnounceMode.toString(0)`: its class isn't translated (its constants carry values) (traben/entity_sound_features/ESFConfig.java:135)

### `ESFConfig.EntitySearchMode`: its constants carry values

- [ ] `ESFConfig.EntitySearchMode.EntitySearchMode(1)`: its class isn't translated (its constants carry values) (traben/entity_sound_features/ESFConfig.java:149)
- [ ] `ESFConfig.EntitySearchMode.toString(0)`: its class isn't translated (its constants carry values) (traben/entity_sound_features/ESFConfig.java:153)

### `ESFInit`: it implements Fabric's `ClientModInitializer`

- [ ] `ESFInit.onInitializeClient(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (traben/entity_sound_features/ESFInit.java:9)

### `ESFModMenu`: it implements `ModMenuApi` (com.terraformersmc.modmenu.api)

- [ ] `ESFModMenu.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (traben/entity_sound_features/ESFModMenu.java:11)

### `ESFSound`: it extends Minecraft's `Sound`

- [ ] `ESFSound.ESFSound(1)`: its class isn't translated (it extends Minecraft's `Sound`) (traben/entity_sound_features/ESFSound.java:10)
- [ ] `ESFSound.ESFSound(2)`: its class isn't translated (it extends Minecraft's `Sound`) (traben/entity_sound_features/ESFSound.java:14)
- [ ] `ESFSound.getPath(0)`: its class isn't translated (it extends Minecraft's `Sound`) (traben/entity_sound_features/ESFSound.java:27)

### `ESFSoundContext`

- [ ] `ESFSoundContext.variantSuppliers`: it uses ESFVariantSupplier, which isn't translated (it uses Minecraft's `WeighedSoundEvents`) (traben/entity_sound_features/ESFSoundContext.java:21)
- [x] `ESFSoundContext.announcedSounds` (traben/entity_sound_features/ESFSoundContext.java:22)
- [x] `ESFSoundContext.knownESFSounds` (traben/entity_sound_features/ESFSoundContext.java:23)
- [ ] `ESFSoundContext.entitySource`: it uses `ETFEntityRenderState` (traben.entity_texture_features.features.state) (traben/entity_sound_features/ESFSoundContext.java:25)
- [ ] `ESFSoundContext.lastRuleMet`: it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache) (traben/entity_sound_features/ESFSoundContext.java:26)
- [ ] `ESFSoundContext.lastSuffix`: it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache) (traben/entity_sound_features/ESFSoundContext.java:27)
- [x] `ESFSoundContext.addKnownESFSound(1)` (traben/entity_sound_features/ESFSoundContext.java:29)
- [ ] `ESFSoundContext.shouldCaptureEntity(1)`: it calls ESF.config, which isn't translated (traben/entity_sound_features/ESFSoundContext.java:33)
- [ ] `ESFSoundContext.setSource(1)`: it uses `ETFEntity` (traben.entity_texture_features.utils) (traben/entity_sound_features/ESFSoundContext.java:37)
- [ ] `ESFSoundContext.preTestEntity(1)`: it uses `ETFEntityRenderState` (traben.entity_texture_features.features.state) (traben/entity_sound_features/ESFSoundContext.java:42)
- [ ] `ESFSoundContext.announceSound(2)`: it uses ESFSoundContext.entitySource, which isn't translated (it uses `ETFEntityRenderState` (traben.entity_texture_features.features.state)) (traben/entity_sound_features/ESFSoundContext.java:48)
- [ ] `ESFSoundContext.announceWithEntity(2)`: it uses ESFSoundContext.entitySource, which isn't translated (it uses `ETFEntityRenderState` (traben.entity_texture_features.features.state)) (traben/entity_sound_features/ESFSoundContext.java:68)
- [ ] `ESFSoundContext.resetContext(0)`: it uses ESFSoundContext.variantSuppliers, which isn't translated (it uses ESFVariantSupplier, which isn't translated (it uses Minecraft's `WeighedSoundEvents`)) (traben/entity_sound_features/ESFSoundContext.java:91)
- [ ] `ESFSoundContext.searchForEntity(4)`: it uses Minecraft's `Entity` (traben/entity_sound_features/ESFSoundContext.java:99)
- [ ] `ESFSoundContext.searchForBlockEntity(4)`: it uses Minecraft's `Level` (traben/entity_sound_features/ESFSoundContext.java:148)

### `ESFVariantSupplier`: it uses Minecraft's `WeighedSoundEvents`

- [ ] `ESFVariantSupplier.SOUND_EVENT_REGISTRATION_TYPE`: its value couldn't be read: an anonymous class (traben/entity_sound_features/ESFVariantSupplier.java:31)
- [ ] `ESFVariantSupplier.ESFVariantSupplier(3)`: its class isn't translated (it uses Minecraft's `WeighedSoundEvents`) (traben/entity_sound_features/ESFVariantSupplier.java:37)
- [ ] `ESFVariantSupplier.getOrNull(1)`: it uses ESFVariantSupplier, which isn't translated (it uses Minecraft's `WeighedSoundEvents`) (traben/entity_sound_features/ESFVariantSupplier.java:58)
- [ ] `ESFVariantSupplier.parseSoundEventVariant(4)`: its body couldn't be read: an anonymous class (traben/entity_sound_features/ESFVariantSupplier.java:130)
- [ ] `ESFVariantSupplier.preTestEntity(1)`: its class isn't translated (it uses Minecraft's `WeighedSoundEvents`) (traben/entity_sound_features/ESFVariantSupplier.java:194)
- [ ] `ESFVariantSupplier.getSoundVariantOrNull(0)`: its class isn't translated (it uses Minecraft's `WeighedSoundEvents`) (traben/entity_sound_features/ESFVariantSupplier.java:198)
- [ ] `ESFVariantSupplier.toString(0)`: its class isn't translated (it uses Minecraft's `WeighedSoundEvents`) (traben/entity_sound_features/ESFVariantSupplier.java:211)

### `ESFMethods`

- [x] `ESFMethods.lastSoundTicks` (traben/entity_sound_features/methods/ESFMethods.java:24)
- [ ] `ESFMethods.register(0)`: unknown static ESFMethods.class (traben/entity_sound_features/methods/ESFMethods.java:26)
- [ ] `ESFMethods.playingSound(1)`: it calls ESF.res, which isn't translated (traben/entity_sound_features/methods/ESFMethods.java:54)
- [ ] `ESFMethods.playsoundStatic(6)`: it uses `EMFEntityRenderState` (traben.entity_model_features.models.animation.state) (traben/entity_sound_features/methods/ESFMethods.java:91)
- [ ] `MixinClientLevel`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_sound_features/methods/ESFMethods.java:25)
- [ ] `MixinResourceReload`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_sound_features/methods/ESFMethods.java:11)
- [ ] `MixinSoundManager$Preparations`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_sound_features/methods/ESFMethods.java:16)
- [ ] `MixinWeighedSoundEvents`: a Mixin class: its handlers are the injections, in the bundle's helpers (traben/entity_sound_features/methods/ESFMethods.java:19)

### `Plugin`: it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)

- [ ] `Plugin.onLoad(1)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_sound_features/mixin/Plugin.java:11)
- [ ] `Plugin.getRefMapperConfig(0)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_sound_features/mixin/Plugin.java:15)
- [ ] `Plugin.shouldApplyMixin(2)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_sound_features/mixin/Plugin.java:19)
- [ ] `Plugin.acceptTargets(2)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_sound_features/mixin/Plugin.java:23)
- [ ] `Plugin.getMixins(0)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_sound_features/mixin/Plugin.java:26)
- [ ] `Plugin.preApply(4)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_sound_features/mixin/Plugin.java:30)
- [ ] `Plugin.postApply(4)`: its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)) (traben/entity_sound_features/mixin/Plugin.java:33)

### `PlayingSoundProperty`: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `PlayingSoundProperty.PlayingSoundProperty(2)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_sound_features/properties/PlayingSoundProperty.java:21)
- [ ] `PlayingSoundProperty.getPropertyOrNull(2)`: it uses PlayingSoundProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_sound_features/properties/PlayingSoundProperty.java:26)
- [ ] `PlayingSoundProperty.testEntityInternal(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_sound_features/properties/PlayingSoundProperty.java:36)
- [ ] `PlayingSoundProperty.getPlayingSounds(0)`: it uses Minecraft's `SoundEngine` (traben/entity_sound_features/properties/PlayingSoundProperty.java:59)
- [ ] `PlayingSoundProperty.getPropertyIds(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_sound_features/properties/PlayingSoundProperty.java:81)
- [ ] `PlayingSoundProperty.shouldForceLowerCaseCheck(0)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_sound_features/properties/PlayingSoundProperty.java:86)
- [ ] `PlayingSoundProperty.getValueFromEntity(1)`: its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_sound_features/properties/PlayingSoundProperty.java:90)

### `SoundRuleIndexProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `SoundRuleIndexProperty.SoundRuleIndexProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_sound_features/properties/SoundRuleIndexProperty.java:11)
- [ ] `SoundRuleIndexProperty.getPropertyOrNull(2)`: it uses SoundRuleIndexProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_sound_features/properties/SoundRuleIndexProperty.java:15)
- [ ] `SoundRuleIndexProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_sound_features/properties/SoundRuleIndexProperty.java:23)
- [ ] `SoundRuleIndexProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_sound_features/properties/SoundRuleIndexProperty.java:28)

### `SoundSuffixProperty`: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)

- [ ] `SoundSuffixProperty.SoundSuffixProperty(2)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_sound_features/properties/SoundSuffixProperty.java:11)
- [ ] `SoundSuffixProperty.getPropertyOrNull(2)`: it uses SoundSuffixProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_sound_features/properties/SoundSuffixProperty.java:15)
- [ ] `SoundSuffixProperty.getPropertyIds(0)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_sound_features/properties/SoundSuffixProperty.java:23)
- [ ] `SoundSuffixProperty.getValueFromEntity(1)`: its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)) (traben/entity_sound_features/properties/SoundSuffixProperty.java:28)
