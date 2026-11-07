// Entity Sound Features, ported to Horizon by horizon-port from entity_sound_features-0.8.2-26.1-fabric.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_entity_sound_features). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "entity_sound_features";
pub const NAME: &str = "Entity Sound Features";
pub const VERSION: &str = "0.8.2";
pub const DESCRIPTION: &str = "This is an expansion of the ETF mod, it adds support for ESF format resource packs. The ESF format allows you to vary an entities sound with .properties files just like OptiFine/ETF random textures";

/// The mod's keys for Options > Mods: name, label, default key.
pub const KEYS: &[[&str; 3]] = &[];

/// The mod's own Java that doesn't need the game, translated to Rust by horizon-port.
/// Each Java method is here whole or not at all; the ones left out say why.
#[allow(dead_code, unused_variables, unused_mut, unused_parens, unused_imports, unreachable_code,
    non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
pub mod java {
    use std::sync::{LazyLock, Mutex};

    /// A Java static that changes: one value behind a lock.
    pub struct JCell<T>(Mutex<T>);
    impl<T: Clone> JCell<T> {
        pub const fn new(v: T) -> Self {
            Self(Mutex::new(v))
        }
        pub fn get(&self) -> T {
            self.0.lock().unwrap_or_else(|e| e.into_inner()).clone()
        }
        pub fn set(&self, v: T) {
            *self.0.lock().unwrap_or_else(|e| e.into_inner()) = v;
        }
        pub fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            f(&mut self.0.lock().unwrap_or_else(|e| e.into_inner()))
        }
    }

    /// Java's HashMap, LinkedHashMap and TreeMap. Keeps insertion order (LinkedHashMap's; HashMap's
    /// order is unspecified), or key order when made with `new_sorted` (TreeMap's).
    #[derive(Clone, Debug)]
    pub struct JMap<K, V> {
        index: std::collections::HashMap<K, usize>,
        entries: Vec<(K, V)>,
        sorted: bool,
    }
    /// Java's Map.equals: the same keys with equal values, in any order.
    impl<K: std::hash::Hash + Eq, V: PartialEq> PartialEq for JMap<K, V> {
        fn eq(&self, o: &Self) -> bool {
            self.entries.len() == o.entries.len()
                && self.entries.iter().all(|(k, v)| o.index.get(k).is_some_and(|&i| o.entries[i].1 == *v))
        }
    }
    impl<K, V> Default for JMap<K, V> {
        fn default() -> Self {
            Self { index: std::collections::HashMap::new(), entries: Vec::new(), sorted: false }
        }
    }
    impl<K: std::hash::Hash + Ord + Clone, V> JMap<K, V> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self { sorted: true, ..Self::default() }
        }
        pub fn from_iter(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn from_iter_sorted(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new_sorted();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn len(&self) -> usize {
            self.entries.len()
        }
        pub fn is_empty(&self) -> bool {
            self.entries.is_empty()
        }
        pub fn clear(&mut self) {
            self.index.clear();
            self.entries.clear();
        }
        pub fn contains_key<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> bool
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.contains_key(k)
        }
        pub fn get<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> Option<&V>
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.get(k).map(|&i| &self.entries[i].1)
        }
        fn reindex(&mut self) {
            self.index.clear();
            for (i, (k, _)) in self.entries.iter().enumerate() {
                self.index.insert(k.clone(), i);
            }
        }
        fn slot(&mut self, k: K, v: V) -> usize {
            if self.sorted {
                let at = self.entries.partition_point(|e| e.0 < k);
                self.entries.insert(at, (k, v));
                self.reindex();
                at
            } else {
                self.index.insert(k.clone(), self.entries.len());
                self.entries.push((k, v));
                self.entries.len() - 1
            }
        }
        pub fn insert(&mut self, k: K, v: V) -> Option<V> {
            if let Some(&i) = self.index.get(&k) {
                return Some(std::mem::replace(&mut self.entries[i].1, v));
            }
            self.slot(k, v);
            None
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, k: &Q) -> Option<V>
        where
            K: std::borrow::Borrow<Q>,
        {
            let i = *self.index.get(k)?;
            let (_, v) = self.entries.remove(i);
            self.reindex();
            Some(v)
        }
        /// computeIfAbsent and putIfAbsent: the value, made first when the key is missing.
        pub fn or_insert_with(&mut self, k: K, f: impl FnOnce() -> V) -> &mut V {
            let i = match self.index.get(&k) {
                Some(&i) => i,
                None => self.slot(k, f()),
            };
            &mut self.entries[i].1
        }
        pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
            self.entries.iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &K> {
            self.entries.iter().map(|e| &e.0)
        }
        pub fn values(&self) -> impl Iterator<Item = &V> {
            self.entries.iter().map(|e| &e.1)
        }
        pub fn clone_pairs(&self) -> Vec<(K, V)>
        where
            V: Clone,
        {
            self.entries.clone()
        }
    }

    /// Java's HashSet, LinkedHashSet and TreeSet, kept like JMap's keys.
    #[derive(Clone, Debug)]
    pub struct JSet<T>(JMap<T, ()>);
    impl<T: std::hash::Hash + Eq> PartialEq for JSet<T> {
        fn eq(&self, o: &Self) -> bool {
            self.0 == o.0
        }
    }
    impl<T> Default for JSet<T> {
        fn default() -> Self {
            Self(JMap::default())
        }
    }
    impl<T: std::hash::Hash + Ord + Clone> JSet<T> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self(JMap::new_sorted())
        }
        pub fn from_iter(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn from_iter_sorted(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new_sorted();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn len(&self) -> usize {
            self.0.len()
        }
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        pub fn clear(&mut self) {
            self.0.clear();
        }
        /// True when it wasn't there yet, as Java's add.
        pub fn insert(&mut self, v: T) -> bool {
            self.0.insert(v, ()).is_none()
        }
        pub fn contains<Q: std::hash::Hash + Eq + ?Sized>(&self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.contains_key(v)
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.remove(v).is_some()
        }
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.0.keys()
        }
        pub fn clone_pairs(&self) -> Vec<T> {
            self.0.keys().cloned().collect()
        }
    }

    /// The mod's data folder in Horizon, set by the hooks (game_side.rs `ensure_init`) before the mod's code runs.
    pub static J_DATA_DIR: JCell<Option<std::path::PathBuf>> = JCell::new(None);
    /// FabricLoader's config folder: the mod's data folder in Horizon (./config until the game sets it).
    pub fn j_config_dir() -> std::path::PathBuf {
        J_DATA_DIR.get().unwrap_or_else(|| std::path::PathBuf::from("config"))
    }

    /// Minecraft's RandomSource / java.util.Random: other numbers than Java's, the same ranges.
    #[derive(Clone, Debug)]
    pub struct JRandom(u64);
    impl JRandom {
        pub fn new() -> Self {
            let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0x9E37_79B9);
            Self(t | 1)
        }
        fn next(&mut self) -> u64 {
            let mut v = self.0;
            v ^= v << 13;
            v ^= v >> 7;
            v ^= v << 17;
            self.0 = v;
            v
        }
        /// 0 inclusive to n exclusive; Java throws on n <= 0.
        pub fn next_int(&mut self, n: i32) -> i32 {
            assert!(n > 0, "bound must be positive");
            (self.next() % n as u64) as i32
        }
        pub fn next_int_between(&mut self, lo: i32, hi: i32) -> i32 {
            lo + self.next_int(hi - lo)
        }
        pub fn next_float(&mut self) -> f32 {
            (self.next() >> 40) as f32 / (1u64 << 24) as f32
        }
        pub fn next_double(&mut self) -> f64 {
            (self.next() >> 11) as f64 / (1u64 << 53) as f64
        }
        pub fn next_boolean(&mut self) -> bool {
            self.next() & 1 == 1
        }
        pub fn next_gaussian(&mut self) -> f64 {
            let (u, v) = (self.next_double().max(1e-12), self.next_double());
            (-2.0 * u.ln()).sqrt() * (std::f64::consts::TAU * v).cos()
        }
    }

    /// Minecraft's Identifier (`namespace:path`), which is only text.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct JIdentifier {
        pub namespace: String,
        pub path: String,
    }
    impl JIdentifier {
        pub fn new(namespace: &str, path: &str) -> Self {
            Self { namespace: namespace.to_string(), path: path.to_string() }
        }
        /// `a:b`, or `minecraft:b` without a namespace.
        pub fn parse(s: &str) -> Self {
            match s.split_once(':') {
                Some((n, p)) => Self::new(n, p),
                None => Self::new("minecraft", s),
            }
        }
        /// Identifier.tryParse: None (null) when it has characters an identifier can't.
        pub fn try_parse(s: &str) -> Option<Self> {
            let id = Self::parse(s);
            Self::try_build(&id.namespace, &id.path)
        }
        pub fn try_build(namespace: &str, path: &str) -> Option<Self> {
            let ns_ok = namespace.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-'));
            let path_ok = path.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-' | '/'));
            if ns_ok && path_ok { Some(Self::new(namespace, path)) } else { None }
        }
    }
    impl std::fmt::Display for JIdentifier {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.namespace, self.path)
        }
    }

    /// Java's String.equals.
    pub fn j_eq(a: &str, b: &str) -> bool {
        a == b
    }
    pub fn j_eq_ignore_case(a: &str, b: &str) -> bool {
        a.to_lowercase() == b.to_lowercase()
    }
    /// Java's String.length (in chars).
    pub fn j_len(s: &str) -> i32 {
        s.chars().count() as i32
    }
    pub fn j_char_at(s: &str, i: i32) -> char {
        s.chars().nth(i as usize).expect("charAt out of range")
    }
    /// Java's substring, by char.
    pub fn j_substring(s: &str, from: i32, to: Option<i32>) -> String {
        let n = s.chars().count() as i32;
        let to = to.unwrap_or(n);
        assert!(0 <= from && from <= to && to <= n, "substring out of range");
        s.chars().skip(from as usize).take((to - from) as usize).collect()
    }
    pub fn j_index_of(s: &str, sub: &str) -> i32 {
        s.find(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    pub fn j_index_of_char(s: &str, c: char) -> i32 {
        s.chars().position(|x| x == c).map_or(-1, |i| i as i32)
    }
    pub fn j_last_index_of(s: &str, sub: &str) -> i32 {
        s.rfind(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    /// Java's String.split on plain text: trailing empty parts dropped.
    pub fn j_split(s: &str, sep: &str) -> Vec<String> {
        let mut v: Vec<String> = s.split(sep).map(str::to_string).collect();
        while v.len() > 1 && v.last().is_some_and(|p| p.is_empty()) {
            v.pop();
        }
        v
    }
    /// Java's String.split(sep, limit) on plain text.
    pub fn j_split_limit(s: &str, sep: &str, limit: i32) -> Vec<String> {
        if limit == 0 {
            return j_split(s, sep);
        }
        if limit > 0 {
            return s.splitn(limit as usize, sep).map(str::to_string).collect();
        }
        s.split(sep).map(str::to_string).collect()
    }
    /// Java's String.compareTo: by UTF-16 unit, then by length.
    pub fn j_compare(a: &str, b: &str) -> i32 {
        let (x, y): (Vec<u16>, Vec<u16>) = (a.encode_utf16().collect(), b.encode_utf16().collect());
        for (p, q) in x.iter().zip(y.iter()) {
            if p != q {
                return *p as i32 - *q as i32;
            }
        }
        x.len() as i32 - y.len() as i32
    }
    /// Java's String.regionMatches, by char.
    pub fn j_region_matches(s: &str, ignore_case: bool, from: i32, other: &str, other_from: i32, len: i32) -> bool {
        let (a, b): (Vec<char>, Vec<char>) = (s.chars().collect(), other.chars().collect());
        if from < 0 || other_from < 0 || from as i64 + len as i64 > a.len() as i64 || other_from as i64 + len as i64 > b.len() as i64 {
            return false;
        }
        (0..len.max(0) as usize).all(|i| {
            let (p, q) = (a[from as usize + i], b[other_from as usize + i]);
            p == q || ignore_case && (j_upper_char(p) == j_upper_char(q) || j_lower_char(p) == j_lower_char(q))
        })
    }
    /// Java's Character.toLowerCase / toUpperCase (one char).
    pub fn j_lower_char(c: char) -> char {
        let mut l = c.to_lowercase();
        if l.len() == 1 { l.next().unwrap_or(c) } else { c }
    }
    pub fn j_upper_char(c: char) -> char {
        let mut u = c.to_uppercase();
        if u.len() == 1 { u.next().unwrap_or(c) } else { c }
    }
    /// Java's Arrays.copyOfRange: past the end is filled with defaults.
    pub fn j_copy_of_range<T: Clone + Default>(a: &[T], from: usize, to: usize) -> Vec<T> {
        assert!(from <= to && from <= a.len(), "copyOfRange out of range");
        let mut v: Vec<T> = a[from..to.min(a.len())].to_vec();
        v.resize(to - from, T::default());
        v
    }
    pub fn j_hash(s: &str) -> i32 {
        s.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
    }
    /// Java's Boolean.parseBoolean.
    pub fn j_parse_bool(s: &str) -> bool {
        s.eq_ignore_ascii_case("true")
    }
    /// A float as Java prints it in "..." + x (1.0, not 1).
    pub fn j_float(x: f64) -> String {
        if x.is_finite() && x == x.trunc() && x.abs() < 1e7 { format!("{x:.1}") } else { format!("{x}") }
    }
    /// Java's Math.round.
    pub fn j_round_f(x: f32) -> i32 {
        (x + 0.5).floor() as i32
    }
    pub fn j_round_d(x: f64) -> i64 {
        (x + 0.5).floor() as i64
    }
    /// Java's Math.clamp / Mth.clamp (no panic when min > max).
    pub fn j_clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
        if v < min { min } else if v > max { max } else { v }
    }
    pub fn j_lerp<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>>(d: T, a: T, b: T) -> T {
        a + d * (b - a)
    }
    pub fn j_signum_f(x: f32) -> f32 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_signum_d(x: f64) -> f64 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_wrap_degrees(x: f64) -> f64 {
        let mut v = x % 360.0;
        if v >= 180.0 {
            v -= 360.0;
        }
        if v < -180.0 {
            v += 360.0;
        }
        v
    }
    pub fn j_frac(x: f64) -> f64 {
        x - x.floor()
    }
    pub fn j_compare_f(a: f64, b: f64) -> i32 {
        a.partial_cmp(&b).map_or(0, |o| o as i32)
    }
    pub fn j_copy_of<T: Clone + Default>(a: &[T], n: usize) -> Vec<T> {
        let mut v: Vec<T> = a.iter().take(n).cloned().collect();
        v.resize(n, T::default());
        v
    }
    /// Java's System.currentTimeMillis.
    pub fn j_millis() -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
    }
    /// Java's System.nanoTime: only differences mean anything.
    pub fn j_nanos() -> i64 {
        static START: LazyLock<std::time::Instant> = LazyLock::new(std::time::Instant::now);
        START.elapsed().as_nanos() as i64
    }

    /// `ESF` (traben/entity_sound_features/ESF.java)
    pub mod esf {
        use super::*;
        pub const MOD_ID: &str = "entity_sound_features";
        // LOGGER: the logger. Log calls become comments.
        // configHandler: not translated, it uses `TConfigHandler` (traben.tconfig).
        #[derive(Clone, Debug)]
        pub struct ESF {
        }
        impl ESF {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ESF.res(1): not translated, it calls `ETFUtils2` (traben.entity_texture_features.utils).res.
        // ESF.res(2): not translated, it calls `ETFUtils2` (traben.entity_texture_features.utils).res.
        // ESF.config(0): not translated, it uses `TConfigHandler` (traben.tconfig).
        // ESF.init(0): not translated, it uses ESF.configHandler, which isn't translated (it uses `TConfigHandler` (traben.tconfig)).
        // ESF.getConfigScreen(1): not translated, it uses Minecraft's `Screen`.
        // ESF.getConfigScreen(2): not translated, it uses Minecraft's `Screen`.
        pub fn log(message: &str) {
            // log: "[ESF] {}"
        }
        pub fn log_error(message: &str) {
            // log: "[ESF] {}"
        }
        pub fn log_warn(message: &str) {
            // log: "[ESF] {}"
        }
    }

    /// `ESFConfig` (traben/entity_sound_features/ESFConfig.java)
    pub mod esf_config {
        use super::*;
        // The class itself isn't translated: it extends `TConfig` (traben.tconfig). Its static parts are below.
        // ESFConfig.isEntityAllowedToModifySounds(1): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ESFConfig.getGUIOptions(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ESFConfig.getEntitySettings(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ESFConfig.addEntitySettings(2): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // ESFConfig.getModIcon(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
    }

    /// `ESFConfig.AnnounceMode` (traben/entity_sound_features/ESFConfig.java)
    pub mod esf_config_announce_mode {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ESFConfig.AnnounceMode.all(0): not translated, its class isn't translated (its constants carry values).
        // ESFConfig.AnnounceMode.AnnounceMode(1): not translated, its class isn't translated (its constants carry values).
        // ESFConfig.AnnounceMode.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `ESFConfig.EntitySearchMode` (traben/entity_sound_features/ESFConfig.java)
    pub mod esf_config_entity_search_mode {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ESFConfig.EntitySearchMode.EntitySearchMode(1): not translated, its class isn't translated (its constants carry values).
        // ESFConfig.EntitySearchMode.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `ESFInit` (traben/entity_sound_features/ESFInit.java)
    pub mod esf_init {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        // ESFInit.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
    }

    /// `ESFModMenu` (traben/entity_sound_features/ESFModMenu.java)
    pub mod esf_mod_menu {
        use super::*;
        // The class itself isn't translated: it implements `ModMenuApi` (com.terraformersmc.modmenu.api). Its static parts are below.
        // ESFModMenu.getModConfigScreenFactory(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
    }

    /// `ESFSound` (traben/entity_sound_features/ESFSound.java)
    pub mod esf_sound {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Sound`. Its static parts are below.
        // ESFSound.ESFSound(1): not translated, its class isn't translated (it extends Minecraft's `Sound`).
        // ESFSound.ESFSound(2): not translated, its class isn't translated (it extends Minecraft's `Sound`).
        // ESFSound.getPath(0): not translated, its class isn't translated (it extends Minecraft's `Sound`).
    }

    /// `ESFSoundContext` (traben/entity_sound_features/ESFSoundContext.java)
    pub mod esf_sound_context {
        use super::*;
        // variantSuppliers: not translated, it uses ESFVariantSupplier, which isn't translated (it uses Minecraft's `WeighedSoundEvents`).
        pub static ANNOUNCED_SOUNDS: LazyLock<JCell<JSet<JIdentifier>>> = LazyLock::new(|| JCell::new(JSet::new()));
        pub static KNOWN_ESF_SOUNDS: LazyLock<JCell<JSet<JIdentifier>>> = LazyLock::new(|| JCell::new(JSet::new()));
        // entitySource: not translated, it uses `ETFEntityRenderState` (traben.entity_texture_features.features.state).
        // lastRuleMet: not translated, it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache).
        // lastSuffix: not translated, it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache).
        #[derive(Clone, Debug)]
        pub struct ESFSoundContext {
        }
        impl ESFSoundContext {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn add_known_esf_sound(sound: Option<JIdentifier>) {
            { let a_0 = (sound.as_ref().expect("null")).clone(); esf_sound_context::KNOWN_ESF_SOUNDS.with(|o| o.insert(a_0)) };
        }
        // ESFSoundContext.shouldCaptureEntity(1): not translated, it calls ESF.config, which isn't translated.
        // ESFSoundContext.setSource(1): not translated, it uses `ETFEntity` (traben.entity_texture_features.utils).
        // ESFSoundContext.preTestEntity(1): not translated, it uses `ETFEntityRenderState` (traben.entity_texture_features.features.state).
        // ESFSoundContext.announceSound(2): not translated, it uses ESFSoundContext.entitySource, which isn't translated (it uses `ETFEntityRenderState` (traben.entity_texture_features.features.state)).
        // ESFSoundContext.announceWithEntity(2): not translated, it uses ESFSoundContext.entitySource, which isn't translated (it uses `ETFEntityRenderState` (traben.entity_texture_features.features.state)).
        // ESFSoundContext.resetContext(0): not translated, it uses ESFSoundContext.lastRuleMet, which isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)).
        // ESFSoundContext.searchForEntity(4): not translated, it uses Minecraft's `Entity`.
        // ESFSoundContext.searchForBlockEntity(4): not translated, it uses Minecraft's `Level`.
    }

    /// `ESFVariantSupplier` (traben/entity_sound_features/ESFVariantSupplier.java)
    pub mod esf_variant_supplier {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `WeighedSoundEvents`. Its static parts are below.
        // SOUND_EVENT_REGISTRATION_TYPE: not translated, its value couldn't be read: an anonymous class.
        // ESFVariantSupplier.ESFVariantSupplier(3): not translated, its class isn't translated (it uses Minecraft's `WeighedSoundEvents`).
        // ESFVariantSupplier.getOrNull(1): not translated, it uses ESFVariantSupplier, which isn't translated (it uses Minecraft's `WeighedSoundEvents`).
        // ESFVariantSupplier.parseSoundEventVariant(4): not translated, its body couldn't be read: an anonymous class.
        // ESFVariantSupplier.preTestEntity(1): not translated, its class isn't translated (it uses Minecraft's `WeighedSoundEvents`).
        // ESFVariantSupplier.getSoundVariantOrNull(0): not translated, its class isn't translated (it uses Minecraft's `WeighedSoundEvents`).
        // ESFVariantSupplier.toString(0): not translated, its class isn't translated (it uses Minecraft's `WeighedSoundEvents`).
    }

    /// `ESFMethods` (traben/entity_sound_features/methods/ESFMethods.java)
    pub mod esf_methods {
        use super::*;
        pub static LAST_SOUND_TICKS: LazyLock<JMap<String, i64>> = LazyLock::new(|| JMap::new());
        #[derive(Clone, Debug)]
        pub struct ESFMethods {
        }
        impl ESFMethods {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ESFMethods.register(0): not translated, unknown static ESFMethods.class.
        // ESFMethods.playingSound(1): not translated, it calls ESF.res, which isn't translated.
        // ESFMethods.playsoundStatic(6): not translated, it uses `EMFEntityRenderState` (traben.entity_model_features.models.animation.state).
    }

    /// `Plugin` (traben/entity_sound_features/mixin/Plugin.java)
    pub mod plugin {
        use super::*;
        // The class itself isn't translated: it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility). Its static parts are below.
        // Plugin.onLoad(1): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.getRefMapperConfig(0): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.shouldApplyMixin(2): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.acceptTargets(2): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.getMixins(0): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.preApply(4): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.postApply(4): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
    }

    /// `PlayingSoundProperty` (traben/entity_sound_features/properties/PlayingSoundProperty.java)
    pub mod playing_sound_property {
        use super::*;
        // The class itself isn't translated: it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // PlayingSoundProperty.PlayingSoundProperty(2): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // PlayingSoundProperty.getPropertyOrNull(2): not translated, it uses PlayingSoundProperty, which isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // PlayingSoundProperty.testEntityInternal(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // PlayingSoundProperty.getPlayingSounds(0): not translated, it uses Minecraft's `SoundEngine`.
        // PlayingSoundProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // PlayingSoundProperty.shouldForceLowerCaseCheck(0): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // PlayingSoundProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `StringArrayOrRegexProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `SoundRuleIndexProperty` (traben/entity_sound_features/properties/SoundRuleIndexProperty.java)
    pub mod sound_rule_index_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // SoundRuleIndexProperty.SoundRuleIndexProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SoundRuleIndexProperty.getPropertyOrNull(2): not translated, it uses SoundRuleIndexProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SoundRuleIndexProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SoundRuleIndexProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `SoundSuffixProperty` (traben/entity_sound_features/properties/SoundSuffixProperty.java)
    pub mod sound_suffix_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // SoundSuffixProperty.SoundSuffixProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SoundSuffixProperty.getPropertyOrNull(2): not translated, it uses SoundSuffixProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SoundSuffixProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // SoundSuffixProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "entity_sound_features");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
