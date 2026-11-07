# Java translated to Rust

Translated 1 of 269 methods and 15 of 44 static fields to Rust (3 of those call the game, through Horizon's helpers). It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 226 | `FireParticlesEvent.FireParticlesEvent(3)`: its class isn't translated (it extends `Event` (net.neoforged.bus.api)) |
| Other Java this translator doesn't handle yet | 33 | `VisualCategory.isAffectedByWater(0)`: it has no body |
| Uses Minecraft classes (needs Horizon's game API) | 5 | `TextureCache.parse(3)`: it uses Minecraft's `ResourceManager` |
| Calls the game where Horizon has no helper yet | 3 | `EVClient.shouldRender(0)`: it calls Minecraft's LocalPlayer.isSpectator (no Horizon helper for it yet) |
| Uses other libraries | 1 | `SoundMuteHandler.startMuting(1)`: it uses `DecimalCurve` (team.creative.creativecore.common.config.premade.curve) |

## Every class


### `FireParticlesEvent`: it extends `Event` (net.neoforged.bus.api)

- [ ] `FireParticlesEvent.FireParticlesEvent(3)`: its class isn't translated (it extends `Event` (net.neoforged.bus.api)) (team/creative/enhancedvisuals/api/event/FireParticlesEvent.java:13)
- [ ] `FireParticlesEvent.setFireSplashes(1)`: its class isn't translated (it extends `Event` (net.neoforged.bus.api)) (team/creative/enhancedvisuals/api/event/FireParticlesEvent.java:19)
- [ ] `FireParticlesEvent.setDuration(2)`: its class isn't translated (it extends `Event` (net.neoforged.bus.api)) (team/creative/enhancedvisuals/api/event/FireParticlesEvent.java:23)
- [ ] `FireParticlesEvent.getNewFireSplashes(0)`: its class isn't translated (it extends `Event` (net.neoforged.bus.api)) (team/creative/enhancedvisuals/api/event/FireParticlesEvent.java:28)
- [ ] `FireParticlesEvent.getNewFireDurationMin(0)`: its class isn't translated (it extends `Event` (net.neoforged.bus.api)) (team/creative/enhancedvisuals/api/event/FireParticlesEvent.java:32)
- [ ] `FireParticlesEvent.getNewFireDurationMax(0)`: its class isn't translated (it extends `Event` (net.neoforged.bus.api)) (team/creative/enhancedvisuals/api/event/FireParticlesEvent.java:36)

### `SelectEndermanEvent`: it extends `Event` (net.neoforged.bus.api)

- [ ] `SelectEndermanEvent.SelectEndermanEvent(1)`: its class isn't translated (it extends `Event` (net.neoforged.bus.api)) (team/creative/enhancedvisuals/api/event/SelectEndermanEvent.java:10)

### `SplashEvent`: it extends `Event` (net.neoforged.bus.api)

- [ ] `SplashEvent.SplashEvent(1)`: its class isn't translated (it extends `Event` (net.neoforged.bus.api)) (team/creative/enhancedvisuals/api/event/SplashEvent.java:10)

### `VisualExplosionEvent`: it extends `Event` (net.neoforged.bus.api)

- [ ] `VisualExplosionEvent.VisualExplosionEvent(1)`: its class isn't translated (it extends `Event` (net.neoforged.bus.api)) (team/creative/enhancedvisuals/api/event/VisualExplosionEvent.java:11)
- [ ] `VisualExplosionEvent.isBlurDisabled(0)`: its class isn't translated (it extends `Event` (net.neoforged.bus.api)) (team/creative/enhancedvisuals/api/event/VisualExplosionEvent.java:15)
- [ ] `VisualExplosionEvent.setDisableBlur(1)`: its class isn't translated (it extends `Event` (net.neoforged.bus.api)) (team/creative/enhancedvisuals/api/event/VisualExplosionEvent.java:19)
- [ ] `VisualExplosionEvent.setNewDamage(1)`: its class isn't translated (it extends `Event` (net.neoforged.bus.api)) (team/creative/enhancedvisuals/api/event/VisualExplosionEvent.java:23)
- [ ] `VisualExplosionEvent.getNewDamage(0)`: its class isn't translated (it extends `Event` (net.neoforged.bus.api)) (team/creative/enhancedvisuals/api/event/VisualExplosionEvent.java:27)

### `Particle`: it extends `Visual`

- [ ] `Particle.Particle(9)`: its class isn't translated (it extends `Visual`) (team/creative/enhancedvisuals/api/Particle.java:13)
- [ ] `Particle.Particle(8)`: its class isn't translated (it extends `Visual`) (team/creative/enhancedvisuals/api/Particle.java:22)
- [ ] `Particle.getWidth(1)`: its class isn't translated (it extends `Visual`) (team/creative/enhancedvisuals/api/Particle.java:31)
- [ ] `Particle.getHeight(1)`: its class isn't translated (it extends `Visual`) (team/creative/enhancedvisuals/api/Particle.java:36)

### `VisualType`: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)

- [ ] `VisualType.TYPES`: it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/type/VisualType.java:16)
- [ ] `VisualType.types(0)`: it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/type/VisualType.java:27)
- [ ] `VisualType.VisualType(2)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/type/VisualType.java:31)
- [ ] `VisualType.setIgnoreWater(0)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/type/VisualType.java:37)
- [ ] `VisualType.isAffectedByWater(0)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/type/VisualType.java:42)
- [ ] `VisualType.configured(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/type/VisualType.java:46)
- [ ] `VisualType.getColor(0)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/type/VisualType.java:49)
- [ ] `VisualType.canRotate(0)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/type/VisualType.java:53)
- [ ] `VisualType.isVisible(2)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/type/VisualType.java:57)
- [ ] `VisualType.scaleVariants(0)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/type/VisualType.java:61)
- [ ] `VisualType.randomScale(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/type/VisualType.java:65)

### `VisualTypeAnimatedTexture`


### `VisualTypeBlur`: it extends `VisualTypeShader`

- [x] `VisualTypeBlur.BLUR_SHADER` (team/creative/enhancedvisuals/api/type/VisualTypeBlur.java:6)
- [ ] `VisualTypeBlur.VisualTypeBlur(1)`: its class isn't translated (it extends `VisualTypeShader`) (team/creative/enhancedvisuals/api/type/VisualTypeBlur.java:8)

### `VisualTypeFocus`: it extends `VisualTypeShader`

- [x] `VisualTypeFocus.IDENTIFIER` (team/creative/enhancedvisuals/api/type/VisualTypeFocus.java:6)
- [ ] `VisualTypeFocus.VisualTypeFocus(1)`: its class isn't translated (it extends `VisualTypeShader`) (team/creative/enhancedvisuals/api/type/VisualTypeFocus.java:8)

### `VisualTypeOverlay`: it extends `VisualTypeTexture`

- [ ] `VisualTypeOverlay.VisualTypeOverlay(2)`: its class isn't translated (it extends `VisualTypeTexture`) (team/creative/enhancedvisuals/api/type/VisualTypeOverlay.java:6)
- [ ] `VisualTypeOverlay.VisualTypeOverlay(1)`: its class isn't translated (it extends `VisualTypeTexture`) (team/creative/enhancedvisuals/api/type/VisualTypeOverlay.java:10)

### `VisualTypeParticle`: it extends `VisualTypeTexture`

- [ ] `VisualTypeParticle.VisualTypeParticle(3)`: its class isn't translated (it extends `VisualTypeTexture`) (team/creative/enhancedvisuals/api/type/VisualTypeParticle.java:12)
- [ ] `VisualTypeParticle.VisualTypeParticle(1)`: its class isn't translated (it extends `VisualTypeTexture`) (team/creative/enhancedvisuals/api/type/VisualTypeParticle.java:17)
- [ ] `VisualTypeParticle.scaleVariants(0)`: its class isn't translated (it extends `VisualTypeTexture`) (team/creative/enhancedvisuals/api/type/VisualTypeParticle.java:21)
- [ ] `VisualTypeParticle.randomScale(1)`: its class isn't translated (it extends `VisualTypeTexture`) (team/creative/enhancedvisuals/api/type/VisualTypeParticle.java:26)

### `VisualTypeParticleColored`: it extends `VisualTypeParticle`

- [ ] `VisualTypeParticleColored.VisualTypeParticleColored(4)`: its class isn't translated (it extends `VisualTypeParticle`) (team/creative/enhancedvisuals/api/type/VisualTypeParticleColored.java:24)
- [ ] `VisualTypeParticleColored.VisualTypeParticleColored(2)`: its class isn't translated (it extends `VisualTypeParticle`) (team/creative/enhancedvisuals/api/type/VisualTypeParticleColored.java:33)
- [ ] `VisualTypeParticleColored.configured(1)`: its class isn't translated (it extends `VisualTypeParticle`) (team/creative/enhancedvisuals/api/type/VisualTypeParticleColored.java:37)
- [ ] `VisualTypeParticleColored.getColor(0)`: its class isn't translated (it extends `VisualTypeParticle`) (team/creative/enhancedvisuals/api/type/VisualTypeParticleColored.java:42)

### `VisualTypeSaturation`: it extends `VisualTypeShader`

- [ ] `VisualTypeSaturation.VisualTypeSaturation(1)`: its class isn't translated (it extends `VisualTypeShader`) (team/creative/enhancedvisuals/api/type/VisualTypeSaturation.java:8)
- [ ] `VisualTypeSaturation.isVisible(2)`: its class isn't translated (it extends `VisualTypeShader`) (team/creative/enhancedvisuals/api/type/VisualTypeSaturation.java:12)

### `VisualTypeShader`: it extends `VisualType`

- [ ] `VisualTypeShader.VisualTypeShader(2)`: its class isn't translated (it extends `VisualType`) (team/creative/enhancedvisuals/api/type/VisualTypeShader.java:9)

### `VisualTypeTexture`: it extends `VisualType`

- [ ] `VisualTypeTexture.VisualTypeTexture(4)`: its class isn't translated (it extends `VisualType`) (team/creative/enhancedvisuals/api/type/VisualTypeTexture.java:11)
- [ ] `VisualTypeTexture.VisualTypeTexture(3)`: its class isn't translated (it extends `VisualType`) (team/creative/enhancedvisuals/api/type/VisualTypeTexture.java:17)

### `Visual`: it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))

- [ ] `Visual.Visual(4)`: its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/api/Visual.java:18)
- [ ] `Visual.Visual(3)`: its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/api/Visual.java:27)
- [ ] `Visual.setOpacityInternal(1)`: its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/api/Visual.java:36)
- [ ] `Visual.getOpacityInternal(0)`: its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/api/Visual.java:40)
- [ ] `Visual.getOpacity(0)`: its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/api/Visual.java:44)
- [ ] `Visual.displayed(0)`: its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/api/Visual.java:48)
- [ ] `Visual.addToDisplay(0)`: its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/api/Visual.java:52)
- [ ] `Visual.removeFromDisplay(0)`: its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/api/Visual.java:56)
- [ ] `Visual.getCategory(0)`: its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/api/Visual.java:60)
- [ ] `Visual.isVisible(0)`: its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/api/Visual.java:64)
- [ ] `Visual.tick(0)`: its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/api/Visual.java:68)
- [ ] `Visual.getWidth(1)`: its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/api/Visual.java:77)
- [ ] `Visual.getHeight(1)`: its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/api/Visual.java:81)
- [ ] `Visual.isAffectedByWater(0)`: its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/api/Visual.java:85)

### `VisualCategory`: its constants carry values

- [ ] `VisualCategory.isAffectedByWater(0)`: it has no body (team/creative/enhancedvisuals/api/VisualCategory.java:23)

### `VisualHandler`: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)

- [ ] `VisualHandler.configured(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:28)
- [ ] `VisualHandler.tick(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:31)
- [ ] `VisualHandler.isEnabled(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:34)
- [ ] `VisualHandler.manager(0)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:38)
- [ ] `VisualHandler.random(0)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:42)
- [ ] `VisualHandler.playSound(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:47)
- [ ] `VisualHandler.playSound(2)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:52)
- [ ] `VisualHandler.playSound(2)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:57)
- [ ] `VisualHandler.playSound(3)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:62)
- [ ] `VisualHandler.playSoundFadeOut(3)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:67)
- [ ] `VisualHandler.visuals(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:72)
- [ ] `VisualHandler.clearEverything(0)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:77)
- [ ] `VisualHandler.add(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:82)
- [ ] `VisualHandler.remove(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:87)
- [ ] `VisualHandler.playTicking(3)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:92)
- [ ] `VisualHandler.addVisualFadeOut(3)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:97)
- [ ] `VisualHandler.addVisualFadeOut(3)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:102)
- [ ] `VisualHandler.addVisualFadeOut(3)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:107)
- [ ] `VisualHandler.addParticlesFadeOut(5)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:112)
- [ ] `VisualHandler.addParticlesFadeOut(6)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:117)
- [ ] `VisualHandler.addParticlesFadeOut(5)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:122)
- [ ] `VisualHandler.addParticlesFadeOut(6)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:127)
- [ ] `VisualHandler.addParticle(4)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:132)
- [ ] `VisualHandler.generateOffset(3)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/api/VisualHandler.java:137)

### `EVClient`

- [ ] `EVClient.CLIENT_TYPES`: it uses `PairList` (team.creative.creativecore.common.util.type.list) (team/creative/enhancedvisuals/client/EVClient.java:27)
- [ ] `EVClient.get(1)`: it uses VisualTypeClient, which isn't translated (it's generic) (team/creative/enhancedvisuals/client/EVClient.java:29)
- [ ] `EVClient.register(2)`: it's generic (team/creative/enhancedvisuals/client/EVClient.java:57)
- [ ] `EVClient.preInit(0)`: its body couldn't be read: an anonymous class (team/creative/enhancedvisuals/client/EVClient.java:61)
- [ ] `EVClient.init(0)`: it calls EVClient.register, which isn't translated (team/creative/enhancedvisuals/client/EVClient.java:74)
- [ ] `EVClient.shouldRender(0)`: it calls Minecraft's LocalPlayer.isSpectator (no Horizon helper for it yet) (team/creative/enhancedvisuals/client/EVClient.java:92)
- [ ] `EVClient.shouldTick(0)`: it calls Minecraft's Minecraft.isPaused (no Horizon helper for it yet) (team/creative/enhancedvisuals/client/EVClient.java:97)

### `EVManagerClient`: it implements `EVManager` (team.creative.enhancedvisuals)

- [ ] `EVManagerClient.respawn(0)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:38)
- [ ] `EVManagerClient.clientTick(0)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:42)
- [ ] `EVManagerClient.random(0)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:51)
- [ ] `EVManagerClient.onTick(1)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:56)
- [ ] `EVManagerClient.playSound(1)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:93)
- [ ] `EVManagerClient.playSound(2)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:98)
- [ ] `EVManagerClient.playSound(2)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:103)
- [ ] `EVManagerClient.playSound(3)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:108)
- [ ] `EVManagerClient.playSoundFadeOut(3)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:119)
- [ ] `EVManagerClient.visuals(1)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:126)
- [ ] `EVManagerClient.clearEverything(0)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:131)
- [ ] `EVManagerClient.add(1)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:147)
- [ ] `EVManagerClient.remove(1)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:155)
- [ ] `EVManagerClient.playTicking(3)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:165)
- [ ] `EVManagerClient.addVisualFadeOut(3)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:178)
- [ ] `EVManagerClient.addVisualFadeOut(3)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:183)
- [ ] `EVManagerClient.addVisualFadeOut(3)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:188)
- [ ] `EVManagerClient.addParticlesFadeOut(5)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:196)
- [ ] `EVManagerClient.addParticlesFadeOut(6)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:201)
- [ ] `EVManagerClient.addParticlesFadeOut(5)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:206)
- [ ] `EVManagerClient.addParticlesFadeOut(6)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:211)
- [ ] `EVManagerClient.addParticle(4)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:250)
- [ ] `EVManagerClient.generateOffset(3)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/client/EVManagerClient.java:287)

