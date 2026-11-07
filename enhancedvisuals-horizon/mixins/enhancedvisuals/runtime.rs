// EnhancedVisuals, ported to Horizon by horizon-port from EnhancedVisuals_FABRIC_v1.8.35_mc26.3.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_enhancedvisuals). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "enhancedvisuals";
pub const NAME: &str = "EnhancedVisuals";
pub const VERSION: &str = "1.8.35";
pub const DESCRIPTION: &str = "Experience realism and extra video effects for your in-game experience.";

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

    /// `FireParticlesEvent` (team/creative/enhancedvisuals/api/event/FireParticlesEvent.java)
    pub mod fire_particles_event {
        use super::*;
        // The class itself isn't translated: it extends `Event` (net.neoforged.bus.api). Its static parts are below.
        // FireParticlesEvent.FireParticlesEvent(3): not translated, its class isn't translated (it extends `Event` (net.neoforged.bus.api)).
        // FireParticlesEvent.setFireSplashes(1): not translated, its class isn't translated (it extends `Event` (net.neoforged.bus.api)).
        // FireParticlesEvent.setDuration(2): not translated, its class isn't translated (it extends `Event` (net.neoforged.bus.api)).
        // FireParticlesEvent.getNewFireSplashes(0): not translated, its class isn't translated (it extends `Event` (net.neoforged.bus.api)).
        // FireParticlesEvent.getNewFireDurationMin(0): not translated, its class isn't translated (it extends `Event` (net.neoforged.bus.api)).
        // FireParticlesEvent.getNewFireDurationMax(0): not translated, its class isn't translated (it extends `Event` (net.neoforged.bus.api)).
    }

    /// `SelectEndermanEvent` (team/creative/enhancedvisuals/api/event/SelectEndermanEvent.java)
    pub mod select_enderman_event {
        use super::*;
        // The class itself isn't translated: it extends `Event` (net.neoforged.bus.api). Its static parts are below.
        // SelectEndermanEvent.SelectEndermanEvent(1): not translated, its class isn't translated (it extends `Event` (net.neoforged.bus.api)).
    }

    /// `SplashEvent` (team/creative/enhancedvisuals/api/event/SplashEvent.java)
    pub mod splash_event {
        use super::*;
        // The class itself isn't translated: it extends `Event` (net.neoforged.bus.api). Its static parts are below.
        // SplashEvent.SplashEvent(1): not translated, its class isn't translated (it extends `Event` (net.neoforged.bus.api)).
    }

    /// `VisualExplosionEvent` (team/creative/enhancedvisuals/api/event/VisualExplosionEvent.java)
    pub mod visual_explosion_event {
        use super::*;
        // The class itself isn't translated: it extends `Event` (net.neoforged.bus.api). Its static parts are below.
        // VisualExplosionEvent.VisualExplosionEvent(1): not translated, its class isn't translated (it extends `Event` (net.neoforged.bus.api)).
        // VisualExplosionEvent.isBlurDisabled(0): not translated, its class isn't translated (it extends `Event` (net.neoforged.bus.api)).
        // VisualExplosionEvent.setDisableBlur(1): not translated, its class isn't translated (it extends `Event` (net.neoforged.bus.api)).
        // VisualExplosionEvent.setNewDamage(1): not translated, its class isn't translated (it extends `Event` (net.neoforged.bus.api)).
        // VisualExplosionEvent.getNewDamage(0): not translated, its class isn't translated (it extends `Event` (net.neoforged.bus.api)).
    }

    /// `Particle` (team/creative/enhancedvisuals/api/Particle.java)
    pub mod particle {
        use super::*;
        // The class itself isn't translated: it extends `Visual`. Its static parts are below.
        // Particle.Particle(9): not translated, its class isn't translated (it extends `Visual`).
        // Particle.Particle(8): not translated, its class isn't translated (it extends `Visual`).
        // Particle.getWidth(1): not translated, its class isn't translated (it extends `Visual`).
        // Particle.getHeight(1): not translated, its class isn't translated (it extends `Visual`).
    }

    /// `VisualType` (team/creative/enhancedvisuals/api/type/VisualType.java)
    pub mod visual_type {
        use super::*;
        // The class itself isn't translated: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api). Its static parts are below.
        // TYPES: not translated, it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualType.types(0): not translated, it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualType.VisualType(2): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualType.setIgnoreWater(0): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualType.isAffectedByWater(0): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualType.configured(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualType.getColor(0): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualType.canRotate(0): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualType.isVisible(2): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualType.scaleVariants(0): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualType.randomScale(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
    }

    /// `VisualTypeAnimatedTexture` (team/creative/enhancedvisuals/api/type/VisualTypeAnimatedTexture.java)
    pub mod visual_type_animated_texture {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct VisualTypeAnimatedTexture {
        }
        impl VisualTypeAnimatedTexture {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `VisualTypeBlur` (team/creative/enhancedvisuals/api/type/VisualTypeBlur.java)
    pub mod visual_type_blur {
        use super::*;
        // The class itself isn't translated: it extends `VisualTypeShader`. Its static parts are below.
        pub static BLUR_SHADER: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::try_build("enhancedvisuals", "blur").expect("null"));
        // VisualTypeBlur.VisualTypeBlur(1): not translated, its class isn't translated (it extends `VisualTypeShader`).
    }

    /// `VisualTypeFocus` (team/creative/enhancedvisuals/api/type/VisualTypeFocus.java)
    pub mod visual_type_focus {
        use super::*;
        // The class itself isn't translated: it extends `VisualTypeShader`. Its static parts are below.
        pub static IDENTIFIER: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::try_build("enhancedvisuals", "blobs2").expect("null"));
        // VisualTypeFocus.VisualTypeFocus(1): not translated, its class isn't translated (it extends `VisualTypeShader`).
    }

    /// `VisualTypeOverlay` (team/creative/enhancedvisuals/api/type/VisualTypeOverlay.java)
    pub mod visual_type_overlay {
        use super::*;
        // The class itself isn't translated: it extends `VisualTypeTexture`. Its static parts are below.
        // VisualTypeOverlay.VisualTypeOverlay(2): not translated, its class isn't translated (it extends `VisualTypeTexture`).
        // VisualTypeOverlay.VisualTypeOverlay(1): not translated, its class isn't translated (it extends `VisualTypeTexture`).
    }

    /// `VisualTypeParticle` (team/creative/enhancedvisuals/api/type/VisualTypeParticle.java)
    pub mod visual_type_particle {
        use super::*;
        // The class itself isn't translated: it extends `VisualTypeTexture`. Its static parts are below.
        // VisualTypeParticle.VisualTypeParticle(3): not translated, its class isn't translated (it extends `VisualTypeTexture`).
        // VisualTypeParticle.VisualTypeParticle(1): not translated, its class isn't translated (it extends `VisualTypeTexture`).
        // VisualTypeParticle.scaleVariants(0): not translated, its class isn't translated (it extends `VisualTypeTexture`).
        // VisualTypeParticle.randomScale(1): not translated, its class isn't translated (it extends `VisualTypeTexture`).
    }

    /// `VisualTypeParticleColored` (team/creative/enhancedvisuals/api/type/VisualTypeParticleColored.java)
    pub mod visual_type_particle_colored {
        use super::*;
        // The class itself isn't translated: it extends `VisualTypeParticle`. Its static parts are below.
        // VisualTypeParticleColored.VisualTypeParticleColored(4): not translated, its class isn't translated (it extends `VisualTypeParticle`).
        // VisualTypeParticleColored.VisualTypeParticleColored(2): not translated, its class isn't translated (it extends `VisualTypeParticle`).
        // VisualTypeParticleColored.configured(1): not translated, its class isn't translated (it extends `VisualTypeParticle`).
        // VisualTypeParticleColored.getColor(0): not translated, its class isn't translated (it extends `VisualTypeParticle`).
    }

    /// `VisualTypeSaturation` (team/creative/enhancedvisuals/api/type/VisualTypeSaturation.java)
    pub mod visual_type_saturation {
        use super::*;
        // The class itself isn't translated: it extends `VisualTypeShader`. Its static parts are below.
        // VisualTypeSaturation.VisualTypeSaturation(1): not translated, its class isn't translated (it extends `VisualTypeShader`).
        // VisualTypeSaturation.isVisible(2): not translated, its class isn't translated (it extends `VisualTypeShader`).
    }

    /// `VisualTypeShader` (team/creative/enhancedvisuals/api/type/VisualTypeShader.java)
    pub mod visual_type_shader {
        use super::*;
        // The class itself isn't translated: it extends `VisualType`. Its static parts are below.
        // VisualTypeShader.VisualTypeShader(2): not translated, its class isn't translated (it extends `VisualType`).
    }

    /// `VisualTypeTexture` (team/creative/enhancedvisuals/api/type/VisualTypeTexture.java)
    pub mod visual_type_texture {
        use super::*;
        // The class itself isn't translated: it extends `VisualType`. Its static parts are below.
        // VisualTypeTexture.VisualTypeTexture(4): not translated, its class isn't translated (it extends `VisualType`).
        // VisualTypeTexture.VisualTypeTexture(3): not translated, its class isn't translated (it extends `VisualType`).
    }

    /// `Visual` (team/creative/enhancedvisuals/api/Visual.java)
    pub mod visual {
        use super::*;
        // The class itself isn't translated: it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)). Its static parts are below.
        // Visual.Visual(4): not translated, its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))).
        // Visual.Visual(3): not translated, its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))).
        // Visual.setOpacityInternal(1): not translated, its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))).
        // Visual.getOpacityInternal(0): not translated, its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))).
        // Visual.getOpacity(0): not translated, its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))).
        // Visual.displayed(0): not translated, its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))).
        // Visual.addToDisplay(0): not translated, its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))).
        // Visual.removeFromDisplay(0): not translated, its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))).
        // Visual.getCategory(0): not translated, its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))).
        // Visual.isVisible(0): not translated, its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))).
        // Visual.tick(0): not translated, its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))).
        // Visual.getWidth(1): not translated, its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))).
        // Visual.getHeight(1): not translated, its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))).
        // Visual.isAffectedByWater(0): not translated, its class isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))).
    }

    /// `VisualCategory` (team/creative/enhancedvisuals/api/VisualCategory.java)
    pub mod visual_category {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // VisualCategory.isAffectedByWater(0): not translated, it has no body.
    }

    /// `VisualHandler` (team/creative/enhancedvisuals/api/VisualHandler.java)
    pub mod visual_handler {
        use super::*;
        // The class itself isn't translated: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api). Its static parts are below.
        // VisualHandler.configured(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.tick(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.isEnabled(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.manager(0): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.random(0): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.playSound(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.playSound(2): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.playSound(2): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.playSound(3): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.playSoundFadeOut(3): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.visuals(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.clearEverything(0): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.add(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.remove(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.playTicking(3): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.addVisualFadeOut(3): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.addVisualFadeOut(3): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.addVisualFadeOut(3): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.addParticlesFadeOut(5): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.addParticlesFadeOut(6): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.addParticlesFadeOut(5): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.addParticlesFadeOut(6): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.addParticle(4): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualHandler.generateOffset(3): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
    }

    /// `EVClient` (team/creative/enhancedvisuals/client/EVClient.java)
    pub mod ev_client {
        use super::*;
        // CLIENT_TYPES: not translated, it uses `PairList` (team.creative.creativecore.common.util.type.list).
        #[derive(Clone, Debug)]
        pub struct EVClient {
        }
        impl EVClient {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EVClient.get(1): not translated, it uses VisualTypeClient, which isn't translated (it's generic).
        // EVClient.register(2): not translated, it's generic.
        // EVClient.preInit(0): not translated, its body couldn't be read: an anonymous class.
        // EVClient.init(0): not translated, it calls EVClient.register, which isn't translated.
        // EVClient.shouldRender(0): not translated, it uses Minecraft's `Minecraft`.
        // EVClient.shouldTick(0): not translated, it uses Minecraft's `Minecraft`.
    }

    /// `EVManagerClient` (team/creative/enhancedvisuals/client/EVManagerClient.java)
    pub mod ev_manager_client {
        use super::*;
        // The class itself isn't translated: it implements `EVManager` (team.creative.enhancedvisuals). Its static parts are below.
        // EVManagerClient.respawn(0): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.clientTick(0): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.random(0): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.onTick(1): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.playSound(1): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.playSound(2): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.playSound(2): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.playSound(3): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.playSoundFadeOut(3): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.visuals(1): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.clearEverything(0): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.add(1): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.remove(1): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.playTicking(3): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.addVisualFadeOut(3): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.addVisualFadeOut(3): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.addVisualFadeOut(3): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.addParticlesFadeOut(5): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.addParticlesFadeOut(6): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.addParticlesFadeOut(5): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.addParticlesFadeOut(6): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.addParticle(4): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerClient.generateOffset(3): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
    }

    /// `GameRendererExtender` (team/creative/enhancedvisuals/client/mc/GameRendererExtender.java)
    pub mod game_renderer_extender {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // GameRendererExtender.getResourcePool(0): not translated, it has no body.
    }

    /// `EVRenderer` (team/creative/enhancedvisuals/client/render/EVRenderer.java)
    pub mod ev_renderer {
        use super::*;
        pub static LAST_RENDERED_MESSAGE: LazyLock<JCell<Option<String>>> = LazyLock::new(|| JCell::new(None));
        pub static FRAMEBUFFER_WIDTH: JCell<i32> = JCell::new(0i32);
        pub static FRAMEBUFFER_HEIGHT: JCell<i32> = JCell::new(0i32);
        pub static RELOAD_RESOURCES: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EVRenderer {
        }
        impl EVRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn init() {
        }
        // EVRenderer.renderShaders(1): not translated, it uses Minecraft's `DeltaTracker`.
        // EVRenderer.render(1): not translated, it uses `Object`.
        // EVRenderer.render(5): not translated, it uses Visual, which isn't translated (it uses VisualType, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api))).
        // EVRenderer.renderVisuals(5): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
    }

    /// `TextureCache` (team/creative/enhancedvisuals/client/render/TextureCache.java)
    pub mod texture_cache {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct TextureCache {
        }
        impl TextureCache {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // TextureCache.getFirst(0): not translated, it has no body.
            // TextureCache.getResource(0): not translated, it has no body.
        }
        // TextureCache.parse(3): not translated, it uses Minecraft's `ResourceManager`.
    }

    /// `TextureCacheAnimation` (team/creative/enhancedvisuals/client/render/TextureCacheAnimation.java)
    pub mod texture_cache_animation {
        use super::*;
        // The class itself isn't translated: it extends `TextureCache`. Its static parts are below.
        // TextureCacheAnimation.TextureCacheAnimation(2): not translated, its class isn't translated (it extends `TextureCache`).
        // TextureCacheAnimation.getFirst(0): not translated, its class isn't translated (it extends `TextureCache`).
        // TextureCacheAnimation.getResource(0): not translated, its class isn't translated (it extends `TextureCache`).
    }

    /// `TextureCacheSimple` (team/creative/enhancedvisuals/client/render/TextureCacheSimple.java)
    pub mod texture_cache_simple {
        use super::*;
        // The class itself isn't translated: it extends `TextureCache`. Its static parts are below.
        // TextureCacheSimple.TextureCacheSimple(1): not translated, its class isn't translated (it extends `TextureCache`).
        // TextureCacheSimple.getResource(0): not translated, its class isn't translated (it extends `TextureCache`).
        // TextureCacheSimple.getFirst(0): not translated, its class isn't translated (it extends `TextureCache`).
    }

    /// `PositionedSound` (team/creative/enhancedvisuals/client/sound/PositionedSound.java)
    pub mod positioned_sound {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractSoundInstance`. Its static parts are below.
        // PositionedSound.PositionedSound(4): not translated, its class isn't translated (it extends Minecraft's `AbstractSoundInstance`).
        // PositionedSound.PositionedSound(5): not translated, its class isn't translated (it extends Minecraft's `AbstractSoundInstance`).
    }

    /// `SoundMuteHandler` (team/creative/enhancedvisuals/client/sound/SoundMuteHandler.java)
    pub mod sound_mute_handler {
        use super::*;
        pub static IS_MUTING: JCell<bool> = JCell::new(false);
        // engine: not translated, it uses Minecraft's `SoundEngine`.
        // manager: not translated, it uses Minecraft's `SoundManager`.
        // muteGraph: not translated, it uses `DecimalCurve` (team.creative.creativecore.common.config.premade.curve).
        pub static TIME_TICK: JCell<i32> = JCell::new(0);
        #[derive(Clone, Debug)]
        pub struct SoundMuteHandler {
        }
        impl SoundMuteHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SoundMuteHandler.tick(0): not translated, it uses SoundMuteHandler.muteGraph, which isn't translated (it uses `DecimalCurve` (team.creative.creativecore.common.config.premade.curve)).
        // SoundMuteHandler.getClampedVolume(1): not translated, it uses SoundMuteHandler.muteGraph, which isn't translated (it uses `DecimalCurve` (team.creative.creativecore.common.config.premade.curve)).
        // SoundMuteHandler.getSounds(0): not translated, it uses Minecraft's `SoundInstance`.
        // SoundMuteHandler.setMuteVolume(1): not translated, it calls SoundMuteHandler.getSounds, which isn't translated.
        // SoundMuteHandler.startMuting(1): not translated, it uses `DecimalCurve` (team.creative.creativecore.common.config.premade.curve).
        // SoundMuteHandler.endMuting(0): not translated, it calls SoundMuteHandler.setMuteVolume, which isn't translated.
    }

    /// `TickedSound` (team/creative/enhancedvisuals/client/sound/TickedSound.java)
    pub mod ticked_sound {
        use super::*;
        // The class itself isn't translated: it extends `PositionedSound`. Its static parts are below.
        // TickedSound.TickedSound(4): not translated, its class isn't translated (it extends `PositionedSound`).
        // TickedSound.TickedSound(5): not translated, its class isn't translated (it extends `PositionedSound`).
        // TickedSound.isStopped(0): not translated, its class isn't translated (it extends `PositionedSound`).
        // TickedSound.stop(0): not translated, its class isn't translated (it extends `PositionedSound`).
        // TickedSound.tick(0): not translated, its class isn't translated (it extends `PositionedSound`).
    }

    /// `VisualTypeBlurClient` (team/creative/enhancedvisuals/client/type/VisualTypeBlurClient.java)
    pub mod visual_type_blur_client {
        use super::*;
        // The class itself isn't translated: it extends `VisualTypeShaderClient`. Its static parts are below.
        // UNIFORM_SIZE: not translated, it creates Minecraft's `Std140SizeCalculator`.
        // VisualTypeBlurClient.VisualTypeBlurClient(1): not translated, its class isn't translated (it extends `VisualTypeShaderClient`).
        // VisualTypeBlurClient.changeProperties(1): not translated, its class isn't translated (it extends `VisualTypeShaderClient`).
    }

    /// `VisualTypeClient` (team/creative/enhancedvisuals/client/type/VisualTypeClient.java)
    pub mod visual_type_client {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // VisualTypeClient.VisualTypeClient(1): not translated, its class isn't translated (it's generic).
        // VisualTypeClient.loadResources(1): not translated, it has no body.
        // VisualTypeClient.render(6): not translated, it has no body.
        // VisualTypeClient.getVariantAmount(0): not translated, its class isn't translated (it's generic).
        // VisualTypeClient.resize(1): not translated, its class isn't translated (it's generic).
        // VisualTypeClient.getWidth(2): not translated, its class isn't translated (it's generic).
        // VisualTypeClient.getHeight(2): not translated, its class isn't translated (it's generic).
    }

    /// `VisualTypeFocusClient` (team/creative/enhancedvisuals/client/type/VisualTypeFocusClient.java)
    pub mod visual_type_focus_client {
        use super::*;
        // The class itself isn't translated: it extends `VisualTypeShaderClient`. Its static parts are below.
        // UNIFORM_SIZE: not translated, it creates Minecraft's `Std140SizeCalculator`.
        // VisualTypeFocusClient.VisualTypeFocusClient(1): not translated, its class isn't translated (it extends `VisualTypeShaderClient`).
        // VisualTypeFocusClient.changeProperties(1): not translated, its class isn't translated (it extends `VisualTypeShaderClient`).
    }

    /// `VisualTypeSaturationClient` (team/creative/enhancedvisuals/client/type/VisualTypeSaturationClient.java)
    pub mod visual_type_saturation_client {
        use super::*;
        // The class itself isn't translated: it extends `VisualTypeShaderClient`. Its static parts are below.
        // UNIFORM_SIZE: not translated, it creates Minecraft's `Std140SizeCalculator`.
        // VisualTypeSaturationClient.VisualTypeSaturationClient(1): not translated, its class isn't translated (it extends `VisualTypeShaderClient`).
        // VisualTypeSaturationClient.changeProperties(1): not translated, its class isn't translated (it extends `VisualTypeShaderClient`).
    }

    /// `VisualTypeShaderClient` (team/creative/enhancedvisuals/client/type/VisualTypeShaderClient.java)
    pub mod visual_type_shader_client {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // VisualTypeShaderClient.VisualTypeShaderClient(1): not translated, its class isn't translated (it's generic).
        // VisualTypeShaderClient.loadResources(1): not translated, its class isn't translated (it's generic).
        // VisualTypeShaderClient.getVariantAmount(0): not translated, its class isn't translated (it's generic).
        // VisualTypeShaderClient.resize(1): not translated, its class isn't translated (it's generic).
        // VisualTypeShaderClient.render(6): not translated, its class isn't translated (it's generic).
        // VisualTypeShaderClient.changeProperties(1): not translated, it has no body.
    }

    /// `VisualTypeTextureClient` (team/creative/enhancedvisuals/client/type/VisualTypeTextureClient.java)
    pub mod visual_type_texture_client {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const DEFAULT_PARTICLE_SIZE: f32 = 0.15f32;
        // VisualTypeTextureClient.VisualTypeTextureClient(1): not translated, its class isn't translated (it's generic).
        // VisualTypeTextureClient.loadResources(1): not translated, its class isn't translated (it's generic).
        // VisualTypeTextureClient.getVariantAmount(0): not translated, its class isn't translated (it's generic).
        // VisualTypeTextureClient.getResource(1): not translated, its class isn't translated (it's generic).
        // VisualTypeTextureClient.render(6): not translated, its class isn't translated (it's generic).
        // VisualTypeTextureClient.getWidth(2): not translated, its class isn't translated (it's generic).
        // VisualTypeTextureClient.getHeight(2): not translated, its class isn't translated (it's generic).
    }

    /// `ColdSweatAddon` (team/creative/enhancedvisuals/common/addon/coldsweat/ColdSweatAddon.java)
    pub mod cold_sweat_addon {
        use super::*;
        // temperature: not translated, it uses TemperatureHandler, which isn't translated (another class has the same name, in a different package).
        #[derive(Clone, Debug)]
        pub struct ColdSweatAddon {
        }
        impl ColdSweatAddon {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ColdSweatAddon.load(0): not translated, it uses VisualHandler, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
    }

    /// `TemperatureHandler` (team/creative/enhancedvisuals/common/addon/coldsweat/TemperatureHandler.java)
    pub mod temperature_handler {
        use super::*;
        // The class itself isn't translated: another class has the same name, in a different package. Its static parts are below.
        // TemperatureHandler.getTemperature(1): not translated, its class isn't translated (another class has the same name, in a different package).
        // TemperatureHandler.tick(1): not translated, its class isn't translated (another class has the same name, in a different package).
    }

    /// `SurviveAddon` (team/creative/enhancedvisuals/common/addon/survive/SurviveAddon.java)
    pub mod survive_addon {
        use super::*;
        // thirst: not translated, it uses ThirstHandler, which isn't translated (another class has the same name, in a different package).
        // temperature: not translated, it uses TemperatureHandler, which isn't translated (another class has the same name, in a different package).
        #[derive(Clone, Debug)]
        pub struct SurviveAddon {
        }
        impl SurviveAddon {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SurviveAddon.load(0): not translated, it uses VisualHandler, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
    }

    /// `TemperatureHandler` (team/creative/enhancedvisuals/common/addon/survive/TemperatureHandler.java)
    pub mod temperature_handler_2 {
        use super::*;
        // The class itself isn't translated: another class has the same name, in a different package. Its static parts are below.
        // TemperatureHandler.getTemperature(1): not translated, its class isn't translated (another class has the same name, in a different package).
        // TemperatureHandler.tick(1): not translated, its class isn't translated (another class has the same name, in a different package).
    }

    /// `ThirstHandler` (team/creative/enhancedvisuals/common/addon/survive/ThirstHandler.java)
    pub mod thirst_handler {
        use super::*;
        // The class itself isn't translated: another class has the same name, in a different package. Its static parts are below.
        // ThirstHandler.getThirst(1): not translated, its class isn't translated (another class has the same name, in a different package).
        // ThirstHandler.tick(1): not translated, its class isn't translated (another class has the same name, in a different package).
    }

    /// `TANAddon` (team/creative/enhancedvisuals/common/addon/toughasnails/TANAddon.java)
    pub mod tan_addon {
        use super::*;
        // thirst: not translated, it uses ThirstHandler, which isn't translated (another class has the same name, in a different package).
        // temperature: not translated, it uses TemperatureHandler, which isn't translated (another class has the same name, in a different package).
        #[derive(Clone, Debug)]
        pub struct TANAddon {
        }
        impl TANAddon {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // TANAddon.load(0): not translated, it uses VisualHandler, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
    }

    /// `TemperatureHandler` (team/creative/enhancedvisuals/common/addon/toughasnails/TemperatureHandler.java)
    pub mod temperature_handler_3 {
        use super::*;
        // The class itself isn't translated: another class has the same name, in a different package. Its static parts are below.
        // TemperatureHandler.tick(1): not translated, its class isn't translated (another class has the same name, in a different package).
    }

    /// `ThirstHandler` (team/creative/enhancedvisuals/common/addon/toughasnails/ThirstHandler.java)
    pub mod thirst_handler_2 {
        use super::*;
        // The class itself isn't translated: another class has the same name, in a different package. Its static parts are below.
        // ThirstHandler.tick(1): not translated, its class isn't translated (another class has the same name, in a different package).
    }

    /// `DeathMessages` (team/creative/enhancedvisuals/common/death/DeathMessages.java)
    pub mod death_messages {
        use super::*;
        // The class itself isn't translated: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api). Its static parts are below.
        // DeathMessages.pickRandomDeathMessage(0): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // DeathMessages.configured(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
    }

    /// `EVEvents` (team/creative/enhancedvisuals/common/event/EVEvents.java)
    pub mod ev_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EVEvents {
        }
        impl EVEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // EVEvents.explosion(2): not translated, it uses Minecraft's `Explosion`.
            // EVEvents.impact(1): not translated, it uses Minecraft's `Projectile`.
            // EVEvents.damage(3): not translated, it uses Minecraft's `Player`.
        }
        // EVEvents.areEyesInWater(1): not translated, it uses Minecraft's `Player`.
    }

    /// `DamageHandler` (team/creative/enhancedvisuals/common/handler/DamageHandler.java)
    pub mod damage_handler {
        use super::*;
        // The class itself isn't translated: it extends `VisualHandler` (team.creative.enhancedvisuals.api). Its static parts are below.
        // sharpList: a game value, so it's in game_side.rs.
        // bluntList: a game value, so it's in game_side.rs.
        // pierceList: a game value, so it's in game_side.rs.
        // DamageHandler.playerDamaged(3): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // DamageHandler.createVisualFromDamageAndDistance(4): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // DamageHandler.isSharp(1): not translated, it uses Minecraft's `ItemStack`.
        // DamageHandler.isBlunt(1): not translated, it uses Minecraft's `ItemStack`.
        // DamageHandler.isPierce(1): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `ExplosionHandler` (team/creative/enhancedvisuals/common/handler/ExplosionHandler.java)
    pub mod explosion_handler {
        use super::*;
        // The class itself isn't translated: it extends `VisualHandler` (team.creative.enhancedvisuals.api). Its static parts are below.
        // ExplosionHandler.isMace(2): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // ExplosionHandler.onExploded(6): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
    }

    /// `HealthHandler` (team/creative/enhancedvisuals/common/handler/HealthHandler.java)
    pub mod health_handler {
        use super::*;
        // The class itself isn't translated: it extends `VisualHandler` (team.creative.enhancedvisuals.api). Its static parts are below.
        // HealthHandler.HealthHandler(0): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // HealthHandler.tick(1): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
    }

    /// `HeartbeatHandler` (team/creative/enhancedvisuals/common/handler/HeartbeatHandler.java)
    pub mod heartbeat_handler {
        use super::*;
        // The class itself isn't translated: it extends `VisualHandler` (team.creative.enhancedvisuals.api). Its static parts are below.
        // HeartbeatHandler.tick(1): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // HeartbeatHandler.resetBufferTicks(1): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // HeartbeatHandler.getIntensity(1): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // HeartbeatHandler.shouldHeartbeatTrigger(1): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
    }

    /// `PotionHandler` (team/creative/enhancedvisuals/common/handler/PotionHandler.java)
    pub mod potion_handler {
        use super::*;
        // The class itself isn't translated: it extends `VisualHandler` (team.creative.enhancedvisuals.api). Its static parts are below.
        // PotionHandler.impact(2): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
    }

    /// `RainHandler` (team/creative/enhancedvisuals/common/handler/RainHandler.java)
    pub mod rain_handler {
        use super::*;
        // The class itself isn't translated: it extends `VisualHandler` (team.creative.enhancedvisuals.api). Its static parts are below.
        // RainHandler.tick(1): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // RainHandler.updateDelay(1): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // RainHandler.configured(1): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
    }

    /// `SandSplatHandler` (team/creative/enhancedvisuals/common/handler/SandSplatHandler.java)
    pub mod sand_splat_handler {
        use super::*;
        // The class itself isn't translated: it extends `VisualHandler` (team.creative.enhancedvisuals.api). Its static parts are below.
        // SandSplatHandler.tick(1): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // SandSplatHandler.isOnSand(1): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
    }

    /// `SaturationHandler` (team/creative/enhancedvisuals/common/handler/SaturationHandler.java)
    pub mod saturation_handler {
        use super::*;
        // The class itself isn't translated: it extends `VisualHandler` (team.creative.enhancedvisuals.api). Its static parts are below.
        // SaturationHandler.tick(1): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
    }

    /// `SlenderHandler` (team/creative/enhancedvisuals/common/handler/SlenderHandler.java)
    pub mod slender_handler {
        use super::*;
        // The class itself isn't translated: it extends `VisualHandler` (team.creative.enhancedvisuals.api). Its static parts are below.
        // SlenderHandler.loadMutantEnderman(0): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // SlenderHandler.tick(1): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
    }

    /// `SplashHandler` (team/creative/enhancedvisuals/common/handler/SplashHandler.java)
    pub mod splash_handler {
        use super::*;
        // The class itself isn't translated: it extends `VisualHandler` (team.creative.enhancedvisuals.api). Its static parts are below.
        // SplashHandler.tick(1): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
    }

    /// `UnderwaterHandler` (team/creative/enhancedvisuals/common/handler/UnderwaterHandler.java)
    pub mod underwater_handler {
        use super::*;
        // The class itself isn't translated: it extends `VisualHandler` (team.creative.enhancedvisuals.api). Its static parts are below.
        // UnderwaterHandler.UnderwaterHandler(0): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // UnderwaterHandler.tick(1): not translated, its class isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
    }

    /// `VisualHandlers` (team/creative/enhancedvisuals/common/handler/VisualHandlers.java)
    pub mod visual_handlers {
        use super::*;
        // EXPLOSION: not translated, it uses ExplosionHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // POTION: not translated, it uses PotionHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // SAND: not translated, it uses SandSplatHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // SPLASH: not translated, it uses SplashHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // DAMAGE: not translated, it uses DamageHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // SLENDER: not translated, it uses SlenderHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // SATURATION: not translated, it uses SaturationHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // HEARTBEAT: not translated, it uses HeartbeatHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // UNDERWATER: not translated, it uses UnderwaterHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // RAIN: not translated, it uses RainHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        // HEALTH: not translated, it uses HealthHandler, which isn't translated (it extends `VisualHandler` (team.creative.enhancedvisuals.api)).
        #[derive(Clone, Debug)]
        pub struct VisualHandlers {
        }
        impl VisualHandlers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // VisualHandlers.init(0): not translated, it uses VisualHandler, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
    }

    /// `DamagePacket` (team/creative/enhancedvisuals/common/packet/DamagePacket.java)
    pub mod damage_packet {
        use super::*;
        // The class itself isn't translated: it extends `CreativePacket` (team.creative.creativecore.common.network). Its static parts are below.
        // DamagePacket.DamagePacket(3): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // DamagePacket.DamagePacket(0): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // DamagePacket.getSource(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // DamagePacket.executeClient(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // DamagePacket.executeServer(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
    }

    /// `ExplosionPacket` (team/creative/enhancedvisuals/common/packet/ExplosionPacket.java)
    pub mod explosion_packet {
        use super::*;
        // The class itself isn't translated: it extends `CreativePacket` (team.creative.creativecore.common.network). Its static parts are below.
        // ExplosionPacket.ExplosionPacket(5): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ExplosionPacket.ExplosionPacket(0): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ExplosionPacket.executeClient(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // ExplosionPacket.executeServer(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
    }

    /// `PotionPacket` (team/creative/enhancedvisuals/common/packet/PotionPacket.java)
    pub mod potion_packet {
        use super::*;
        // The class itself isn't translated: it extends `CreativePacket` (team.creative.creativecore.common.network). Its static parts are below.
        // PotionPacket.PotionPacket(0): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // PotionPacket.PotionPacket(2): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // PotionPacket.executeClient(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
        // PotionPacket.executeServer(1): not translated, its class isn't translated (it extends `CreativePacket` (team.creative.creativecore.common.network)).
    }

    /// `VisualRegistry` (team/creative/enhancedvisuals/common/visual/VisualRegistry.java)
    pub mod visual_registry {
        use super::*;
        // handlers: not translated, it uses VisualHandler, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        #[derive(Clone, Debug)]
        pub struct VisualRegistry {
        }
        impl VisualRegistry {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // VisualRegistry.registerHandler(2): not translated, it uses VisualHandler, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualRegistry.handlers(0): not translated, it uses VisualHandler, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // VisualRegistry.entrySet(0): not translated, it uses VisualHandler, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
    }

    /// `EnhancedVisuals` (team/creative/enhancedvisuals/EnhancedVisuals.java)
    pub mod enhanced_visuals {
        use super::*;
        // The class itself isn't translated: it implements `CommonLoader` (team.creative.creativecore.common). Its static parts are below.
        pub const MODID: &str = "enhancedvisuals";
        // LOGGER: the logger. Log calls become comments.
        // NETWORK: not translated, it uses `CreativeNetwork` (team.creative.creativecore.common.network).
        pub static EVENTS: LazyLock<JCell<Option<ev_events::EVEvents>>> = LazyLock::new(|| JCell::new(None));
        // MESSAGES: not translated, it uses DeathMessages, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // CONFIG: not translated, it uses EnhancedVisualsConfig, which isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
        // MANAGER: not translated, it uses EVManager, which isn't translated (it's an interface).
        // EnhancedVisuals.EnhancedVisuals(0): not translated, its class isn't translated (it implements `CommonLoader` (team.creative.creativecore.common)).
        // EnhancedVisuals.onInitializeClient(0): not translated, its class isn't translated (it implements `CommonLoader` (team.creative.creativecore.common)).
        // EnhancedVisuals.onInitialize(0): not translated, its class isn't translated (it implements `CommonLoader` (team.creative.creativecore.common)).
    }

    /// `EnhancedVisualsConfig` (team/creative/enhancedvisuals/EnhancedVisualsConfig.java)
    pub mod enhanced_visuals_config {
        use super::*;
        // The class itself isn't translated: it implements `ICreativeConfig` (team.creative.creativecore.common.config.api). Its static parts are below.
        // EnhancedVisualsConfig.configured(1): not translated, its class isn't translated (it implements `ICreativeConfig` (team.creative.creativecore.common.config.api)).
    }

    /// `EVManager` (team/creative/enhancedvisuals/EVManager.java)
    pub mod ev_manager {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // EVManager.random(0): not translated, it has no body.
        // EVManager.playSound(1): not translated, it has no body.
        // EVManager.playSound(2): not translated, it has no body.
        // EVManager.playSound(2): not translated, it has no body.
        // EVManager.playSound(3): not translated, it has no body.
        // EVManager.playSoundFadeOut(3): not translated, it has no body.
        // EVManager.visuals(1): not translated, it has no body.
        // EVManager.clearEverything(0): not translated, it has no body.
        // EVManager.add(1): not translated, it has no body.
        // EVManager.remove(1): not translated, it has no body.
        // EVManager.playTicking(3): not translated, it has no body.
        // EVManager.addVisualFadeOut(3): not translated, it has no body.
        // EVManager.addVisualFadeOut(3): not translated, it has no body.
        // EVManager.addVisualFadeOut(3): not translated, it has no body.
        // EVManager.addParticlesFadeOut(5): not translated, it has no body.
        // EVManager.addParticlesFadeOut(6): not translated, it has no body.
        // EVManager.addParticlesFadeOut(5): not translated, it has no body.
        // EVManager.addParticlesFadeOut(6): not translated, it has no body.
        // EVManager.addParticle(4): not translated, it has no body.
        // EVManager.generateOffset(3): not translated, it has no body.
    }

    /// `ClearVisualCommand` (team/creative/enhancedvisuals/server/command/ClearVisualCommand.java)
    pub mod clear_visual_command {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClearVisualCommand {
        }
        impl ClearVisualCommand {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `DisplayVisualCommand` (team/creative/enhancedvisuals/server/command/DisplayVisualCommand.java)
    pub mod display_visual_command {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DisplayVisualCommand {
        }
        impl DisplayVisualCommand {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EVManagerServer` (team/creative/enhancedvisuals/server/EVManagerServer.java)
    pub mod ev_manager_server {
        use super::*;
        // The class itself isn't translated: it implements `EVManager` (team.creative.enhancedvisuals). Its static parts are below.
        // EVManagerServer.playSound(1): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.playSound(2): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.playSound(2): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.playSound(3): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.playSoundFadeOut(3): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.visuals(1): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.clearEverything(0): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.add(1): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.remove(1): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.playTicking(3): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.addVisualFadeOut(3): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.addVisualFadeOut(3): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.addVisualFadeOut(3): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.addParticlesFadeOut(5): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.addParticlesFadeOut(6): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.addParticlesFadeOut(5): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.addParticlesFadeOut(6): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.addParticle(4): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.generateOffset(3): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
        // EVManagerServer.random(0): not translated, its class isn't translated (it implements `EVManager` (team.creative.enhancedvisuals)).
    }

    /// `EVServer` (team/creative/enhancedvisuals/server/EVServer.java)
    pub mod ev_server {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EVServer {
        }
        impl EVServer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "enhancedvisuals");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