### `GameRendererExtender`: it's an interface

- [ ] `GameRendererExtender.getResourcePool(0)`: it has no body (team/creative/enhancedvisuals/client/mc/GameRendererExtender.java:6)

### `EVRenderer`

- [x] `EVRenderer.lastRenderedMessage` (team/creative/enhancedvisuals/client/render/EVRenderer.java:20)
- [x] `EVRenderer.framebufferWidth` (team/creative/enhancedvisuals/client/render/EVRenderer.java:21)
- [x] `EVRenderer.framebufferHeight` (team/creative/enhancedvisuals/client/render/EVRenderer.java:22)
- [x] `EVRenderer.reloadResources` (team/creative/enhancedvisuals/client/render/EVRenderer.java:23)
- [x] `EVRenderer.init(0)` (team/creative/enhancedvisuals/client/render/EVRenderer.java:25)
- [ ] `EVRenderer.renderShaders(1)`: it calls Minecraft's Minecraft.getWindow (no Horizon helper for it yet) (team/creative/enhancedvisuals/client/render/EVRenderer.java:28)
- [ ] `EVRenderer.render(1)`: it uses `Object` (team/creative/enhancedvisuals/client/render/EVRenderer.java:36)
- [ ] `EVRenderer.render(5)`: it uses Visual, which isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/client/render/EVRenderer.java:80)
- [ ] `EVRenderer.renderVisuals(5)`: it uses Visual, which isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))) (team/creative/enhancedvisuals/client/render/EVRenderer.java:94)

### `TextureCache`

- [ ] `TextureCache.getFirst(0)`: it has no body (team/creative/enhancedvisuals/client/render/TextureCache.java:16)
- [ ] `TextureCache.getResource(0)`: it has no body (team/creative/enhancedvisuals/client/render/TextureCache.java:18)
- [ ] `TextureCache.parse(3)`: it uses Minecraft's `ResourceManager` (team/creative/enhancedvisuals/client/render/TextureCache.java:20)

### `TextureCacheAnimation`: it extends `TextureCache`

- [ ] `TextureCacheAnimation.TextureCacheAnimation(2)`: its class isn't translated (it extends `TextureCache`) (team/creative/enhancedvisuals/client/render/TextureCacheAnimation.java:9)
- [ ] `TextureCacheAnimation.getFirst(0)`: its class isn't translated (it extends `TextureCache`) (team/creative/enhancedvisuals/client/render/TextureCacheAnimation.java:14)
- [ ] `TextureCacheAnimation.getResource(0)`: its class isn't translated (it extends `TextureCache`) (team/creative/enhancedvisuals/client/render/TextureCacheAnimation.java:19)

### `TextureCacheSimple`: it extends `TextureCache`

- [ ] `TextureCacheSimple.TextureCacheSimple(1)`: its class isn't translated (it extends `TextureCache`) (team/creative/enhancedvisuals/client/render/TextureCacheSimple.java:8)
- [ ] `TextureCacheSimple.getResource(0)`: its class isn't translated (it extends `TextureCache`) (team/creative/enhancedvisuals/client/render/TextureCacheSimple.java:12)
- [ ] `TextureCacheSimple.getFirst(0)`: its class isn't translated (it extends `TextureCache`) (team/creative/enhancedvisuals/client/render/TextureCacheSimple.java:17)

### `PositionedSound`: it extends Minecraft's `AbstractSoundInstance`

- [ ] `PositionedSound.PositionedSound(4)`: its class isn't translated (it extends Minecraft's `AbstractSoundInstance`) (team/creative/enhancedvisuals/client/sound/PositionedSound.java:11)
- [ ] `PositionedSound.PositionedSound(5)`: its class isn't translated (it extends Minecraft's `AbstractSoundInstance`) (team/creative/enhancedvisuals/client/sound/PositionedSound.java:18)

### `SoundMuteHandler`

- [x] `SoundMuteHandler.isMuting` (team/creative/enhancedvisuals/client/sound/SoundMuteHandler.java:16)
- [ ] `SoundMuteHandler.engine`: it uses Minecraft's `SoundEngine` (team/creative/enhancedvisuals/client/sound/SoundMuteHandler.java:17)
- [ ] `SoundMuteHandler.manager`: it uses Minecraft's `SoundManager` (team/creative/enhancedvisuals/client/sound/SoundMuteHandler.java:18)
- [ ] `SoundMuteHandler.muteGraph`: it uses `DecimalCurve` (team.creative.creativecore.common.config.premade.curve) (team/creative/enhancedvisuals/client/sound/SoundMuteHandler.java:19)
- [x] `SoundMuteHandler.timeTick` (team/creative/enhancedvisuals/client/sound/SoundMuteHandler.java:20)
- [ ] `SoundMuteHandler.tick(0)`: it uses SoundMuteHandler.muteGraph, which isn't translated (it uses `DecimalCurve` (team.creative.creativecore.common.config.premade.curve)) (team/creative/enhancedvisuals/client/sound/SoundMuteHandler.java:22)
- [ ] `SoundMuteHandler.getClampedVolume(1)`: it uses SoundMuteHandler.muteGraph, which isn't translated (it uses `DecimalCurve` (team.creative.creativecore.common.config.premade.curve)) (team/creative/enhancedvisuals/client/sound/SoundMuteHandler.java:34)
- [ ] `SoundMuteHandler.getSounds(0)`: it uses Minecraft's `SoundInstance` (team/creative/enhancedvisuals/client/sound/SoundMuteHandler.java:38)
- [ ] `SoundMuteHandler.setMuteVolume(1)`: it calls SoundMuteHandler.getSounds, which isn't translated (team/creative/enhancedvisuals/client/sound/SoundMuteHandler.java:42)
- [ ] `SoundMuteHandler.startMuting(1)`: it uses `DecimalCurve` (team.creative.creativecore.common.config.premade.curve) (team/creative/enhancedvisuals/client/sound/SoundMuteHandler.java:56)
- [ ] `SoundMuteHandler.endMuting(0)`: it calls SoundMuteHandler.setMuteVolume, which isn't translated (team/creative/enhancedvisuals/client/sound/SoundMuteHandler.java:78)

### `TickedSound`: it extends `PositionedSound`

- [ ] `TickedSound.TickedSound(4)`: its class isn't translated (it extends `PositionedSound`) (team/creative/enhancedvisuals/client/sound/TickedSound.java:13)
- [ ] `TickedSound.TickedSound(5)`: its class isn't translated (it extends `PositionedSound`) (team/creative/enhancedvisuals/client/sound/TickedSound.java:19)
- [ ] `TickedSound.isStopped(0)`: its class isn't translated (it extends `PositionedSound`) (team/creative/enhancedvisuals/client/sound/TickedSound.java:25)
- [ ] `TickedSound.stop(0)`: its class isn't translated (it extends `PositionedSound`) (team/creative/enhancedvisuals/client/sound/TickedSound.java:29)
- [ ] `TickedSound.tick(0)`: its class isn't translated (it extends `PositionedSound`) (team/creative/enhancedvisuals/client/sound/TickedSound.java:34)

### `VisualTypeBlurClient`: it extends `VisualTypeShaderClient`

- [ ] `VisualTypeBlurClient.UNIFORM_SIZE`: it creates Minecraft's `Std140SizeCalculator` (team/creative/enhancedvisuals/client/type/VisualTypeBlurClient.java:16)
- [ ] `VisualTypeBlurClient.VisualTypeBlurClient(1)`: its class isn't translated (it extends `VisualTypeShaderClient`) (team/creative/enhancedvisuals/client/type/VisualTypeBlurClient.java:18)
- [ ] `VisualTypeBlurClient.changeProperties(1)`: its class isn't translated (it extends `VisualTypeShaderClient`) (team/creative/enhancedvisuals/client/type/VisualTypeBlurClient.java:22)

### `VisualTypeClient`: it's generic

- [ ] `VisualTypeClient.VisualTypeClient(1)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeClient.java:13)
- [ ] `VisualTypeClient.loadResources(1)`: it has no body (team/creative/enhancedvisuals/client/type/VisualTypeClient.java:17)
- [ ] `VisualTypeClient.render(6)`: it has no body (team/creative/enhancedvisuals/client/type/VisualTypeClient.java:19)
- [ ] `VisualTypeClient.getVariantAmount(0)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeClient.java:21)
- [ ] `VisualTypeClient.resize(1)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeClient.java:25)
- [ ] `VisualTypeClient.getWidth(2)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeClient.java:28)
- [ ] `VisualTypeClient.getHeight(2)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeClient.java:32)

### `VisualTypeFocusClient`: it extends `VisualTypeShaderClient`

- [ ] `VisualTypeFocusClient.UNIFORM_SIZE`: it creates Minecraft's `Std140SizeCalculator` (team/creative/enhancedvisuals/client/type/VisualTypeFocusClient.java:16)
- [ ] `VisualTypeFocusClient.VisualTypeFocusClient(1)`: its class isn't translated (it extends `VisualTypeShaderClient`) (team/creative/enhancedvisuals/client/type/VisualTypeFocusClient.java:18)
- [ ] `VisualTypeFocusClient.changeProperties(1)`: its class isn't translated (it extends `VisualTypeShaderClient`) (team/creative/enhancedvisuals/client/type/VisualTypeFocusClient.java:22)

### `VisualTypeSaturationClient`: it extends `VisualTypeShaderClient`

- [ ] `VisualTypeSaturationClient.UNIFORM_SIZE`: it creates Minecraft's `Std140SizeCalculator` (team/creative/enhancedvisuals/client/type/VisualTypeSaturationClient.java:16)
- [ ] `VisualTypeSaturationClient.VisualTypeSaturationClient(1)`: its class isn't translated (it extends `VisualTypeShaderClient`) (team/creative/enhancedvisuals/client/type/VisualTypeSaturationClient.java:18)
- [ ] `VisualTypeSaturationClient.changeProperties(1)`: its class isn't translated (it extends `VisualTypeShaderClient`) (team/creative/enhancedvisuals/client/type/VisualTypeSaturationClient.java:22)

### `VisualTypeShaderClient`: it's generic

- [ ] `VisualTypeShaderClient.VisualTypeShaderClient(1)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeShaderClient.java:17)
- [ ] `VisualTypeShaderClient.loadResources(1)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeShaderClient.java:21)
- [ ] `VisualTypeShaderClient.getVariantAmount(0)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeShaderClient.java:25)
- [ ] `VisualTypeShaderClient.resize(1)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeShaderClient.java:30)
- [ ] `VisualTypeShaderClient.render(6)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeShaderClient.java:34)
- [ ] `VisualTypeShaderClient.changeProperties(1)`: it has no body (team/creative/enhancedvisuals/client/type/VisualTypeShaderClient.java:47)

### `VisualTypeTextureClient`: it's generic

- [x] `VisualTypeTextureClient.DEFAULT_PARTICLE_SIZE` (team/creative/enhancedvisuals/client/type/VisualTypeTextureClient.java:24)
- [ ] `VisualTypeTextureClient.VisualTypeTextureClient(1)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeTextureClient.java:29)
- [ ] `VisualTypeTextureClient.loadResources(1)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeTextureClient.java:33)
- [ ] `VisualTypeTextureClient.getVariantAmount(0)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeTextureClient.java:71)
- [ ] `VisualTypeTextureClient.getResource(1)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeTextureClient.java:76)
- [ ] `VisualTypeTextureClient.render(6)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeTextureClient.java:85)
- [ ] `VisualTypeTextureClient.getWidth(2)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeTextureClient.java:97)
- [ ] `VisualTypeTextureClient.getHeight(2)`: its class isn't translated (it's generic) (team/creative/enhancedvisuals/client/type/VisualTypeTextureClient.java:102)

### `ColdSweatAddon`

- [ ] `ColdSweatAddon.temperature`: it uses TemperatureHandler, which isn't translated (another class has the same name, in a different package) (team/creative/enhancedvisuals/common/addon/coldsweat/ColdSweatAddon.java:7)
- [ ] `ColdSweatAddon.load(0)`: it calls VisualRegistry.registerHandler, which isn't translated (team/creative/enhancedvisuals/common/addon/coldsweat/ColdSweatAddon.java:9)

### `TemperatureHandler`: another class has the same name, in a different package

- [ ] `TemperatureHandler.getTemperature(1)`: its class isn't translated (another class has the same name, in a different package) (team/creative/enhancedvisuals/common/addon/coldsweat/TemperatureHandler.java:43)
- [ ] `TemperatureHandler.tick(1)`: its class isn't translated (another class has the same name, in a different package) (team/creative/enhancedvisuals/common/addon/coldsweat/TemperatureHandler.java:47)

### `SurviveAddon`

- [ ] `SurviveAddon.thirst`: it uses ThirstHandler, which isn't translated (another class has the same name, in a different package) (team/creative/enhancedvisuals/common/addon/survive/SurviveAddon.java:7)
- [ ] `SurviveAddon.temperature`: it uses TemperatureHandler, which isn't translated (another class has the same name, in a different package) (team/creative/enhancedvisuals/common/addon/survive/SurviveAddon.java:8)
- [ ] `SurviveAddon.load(0)`: it calls VisualRegistry.registerHandler, which isn't translated (team/creative/enhancedvisuals/common/addon/survive/SurviveAddon.java:10)

### `TemperatureHandler`: another class has the same name, in a different package

- [ ] `TemperatureHandler.getTemperature(1)`: its class isn't translated (another class has the same name, in a different package) (team/creative/enhancedvisuals/common/addon/survive/TemperatureHandler.java:42)
- [ ] `TemperatureHandler.tick(1)`: its class isn't translated (another class has the same name, in a different package) (team/creative/enhancedvisuals/common/addon/survive/TemperatureHandler.java:46)

### `ThirstHandler`: another class has the same name, in a different package

- [ ] `ThirstHandler.getThirst(1)`: its class isn't translated (another class has the same name, in a different package) (team/creative/enhancedvisuals/common/addon/survive/ThirstHandler.java:26)
- [ ] `ThirstHandler.tick(1)`: its class isn't translated (another class has the same name, in a different package) (team/creative/enhancedvisuals/common/addon/survive/ThirstHandler.java:30)

### `TANAddon`

- [ ] `TANAddon.thirst`: it uses ThirstHandler, which isn't translated (another class has the same name, in a different package) (team/creative/enhancedvisuals/common/addon/toughasnails/TANAddon.java:7)
- [ ] `TANAddon.temperature`: it uses TemperatureHandler, which isn't translated (another class has the same name, in a different package) (team/creative/enhancedvisuals/common/addon/toughasnails/TANAddon.java:8)
- [ ] `TANAddon.load(0)`: it calls VisualRegistry.registerHandler, which isn't translated (team/creative/enhancedvisuals/common/addon/toughasnails/TANAddon.java:10)

### `TemperatureHandler`: another class has the same name, in a different package

- [ ] `TemperatureHandler.tick(1)`: its class isn't translated (another class has the same name, in a different package) (team/creative/enhancedvisuals/common/addon/toughasnails/TemperatureHandler.java:33)

### `ThirstHandler`: another class has the same name, in a different package

- [ ] `ThirstHandler.tick(1)`: its class isn't translated (another class has the same name, in a different package) (team/creative/enhancedvisuals/common/addon/toughasnails/ThirstHandler.java:26)

### `DeathMessages`: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)

- [ ] `DeathMessages.pickRandomDeathMessage(0)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/common/death/DeathMessages.java:27)
- [ ] `DeathMessages.configured(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/common/death/DeathMessages.java:35)

### `EVEvents`

- [ ] `EVEvents.explosion(2)`: it uses Minecraft's `Explosion` (team/creative/enhancedvisuals/common/event/EVEvents.java:21)
- [ ] `EVEvents.impact(1)`: it uses Minecraft's `Projectile` (team/creative/enhancedvisuals/common/event/EVEvents.java:37)
- [ ] `EVEvents.damage(3)`: it uses Minecraft's `Player` (team/creative/enhancedvisuals/common/event/EVEvents.java:54)
- [ ] `EVEvents.areEyesInWater(1)`: it uses EntityAccessor, which isn't translated (a Mixin class: its handlers are the injections, in the bundle's helpers) (team/creative/enhancedvisuals/common/event/EVEvents.java:64)

### `DamageHandler`: it extends `VisualHandler` (team.creative.enhancedvisuals.api)

- [x] `DamageHandler.sharpList` in game_side.rs (team/creative/enhancedvisuals/common/handler/DamageHandler.java:35)
- [x] `DamageHandler.bluntList` in game_side.rs (team/creative/enhancedvisuals/common/handler/DamageHandler.java:36)
- [x] `DamageHandler.pierceList` in game_side.rs (team/creative/enhancedvisuals/common/handler/DamageHandler.java:37)
- [ ] `DamageHandler.playerDamaged(3)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/DamageHandler.java:117)
- [ ] `DamageHandler.createVisualFromDamageAndDistance(4)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/DamageHandler.java:178)
- [ ] `DamageHandler.isSharp(1)`: List.contains on objects (team/creative/enhancedvisuals/common/handler/DamageHandler.java:193)
- [ ] `DamageHandler.isBlunt(1)`: List.contains on objects (team/creative/enhancedvisuals/common/handler/DamageHandler.java:197)
- [ ] `DamageHandler.isPierce(1)`: List.contains on objects (team/creative/enhancedvisuals/common/handler/DamageHandler.java:201)

### `ExplosionHandler`: it extends `VisualHandler` (team.creative.enhancedvisuals.api)

- [ ] `ExplosionHandler.isMace(2)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/ExplosionHandler.java:58)
- [ ] `ExplosionHandler.onExploded(6)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/ExplosionHandler.java:66)

### `HealthHandler`: it extends `VisualHandler` (team.creative.enhancedvisuals.api)

- [ ] `HealthHandler.HealthHandler(0)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/HealthHandler.java:20)
- [ ] `HealthHandler.tick(1)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/HealthHandler.java:24)

### `HeartbeatHandler`: it extends `VisualHandler` (team.creative.enhancedvisuals.api)

- [ ] `HeartbeatHandler.tick(1)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/HeartbeatHandler.java:42)
- [ ] `HeartbeatHandler.resetBufferTicks(1)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/HeartbeatHandler.java:67)
- [ ] `HeartbeatHandler.getIntensity(1)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/HeartbeatHandler.java:72)
- [ ] `HeartbeatHandler.shouldHeartbeatTrigger(1)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/HeartbeatHandler.java:78)

### `PotionHandler`: it extends `VisualHandler` (team.creative.enhancedvisuals.api)

- [ ] `PotionHandler.impact(2)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/PotionHandler.java:21)

### `RainHandler`: it extends `VisualHandler` (team.creative.enhancedvisuals.api)

- [ ] `RainHandler.tick(1)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/RainHandler.java:35)
- [ ] `RainHandler.updateDelay(1)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/RainHandler.java:80)
- [ ] `RainHandler.configured(1)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/RainHandler.java:87)

### `SandSplatHandler`: it extends `VisualHandler` (team.creative.enhancedvisuals.api)

- [ ] `SandSplatHandler.tick(1)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/SandSplatHandler.java:22)
- [ ] `SandSplatHandler.isOnSand(1)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/SandSplatHandler.java:34)

### `SaturationHandler`: it extends `VisualHandler` (team.creative.enhancedvisuals.api)

- [ ] `SaturationHandler.tick(1)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/SaturationHandler.java:21)

### `SlenderHandler`: it extends `VisualHandler` (team.creative.enhancedvisuals.api)

- [ ] `SlenderHandler.loadMutantEnderman(0)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/SlenderHandler.java:30)
- [ ] `SlenderHandler.tick(1)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/SlenderHandler.java:38)

### `SplashHandler`: it extends `VisualHandler` (team.creative.enhancedvisuals.api)

- [ ] `SplashHandler.tick(1)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/SplashHandler.java:25)

### `UnderwaterHandler`: it extends `VisualHandler` (team.creative.enhancedvisuals.api)

- [ ] `UnderwaterHandler.UnderwaterHandler(0)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/UnderwaterHandler.java:21)
- [ ] `UnderwaterHandler.tick(1)`: its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/UnderwaterHandler.java:25)

### `VisualHandlers`

- [ ] `VisualHandlers.EXPLOSION`: it uses ExplosionHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/VisualHandlers.java:7)
- [ ] `VisualHandlers.POTION`: it uses PotionHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/VisualHandlers.java:8)
- [ ] `VisualHandlers.SAND`: it uses SandSplatHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/VisualHandlers.java:9)
- [ ] `VisualHandlers.SPLASH`: it uses SplashHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/VisualHandlers.java:10)
- [ ] `VisualHandlers.DAMAGE`: it uses DamageHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/VisualHandlers.java:11)
- [ ] `VisualHandlers.SLENDER`: it uses SlenderHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/VisualHandlers.java:12)
- [ ] `VisualHandlers.SATURATION`: it uses SaturationHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/VisualHandlers.java:13)
- [ ] `VisualHandlers.HEARTBEAT`: it uses HeartbeatHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/VisualHandlers.java:14)
- [ ] `VisualHandlers.UNDERWATER`: it uses UnderwaterHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/VisualHandlers.java:15)
- [ ] `VisualHandlers.RAIN`: it uses RainHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/VisualHandlers.java:16)
- [ ] `VisualHandlers.HEALTH`: it uses HealthHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)) (team/creative/enhancedvisuals/common/handler/VisualHandlers.java:17)
- [ ] `VisualHandlers.init(0)`: it calls VisualRegistry.registerHandler, which isn't translated (team/creative/enhancedvisuals/common/handler/VisualHandlers.java:19)

### `DamagePacket`: it extends `CreativePacket` (team.creative.creativecore.common.network)

- [ ] `DamagePacket.DamagePacket(3)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/enhancedvisuals/common/packet/DamagePacket.java:22)
- [ ] `DamagePacket.DamagePacket(0)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/enhancedvisuals/common/packet/DamagePacket.java:30)
- [ ] `DamagePacket.getSource(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/enhancedvisuals/common/packet/DamagePacket.java:33)
- [ ] `DamagePacket.executeClient(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/enhancedvisuals/common/packet/DamagePacket.java:39)
- [ ] `DamagePacket.executeServer(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/enhancedvisuals/common/packet/DamagePacket.java:45)

### `ExplosionPacket`: it extends `CreativePacket` (team.creative.creativecore.common.network)

- [ ] `ExplosionPacket.ExplosionPacket(5)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/enhancedvisuals/common/packet/ExplosionPacket.java:19)
- [ ] `ExplosionPacket.ExplosionPacket(0)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/enhancedvisuals/common/packet/ExplosionPacket.java:27)
- [ ] `ExplosionPacket.executeClient(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/enhancedvisuals/common/packet/ExplosionPacket.java:30)
- [ ] `ExplosionPacket.executeServer(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/enhancedvisuals/common/packet/ExplosionPacket.java:36)

### `PotionPacket`: it extends `CreativePacket` (team.creative.creativecore.common.network)

- [ ] `PotionPacket.PotionPacket(0)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/enhancedvisuals/common/packet/PotionPacket.java:13)
- [ ] `PotionPacket.PotionPacket(2)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/enhancedvisuals/common/packet/PotionPacket.java:16)
- [ ] `PotionPacket.executeClient(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/enhancedvisuals/common/packet/PotionPacket.java:21)
- [ ] `PotionPacket.executeServer(1)`: its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)) (team/creative/enhancedvisuals/common/packet/PotionPacket.java:27)

### `VisualRegistry`

- [ ] `VisualRegistry.handlers`: it uses VisualHandler, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/common/visual/VisualRegistry.java:11)
- [ ] `VisualRegistry.registerHandler(2)`: it uses VisualHandler, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/common/visual/VisualRegistry.java:13)
- [ ] `VisualRegistry.handlers(0)`: it uses VisualHandler, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/common/visual/VisualRegistry.java:17)
- [ ] `VisualRegistry.entrySet(0)`: it uses VisualHandler, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/common/visual/VisualRegistry.java:21)

### `EnhancedVisuals`: it implements `CommonLoader` (team.creative.creativecore.common)

- [x] `EnhancedVisuals.MODID` (team/creative/enhancedvisuals/EnhancedVisuals.java:31)
- [x] `EnhancedVisuals.LOGGER` (team/creative/enhancedvisuals/EnhancedVisuals.java:32)
- [ ] `EnhancedVisuals.NETWORK`: it uses `CreativeNetwork` (team.creative.creativecore.common.network) (team/creative/enhancedvisuals/EnhancedVisuals.java:33)
- [x] `EnhancedVisuals.EVENTS` (team/creative/enhancedvisuals/EnhancedVisuals.java:34)
- [ ] `EnhancedVisuals.MESSAGES`: it uses DeathMessages, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/EnhancedVisuals.java:35)
- [ ] `EnhancedVisuals.CONFIG`: it uses EnhancedVisualsConfig, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/EnhancedVisuals.java:36)
- [ ] `EnhancedVisuals.MANAGER`: it uses EVManager, which isn't translated (it's an interface) (team/creative/enhancedvisuals/EnhancedVisuals.java:37)
- [ ] `EnhancedVisuals.EnhancedVisuals(0)`: its class isn't translated (it implements `CommonLoader` (team.creative.creativecore.common)) (team/creative/enhancedvisuals/EnhancedVisuals.java:39)
- [ ] `EnhancedVisuals.onInitializeClient(0)`: its class isn't translated (it implements `CommonLoader` (team.creative.creativecore.common)) (team/creative/enhancedvisuals/EnhancedVisuals.java:48)
- [ ] `EnhancedVisuals.onInitialize(0)`: its class isn't translated (it implements `CommonLoader` (team.creative.creativecore.common)) (team/creative/enhancedvisuals/EnhancedVisuals.java:52)

### `EnhancedVisualsConfig`: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)

- [ ] `EnhancedVisualsConfig.configured(1)`: its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)) (team/creative/enhancedvisuals/EnhancedVisualsConfig.java:17)

### `EVManager`: it's an interface

- [ ] `EVManager.random(0)`: it has no body (team/creative/enhancedvisuals/EVManager.java:19)
- [ ] `EVManager.playSound(1)`: it has no body (team/creative/enhancedvisuals/EVManager.java:21)
- [ ] `EVManager.playSound(2)`: it has no body (team/creative/enhancedvisuals/EVManager.java:23)
- [ ] `EVManager.playSound(2)`: it has no body (team/creative/enhancedvisuals/EVManager.java:25)
- [ ] `EVManager.playSound(3)`: it has no body (team/creative/enhancedvisuals/EVManager.java:27)
- [ ] `EVManager.playSoundFadeOut(3)`: it has no body (team/creative/enhancedvisuals/EVManager.java:29)
- [ ] `EVManager.visuals(1)`: it has no body (team/creative/enhancedvisuals/EVManager.java:31)
- [ ] `EVManager.clearEverything(0)`: it has no body (team/creative/enhancedvisuals/EVManager.java:33)
- [ ] `EVManager.add(1)`: it has no body (team/creative/enhancedvisuals/EVManager.java:35)
- [ ] `EVManager.remove(1)`: it has no body (team/creative/enhancedvisuals/EVManager.java:37)
- [ ] `EVManager.playTicking(3)`: it has no body (team/creative/enhancedvisuals/EVManager.java:39)
- [ ] `EVManager.addVisualFadeOut(3)`: it has no body (team/creative/enhancedvisuals/EVManager.java:41)
- [ ] `EVManager.addVisualFadeOut(3)`: it has no body (team/creative/enhancedvisuals/EVManager.java:43)
- [ ] `EVManager.addVisualFadeOut(3)`: it has no body (team/creative/enhancedvisuals/EVManager.java:45)
- [ ] `EVManager.addParticlesFadeOut(5)`: it has no body (team/creative/enhancedvisuals/EVManager.java:47)
- [ ] `EVManager.addParticlesFadeOut(6)`: it has no body (team/creative/enhancedvisuals/EVManager.java:49)
- [ ] `EVManager.addParticlesFadeOut(5)`: it has no body (team/creative/enhancedvisuals/EVManager.java:51)
- [ ] `EVManager.addParticlesFadeOut(6)`: it has no body (team/creative/enhancedvisuals/EVManager.java:53)
- [ ] `EVManager.addParticle(4)`: it has no body (team/creative/enhancedvisuals/EVManager.java:55)
- [ ] `EVManager.generateOffset(3)`: it has no body (team/creative/enhancedvisuals/EVManager.java:57)
- [ ] `ClientPacketListenerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/enhancedvisuals/EVManager.java:13)
- [ ] `EntityAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/enhancedvisuals/EVManager.java:8)
- [ ] `ExplosionMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/enhancedvisuals/EVManager.java:16)
- [ ] `GameRendererMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/enhancedvisuals/EVManager.java:17)
- [ ] `PlayerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/enhancedvisuals/EVManager.java:13)
- [ ] `PostChainAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/enhancedvisuals/EVManager.java:10)
- [ ] `PostPassAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/enhancedvisuals/EVManager.java:10)
- [ ] `ProjectileMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/enhancedvisuals/EVManager.java:12)
- [ ] `SoundEngineAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/enhancedvisuals/EVManager.java:12)
- [ ] `SoundManagerAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (team/creative/enhancedvisuals/EVManager.java:9)

### `ClearVisualCommand`


### `DisplayVisualCommand`


### `EVManagerServer`: it implements `EVManager` (team.creative.enhancedvisuals)

- [ ] `EVManagerServer.playSound(1)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:23)
- [ ] `EVManagerServer.playSound(2)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:27)
- [ ] `EVManagerServer.playSound(2)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:31)
- [ ] `EVManagerServer.playSound(3)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:35)
- [ ] `EVManagerServer.playSoundFadeOut(3)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:39)
- [ ] `EVManagerServer.visuals(1)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:43)
- [ ] `EVManagerServer.clearEverything(0)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:48)
- [ ] `EVManagerServer.add(1)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:52)
- [ ] `EVManagerServer.remove(1)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:56)
- [ ] `EVManagerServer.playTicking(3)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:61)
- [ ] `EVManagerServer.addVisualFadeOut(3)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:65)
- [ ] `EVManagerServer.addVisualFadeOut(3)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:70)
- [ ] `EVManagerServer.addVisualFadeOut(3)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:75)
- [ ] `EVManagerServer.addParticlesFadeOut(5)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:80)
- [ ] `EVManagerServer.addParticlesFadeOut(6)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:84)
- [ ] `EVManagerServer.addParticlesFadeOut(5)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:88)
- [ ] `EVManagerServer.addParticlesFadeOut(6)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:92)
- [ ] `EVManagerServer.addParticle(4)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:96)
- [ ] `EVManagerServer.generateOffset(3)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:101)
- [ ] `EVManagerServer.random(0)`: its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)) (team/creative/enhancedvisuals/server/EVManagerServer.java:106)

### `EVServer`

