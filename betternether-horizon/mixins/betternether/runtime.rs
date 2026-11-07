// Better Nether, ported to Horizon by horizon-port from better-nether-26.302.1.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_betternether). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "betternether";
pub const NAME: &str = "Better Nether";
pub const VERSION: &str = "26.302.1";
pub const DESCRIPTION: &str = "Mod that improves Nether";

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

    /// `BNCriterion` (org/betterx/betternether/advancements/BNCriterion.java)
    pub mod bn_criterion {
        use super::*;
        // BREW_BLUE_ID: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // USED_FORGE_ID: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // DISTURBED_WISP_ID: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // BURNED_GLOOMSCULK_CRYSTAL_ID: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // WISP_SHED_EXPERIENCE_ID: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // BREW_BLUE: not translated, it uses Minecraft's `PlayerTrigger`.
        // USED_FORGE: not translated, it uses Minecraft's `PlayerTrigger`.
        // DISTURBED_WISP: not translated, it uses Minecraft's `PlayerTrigger`.
        // BURNED_GLOOMSCULK_CRYSTAL: not translated, it uses Minecraft's `PlayerTrigger`.
        // WISP_SHED_EXPERIENCE: not translated, it uses Minecraft's `PlayerTrigger`.
        // CONVERT_BY_LIGHTNING: not translated, it uses ConvertByLightningTrigger, which isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // BREW_BLUE_CRITERION: not translated, it uses Minecraft's `Criterion`.
        // USED_FORGE_ANY_CRITERION: not translated, it uses Minecraft's `Criterion`.
        // DISTURBED_WISP_CRITERION: not translated, it uses Minecraft's `Criterion`.
        // BURNED_GLOOMSCULK_CRYSTAL_CRITERION: not translated, it uses Minecraft's `Criterion`.
        // WISP_SHED_EXPERIENCE_CRITERION: not translated, it uses Minecraft's `Criterion`.
        #[derive(Clone, Debug)]
        pub struct BNCriterion {
        }
        impl BNCriterion {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BNCriterion.register(1): not translated, it's generic.
        // BNCriterion.register(2): not translated, it's generic.
        // BNCriterion.register(0): not translated, it uses BNCriterion.BREW_BLUE, which isn't translated (it uses Minecraft's `PlayerTrigger`).
    }

    /// `BNCriterion.TriggerWithID` (org/betterx/betternether/advancements/BNCriterion.java)
    pub mod bn_criterion_trigger_with_id {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BNCriterion.TriggerWithID.getId(0): not translated, it has no body.
    }

    /// `ConvertByLightningTrigger` (org/betterx/betternether/advancements/ConvertByLightningTrigger.java)
    pub mod convert_by_lightning_trigger {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleCriterionTrigger`. Its static parts are below.
        // ID: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // ConvertByLightningTrigger.getId(0): not translated, its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // ConvertByLightningTrigger.trigger(2): not translated, its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // ConvertByLightningTrigger.match(1): not translated, its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
        // ConvertByLightningTrigger.codec(0): not translated, its class isn't translated (it extends Minecraft's `SimpleCriterionTrigger`).
    }

    /// `ConvertByLightningTrigger.TriggerInstance` (org/betterx/betternether/advancements/ConvertByLightningTrigger.java)
    pub mod convert_by_lightning_trigger_trigger_instance {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `SimpleInstance`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // ConvertByLightningTrigger.TriggerInstance.TriggerInstance(1): not translated, its class isn't translated (it implements Minecraft's `SimpleInstance`).
        // ConvertByLightningTrigger.TriggerInstance.matches(1): not translated, its class isn't translated (it implements Minecraft's `SimpleInstance`).
        // ConvertByLightningTrigger.TriggerInstance.validate(1): not translated, its class isn't translated (it implements Minecraft's `SimpleInstance`).
        // ConvertByLightningTrigger.TriggerInstance.player(0): not translated, its class isn't translated (it implements Minecraft's `SimpleInstance`).
    }

    /// `BetterNether` (org/betterx/betternether/BetterNether.java)
    pub mod better_nether {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModInitializer`. Its static parts are below.
        // C: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // VANILLA_HAMMERS: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // VANILLA_EXCAVATORS: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // GLIMMERWILD: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // VANILLA_HAMMERS_PACK: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // VANILLA_EXCAVATORS_PACK: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // GLIMMERWILD_PACK: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        pub static LAVAFALL_PARTICLES: JCell<bool> = JCell::new(true);
        pub static FOG_START: JCell<f32> = JCell::new(0.05f32);
        pub static FOG_END: JCell<f32> = JCell::new(0.5f32);
        // BetterNether.onDatagen(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
        // BetterNether.onInitialize(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
        // BetterNether.initOptions(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
        // BetterNether.warnAboutLegacyConfigs(0): not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        pub fn collect_disabled(element: Option<JJson>, disabled: &mut Vec<String>) {
            if element.is_some() && ((element.as_ref().expect("null")).is_json_object()) {
                for e in (element.as_ref().expect("null")).entries() {
                    let value = &e.1;
                    if value.is_json_object() {
                        better_nether::collect_disabled(Some(value.clone()), &mut *disabled);
                    } else if (value.is_json_primitive() && (value.clone().is_boolean())) && (!value.as_bool()) {
                        disabled.push({ static RE: LazyLock<JRegex> = LazyLock::new(|| JRegex::new("\\[def: .*]$")); &*RE }.replace_first(e.0, ""));
                    }
                }
            }
        }
        pub fn has_lavafall_particles() -> bool {
            return better_nether::LAVAFALL_PARTICLES.get();
        }
        pub fn change_fog_density(density: f32) {
            better_nether::FOG_START.set(((-0.45f32) * density) + 0.5f32);
            better_nether::FOG_END.set(((-0.5f32) * density) + 1.0f32);
        }
        pub fn get_fog_start() -> f32 {
            return better_nether::FOG_START.get();
        }
        pub fn get_fog_end() -> f32 {
            return better_nether::FOG_END.get();
        }
    }

    /// `BlockEntityChestOfDrawers` (org/betterx/betternether/blockentities/BlockEntityChestOfDrawers.java)
    pub mod block_entity_chest_of_drawers {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RandomizableContainerBlockEntity`. Its static parts are below.
        // BlockEntityChestOfDrawers.BlockEntityChestOfDrawers(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BlockEntityChestOfDrawers.getContainerSize(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BlockEntityChestOfDrawers.getItems(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BlockEntityChestOfDrawers.setItems(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BlockEntityChestOfDrawers.getDefaultName(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BlockEntityChestOfDrawers.createMenu(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BlockEntityChestOfDrawers.loadAdditional(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BlockEntityChestOfDrawers.saveAdditional(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BlockEntityChestOfDrawers.onInvOpen(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BlockEntityChestOfDrawers.stopOpen(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BlockEntityChestOfDrawers.onInvOpenOrClose(0): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BlockEntityChestOfDrawers.playSound(2): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
        // BlockEntityChestOfDrawers.addItemsToList(1): not translated, its class isn't translated (it extends Minecraft's `RandomizableContainerBlockEntity`).
    }

    /// `BlockEntityForge` (org/betterx/betternether/blockentities/BlockEntityForge.java)
    pub mod block_entity_forge {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractFurnaceBlockEntity`. Its static parts are below.
        pub const SPEEDUP: i32 = 2;
        // BlockEntityForge.BlockEntityForge(2): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`).
        // BlockEntityForge.getDefaultName(0): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`).
        // BlockEntityForge.createMenu(2): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`).
        // BlockEntityForge.getBurnDuration(2): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`).
        // BlockEntityForge.changeCookTime(1): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`).
        // BlockEntityForge.awardUsedRecipesAndPopExperience(1): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`).
    }

    /// `BlockEntityFurnace` (org/betterx/betternether/blockentities/BlockEntityFurnace.java)
    pub mod block_entity_furnace {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractFurnaceBlockEntity`. Its static parts are below.
        // BlockEntityFurnace.BlockEntityFurnace(2): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`).
        // BlockEntityFurnace.getDefaultName(0): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`).
        // BlockEntityFurnace.createMenu(2): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlockEntity`).
    }

    /// `BNBrewingStandBlockEntity` (org/betterx/betternether/blockentities/BNBrewingStandBlockEntity.java)
    pub mod bn_brewing_stand_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BaseContainerBlockEntity`. Its static parts are below.
        pub static TOP_SLOTS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![3]);
        pub static BOTTOM_SLOTS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0, 1, 2, 3]);
        pub static SIDE_SLOTS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0, 1, 2, 4]);
        // BNBrewingStandBlockEntity.BNBrewingStandBlockEntity(2): not translated, its body couldn't be read: an anonymous class.
        // BNBrewingStandBlockEntity.getItems(0): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.setItems(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.getDefaultName(0): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.getContainerSize(0): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.isEmpty(0): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.tick(4): not translated, it uses Minecraft's `Level`.
        // BNBrewingStandBlockEntity.getSlotsEmpty(0): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.isBrewingReagent(2): not translated, it uses Minecraft's `Level`.
        // BNBrewingStandBlockEntity.canCraft(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.craft(0): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.craft(3): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.getItem(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.removeItem(2): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.removeItemNoUpdate(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.setItem(2): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.stillValid(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.canPlaceItem(2): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.getSlotsForFace(1): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.canPlaceItemThroughFace(3): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.canTakeItemThroughFace(3): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.clearContent(0): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
        // BNBrewingStandBlockEntity.createMenu(2): not translated, its class isn't translated (it extends Minecraft's `BaseContainerBlockEntity`).
    }

    /// `ChangebleCookTime` (org/betterx/betternether/blockentities/ChangebleCookTime.java)
    pub mod changeble_cook_time {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChangebleCookTime.changeCookTime(1): not translated, it has no body.
    }

    /// `BlocksHelper` (org/betterx/betternether/BlocksHelper.java)
    pub mod blocks_helper {
        use super::*;
        pub const FLAG_UPDATE_BLOCK: i32 = 1;
        pub const FLAG_SEND_CLIENT_CHANGES: i32 = 2;
        pub const FLAG_NO_RERENDER: i32 = 4;
        pub const FORSE_RERENDER: i32 = 8;
        pub const FLAG_IGNORE_OBSERVERS: i32 = 16;
        pub const SET_SILENT: i32 = 18;
        pub const SET_UPDATE: i32 = 3;
        // HORIZONTAL: not translated, it uses Minecraft's `Direction`.
        // OFFSETS: not translated, it uses Minecraft's `Vec3i`.
        #[derive(Clone, Debug)]
        pub struct BlocksHelper {
        }
        impl BlocksHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BlocksHelper.chunkBounds(2): not translated, it uses Minecraft's `BoundingBox`.
        // BlocksHelper.chunkBounds(4): not translated, it uses Minecraft's `BoundingBox`.
        // BlocksHelper.decorationBounds(2): not translated, it uses Minecraft's `BoundingBox`.
        // BlocksHelper.decorationBounds(4): not translated, it uses Minecraft's `BoundingBox`.
        // BlocksHelper.isInsideHorizontally(3): not translated, it uses Minecraft's `BoundingBox`.
        // BlocksHelper.isInsideHorizontally(2): not translated, it uses Minecraft's `BoundingBox`.
        // BlocksHelper.isNetherrack(1): not translated, it uses Minecraft's `BlockState`.
        // BlocksHelper.isSoulSand(1): not translated, it uses Minecraft's `BlockState`.
        // BlocksHelper.isNetherGround(1): not translated, it uses Minecraft's `BlockState`.
        // BlocksHelper.isSculkLike(1): not translated, it uses Minecraft's `BlockState`.
        // BlocksHelper.isNetherGroundMagma(1): not translated, it uses Minecraft's `BlockState`.
        // BlocksHelper.isNetherMycelium(1): not translated, it uses Minecraft's `BlockState`.
        // BlocksHelper.setWithUpdate(4): not translated, it uses Minecraft's `LevelAccessor`.
        // BlocksHelper.setWithUpdate(3): not translated, it uses Minecraft's `LevelAccessor`.
        // BlocksHelper.setWithoutUpdate(4): not translated, it uses Minecraft's `LevelAccessor`.
        // BlocksHelper.setWithoutUpdate(3): not translated, it uses Minecraft's `LevelAccessor`.
        // BlocksHelper.upRay(3): not translated, it uses Minecraft's `LevelAccessor`.
        // BlocksHelper.downRay(3): not translated, it uses Minecraft's `LevelAccessor`.
        // BlocksHelper.rotateHorizontal(3): not translated, it uses Minecraft's `BlockState`.
        // BlocksHelper.mirrorHorizontal(3): not translated, it uses Minecraft's `BlockState`.
        // BlocksHelper.getLengthDown(3): not translated, it uses Minecraft's `ServerLevel`.
        // BlocksHelper.isFertile(1): not translated, it uses Minecraft's `BlockState`.
        // BlocksHelper.cover(6): not translated, it uses Minecraft's `LevelAccessor`.
        // BlocksHelper.isNylium(1): not translated, it uses Minecraft's `BlockState`.
        // BlocksHelper.createLogIfFree(5): not translated, it uses Minecraft's `LevelAccessor`.
    }

    /// `BaseBlockCommonSapling` (org/betterx/betternether/blocks/BaseBlockCommonSapling.java)
    pub mod base_block_common_sapling {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BaseBlockCommonSapling.BaseBlockCommonSapling(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BaseBlockCommonSapling.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BaseBlockCommonSapling.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BaseBlockCommonSapling.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BaseBlockCommonSapling.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BaseBlockCommonSapling.canGrowTerrain(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BaseBlockCommonSapling.grow(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BaseBlockCommonSapling.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BaseBlockCommonSapling.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BaseBlockMold` (org/betterx/betternether/blocks/BaseBlockMold.java)
    pub mod base_block_mold {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // BaseBlockMold.BaseBlockMold(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BaseBlockMold.getShadeBrightness(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BaseBlockMold.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BaseBlockMold.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BaseBlockNetherGrass` (org/betterx/betternether/blocks/BaseBlockNetherGrass.java)
    pub mod base_block_nether_grass {
        use super::*;
        // The class itself isn't translated: it extends `BasePlantBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BaseBlockNetherGrass.BaseBlockNetherGrass(1): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // BaseBlockNetherGrass.getShadeBrightness(3): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // BaseBlockNetherGrass.getShape(4): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
        // BaseBlockNetherGrass.updateShape(8): not translated, its class isn't translated (it extends `BasePlantBlock` (org.betterx.bclib.blocks)).
    }

    /// `BlockAgave` (org/betterx/betternether/blocks/BlockAgave.java)
    pub mod block_agave {
        use super::*;
        // The class itself isn't translated: it extends `BlockCommonPlant`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockAgave.BlockAgave(1): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockAgave.getShape(4): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockAgave.updateShape(8): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockAgave.entityInside(6): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockAgave.canSurvive(3): not translated, its class isn't translated (it extends `BlockCommonPlant`).
    }

    /// `BlockAnchorTreeSapling` (org/betterx/betternether/blocks/BlockAnchorTreeSapling.java)
    pub mod block_anchor_tree_sapling {
        use super::*;
        // The class itself isn't translated: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api). Its static parts are below.
        // HANGING: not translated, it uses Minecraft's `BooleanProperty`.
        // BlockAnchorTreeSapling.BlockAnchorTreeSapling(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockAnchorTreeSapling.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockAnchorTreeSapling.supports(2): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockAnchorTreeSapling.getStateForPlacement(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockAnchorTreeSapling.canSurvive(3): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockAnchorTreeSapling.getShape(4): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockAnchorTreeSapling.megaFeatureEnabled(3): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockAnchorTreeSapling.smallFeatureEnabled(2): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockAnchorTreeSapling.mayPlaceOn(3): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
    }

    /// `BlockAnchorTreeVine` (org/betterx/betternether/blocks/BlockAnchorTreeVine.java)
    pub mod block_anchor_tree_vine {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE_SELECTION: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // BlockAnchorTreeVine.BlockAnchorTreeVine(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockAnchorTreeVine.getLuminance(1): not translated, it uses Minecraft's `BlockState`.
        // BlockAnchorTreeVine.getOffsetType(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockAnchorTreeVine.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockAnchorTreeVine.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockAnchorTreeVine.getShadeBrightness(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockAnchorTreeVine.propagatesSkylightDown(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockAnchorTreeVine.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockAnchorTreeVine.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockBarrelCactus` (org/betterx/betternether/blocks/BlockBarrelCactus.java)
    pub mod block_barrel_cactus {
        use super::*;
        // The class itself isn't translated: it extends `BlockCommonPlant`. Its static parts are below.
        // EMPTY: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPES: not translated, it uses Minecraft's `VoxelShape`.
        // BlockBarrelCactus.BlockBarrelCactus(1): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockBarrelCactus.canSurvive(3): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockBarrelCactus.updateShape(8): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockBarrelCactus.entityInside(6): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockBarrelCactus.getShape(4): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockBarrelCactus.getCollisionShape(4): not translated, its class isn't translated (it extends `BlockCommonPlant`).
    }

    /// `BlockBlackApple` (org/betterx/betternether/blocks/BlockBlackApple.java)
    pub mod block_black_apple {
        use super::*;
        // The class itself isn't translated: it extends `BlockCommonPlant`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockBlackApple.BlockBlackApple(1): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockBlackApple.getShape(4): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockBlackApple.getCloneItemStack(4): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockBlackApple.canSurvive(3): not translated, its class isn't translated (it extends `BlockCommonPlant`).
    }

    /// `BlockBlackAppleSeed` (org/betterx/betternether/blocks/BlockBlackAppleSeed.java)
    pub mod block_black_apple_seed {
        use super::*;
        // The class itself isn't translated: it extends `BlockCommonSapling`. Its static parts are below.
        // BlockBlackAppleSeed.BlockBlackAppleSeed(1): not translated, its class isn't translated (it extends `BlockCommonSapling`).
    }

    /// `BlockBlackBush` (org/betterx/betternether/blocks/BlockBlackBush.java)
    pub mod block_black_bush {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockBlackBush.BlockBlackBush(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBlackBush.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBlackBush.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBlackBush.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBlackBush.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBlackBush.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBlackBush.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockBlackVine` (org/betterx/betternether/blocks/BlockBlackVine.java)
    pub mod block_black_vine {
        use super::*;
        // The class itself isn't translated: it extends `BaseSimpleVineBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // BlockBlackVine.BlockBlackVine(1): not translated, its class isn't translated (it extends `BaseSimpleVineBlock` (org.betterx.bclib.blocks)).
    }

    /// `BlockBNPot` (org/betterx/betternether/blocks/BlockBNPot.java)
    pub mod block_bn_pot {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockBNPot.BlockBNPot(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBNPot.BlockBNPot(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBNPot.useShapeForLightOcclusion(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBNPot.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBNPot.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockBNPot.Metal` (org/betterx/betternether/blocks/BlockBNPot.java)
    pub mod block_bn_pot_metal {
        use super::*;
        // The class itself isn't translated: it extends `BlockBNPot`. Its static parts are below.
        // BlockBNPot.Metal.Metal(1): not translated, its class isn't translated (it extends `BlockBNPot`).
        // BlockBNPot.Metal.Metal(1): not translated, its class isn't translated (it extends `BlockBNPot`).
    }

    /// `BlockBNPot.Stone` (org/betterx/betternether/blocks/BlockBNPot.java)
    pub mod block_bn_pot_stone {
        use super::*;
        // The class itself isn't translated: it extends `BlockBNPot`. Its static parts are below.
        // BlockBNPot.Stone.Stone(1): not translated, its class isn't translated (it extends `BlockBNPot`).
        // BlockBNPot.Stone.Stone(1): not translated, its class isn't translated (it extends `BlockBNPot`).
    }

    /// `BlockBNPot.Wood` (org/betterx/betternether/blocks/BlockBNPot.java)
    pub mod block_bn_pot_wood {
        use super::*;
        // The class itself isn't translated: it extends `BlockBNPot`. Its static parts are below.
        // BlockBNPot.Wood.Wood(1): not translated, its class isn't translated (it extends `BlockBNPot`).
        // BlockBNPot.Wood.Wood(1): not translated, its class isn't translated (it extends `BlockBNPot`).
    }

    /// `BlockBoneMushroom` (org/betterx/betternether/blocks/BlockBoneMushroom.java)
    pub mod block_bone_mushroom {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE_NORTH: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_SOUTH: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_WEST: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_EAST: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_UP: not translated, it uses Minecraft's `VoxelShape`.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // AGE: not translated, it uses Minecraft's `IntegerProperty`.
        // BlockBoneMushroom.BlockBoneMushroom(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBoneMushroom.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBoneMushroom.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBoneMushroom.getShadeBrightness(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBoneMushroom.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBoneMushroom.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBoneMushroom.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBoneMushroom.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBoneMushroom.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBoneMushroom.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockBrownLargeMushroom` (org/betterx/betternether/blocks/BlockBrownLargeMushroom.java)
    pub mod block_brown_large_mushroom {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // TOP_CENTER_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // TOP_EDGE_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // MIDDLE_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // ROT_SIDE: not translated, it uses BrownMushroomShape, which isn't translated (its constants carry values).
        // ROT_CORNER: not translated, it uses BrownMushroomShape, which isn't translated (its constants carry values).
        // BlockBrownLargeMushroom.BlockBrownLargeMushroom(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBrownLargeMushroom.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBrownLargeMushroom.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBrownLargeMushroom.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBrownLargeMushroom.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBrownLargeMushroom.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBrownLargeMushroom.getRotationIndex(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBrownLargeMushroom.rotOffset(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBrownLargeMushroom.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockBrownLargeMushroom.getStateIfSame(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockChestOfDrawers` (org/betterx/betternether/blocks/BlockChestOfDrawers.java)
    pub mod block_chest_of_drawers {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BaseEntityBlock`. Its static parts are below.
        // BOUNDING_SHAPES: not translated, it uses `EnumMap` (java.util).
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // OPEN: not translated, it uses Minecraft's `BooleanProperty`.
        // BlockChestOfDrawers.BlockChestOfDrawers(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BlockChestOfDrawers.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BlockChestOfDrawers.getShape(4): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BlockChestOfDrawers.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BlockChestOfDrawers.getRenderShape(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BlockChestOfDrawers.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BlockChestOfDrawers.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BlockChestOfDrawers.getDrops(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BlockChestOfDrawers.rotate(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
        // BlockChestOfDrawers.mirror(2): not translated, its class isn't translated (it extends Minecraft's `BaseEntityBlock`).
    }

    /// `BlockCincinnasiteAnvil` (org/betterx/betternether/blocks/BlockCincinnasiteAnvil.java)
    pub mod block_cincinnasite_anvil {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AnvilBlock`. Its static parts are below.
        // BlockCincinnasiteAnvil.BlockCincinnasiteAnvil(1): not translated, its class isn't translated (it extends Minecraft's `AnvilBlock`).
    }

    /// `BlockCincinnasiteForge` (org/betterx/betternether/blocks/BlockCincinnasiteForge.java)
    pub mod block_cincinnasite_forge {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractFurnaceBlock`. Its static parts are below.
        // BlockCincinnasiteForge.BlockCincinnasiteForge(1): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`).
        // BlockCincinnasiteForge.getLuminance(1): not translated, it uses Minecraft's `BlockState`.
        // BlockCincinnasiteForge.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`).
        // BlockCincinnasiteForge.getTicker(3): not translated, it's generic.
        // BlockCincinnasiteForge.openContainer(3): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`).
        // BlockCincinnasiteForge.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`).
    }

    /// `BlockCincinnasiteFrame` (org/betterx/betternether/blocks/BlockCincinnasiteFrame.java)
    pub mod block_cincinnasite_frame {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // BlockCincinnasiteFrame.BlockCincinnasiteFrame(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCincinnasiteFrame.getShadeBrightness(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCincinnasiteFrame.propagatesSkylightDown(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCincinnasiteFrame.skipRendering(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockCincinnasitePedestal` (org/betterx/betternether/blocks/BlockCincinnasitePedestal.java)
    pub mod block_cincinnasite_pedestal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockCincinnasitePedestal.BlockCincinnasitePedestal(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCincinnasitePedestal.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockCincinnasitPillar` (org/betterx/betternether/blocks/BlockCincinnasitPillar.java)
    pub mod block_cincinnasit_pillar {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // BlockCincinnasitPillar.BlockCincinnasitPillar(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCincinnasitPillar.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCincinnasitPillar.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockCommonPlant` (org/betterx/betternether/blocks/BlockCommonPlant.java)
    pub mod block_common_plant {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // AGE: not translated, it uses Minecraft's `IntegerProperty`.
        // BlockCommonPlant.BlockCommonPlant(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCommonPlant.getMaxAge(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCommonPlant.getAgeProperty(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCommonPlant.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCommonPlant.canSurvive(3): not translated, it has no body.
        // BlockCommonPlant.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCommonPlant.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCommonPlant.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCommonPlant.canGrowTerrain(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCommonPlant.grow(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCommonPlant.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockCommonPlant.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockCommonSapling` (org/betterx/betternether/blocks/BlockCommonSapling.java)
    pub mod block_common_sapling {
        use super::*;
        // The class itself isn't translated: it extends `BaseBlockCommonSapling`. Its static parts are below.
        // BlockCommonSapling.BlockCommonSapling(2): not translated, its class isn't translated (it extends `BaseBlockCommonSapling`).
        // BlockCommonSapling.canSurvive(3): not translated, its class isn't translated (it extends `BaseBlockCommonSapling`).
    }

    /// `BlockEggPlant` (org/betterx/betternether/blocks/BlockEggPlant.java)
    pub mod block_egg_plant {
        use super::*;
        // The class itself isn't translated: it extends `BlockCommonPlant`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // DESTRUCTED: not translated, it uses Minecraft's `BooleanProperty`.
        // BlockEggPlant.BlockEggPlant(1): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockEggPlant.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockEggPlant.canSurvive(3): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockEggPlant.getShape(4): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockEggPlant.animateTick(4): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockEggPlant.entityInside(6): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockEggPlant.grow(4): not translated, its class isn't translated (it extends `BlockCommonPlant`).
    }

    /// `BlockEyeball` (org/betterx/betternether/blocks/BlockEyeball.java)
    pub mod block_eyeball {
        use super::*;
        // The class itself isn't translated: it extends `BlockEyeBase`. Its static parts are below.
        // BlockEyeball.BlockEyeball(1): not translated, its class isn't translated (it extends `BlockEyeBase`).
        // BlockEyeball.animateTick(4): not translated, its class isn't translated (it extends `BlockEyeBase`).
        // BlockEyeball.randomTick(4): not translated, its class isn't translated (it extends `BlockEyeBase`).
    }

    /// `BlockEyeballSmall` (org/betterx/betternether/blocks/BlockEyeballSmall.java)
    pub mod block_eyeball_small {
        use super::*;
        // The class itself isn't translated: it extends `BlockEyeBase`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockEyeballSmall.BlockEyeballSmall(1): not translated, its class isn't translated (it extends `BlockEyeBase`).
        // BlockEyeballSmall.getShape(4): not translated, its class isn't translated (it extends `BlockEyeBase`).
        // BlockEyeballSmall.animateTick(4): not translated, its class isn't translated (it extends `BlockEyeBase`).
        // BlockEyeballSmall.canSuffocate(3): not translated, its class isn't translated (it extends `BlockEyeBase`).
        // BlockEyeballSmall.isSimpleFullBlock(3): not translated, its class isn't translated (it extends `BlockEyeBase`).
    }

    /// `BlockEyeBase` (org/betterx/betternether/blocks/BlockEyeBase.java)
    pub mod block_eye_base {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // BlockEyeBase.BlockEyeBase(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockEyeBase.allowsSpawning(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockEyeBase.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockEyeBase.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockEyeSeed` (org/betterx/betternether/blocks/BlockEyeSeed.java)
    pub mod block_eye_seed {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockEyeSeed.BlockEyeSeed(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockEyeSeed.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockEyeSeed.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockEyeSeed.canGrowRandomly(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockEyeSeed.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockEyeSeed.grow(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockEyeSeed.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockEyeSeed.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockEyeSeed.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockEyeSeed.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockEyeVine` (org/betterx/betternether/blocks/BlockEyeVine.java)
    pub mod block_eye_vine {
        use super::*;
        // The class itself isn't translated: it extends `BaseVineBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // BlockEyeVine.BlockEyeVine(1): not translated, its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)).
        // BlockEyeVine.updateShape(8): not translated, its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)).
        // BlockEyeVine.getCloneItemStack(4): not translated, its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)).
    }

    /// `BlockFarmland` (org/betterx/betternether/blocks/BlockFarmland.java)
    pub mod block_farmland {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // BlockFarmland.BlockFarmland(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockFeatherFern` (org/betterx/betternether/blocks/BlockFeatherFern.java)
    pub mod block_feather_fern {
        use super::*;
        // The class itself isn't translated: it extends `BlockCommonPlant`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockFeatherFern.BlockFeatherFern(1): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockFeatherFern.getShape(4): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockFeatherFern.canSurvive(3): not translated, its class isn't translated (it extends `BlockCommonPlant`).
    }

    /// `BlockFireBowl` (org/betterx/betternether/blocks/BlockFireBowl.java)
    pub mod block_fire_bowl {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // CULL_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // FIRE: not translated, it uses Minecraft's `BooleanProperty`.
        // BlockFireBowl.BlockFireBowl(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockFireBowl.BlockFireBowl(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockFireBowl.getLuminance(1): not translated, it uses Minecraft's `BlockState`.
        // BlockFireBowl.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockFireBowl.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockFireBowl.getOcclusionShape(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockFireBowl.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockFireBowl.stepOn(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockFireBowl.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockFireBowl.Metal` (org/betterx/betternether/blocks/BlockFireBowl.java)
    pub mod block_fire_bowl_metal {
        use super::*;
        // The class itself isn't translated: it extends `BlockFireBowl`. Its static parts are below.
        // BlockFireBowl.Metal.Metal(1): not translated, its class isn't translated (it extends `BlockFireBowl`).
        // BlockFireBowl.Metal.Metal(1): not translated, its class isn't translated (it extends `BlockFireBowl`).
    }

    /// `BlockFireBowl.Stone` (org/betterx/betternether/blocks/BlockFireBowl.java)
    pub mod block_fire_bowl_stone {
        use super::*;
        // The class itself isn't translated: it extends `BlockFireBowl`. Its static parts are below.
        // BlockFireBowl.Stone.Stone(1): not translated, its class isn't translated (it extends `BlockFireBowl`).
        // BlockFireBowl.Stone.Stone(1): not translated, its class isn't translated (it extends `BlockFireBowl`).
    }

    /// `BlockFireBowl.Wood` (org/betterx/betternether/blocks/BlockFireBowl.java)
    pub mod block_fire_bowl_wood {
        use super::*;
        // The class itself isn't translated: it extends `BlockFireBowl`. Its static parts are below.
        // BlockFireBowl.Wood.Wood(1): not translated, its class isn't translated (it extends `BlockFireBowl`).
        // BlockFireBowl.Wood.Wood(1): not translated, its class isn't translated (it extends `BlockFireBowl`).
    }

    /// `BlockGeyser` (org/betterx/betternether/blocks/BlockGeyser.java)
    pub mod block_geyser {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockGeyser.BlockGeyser(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGeyser.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGeyser.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGeyser.stepOn(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGeyser.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGeyser.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockGiantLucis` (org/betterx/betternether/blocks/BlockGiantLucis.java)
    pub mod block_giant_lucis {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `HugeMushroomBlock`. Its static parts are below.
        // BlockGiantLucis.BlockGiantLucis(1): not translated, its class isn't translated (it extends Minecraft's `HugeMushroomBlock`).
    }

    /// `BlockGiantMold` (org/betterx/betternether/blocks/BlockGiantMold.java)
    pub mod block_giant_mold {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // TOP_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // MIDDLE_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // BlockGiantMold.BlockGiantMold(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGiantMold.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGiantMold.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGiantMold.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGiantMold.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockGiantMoldSapling` (org/betterx/betternether/blocks/BlockGiantMoldSapling.java)
    pub mod block_giant_mold_sapling {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        pub const BONEMEAL_SUCCESS_CHANCE: f32 = 0.4f32;
        // BlockGiantMoldSapling.BlockGiantMoldSapling(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGiantMoldSapling.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGiantMoldSapling.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGiantMoldSapling.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGiantMoldSapling.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGiantMoldSapling.canGrowRandomly(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGiantMoldSapling.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGiantMoldSapling.grow(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGiantMoldSapling.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGiantMoldSapling.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockGloomsculkCrystal` (org/betterx/betternether/blocks/BlockGloomsculkCrystal.java)
    pub mod block_gloomsculk_crystal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // SHAPES: not translated, it uses `EnumMap` (java.util).
        // BlockGloomsculkCrystal.BlockGloomsculkCrystal(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomsculkCrystal.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomsculkCrystal.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomsculkCrystal.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomsculkCrystal.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomsculkCrystal.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockGloomsculkGeode` (org/betterx/betternether/blocks/BlockGloomsculkGeode.java)
    pub mod block_gloomsculk_geode {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        pub const BUD_CHANCE: i32 = 18;
        // BlockGloomsculkGeode.BlockGloomsculkGeode(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomsculkGeode.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomsculkGeode.playerDestroy(6): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockGloomwispVine` (org/betterx/betternether/blocks/BlockGloomwispVine.java)
    pub mod block_gloomwisp_vine {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        pub const MAX_HEIGHT: i32 = 6;
        pub const MAX_NATURAL_HEIGHT: i32 = 4;
        pub const GROW_CHANCE: i32 = 32;
        pub const MIN_HEIGHT: i32 = 1;
        pub const SMOKE_MOTES: i32 = 3;
        pub const SHEAR_PARTICLES: i32 = 12;
        pub const DISTURB_PARTICLES: i32 = 6;
        pub const DISTURB_SPEED_SQR: f64 = 9.0e-6f64;
        pub const BOUNDARY_EPSILON: f64 = 1.0e-4f64;
        pub const CHIME_CHANCE: f32 = 0.5f32;
        pub const CHIME_VOLUME: f32 = 0.35f32;
        pub const CHIME_PITCH_MIN: f32 = 0.75f32;
        pub const CHIME_PITCH_SPREAD: f32 = 0.6f32;
        pub const BOON_VOLUME: f32 = 0.5f32;
        pub const BOON_PITCH_MIN: f32 = 0.92f32;
        pub const BOON_PITCH_SPREAD: f32 = 0.16f32;
        pub const EXPERIENCE_CHANCE: f32 = 0.035f32;
        pub const EXPERIENCE_MIN: i32 = 1;
        pub const EXPERIENCE_SPREAD: i32 = 3;
        pub const RECHARGE_TICKS: i32 = 1200;
        pub const RECHARGE_SPREAD_TICKS: i32 = 1200;
        pub const SCATTER_RADIUS: f64 = 0.75f64;
        pub const SHED_HEIGHT: f64 = 0.4f64;
        // STEM_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // HEAD_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // PERSISTENT: not translated, it uses Minecraft's `BooleanProperty`.
        // ROTATION: not translated, it uses Minecraft's `EnumProperty`.
        // OFFSET: not translated, it uses Minecraft's `BooleanProperty`.
        // SPENT: not translated, it uses Minecraft's `BooleanProperty`.
        // BlockGloomwispVine.BlockGloomwispVine(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.stateAwareOffset(1): not translated, it uses BlockBehaviourPropertiesAccessor, which isn't translated (a Mixin class: its handlers are the injections, in the bundle's helpers).
        // BlockGloomwispVine.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.getShadeBrightness(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.shapeAt(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.entityInside(6): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.shedExperience(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.drain(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.tick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.base(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.head(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockGloomwispVine.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockGloomwoodSapling` (org/betterx/betternether/blocks/BlockGloomwoodSapling.java)
    pub mod block_gloomwood_sapling {
        use super::*;
        // The class itself isn't translated: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api). Its static parts are below.
        // BlockGloomwoodSapling.BlockGloomwoodSapling(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockGloomwoodSapling.mayPlaceOn(3): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockGloomwoodSapling.isValidBonemealTarget(4): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockGloomwoodSapling.isRandomGrowthSuccess(4): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
    }

    /// `BlockGoldenVine` (org/betterx/betternether/blocks/BlockGoldenVine.java)
    pub mod block_golden_vine {
        use super::*;
        // The class itself isn't translated: it extends `BaseSimpleVineBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // BlockGoldenVine.BlockGoldenVine(1): not translated, its class isn't translated (it extends `BaseSimpleVineBlock` (org.betterx.bclib.blocks)).
    }

    /// `BlockGrayMold` (org/betterx/betternether/blocks/BlockGrayMold.java)
    pub mod block_gray_mold {
        use super::*;
        // The class itself isn't translated: it extends `BlockMold`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockGrayMold.BlockGrayMold(1): not translated, its class isn't translated (it extends `BlockMold`).
        // BlockGrayMold.animateTick(4): not translated, its class isn't translated (it extends `BlockMold`).
        // BlockGrayMold.getShape(4): not translated, its class isn't translated (it extends `BlockMold`).
    }

    /// `BlockHookMushroom` (org/betterx/betternether/blocks/BlockHookMushroom.java)
    pub mod block_hook_mushroom {
        use super::*;
        // The class itself isn't translated: it extends `BaseBlockMold`. Its static parts are below.
        // BlockHookMushroom.BlockHookMushroom(1): not translated, its class isn't translated (it extends `BaseBlockMold`).
        // BlockHookMushroom.canSurvive(3): not translated, its class isn't translated (it extends `BaseBlockMold`).
    }

    /// `BlockInkBush` (org/betterx/betternether/blocks/BlockInkBush.java)
    pub mod block_ink_bush {
        use super::*;
        // The class itself isn't translated: it extends `BlockCommonPlant`. Its static parts are below.
        // BlockInkBush.BlockInkBush(1): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockInkBush.getOcclusionShape(1): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockInkBush.getCloneItemStack(4): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockInkBush.canSurvive(3): not translated, its class isn't translated (it extends `BlockCommonPlant`).
    }

    /// `BlockInkBushSeed` (org/betterx/betternether/blocks/BlockInkBushSeed.java)
    pub mod block_ink_bush_seed {
        use super::*;
        // The class itself isn't translated: it extends `BlockCommonSapling`. Its static parts are below.
        // BlockInkBushSeed.BlockInkBushSeed(1): not translated, its class isn't translated (it extends `BlockCommonSapling`).
    }

    /// `BlockJellyfishMushroom` (org/betterx/betternether/blocks/BlockJellyfishMushroom.java)
    pub mod block_jellyfish_mushroom {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // TOP_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // MIDDLE_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // VISUAL: not translated, it uses Minecraft's `EnumProperty`.
        // BlockJellyfishMushroom.BlockJellyfishMushroom(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroom.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroom.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroom.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroom.getShadeBrightness(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroom.propagatesSkylightDown(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroom.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroom.fallOn(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroom.stepOn(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockJellyfishMushroomSapling` (org/betterx/betternether/blocks/BlockJellyfishMushroomSapling.java)
    pub mod block_jellyfish_mushroom_sapling {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        pub const BONEMEAL_SUCCESS_CHANCE: f32 = 0.4f32;
        // BlockJellyfishMushroomSapling.BlockJellyfishMushroomSapling(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroomSapling.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroomSapling.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroomSapling.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroomSapling.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroomSapling.canGrowRandomly(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroomSapling.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroomSapling.grow(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroomSapling.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockJellyfishMushroomSapling.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockLucisMushroom` (org/betterx/betternether/blocks/BlockLucisMushroom.java)
    pub mod block_lucis_mushroom {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // V_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // BlockLucisMushroom.BlockLucisMushroom(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisMushroom.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisMushroom.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisMushroom.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisMushroom.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisMushroom.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockLucisSpore` (org/betterx/betternether/blocks/BlockLucisSpore.java)
    pub mod block_lucis_spore {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // BOUNDING_SHAPES: not translated, it uses `EnumMap` (java.util).
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // BlockLucisSpore.BlockLucisSpore(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisSpore.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisSpore.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisSpore.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisSpore.canGrowRandomly(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisSpore.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisSpore.grow(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisSpore.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisSpore.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisSpore.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisSpore.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisSpore.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisSpore.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLucisSpore.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockLumabusSeed` (org/betterx/betternether/blocks/BlockLumabusSeed.java)
    pub mod block_lumabus_seed {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockLumabusSeed.BlockLumabusSeed(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLumabusSeed.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLumabusSeed.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLumabusSeed.canGrowRandomly(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLumabusSeed.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLumabusSeed.grow(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLumabusSeed.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLumabusSeed.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLumabusSeed.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockLumabusSeed.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockLumabusVine` (org/betterx/betternether/blocks/BlockLumabusVine.java)
    pub mod block_lumabus_vine {
        use super::*;
        // The class itself isn't translated: it extends `BaseVineBlock` (org.betterx.bclib.blocks). Its static parts are below.
        // MIDDLE_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BOTTOM_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockLumabusVine.BlockLumabusVine(2): not translated, its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)).
        // BlockLumabusVine.getLuminance(0): not translated, it uses Minecraft's `BlockState`.
        // BlockLumabusVine.getShape(4): not translated, its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)).
        // BlockLumabusVine.getCloneItemStack(4): not translated, its class isn't translated (it extends `BaseVineBlock` (org.betterx.bclib.blocks)).
    }

    /// `BlockMagmaFlower` (org/betterx/betternether/blocks/BlockMagmaFlower.java)
    pub mod block_magma_flower {
        use super::*;
        // The class itself isn't translated: it extends `BlockCommonPlant`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockMagmaFlower.BlockMagmaFlower(1): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockMagmaFlower.getShape(4): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockMagmaFlower.canSurvive(3): not translated, its class isn't translated (it extends `BlockCommonPlant`).
    }

    /// `BlockMold` (org/betterx/betternether/blocks/BlockMold.java)
    pub mod block_mold {
        use super::*;
        // The class itself isn't translated: it extends `BaseBlockMold`. Its static parts are below.
        // BlockMold.BlockMold(1): not translated, its class isn't translated (it extends `BaseBlockMold`).
        // BlockMold.canSurvive(3): not translated, its class isn't translated (it extends `BaseBlockMold`).
    }

    /// `BlockMoltenGloomsculk` (org/betterx/betternether/blocks/BlockMoltenGloomsculk.java)
    pub mod block_molten_gloomsculk {
        use super::*;
        // The class itself isn't translated: it extends `BlockTerrain`. Its static parts are below.
        pub const CINDER_CHANCE: i32 = 12;
        pub const SPIT_CHANCE: i32 = 5;
        // BlockMoltenGloomsculk.BlockMoltenGloomsculk(1): not translated, its class isn't translated (it extends `BlockTerrain`).
        // BlockMoltenGloomsculk.animateTick(4): not translated, its class isn't translated (it extends `BlockTerrain`).
    }

    /// `BlockMoltenGloomwoodLog` (org/betterx/betternether/blocks/BlockMoltenGloomwoodLog.java)
    pub mod block_molten_gloomwood_log {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RotatedPillarBlock`. Its static parts are below.
        pub const EMBERS: i32 = 2;
        // BlockMoltenGloomwoodLog.BlockMoltenGloomwoodLog(1): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
        // BlockMoltenGloomwoodLog.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
    }

    /// `BlockMossCover` (org/betterx/betternether/blocks/BlockMossCover.java)
    pub mod block_moss_cover {
        use super::*;
        // The class itself isn't translated: it extends `BaseBlockMold`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockMossCover.BlockMossCover(1): not translated, its class isn't translated (it extends `BaseBlockMold`).
        // BlockMossCover.canSurvive(3): not translated, its class isn't translated (it extends `BaseBlockMold`).
        // BlockMossCover.getShape(4): not translated, its class isn't translated (it extends `BaseBlockMold`).
    }

    /// `BlockMushroomFir` (org/betterx/betternether/blocks/BlockMushroomFir.java)
    pub mod block_mushroom_fir {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // BOTTOM_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // MIDDLE_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // TOP_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SIDE_BIG_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SIDE_SMALL_N_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SIDE_SMALL_S_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SIDE_SMALL_E_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SIDE_SMALL_W_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // END_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockMushroomFir.BlockMushroomFir(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockMushroomFir.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockMushroomFir.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockMushroomFir.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockMushroomFir.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockMushroomFir.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockMushroomFir.tick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockMushroomFir.MushroomFirShape` (org/betterx/betternether/blocks/BlockMushroomFir.java)
    pub mod block_mushroom_fir_mushroom_fir_shape {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // BlockMushroomFir.MushroomFirShape.MushroomFirShape(1): not translated, its class isn't translated (its constants carry values).
        // BlockMushroomFir.MushroomFirShape.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // BlockMushroomFir.MushroomFirShape.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `BlockMushroomFirSapling` (org/betterx/betternether/blocks/BlockMushroomFirSapling.java)
    pub mod block_mushroom_fir_sapling {
        use super::*;
        // The class itself isn't translated: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api). Its static parts are below.
        // BlockMushroomFirSapling.BlockMushroomFirSapling(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockMushroomFirSapling.mayPlaceOn(3): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockMushroomFirSapling.isRandomGrowthSuccess(4): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
    }

    /// `BlockNeonEquisetum` (org/betterx/betternether/blocks/BlockNeonEquisetum.java)
    pub mod block_neon_equisetum {
        use super::*;
        // The class itself isn't translated: it extends `Growing` (org.betterx.bclib.blocks.BaseVineBlock). Its static parts are below.
        // BlockNeonEquisetum.BlockNeonEquisetum(1): not translated, its class isn't translated (it extends `Growing` (org.betterx.bclib.blocks.BaseVineBlock)).
    }

    /// `BlockNetherCactus` (org/betterx/betternether/blocks/BlockNetherCactus.java)
    pub mod block_nether_cactus {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // TOP_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SIDE_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // TOP: not translated, it uses Minecraft's `BooleanProperty`.
        // BlockNetherCactus.BlockNetherCactus(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherCactus.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherCactus.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherCactus.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherCactus.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherCactus.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherCactus.entityInside(6): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockNetherFurnace` (org/betterx/betternether/blocks/BlockNetherFurnace.java)
    pub mod block_nether_furnace {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractFurnaceBlock`. Its static parts are below.
        // BlockNetherFurnace.BlockNetherFurnace(1): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`).
        // BlockNetherFurnace.getLuminance(1): not translated, it uses Minecraft's `BlockState`.
        // BlockNetherFurnace.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`).
        // BlockNetherFurnace.getTicker(3): not translated, it's generic.
        // BlockNetherFurnace.openContainer(3): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`).
        // BlockNetherFurnace.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `AbstractFurnaceBlock`).
    }

    /// `BlockNetherGrass` (org/betterx/betternether/blocks/BlockNetherGrass.java)
    pub mod block_nether_grass {
        use super::*;
        // The class itself isn't translated: it extends `BaseBlockNetherGrass`. Its static parts are below.
        // BlockNetherGrass.BlockNetherGrass(1): not translated, its class isn't translated (it extends `BaseBlockNetherGrass`).
    }

    /// `BlockNetherGrass.BoneGrass` (org/betterx/betternether/blocks/BlockNetherGrass.java)
    pub mod block_nether_grass_bone_grass {
        use super::*;
        // The class itself isn't translated: it extends `BaseBlockNetherGrass`. Its static parts are below.
        // BlockNetherGrass.BoneGrass.BoneGrass(1): not translated, its class isn't translated (it extends `BaseBlockNetherGrass`).
    }

    /// `BlockNetherGrass.JunglePlant` (org/betterx/betternether/blocks/BlockNetherGrass.java)
    pub mod block_nether_grass_jungle_plant {
        use super::*;
        // The class itself isn't translated: it extends `BlockNetherGrass`. Its static parts are below.
        // BlockNetherGrass.JunglePlant.JunglePlant(1): not translated, its class isn't translated (it extends `BlockNetherGrass`).
    }

    /// `BlockNetherGrass.NetherGrass` (org/betterx/betternether/blocks/BlockNetherGrass.java)
    pub mod block_nether_grass_nether_grass {
        use super::*;
        // The class itself isn't translated: it extends `BlockNetherGrass`. Its static parts are below.
        // BlockNetherGrass.NetherGrass.NetherGrass(1): not translated, its class isn't translated (it extends `BlockNetherGrass`).
    }

    /// `BlockNetherGrass.SepiaBoneGrass` (org/betterx/betternether/blocks/BlockNetherGrass.java)
    pub mod block_nether_grass_sepia_bone_grass {
        use super::*;
        // The class itself isn't translated: it extends `BaseBlockNetherGrass`. Its static parts are below.
        // BlockNetherGrass.SepiaBoneGrass.SepiaBoneGrass(1): not translated, its class isn't translated (it extends `BaseBlockNetherGrass`).
    }

    /// `BlockNetherGrass.SwampGrass` (org/betterx/betternether/blocks/BlockNetherGrass.java)
    pub mod block_nether_grass_swamp_grass {
        use super::*;
        // The class itself isn't translated: it extends `BlockNetherGrass`. Its static parts are below.
        // BlockNetherGrass.SwampGrass.SwampGrass(1): not translated, its class isn't translated (it extends `BlockNetherGrass`).
    }

    /// `BlockNetherMycelium` (org/betterx/betternether/blocks/BlockNetherMycelium.java)
    pub mod block_nether_mycelium {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // IS_BLUE: not translated, it uses Minecraft's `BooleanProperty`.
        // BlockNetherMycelium.BlockNetherMycelium(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherMycelium.setVegetationFeature(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherMycelium.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherMycelium.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherMycelium.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherMycelium.getHostBlock(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherMycelium.getCoverFeature(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockNetherReed` (org/betterx/betternether/blocks/BlockNetherReed.java)
    pub mod block_nether_reed {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // TOP: not translated, it uses Minecraft's `BooleanProperty`.
        // BlockNetherReed.BlockNetherReed(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherReed.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherReed.getShadeBrightness(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherReed.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherReed.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockNetherReed.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockNetherSakuraLeaves` (org/betterx/betternether/blocks/BlockNetherSakuraLeaves.java)
    pub mod block_nether_sakura_leaves {
        use super::*;
        // The class itself isn't translated: it extends `BNLeaves`. Its static parts are below.
        pub static COLOR: LazyLock<i32> = LazyLock::new(|| m_helper::color(251, 113, 143));
        // BlockNetherSakuraLeaves.BlockNetherSakuraLeaves(1): not translated, its class isn't translated (it extends `BNLeaves`).
        // BlockNetherSakuraLeaves.getShadeBrightness(3): not translated, its class isn't translated (it extends `BNLeaves`).
        // BlockNetherSakuraLeaves.propagatesSkylightDown(1): not translated, its class isn't translated (it extends `BNLeaves`).
        // BlockNetherSakuraLeaves.animateTick(4): not translated, its class isn't translated (it extends `BNLeaves`).
        // BlockNetherSakuraLeaves.getColor(3): not translated, its class isn't translated (it extends `BNLeaves`).
    }

    /// `BlockNetherSakuraSapling` (org/betterx/betternether/blocks/BlockNetherSakuraSapling.java)
    pub mod block_nether_sakura_sapling {
        use super::*;
        // The class itself isn't translated: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api). Its static parts are below.
        // BlockNetherSakuraSapling.BlockNetherSakuraSapling(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockNetherSakuraSapling.mayPlaceOn(3): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
    }

    /// `BlockObsidianGlass` (org/betterx/betternether/blocks/BlockObsidianGlass.java)
    pub mod block_obsidian_glass {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // BlockObsidianGlass.BlockObsidianGlass(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockObsidianGlass.getShadeBrightness(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockObsidianGlass.propagatesSkylightDown(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockObsidianGlass.skipRendering(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockOrangeMushroom` (org/betterx/betternether/blocks/BlockOrangeMushroom.java)
    pub mod block_orange_mushroom {
        use super::*;
        // The class itself isn't translated: it extends `BlockCommonPlant`. Its static parts are below.
        // SHAPES: not translated, it uses Minecraft's `VoxelShape`.
        // BlockOrangeMushroom.BlockOrangeMushroom(1): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockOrangeMushroom.canSurvive(3): not translated, its class isn't translated (it extends `BlockCommonPlant`).
        // BlockOrangeMushroom.getShape(4): not translated, its class isn't translated (it extends `BlockCommonPlant`).
    }

    /// `BlockOre` (org/betterx/betternether/blocks/BlockOre.java)
    pub mod block_ore {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `DropExperienceBlock`. Its static parts are below.
        // BlockOre.BlockOre(2): not translated, its class isn't translated (it extends Minecraft's `DropExperienceBlock`).
    }

    /// `BlockPottedPlant` (org/betterx/betternether/blocks/BlockPottedPlant.java)
    pub mod block_potted_plant {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // PLANT: not translated, it uses Minecraft's `EnumProperty`.
        // BlockPottedPlant.BlockPottedPlant(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockPottedPlant.getLuminance(1): not translated, it uses Minecraft's `BlockState`.
        // BlockPottedPlant.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockPottedPlant.propagatesSkylightDown(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockPottedPlant.getCollisionShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockPottedPlant.getBlockSupportShape(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockPottedPlant.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockPottedPlant.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockPottedPlant.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockPottedPlant.getPlant(1): not translated, it uses Minecraft's `BlockState`.
    }

    /// `BlockRedLargeMushroom` (org/betterx/betternether/blocks/BlockRedLargeMushroom.java)
    pub mod block_red_large_mushroom {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // TOP_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // MIDDLE_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // BlockRedLargeMushroom.BlockRedLargeMushroom(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockRedLargeMushroom.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockRedLargeMushroom.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockRedLargeMushroom.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockRedLargeMushroom.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockRedMold` (org/betterx/betternether/blocks/BlockRedMold.java)
    pub mod block_red_mold {
        use super::*;
        // The class itself isn't translated: it extends `BlockMold`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockRedMold.BlockRedMold(1): not translated, its class isn't translated (it extends `BlockMold`).
        // BlockRedMold.getShape(4): not translated, its class isn't translated (it extends `BlockMold`).
    }

    /// `BlockReedsBlock` (org/betterx/betternether/blocks/BlockReedsBlock.java)
    pub mod block_reeds_block {
        use super::*;
        // The class itself isn't translated: it extends `BNPillar`. Its static parts are below.
        // BlockReedsBlock.BlockReedsBlock(1): not translated, its class isn't translated (it extends `BNPillar`).
    }

    /// `BlockRubeusCone` (org/betterx/betternether/blocks/BlockRubeusCone.java)
    pub mod block_rubeus_cone {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockRubeusCone.BlockRubeusCone(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockRubeusCone.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockRubeusCone.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockRubeusCone.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockRubeusLeaves` (org/betterx/betternether/blocks/BlockRubeusLeaves.java)
    pub mod block_rubeus_leaves {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `TintedParticleLeavesBlock`. Its static parts are below.
        // BlockRubeusLeaves.BlockRubeusLeaves(1): not translated, its class isn't translated (it extends Minecraft's `TintedParticleLeavesBlock`).
    }

    /// `BlockRubeusSapling` (org/betterx/betternether/blocks/BlockRubeusSapling.java)
    pub mod block_rubeus_sapling {
        use super::*;
        // The class itself isn't translated: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api). Its static parts are below.
        // BlockRubeusSapling.BlockRubeusSapling(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockRubeusSapling.mayPlaceOn(3): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockRubeusSapling.isRandomGrowthSuccess(4): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
    }

    /// `BlockSmallLantern` (org/betterx/betternether/blocks/BlockSmallLantern.java)
    pub mod block_small_lantern {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE_NORTH: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_SOUTH: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_WEST: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_EAST: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_UP: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_DOWN: not translated, it uses Minecraft's `VoxelShape`.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // BlockSmallLantern.BlockSmallLantern(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSmallLantern.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSmallLantern.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSmallLantern.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSmallLantern.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSmallLantern.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSmallLantern.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSmallLantern.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockSmallLantern.Metal` (org/betterx/betternether/blocks/BlockSmallLantern.java)
    pub mod block_small_lantern_metal {
        use super::*;
        // The class itself isn't translated: it extends `BlockSmallLantern`. Its static parts are below.
        // BlockSmallLantern.Metal.Metal(1): not translated, its class isn't translated (it extends `BlockSmallLantern`).
    }

    /// `BlockSmoker` (org/betterx/betternether/blocks/BlockSmoker.java)
    pub mod block_smoker {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // TOP_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // MIDDLE_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // BlockSmoker.BlockSmoker(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSmoker.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSmoker.animateTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSmoker.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSmoker.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSmoker.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockSoulGrass` (org/betterx/betternether/blocks/BlockSoulGrass.java)
    pub mod block_soul_grass {
        use super::*;
        // The class itself isn't translated: it extends `BaseBlockNetherGrass`. Its static parts are below.
        // BlockSoulGrass.BlockSoulGrass(1): not translated, its class isn't translated (it extends `BaseBlockNetherGrass`).
        // BlockSoulGrass.animateTick(4): not translated, its class isn't translated (it extends `BaseBlockNetherGrass`).
    }

    /// `BlockSoulLily` (org/betterx/betternether/blocks/BlockSoulLily.java)
    pub mod block_soul_lily {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // SHAPE_SMALL: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_MEDIUM_BOTTOM: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_MEDIUM_TOP: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_BIG_BOTTOM: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_BIG_MIDDLE: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_BIG_TOP_CENTER: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_BIG_TOP_SIDE_N: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_BIG_TOP_SIDE_S: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_BIG_TOP_SIDE_E: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_BIG_TOP_SIDE_W: not translated, it uses Minecraft's `VoxelShape`.
        // STRUCTURE: not translated, it uses SoulLilyFeature, which isn't translated (it extends `ContextFeature`).
        // ROT: not translated, it uses SoulLilyShape, which isn't translated (its constants carry values).
        // BlockSoulLily.BlockSoulLily(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulLily.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulLily.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulLily.canGrow(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulLily.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulLily.isAirSides(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulLily.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulLily.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulLily.getRotationIndex(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulLily.rotOffset(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulLily.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulLily.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulLily.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockSoulLily.SoulLilyShape` (org/betterx/betternether/blocks/BlockSoulLily.java)
    pub mod block_soul_lily_soul_lily_shape {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // BlockSoulLily.SoulLilyShape.SoulLilyShape(1): not translated, its class isn't translated (its constants carry values).
        // BlockSoulLily.SoulLilyShape.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // BlockSoulLily.SoulLilyShape.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `BlockSoulLilySapling` (org/betterx/betternether/blocks/BlockSoulLilySapling.java)
    pub mod block_soul_lily_sapling {
        use super::*;
        // The class itself isn't translated: it extends `BaseBlockCommonSapling`. Its static parts are below.
        // BlockSoulLilySapling.BlockSoulLilySapling(1): not translated, its class isn't translated (it extends `BaseBlockCommonSapling`).
        // BlockSoulLilySapling.canSurvive(3): not translated, its class isn't translated (it extends `BaseBlockCommonSapling`).
    }

    /// `BlockSoulSandstone` (org/betterx/betternether/blocks/BlockSoulSandstone.java)
    pub mod block_soul_sandstone {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // UP: not translated, it uses Minecraft's `BooleanProperty`.
        // BlockSoulSandstone.BlockSoulSandstone(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulSandstone.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulSandstone.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulSandstone.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockSoulVein` (org/betterx/betternether/blocks/BlockSoulVein.java)
    pub mod block_soul_vein {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // BlockSoulVein.BlockSoulVein(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulVein.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulVein.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulVein.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulVein.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulVein.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulVein.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockSoulVein.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockStalactite` (org/betterx/betternether/blocks/BlockStalactite.java)
    pub mod block_stalactite {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SIZE: not translated, it uses Minecraft's `IntegerProperty`.
        // SHAPES: not translated, it uses Minecraft's `VoxelShape`.
        // BlockStalactite.BlockStalactite(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStalactite.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStalactite.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStalactite.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStalactite.destroy(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStalactite.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStalactite.canPlace(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockStalagnate` (org/betterx/betternether/blocks/BlockStalagnate.java)
    pub mod block_stalagnate {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SELECT_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // COLLISION_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // BlockStalagnate.BlockStalagnate(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStalagnate.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStalagnate.getCollisionShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStalagnate.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStalagnate.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockStalagnateBowl` (org/betterx/betternether/blocks/BlockStalagnateBowl.java)
    pub mod block_stalagnate_bowl {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // FOOD: not translated, it uses Minecraft's `EnumProperty`.
        // BlockStalagnateBowl.BlockStalagnateBowl(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStalagnateBowl.BlockStalagnateBowl(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStalagnateBowl.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStalagnateBowl.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStalagnateBowl.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStalagnateBowl.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockStalagnateSeed` (org/betterx/betternether/blocks/BlockStalagnateSeed.java)
    pub mod block_stalagnate_seed {
        use super::*;
        // The class itself isn't translated: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api). Its static parts are below.
        pub const MAX_SEARCH_LENGTH: i32 = 25;
        pub const MIN_LENGTH: i32 = 3;
        // SHAPE_TOP: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_BOTTOM: not translated, it uses Minecraft's `VoxelShape`.
        // TOP: not translated, it uses Minecraft's `BooleanProperty`.
        // BlockStalagnateSeed.BlockStalagnateSeed(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockStalagnateSeed.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockStalagnateSeed.getStateForPlacement(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockStalagnateSeed.getShape(4): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockStalagnateSeed.growsDownward(1): not translated, it uses Minecraft's `BlockState`.
        // BlockStalagnateSeed.isBonemealSuccess(5): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockStalagnateSeed.isRandomGrowthSuccess(4): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockStalagnateSeed.hasRoomToGrow(3): not translated, it uses Minecraft's `Level`.
        // BlockStalagnateSeed.mayPlaceOn(3): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockStalagnateSeed.canSurvive(3): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockStalagnateSeed.growFeature(4): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
    }

    /// `BlockStatueRespawner` (org/betterx/betternether/blocks/BlockStatueRespawner.java)
    pub mod block_statue_respawner {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // CULL_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // EFFECT: not translated, it uses Minecraft's `DustParticleOptions`.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // TOP: not translated, it uses Minecraft's `BooleanProperty`.
        // BlockStatueRespawner.BlockStatueRespawner(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStatueRespawner.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStatueRespawner.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStatueRespawner.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStatueRespawner.getOcclusionShape(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStatueRespawner.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStatueRespawner.getHelpMessage(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStatueRespawner.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStatueRespawner.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStatueRespawner.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStatueRespawner.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStatueRespawner.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStatueRespawner.playerWillDestroy(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockStem` (org/betterx/betternether/blocks/BlockStem.java)
    pub mod block_stem {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // AXIS: not translated, it uses Minecraft's `EnumProperty`.
        // OUTLINES: not translated, it uses `EnumMap` (java.util).
        // BlockStem.BlockStem(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStem.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStem.getCollisionShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStem.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStem.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockStem.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockTerrain` (org/betterx/betternether/blocks/BlockTerrain.java)
    pub mod block_terrain {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // TERRAIN_SOUND: not translated, it uses Minecraft's `SoundType`.
        // BlockTerrain.BlockTerrain(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockTerrain.setVegetationFeature(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockTerrain.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockTerrain.getHostBlock(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockTerrain.getCoverFeature(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockVeinedSand` (org/betterx/betternether/blocks/BlockVeinedSand.java)
    pub mod block_veined_sand {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // BlockVeinedSand.BlockVeinedSand(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockVeinedSand.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockVeinedSand.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockWartRoots` (org/betterx/betternether/blocks/BlockWartRoots.java)
    pub mod block_wart_roots {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // BlockWartRoots.BlockWartRoots(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWartRoots.BlockWartRoots(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockWartSeed` (org/betterx/betternether/blocks/BlockWartSeed.java)
    pub mod block_wart_seed {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // BOUNDING_SHAPES: not translated, it uses `EnumMap` (java.util).
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // BlockWartSeed.BlockWartSeed(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWartSeed.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWartSeed.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWartSeed.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWartSeed.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWartSeed.isValidBonemealTarget(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWartSeed.isBonemealSuccess(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWartSeed.performBonemeal(5): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWartSeed.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWartSeed.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWartSeed.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockWhisperingGourdLantern` (org/betterx/betternether/blocks/BlockWhisperingGourdLantern.java)
    pub mod block_whispering_gourd_lantern {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // BlockWhisperingGourdLantern.BlockWhisperingGourdLantern(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWhisperingGourdLantern.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWhisperingGourdLantern.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWhisperingGourdLantern.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWhisperingGourdLantern.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockWhisperingGourdVine` (org/betterx/betternether/blocks/BlockWhisperingGourdVine.java)
    pub mod block_whispering_gourd_vine {
        use super::*;
        // The class itself isn't translated: it extends `Growing` (org.betterx.bclib.blocks.BaseVineBlock). Its static parts are below.
        // BlockWhisperingGourdVine.BlockWhisperingGourdVine(1): not translated, its class isn't translated (it extends `Growing` (org.betterx.bclib.blocks.BaseVineBlock)).
        // BlockWhisperingGourdVine.updateShape(8): not translated, its class isn't translated (it extends `Growing` (org.betterx.bclib.blocks.BaseVineBlock)).
        // BlockWhisperingGourdVine.useItemOn(7): not translated, its class isn't translated (it extends `Growing` (org.betterx.bclib.blocks.BaseVineBlock)).
    }

    /// `BlockWillowBranch` (org/betterx/betternether/blocks/BlockWillowBranch.java)
    pub mod block_willow_branch {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // V_SHAPE: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // BlockWillowBranch.BlockWillowBranch(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWillowBranch.getLuminance(1): not translated, it uses Minecraft's `BlockState`.
        // BlockWillowBranch.getShadeBrightness(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWillowBranch.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWillowBranch.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWillowBranch.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWillowBranch.getCloneItemStack(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockWillowLeaves` (org/betterx/betternether/blocks/BlockWillowLeaves.java)
    pub mod block_willow_leaves {
        use super::*;
        // The class itself isn't translated: it extends `BNLeaves`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // NATURAL: not translated, it uses Minecraft's `BooleanProperty`.
        // BlockWillowLeaves.BlockWillowLeaves(1): not translated, its class isn't translated (it extends `BNLeaves`).
        // BlockWillowLeaves.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BNLeaves`).
        // BlockWillowLeaves.getStateForPlacement(1): not translated, its class isn't translated (it extends `BNLeaves`).
        // BlockWillowLeaves.rotate(2): not translated, its class isn't translated (it extends `BNLeaves`).
        // BlockWillowLeaves.mirror(2): not translated, its class isn't translated (it extends `BNLeaves`).
        // BlockWillowLeaves.updateShape(8): not translated, its class isn't translated (it extends `BNLeaves`).
        // BlockWillowLeaves.getShadeBrightness(3): not translated, its class isn't translated (it extends `BNLeaves`).
        // BlockWillowLeaves.propagatesSkylightDown(1): not translated, its class isn't translated (it extends `BNLeaves`).
        // BlockWillowLeaves.getBlockSupportShape(3): not translated, its class isn't translated (it extends `BNLeaves`).
    }

    /// `BlockWillowSapling` (org/betterx/betternether/blocks/BlockWillowSapling.java)
    pub mod block_willow_sapling {
        use super::*;
        // The class itself isn't translated: it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api). Its static parts are below.
        // BlockWillowSapling.BlockWillowSapling(1): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockWillowSapling.megaFeatureEnabled(3): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockWillowSapling.isRandomGrowthSuccess(4): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
        // BlockWillowSapling.mayPlaceOn(3): not translated, its class isn't translated (it extends `FeatureSaplingBlock` (de.ambertation.wover.block.api)).
    }

    /// `BlockWillowTorch` (org/betterx/betternether/blocks/BlockWillowTorch.java)
    pub mod block_willow_torch {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE_NORTH: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_SOUTH: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_WEST: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_EAST: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_UP: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_DOWN: not translated, it uses Minecraft's `VoxelShape`.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // BlockWillowTorch.BlockWillowTorch(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWillowTorch.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWillowTorch.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWillowTorch.canSurvive(3): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWillowTorch.rotate(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWillowTorch.mirror(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWillowTorch.updateShape(8): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWillowTorch.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockWillowTrunk` (org/betterx/betternether/blocks/BlockWillowTrunk.java)
    pub mod block_willow_trunk {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // SHAPE_BOTTOM: not translated, it uses Minecraft's `VoxelShape`.
        // SHAPE_TOP: not translated, it uses Minecraft's `VoxelShape`.
        // BlockWillowTrunk.BlockWillowTrunk(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWillowTrunk.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockWillowTrunk.getShape(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlueCryingObsidianBlock` (org/betterx/betternether/blocks/BlueCryingObsidianBlock.java)
    pub mod blue_crying_obsidian_block {
        use super::*;
        // The class itself isn't translated: it extends `BNObsidianBase`. Its static parts are below.
        // BlueCryingObsidianBlock.BlueCryingObsidianBlock(1): not translated, its class isn't translated (it extends `BNObsidianBase`).
        // BlueCryingObsidianBlock.animateTick(4): not translated, its class isn't translated (it extends `BNObsidianBase`).
    }

    /// `BlueWeepingObsidianBlock` (org/betterx/betternether/blocks/BlueWeepingObsidianBlock.java)
    pub mod blue_weeping_obsidian_block {
        use super::*;
        // The class itself isn't translated: it extends `BNObsidianBase`. Its static parts are below.
        // BlueWeepingObsidianBlock.BlueWeepingObsidianBlock(1): not translated, its class isn't translated (it extends `BNObsidianBase`).
        // BlueWeepingObsidianBlock.animateTick(4): not translated, its class isn't translated (it extends `BNObsidianBase`).
    }

    /// `BNBlockProperties` (org/betterx/betternether/blocks/BNBlockProperties.java)
    pub mod bn_block_properties {
        use super::*;
        // PILLAR_SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // BROWN_MUSHROOM_SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // WILLOW_SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // JELLY_MUSHROOM_VISUAL: not translated, it uses Minecraft's `EnumProperty`.
        // LUCIS_SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // PLANT: not translated, it uses Minecraft's `EnumProperty`.
        // FOOD: not translated, it uses Minecraft's `EnumProperty`.
        // DESTRUCTED: not translated, it uses Minecraft's `BooleanProperty`.
        // FLOOR: not translated, it uses Minecraft's `BooleanProperty`.
        // OPEN: not translated, it uses Minecraft's `BooleanProperty`.
        // FIRE: not translated, it uses Minecraft's `BooleanProperty`.
        #[derive(Clone, Debug)]
        pub struct BNBlockProperties {
        }
        impl BNBlockProperties {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `BNBlockProperties.BrownMushroomShape` (org/betterx/betternether/blocks/BNBlockProperties.java)
    pub mod bn_block_properties_brown_mushroom_shape {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // BNBlockProperties.BrownMushroomShape.BrownMushroomShape(1): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.BrownMushroomShape.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.BrownMushroomShape.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `BNBlockProperties.CincinnasitPillarShape` (org/betterx/betternether/blocks/BNBlockProperties.java)
    pub mod bn_block_properties_cincinnasit_pillar_shape {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // BNBlockProperties.CincinnasitPillarShape.CincinnasitPillarShape(1): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.CincinnasitPillarShape.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.CincinnasitPillarShape.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `BNBlockProperties.EnumLucisShape` (org/betterx/betternether/blocks/BNBlockProperties.java)
    pub mod bn_block_properties_enum_lucis_shape {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // BNBlockProperties.EnumLucisShape.EnumLucisShape(1): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.EnumLucisShape.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.EnumLucisShape.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `BNBlockProperties.FoodShape` (org/betterx/betternether/blocks/BNBlockProperties.java)
    pub mod bn_block_properties_food_shape {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // BNBlockProperties.FoodShape.FoodShape(1): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.FoodShape.setItem(1): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.FoodShape.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.FoodShape.toString(0): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.FoodShape.getItem(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `BNBlockProperties.JellyShape` (org/betterx/betternether/blocks/BNBlockProperties.java)
    pub mod bn_block_properties_jelly_shape {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // BNBlockProperties.JellyShape.JellyShape(1): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.JellyShape.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.JellyShape.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `BNBlockProperties.PottedPlantShape` (org/betterx/betternether/blocks/BNBlockProperties.java)
    pub mod bn_block_properties_potted_plant_shape {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // BNBlockProperties.PottedPlantShape.PottedPlantShape(2): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.PottedPlantShape.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.PottedPlantShape.toString(0): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.PottedPlantShape.getItem(0): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.PottedPlantShape.getBlock(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `BNBlockProperties.WillowBranchShape` (org/betterx/betternether/blocks/BNBlockProperties.java)
    pub mod bn_block_properties_willow_branch_shape {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // BNBlockProperties.WillowBranchShape.WillowBranchShape(1): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.WillowBranchShape.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // BNBlockProperties.WillowBranchShape.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `BNBrewingStand` (org/betterx/betternether/blocks/BNBrewingStand.java)
    pub mod bn_brewing_stand {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BrewingStandBlock`. Its static parts are below.
        // BNBrewingStand.BNBrewingStand(1): not translated, its class isn't translated (it extends Minecraft's `BrewingStandBlock`).
        // BNBrewingStand.getTicker(3): not translated, it's generic.
        // BNBrewingStand.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `BrewingStandBlock`).
        // BNBrewingStand.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `BrewingStandBlock`).
        // BNBrewingStand.affectNeighborsAfterRemoval(4): not translated, its class isn't translated (it extends Minecraft's `BrewingStandBlock`).
    }

    /// `BNLeaves` (org/betterx/betternether/blocks/BNLeaves.java)
    pub mod bn_leaves {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `TintedParticleLeavesBlock`. Its static parts are below.
        // BNLeaves.BNLeaves(1): not translated, its class isn't translated (it extends Minecraft's `TintedParticleLeavesBlock`).
        // BNLeaves.isRandomlyTicking(1): not translated, its class isn't translated (it extends Minecraft's `TintedParticleLeavesBlock`).
        // BNLeaves.randomTick(4): not translated, its class isn't translated (it extends Minecraft's `TintedParticleLeavesBlock`).
        // BNLeaves.tick(4): not translated, its class isn't translated (it extends Minecraft's `TintedParticleLeavesBlock`).
    }

    /// `BNObsidian` (org/betterx/betternether/blocks/BNObsidian.java)
    pub mod bn_obsidian {
        use super::*;
        // The class itself isn't translated: it extends `BNObsidianBase`. Its static parts are below.
        // BNObsidian.BNObsidian(0): not translated, its class isn't translated (it extends `BNObsidianBase`).
        // BNObsidian.BNObsidian(1): not translated, its class isn't translated (it extends `BNObsidianBase`).
        // BNObsidian.BNObsidian(2): not translated, its class isn't translated (it extends `BNObsidianBase`).
        // BNObsidian.onLightningUpdate(3): not translated, it uses Minecraft's `Level`.
    }

    /// `BNObsidianBase` (org/betterx/betternether/blocks/BNObsidianBase.java)
    pub mod bn_obsidian_base {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // BNObsidianBase.BNObsidianBase(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BNObsidianBase.BNObsidianBase(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BNObsidianBase.BNObsidianBase(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BNObsidianBase.neighborChanged(6): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BNPane` (org/betterx/betternether/blocks/BNPane.java)
    pub mod bn_pane {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `IronBarsBlock`. Its static parts are below.
        // BNPane.BNPane(1): not translated, its class isn't translated (it extends Minecraft's `IronBarsBlock`).
        // BNPane.skipRendering(3): not translated, its class isn't translated (it extends Minecraft's `IronBarsBlock`).
    }

    /// `BNPillar` (org/betterx/betternether/blocks/BNPillar.java)
    pub mod bn_pillar {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RotatedPillarBlock`. Its static parts are below.
        // BNPillar.BNPillar(1): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
        // BNPillar.BNPillar(1): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
    }

    /// `BNWoodlikeDoor` (org/betterx/betternether/blocks/BNWoodlikeDoor.java)
    pub mod bn_woodlike_door {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `DoorBlock`. Its static parts are below.
        // BNWoodlikeDoor.BNWoodlikeDoor(2): not translated, its class isn't translated (it extends Minecraft's `DoorBlock`).
    }

    /// `AnchorTreeMaterial` (org/betterx/betternether/blocks/complex/AnchorTreeMaterial.java)
    pub mod anchor_tree_material {
        use super::*;
        // The class itself isn't translated: it extends `NetherWoodenMaterial`. Its static parts are below.
        pub static LOG_VARIANTS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![1, 1, 1]);
        // AnchorTreeMaterial.AnchorTreeMaterial(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // AnchorTreeMaterial.logModel(1): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // AnchorTreeMaterial.barkModel(2): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // AnchorTreeMaterial.trapdoorModel(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // AnchorTreeMaterial.defineExtras(1): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // AnchorTreeMaterial.getSapling(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
    }

    /// `AnchorTreeSaplingModel` (org/betterx/betternether/blocks/complex/AnchorTreeSaplingModel.java)
    pub mod anchor_tree_sapling_model {
        use super::*;
        // MODEL: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        #[derive(Clone, Debug)]
        pub struct AnchorTreeSaplingModel {
        }
        impl AnchorTreeSaplingModel {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
        // AnchorTreeSaplingModel.model(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
    }

    /// `AnchorTreeSaplingModel.ClientModel` (org/betterx/betternether/blocks/complex/AnchorTreeSaplingModel.java)
    pub mod anchor_tree_sapling_model_client_model {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientModel {
        }
        impl ClientModel {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // AnchorTreeSaplingModel.ClientModel.build(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
    }

    /// `ColoredGlassMaterial` (org/betterx/betternether/blocks/complex/ColoredGlassMaterial.java)
    pub mod colored_glass_material {
        use super::*;
        // The class itself isn't translated: it uses `BlockKey` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // ColoredGlassMaterial.ColoredGlassMaterial(3): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // ColoredGlassMaterial.ColoredGlassMaterial(4): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // ColoredGlassMaterial.ColoredGlassMaterial(7): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // ColoredGlassMaterial.makeKey(1): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // ColoredGlassMaterial.register(1): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // ColoredGlassMaterial.define(3): not translated, its class isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
    }

    /// `GloomwoodDarkMaterial` (org/betterx/betternether/blocks/complex/GloomwoodDarkMaterial.java)
    pub mod gloomwood_dark_material {
        use super::*;
        // The class itself isn't translated: it extends `NetherWoodenMaterial`. Its static parts are below.
        // GloomwoodDarkMaterial.GloomwoodDarkMaterial(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // GloomwoodDarkMaterial.logModel(1): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // GloomwoodDarkMaterial.barkModel(2): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // GloomwoodDarkMaterial.trapdoorModel(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // GloomwoodDarkMaterial.hasBoat(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
    }

    /// `GloomwoodMaterial` (org/betterx/betternether/blocks/complex/GloomwoodMaterial.java)
    pub mod gloomwood_material {
        use super::*;
        // The class itself isn't translated: it extends `NetherWoodenMaterial`. Its static parts are below.
        // GloomwoodMaterial.GloomwoodMaterial(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // GloomwoodMaterial.logModel(1): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // GloomwoodMaterial.barkModel(2): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // GloomwoodMaterial.trapdoorModel(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // GloomwoodMaterial.defineExtras(1): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // GloomwoodMaterial.getSapling(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
    }

    /// `MushroomFirMaterial` (org/betterx/betternether/blocks/complex/MushroomFirMaterial.java)
    pub mod mushroom_fir_material {
        use super::*;
        // The class itself isn't translated: it extends `NetherWoodenMaterial`. Its static parts are below.
        // MushroomFirMaterial.MushroomFirMaterial(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // MushroomFirMaterial.trapdoorModel(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // MushroomFirMaterial.defineExtras(1): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // MushroomFirMaterial.getStem(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // MushroomFirMaterial.getSapling(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // MushroomFirMaterial.getTrunk(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
    }

    /// `NetherMushroomMaterial` (org/betterx/betternether/blocks/complex/NetherMushroomMaterial.java)
    pub mod nether_mushroom_material {
        use super::*;
        // The class itself isn't translated: it extends `NetherWoodenMaterial`. Its static parts are below.
        // NetherMushroomMaterial.NetherMushroomMaterial(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // NetherMushroomMaterial.hasLogs(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // NetherMushroomMaterial.withoutKinds(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // NetherMushroomMaterial.strippedLogStandIn(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // NetherMushroomMaterial.defineBeforeSet(1): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // NetherMushroomMaterial.defineBlocks(1): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // NetherMushroomMaterial.planksRecipe(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // NetherMushroomMaterial.getStem(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
    }

    /// `NetherReedMaterial` (org/betterx/betternether/blocks/complex/NetherReedMaterial.java)
    pub mod nether_reed_material {
        use super::*;
        // The class itself isn't translated: it extends `RoofMaterial`. Its static parts are below.
        // NetherReedMaterial.NetherReedMaterial(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // NetherReedMaterial.hasLogs(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // NetherReedMaterial.withoutKinds(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // NetherReedMaterial.strippedLogStandIn(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // NetherReedMaterial.isRaft(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // NetherReedMaterial.ladderModel(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // NetherReedMaterial.trapdoorModel(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // NetherReedMaterial.defineBlocks(1): not translated, its class isn't translated (it extends `RoofMaterial`).
        // NetherReedMaterial.planksRecipe(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // NetherReedMaterial.getStem(0): not translated, its class isn't translated (it extends `RoofMaterial`).
    }

    /// `NetherSakuraMaterial` (org/betterx/betternether/blocks/complex/NetherSakuraMaterial.java)
    pub mod nether_sakura_material {
        use super::*;
        // The class itself isn't translated: it extends `NetherWoodenMaterial`. Its static parts are below.
        // NetherSakuraMaterial.NetherSakuraMaterial(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // NetherSakuraMaterial.defineExtras(1): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // NetherSakuraMaterial.getSapling(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
    }

    /// `NetherWoodenMaterial` (org/betterx/betternether/blocks/complex/NetherWoodenMaterial.java)
    pub mod nether_wooden_material {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NetherWoodenMaterial.NetherWoodenMaterial(3): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.self(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.setFurnitureCloth(1): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.setFurnitureCloth(1): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.init(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.register(1): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.getMaterial(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.createMaterial(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.defineBlocks(1): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.declareExtras(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.defineBeforeSet(1): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.defineExtras(1): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.withoutKinds(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.strippedLogStandIn(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.hasLogs(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.trapdoorModel(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.ladderModel(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.logModel(1): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.barkModel(2): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.logRecipe(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.logFactory(1): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.barkFactory(1): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.logAndBark(2): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.logOrBark(3): not translated, it uses `BlockDefinition` (de.ambertation.wover.block.api).
        // NetherWoodenMaterial.addFurniture(4): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.furniture(4): not translated, it uses `EntryConfig` (de.ambertation.wover.sets.api.classify.BlockSet).
        // NetherWoodenMaterial.key(2): not translated, it's generic.
        // NetherWoodenMaterial.sapling(4): not translated, it's generic.
        // NetherWoodenMaterial.stem(1): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.trunk(4): not translated, it's generic.
        // NetherWoodenMaterial.woodBlock(4): not translated, it's generic.
        // NetherWoodenMaterial.woodBlock(5): not translated, it's generic.
        // NetherWoodenMaterial.dropLog(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.logsBlockTag(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.planks(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.withoutItemKinds(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.defineBoats(1): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.hasBoat(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.isRaft(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.blocks(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.woodType(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.getBlock(1): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.getBlock(2): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.isTreeLog(1): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.isTreeLog(1): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.getPlanks(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.getSlab(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.getLog(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.getBark(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.getStrippedLog(0): not translated, its class isn't translated (it's generic).
        // NetherWoodenMaterial.getStrippedBark(0): not translated, its class isn't translated (it's generic).
    }

    /// `NetherWoodModels` (org/betterx/betternether/blocks/complex/NetherWoodModels.java)
    pub mod nether_wood_models {
        use super::*;
        // LADDER_TEMPLATE: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // TRAPDOOR_TEMPLATE: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // TRAPDOOR_SIDE_TEMPLATE: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // FENCE_POST_TEMPLATE: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // FENCE_SIDE_TEMPLATE: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        #[derive(Clone, Debug)]
        pub struct NetherWoodModels {
        }
        impl NetherWoodModels {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherWoodModels.model(1): not translated, a raw Supplier.
        // NetherWoodModels.external(0): not translated, a raw Supplier.
        // NetherWoodModels.ladder(0): not translated, a raw Supplier.
        // NetherWoodModels.trapdoor(0): not translated, a raw Supplier.
        // NetherWoodModels.trapdoor(1): not translated, a raw Supplier.
        // NetherWoodModels.fence(3): not translated, a raw Supplier.
    }

    /// `RoofMaterial` (org/betterx/betternether/blocks/complex/RoofMaterial.java)
    pub mod roof_material {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // RoofMaterial.RoofMaterial(3): not translated, its class isn't translated (it's generic).
        // RoofMaterial.defineBlocks(1): not translated, its class isn't translated (it's generic).
        // RoofMaterial.noFuel(1): not translated, it uses `BlockDefinition` (de.ambertation.wover.block.api).
        // RoofMaterial.roofRecipe(0): not translated, its class isn't translated (it's generic).
        // RoofMaterial.roofRecipe(3): not translated, its class isn't translated (it's generic).
        // RoofMaterial.roofRecipe(4): not translated, its class isn't translated (it's generic).
        // RoofMaterial.getRoof(0): not translated, its class isn't translated (it's generic).
        // RoofMaterial.getRoofStairs(0): not translated, its class isn't translated (it's generic).
        // RoofMaterial.getRoofSlab(0): not translated, its class isn't translated (it's generic).
    }

    /// `RubeusMaterial` (org/betterx/betternether/blocks/complex/RubeusMaterial.java)
    pub mod rubeus_material {
        use super::*;
        // The class itself isn't translated: it extends `NetherWoodenMaterial`. Its static parts are below.
        // RubeusMaterial.RubeusMaterial(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // RubeusMaterial.logModel(1): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // RubeusMaterial.barkModel(2): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // RubeusMaterial.logFactory(1): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // RubeusMaterial.barkFactory(1): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // RubeusMaterial.defineExtras(1): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // RubeusMaterial.getCone(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
        // RubeusMaterial.getSapling(0): not translated, its class isn't translated (it extends `NetherWoodenMaterial`).
    }

    /// `StalagnateMaterial` (org/betterx/betternether/blocks/complex/StalagnateMaterial.java)
    pub mod stalagnate_material {
        use super::*;
        // The class itself isn't translated: it extends `RoofMaterial`. Its static parts are below.
        // StalagnateMaterial.StalagnateMaterial(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // StalagnateMaterial.logRecipe(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // StalagnateMaterial.defineExtras(1): not translated, its class isn't translated (it extends `RoofMaterial`).
        // StalagnateMaterial.getTrunk(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // StalagnateMaterial.getStem(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // StalagnateMaterial.getBowl(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // StalagnateMaterial.getSeed(0): not translated, its class isn't translated (it extends `RoofMaterial`).
    }

    /// `VanillaFallback` (org/betterx/betternether/blocks/complex/VanillaFallback.java)
    pub mod vanilla_fallback {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // VanillaFallback.VanillaFallback(3): not translated, its class isn't translated (it's generic).
        // VanillaFallback.createMaterial(0): not translated, its class isn't translated (it's generic).
        // VanillaFallback.material(0): not translated, its class isn't translated (it's generic).
        // VanillaFallback.addVanillaFurniture(1): not translated, its class isn't translated (it's generic).
        // VanillaFallback.burning(1): not translated, its class isn't translated (it's generic).
        // VanillaFallback.planks(0): not translated, its class isn't translated (it's generic).
        // VanillaFallback.vanilla(1): not translated, its class isn't translated (it's generic).
        // VanillaFallback.getVanillaBlock(2): not translated, it uses Minecraft's `Block`.
    }

    /// `VanillaNetherWood` (org/betterx/betternether/blocks/complex/VanillaNetherWood.java)
    pub mod vanilla_nether_wood {
        use super::*;
        // The class itself isn't translated: it extends `VanillaFallback`. Its static parts are below.
        // VanillaNetherWood.VanillaNetherWood(3): not translated, its class isn't translated (it extends `VanillaFallback`).
        // VanillaNetherWood.material(0): not translated, its class isn't translated (it extends `VanillaFallback`).
        // VanillaNetherWood.defineBlocks(1): not translated, its class isn't translated (it extends `VanillaFallback`).
    }

    /// `VanillaWood` (org/betterx/betternether/blocks/complex/VanillaWood.java)
    pub mod vanilla_wood {
        use super::*;
        // The class itself isn't translated: it extends `VanillaFallback`. Its static parts are below.
        // VanillaWood.VanillaWood(3): not translated, its class isn't translated (it extends `VanillaFallback`).
        // VanillaWood.defineBlocks(1): not translated, its class isn't translated (it extends `VanillaFallback`).
        // VanillaWood.burning(1): not translated, its class isn't translated (it extends `VanillaFallback`).
        // VanillaWood.hasBoat(0): not translated, its class isn't translated (it extends `VanillaFallback`).
        // VanillaWood.create(2): not translated, it uses VanillaWood, which isn't translated (it extends `VanillaFallback`).
    }

    /// `WartMaterial` (org/betterx/betternether/blocks/complex/WartMaterial.java)
    pub mod wart_material {
        use super::*;
        // The class itself isn't translated: it extends `RoofMaterial`. Its static parts are below.
        // WartMaterial.WartMaterial(3): not translated, its class isn't translated (it extends `RoofMaterial`).
        // WartMaterial.trapdoorModel(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // WartMaterial.defineExtras(1): not translated, its class isn't translated (it extends `RoofMaterial`).
        // WartMaterial.getRoot(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // WartMaterial.getSeed(0): not translated, its class isn't translated (it extends `RoofMaterial`).
    }

    /// `WillowMaterial` (org/betterx/betternether/blocks/complex/WillowMaterial.java)
    pub mod willow_material {
        use super::*;
        // The class itself isn't translated: it extends `RoofMaterial`. Its static parts are below.
        pub static LOG_VARIANTS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![1, 1]);
        // WillowMaterial.WillowMaterial(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // WillowMaterial.trapdoorModel(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // WillowMaterial.logModel(1): not translated, its class isn't translated (it extends `RoofMaterial`).
        // WillowMaterial.barkModel(2): not translated, its class isn't translated (it extends `RoofMaterial`).
        // WillowMaterial.declareExtras(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // WillowMaterial.defineExtras(1): not translated, its class isn't translated (it extends `RoofMaterial`).
        // WillowMaterial.willowTrunkModel(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // WillowMaterial.getTrunk(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // WillowMaterial.getBranch(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // WillowMaterial.getSapling(0): not translated, its class isn't translated (it extends `RoofMaterial`).
        // WillowMaterial.getTorch(0): not translated, its class isn't translated (it extends `RoofMaterial`).
    }

    /// `NetherLoot` (org/betterx/betternether/blocks/NetherLoot.java)
    pub mod nether_loot {
        use super::*;
        // NONE: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // DROP_SOUL_SAND: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // DROP_LOG: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // NETHER_GRASS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // GIANT_LUCIS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // ONLY_TOP_DROPS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // WHISPERING_GOURD_VINE: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // LUMABUS_VINE: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // BLACK_APPLE: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // BONE_MUSHROOM: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // FEATHER_FERN: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // MAGMA_FLOWER: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // ORANGE_MUSHROOM: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // INK_BUSH: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // EYEBALL: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // EYEBALL_SMALL: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // RED_LARGE_MUSHROOM: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // BROWN_LARGE_MUSHROOM: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // GIANT_MOLD: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        pub const MUSHROOM_FIR_SAPLING_CHANCE: f32 = 0.2f32;
        // MUSHROOM_FIR_TRUNK: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // STALAGNATE_TRUNK: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // PIG_STATUE_RESPAWNER: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // LUCIS_MUSHROOM: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // BARREL_CACTUS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // AGAVE: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // JELLYFISH_MUSHROOM: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // WILLOW_BRANCH: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // SOUL_LILY: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // CINCINNASITE_ANVIL: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // STALAGNATE_BOWL: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // POTTED_PLANT: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        #[derive(Clone, Debug)]
        pub struct NetherLoot {
        }
        impl NetherLoot {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherLoot.none(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.dropSoulSand(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.dropLog(1): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        pub fn ensure_loaded() {
        }
        // NetherLoot.netherGrass(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.giantLucis(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.onlyTopDrops(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.whisperingGourdVine(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.lumabusVine(1): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.stateIs(4): not translated, it's generic.
        // NetherLoot.stateIs(4): not translated, it's generic.
        // NetherLoot.stateIs(4): not translated, it's generic.
        // NetherLoot.survivingPool(1): not translated, it uses Minecraft's `ItemLike`.
        // NetherLoot.ripePool(6): not translated, it uses `LootLookupProvider` (de.ambertation.wover.loot.api).
        // NetherLoot.ripePlant(6): not translated, it uses Minecraft's `Builder`.
        // NetherLoot.blackApple(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.boneMushroom(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.featherFern(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.magmaFlower(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.orangeMushroom(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.inkBush(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.eyeball(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.eyeballSmall(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.redLargeMushroom(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.brownLargeMushroom(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.giantMold(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.mushroomFirTrunk(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.stalagnateTrunk(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.pigStatueRespawner(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.stateIsAnyOf(5): not translated, it's generic.
        // NetherLoot.toolIsIn(2): not translated, it uses Minecraft's `Builder`.
        // NetherLoot.lucisMushroom(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.barrelCactus(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.agave(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.jellyfishMushroom(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.willowBranch(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.soulLily(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.cincinnasiteAnvil(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.stalagnateBowl(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherLoot.pottedPlant(0): not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
    }

    /// `NetherLoot.DropLog` (org/betterx/betternether/blocks/NetherLoot.java)
    pub mod nether_loot_drop_log {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherLoot.DropLog.DropLog(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherLoot.DropLog.log(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherLoot.LumabusVine` (org/betterx/betternether/blocks/NetherLoot.java)
    pub mod nether_loot_lumabus_vine {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherLoot.LumabusVine.LumabusVine(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherLoot.LumabusVine.seed(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherModels` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models {
        use super::*;
        // BASALT_BRICKS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // SLAB: not translated, it uses `Template4` (de.ambertation.wover.core.api.template).
        // SLAB_COLUMN_DOUBLE: not translated, it uses `Template2` (de.ambertation.wover.core.api.template).
        // SOUL_SANDSTONE_STAIRS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // SOUL_SANDSTONE_SMOOTH_STAIRS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // SOUL_SANDSTONE_CUT_STAIRS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // REED_STAIRS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // PRESSURE_PLATE: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // BUTTON: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // CHAIR_CINCINNASITE: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // BAR_STOOL_CINCINNASITE: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // TABURET_CINCINNASITE: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // GRASS: not translated, it uses `Template2` (de.ambertation.wover.core.api.template).
        // NETHER_GRASS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // TERRAIN_COVER: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // CUBE_WITH_BORROWED_TOP_AND_BOTTOM: not translated, it uses `Template2` (de.ambertation.wover.core.api.template).
        // JUNGLE_PLANT: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // QUARTZ_GLASS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // OBSIDIAN_VARIANTS: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // CUBE_FROM_TEXTURE: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        #[derive(Clone, Debug)]
        pub struct NetherModels {
        }
        impl NetherModels {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
        // NetherModels.basaltBricks(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.slab(4): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.slabColumnDouble(2): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.soulSandstoneStairs(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.soulSandstoneSmoothStairs(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.soulSandstoneCutStairs(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.reedStairs(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.pressurePlate(1): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.button(1): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.chairCincinnasite(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.barStoolCincinnasite(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.taburetCincinnasite(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.grass(2): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.netherGrass(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.terrainCover(1): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.terrainCoverVariants(3): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.cubeWithBorrowedTopAndBottom(2): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.junglePlant(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.quartzGlass(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.obsidianVariants(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.debugItem(1): not translated, it uses `ItemModel` (de.ambertation.wover.item.api.model).
        // NetherModels.gloomwispVineModelTrait(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.lumabusVineModelTrait(1): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.cubeFromTexture(1): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
        // NetherModels.netherSakuraLeavesModelTrait(0): not translated, it uses `BlockModel` (de.ambertation.wover.block.api.model).
    }

    /// `NetherModels.BuildParams` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_build_params {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct BuildParams {
            pub texture: JIdentifier,
        }
        impl BuildParams {
            pub fn new(texture: &JIdentifier) -> Self {
                let mut s = Self { texture: texture.clone() };
                s.texture = texture.clone();
                s
            }
            pub fn texture(&self) -> JIdentifier {
                return self.texture.clone();
            }
        }
    }

    /// `NetherModels.Button` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_button {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct Button {
            pub texture: String,
        }
        impl Button {
            pub fn new(texture: &str) -> Self {
                let mut s = Self { texture: String::new() };
                s.texture = texture.to_string();
                s
            }
            pub fn texture(&self) -> String {
                return self.texture.to_string();
            }
        }
    }

    /// `NetherModels.ButtonParams` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_button_params {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct ButtonParams {
            pub texture: String,
        }
        impl ButtonParams {
            pub fn new(texture: &str) -> Self {
                let mut s = Self { texture: String::new() };
                s.texture = texture.to_string();
                s
            }
            pub fn texture(&self) -> String {
                return self.texture.to_string();
            }
        }
    }

    /// `NetherModels.ClientCubeModel` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_client_cube_model {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientCubeModel {
        }
        impl ClientCubeModel {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherModels.ClientCubeModel.buildFrom(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.ClientCubeModel.build(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
    }

    /// `NetherModels.CubeFromTexture` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_cube_from_texture {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct CubeFromTexture {
            pub texture: JIdentifier,
        }
        impl CubeFromTexture {
            pub fn new(texture: &JIdentifier) -> Self {
                let mut s = Self { texture: texture.clone() };
                s.texture = texture.clone();
                s
            }
            pub fn texture(&self) -> JIdentifier {
                return self.texture.clone();
            }
        }
    }

    /// `NetherModels.CubeWithBorrowedTopAndBottom` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_cube_with_borrowed_top_and_bottom {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherModels.CubeWithBorrowedTopAndBottom.CubeWithBorrowedTopAndBottom(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherModels.CubeWithBorrowedTopAndBottom.top(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherModels.CubeWithBorrowedTopAndBottom.bottom(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherModels.CubeWithBorrowedTopAndBottomParams` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_cube_with_borrowed_top_and_bottom_params {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // NetherModels.CubeWithBorrowedTopAndBottomParams.CubeWithBorrowedTopAndBottomParams(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherModels.CubeWithBorrowedTopAndBottomParams.top(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherModels.CubeWithBorrowedTopAndBottomParams.bottom(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherModels.Grass` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_grass {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct Grass {
            pub name: String,
            pub variants: i32,
        }
        impl Grass {
            pub fn new(name: &str, variants: i32) -> Self {
                let mut s = Self { name: String::new(), variants: variants };
                s.name = name.to_string();
                s.variants = variants;
                s
            }
            pub fn name(&self) -> String {
                return self.name.to_string();
            }
            pub fn variants(&self) -> i32 {
                return self.variants;
            }
        }
    }

    /// `NetherModels.GrassParams` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_grass_params {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct GrassParams {
            pub name: String,
            pub variants: i32,
        }
        impl GrassParams {
            pub fn new(name: &str, variants: i32) -> Self {
                let mut s = Self { name: String::new(), variants: variants };
                s.name = name.to_string();
                s.variants = variants;
                s
            }
            pub fn name(&self) -> String {
                return self.name.to_string();
            }
            pub fn variants(&self) -> i32 {
                return self.variants;
            }
        }
    }

    /// `NetherModels.Impl` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_impl {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Impl {
        }
        impl Impl {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherModels.Impl.grassFrom(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.terrainCoverFrom(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.cubeWithBorrowedTopAndBottomFrom(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.slabFrom(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.slabColumnDoubleFrom(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.pressurePlateFrom(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.buttonFrom(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.debugItem(1): not translated, it uses `ItemModel` (de.ambertation.wover.item.api.model).
        // NetherModels.Impl.grass(2): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.terrainCover(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.cubeWithBorrowedTopAndBottom(2): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.netherGrass(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.junglePlant(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.quartzGlass(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.obsidianVariants(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.basaltBricks(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.slab(4): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.slabColumnDouble(2): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.soulSandstoneStairs(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.soulSandstoneSmoothStairs(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.soulSandstoneCutStairs(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.reedStairs(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.pressurePlate(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.button(1): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.chairCincinnasite(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.barStoolCincinnasite(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.taburetCincinnasite(0): not translated, it uses `BlockModelFactory` (de.ambertation.wover.block.api.model).
        // NetherModels.Impl.toArray(1): not translated, it creates `JsonArray` (com.google.gson).
    }

    /// `NetherModels.PressurePlate` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_pressure_plate {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct PressurePlate {
            pub texture: String,
        }
        impl PressurePlate {
            pub fn new(texture: &str) -> Self {
                let mut s = Self { texture: String::new() };
                s.texture = texture.to_string();
                s
            }
            pub fn texture(&self) -> String {
                return self.texture.to_string();
            }
        }
    }

    /// `NetherModels.PressurePlateParams` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_pressure_plate_params {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct PressurePlateParams {
            pub texture: String,
        }
        impl PressurePlateParams {
            pub fn new(texture: &str) -> Self {
                let mut s = Self { texture: String::new() };
                s.texture = texture.to_string();
                s
            }
            pub fn texture(&self) -> String {
                return self.texture.to_string();
            }
        }
    }

    /// `NetherModels.Slab` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_slab {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherModels.Slab.Slab(4): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherModels.Slab.source(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherModels.Slab.top(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherModels.Slab.bottom(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherModels.Slab.side(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherModels.SlabColumnDouble` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_slab_column_double {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct SlabColumnDouble {
            pub top_bottom: JIdentifier,
            pub side: JIdentifier,
        }
        impl SlabColumnDouble {
            pub fn new(top_bottom: &JIdentifier, side: &JIdentifier) -> Self {
                let mut s = Self { top_bottom: top_bottom.clone(), side: side.clone() };
                s.top_bottom = top_bottom.clone();
                s.side = side.clone();
                s
            }
            pub fn top_bottom(&self) -> JIdentifier {
                return self.top_bottom.clone();
            }
            pub fn side(&self) -> JIdentifier {
                return self.side.clone();
            }
        }
    }

    /// `NetherModels.SlabColumnDoubleParams` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_slab_column_double_params {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct SlabColumnDoubleParams {
            pub top_bottom: JIdentifier,
            pub side: JIdentifier,
        }
        impl SlabColumnDoubleParams {
            pub fn new(top_bottom: &JIdentifier, side: &JIdentifier) -> Self {
                let mut s = Self { top_bottom: top_bottom.clone(), side: side.clone() };
                s.top_bottom = top_bottom.clone();
                s.side = side.clone();
                s
            }
            pub fn top_bottom(&self) -> JIdentifier {
                return self.top_bottom.clone();
            }
            pub fn side(&self) -> JIdentifier {
                return self.side.clone();
            }
        }
    }

    /// `NetherModels.SlabParams` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_slab_params {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // NetherModels.SlabParams.SlabParams(4): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherModels.SlabParams.source(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherModels.SlabParams.top(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherModels.SlabParams.bottom(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherModels.SlabParams.side(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherModels.TerrainCover` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_terrain_cover {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherModels.TerrainCover.TerrainCover(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherModels.TerrainCover.bottom(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherModels.TerrainCoverParams` (org/betterx/betternether/blocks/NetherModels.java)
    pub mod nether_models_terrain_cover_params {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // NetherModels.TerrainCoverParams.TerrainCoverParams(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherModels.TerrainCoverParams.bottom(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `RedstoneOreBlock` (org/betterx/betternether/blocks/RedstoneOreBlock.java)
    pub mod redstone_ore_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RedStoneOreBlock`. Its static parts are below.
        // RedstoneOreBlock.RedstoneOreBlock(1): not translated, its class isn't translated (it extends Minecraft's `RedStoneOreBlock`).
        // RedstoneOreBlock.litBlockEmission(1): not translated, it uses Minecraft's `BlockState`.
    }

    /// `RubeusBark` (org/betterx/betternether/blocks/RubeusBark.java)
    pub mod rubeus_bark {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RotatedPillarBlock`. Its static parts are below.
        // RubeusBark.RubeusBark(1): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
        // RubeusBark.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
    }

    /// `RubeusLog` (org/betterx/betternether/blocks/RubeusLog.java)
    pub mod rubeus_log {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RotatedPillarBlock`. Its static parts are below.
        // SHAPE: not translated, it uses Minecraft's `EnumProperty`.
        // RubeusLog.RubeusLog(1): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
        // RubeusLog.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `RotatedPillarBlock`).
    }

    /// `VanillaWeepingObsidianBlock` (org/betterx/betternether/blocks/VanillaWeepingObsidianBlock.java)
    pub mod vanilla_weeping_obsidian_block {
        use super::*;
        // The class itself isn't translated: it extends `BNObsidianBase`. Its static parts are below.
        // VanillaWeepingObsidianBlock.VanillaWeepingObsidianBlock(1): not translated, its class isn't translated (it extends `BNObsidianBase`).
        // VanillaWeepingObsidianBlock.animateTick(4): not translated, its class isn't translated (it extends `BNObsidianBase`).
    }

    /// `WispRotation` (org/betterx/betternether/blocks/WispRotation.java)
    pub mod wisp_rotation {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // WispRotation.WispRotation(1): not translated, its class isn't translated (its constants carry values).
        // WispRotation.yRotation(0): not translated, its class isn't translated (its constants carry values).
        // WispRotation.forHorizontal(1): not translated, it uses WispRotation, which isn't translated (its constants carry values).
        // WispRotation.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `BN` (org/betterx/betternether/BN.java)
    pub mod bn {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BN {
        }
        impl BN {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BN.id(1): not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
    }

    /// `BetterNetherClient` (org/betterx/betternether/client/BetterNetherClient.java)
    pub mod better_nether_client {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        // BetterNetherClient.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
    }

    /// `BNModels` (org/betterx/betternether/client/block/BNModels.java)
    pub mod bn_models {
        use super::*;
        // TEMPLATES: not translated, it uses Minecraft's `ModelTemplate`.
        // CROP_BLOCK_MODEL_LOCATION: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // JUNGLE_PLANT_MODEL_LOCATION: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // GRASS_FAN_MODEL_LOCATION: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // STAIRS_WITH_TOP_MODEL_LOCATION: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // STAIRS_WITH_TOP_MODEL: not translated, it uses Minecraft's `ModelTemplate`.
        #[derive(Clone, Debug)]
        pub struct BNModels {
        }
        impl BNModels {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BNModels.createComplex(3): not translated, it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model).
        // BNModels.createComplex(4): not translated, it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model).
        // BNModels.getCropBlockModelTemplate(1): not translated, it uses Minecraft's `ModelTemplate`.
        // BNModels.getCropCrossBlockModelTemplate(1): not translated, it uses Minecraft's `ModelTemplate`.
        // BNModels.provideGrassBlockModels(4): not translated, it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model).
        // BNModels.provideSimpleMultiStateBlock(3): not translated, it uses `WoverBlockModelGenerators` (de.ambertation.wover.block.api.model).
    }

    /// `BNModels.ModelSource` (org/betterx/betternether/client/block/BNModels.java)
    pub mod bn_models_model_source {
        use super::*;
        // The class itself isn't translated: it uses VariantSupplier, which isn't translated (it's an interface). Its static parts are below.
        // BNModels.ModelSource.ModelSource(4): not translated, its class isn't translated (it uses VariantSupplier, which isn't translated (it's an interface)).
        // BNModels.ModelSource.of(4): not translated, it uses ModelSource, which isn't translated (it uses VariantSupplier, which isn't translated (it's an interface)).
        // BNModels.ModelSource.parent(0): not translated, its class isn't translated (it uses VariantSupplier, which isn't translated (it's an interface)).
        // BNModels.ModelSource.suffix(0): not translated, its class isn't translated (it uses VariantSupplier, which isn't translated (it's an interface)).
        // BNModels.ModelSource.variants(0): not translated, its class isn't translated (it uses VariantSupplier, which isn't translated (it's an interface)).
        // BNModels.ModelSource.textures(0): not translated, its class isn't translated (it uses VariantSupplier, which isn't translated (it's an interface)).
    }

    /// `BNModels.TextureSource` (org/betterx/betternether/client/block/BNModels.java)
    pub mod bn_models_texture_source {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `TextureSlot`. Its static parts are below.
        // BNModels.TextureSource.TextureSource(2): not translated, its class isn't translated (it uses Minecraft's `TextureSlot`).
        // BNModels.TextureSource.of(2): not translated, it uses TextureSource, which isn't translated (it uses Minecraft's `TextureSlot`).
        // BNModels.TextureSource.slot(0): not translated, its class isn't translated (it uses Minecraft's `TextureSlot`).
        // BNModels.TextureSource.texture(0): not translated, its class isn't translated (it uses Minecraft's `TextureSlot`).
    }

    /// `BNModels.VariantSupplier` (org/betterx/betternether/client/block/BNModels.java)
    pub mod bn_models_variant_supplier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BNModels.VariantSupplier.apply(2): not translated, it has no body.
    }

    /// `CommandRegistry` (org/betterx/betternether/commands/CommandRegistry.java)
    pub mod command_registry {
        use super::*;
        // ERROR_BIOME_NOT_FOUND: not translated, it uses Minecraft's `DynamicCommandExceptionType`.
        pub const MAX_SEARCH_RADIUS: i32 = 12800;
        pub const SAMPLE_RESOLUTION_HORIZONTAL: i32 = 32;
        pub const SAMPLE_RESOLUTION_VERTICAL: i32 = 64;
        pub static BIOME_INDEX: JCell<i32> = JCell::new(0);
        pub static PLACE_MAP_IDX: JCell<i32> = JCell::new(0);
        // states: not translated, it uses Minecraft's `BlockState`.
        // states2: not translated, it uses Minecraft's `BlockState`.
        // BIOME_INFO_NOISE: not translated, it uses Minecraft's `PerlinNoise`.
        #[derive(Clone, Debug)]
        pub struct CommandRegistry {
        }
        impl CommandRegistry {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CommandRegistry.register(0): not translated, it uses Fabric's `CommandRegistrationCallback`.EVENT.
        // CommandRegistry.register(3): not translated, it uses Minecraft's `CommandDispatcher`.
        // CommandRegistry.teleportToNextBiome(1): not translated, its body couldn't be read: an anonymous class.
        // CommandRegistry.placeMatchingBlocks(2): not translated, it uses Minecraft's `CommandContext`.
        // CommandRegistry.compareBlockNames(2): not translated, it uses Minecraft's `Block`.
        // CommandRegistry.findSurface(1): not translated, it uses Minecraft's `CommandContext`.
        // CommandRegistry.testPlace(1): not translated, it uses Minecraft's `CommandContext`.
        // CommandRegistry.placeAllBlocks(1): not translated, it uses Minecraft's `CommandContext`.
        // CommandRegistry.placeBlockRow(5): not translated, it uses Minecraft's `ServerPlayer`.
        // CommandRegistry.place(4): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `PlaceCommand` (org/betterx/betternether/commands/PlaceCommand.java)
    pub mod place_command {
        use super::*;
        // ERROR_STRUCTURE_INVALID: not translated, it uses Minecraft's `DynamicCommandExceptionType`.
        // ERROR_NBT_STRUCTURE_NOT_FOUND: not translated, it uses Minecraft's `DynamicCommandExceptionType`.
        #[derive(Clone, Debug)]
        pub struct PlaceCommand {
        }
        impl PlaceCommand {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PlaceCommand.register(1): not translated, it uses Minecraft's `LiteralArgumentBuilder`.
        // PlaceCommand.placeStructure(2): not translated, it uses Minecraft's `CommandSourceStack`.
    }

    /// `ClientConfig` (org/betterx/betternether/config/ClientConfig.java)
    pub mod client_config {
        use super::*;
        // The class itself isn't translated: it extends `ConfigFile` (de.ambertation.wunderlib.configs). Its static parts are below.
        // ClientConfig.ClientConfig(0): not translated, its class isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)).
    }

    /// `Config` (org/betterx/betternether/config/Config.java)
    pub mod config {
        use super::*;
        // ALL: not translated, it calls `Lists` (com.google.common.collect).newArrayList.
        #[derive(Clone, Debug)]
        pub struct Config {
            pub config: Option<JJson>,
            pub rewrite: bool,
            pub name: String,
        }
        impl Config {
            // Config.Config(1): not translated, it uses Config.ALL, which isn't translated (it calls `Lists` (com.google.common.collect).newArrayList).
            // Config.load(0): not translated, it uses `File` (java.io).
            // Config.getFile(0): not translated, it uses `File` (java.io).
            // Config.getFolder(0): not translated, it uses `File` (java.io).
            // Config.getBoolean(3): not translated, it calls Config.load, which isn't translated.
            // Config.setBoolean(4): not translated, it changes an object through a second name.
            // Config.getFloat(3): not translated, it calls Config.load, which isn't translated.
            // Config.setFloat(4): not translated, it changes an object through a second name.
            // Config.getInt(3): not translated, it calls Config.load, which isn't translated.
            // Config.getString(3): not translated, it calls Config.load, which isn't translated.
            // Config.setInt(4): not translated, it changes an object through a second name.
            // Config.setStringLoad(3): not translated, it changes an object through a second name.
            // Config.getStringArray(3): not translated, it calls Config.load, which isn't translated.
            // Config.toStringArray(1): not translated, it calls Config.load, which isn't translated.
            // Config.toJsonArray(1): not translated, it calls Config.load, which isn't translated.
            // Config.getGroup(1): not translated, it changes an object through a second name.
            // Config.getBaseGroups(0): not translated, .iterator on a map's entries.
            // Config.getGroupMembers(1): not translated, Minecraft's entry kept in a variable or field.
            pub fn mark_to_save(&mut self) {
                self.rewrite = true;
            }
        }
        // Config.save(0): not translated, it uses Config.ALL, which isn't translated (it calls `Lists` (com.google.common.collect).newArrayList).
    }

    /// `Configs` (org/betterx/betternether/config/Configs.java)
    pub mod configs {
        use super::*;
        // CLIENT: not translated, it uses ClientConfig, which isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)).
        // GAME_RULES: not translated, it uses DefaultWorldConfig, which isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)).
        // WORLD: not translated, it uses WorldConfig, which isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)).
        #[derive(Clone, Debug)]
        pub struct Configs {
        }
        impl Configs {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Configs.saveConfigs(0): not translated, unknown name `de`.
    }

    /// `DefaultWorldConfig` (org/betterx/betternether/config/DefaultWorldConfig.java)
    pub mod default_world_config {
        use super::*;
        // The class itself isn't translated: it extends `ConfigFile` (de.ambertation.wunderlib.configs). Its static parts are below.
        // DAMAGE_GROUP: not translated, it uses `Group` (de.ambertation.wunderlib.configs.AbstractConfig).
        // DefaultWorldConfig.DefaultWorldConfig(0): not translated, its class isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)).
    }

    /// `ConfigScreen` (org/betterx/betternether/config/screen/ConfigScreen.java)
    pub mod config_screen {
        use super::*;
        // The class itself isn't translated: it extends `ConfigScreen`. Its static parts are below.
        // BN_LOGO_LOCATION: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // ConfigScreen.ConfigScreen(1): not translated, its class isn't translated (it extends `ConfigScreen`).
        // ConfigScreen.onClose(0): not translated, its class isn't translated (it extends `ConfigScreen`).
    }

    /// `WorldConfig` (org/betterx/betternether/config/WorldConfig.java)
    pub mod world_config {
        use super::*;
        // The class itself isn't translated: it extends `ConfigFile` (de.ambertation.wunderlib.configs). Its static parts are below.
        // WorldConfig.WorldConfig(0): not translated, its class isn't translated (it extends `ConfigFile` (de.ambertation.wunderlib.configs)).
    }

    /// `ObsidianBreaker` (org/betterx/betternether/enchantments/ObsidianBreaker.java)
    pub mod obsidian_breaker {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ObsidianBreaker {
        }
        impl ObsidianBreaker {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ObsidianBreaker.modifyObsidianBreakerSpeed(3): not translated, it uses Minecraft's `BlockState`.
    }

    /// `RubyFire` (org/betterx/betternether/enchantments/RubyFire.java)
    pub mod ruby_fire {
        use super::*;
        // ARMOR_SLOTS: not translated, it uses Minecraft's `EquipmentSlot`.
        // FIRE_CONVERSIONS: not translated, it uses Minecraft's `Item`.
        // convertedDrops: not translated, it uses `ThreadLocal`.
        #[derive(Clone, Debug)]
        pub struct RubyFire {
        }
        impl RubyFire {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RubyFire.getDrops(5): not translated, it uses Minecraft's `BlockState`.
        // RubyFire.popExperience(3): not translated, it uses Minecraft's `ServerLevel`.
        // RubyFire.buildConversionTable(1): not translated, it uses Minecraft's `ServerLevel`.
    }

    /// `EntityFirefly` (org/betterx/betternether/entity/EntityFirefly.java)
    pub mod entity_firefly {
        use super::*;
        // The class itself isn't translated: it extends `DespawnableAnimal` (org.betterx.bclib.entity). Its static parts are below.
        // SEARCH: not translated, it uses Minecraft's `Vec3i`.
        // COLOR: not translated, it uses Minecraft's `EntityDataAccessor`.
        // EntityFirefly.EntityFirefly(2): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.defineSynchedData(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.createMobAttributes(0): not translated, it uses Minecraft's `Builder`.
        // EntityFirefly.createNavigation(1): not translated, its body couldn't be read: an anonymous class.
        // EntityFirefly.registerGoals(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.getWalkTargetValue(2): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.isFood(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.isFlapping(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.jumpInLiquid(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.getLightLevelDependentMagicValue(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.causeFallDamage(3): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.checkFallDamage(4): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.isNoGravity(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.getColor(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.readAdditionalSaveData(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.getBreedOffspring(2): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.omnidirectionalAirMover(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.checkMovement(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.getAmbientSound(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.getSoundVolume(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.makeColor(3): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.getMaxSpawnClusterSize(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFirefly.canSpawn(5): not translated, it uses Minecraft's `EntityType`.
        // EntityFirefly.isPushable(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
    }

    /// `EntityFirefly.FreflyLookControl` (org/betterx/betternether/entity/EntityFirefly.java)
    pub mod entity_firefly_frefly_look_control {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `LookControl`. Its static parts are below.
        // EntityFirefly.FreflyLookControl.FreflyLookControl(1): not translated, its class isn't translated (it extends Minecraft's `LookControl`).
        // EntityFirefly.FreflyLookControl.resetXRotOnTick(0): not translated, its class isn't translated (it extends Minecraft's `LookControl`).
    }

    /// `EntityFirefly.MoveRandomGoal` (org/betterx/betternether/entity/EntityFirefly.java)
    pub mod entity_firefly_move_random_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // EntityFirefly.MoveRandomGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.MoveRandomGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.MoveRandomGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.MoveRandomGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `EntityFirefly.MoveToFlowersGoal` (org/betterx/betternether/entity/EntityFirefly.java)
    pub mod entity_firefly_move_to_flowers_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // EntityFirefly.MoveToFlowersGoal.MoveToFlowersGoal(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.MoveToFlowersGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.MoveToFlowersGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.MoveToFlowersGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.MoveToFlowersGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.MoveToFlowersGoal.getFlowerLocation(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.MoveToFlowersGoal.isFlower(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.MoveToFlowersGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `EntityFirefly.SittingGoal` (org/betterx/betternether/entity/EntityFirefly.java)
    pub mod entity_firefly_sitting_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // EntityFirefly.SittingGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.SittingGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.SittingGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.SittingGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.SittingGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `EntityFirefly.WanderAroundGoal` (org/betterx/betternether/entity/EntityFirefly.java)
    pub mod entity_firefly_wander_around_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // EntityFirefly.WanderAroundGoal.WanderAroundGoal(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.WanderAroundGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.WanderAroundGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.WanderAroundGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.WanderAroundGoal.getRandomLocation(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.WanderAroundGoal.randomRange(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFirefly.WanderAroundGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `EntityFlyingPig` (org/betterx/betternether/entity/EntityFlyingPig.java)
    pub mod entity_flying_pig {
        use super::*;
        // The class itself isn't translated: it extends `DespawnableAnimal` (org.betterx.bclib.entity). Its static parts are below.
        // DATA_SHARED_FLAGS_ID: not translated, it uses Minecraft's `EntityDataAccessor`.
        pub const BIT_ROOSTING: i32 = 0;
        pub const BIT_WARTED: i32 = 1;
        // EntityFlyingPig.EntityFlyingPig(2): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.getFlyingSpeed(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.registerGoals(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.createMobAttributes(0): not translated, it calls Minecraft's `Mob`.createMobAttributes.
        // EntityFlyingPig.createNavigation(1): not translated, its body couldn't be read: an anonymous class.
        // EntityFlyingPig.defineSynchedData(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.addAdditionalSaveData(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.readAdditionalSaveData(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.isRoosting(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.setRoosting(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.isWarted(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.setWarted(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.getSoundVolume(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.getVoicePitch(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.classicPigSounds(0): not translated, it uses Minecraft's `PigSoundSet`.
        // EntityFlyingPig.getHurtSound(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.getDeathSound(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.getAmbientSound(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.isPushable(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.doPush(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.pushEntities(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.isFlapping(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.causeFallDamage(3): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.checkFallDamage(4): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.tickDeath(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.getMaxSpawnClusterSize(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.omnidirectionalAirMover(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.getBreedOffspring(2): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityFlyingPig.isFood(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
    }

    /// `EntityFlyingPig.FindFoodGoal` (org/betterx/betternether/entity/EntityFlyingPig.java)
    pub mod entity_flying_pig_find_food_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // EntityFlyingPig.FindFoodGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.FindFoodGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.FindFoodGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.FindFoodGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.FindFoodGoal.getFood(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.FindFoodGoal.hasNearFood(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `EntityFlyingPig.RoostingGoal` (org/betterx/betternether/entity/EntityFlyingPig.java)
    pub mod entity_flying_pig_roosting_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // EntityFlyingPig.RoostingGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.RoostingGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.RoostingGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.RoostingGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.RoostingGoal.getRoostingLocation(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `EntityFlyingPig.SittingGoal` (org/betterx/betternether/entity/EntityFlyingPig.java)
    pub mod entity_flying_pig_sitting_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // EntityFlyingPig.SittingGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.SittingGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.SittingGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.SittingGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.SittingGoal.tick(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `EntityFlyingPig.WanderAroundGoal` (org/betterx/betternether/entity/EntityFlyingPig.java)
    pub mod entity_flying_pig_wander_around_goal {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Goal`. Its static parts are below.
        // EntityFlyingPig.WanderAroundGoal.WanderAroundGoal(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.WanderAroundGoal.canUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.WanderAroundGoal.canContinueToUse(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.WanderAroundGoal.start(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.WanderAroundGoal.getRandomLocation(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.WanderAroundGoal.randomRange(1): not translated, its class isn't translated (it extends Minecraft's `Goal`).
        // EntityFlyingPig.WanderAroundGoal.stop(0): not translated, its class isn't translated (it extends Minecraft's `Goal`).
    }

    /// `EntityHydrogenJellyfish` (org/betterx/betternether/entity/EntityHydrogenJellyfish.java)
    pub mod entity_hydrogen_jellyfish {
        use super::*;
        // The class itself isn't translated: it extends `DespawnableAnimal` (org.betterx.bclib.entity). Its static parts are below.
        // EntityHydrogenJellyfish.EntityHydrogenJellyfish(2): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityHydrogenJellyfish.createMobAttributes(0): not translated, it calls Minecraft's `Mob`.createMobAttributes.
        // EntityHydrogenJellyfish.isFlapping(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityHydrogenJellyfish.jumpInLiquid(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityHydrogenJellyfish.isNoGravity(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityHydrogenJellyfish.isFood(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityHydrogenJellyfish.playerTouch(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityHydrogenJellyfish.customServerAiStep(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        pub fn lerp_angle_degrees(delta: f32, first: f32, second: f32) -> f32 {
            return first + (delta * (j_wrap_degrees(((second - first) as f64)) as f32));
        }
        // EntityHydrogenJellyfish.getMaxSpawnClusterSize(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityHydrogenJellyfish.die(1): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityHydrogenJellyfish.getAmbientSound(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityHydrogenJellyfish.getSoundVolume(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityHydrogenJellyfish.getBreedOffspring(2): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityHydrogenJellyfish.causeFallDamage(3): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityHydrogenJellyfish.checkFallDamage(4): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityHydrogenJellyfish.hurtServer(3): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
        // EntityHydrogenJellyfish.canSpawn(5): not translated, it uses Minecraft's `EntityType`.
        // EntityHydrogenJellyfish.omnidirectionalAirMover(0): not translated, its class isn't translated (it extends `DespawnableAnimal` (org.betterx.bclib.entity)).
    }

    /// `EntityJungleSkeleton` (org/betterx/betternether/entity/EntityJungleSkeleton.java)
    pub mod entity_jungle_skeleton {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Skeleton`. Its static parts are below.
        // EntityJungleSkeleton.EntityJungleSkeleton(2): not translated, its class isn't translated (it extends Minecraft's `Skeleton`).
        // EntityJungleSkeleton.checkDespawn(0): not translated, its class isn't translated (it extends Minecraft's `Skeleton`).
        // EntityJungleSkeleton.finalizeSpawn(4): not translated, its class isn't translated (it extends Minecraft's `Skeleton`).
        // EntityJungleSkeleton.getHandItem(0): not translated, its class isn't translated (it extends Minecraft's `Skeleton`).
        // EntityJungleSkeleton.getRandomOffhandItem(0): not translated, its class isn't translated (it extends Minecraft's `Skeleton`).
    }

    /// `EntityNaga` (org/betterx/betternether/entity/EntityNaga.java)
    pub mod entity_naga {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Monster`. Its static parts are below.
        // EntityNaga.EntityNaga(2): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntityNaga.registerGoals(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntityNaga.createMobAttributes(0): not translated, it calls Minecraft's `Mob`.createMobAttributes.
        // EntityNaga.performRangedAttack(2): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntityNaga.canBeAffected(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntityNaga.getAmbientSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntityNaga.getHurtSound(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntityNaga.getDeathSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntityNaga.checkDespawn(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntityNaga.getMaxHeadYRot(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntityNaga.canSpawn(5): not translated, it uses Minecraft's `EntityType`.
    }

    /// `EntityNagaProjectile` (org/betterx/betternether/entity/EntityNagaProjectile.java)
    pub mod entity_naga_projectile {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Mob`. Its static parts are below.
        pub const MAX_LIFE_TIME: i32 = 60;
        // EntityNagaProjectile.EntityNagaProjectile(2): not translated, its class isn't translated (it extends Minecraft's `Mob`).
        // EntityNagaProjectile.setParams(2): not translated, its class isn't translated (it extends Minecraft's `Mob`).
        // EntityNagaProjectile.isNoGravity(0): not translated, its class isn't translated (it extends Minecraft's `Mob`).
        // EntityNagaProjectile.checkFallDamage(4): not translated, its class isn't translated (it extends Minecraft's `Mob`).
        // EntityNagaProjectile.travel(1): not translated, its class isn't translated (it extends Minecraft's `Mob`).
        // EntityNagaProjectile.onClimbable(0): not translated, its class isn't translated (it extends Minecraft's `Mob`).
        // EntityNagaProjectile.shouldRenderAtSqrDistance(1): not translated, its class isn't translated (it extends Minecraft's `Mob`).
        // EntityNagaProjectile.tick(0): not translated, its class isn't translated (it extends Minecraft's `Mob`).
        // EntityNagaProjectile.isSame(2): not translated, its class isn't translated (it extends Minecraft's `Mob`).
        // EntityNagaProjectile.onCollision(1): not translated, its class isn't translated (it extends Minecraft's `Mob`).
        // EntityNagaProjectile.effectKill(0): not translated, its class isn't translated (it extends Minecraft's `Mob`).
        // EntityNagaProjectile.canBeAffected(1): not translated, its class isn't translated (it extends Minecraft's `Mob`).
        // EntityNagaProjectile.isSilent(0): not translated, its class isn't translated (it extends Minecraft's `Mob`).
        // EntityNagaProjectile.addAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Mob`).
        // EntityNagaProjectile.readAdditionalSaveData(1): not translated, its class isn't translated (it extends Minecraft's `Mob`).
    }

    /// `EntitySkull` (org/betterx/betternether/entity/EntitySkull.java)
    pub mod entity_skull {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Monster`. Its static parts are below.
        pub static PARTICLE_X: JCell<f64> = JCell::new(0.0f64);
        pub static PARTICLE_Y: JCell<f64> = JCell::new(0.0f64);
        pub static PARTICLE_Z: JCell<f64> = JCell::new(0.0f64);
        // EntitySkull.EntitySkull(2): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.createMobAttributes(0): not translated, it calls Minecraft's `Mob`.createMobAttributes.
        // EntitySkull.getFlyingSpeed(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.omnidirectionalAirMover(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.getAttackDamage(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.playerTouch(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.checkDespawn(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.tick(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.getAmbientSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.getHurtSound(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.getDeathSound(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.moveRandomDir(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.lookAt(1): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.updateParticlePos(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.isFlapping(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.causeFallDamage(3): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.checkFallDamage(4): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.isNoGravity(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.isPushable(0): not translated, its class isn't translated (it extends Minecraft's `Monster`).
        // EntitySkull.canSpawn(5): not translated, it uses Minecraft's `EntityType`.
    }

    /// `EntitySkull.SkullLookControl` (org/betterx/betternether/entity/EntitySkull.java)
    pub mod entity_skull_skull_look_control {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `LookControl`. Its static parts are below.
        // EntitySkull.SkullLookControl.SkullLookControl(1): not translated, its class isn't translated (it extends Minecraft's `LookControl`).
        // EntitySkull.SkullLookControl.resetXRotOnTick(0): not translated, its class isn't translated (it extends Minecraft's `LookControl`).
    }

    /// `ModelEntityFirefly` (org/betterx/betternether/entity/model/ModelEntityFirefly.java)
    pub mod model_entity_firefly {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityModel`. Its static parts are below.
        pub const GLOW: &str = "glow";
        pub const GLOW_PLANE: &str = "glow_plane";
        // ModelEntityFirefly.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // ModelEntityFirefly.ModelEntityFirefly(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // ModelEntityFirefly.setupAnim(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // ModelEntityFirefly.syncTransform(0): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // ModelEntityFirefly.getGlowPart(0): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
    }

    /// `ModelEntityFlyingPig` (org/betterx/betternether/entity/model/ModelEntityFlyingPig.java)
    pub mod model_entity_flying_pig {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityModel`. Its static parts are below.
        // ModelEntityFlyingPig.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // ModelEntityFlyingPig.ModelEntityFlyingPig(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // ModelEntityFlyingPig.setupAnim(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
    }

    /// `ModelEntityHydrogenJellyfish` (org/betterx/betternether/entity/model/ModelEntityHydrogenJellyfish.java)
    pub mod model_entity_hydrogen_jellyfish {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityModel`. Its static parts are below.
        pub const BODY_TOP: &str = "body_top";
        pub static HALF_PI: LazyLock<f64> = LazyLock::new(|| std::f64::consts::PI / 2.0f64);
        pub const LEGS: i32 = 8;
        // ModelEntityHydrogenJellyfish.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // ModelEntityHydrogenJellyfish.ModelEntityHydrogenJellyfish(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // ModelEntityHydrogenJellyfish.setupAnim(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
    }

    /// `ModelJungleSkeleton` (org/betterx/betternether/entity/model/ModelJungleSkeleton.java)
    pub mod model_jungle_skeleton {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SkeletonModel`. Its static parts are below.
        pub static ANGLE45: LazyLock<f32> = LazyLock::new(|| ((std::f64::consts::PI / 4.0f64) as f32));
        pub static ANGLE90: LazyLock<f32> = LazyLock::new(|| ((std::f64::consts::PI / 2.0f64) as f32));
        // RANDOM: not translated, it creates Minecraft's `LegacyRandomSource`.
        pub static BOUND_MIN: LazyLock<f32> = LazyLock::new(|| ((std::f64::consts::PI / 3.0f64) as f32));
        pub static BOUND_MAX: LazyLock<f32> = LazyLock::new(|| (((std::f64::consts::PI * 2.0f64) / 5.0f64) as f32));
        // ModelJungleSkeleton.createBodyLayer(0): not translated, it uses Minecraft's `LayerDefinition`.
        // ModelJungleSkeleton.ModelJungleSkeleton(1): not translated, its class isn't translated (it extends Minecraft's `SkeletonModel`).
    }

    /// `ModelNaga` (org/betterx/betternether/entity/model/ModelNaga.java)
    pub mod model_naga {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityModel`. Its static parts are below.
        pub const SPIKE_COUNT: i32 = 8;
        pub const TAIL_COUNT: i32 = 4;
        // ModelNaga.getTexturedModelData(0): not translated, its body couldn't be read: an assert.
        // ModelNaga.ModelNaga(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // ModelNaga.setupAnim(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // ModelNaga.lerpAngle(3): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
    }

    /// `ModelSkull` (org/betterx/betternether/entity/model/ModelSkull.java)
    pub mod model_skull {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityModel`. Its static parts are below.
        // ModelSkull.getTexturedModelData(0): not translated, it uses Minecraft's `LayerDefinition`.
        // ModelSkull.ModelSkull(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // ModelSkull.setupAnim(1): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
        // ModelSkull.lerpAngle(3): not translated, its class isn't translated (it extends Minecraft's `EntityModel`).
    }

    /// `FireflyGlowFeatureRenderer` (org/betterx/betternether/entity/render/FireflyGlowFeatureRenderer.java)
    pub mod firefly_glow_feature_renderer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `RenderLayer`. Its static parts are below.
        pub const LIT: i32 = 15728880;
        // TEXTURE: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // FireflyGlowFeatureRenderer.FireflyGlowFeatureRenderer(1): not translated, its class isn't translated (it extends Minecraft's `RenderLayer`).
        // FireflyGlowFeatureRenderer.submit(6): not translated, its class isn't translated (it extends Minecraft's `RenderLayer`).
        // FireflyGlowFeatureRenderer.submitViewAlignedGlow(4): not translated, its class isn't translated (it extends Minecraft's `RenderLayer`).
        // FireflyGlowFeatureRenderer.addVertex(8): not translated, it uses Minecraft's `Pose`.
    }

    /// `FireflyRenderState` (org/betterx/betternether/entity/render/FireflyRenderState.java)
    pub mod firefly_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `LivingEntityRenderState`. Its static parts are below.
    }

    /// `FlyingPigRenderState` (org/betterx/betternether/entity/render/FlyingPigRenderState.java)
    pub mod flying_pig_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `LivingEntityRenderState`. Its static parts are below.
    }

    /// `HydrogenJellyfishRenderState` (org/betterx/betternether/entity/render/HydrogenJellyfishRenderState.java)
    pub mod hydrogen_jellyfish_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `LivingEntityRenderState`. Its static parts are below.
    }

    /// `JungleSkeletonRenderState` (org/betterx/betternether/entity/render/JungleSkeletonRenderState.java)
    pub mod jungle_skeleton_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SkeletonRenderState`. Its static parts are below.
    }

    /// `NagaRenderState` (org/betterx/betternether/entity/render/NagaRenderState.java)
    pub mod naga_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `LivingEntityRenderState`. Its static parts are below.
    }

    /// `RenderFirefly` (org/betterx/betternether/entity/render/RenderFirefly.java)
    pub mod render_firefly {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobRenderer`. Its static parts are below.
        // TEXTURE: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // RenderFirefly.RenderFirefly(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderFirefly.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderFirefly.extractRenderState(3): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderFirefly.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderFirefly.getBlockLightLevel(2): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
    }

    /// `RenderFlyingPig` (org/betterx/betternether/entity/render/RenderFlyingPig.java)
    pub mod render_flying_pig {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobRenderer`. Its static parts are below.
        // TEXTURE: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // TEXTURE_WARTED: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // RenderFlyingPig.RenderFlyingPig(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderFlyingPig.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderFlyingPig.extractRenderState(3): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderFlyingPig.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
    }

    /// `RenderHydrogenJellyfish` (org/betterx/betternether/entity/render/RenderHydrogenJellyfish.java)
    pub mod render_hydrogen_jellyfish {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobRenderer`. Its static parts are below.
        // TEXTURE: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // RenderHydrogenJellyfish.RenderHydrogenJellyfish(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderHydrogenJellyfish.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderHydrogenJellyfish.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderHydrogenJellyfish.getBlockLightLevel(2): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
    }

    /// `RenderJungleSkeleton` (org/betterx/betternether/entity/render/RenderJungleSkeleton.java)
    pub mod render_jungle_skeleton {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractSkeletonRenderer`. Its static parts are below.
        // TEXTURE: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // RenderJungleSkeleton.RenderJungleSkeleton(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSkeletonRenderer`).
        // RenderJungleSkeleton.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `AbstractSkeletonRenderer`).
        // RenderJungleSkeleton.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `AbstractSkeletonRenderer`).
    }

    /// `RenderNaga` (org/betterx/betternether/entity/render/RenderNaga.java)
    pub mod render_naga {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobRenderer`. Its static parts are below.
        // TEXTURE: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // RenderNaga.RenderNaga(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderNaga.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderNaga.extractRenderState(3): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderNaga.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
    }

    /// `RenderNagaProjectile` (org/betterx/betternether/entity/render/RenderNagaProjectile.java)
    pub mod render_naga_projectile {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EntityRenderer`. Its static parts are below.
        // TEXTURE: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // LAYER: not translated, it uses Minecraft's `RenderType`.
        // RenderNagaProjectile.RenderNagaProjectile(1): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
        // RenderNagaProjectile.getBlockLightLevel(2): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
        // RenderNagaProjectile.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
        // RenderNagaProjectile.submit(4): not translated, its class isn't translated (it extends Minecraft's `EntityRenderer`).
        // RenderNagaProjectile.vertex(7): not translated, it uses Minecraft's `VertexConsumer`.
    }

    /// `RenderPhaseAccessor` (org/betterx/betternether/entity/render/RenderPhaseAccessor.java)
    pub mod render_phase_accessor {
        use super::*;
        // FIREFLY_PIPELINE: not translated, it uses Minecraft's `RenderPipeline`.
        // FIREFLY_RENDER_TYPE: not translated, it uses Minecraft's `RenderType`.
        #[derive(Clone, Debug)]
        pub struct RenderPhaseAccessor {
        }
        impl RenderPhaseAccessor {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RenderPhaseAccessor.createFirefly(1): not translated, it uses Minecraft's `RenderType`.
        // RenderPhaseAccessor.getFirefly(1): not translated, it uses Minecraft's `RenderType`.
        // RenderPhaseAccessor.assignIrisProgram(1): not translated, it uses Minecraft's `RenderPipeline`.
    }

    /// `RenderSkull` (org/betterx/betternether/entity/render/RenderSkull.java)
    pub mod render_skull {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MobRenderer`. Its static parts are below.
        // TEXTURE: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // RenderSkull.RenderSkull(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderSkull.createRenderState(0): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderSkull.extractRenderState(3): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderSkull.getTextureLocation(1): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
        // RenderSkull.getBlockLightLevel(2): not translated, its class isn't translated (it extends Minecraft's `MobRenderer`).
    }

    /// `RenderSkull.GlowFeatureRenderer` (org/betterx/betternether/entity/render/RenderSkull.java)
    pub mod render_skull_glow_feature_renderer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `EyesLayer`. Its static parts are below.
        // SKIN: not translated, it uses Minecraft's `RenderType`.
        // RenderSkull.GlowFeatureRenderer.GlowFeatureRenderer(1): not translated, its class isn't translated (it extends Minecraft's `EyesLayer`).
        // RenderSkull.GlowFeatureRenderer.renderType(0): not translated, its class isn't translated (it extends Minecraft's `EyesLayer`).
    }

    /// `SkullRenderState` (org/betterx/betternether/entity/render/SkullRenderState.java)
    pub mod skull_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `LivingEntityRenderState`. Its static parts are below.
    }

    /// `EMIForgeRecipe` (org/betterx/betternether/integrations/emi/EMIForgeRecipe.java)
    pub mod emi_forge_recipe {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EMIForgeRecipe {
        }
        impl EMIForgeRecipe {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EMIPlugin` (org/betterx/betternether/integrations/emi/EMIPlugin.java)
    pub mod emi_plugin {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EMIPlugin {
        }
        impl EMIPlugin {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `JeiPlugin` (org/betterx/betternether/integrations/jei/JeiPlugin.java)
    pub mod jei_plugin {
        use super::*;
        // The class itself isn't translated: it implements `IModPlugin` (mezz.jei.api). Its static parts are below.
        // UID: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // JeiPlugin.getPluginUid(0): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
        // JeiPlugin.registerRecipeCatalysts(1): not translated, its class isn't translated (it implements `IModPlugin` (mezz.jei.api)).
    }

    /// `VanillaExcavatorsIntegration` (org/betterx/betternether/integrations/VanillaExcavatorsIntegration.java)
    pub mod vanilla_excavators_integration {
        use super::*;
        // hasExcavators: not translated, it uses BetterNether.VANILLA_EXCAVATORS, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // excavatorConstructor: not translated, it uses `Constructor` (java.lang.reflect).
        #[derive(Clone, Debug)]
        pub struct VanillaExcavatorsIntegration {
        }
        impl VanillaExcavatorsIntegration {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // VanillaExcavatorsIntegration.makeExcavator(4): not translated, it uses Minecraft's `Item`.
        // VanillaExcavatorsIntegration.hasExcavators(0): not translated, it uses VanillaExcavatorsIntegration.hasExcavators, which isn't translated (it uses BetterNether.VANILLA_EXCAVATORS, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api))).
    }

    /// `VanillaHammersIntegration` (org/betterx/betternether/integrations/VanillaHammersIntegration.java)
    pub mod vanilla_hammers_integration {
        use super::*;
        // hasHammers: not translated, it uses BetterNether.VANILLA_HAMMERS, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // hammerConstructor: not translated, it uses `Constructor` (java.lang.reflect).
        #[derive(Clone, Debug)]
        pub struct VanillaHammersIntegration {
        }
        impl VanillaHammersIntegration {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // VanillaHammersIntegration.makeHammer(4): not translated, it uses Minecraft's `Item`.
        // VanillaHammersIntegration.hasHammers(0): not translated, it uses VanillaHammersIntegration.hasHammers, which isn't translated (it uses BetterNether.VANILLA_HAMMERS, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api))).
    }

    /// `DiamondSet` (org/betterx/betternether/items/complex/DiamondSet.java)
    pub mod diamond_set {
        use super::*;
        // The class itself isn't translated: it extends `NetherSet`. Its static parts are below.
        // DiamondSet.DiamondSet(1): not translated, its class isn't translated (it extends `NetherSet`).
        // DiamondSet.nameForSlot(1): not translated, its class isn't translated (it extends `NetherSet`).
    }

    /// `NetherSet` (org/betterx/betternether/items/complex/NetherSet.java)
    pub mod nether_set {
        use super::*;
        // The class itself isn't translated: it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment). Its static parts are below.
        // NetherSet.NetherSet(4): not translated, its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)).
        // NetherSet.NetherSet(5): not translated, its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)).
        // NetherSet.shearsTool(1): not translated, it uses Minecraft's `Tool`.
        // NetherSet.commonToolProperties(1): not translated, its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)).
        // NetherSet.addEquipmentSetSlotCriterion(2): not translated, its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)).
        // NetherSet.addEquipmentSetSlotCriterion(2): not translated, its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)).
        // NetherSet.addArmorSetCriterion(1): not translated, its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)).
        // NetherSet.addToolSetCriterion(1): not translated, its class isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)).
    }

    /// `ItemBlackApple` (org/betterx/betternether/items/ItemBlackApple.java)
    pub mod item_black_apple {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // BLACK_APPLE: not translated, it uses Minecraft's `FoodProperties`.
        // ItemBlackApple.ItemBlackApple(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemBlackApple.finishUsingItem(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `ItemBowlFood` (org/betterx/betternether/items/ItemBowlFood.java)
    pub mod item_bowl_food {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // ItemBowlFood.ItemBowlFood(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemBowlFood.useOn(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemBowlFood.finishUsingItem(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `ItemBowlMedicine` (org/betterx/betternether/items/ItemBowlMedicine.java)
    pub mod item_bowl_medicine {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // ItemBowlMedicine.ItemBowlMedicine(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemBowlMedicine.finishUsingItem(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `BNArmorMaterials` (org/betterx/betternether/items/materials/BNArmorMaterials.java)
    pub mod bn_armor_materials {
        use super::*;
        // CINCINNASITE: not translated, it uses Minecraft's `Holder`.
        // NETHER_RUBY: not translated, it uses Minecraft's `Holder`.
        // FLAMING_RUBY: not translated, it uses Minecraft's `Holder`.
        #[derive(Clone, Debug)]
        pub struct BNArmorMaterials {
        }
        impl BNArmorMaterials {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `BNArmorTiers` (org/betterx/betternether/items/materials/BNArmorTiers.java)
    pub mod bn_armor_tiers {
        use super::*;
        // CINCINNASITE: not translated, it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment).
        // NETHER_RUBY: not translated, it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment).
        // FLAMING_RUBY: not translated, it uses `ArmorTier` (de.ambertation.wover.complex.api.equipment).
        #[derive(Clone, Debug)]
        pub struct BNArmorTiers {
        }
        impl BNArmorTiers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `BNToolMaterial` (org/betterx/betternether/items/materials/BNToolMaterial.java)
    pub mod bn_tool_material {
        use super::*;
        // CINCINNASITE: not translated, it uses Minecraft's `ToolMaterial`.
        // CINCINNASITE_DIAMOND: not translated, it uses Minecraft's `ToolMaterial`.
        // NETHER_RUBY: not translated, it uses Minecraft's `ToolMaterial`.
        // FLAMING_RUBY: not translated, it uses Minecraft's `ToolMaterial`.
        #[derive(Clone, Debug)]
        pub struct BNToolMaterial {
        }
        impl BNToolMaterial {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `BNToolTiers` (org/betterx/betternether/items/materials/BNToolTiers.java)
    pub mod bn_tool_tiers {
        use super::*;
        // CINCINNASITE: not translated, it uses `ToolTier` (de.ambertation.wover.complex.api.equipment).
        // CINCINNASITE_DIAMOND: not translated, it uses `ToolTier` (de.ambertation.wover.complex.api.equipment).
        // NETHER_RUBY: not translated, it uses `ToolTier` (de.ambertation.wover.complex.api.equipment).
        // FLAMING_RUBY: not translated, it uses `ToolTier` (de.ambertation.wover.complex.api.equipment).
        #[derive(Clone, Debug)]
        pub struct BNToolTiers {
        }
        impl BNToolTiers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `NetherArmor` (org/betterx/betternether/items/NetherArmor.java)
    pub mod nether_armor {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // NetherArmor.NetherArmor(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // NetherArmor.setupItemStack(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `NetherAxe` (org/betterx/betternether/items/NetherAxe.java)
    pub mod nether_axe {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // NetherAxe.NetherAxe(4): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // NetherAxe.setupItemStack(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `NetherHoe` (org/betterx/betternether/items/NetherHoe.java)
    pub mod nether_hoe {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // NetherHoe.NetherHoe(4): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // NetherHoe.setupItemStack(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `NetherPickaxe` (org/betterx/betternether/items/NetherPickaxe.java)
    pub mod nether_pickaxe {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // NetherPickaxe.NetherPickaxe(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // NetherPickaxe.setupItemStack(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `NetherShovel` (org/betterx/betternether/items/NetherShovel.java)
    pub mod nether_shovel {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // NetherShovel.NetherShovel(4): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // NetherShovel.setupItemStack(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `NetherSpear` (org/betterx/betternether/items/NetherSpear.java)
    pub mod nether_spear {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // NetherSpear.NetherSpear(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // NetherSpear.setupItemStack(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `NetherSword` (org/betterx/betternether/items/NetherSword.java)
    pub mod nether_sword {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // NetherSword.NetherSword(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // NetherSword.setupItemStack(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `LegacySpawnerFix` (org/betterx/betternether/LegacySpawnerFix.java)
    pub mod legacy_spawner_fix {
        use super::*;
        pub static ARMOR_SLOTS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["feet".to_string(), "legs".to_string(), "chest".to_string(), "head".to_string()]);
        pub static HAND_SLOTS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["mainhand".to_string(), "offhand".to_string()]);
        // VANILLA_EFFECT_IDS: not translated, it calls `Map` (java.util).entry.
        #[derive(Clone, Debug)]
        pub struct LegacySpawnerFix {
        }
        impl LegacySpawnerFix {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LegacySpawnerFix.patchChunk(1): not translated, it uses Minecraft's `CompoundTag`.
        // LegacySpawnerFix.patchBlockEntities(2): not translated, it uses Minecraft's `ListTag`.
        // LegacySpawnerFix.patchSpawner(1): not translated, it uses Minecraft's `CompoundTag`.
        // LegacySpawnerFix.spawnEntityHolders(1): not translated, it uses Minecraft's `CompoundTag`.
        // LegacySpawnerFix.patchEntity(1): not translated, it uses Minecraft's `CompoundTag`.
        // LegacySpawnerFix.moveEquipment(4): not translated, it uses Minecraft's `CompoundTag`.
        // LegacySpawnerFix.moveDropChances(5): not translated, it uses Minecraft's `CompoundTag`.
        // LegacySpawnerFix.convertItem(1): not translated, it uses Minecraft's `CompoundTag`.
        // LegacySpawnerFix.convertEffect(1): not translated, it uses Minecraft's `CompoundTag`.
    }

    /// `BNLoot` (org/betterx/betternether/loot/BNLoot.java)
    pub mod bn_loot {
        use super::*;
        // CITY_LOOT: not translated, it uses Minecraft's `ResourceKey`.
        // CITY_LOOT_COMMON: not translated, it uses Minecraft's `ResourceKey`.
        // CITY_LOOT_SURPRISE: not translated, it uses Minecraft's `ResourceKey`.
        // LIBRARY_LOOT: not translated, it uses Minecraft's `ResourceKey`.
        // WITHER_TOWER_LOOT: not translated, it uses Minecraft's `ResourceKey`.
        // WITHER_TOWER_BONUS_LOOT: not translated, it uses Minecraft's `ResourceKey`.
        // GHAST_HIVE: not translated, it uses Minecraft's `ResourceKey`.
        // FIREFLY: not translated, it uses Minecraft's `ResourceKey`.
        // FLYING_PIG: not translated, it uses Minecraft's `ResourceKey`.
        // JUNGLE_SKELETON: not translated, it uses Minecraft's `ResourceKey`.
        // NAGA: not translated, it uses Minecraft's `ResourceKey`.
        // SKULL: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct BNLoot {
        }
        impl BNLoot {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `MHelper` (org/betterx/betternether/MHelper.java)
    pub mod m_helper {
        use super::*;
        pub static PI2: LazyLock<f32> = LazyLock::new(|| ((std::f64::consts::PI * 2.0f64) as f32));
        pub const ALPHA: i32 = -16777216;
        // RANDOM: not translated, it creates a ThreadLocalRandomSource, which isn't translated (it implements Minecraft's `RandomSource`).
        #[derive(Clone, Debug)]
        pub struct MHelper {
        }
        impl MHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn color(r: i32, g: i32, b: i32) -> i32 {
            return (((0xFF000000u32 as i32) | (r << 16)) | (g << 8)) | b;
        }
        // MHelper.randRange(3): not translated, it's overloaded with the same number of parameters.
        // MHelper.randRange(3): not translated, it's overloaded with the same number of parameters.
        pub fn set_bit(source: i8, pos: i32, value: bool) -> i8 {
            return ((if value { (m_helper::set_bit_true(source, pos) as i32) } else { (m_helper::set_bit_false(source, pos) as i32) }) as i8);
        }
        pub fn set_bit_true(source: i8, pos: i32) -> i8 {
            return (((source as i32) | (1 << pos)) as i8);
        }
        pub fn set_bit_false(source: i8, pos: i32) -> i8 {
            return (((source as i32) & (!(1 << pos))) as i8);
        }
        pub fn get_bit(source: i8, pos: i32) -> bool {
            return (((source as i32) >> pos) & 1) == 1;
        }
        // MHelper.floor(1): not translated, it's overloaded with the same number of parameters.
        // MHelper.wrap(2): not translated, it calls MHelper.floor, which isn't translated.
        // MHelper.floor(1): not translated, it's overloaded with the same number of parameters.
        pub fn next_float(random: JRandom, d: f32) -> f32 {
            return random.next_float() * d;
        }
        pub fn next_double(random: JRandom, d: f64) -> f64 {
            return random.next_double() * d;
        }
    }

    /// `MHelper.ThreadLocalRandomSource` (org/betterx/betternether/MHelper.java)
    pub mod m_helper_thread_local_random_source {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `RandomSource`. Its static parts are below.
        // MHelper.ThreadLocalRandomSource.fork(0): not translated, its class isn't translated (it implements Minecraft's `RandomSource`).
        // MHelper.ThreadLocalRandomSource.forkPositional(0): not translated, its class isn't translated (it implements Minecraft's `RandomSource`).
        // MHelper.ThreadLocalRandomSource.setSeed(1): not translated, its class isn't translated (it implements Minecraft's `RandomSource`).
        // MHelper.ThreadLocalRandomSource.nextInt(0): not translated, its class isn't translated (it implements Minecraft's `RandomSource`).
        // MHelper.ThreadLocalRandomSource.nextInt(1): not translated, its class isn't translated (it implements Minecraft's `RandomSource`).
        // MHelper.ThreadLocalRandomSource.nextLong(0): not translated, its class isn't translated (it implements Minecraft's `RandomSource`).
        // MHelper.ThreadLocalRandomSource.nextBoolean(0): not translated, its class isn't translated (it implements Minecraft's `RandomSource`).
        // MHelper.ThreadLocalRandomSource.nextFloat(0): not translated, its class isn't translated (it implements Minecraft's `RandomSource`).
        // MHelper.ThreadLocalRandomSource.nextDouble(0): not translated, its class isn't translated (it implements Minecraft's `RandomSource`).
        // MHelper.ThreadLocalRandomSource.nextGaussian(0): not translated, its class isn't translated (it implements Minecraft's `RandomSource`).
    }

    /// `OpenSimplexNoise` (org/betterx/betternether/noise/OpenSimplexNoise.java)
    pub mod open_simplex_noise {
        use super::*;
        pub const STRETCH_CONSTANT_2D: f64 = -0.211324865405187f64;
        pub const SQUISH_CONSTANT_2D: f64 = 0.366025403784439f64;
        pub const STRETCH_CONSTANT_3D: f64 = -0.16666666666666666f64;
        pub const SQUISH_CONSTANT_3D: f64 = 0.3333333333333333f64;
        pub const STRETCH_CONSTANT_4D: f64 = -0.138196601125011f64;
        pub const SQUISH_CONSTANT_4D: f64 = 0.309016994374947f64;
        pub const NORM_CONSTANT_2D: f64 = 47.0f64;
        pub const NORM_CONSTANT_3D: f64 = 103.0f64;
        pub const NORM_CONSTANT_4D: f64 = 30.0f64;
        pub const DEFAULT_SEED: i64 = 0i64;
        pub static GRADIENTS2_D: LazyLock<Vec<i8>> = LazyLock::new(|| vec![5i8, 2i8, 2i8, 5i8, -5i8, 2i8, -2i8, 5i8, 5i8, -2i8, 2i8, -5i8, -5i8, -2i8, -2i8, -5i8]);
        pub static GRADIENTS3_D: LazyLock<Vec<i8>> = LazyLock::new(|| vec![-11i8, 4i8, 4i8, -4i8, 11i8, 4i8, -4i8, 4i8, 11i8, 11i8, 4i8, 4i8, 4i8, 11i8, 4i8, 4i8, 4i8, 11i8, -11i8, -4i8, 4i8, -4i8, -11i8, 4i8, -4i8, -4i8, 11i8, 11i8, -4i8, 4i8, 4i8, -11i8, 4i8, 4i8, -4i8, 11i8, -11i8, 4i8, -4i8, -4i8, 11i8, -4i8, -4i8, 4i8, -11i8, 11i8, 4i8, -4i8, 4i8, 11i8, -4i8, 4i8, 4i8, -11i8, -11i8, -4i8, -4i8, -4i8, -11i8, -4i8, -4i8, -4i8, -11i8, 11i8, -4i8, -4i8, 4i8, -11i8, -4i8, 4i8, -4i8, -11i8]);
        pub static GRADIENTS4_D: LazyLock<Vec<i8>> = LazyLock::new(|| vec![3i8, 1i8, 1i8, 1i8, 1i8, 3i8, 1i8, 1i8, 1i8, 1i8, 3i8, 1i8, 1i8, 1i8, 1i8, 3i8, -3i8, 1i8, 1i8, 1i8, -1i8, 3i8, 1i8, 1i8, -1i8, 1i8, 3i8, 1i8, -1i8, 1i8, 1i8, 3i8, 3i8, -1i8, 1i8, 1i8, 1i8, -3i8, 1i8, 1i8, 1i8, -1i8, 3i8, 1i8, 1i8, -1i8, 1i8, 3i8, -3i8, -1i8, 1i8, 1i8, -1i8, -3i8, 1i8, 1i8, -1i8, -1i8, 3i8, 1i8, -1i8, -1i8, 1i8, 3i8, 3i8, 1i8, -1i8, 1i8, 1i8, 3i8, -1i8, 1i8, 1i8, 1i8, -3i8, 1i8, 1i8, 1i8, -1i8, 3i8, -3i8, 1i8, -1i8, 1i8, -1i8, 3i8, -1i8, 1i8, -1i8, 1i8, -3i8, 1i8, -1i8, 1i8, -1i8, 3i8, 3i8, -1i8, -1i8, 1i8, 1i8, -3i8, -1i8, 1i8, 1i8, -1i8, -3i8, 1i8, 1i8, -1i8, -1i8, 3i8, -3i8, -1i8, -1i8, 1i8, -1i8, -3i8, -1i8, 1i8, -1i8, -1i8, -3i8, 1i8, -1i8, -1i8, -1i8, 3i8, 3i8, 1i8, 1i8, -1i8, 1i8, 3i8, 1i8, -1i8, 1i8, 1i8, 3i8, -1i8, 1i8, 1i8, 1i8, -3i8, -3i8, 1i8, 1i8, -1i8, -1i8, 3i8, 1i8, -1i8, -1i8, 1i8, 3i8, -1i8, -1i8, 1i8, 1i8, -3i8, 3i8, -1i8, 1i8, -1i8, 1i8, -3i8, 1i8, -1i8, 1i8, -1i8, 3i8, -1i8, 1i8, -1i8, 1i8, -3i8, -3i8, -1i8, 1i8, -1i8, -1i8, -3i8, 1i8, -1i8, -1i8, -1i8, 3i8, -1i8, -1i8, -1i8, 1i8, -3i8, 3i8, 1i8, -1i8, -1i8, 1i8, 3i8, -1i8, -1i8, 1i8, 1i8, -3i8, -1i8, 1i8, 1i8, -1i8, -3i8, -3i8, 1i8, -1i8, -1i8, -1i8, 3i8, -1i8, -1i8, -1i8, 1i8, -3i8, -1i8, -1i8, 1i8, -1i8, -3i8, 3i8, -1i8, -1i8, -1i8, 1i8, -3i8, -1i8, -1i8, 1i8, -1i8, -3i8, -1i8, 1i8, -1i8, -1i8, -3i8, -3i8, -1i8, -1i8, -1i8, -1i8, -3i8, -1i8, -1i8, -1i8, -1i8, -3i8, -1i8, -1i8, -1i8, -1i8, -3i8]);
        #[derive(Clone, Debug)]
        pub struct OpenSimplexNoise {
            pub perm: Option<Vec<i16>>,
            pub perm_grad_index3_d: Option<Vec<i16>>,
        }
        impl OpenSimplexNoise {
            // OpenSimplexNoise.OpenSimplexNoise(0): not translated, its constructors are overloaded with the same number of parameters.
            // OpenSimplexNoise.OpenSimplexNoise(1): not translated, its constructors are overloaded with the same number of parameters.
            // OpenSimplexNoise.OpenSimplexNoise(1): not translated, its constructors are overloaded with the same number of parameters.
            pub fn eval_2(&self, x: f64, y: f64) -> f64 {
                let stretch_offset: f64 = (x + y) * (-0.211324865405187f64);
                let xs: f64 = x + stretch_offset;
                let ys: f64 = y + stretch_offset;
                let mut xsb: i32 = open_simplex_noise::fast_floor(xs);
                let mut ysb: i32 = open_simplex_noise::fast_floor(ys);
                let squish_offset: f64 = ((xsb + ysb) as f64) * 0.366025403784439f64;
                let xb: f64 = (xsb as f64) + squish_offset;
                let yb: f64 = (ysb as f64) + squish_offset;
                let xins: f64 = xs - (xsb as f64);
                let yins: f64 = ys - (ysb as f64);
                let in_sum: f64 = xins + yins;
                let mut dx0: f64 = x - xb;
                let mut dy0: f64 = y - yb;
                let mut value: f64 = 0.0f64;
                let dx1: f64 = (dx0 - 1.0f64) - 0.366025403784439f64;
                let dy1: f64 = (dy0 - 0.0f64) - 0.366025403784439f64;
                let mut attn1: f64 = (2.0f64 - (dx1 * dx1)) - (dy1 * dy1);
                if attn1 > 0.0f64 {
                    attn1 *= attn1;
                    value += (attn1 * attn1) * (self.extrapolate_4(xsb + 1, ysb + 0, dx1, dy1));
                }
                let dx2: f64 = (dx0 - 0.0f64) - 0.366025403784439f64;
                let dy2: f64 = (dy0 - 1.0f64) - 0.366025403784439f64;
                let mut attn2: f64 = (2.0f64 - (dx2 * dx2)) - (dy2 * dy2);
                if attn2 > 0.0f64 {
                    attn2 *= attn2;
                    value += (attn2 * attn2) * (self.extrapolate_4(xsb + 0, ysb + 1, dx2, dy2));
                }
                let mut dx_ext: f64;
                let mut dy_ext: f64;
                let mut xsv_ext: i32;
                let mut ysv_ext: i32;
                if in_sum <= 1.0f64 {
                    let zins: f64 = 1.0f64 - in_sum;
                    if (!(zins > xins)) && (!(zins > yins)) {
                        xsv_ext = xsb + 1;
                        ysv_ext = ysb + 1;
                        dx_ext = (dx0 - 1.0f64) - 0.732050807568878f64;
                        dy_ext = (dy0 - 1.0f64) - 0.732050807568878f64;
                    } else if xins > yins {
                        xsv_ext = xsb + 1;
                        ysv_ext = ysb - 1;
                        dx_ext = dx0 - 1.0f64;
                        dy_ext = dy0 + 1.0f64;
                    } else {
                        xsv_ext = xsb - 1;
                        ysv_ext = ysb + 1;
                        dx_ext = dx0 + 1.0f64;
                        dy_ext = dy0 - 1.0f64;
                    }
                } else {
                    let zins: f64 = 2.0f64 - in_sum;
                    if (!(zins < xins)) && (!(zins < yins)) {
                        dx_ext = dx0;
                        dy_ext = dy0;
                        xsv_ext = xsb;
                        ysv_ext = ysb;
                    } else if xins > yins {
                        xsv_ext = xsb + 2;
                        ysv_ext = ysb + 0;
                        dx_ext = (dx0 - 2.0f64) - 0.732050807568878f64;
                        dy_ext = (dy0 + 0.0f64) - 0.732050807568878f64;
                    } else {
                        xsv_ext = xsb + 0;
                        ysv_ext = ysb + 2;
                        dx_ext = (dx0 + 0.0f64) - 0.732050807568878f64;
                        dy_ext = (dy0 - 2.0f64) - 0.732050807568878f64;
                    }
                    xsb += 1;
                    ysb += 1;
                    dx0 = (dx0 - 1.0f64) - 0.732050807568878f64;
                    dy0 = (dy0 - 1.0f64) - 0.732050807568878f64;
                }
                let mut attn0: f64 = (2.0f64 - (dx0 * dx0)) - (dy0 * dy0);
                if attn0 > 0.0f64 {
                    attn0 *= attn0;
                    value += (attn0 * attn0) * (self.extrapolate_4(xsb, ysb, dx0, dy0));
                }
                let mut attn_ext: f64 = (2.0f64 - (dx_ext * dx_ext)) - (dy_ext * dy_ext);
                if attn_ext > 0.0f64 {
                    attn_ext *= attn_ext;
                    value += (attn_ext * attn_ext) * (self.extrapolate_4(xsv_ext, ysv_ext, dx_ext, dy_ext));
                }
                return value / 47.0f64;
            }
            pub fn eval_3(&self, x: f64, y: f64, z: f64) -> f64 {
                let stretch_offset: f64 = ((x + y) + z) * (-0.16666666666666666f64);
                let xs: f64 = x + stretch_offset;
                let ys: f64 = y + stretch_offset;
                let zs: f64 = z + stretch_offset;
                let xsb: i32 = open_simplex_noise::fast_floor(xs);
                let ysb: i32 = open_simplex_noise::fast_floor(ys);
                let zsb: i32 = open_simplex_noise::fast_floor(zs);
                let squish_offset: f64 = (((xsb + ysb) + zsb) as f64) * 0.3333333333333333f64;
                let xb: f64 = (xsb as f64) + squish_offset;
                let yb: f64 = (ysb as f64) + squish_offset;
                let zb: f64 = (zsb as f64) + squish_offset;
                let xins: f64 = xs - (xsb as f64);
                let yins: f64 = ys - (ysb as f64);
                let zins: f64 = zs - (zsb as f64);
                let in_sum: f64 = (xins + yins) + zins;
                let mut dx0: f64 = x - xb;
                let mut dy0: f64 = y - yb;
                let mut dz0: f64 = z - zb;
                let mut value: f64 = 0.0f64;
                let mut dx_ext0: f64;
                let mut dy_ext0: f64;
                let mut dz_ext0: f64;
                let mut dx_ext1: f64;
                let mut dy_ext1: f64;
                let mut dz_ext1: f64;
                let mut xsv_ext0: i32;
                let mut ysv_ext0: i32;
                let mut zsv_ext0: i32;
                let mut xsv_ext1: i32;
                let mut ysv_ext1: i32;
                let mut zsv_ext1: i32;
                if in_sum <= 1.0f64 {
                    let mut a_point: i8 = 1i8;
                    let mut a_score: f64 = xins;
                    let mut b_point: i8 = 2i8;
                    let mut b_score: f64 = yins;
                    if (a_score >= b_score) && (zins > b_score) {
                        b_score = zins;
                        b_point = 4i8;
                    } else if (a_score < b_score) && (zins > a_score) {
                        a_score = zins;
                        a_point = 4i8;
                    }
                    let wins: f64 = 1.0f64 - in_sum;
                    if (!(wins > a_score)) && (!(wins > b_score)) {
                        let c: i8 = (((a_point as i32) | (b_point as i32)) as i8);
                        if ((c as i32) & 1) == 0 {
                            xsv_ext0 = xsb;
                            xsv_ext1 = xsb - 1;
                            dx_ext0 = dx0 - 0.6666666666666666f64;
                            dx_ext1 = (dx0 + 1.0f64) - 0.3333333333333333f64;
                        } else {
                            let chain_0 = xsb + 1;
                            xsv_ext1 = chain_0;
                            xsv_ext0 = chain_0;
                            dx_ext0 = (dx0 - 1.0f64) - 0.6666666666666666f64;
                            dx_ext1 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                        }
                        if ((c as i32) & 2) == 0 {
                            ysv_ext0 = ysb;
                            ysv_ext1 = ysb - 1;
                            dy_ext0 = dy0 - 0.6666666666666666f64;
                            dy_ext1 = (dy0 + 1.0f64) - 0.3333333333333333f64;
                        } else {
                            let chain_1 = ysb + 1;
                            ysv_ext1 = chain_1;
                            ysv_ext0 = chain_1;
                            dy_ext0 = (dy0 - 1.0f64) - 0.6666666666666666f64;
                            dy_ext1 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                        }
                        if ((c as i32) & 4) == 0 {
                            zsv_ext0 = zsb;
                            zsv_ext1 = zsb - 1;
                            dz_ext0 = dz0 - 0.6666666666666666f64;
                            dz_ext1 = (dz0 + 1.0f64) - 0.3333333333333333f64;
                        } else {
                            let chain_2 = zsb + 1;
                            zsv_ext1 = chain_2;
                            zsv_ext0 = chain_2;
                            dz_ext0 = (dz0 - 1.0f64) - 0.6666666666666666f64;
                            dz_ext1 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                        }
                    } else {
                        let c: i8 = ((if b_score > a_score { (b_point as i32) } else { (a_point as i32) }) as i8);
                        if ((c as i32) & 1) == 0 {
                            xsv_ext0 = xsb - 1;
                            xsv_ext1 = xsb;
                            dx_ext0 = dx0 + 1.0f64;
                            dx_ext1 = dx0;
                        } else {
                            let chain_3 = xsb + 1;
                            xsv_ext1 = chain_3;
                            xsv_ext0 = chain_3;
                            let chain_4 = dx0 - 1.0f64;
                            dx_ext1 = chain_4;
                            dx_ext0 = chain_4;
                        }
                        if ((c as i32) & 2) == 0 {
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            dy_ext1 = dy0;
                            dy_ext0 = dy0;
                            if ((c as i32) & 1) == 0 {
                                ysv_ext1 -= 1;
                                dy_ext1 += 1.0f64;
                            } else {
                                ysv_ext0 -= 1;
                                dy_ext0 += 1.0f64;
                            }
                        } else {
                            let chain_5 = ysb + 1;
                            ysv_ext1 = chain_5;
                            ysv_ext0 = chain_5;
                            let chain_6 = dy0 - 1.0f64;
                            dy_ext1 = chain_6;
                            dy_ext0 = chain_6;
                        }
                        if ((c as i32) & 4) == 0 {
                            zsv_ext0 = zsb;
                            zsv_ext1 = zsb - 1;
                            dz_ext0 = dz0;
                            dz_ext1 = dz0 + 1.0f64;
                        } else {
                            let chain_7 = zsb + 1;
                            zsv_ext1 = chain_7;
                            zsv_ext0 = chain_7;
                            let chain_8 = dz0 - 1.0f64;
                            dz_ext1 = chain_8;
                            dz_ext0 = chain_8;
                        }
                    }
                    let mut attn0: f64 = ((2.0f64 - (dx0 * dx0)) - (dy0 * dy0)) - (dz0 * dz0);
                    if attn0 > 0.0f64 {
                        attn0 *= attn0;
                        value += (attn0 * attn0) * (self.extrapolate_6(xsb + 0, ysb + 0, zsb + 0, dx0, dy0, dz0));
                    }
                    let dx1: f64 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                    let dy1: f64 = (dy0 - 0.0f64) - 0.3333333333333333f64;
                    let dz1: f64 = (dz0 - 0.0f64) - 0.3333333333333333f64;
                    let mut attn1: f64 = ((2.0f64 - (dx1 * dx1)) - (dy1 * dy1)) - (dz1 * dz1);
                    if attn1 > 0.0f64 {
                        attn1 *= attn1;
                        value += (attn1 * attn1) * (self.extrapolate_6(xsb + 1, ysb + 0, zsb + 0, dx1, dy1, dz1));
                    }
                    let dx2: f64 = (dx0 - 0.0f64) - 0.3333333333333333f64;
                    let dy2: f64 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                    let dz2: f64 = dz1;
                    let mut attn2: f64 = ((2.0f64 - (dx2 * dx2)) - (dy2 * dy2)) - (dz2 * dz2);
                    if attn2 > 0.0f64 {
                        attn2 *= attn2;
                        value += (attn2 * attn2) * (self.extrapolate_6(xsb + 0, ysb + 1, zsb + 0, dx2, dy2, dz2));
                    }
                    let dx3: f64 = dx2;
                    let dy3: f64 = dy1;
                    let dz3: f64 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                    let mut attn3: f64 = ((2.0f64 - (dx3 * dx3)) - (dy3 * dy3)) - (dz3 * dz3);
                    if attn3 > 0.0f64 {
                        attn3 *= attn3;
                        value += (attn3 * attn3) * (self.extrapolate_6(xsb + 0, ysb + 0, zsb + 1, dx3, dy3, dz3));
                    }
                } else if in_sum >= 2.0f64 {
                    let mut a_point: i8 = 6i8;
                    let mut a_score: f64 = xins;
                    let mut b_point: i8 = 5i8;
                    let mut b_score: f64 = yins;
                    if (a_score <= b_score) && (zins < b_score) {
                        b_score = zins;
                        b_point = 3i8;
                    } else if (a_score > b_score) && (zins < a_score) {
                        a_score = zins;
                        a_point = 3i8;
                    }
                    let wins: f64 = 3.0f64 - in_sum;
                    if (!(wins < a_score)) && (!(wins < b_score)) {
                        let c: i8 = (((a_point as i32) & (b_point as i32)) as i8);
                        if ((c as i32) & 1) != 0 {
                            xsv_ext0 = xsb + 1;
                            xsv_ext1 = xsb + 2;
                            dx_ext0 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                            dx_ext1 = (dx0 - 2.0f64) - 0.6666666666666666f64;
                        } else {
                            xsv_ext1 = xsb;
                            xsv_ext0 = xsb;
                            dx_ext0 = dx0 - 0.3333333333333333f64;
                            dx_ext1 = dx0 - 0.6666666666666666f64;
                        }
                        if ((c as i32) & 2) != 0 {
                            ysv_ext0 = ysb + 1;
                            ysv_ext1 = ysb + 2;
                            dy_ext0 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                            dy_ext1 = (dy0 - 2.0f64) - 0.6666666666666666f64;
                        } else {
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            dy_ext0 = dy0 - 0.3333333333333333f64;
                            dy_ext1 = dy0 - 0.6666666666666666f64;
                        }
                        if ((c as i32) & 4) != 0 {
                            zsv_ext0 = zsb + 1;
                            zsv_ext1 = zsb + 2;
                            dz_ext0 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                            dz_ext1 = (dz0 - 2.0f64) - 0.6666666666666666f64;
                        } else {
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            dz_ext0 = dz0 - 0.3333333333333333f64;
                            dz_ext1 = dz0 - 0.6666666666666666f64;
                        }
                    } else {
                        let c: i8 = ((if b_score < a_score { (b_point as i32) } else { (a_point as i32) }) as i8);
                        if ((c as i32) & 1) != 0 {
                            xsv_ext0 = xsb + 2;
                            xsv_ext1 = xsb + 1;
                            dx_ext0 = (dx0 - 2.0f64) - 1.0f64;
                            dx_ext1 = (dx0 - 1.0f64) - 1.0f64;
                        } else {
                            xsv_ext1 = xsb;
                            xsv_ext0 = xsb;
                            let chain_9 = dx0 - 1.0f64;
                            dx_ext1 = chain_9;
                            dx_ext0 = chain_9;
                        }
                        if ((c as i32) & 2) != 0 {
                            let chain_10 = ysb + 1;
                            ysv_ext1 = chain_10;
                            ysv_ext0 = chain_10;
                            let chain_11 = (dy0 - 1.0f64) - 1.0f64;
                            dy_ext1 = chain_11;
                            dy_ext0 = chain_11;
                            if ((c as i32) & 1) != 0 {
                                ysv_ext1 += 1;
                                dy_ext1 -= 1.0f64;
                            } else {
                                ysv_ext0 += 1;
                                dy_ext0 -= 1.0f64;
                            }
                        } else {
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            let chain_12 = dy0 - 1.0f64;
                            dy_ext1 = chain_12;
                            dy_ext0 = chain_12;
                        }
                        if ((c as i32) & 4) != 0 {
                            zsv_ext0 = zsb + 1;
                            zsv_ext1 = zsb + 2;
                            dz_ext0 = (dz0 - 1.0f64) - 1.0f64;
                            dz_ext1 = (dz0 - 2.0f64) - 1.0f64;
                        } else {
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            let chain_13 = dz0 - 1.0f64;
                            dz_ext1 = chain_13;
                            dz_ext0 = chain_13;
                        }
                    }
                    let dx3: f64 = (dx0 - 1.0f64) - 0.6666666666666666f64;
                    let dy3: f64 = (dy0 - 1.0f64) - 0.6666666666666666f64;
                    let dz3: f64 = (dz0 - 0.0f64) - 0.6666666666666666f64;
                    let mut attn3: f64 = ((2.0f64 - (dx3 * dx3)) - (dy3 * dy3)) - (dz3 * dz3);
                    if attn3 > 0.0f64 {
                        attn3 *= attn3;
                        value += (attn3 * attn3) * (self.extrapolate_6(xsb + 1, ysb + 1, zsb + 0, dx3, dy3, dz3));
                    }
                    let dx2: f64 = dx3;
                    let dy2: f64 = (dy0 - 0.0f64) - 0.6666666666666666f64;
                    let dz2: f64 = (dz0 - 1.0f64) - 0.6666666666666666f64;
                    let mut attn2: f64 = ((2.0f64 - (dx2 * dx2)) - (dy2 * dy2)) - (dz2 * dz2);
                    if attn2 > 0.0f64 {
                        attn2 *= attn2;
                        value += (attn2 * attn2) * (self.extrapolate_6(xsb + 1, ysb + 0, zsb + 1, dx2, dy2, dz2));
                    }
                    let dx1: f64 = (dx0 - 0.0f64) - 0.6666666666666666f64;
                    let dy1: f64 = dy3;
                    let dz1: f64 = dz2;
                    let mut attn1: f64 = ((2.0f64 - (dx1 * dx1)) - (dy1 * dy1)) - (dz1 * dz1);
                    if attn1 > 0.0f64 {
                        attn1 *= attn1;
                        value += (attn1 * attn1) * (self.extrapolate_6(xsb + 0, ysb + 1, zsb + 1, dx1, dy1, dz1));
                    }
                    dx0 = (dx0 - 1.0f64) - 1.0f64;
                    dy0 = (dy0 - 1.0f64) - 1.0f64;
                    dz0 = (dz0 - 1.0f64) - 1.0f64;
                    let mut attn0: f64 = ((2.0f64 - (dx0 * dx0)) - (dy0 * dy0)) - (dz0 * dz0);
                    if attn0 > 0.0f64 {
                        attn0 *= attn0;
                        value += (attn0 * attn0) * (self.extrapolate_6(xsb + 1, ysb + 1, zsb + 1, dx0, dy0, dz0));
                    }
                } else {
                    let p1: f64 = xins + yins;
                    let mut a_point: i8;
                    let mut a_score: f64;
                    let mut a_is_further_side: bool;
                    if p1 > 1.0f64 {
                        a_score = p1 - 1.0f64;
                        a_point = 3i8;
                        a_is_further_side = true;
                    } else {
                        a_score = 1.0f64 - p1;
                        a_point = 4i8;
                        a_is_further_side = false;
                    }
                    let p2: f64 = xins + zins;
                    let mut b_is_further_side: bool;
                    let mut b_score: f64;
                    let mut b_point: i8;
                    if p2 > 1.0f64 {
                        b_score = p2 - 1.0f64;
                        b_point = 5i8;
                        b_is_further_side = true;
                    } else {
                        b_score = 1.0f64 - p2;
                        b_point = 2i8;
                        b_is_further_side = false;
                    }
                    let p3: f64 = yins + zins;
                    if p3 > 1.0f64 {
                        let score: f64 = p3 - 1.0f64;
                        if (a_score <= b_score) && (a_score < score) {
                            a_point = 6i8;
                            a_is_further_side = true;
                        } else if (a_score > b_score) && (b_score < score) {
                            b_point = 6i8;
                            b_is_further_side = true;
                        }
                    } else {
                        let score: f64 = 1.0f64 - p3;
                        if (a_score <= b_score) && (a_score < score) {
                            a_point = 1i8;
                            a_is_further_side = false;
                        } else if (a_score > b_score) && (b_score < score) {
                            b_point = 1i8;
                            b_is_further_side = false;
                        }
                    }
                    if a_is_further_side == b_is_further_side {
                        if a_is_further_side {
                            dx_ext0 = (dx0 - 1.0f64) - 1.0f64;
                            dy_ext0 = (dy0 - 1.0f64) - 1.0f64;
                            dz_ext0 = (dz0 - 1.0f64) - 1.0f64;
                            xsv_ext0 = xsb + 1;
                            ysv_ext0 = ysb + 1;
                            zsv_ext0 = zsb + 1;
                            let c: i8 = (((a_point as i32) & (b_point as i32)) as i8);
                            if ((c as i32) & 1) != 0 {
                                dx_ext1 = (dx0 - 2.0f64) - 0.6666666666666666f64;
                                dy_ext1 = dy0 - 0.6666666666666666f64;
                                dz_ext1 = dz0 - 0.6666666666666666f64;
                                xsv_ext1 = xsb + 2;
                                ysv_ext1 = ysb;
                                zsv_ext1 = zsb;
                            } else if ((c as i32) & 2) != 0 {
                                dx_ext1 = dx0 - 0.6666666666666666f64;
                                dy_ext1 = (dy0 - 2.0f64) - 0.6666666666666666f64;
                                dz_ext1 = dz0 - 0.6666666666666666f64;
                                xsv_ext1 = xsb;
                                ysv_ext1 = ysb + 2;
                                zsv_ext1 = zsb;
                            } else {
                                dx_ext1 = dx0 - 0.6666666666666666f64;
                                dy_ext1 = dy0 - 0.6666666666666666f64;
                                dz_ext1 = (dz0 - 2.0f64) - 0.6666666666666666f64;
                                xsv_ext1 = xsb;
                                ysv_ext1 = ysb;
                                zsv_ext1 = zsb + 2;
                            }
                        } else {
                            dx_ext0 = dx0;
                            dy_ext0 = dy0;
                            dz_ext0 = dz0;
                            xsv_ext0 = xsb;
                            ysv_ext0 = ysb;
                            zsv_ext0 = zsb;
                            let c: i8 = (((a_point as i32) | (b_point as i32)) as i8);
                            if ((c as i32) & 1) == 0 {
                                dx_ext1 = (dx0 + 1.0f64) - 0.3333333333333333f64;
                                dy_ext1 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                                dz_ext1 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                                xsv_ext1 = xsb - 1;
                                ysv_ext1 = ysb + 1;
                                zsv_ext1 = zsb + 1;
                            } else if ((c as i32) & 2) == 0 {
                                dx_ext1 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                                dy_ext1 = (dy0 + 1.0f64) - 0.3333333333333333f64;
                                dz_ext1 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                                xsv_ext1 = xsb + 1;
                                ysv_ext1 = ysb - 1;
                                zsv_ext1 = zsb + 1;
                            } else {
                                dx_ext1 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                                dy_ext1 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                                dz_ext1 = (dz0 + 1.0f64) - 0.3333333333333333f64;
                                xsv_ext1 = xsb + 1;
                                ysv_ext1 = ysb + 1;
                                zsv_ext1 = zsb - 1;
                            }
                        }
                    } else {
                        let mut c2: i8;
                        let mut c1: i8;
                        if a_is_further_side {
                            c1 = a_point;
                            c2 = b_point;
                        } else {
                            c1 = b_point;
                            c2 = a_point;
                        }
                        if ((c1 as i32) & 1) == 0 {
                            dx_ext0 = (dx0 + 1.0f64) - 0.3333333333333333f64;
                            dy_ext0 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                            dz_ext0 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                            xsv_ext0 = xsb - 1;
                            ysv_ext0 = ysb + 1;
                            zsv_ext0 = zsb + 1;
                        } else if ((c1 as i32) & 2) == 0 {
                            dx_ext0 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                            dy_ext0 = (dy0 + 1.0f64) - 0.3333333333333333f64;
                            dz_ext0 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                            xsv_ext0 = xsb + 1;
                            ysv_ext0 = ysb - 1;
                            zsv_ext0 = zsb + 1;
                        } else {
                            dx_ext0 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                            dy_ext0 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                            dz_ext0 = (dz0 + 1.0f64) - 0.3333333333333333f64;
                            xsv_ext0 = xsb + 1;
                            ysv_ext0 = ysb + 1;
                            zsv_ext0 = zsb - 1;
                        }
                        dx_ext1 = dx0 - 0.6666666666666666f64;
                        dy_ext1 = dy0 - 0.6666666666666666f64;
                        dz_ext1 = dz0 - 0.6666666666666666f64;
                        xsv_ext1 = xsb;
                        ysv_ext1 = ysb;
                        zsv_ext1 = zsb;
                        if ((c2 as i32) & 1) != 0 {
                            dx_ext1 -= 2.0f64;
                            xsv_ext1 += 2;
                        } else if ((c2 as i32) & 2) != 0 {
                            dy_ext1 -= 2.0f64;
                            ysv_ext1 += 2;
                        } else {
                            dz_ext1 -= 2.0f64;
                            zsv_ext1 += 2;
                        }
                    }
                    let dx1: f64 = (dx0 - 1.0f64) - 0.3333333333333333f64;
                    let dy1: f64 = (dy0 - 0.0f64) - 0.3333333333333333f64;
                    let dz1: f64 = (dz0 - 0.0f64) - 0.3333333333333333f64;
                    let mut attn1: f64 = ((2.0f64 - (dx1 * dx1)) - (dy1 * dy1)) - (dz1 * dz1);
                    if attn1 > 0.0f64 {
                        attn1 *= attn1;
                        value += (attn1 * attn1) * (self.extrapolate_6(xsb + 1, ysb + 0, zsb + 0, dx1, dy1, dz1));
                    }
                    let dx2: f64 = (dx0 - 0.0f64) - 0.3333333333333333f64;
                    let dy2: f64 = (dy0 - 1.0f64) - 0.3333333333333333f64;
                    let dz2: f64 = dz1;
                    let mut attn2: f64 = ((2.0f64 - (dx2 * dx2)) - (dy2 * dy2)) - (dz2 * dz2);
                    if attn2 > 0.0f64 {
                        attn2 *= attn2;
                        value += (attn2 * attn2) * (self.extrapolate_6(xsb + 0, ysb + 1, zsb + 0, dx2, dy2, dz2));
                    }
                    let dx3: f64 = dx2;
                    let dy3: f64 = dy1;
                    let dz3: f64 = (dz0 - 1.0f64) - 0.3333333333333333f64;
                    let mut attn3: f64 = ((2.0f64 - (dx3 * dx3)) - (dy3 * dy3)) - (dz3 * dz3);
                    if attn3 > 0.0f64 {
                        attn3 *= attn3;
                        value += (attn3 * attn3) * (self.extrapolate_6(xsb + 0, ysb + 0, zsb + 1, dx3, dy3, dz3));
                    }
                    let dx4: f64 = (dx0 - 1.0f64) - 0.6666666666666666f64;
                    let dy4: f64 = (dy0 - 1.0f64) - 0.6666666666666666f64;
                    let dz4: f64 = (dz0 - 0.0f64) - 0.6666666666666666f64;
                    let mut attn4: f64 = ((2.0f64 - (dx4 * dx4)) - (dy4 * dy4)) - (dz4 * dz4);
                    if attn4 > 0.0f64 {
                        attn4 *= attn4;
                        value += (attn4 * attn4) * (self.extrapolate_6(xsb + 1, ysb + 1, zsb + 0, dx4, dy4, dz4));
                    }
                    let dx5: f64 = dx4;
                    let dy5: f64 = (dy0 - 0.0f64) - 0.6666666666666666f64;
                    let dz5: f64 = (dz0 - 1.0f64) - 0.6666666666666666f64;
                    let mut attn5: f64 = ((2.0f64 - (dx5 * dx5)) - (dy5 * dy5)) - (dz5 * dz5);
                    if attn5 > 0.0f64 {
                        attn5 *= attn5;
                        value += (attn5 * attn5) * (self.extrapolate_6(xsb + 1, ysb + 0, zsb + 1, dx5, dy5, dz5));
                    }
                    let dx6: f64 = (dx0 - 0.0f64) - 0.6666666666666666f64;
                    let dy6: f64 = dy4;
                    let dz6: f64 = dz5;
                    let mut attn6: f64 = ((2.0f64 - (dx6 * dx6)) - (dy6 * dy6)) - (dz6 * dz6);
                    if attn6 > 0.0f64 {
                        attn6 *= attn6;
                        value += (attn6 * attn6) * (self.extrapolate_6(xsb + 0, ysb + 1, zsb + 1, dx6, dy6, dz6));
                    }
                }
                let mut attn_ext0: f64 = ((2.0f64 - (dx_ext0 * dx_ext0)) - (dy_ext0 * dy_ext0)) - (dz_ext0 * dz_ext0);
                if attn_ext0 > 0.0f64 {
                    attn_ext0 *= attn_ext0;
                    value += (attn_ext0 * attn_ext0) * (self.extrapolate_6(xsv_ext0, ysv_ext0, zsv_ext0, dx_ext0, dy_ext0, dz_ext0));
                }
                let mut attn_ext1: f64 = ((2.0f64 - (dx_ext1 * dx_ext1)) - (dy_ext1 * dy_ext1)) - (dz_ext1 * dz_ext1);
                if attn_ext1 > 0.0f64 {
                    attn_ext1 *= attn_ext1;
                    value += (attn_ext1 * attn_ext1) * (self.extrapolate_6(xsv_ext1, ysv_ext1, zsv_ext1, dx_ext1, dy_ext1, dz_ext1));
                }
                return value / 103.0f64;
            }
            pub fn eval_4(&self, x: f64, y: f64, z: f64, w: f64) -> f64 {
                let stretch_offset: f64 = (((x + y) + z) + w) * (-0.138196601125011f64);
                let xs: f64 = x + stretch_offset;
                let ys: f64 = y + stretch_offset;
                let zs: f64 = z + stretch_offset;
                let ws: f64 = w + stretch_offset;
                let xsb: i32 = open_simplex_noise::fast_floor(xs);
                let ysb: i32 = open_simplex_noise::fast_floor(ys);
                let zsb: i32 = open_simplex_noise::fast_floor(zs);
                let wsb: i32 = open_simplex_noise::fast_floor(ws);
                let squish_offset: f64 = ((((xsb + ysb) + zsb) + wsb) as f64) * 0.309016994374947f64;
                let xb: f64 = (xsb as f64) + squish_offset;
                let yb: f64 = (ysb as f64) + squish_offset;
                let zb: f64 = (zsb as f64) + squish_offset;
                let wb: f64 = (wsb as f64) + squish_offset;
                let xins: f64 = xs - (xsb as f64);
                let yins: f64 = ys - (ysb as f64);
                let zins: f64 = zs - (zsb as f64);
                let wins: f64 = ws - (wsb as f64);
                let in_sum: f64 = ((xins + yins) + zins) + wins;
                let mut dx0: f64 = x - xb;
                let mut dy0: f64 = y - yb;
                let mut dz0: f64 = z - zb;
                let mut dw0: f64 = w - wb;
                let mut value: f64 = 0.0f64;
                let mut dx_ext0: f64;
                let mut dy_ext0: f64;
                let mut dz_ext0: f64;
                let mut dw_ext0: f64;
                let mut dx_ext1: f64;
                let mut dy_ext1: f64;
                let mut dz_ext1: f64;
                let mut dw_ext1: f64;
                let mut dx_ext2: f64;
                let mut dy_ext2: f64;
                let mut dz_ext2: f64;
                let mut dw_ext2: f64;
                let mut xsv_ext0: i32;
                let mut ysv_ext0: i32;
                let mut zsv_ext0: i32;
                let mut wsv_ext0: i32;
                let mut xsv_ext1: i32;
                let mut ysv_ext1: i32;
                let mut zsv_ext1: i32;
                let mut wsv_ext1: i32;
                let mut xsv_ext2: i32;
                let mut ysv_ext2: i32;
                let mut zsv_ext2: i32;
                let mut wsv_ext2: i32;
                if in_sum <= 1.0f64 {
                    let mut a_point: i8 = 1i8;
                    let mut a_score: f64 = xins;
                    let mut b_point: i8 = 2i8;
                    let mut b_score: f64 = yins;
                    if (a_score >= b_score) && (zins > b_score) {
                        b_score = zins;
                        b_point = 4i8;
                    } else if (a_score < b_score) && (zins > a_score) {
                        a_score = zins;
                        a_point = 4i8;
                    }
                    if (a_score >= b_score) && (wins > b_score) {
                        b_score = wins;
                        b_point = 8i8;
                    } else if (a_score < b_score) && (wins > a_score) {
                        a_score = wins;
                        a_point = 8i8;
                    }
                    let uins: f64 = 1.0f64 - in_sum;
                    if (!(uins > a_score)) && (!(uins > b_score)) {
                        let c: i8 = (((a_point as i32) | (b_point as i32)) as i8);
                        if ((c as i32) & 1) == 0 {
                            xsv_ext2 = xsb;
                            xsv_ext0 = xsb;
                            xsv_ext1 = xsb - 1;
                            dx_ext0 = dx0 - 0.618033988749894f64;
                            dx_ext1 = (dx0 + 1.0f64) - 0.309016994374947f64;
                            dx_ext2 = dx0 - 0.309016994374947f64;
                        } else {
                            let chain_0 = xsb + 1;
                            xsv_ext2 = chain_0;
                            xsv_ext1 = chain_0;
                            xsv_ext0 = chain_0;
                            dx_ext0 = (dx0 - 1.0f64) - 0.618033988749894f64;
                            let chain_1 = (dx0 - 1.0f64) - 0.309016994374947f64;
                            dx_ext2 = chain_1;
                            dx_ext1 = chain_1;
                        }
                        if ((c as i32) & 2) == 0 {
                            ysv_ext2 = ysb;
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            dy_ext0 = dy0 - 0.618033988749894f64;
                            let chain_2 = dy0 - 0.309016994374947f64;
                            dy_ext2 = chain_2;
                            dy_ext1 = chain_2;
                            if ((c as i32) & 1) == 1 {
                                ysv_ext1 -= 1;
                                dy_ext1 += 1.0f64;
                            } else {
                                ysv_ext2 -= 1;
                                dy_ext2 += 1.0f64;
                            }
                        } else {
                            let chain_3 = ysb + 1;
                            ysv_ext2 = chain_3;
                            ysv_ext1 = chain_3;
                            ysv_ext0 = chain_3;
                            dy_ext0 = (dy0 - 1.0f64) - 0.618033988749894f64;
                            let chain_4 = (dy0 - 1.0f64) - 0.309016994374947f64;
                            dy_ext2 = chain_4;
                            dy_ext1 = chain_4;
                        }
                        if ((c as i32) & 4) == 0 {
                            zsv_ext2 = zsb;
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            dz_ext0 = dz0 - 0.618033988749894f64;
                            let chain_5 = dz0 - 0.309016994374947f64;
                            dz_ext2 = chain_5;
                            dz_ext1 = chain_5;
                            if ((c as i32) & 3) == 3 {
                                zsv_ext1 -= 1;
                                dz_ext1 += 1.0f64;
                            } else {
                                zsv_ext2 -= 1;
                                dz_ext2 += 1.0f64;
                            }
                        } else {
                            let chain_6 = zsb + 1;
                            zsv_ext2 = chain_6;
                            zsv_ext1 = chain_6;
                            zsv_ext0 = chain_6;
                            dz_ext0 = (dz0 - 1.0f64) - 0.618033988749894f64;
                            let chain_7 = (dz0 - 1.0f64) - 0.309016994374947f64;
                            dz_ext2 = chain_7;
                            dz_ext1 = chain_7;
                        }
                        if ((c as i32) & 8) == 0 {
                            wsv_ext1 = wsb;
                            wsv_ext0 = wsb;
                            wsv_ext2 = wsb - 1;
                            dw_ext0 = dw0 - 0.618033988749894f64;
                            dw_ext1 = dw0 - 0.309016994374947f64;
                            dw_ext2 = (dw0 + 1.0f64) - 0.309016994374947f64;
                        } else {
                            let chain_8 = wsb + 1;
                            wsv_ext2 = chain_8;
                            wsv_ext1 = chain_8;
                            wsv_ext0 = chain_8;
                            dw_ext0 = (dw0 - 1.0f64) - 0.618033988749894f64;
                            let chain_9 = (dw0 - 1.0f64) - 0.309016994374947f64;
                            dw_ext2 = chain_9;
                            dw_ext1 = chain_9;
                        }
                    } else {
                        let c: i8 = ((if b_score > a_score { (b_point as i32) } else { (a_point as i32) }) as i8);
                        if ((c as i32) & 1) == 0 {
                            xsv_ext0 = xsb - 1;
                            xsv_ext2 = xsb;
                            xsv_ext1 = xsb;
                            dx_ext0 = dx0 + 1.0f64;
                            dx_ext2 = dx0;
                            dx_ext1 = dx0;
                        } else {
                            let chain_10 = xsb + 1;
                            xsv_ext2 = chain_10;
                            xsv_ext1 = chain_10;
                            xsv_ext0 = chain_10;
                            let chain_11 = dx0 - 1.0f64;
                            dx_ext2 = chain_11;
                            dx_ext1 = chain_11;
                            dx_ext0 = chain_11;
                        }
                        if ((c as i32) & 2) == 0 {
                            ysv_ext2 = ysb;
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            dy_ext2 = dy0;
                            dy_ext1 = dy0;
                            dy_ext0 = dy0;
                            if ((c as i32) & 1) == 1 {
                                ysv_ext0 -= 1;
                                dy_ext0 += 1.0f64;
                            } else {
                                ysv_ext1 -= 1;
                                dy_ext1 += 1.0f64;
                            }
                        } else {
                            let chain_12 = ysb + 1;
                            ysv_ext2 = chain_12;
                            ysv_ext1 = chain_12;
                            ysv_ext0 = chain_12;
                            let chain_13 = dy0 - 1.0f64;
                            dy_ext2 = chain_13;
                            dy_ext1 = chain_13;
                            dy_ext0 = chain_13;
                        }
                        if ((c as i32) & 4) == 0 {
                            zsv_ext2 = zsb;
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            dz_ext2 = dz0;
                            dz_ext1 = dz0;
                            dz_ext0 = dz0;
                            if ((c as i32) & 3) != 0 {
                                if ((c as i32) & 3) == 3 {
                                    zsv_ext0 -= 1;
                                    dz_ext0 += 1.0f64;
                                } else {
                                    zsv_ext1 -= 1;
                                    dz_ext1 += 1.0f64;
                                }
                            } else {
                                zsv_ext2 -= 1;
                                dz_ext2 += 1.0f64;
                            }
                        } else {
                            let chain_14 = zsb + 1;
                            zsv_ext2 = chain_14;
                            zsv_ext1 = chain_14;
                            zsv_ext0 = chain_14;
                            let chain_15 = dz0 - 1.0f64;
                            dz_ext2 = chain_15;
                            dz_ext1 = chain_15;
                            dz_ext0 = chain_15;
                        }
                        if ((c as i32) & 8) == 0 {
                            wsv_ext1 = wsb;
                            wsv_ext0 = wsb;
                            wsv_ext2 = wsb - 1;
                            dw_ext1 = dw0;
                            dw_ext0 = dw0;
                            dw_ext2 = dw0 + 1.0f64;
                        } else {
                            let chain_16 = wsb + 1;
                            wsv_ext2 = chain_16;
                            wsv_ext1 = chain_16;
                            wsv_ext0 = chain_16;
                            let chain_17 = dw0 - 1.0f64;
                            dw_ext2 = chain_17;
                            dw_ext1 = chain_17;
                            dw_ext0 = chain_17;
                        }
                    }
                    let mut attn0: f64 = (((2.0f64 - (dx0 * dx0)) - (dy0 * dy0)) - (dz0 * dz0)) - (dw0 * dw0);
                    if attn0 > 0.0f64 {
                        attn0 *= attn0;
                        value += (attn0 * attn0) * (self.extrapolate_8(xsb + 0, ysb + 0, zsb + 0, wsb + 0, dx0, dy0, dz0, dw0));
                    }
                    let dx1: f64 = (dx0 - 1.0f64) - 0.309016994374947f64;
                    let dy1: f64 = (dy0 - 0.0f64) - 0.309016994374947f64;
                    let dz1: f64 = (dz0 - 0.0f64) - 0.309016994374947f64;
                    let dw1: f64 = (dw0 - 0.0f64) - 0.309016994374947f64;
                    let mut attn1: f64 = (((2.0f64 - (dx1 * dx1)) - (dy1 * dy1)) - (dz1 * dz1)) - (dw1 * dw1);
                    if attn1 > 0.0f64 {
                        attn1 *= attn1;
                        value += (attn1 * attn1) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 0, wsb + 0, dx1, dy1, dz1, dw1));
                    }
                    let dx2: f64 = (dx0 - 0.0f64) - 0.309016994374947f64;
                    let dy2: f64 = (dy0 - 1.0f64) - 0.309016994374947f64;
                    let dz2: f64 = dz1;
                    let dw2: f64 = dw1;
                    let mut attn2: f64 = (((2.0f64 - (dx2 * dx2)) - (dy2 * dy2)) - (dz2 * dz2)) - (dw2 * dw2);
                    if attn2 > 0.0f64 {
                        attn2 *= attn2;
                        value += (attn2 * attn2) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 0, wsb + 0, dx2, dy2, dz2, dw2));
                    }
                    let dx3: f64 = dx2;
                    let dy3: f64 = dy1;
                    let dz3: f64 = (dz0 - 1.0f64) - 0.309016994374947f64;
                    let dw3: f64 = dw1;
                    let mut attn3: f64 = (((2.0f64 - (dx3 * dx3)) - (dy3 * dy3)) - (dz3 * dz3)) - (dw3 * dw3);
                    if attn3 > 0.0f64 {
                        attn3 *= attn3;
                        value += (attn3 * attn3) * (self.extrapolate_8(xsb + 0, ysb + 0, zsb + 1, wsb + 0, dx3, dy3, dz3, dw3));
                    }
                    let dx4: f64 = dx2;
                    let dy4: f64 = dy1;
                    let dz4: f64 = dz1;
                    let dw4: f64 = (dw0 - 1.0f64) - 0.309016994374947f64;
                    let mut attn4: f64 = (((2.0f64 - (dx4 * dx4)) - (dy4 * dy4)) - (dz4 * dz4)) - (dw4 * dw4);
                    if attn4 > 0.0f64 {
                        attn4 *= attn4;
                        value += (attn4 * attn4) * (self.extrapolate_8(xsb + 0, ysb + 0, zsb + 0, wsb + 1, dx4, dy4, dz4, dw4));
                    }
                } else if in_sum >= 3.0f64 {
                    let mut a_point: i8 = 14i8;
                    let mut a_score: f64 = xins;
                    let mut b_point: i8 = 13i8;
                    let mut b_score: f64 = yins;
                    if (a_score <= b_score) && (zins < b_score) {
                        b_score = zins;
                        b_point = 11i8;
                    } else if (a_score > b_score) && (zins < a_score) {
                        a_score = zins;
                        a_point = 11i8;
                    }
                    if (a_score <= b_score) && (wins < b_score) {
                        b_score = wins;
                        b_point = 7i8;
                    } else if (a_score > b_score) && (wins < a_score) {
                        a_score = wins;
                        a_point = 7i8;
                    }
                    let uins: f64 = 4.0f64 - in_sum;
                    if (!(uins < a_score)) && (!(uins < b_score)) {
                        let c: i8 = (((a_point as i32) & (b_point as i32)) as i8);
                        if ((c as i32) & 1) != 0 {
                            let chain_18 = xsb + 1;
                            xsv_ext2 = chain_18;
                            xsv_ext0 = chain_18;
                            xsv_ext1 = xsb + 2;
                            dx_ext0 = (dx0 - 1.0f64) - 0.618033988749894f64;
                            dx_ext1 = (dx0 - 2.0f64) - 0.927050983124841f64;
                            dx_ext2 = (dx0 - 1.0f64) - 0.927050983124841f64;
                        } else {
                            xsv_ext2 = xsb;
                            xsv_ext1 = xsb;
                            xsv_ext0 = xsb;
                            dx_ext0 = dx0 - 0.618033988749894f64;
                            let chain_19 = dx0 - 0.927050983124841f64;
                            dx_ext2 = chain_19;
                            dx_ext1 = chain_19;
                        }
                        if ((c as i32) & 2) != 0 {
                            let chain_20 = ysb + 1;
                            ysv_ext2 = chain_20;
                            ysv_ext1 = chain_20;
                            ysv_ext0 = chain_20;
                            dy_ext0 = (dy0 - 1.0f64) - 0.618033988749894f64;
                            let chain_21 = (dy0 - 1.0f64) - 0.927050983124841f64;
                            dy_ext2 = chain_21;
                            dy_ext1 = chain_21;
                            if ((c as i32) & 1) != 0 {
                                ysv_ext2 += 1;
                                dy_ext2 -= 1.0f64;
                            } else {
                                ysv_ext1 += 1;
                                dy_ext1 -= 1.0f64;
                            }
                        } else {
                            ysv_ext2 = ysb;
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            dy_ext0 = dy0 - 0.618033988749894f64;
                            let chain_22 = dy0 - 0.927050983124841f64;
                            dy_ext2 = chain_22;
                            dy_ext1 = chain_22;
                        }
                        if ((c as i32) & 4) != 0 {
                            let chain_23 = zsb + 1;
                            zsv_ext2 = chain_23;
                            zsv_ext1 = chain_23;
                            zsv_ext0 = chain_23;
                            dz_ext0 = (dz0 - 1.0f64) - 0.618033988749894f64;
                            let chain_24 = (dz0 - 1.0f64) - 0.927050983124841f64;
                            dz_ext2 = chain_24;
                            dz_ext1 = chain_24;
                            if ((c as i32) & 3) != 0 {
                                zsv_ext2 += 1;
                                dz_ext2 -= 1.0f64;
                            } else {
                                zsv_ext1 += 1;
                                dz_ext1 -= 1.0f64;
                            }
                        } else {
                            zsv_ext2 = zsb;
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            dz_ext0 = dz0 - 0.618033988749894f64;
                            let chain_25 = dz0 - 0.927050983124841f64;
                            dz_ext2 = chain_25;
                            dz_ext1 = chain_25;
                        }
                        if ((c as i32) & 8) != 0 {
                            let chain_26 = wsb + 1;
                            wsv_ext1 = chain_26;
                            wsv_ext0 = chain_26;
                            wsv_ext2 = wsb + 2;
                            dw_ext0 = (dw0 - 1.0f64) - 0.618033988749894f64;
                            dw_ext1 = (dw0 - 1.0f64) - 0.927050983124841f64;
                            dw_ext2 = (dw0 - 2.0f64) - 0.927050983124841f64;
                        } else {
                            wsv_ext2 = wsb;
                            wsv_ext1 = wsb;
                            wsv_ext0 = wsb;
                            dw_ext0 = dw0 - 0.618033988749894f64;
                            let chain_27 = dw0 - 0.927050983124841f64;
                            dw_ext2 = chain_27;
                            dw_ext1 = chain_27;
                        }
                    } else {
                        let c: i8 = ((if b_score < a_score { (b_point as i32) } else { (a_point as i32) }) as i8);
                        if ((c as i32) & 1) != 0 {
                            xsv_ext0 = xsb + 2;
                            let chain_28 = xsb + 1;
                            xsv_ext2 = chain_28;
                            xsv_ext1 = chain_28;
                            dx_ext0 = (dx0 - 2.0f64) - 1.236067977499788f64;
                            let chain_29 = (dx0 - 1.0f64) - 1.236067977499788f64;
                            dx_ext2 = chain_29;
                            dx_ext1 = chain_29;
                        } else {
                            xsv_ext2 = xsb;
                            xsv_ext1 = xsb;
                            xsv_ext0 = xsb;
                            let chain_30 = dx0 - 1.236067977499788f64;
                            dx_ext2 = chain_30;
                            dx_ext1 = chain_30;
                            dx_ext0 = chain_30;
                        }
                        if ((c as i32) & 2) != 0 {
                            let chain_31 = ysb + 1;
                            ysv_ext2 = chain_31;
                            ysv_ext1 = chain_31;
                            ysv_ext0 = chain_31;
                            let chain_32 = (dy0 - 1.0f64) - 1.236067977499788f64;
                            dy_ext2 = chain_32;
                            dy_ext1 = chain_32;
                            dy_ext0 = chain_32;
                            if ((c as i32) & 1) != 0 {
                                ysv_ext1 += 1;
                                dy_ext1 -= 1.0f64;
                            } else {
                                ysv_ext0 += 1;
                                dy_ext0 -= 1.0f64;
                            }
                        } else {
                            ysv_ext2 = ysb;
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            let chain_33 = dy0 - 1.236067977499788f64;
                            dy_ext2 = chain_33;
                            dy_ext1 = chain_33;
                            dy_ext0 = chain_33;
                        }
                        if ((c as i32) & 4) != 0 {
                            let chain_34 = zsb + 1;
                            zsv_ext2 = chain_34;
                            zsv_ext1 = chain_34;
                            zsv_ext0 = chain_34;
                            let chain_35 = (dz0 - 1.0f64) - 1.236067977499788f64;
                            dz_ext2 = chain_35;
                            dz_ext1 = chain_35;
                            dz_ext0 = chain_35;
                            if ((c as i32) & 3) != 3 {
                                if ((c as i32) & 3) == 0 {
                                    zsv_ext0 += 1;
                                    dz_ext0 -= 1.0f64;
                                } else {
                                    zsv_ext1 += 1;
                                    dz_ext1 -= 1.0f64;
                                }
                            } else {
                                zsv_ext2 += 1;
                                dz_ext2 -= 1.0f64;
                            }
                        } else {
                            zsv_ext2 = zsb;
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            let chain_36 = dz0 - 1.236067977499788f64;
                            dz_ext2 = chain_36;
                            dz_ext1 = chain_36;
                            dz_ext0 = chain_36;
                        }
                        if ((c as i32) & 8) != 0 {
                            let chain_37 = wsb + 1;
                            wsv_ext1 = chain_37;
                            wsv_ext0 = chain_37;
                            wsv_ext2 = wsb + 2;
                            let chain_38 = (dw0 - 1.0f64) - 1.236067977499788f64;
                            dw_ext1 = chain_38;
                            dw_ext0 = chain_38;
                            dw_ext2 = (dw0 - 2.0f64) - 1.236067977499788f64;
                        } else {
                            wsv_ext2 = wsb;
                            wsv_ext1 = wsb;
                            wsv_ext0 = wsb;
                            let chain_39 = dw0 - 1.236067977499788f64;
                            dw_ext2 = chain_39;
                            dw_ext1 = chain_39;
                            dw_ext0 = chain_39;
                        }
                    }
                    let dx4: f64 = (dx0 - 1.0f64) - 0.927050983124841f64;
                    let dy4: f64 = (dy0 - 1.0f64) - 0.927050983124841f64;
                    let dz4: f64 = (dz0 - 1.0f64) - 0.927050983124841f64;
                    let dw4: f64 = dw0 - 0.927050983124841f64;
                    let mut attn4: f64 = (((2.0f64 - (dx4 * dx4)) - (dy4 * dy4)) - (dz4 * dz4)) - (dw4 * dw4);
                    if attn4 > 0.0f64 {
                        attn4 *= attn4;
                        value += (attn4 * attn4) * (self.extrapolate_8(xsb + 1, ysb + 1, zsb + 1, wsb + 0, dx4, dy4, dz4, dw4));
                    }
                    let dx3: f64 = dx4;
                    let dy3: f64 = dy4;
                    let dz3: f64 = dz0 - 0.927050983124841f64;
                    let dw3: f64 = (dw0 - 1.0f64) - 0.927050983124841f64;
                    let mut attn3: f64 = (((2.0f64 - (dx3 * dx3)) - (dy3 * dy3)) - (dz3 * dz3)) - (dw3 * dw3);
                    if attn3 > 0.0f64 {
                        attn3 *= attn3;
                        value += (attn3 * attn3) * (self.extrapolate_8(xsb + 1, ysb + 1, zsb + 0, wsb + 1, dx3, dy3, dz3, dw3));
                    }
                    let dx2: f64 = dx4;
                    let dy2: f64 = dy0 - 0.927050983124841f64;
                    let dz2: f64 = dz4;
                    let dw2: f64 = dw3;
                    let mut attn2: f64 = (((2.0f64 - (dx2 * dx2)) - (dy2 * dy2)) - (dz2 * dz2)) - (dw2 * dw2);
                    if attn2 > 0.0f64 {
                        attn2 *= attn2;
                        value += (attn2 * attn2) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 1, wsb + 1, dx2, dy2, dz2, dw2));
                    }
                    let dx1: f64 = dx0 - 0.927050983124841f64;
                    let dz1: f64 = dz4;
                    let dy1: f64 = dy4;
                    let dw1: f64 = dw3;
                    let mut attn1: f64 = (((2.0f64 - (dx1 * dx1)) - (dy1 * dy1)) - (dz1 * dz1)) - (dw1 * dw1);
                    if attn1 > 0.0f64 {
                        attn1 *= attn1;
                        value += (attn1 * attn1) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 1, wsb + 1, dx1, dy1, dz1, dw1));
                    }
                    dx0 = (dx0 - 1.0f64) - 1.236067977499788f64;
                    dy0 = (dy0 - 1.0f64) - 1.236067977499788f64;
                    dz0 = (dz0 - 1.0f64) - 1.236067977499788f64;
                    dw0 = (dw0 - 1.0f64) - 1.236067977499788f64;
                    let mut attn0: f64 = (((2.0f64 - (dx0 * dx0)) - (dy0 * dy0)) - (dz0 * dz0)) - (dw0 * dw0);
                    if attn0 > 0.0f64 {
                        attn0 *= attn0;
                        value += (attn0 * attn0) * (self.extrapolate_8(xsb + 1, ysb + 1, zsb + 1, wsb + 1, dx0, dy0, dz0, dw0));
                    }
                } else if in_sum <= 2.0f64 {
                    let mut a_is_bigger_side: bool = true;
                    let mut b_is_bigger_side: bool = true;
                    let mut a_point: i8;
                    let mut a_score: f64;
                    if (xins + yins) > (zins + wins) {
                        a_score = xins + yins;
                        a_point = 3i8;
                    } else {
                        a_score = zins + wins;
                        a_point = 12i8;
                    }
                    let mut b_score: f64;
                    let mut b_point: i8;
                    if (xins + zins) > (yins + wins) {
                        b_score = xins + zins;
                        b_point = 5i8;
                    } else {
                        b_score = yins + wins;
                        b_point = 10i8;
                    }
                    if (xins + wins) > (yins + zins) {
                        let score: f64 = xins + wins;
                        if (a_score >= b_score) && (score > b_score) {
                            b_score = score;
                            b_point = 9i8;
                        } else if (a_score < b_score) && (score > a_score) {
                            a_score = score;
                            a_point = 9i8;
                        }
                    } else {
                        let score: f64 = yins + zins;
                        if (a_score >= b_score) && (score > b_score) {
                            b_score = score;
                            b_point = 6i8;
                        } else if (a_score < b_score) && (score > a_score) {
                            a_score = score;
                            a_point = 6i8;
                        }
                    }
                    let p1: f64 = (2.0f64 - in_sum) + xins;
                    if (a_score >= b_score) && (p1 > b_score) {
                        b_score = p1;
                        b_point = 1i8;
                        b_is_bigger_side = false;
                    } else if (a_score < b_score) && (p1 > a_score) {
                        a_score = p1;
                        a_point = 1i8;
                        a_is_bigger_side = false;
                    }
                    let p2: f64 = (2.0f64 - in_sum) + yins;
                    if (a_score >= b_score) && (p2 > b_score) {
                        b_score = p2;
                        b_point = 2i8;
                        b_is_bigger_side = false;
                    } else if (a_score < b_score) && (p2 > a_score) {
                        a_score = p2;
                        a_point = 2i8;
                        a_is_bigger_side = false;
                    }
                    let p3: f64 = (2.0f64 - in_sum) + zins;
                    if (a_score >= b_score) && (p3 > b_score) {
                        b_score = p3;
                        b_point = 4i8;
                        b_is_bigger_side = false;
                    } else if (a_score < b_score) && (p3 > a_score) {
                        a_score = p3;
                        a_point = 4i8;
                        a_is_bigger_side = false;
                    }
                    let p4: f64 = (2.0f64 - in_sum) + wins;
                    if (a_score >= b_score) && (p4 > b_score) {
                        b_point = 8i8;
                        b_is_bigger_side = false;
                    } else if (a_score < b_score) && (p4 > a_score) {
                        a_point = 8i8;
                        a_is_bigger_side = false;
                    }
                    if a_is_bigger_side == b_is_bigger_side {
                        if a_is_bigger_side {
                            let mut c1: i8 = (((a_point as i32) | (b_point as i32)) as i8);
                            let mut c2: i8 = (((a_point as i32) & (b_point as i32)) as i8);
                            if ((c1 as i32) & 1) == 0 {
                                xsv_ext0 = xsb;
                                xsv_ext1 = xsb - 1;
                                dx_ext0 = dx0 - 0.927050983124841f64;
                                dx_ext1 = (dx0 + 1.0f64) - 0.618033988749894f64;
                            } else {
                                let chain_40 = xsb + 1;
                                xsv_ext1 = chain_40;
                                xsv_ext0 = chain_40;
                                dx_ext0 = (dx0 - 1.0f64) - 0.927050983124841f64;
                                dx_ext1 = (dx0 - 1.0f64) - 0.618033988749894f64;
                            }
                            if ((c1 as i32) & 2) == 0 {
                                ysv_ext0 = ysb;
                                ysv_ext1 = ysb - 1;
                                dy_ext0 = dy0 - 0.927050983124841f64;
                                dy_ext1 = (dy0 + 1.0f64) - 0.618033988749894f64;
                            } else {
                                let chain_41 = ysb + 1;
                                ysv_ext1 = chain_41;
                                ysv_ext0 = chain_41;
                                dy_ext0 = (dy0 - 1.0f64) - 0.927050983124841f64;
                                dy_ext1 = (dy0 - 1.0f64) - 0.618033988749894f64;
                            }
                            if ((c1 as i32) & 4) == 0 {
                                zsv_ext0 = zsb;
                                zsv_ext1 = zsb - 1;
                                dz_ext0 = dz0 - 0.927050983124841f64;
                                dz_ext1 = (dz0 + 1.0f64) - 0.618033988749894f64;
                            } else {
                                let chain_42 = zsb + 1;
                                zsv_ext1 = chain_42;
                                zsv_ext0 = chain_42;
                                dz_ext0 = (dz0 - 1.0f64) - 0.927050983124841f64;
                                dz_ext1 = (dz0 - 1.0f64) - 0.618033988749894f64;
                            }
                            if ((c1 as i32) & 8) == 0 {
                                wsv_ext0 = wsb;
                                wsv_ext1 = wsb - 1;
                                dw_ext0 = dw0 - 0.927050983124841f64;
                                dw_ext1 = (dw0 + 1.0f64) - 0.618033988749894f64;
                            } else {
                                let chain_43 = wsb + 1;
                                wsv_ext1 = chain_43;
                                wsv_ext0 = chain_43;
                                dw_ext0 = (dw0 - 1.0f64) - 0.927050983124841f64;
                                dw_ext1 = (dw0 - 1.0f64) - 0.618033988749894f64;
                            }
                            xsv_ext2 = xsb;
                            ysv_ext2 = ysb;
                            zsv_ext2 = zsb;
                            wsv_ext2 = wsb;
                            dx_ext2 = dx0 - 0.618033988749894f64;
                            dy_ext2 = dy0 - 0.618033988749894f64;
                            dz_ext2 = dz0 - 0.618033988749894f64;
                            dw_ext2 = dw0 - 0.618033988749894f64;
                            if ((c2 as i32) & 1) != 0 {
                                xsv_ext2 += 2;
                                dx_ext2 -= 2.0f64;
                            } else if ((c2 as i32) & 2) != 0 {
                                ysv_ext2 += 2;
                                dy_ext2 -= 2.0f64;
                            } else if ((c2 as i32) & 4) != 0 {
                                zsv_ext2 += 2;
                                dz_ext2 -= 2.0f64;
                            } else {
                                wsv_ext2 += 2;
                                dw_ext2 -= 2.0f64;
                            }
                        } else {
                            xsv_ext2 = xsb;
                            ysv_ext2 = ysb;
                            zsv_ext2 = zsb;
                            wsv_ext2 = wsb;
                            dx_ext2 = dx0;
                            dy_ext2 = dy0;
                            dz_ext2 = dz0;
                            dw_ext2 = dw0;
                            let c: i8 = (((a_point as i32) | (b_point as i32)) as i8);
                            if ((c as i32) & 1) == 0 {
                                xsv_ext0 = xsb - 1;
                                xsv_ext1 = xsb;
                                dx_ext0 = (dx0 + 1.0f64) - 0.309016994374947f64;
                                dx_ext1 = dx0 - 0.309016994374947f64;
                            } else {
                                let chain_44 = xsb + 1;
                                xsv_ext1 = chain_44;
                                xsv_ext0 = chain_44;
                                let chain_45 = (dx0 - 1.0f64) - 0.309016994374947f64;
                                dx_ext1 = chain_45;
                                dx_ext0 = chain_45;
                            }
                            if ((c as i32) & 2) == 0 {
                                ysv_ext1 = ysb;
                                ysv_ext0 = ysb;
                                let chain_46 = dy0 - 0.309016994374947f64;
                                dy_ext1 = chain_46;
                                dy_ext0 = chain_46;
                                if ((c as i32) & 1) == 1 {
                                    ysv_ext0 -= 1;
                                    dy_ext0 += 1.0f64;
                                } else {
                                    ysv_ext1 -= 1;
                                    dy_ext1 += 1.0f64;
                                }
                            } else {
                                let chain_47 = ysb + 1;
                                ysv_ext1 = chain_47;
                                ysv_ext0 = chain_47;
                                let chain_48 = (dy0 - 1.0f64) - 0.309016994374947f64;
                                dy_ext1 = chain_48;
                                dy_ext0 = chain_48;
                            }
                            if ((c as i32) & 4) == 0 {
                                zsv_ext1 = zsb;
                                zsv_ext0 = zsb;
                                let chain_49 = dz0 - 0.309016994374947f64;
                                dz_ext1 = chain_49;
                                dz_ext0 = chain_49;
                                if ((c as i32) & 3) == 3 {
                                    zsv_ext0 -= 1;
                                    dz_ext0 += 1.0f64;
                                } else {
                                    zsv_ext1 -= 1;
                                    dz_ext1 += 1.0f64;
                                }
                            } else {
                                let chain_50 = zsb + 1;
                                zsv_ext1 = chain_50;
                                zsv_ext0 = chain_50;
                                let chain_51 = (dz0 - 1.0f64) - 0.309016994374947f64;
                                dz_ext1 = chain_51;
                                dz_ext0 = chain_51;
                            }
                            if ((c as i32) & 8) == 0 {
                                wsv_ext0 = wsb;
                                wsv_ext1 = wsb - 1;
                                dw_ext0 = dw0 - 0.309016994374947f64;
                                dw_ext1 = (dw0 + 1.0f64) - 0.309016994374947f64;
                            } else {
                                let chain_52 = wsb + 1;
                                wsv_ext1 = chain_52;
                                wsv_ext0 = chain_52;
                                let chain_53 = (dw0 - 1.0f64) - 0.309016994374947f64;
                                dw_ext1 = chain_53;
                                dw_ext0 = chain_53;
                            }
                        }
                    } else {
                        let mut c1: i8;
                        let mut c2: i8;
                        if a_is_bigger_side {
                            c1 = a_point;
                            c2 = b_point;
                        } else {
                            c1 = b_point;
                            c2 = a_point;
                        }
                        if ((c1 as i32) & 1) == 0 {
                            xsv_ext0 = xsb - 1;
                            xsv_ext1 = xsb;
                            dx_ext0 = (dx0 + 1.0f64) - 0.309016994374947f64;
                            dx_ext1 = dx0 - 0.309016994374947f64;
                        } else {
                            let chain_54 = xsb + 1;
                            xsv_ext1 = chain_54;
                            xsv_ext0 = chain_54;
                            let chain_55 = (dx0 - 1.0f64) - 0.309016994374947f64;
                            dx_ext1 = chain_55;
                            dx_ext0 = chain_55;
                        }
                        if ((c1 as i32) & 2) == 0 {
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            let chain_56 = dy0 - 0.309016994374947f64;
                            dy_ext1 = chain_56;
                            dy_ext0 = chain_56;
                            if ((c1 as i32) & 1) == 1 {
                                ysv_ext0 -= 1;
                                dy_ext0 += 1.0f64;
                            } else {
                                ysv_ext1 -= 1;
                                dy_ext1 += 1.0f64;
                            }
                        } else {
                            let chain_57 = ysb + 1;
                            ysv_ext1 = chain_57;
                            ysv_ext0 = chain_57;
                            let chain_58 = (dy0 - 1.0f64) - 0.309016994374947f64;
                            dy_ext1 = chain_58;
                            dy_ext0 = chain_58;
                        }
                        if ((c1 as i32) & 4) == 0 {
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            let chain_59 = dz0 - 0.309016994374947f64;
                            dz_ext1 = chain_59;
                            dz_ext0 = chain_59;
                            if ((c1 as i32) & 3) == 3 {
                                zsv_ext0 -= 1;
                                dz_ext0 += 1.0f64;
                            } else {
                                zsv_ext1 -= 1;
                                dz_ext1 += 1.0f64;
                            }
                        } else {
                            let chain_60 = zsb + 1;
                            zsv_ext1 = chain_60;
                            zsv_ext0 = chain_60;
                            let chain_61 = (dz0 - 1.0f64) - 0.309016994374947f64;
                            dz_ext1 = chain_61;
                            dz_ext0 = chain_61;
                        }
                        if ((c1 as i32) & 8) == 0 {
                            wsv_ext0 = wsb;
                            wsv_ext1 = wsb - 1;
                            dw_ext0 = dw0 - 0.309016994374947f64;
                            dw_ext1 = (dw0 + 1.0f64) - 0.309016994374947f64;
                        } else {
                            let chain_62 = wsb + 1;
                            wsv_ext1 = chain_62;
                            wsv_ext0 = chain_62;
                            let chain_63 = (dw0 - 1.0f64) - 0.309016994374947f64;
                            dw_ext1 = chain_63;
                            dw_ext0 = chain_63;
                        }
                        xsv_ext2 = xsb;
                        ysv_ext2 = ysb;
                        zsv_ext2 = zsb;
                        wsv_ext2 = wsb;
                        dx_ext2 = dx0 - 0.618033988749894f64;
                        dy_ext2 = dy0 - 0.618033988749894f64;
                        dz_ext2 = dz0 - 0.618033988749894f64;
                        dw_ext2 = dw0 - 0.618033988749894f64;
                        if ((c2 as i32) & 1) != 0 {
                            xsv_ext2 += 2;
                            dx_ext2 -= 2.0f64;
                        } else if ((c2 as i32) & 2) != 0 {
                            ysv_ext2 += 2;
                            dy_ext2 -= 2.0f64;
                        } else if ((c2 as i32) & 4) != 0 {
                            zsv_ext2 += 2;
                            dz_ext2 -= 2.0f64;
                        } else {
                            wsv_ext2 += 2;
                            dw_ext2 -= 2.0f64;
                        }
                    }
                    let dx1: f64 = (dx0 - 1.0f64) - 0.309016994374947f64;
                    let dy1: f64 = (dy0 - 0.0f64) - 0.309016994374947f64;
                    let dz1: f64 = (dz0 - 0.0f64) - 0.309016994374947f64;
                    let dw1: f64 = (dw0 - 0.0f64) - 0.309016994374947f64;
                    let mut attn1: f64 = (((2.0f64 - (dx1 * dx1)) - (dy1 * dy1)) - (dz1 * dz1)) - (dw1 * dw1);
                    if attn1 > 0.0f64 {
                        attn1 *= attn1;
                        value += (attn1 * attn1) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 0, wsb + 0, dx1, dy1, dz1, dw1));
                    }
                    let dx2: f64 = (dx0 - 0.0f64) - 0.309016994374947f64;
                    let dy2: f64 = (dy0 - 1.0f64) - 0.309016994374947f64;
                    let dz2: f64 = dz1;
                    let dw2: f64 = dw1;
                    let mut attn2: f64 = (((2.0f64 - (dx2 * dx2)) - (dy2 * dy2)) - (dz2 * dz2)) - (dw2 * dw2);
                    if attn2 > 0.0f64 {
                        attn2 *= attn2;
                        value += (attn2 * attn2) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 0, wsb + 0, dx2, dy2, dz2, dw2));
                    }
                    let dx3: f64 = dx2;
                    let dy3: f64 = dy1;
                    let dz3: f64 = (dz0 - 1.0f64) - 0.309016994374947f64;
                    let dw3: f64 = dw1;
                    let mut attn3: f64 = (((2.0f64 - (dx3 * dx3)) - (dy3 * dy3)) - (dz3 * dz3)) - (dw3 * dw3);
                    if attn3 > 0.0f64 {
                        attn3 *= attn3;
                        value += (attn3 * attn3) * (self.extrapolate_8(xsb + 0, ysb + 0, zsb + 1, wsb + 0, dx3, dy3, dz3, dw3));
                    }
                    let dx4: f64 = dx2;
                    let dy4: f64 = dy1;
                    let dz4: f64 = dz1;
                    let dw4: f64 = (dw0 - 1.0f64) - 0.309016994374947f64;
                    let mut attn4: f64 = (((2.0f64 - (dx4 * dx4)) - (dy4 * dy4)) - (dz4 * dz4)) - (dw4 * dw4);
                    if attn4 > 0.0f64 {
                        attn4 *= attn4;
                        value += (attn4 * attn4) * (self.extrapolate_8(xsb + 0, ysb + 0, zsb + 0, wsb + 1, dx4, dy4, dz4, dw4));
                    }
                    let dx5: f64 = (dx0 - 1.0f64) - 0.618033988749894f64;
                    let dy5: f64 = (dy0 - 1.0f64) - 0.618033988749894f64;
                    let dz5: f64 = (dz0 - 0.0f64) - 0.618033988749894f64;
                    let dw5: f64 = (dw0 - 0.0f64) - 0.618033988749894f64;
                    let mut attn5: f64 = (((2.0f64 - (dx5 * dx5)) - (dy5 * dy5)) - (dz5 * dz5)) - (dw5 * dw5);
                    if attn5 > 0.0f64 {
                        attn5 *= attn5;
                        value += (attn5 * attn5) * (self.extrapolate_8(xsb + 1, ysb + 1, zsb + 0, wsb + 0, dx5, dy5, dz5, dw5));
                    }
                    let dx6: f64 = (dx0 - 1.0f64) - 0.618033988749894f64;
                    let dy6: f64 = (dy0 - 0.0f64) - 0.618033988749894f64;
                    let dz6: f64 = (dz0 - 1.0f64) - 0.618033988749894f64;
                    let dw6: f64 = (dw0 - 0.0f64) - 0.618033988749894f64;
                    let mut attn6: f64 = (((2.0f64 - (dx6 * dx6)) - (dy6 * dy6)) - (dz6 * dz6)) - (dw6 * dw6);
                    if attn6 > 0.0f64 {
                        attn6 *= attn6;
                        value += (attn6 * attn6) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 1, wsb + 0, dx6, dy6, dz6, dw6));
                    }
                    let dx7: f64 = (dx0 - 1.0f64) - 0.618033988749894f64;
                    let dy7: f64 = (dy0 - 0.0f64) - 0.618033988749894f64;
                    let dz7: f64 = (dz0 - 0.0f64) - 0.618033988749894f64;
                    let dw7: f64 = (dw0 - 1.0f64) - 0.618033988749894f64;
                    let mut attn7: f64 = (((2.0f64 - (dx7 * dx7)) - (dy7 * dy7)) - (dz7 * dz7)) - (dw7 * dw7);
                    if attn7 > 0.0f64 {
                        attn7 *= attn7;
                        value += (attn7 * attn7) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 0, wsb + 1, dx7, dy7, dz7, dw7));
                    }
                    let dx8: f64 = (dx0 - 0.0f64) - 0.618033988749894f64;
                    let dy8: f64 = (dy0 - 1.0f64) - 0.618033988749894f64;
                    let dz8: f64 = (dz0 - 1.0f64) - 0.618033988749894f64;
                    let dw8: f64 = (dw0 - 0.0f64) - 0.618033988749894f64;
                    let mut attn8: f64 = (((2.0f64 - (dx8 * dx8)) - (dy8 * dy8)) - (dz8 * dz8)) - (dw8 * dw8);
                    if attn8 > 0.0f64 {
                        attn8 *= attn8;
                        value += (attn8 * attn8) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 1, wsb + 0, dx8, dy8, dz8, dw8));
                    }
                    let dx9: f64 = (dx0 - 0.0f64) - 0.618033988749894f64;
                    let dy9: f64 = (dy0 - 1.0f64) - 0.618033988749894f64;
                    let dz9: f64 = (dz0 - 0.0f64) - 0.618033988749894f64;
                    let dw9: f64 = (dw0 - 1.0f64) - 0.618033988749894f64;
                    let mut attn9: f64 = (((2.0f64 - (dx9 * dx9)) - (dy9 * dy9)) - (dz9 * dz9)) - (dw9 * dw9);
                    if attn9 > 0.0f64 {
                        attn9 *= attn9;
                        value += (attn9 * attn9) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 0, wsb + 1, dx9, dy9, dz9, dw9));
                    }
                    let dx10: f64 = (dx0 - 0.0f64) - 0.618033988749894f64;
                    let dy10: f64 = (dy0 - 0.0f64) - 0.618033988749894f64;
                    let dz10: f64 = (dz0 - 1.0f64) - 0.618033988749894f64;
                    let dw10: f64 = (dw0 - 1.0f64) - 0.618033988749894f64;
                    let mut attn10: f64 = (((2.0f64 - (dx10 * dx10)) - (dy10 * dy10)) - (dz10 * dz10)) - (dw10 * dw10);
                    if attn10 > 0.0f64 {
                        attn10 *= attn10;
                        value += (attn10 * attn10) * (self.extrapolate_8(xsb + 0, ysb + 0, zsb + 1, wsb + 1, dx10, dy10, dz10, dw10));
                    }
                } else {
                    let mut a_is_bigger_side: bool = true;
                    let mut b_is_bigger_side: bool = true;
                    let mut a_score: f64;
                    let mut a_point: i8;
                    if (xins + yins) < (zins + wins) {
                        a_score = xins + yins;
                        a_point = 12i8;
                    } else {
                        a_score = zins + wins;
                        a_point = 3i8;
                    }
                    let mut b_score: f64;
                    let mut b_point: i8;
                    if (xins + zins) < (yins + wins) {
                        b_score = xins + zins;
                        b_point = 10i8;
                    } else {
                        b_score = yins + wins;
                        b_point = 5i8;
                    }
                    if (xins + wins) < (yins + zins) {
                        let score: f64 = xins + wins;
                        if (a_score <= b_score) && (score < b_score) {
                            b_score = score;
                            b_point = 6i8;
                        } else if (a_score > b_score) && (score < a_score) {
                            a_score = score;
                            a_point = 6i8;
                        }
                    } else {
                        let score: f64 = yins + zins;
                        if (a_score <= b_score) && (score < b_score) {
                            b_score = score;
                            b_point = 9i8;
                        } else if (a_score > b_score) && (score < a_score) {
                            a_score = score;
                            a_point = 9i8;
                        }
                    }
                    let p1: f64 = (3.0f64 - in_sum) + xins;
                    if (a_score <= b_score) && (p1 < b_score) {
                        b_score = p1;
                        b_point = 14i8;
                        b_is_bigger_side = false;
                    } else if (a_score > b_score) && (p1 < a_score) {
                        a_score = p1;
                        a_point = 14i8;
                        a_is_bigger_side = false;
                    }
                    let p2: f64 = (3.0f64 - in_sum) + yins;
                    if (a_score <= b_score) && (p2 < b_score) {
                        b_score = p2;
                        b_point = 13i8;
                        b_is_bigger_side = false;
                    } else if (a_score > b_score) && (p2 < a_score) {
                        a_score = p2;
                        a_point = 13i8;
                        a_is_bigger_side = false;
                    }
                    let p3: f64 = (3.0f64 - in_sum) + zins;
                    if (a_score <= b_score) && (p3 < b_score) {
                        b_score = p3;
                        b_point = 11i8;
                        b_is_bigger_side = false;
                    } else if (a_score > b_score) && (p3 < a_score) {
                        a_score = p3;
                        a_point = 11i8;
                        a_is_bigger_side = false;
                    }
                    let p4: f64 = (3.0f64 - in_sum) + wins;
                    if (a_score <= b_score) && (p4 < b_score) {
                        b_point = 7i8;
                        b_is_bigger_side = false;
                    } else if (a_score > b_score) && (p4 < a_score) {
                        a_point = 7i8;
                        a_is_bigger_side = false;
                    }
                    if a_is_bigger_side == b_is_bigger_side {
                        if a_is_bigger_side {
                            let mut c1: i8 = (((a_point as i32) & (b_point as i32)) as i8);
                            let mut c2: i8 = (((a_point as i32) | (b_point as i32)) as i8);
                            xsv_ext1 = xsb;
                            xsv_ext0 = xsb;
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            wsv_ext1 = wsb;
                            wsv_ext0 = wsb;
                            dx_ext0 = dx0 - 0.309016994374947f64;
                            dy_ext0 = dy0 - 0.309016994374947f64;
                            dz_ext0 = dz0 - 0.309016994374947f64;
                            dw_ext0 = dw0 - 0.309016994374947f64;
                            dx_ext1 = dx0 - 0.618033988749894f64;
                            dy_ext1 = dy0 - 0.618033988749894f64;
                            dz_ext1 = dz0 - 0.618033988749894f64;
                            dw_ext1 = dw0 - 0.618033988749894f64;
                            if ((c1 as i32) & 1) != 0 {
                                xsv_ext0 += 1;
                                dx_ext0 -= 1.0f64;
                                xsv_ext1 += 2;
                                dx_ext1 -= 2.0f64;
                            } else if ((c1 as i32) & 2) != 0 {
                                ysv_ext0 += 1;
                                dy_ext0 -= 1.0f64;
                                ysv_ext1 += 2;
                                dy_ext1 -= 2.0f64;
                            } else if ((c1 as i32) & 4) != 0 {
                                zsv_ext0 += 1;
                                dz_ext0 -= 1.0f64;
                                zsv_ext1 += 2;
                                dz_ext1 -= 2.0f64;
                            } else {
                                wsv_ext0 += 1;
                                dw_ext0 -= 1.0f64;
                                wsv_ext1 += 2;
                                dw_ext1 -= 2.0f64;
                            }
                            xsv_ext2 = xsb + 1;
                            ysv_ext2 = ysb + 1;
                            zsv_ext2 = zsb + 1;
                            wsv_ext2 = wsb + 1;
                            dx_ext2 = (dx0 - 1.0f64) - 0.618033988749894f64;
                            dy_ext2 = (dy0 - 1.0f64) - 0.618033988749894f64;
                            dz_ext2 = (dz0 - 1.0f64) - 0.618033988749894f64;
                            dw_ext2 = (dw0 - 1.0f64) - 0.618033988749894f64;
                            if ((c2 as i32) & 1) == 0 {
                                xsv_ext2 -= 2;
                                dx_ext2 += 2.0f64;
                            } else if ((c2 as i32) & 2) == 0 {
                                ysv_ext2 -= 2;
                                dy_ext2 += 2.0f64;
                            } else if ((c2 as i32) & 4) == 0 {
                                zsv_ext2 -= 2;
                                dz_ext2 += 2.0f64;
                            } else {
                                wsv_ext2 -= 2;
                                dw_ext2 += 2.0f64;
                            }
                        } else {
                            xsv_ext2 = xsb + 1;
                            ysv_ext2 = ysb + 1;
                            zsv_ext2 = zsb + 1;
                            wsv_ext2 = wsb + 1;
                            dx_ext2 = (dx0 - 1.0f64) - 1.236067977499788f64;
                            dy_ext2 = (dy0 - 1.0f64) - 1.236067977499788f64;
                            dz_ext2 = (dz0 - 1.0f64) - 1.236067977499788f64;
                            dw_ext2 = (dw0 - 1.0f64) - 1.236067977499788f64;
                            let c: i8 = (((a_point as i32) & (b_point as i32)) as i8);
                            if ((c as i32) & 1) != 0 {
                                xsv_ext0 = xsb + 2;
                                xsv_ext1 = xsb + 1;
                                dx_ext0 = (dx0 - 2.0f64) - 0.927050983124841f64;
                                dx_ext1 = (dx0 - 1.0f64) - 0.927050983124841f64;
                            } else {
                                xsv_ext1 = xsb;
                                xsv_ext0 = xsb;
                                let chain_64 = dx0 - 0.927050983124841f64;
                                dx_ext1 = chain_64;
                                dx_ext0 = chain_64;
                            }
                            if ((c as i32) & 2) != 0 {
                                let chain_65 = ysb + 1;
                                ysv_ext1 = chain_65;
                                ysv_ext0 = chain_65;
                                let chain_66 = (dy0 - 1.0f64) - 0.927050983124841f64;
                                dy_ext1 = chain_66;
                                dy_ext0 = chain_66;
                                if ((c as i32) & 1) == 0 {
                                    ysv_ext0 += 1;
                                    dy_ext0 -= 1.0f64;
                                } else {
                                    ysv_ext1 += 1;
                                    dy_ext1 -= 1.0f64;
                                }
                            } else {
                                ysv_ext1 = ysb;
                                ysv_ext0 = ysb;
                                let chain_67 = dy0 - 0.927050983124841f64;
                                dy_ext1 = chain_67;
                                dy_ext0 = chain_67;
                            }
                            if ((c as i32) & 4) != 0 {
                                let chain_68 = zsb + 1;
                                zsv_ext1 = chain_68;
                                zsv_ext0 = chain_68;
                                let chain_69 = (dz0 - 1.0f64) - 0.927050983124841f64;
                                dz_ext1 = chain_69;
                                dz_ext0 = chain_69;
                                if ((c as i32) & 3) == 0 {
                                    zsv_ext0 += 1;
                                    dz_ext0 -= 1.0f64;
                                } else {
                                    zsv_ext1 += 1;
                                    dz_ext1 -= 1.0f64;
                                }
                            } else {
                                zsv_ext1 = zsb;
                                zsv_ext0 = zsb;
                                let chain_70 = dz0 - 0.927050983124841f64;
                                dz_ext1 = chain_70;
                                dz_ext0 = chain_70;
                            }
                            if ((c as i32) & 8) != 0 {
                                wsv_ext0 = wsb + 1;
                                wsv_ext1 = wsb + 2;
                                dw_ext0 = (dw0 - 1.0f64) - 0.927050983124841f64;
                                dw_ext1 = (dw0 - 2.0f64) - 0.927050983124841f64;
                            } else {
                                wsv_ext1 = wsb;
                                wsv_ext0 = wsb;
                                let chain_71 = dw0 - 0.927050983124841f64;
                                dw_ext1 = chain_71;
                                dw_ext0 = chain_71;
                            }
                        }
                    } else {
                        let mut c1: i8;
                        let mut c2: i8;
                        if a_is_bigger_side {
                            c1 = a_point;
                            c2 = b_point;
                        } else {
                            c1 = b_point;
                            c2 = a_point;
                        }
                        if ((c1 as i32) & 1) != 0 {
                            xsv_ext0 = xsb + 2;
                            xsv_ext1 = xsb + 1;
                            dx_ext0 = (dx0 - 2.0f64) - 0.927050983124841f64;
                            dx_ext1 = (dx0 - 1.0f64) - 0.927050983124841f64;
                        } else {
                            xsv_ext1 = xsb;
                            xsv_ext0 = xsb;
                            let chain_72 = dx0 - 0.927050983124841f64;
                            dx_ext1 = chain_72;
                            dx_ext0 = chain_72;
                        }
                        if ((c1 as i32) & 2) != 0 {
                            let chain_73 = ysb + 1;
                            ysv_ext1 = chain_73;
                            ysv_ext0 = chain_73;
                            let chain_74 = (dy0 - 1.0f64) - 0.927050983124841f64;
                            dy_ext1 = chain_74;
                            dy_ext0 = chain_74;
                            if ((c1 as i32) & 1) == 0 {
                                ysv_ext0 += 1;
                                dy_ext0 -= 1.0f64;
                            } else {
                                ysv_ext1 += 1;
                                dy_ext1 -= 1.0f64;
                            }
                        } else {
                            ysv_ext1 = ysb;
                            ysv_ext0 = ysb;
                            let chain_75 = dy0 - 0.927050983124841f64;
                            dy_ext1 = chain_75;
                            dy_ext0 = chain_75;
                        }
                        if ((c1 as i32) & 4) != 0 {
                            let chain_76 = zsb + 1;
                            zsv_ext1 = chain_76;
                            zsv_ext0 = chain_76;
                            let chain_77 = (dz0 - 1.0f64) - 0.927050983124841f64;
                            dz_ext1 = chain_77;
                            dz_ext0 = chain_77;
                            if ((c1 as i32) & 3) == 0 {
                                zsv_ext0 += 1;
                                dz_ext0 -= 1.0f64;
                            } else {
                                zsv_ext1 += 1;
                                dz_ext1 -= 1.0f64;
                            }
                        } else {
                            zsv_ext1 = zsb;
                            zsv_ext0 = zsb;
                            let chain_78 = dz0 - 0.927050983124841f64;
                            dz_ext1 = chain_78;
                            dz_ext0 = chain_78;
                        }
                        if ((c1 as i32) & 8) != 0 {
                            wsv_ext0 = wsb + 1;
                            wsv_ext1 = wsb + 2;
                            dw_ext0 = (dw0 - 1.0f64) - 0.927050983124841f64;
                            dw_ext1 = (dw0 - 2.0f64) - 0.927050983124841f64;
                        } else {
                            wsv_ext1 = wsb;
                            wsv_ext0 = wsb;
                            let chain_79 = dw0 - 0.927050983124841f64;
                            dw_ext1 = chain_79;
                            dw_ext0 = chain_79;
                        }
                        xsv_ext2 = xsb + 1;
                        ysv_ext2 = ysb + 1;
                        zsv_ext2 = zsb + 1;
                        wsv_ext2 = wsb + 1;
                        dx_ext2 = (dx0 - 1.0f64) - 0.618033988749894f64;
                        dy_ext2 = (dy0 - 1.0f64) - 0.618033988749894f64;
                        dz_ext2 = (dz0 - 1.0f64) - 0.618033988749894f64;
                        dw_ext2 = (dw0 - 1.0f64) - 0.618033988749894f64;
                        if ((c2 as i32) & 1) == 0 {
                            xsv_ext2 -= 2;
                            dx_ext2 += 2.0f64;
                        } else if ((c2 as i32) & 2) == 0 {
                            ysv_ext2 -= 2;
                            dy_ext2 += 2.0f64;
                        } else if ((c2 as i32) & 4) == 0 {
                            zsv_ext2 -= 2;
                            dz_ext2 += 2.0f64;
                        } else {
                            wsv_ext2 -= 2;
                            dw_ext2 += 2.0f64;
                        }
                    }
                    let dx4: f64 = (dx0 - 1.0f64) - 0.927050983124841f64;
                    let dy4: f64 = (dy0 - 1.0f64) - 0.927050983124841f64;
                    let dz4: f64 = (dz0 - 1.0f64) - 0.927050983124841f64;
                    let dw4: f64 = dw0 - 0.927050983124841f64;
                    let mut attn4: f64 = (((2.0f64 - (dx4 * dx4)) - (dy4 * dy4)) - (dz4 * dz4)) - (dw4 * dw4);
                    if attn4 > 0.0f64 {
                        attn4 *= attn4;
                        value += (attn4 * attn4) * (self.extrapolate_8(xsb + 1, ysb + 1, zsb + 1, wsb + 0, dx4, dy4, dz4, dw4));
                    }
                    let dx3: f64 = dx4;
                    let dy3: f64 = dy4;
                    let dz3: f64 = dz0 - 0.927050983124841f64;
                    let dw3: f64 = (dw0 - 1.0f64) - 0.927050983124841f64;
                    let mut attn3: f64 = (((2.0f64 - (dx3 * dx3)) - (dy3 * dy3)) - (dz3 * dz3)) - (dw3 * dw3);
                    if attn3 > 0.0f64 {
                        attn3 *= attn3;
                        value += (attn3 * attn3) * (self.extrapolate_8(xsb + 1, ysb + 1, zsb + 0, wsb + 1, dx3, dy3, dz3, dw3));
                    }
                    let dx2: f64 = dx4;
                    let dy2: f64 = dy0 - 0.927050983124841f64;
                    let dz2: f64 = dz4;
                    let dw2: f64 = dw3;
                    let mut attn2: f64 = (((2.0f64 - (dx2 * dx2)) - (dy2 * dy2)) - (dz2 * dz2)) - (dw2 * dw2);
                    if attn2 > 0.0f64 {
                        attn2 *= attn2;
                        value += (attn2 * attn2) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 1, wsb + 1, dx2, dy2, dz2, dw2));
                    }
                    let dx1: f64 = dx0 - 0.927050983124841f64;
                    let dz1: f64 = dz4;
                    let dy1: f64 = dy4;
                    let dw1: f64 = dw3;
                    let mut attn1: f64 = (((2.0f64 - (dx1 * dx1)) - (dy1 * dy1)) - (dz1 * dz1)) - (dw1 * dw1);
                    if attn1 > 0.0f64 {
                        attn1 *= attn1;
                        value += (attn1 * attn1) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 1, wsb + 1, dx1, dy1, dz1, dw1));
                    }
                    let dx5: f64 = (dx0 - 1.0f64) - 0.618033988749894f64;
                    let dy5: f64 = (dy0 - 1.0f64) - 0.618033988749894f64;
                    let dz5: f64 = (dz0 - 0.0f64) - 0.618033988749894f64;
                    let dw5: f64 = (dw0 - 0.0f64) - 0.618033988749894f64;
                    let mut attn5: f64 = (((2.0f64 - (dx5 * dx5)) - (dy5 * dy5)) - (dz5 * dz5)) - (dw5 * dw5);
                    if attn5 > 0.0f64 {
                        attn5 *= attn5;
                        value += (attn5 * attn5) * (self.extrapolate_8(xsb + 1, ysb + 1, zsb + 0, wsb + 0, dx5, dy5, dz5, dw5));
                    }
                    let dx6: f64 = (dx0 - 1.0f64) - 0.618033988749894f64;
                    let dy6: f64 = (dy0 - 0.0f64) - 0.618033988749894f64;
                    let dz6: f64 = (dz0 - 1.0f64) - 0.618033988749894f64;
                    let dw6: f64 = (dw0 - 0.0f64) - 0.618033988749894f64;
                    let mut attn6: f64 = (((2.0f64 - (dx6 * dx6)) - (dy6 * dy6)) - (dz6 * dz6)) - (dw6 * dw6);
                    if attn6 > 0.0f64 {
                        attn6 *= attn6;
                        value += (attn6 * attn6) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 1, wsb + 0, dx6, dy6, dz6, dw6));
                    }
                    let dx7: f64 = (dx0 - 1.0f64) - 0.618033988749894f64;
                    let dy7: f64 = (dy0 - 0.0f64) - 0.618033988749894f64;
                    let dz7: f64 = (dz0 - 0.0f64) - 0.618033988749894f64;
                    let dw7: f64 = (dw0 - 1.0f64) - 0.618033988749894f64;
                    let mut attn7: f64 = (((2.0f64 - (dx7 * dx7)) - (dy7 * dy7)) - (dz7 * dz7)) - (dw7 * dw7);
                    if attn7 > 0.0f64 {
                        attn7 *= attn7;
                        value += (attn7 * attn7) * (self.extrapolate_8(xsb + 1, ysb + 0, zsb + 0, wsb + 1, dx7, dy7, dz7, dw7));
                    }
                    let dx8: f64 = (dx0 - 0.0f64) - 0.618033988749894f64;
                    let dy8: f64 = (dy0 - 1.0f64) - 0.618033988749894f64;
                    let dz8: f64 = (dz0 - 1.0f64) - 0.618033988749894f64;
                    let dw8: f64 = (dw0 - 0.0f64) - 0.618033988749894f64;
                    let mut attn8: f64 = (((2.0f64 - (dx8 * dx8)) - (dy8 * dy8)) - (dz8 * dz8)) - (dw8 * dw8);
                    if attn8 > 0.0f64 {
                        attn8 *= attn8;
                        value += (attn8 * attn8) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 1, wsb + 0, dx8, dy8, dz8, dw8));
                    }
                    let dx9: f64 = (dx0 - 0.0f64) - 0.618033988749894f64;
                    let dy9: f64 = (dy0 - 1.0f64) - 0.618033988749894f64;
                    let dz9: f64 = (dz0 - 0.0f64) - 0.618033988749894f64;
                    let dw9: f64 = (dw0 - 1.0f64) - 0.618033988749894f64;
                    let mut attn9: f64 = (((2.0f64 - (dx9 * dx9)) - (dy9 * dy9)) - (dz9 * dz9)) - (dw9 * dw9);
                    if attn9 > 0.0f64 {
                        attn9 *= attn9;
                        value += (attn9 * attn9) * (self.extrapolate_8(xsb + 0, ysb + 1, zsb + 0, wsb + 1, dx9, dy9, dz9, dw9));
                    }
                    let dx10: f64 = (dx0 - 0.0f64) - 0.618033988749894f64;
                    let dy10: f64 = (dy0 - 0.0f64) - 0.618033988749894f64;
                    let dz10: f64 = (dz0 - 1.0f64) - 0.618033988749894f64;
                    let dw10: f64 = (dw0 - 1.0f64) - 0.618033988749894f64;
                    let mut attn10: f64 = (((2.0f64 - (dx10 * dx10)) - (dy10 * dy10)) - (dz10 * dz10)) - (dw10 * dw10);
                    if attn10 > 0.0f64 {
                        attn10 *= attn10;
                        value += (attn10 * attn10) * (self.extrapolate_8(xsb + 0, ysb + 0, zsb + 1, wsb + 1, dx10, dy10, dz10, dw10));
                    }
                }
                let mut attn_ext0: f64 = (((2.0f64 - (dx_ext0 * dx_ext0)) - (dy_ext0 * dy_ext0)) - (dz_ext0 * dz_ext0)) - (dw_ext0 * dw_ext0);
                if attn_ext0 > 0.0f64 {
                    attn_ext0 *= attn_ext0;
                    value += (attn_ext0 * attn_ext0) * (self.extrapolate_8(xsv_ext0, ysv_ext0, zsv_ext0, wsv_ext0, dx_ext0, dy_ext0, dz_ext0, dw_ext0));
                }
                let mut attn_ext1: f64 = (((2.0f64 - (dx_ext1 * dx_ext1)) - (dy_ext1 * dy_ext1)) - (dz_ext1 * dz_ext1)) - (dw_ext1 * dw_ext1);
                if attn_ext1 > 0.0f64 {
                    attn_ext1 *= attn_ext1;
                    value += (attn_ext1 * attn_ext1) * (self.extrapolate_8(xsv_ext1, ysv_ext1, zsv_ext1, wsv_ext1, dx_ext1, dy_ext1, dz_ext1, dw_ext1));
                }
                let mut attn_ext2: f64 = (((2.0f64 - (dx_ext2 * dx_ext2)) - (dy_ext2 * dy_ext2)) - (dz_ext2 * dz_ext2)) - (dw_ext2 * dw_ext2);
                if attn_ext2 > 0.0f64 {
                    attn_ext2 *= attn_ext2;
                    value += (attn_ext2 * attn_ext2) * (self.extrapolate_8(xsv_ext2, ysv_ext2, zsv_ext2, wsv_ext2, dx_ext2, dy_ext2, dz_ext2, dw_ext2));
                }
                return value / 30.0f64;
            }
            pub fn extrapolate_4(&self, xsb: i32, ysb: i32, dx: f64, dy: f64) -> f64 {
                let index: i32 = (((self.perm.as_ref().expect("null"))[(((((self.perm.as_ref().expect("null"))[(xsb & 0xFF) as usize]) as i32) + ysb) & 0xFF) as usize]) as i32) & 14;
                return ((((*open_simplex_noise::GRADIENTS2_D)[index as usize]) as f64) * dx) + ((((*open_simplex_noise::GRADIENTS2_D)[(index + 1) as usize]) as f64) * dy);
            }
            pub fn extrapolate_6(&self, xsb: i32, ysb: i32, zsb: i32, dx: f64, dy: f64, dz: f64) -> f64 {
                let index: i32 = (((self.perm_grad_index3_d.as_ref().expect("null"))[(((((self.perm.as_ref().expect("null"))[(((((self.perm.as_ref().expect("null"))[(xsb & 0xFF) as usize]) as i32) + ysb) & 0xFF) as usize]) as i32) + zsb) & 0xFF) as usize]) as i32);
                return (((((*open_simplex_noise::GRADIENTS3_D)[index as usize]) as f64) * dx) + ((((*open_simplex_noise::GRADIENTS3_D)[(index + 1) as usize]) as f64) * dy)) + ((((*open_simplex_noise::GRADIENTS3_D)[(index + 2) as usize]) as f64) * dz);
            }
            pub fn extrapolate_8(&self, xsb: i32, ysb: i32, zsb: i32, wsb: i32, dx: f64, dy: f64, dz: f64, dw: f64) -> f64 {
                let index: i32 = (((self.perm.as_ref().expect("null"))[(((((self.perm.as_ref().expect("null"))[(((((self.perm.as_ref().expect("null"))[(((((self.perm.as_ref().expect("null"))[(xsb & 0xFF) as usize]) as i32) + ysb) & 0xFF) as usize]) as i32) + zsb) & 0xFF) as usize]) as i32) + wsb) & 0xFF) as usize]) as i32) & 252;
                return ((((((*open_simplex_noise::GRADIENTS4_D)[index as usize]) as f64) * dx) + ((((*open_simplex_noise::GRADIENTS4_D)[(index + 1) as usize]) as f64) * dy)) + ((((*open_simplex_noise::GRADIENTS4_D)[(index + 2) as usize]) as f64) * dz)) + ((((*open_simplex_noise::GRADIENTS4_D)[(index + 3) as usize]) as f64) * dw);
            }
        }
        pub fn fast_floor(x: f64) -> i32 {
            let xi: i32 = (x as i32);
            return if x < (xi as f64) { xi - 1 } else { xi };
        }
    }

    /// `BNDripHangParticle` (org/betterx/betternether/particles/BNDripHangParticle.java)
    pub mod bn_drip_hang_particle {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `DripHangParticle`. Its static parts are below.
        // BNDripHangParticle.BNDripHangParticle(7): not translated, its class isn't translated (it extends Minecraft's `DripHangParticle`).
        // BNDripHangParticle.setup(3): not translated, its class isn't translated (it extends Minecraft's `DripHangParticle`).
    }

    /// `BNDripLandParticle` (org/betterx/betternether/particles/BNDripLandParticle.java)
    pub mod bn_drip_land_particle {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `DripLandParticle`. Its static parts are below.
        // BNDripLandParticle.BNDripLandParticle(6): not translated, its class isn't translated (it extends Minecraft's `DripLandParticle`).
        // BNDripLandParticle.setup(2): not translated, its class isn't translated (it extends Minecraft's `DripLandParticle`).
    }

    /// `BNFallAndLandParticle` (org/betterx/betternether/particles/BNFallAndLandParticle.java)
    pub mod bn_fall_and_land_particle {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `FallAndLandParticle`. Its static parts are below.
        // BNFallAndLandParticle.BNFallAndLandParticle(7): not translated, its class isn't translated (it extends Minecraft's `FallAndLandParticle`).
        // BNFallAndLandParticle.setup(2): not translated, its class isn't translated (it extends Minecraft's `FallAndLandParticle`).
    }

    /// `BNParticleProvider` (org/betterx/betternether/particles/BNParticleProvider.java)
    pub mod bn_particle_provider {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        pub const BLUE_DRIP_R: f32 = 0.07058824f32;
        pub const BLUE_DRIP_G: f32 = 0.3529412f32;
        pub const BLUE_DRIP_B: f32 = 0.9764706f32;
        pub const DRIP_R: f32 = 0.51171875f32;
        pub const DRIP_G: f32 = 0.03125f32;
        pub const DRIP_B: f32 = 0.890625f32;
    }

    /// `BNParticleProvider.ObsidianTearFallProvider` (org/betterx/betternether/particles/BNParticleProvider.java)
    pub mod bn_particle_provider_obsidian_tear_fall_provider {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // BNParticleProvider.ObsidianTearFallProvider.ObsidianTearFallProvider(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // BNParticleProvider.ObsidianTearFallProvider.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `BNParticleProvider.ObsidianTearHangProvider` (org/betterx/betternether/particles/BNParticleProvider.java)
    pub mod bn_particle_provider_obsidian_tear_hang_provider {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // BNParticleProvider.ObsidianTearHangProvider.ObsidianTearHangProvider(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // BNParticleProvider.ObsidianTearHangProvider.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `BNParticleProvider.ObsidianTearLandProvider` (org/betterx/betternether/particles/BNParticleProvider.java)
    pub mod bn_particle_provider_obsidian_tear_land_provider {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // BNParticleProvider.ObsidianTearLandProvider.ObsidianTearLandProvider(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // BNParticleProvider.ObsidianTearLandProvider.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `BNParticleProvider.ObsidianVanillaWeepFallProvider` (org/betterx/betternether/particles/BNParticleProvider.java)
    pub mod bn_particle_provider_obsidian_vanilla_weep_fall_provider {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // BNParticleProvider.ObsidianVanillaWeepFallProvider.ObsidianVanillaWeepFallProvider(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // BNParticleProvider.ObsidianVanillaWeepFallProvider.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `BNParticleProvider.ObsidianVanillaWeepHangProvider` (org/betterx/betternether/particles/BNParticleProvider.java)
    pub mod bn_particle_provider_obsidian_vanilla_weep_hang_provider {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // BNParticleProvider.ObsidianVanillaWeepHangProvider.ObsidianVanillaWeepHangProvider(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // BNParticleProvider.ObsidianVanillaWeepHangProvider.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `BNParticleProvider.ObsidianVanillaWeepLandProvider` (org/betterx/betternether/particles/BNParticleProvider.java)
    pub mod bn_particle_provider_obsidian_vanilla_weep_land_provider {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // BNParticleProvider.ObsidianVanillaWeepLandProvider.ObsidianVanillaWeepLandProvider(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // BNParticleProvider.ObsidianVanillaWeepLandProvider.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `BNParticleProvider.ObsidianWeepFallProvider` (org/betterx/betternether/particles/BNParticleProvider.java)
    pub mod bn_particle_provider_obsidian_weep_fall_provider {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // BNParticleProvider.ObsidianWeepFallProvider.ObsidianWeepFallProvider(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // BNParticleProvider.ObsidianWeepFallProvider.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `BNParticleProvider.ObsidianWeepHangProvider` (org/betterx/betternether/particles/BNParticleProvider.java)
    pub mod bn_particle_provider_obsidian_weep_hang_provider {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // BNParticleProvider.ObsidianWeepHangProvider.ObsidianWeepHangProvider(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // BNParticleProvider.ObsidianWeepHangProvider.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `BNParticleProvider.ObsidianWeepLandProvider` (org/betterx/betternether/particles/BNParticleProvider.java)
    pub mod bn_particle_provider_obsidian_weep_land_provider {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ParticleProvider`. Its static parts are below.
        // BNParticleProvider.ObsidianWeepLandProvider.ObsidianWeepLandProvider(1): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
        // BNParticleProvider.ObsidianWeepLandProvider.createParticle(9): not translated, its class isn't translated (it implements Minecraft's `ParticleProvider`).
    }

    /// `Patcher` (org/betterx/betternether/Patcher.java)
    pub mod patcher {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Patcher {
        }
        impl Patcher {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Patcher.register(0): not translated, a method reference.
    }

    /// `Patcher_001` (org/betterx/betternether/Patcher_001.java)
    pub mod patcher_001 {
        use super::*;
        // The class itself isn't translated: it extends `Patch` (org.betterx.bclib.api.v2.datafixer). Its static parts are below.
        pub static LEAVE_IDS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["betternether:willow_leaves".to_string(), "betternether:rubeus_leaves".to_string(), "betternether:anchor_tree_leaves".to_string(), "betternether:nether_sakura_leaves".to_string()]);
        // Patcher_001.Patcher_001(0): not translated, its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)).
        // Patcher_001.getIDReplacements(0): not translated, its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)).
        // Patcher_001.getChunkPatcher(0): not translated, its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)).
        // Patcher_001.patchChunk(2): not translated, it uses Minecraft's `CompoundTag`.
    }

    /// `Patcher_002` (org/betterx/betternether/Patcher_002.java)
    pub mod patcher_002 {
        use super::*;
        // The class itself isn't translated: it extends `Patch` (org.betterx.bclib.api.v2.datafixer). Its static parts are below.
        // Patcher_002.Patcher_002(0): not translated, its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)).
        // Patcher_002.getIDReplacements(0): not translated, its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)).
    }

    /// `Patcher_003` (org/betterx/betternether/Patcher_003.java)
    pub mod patcher_003 {
        use super::*;
        // The class itself isn't translated: it extends `Patch` (org.betterx.bclib.api.v2.datafixer). Its static parts are below.
        // Patcher_003.Patcher_003(0): not translated, its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)).
        // Patcher_003.getIDReplacements(0): not translated, its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)).
    }

    /// `Patcher_004` (org/betterx/betternether/Patcher_004.java)
    pub mod patcher_004 {
        use super::*;
        // The class itself isn't translated: it extends `Patch` (org.betterx.bclib.api.v2.datafixer). Its static parts are below.
        // Patcher_004.Patcher_004(0): not translated, its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)).
        // Patcher_004.getIDReplacements(0): not translated, its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)).
        // Patcher_004.getBlockStatePatcher(0): not translated, its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)).
        // Patcher_004.patchBlockState(3): not translated, it uses Minecraft's `ListTag`.
    }

    /// `Patcher_005` (org/betterx/betternether/Patcher_005.java)
    pub mod patcher_005 {
        use super::*;
        // The class itself isn't translated: it extends `Patch` (org.betterx.bclib.api.v2.datafixer). Its static parts are below.
        // Patcher_005.Patcher_005(0): not translated, its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)).
        // Patcher_005.getChunkPatcher(0): not translated, its class isn't translated (it extends `Patch` (org.betterx.bclib.api.v2.datafixer)).
    }

    /// `BNPortalShape` (org/betterx/betternether/portals/BNPortalShape.java)
    pub mod bn_portal_shape {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockGetter`. Its static parts are below.
        // DIR_X: not translated, it uses Minecraft's `Direction`.
        // DIR_Z: not translated, it uses Minecraft's `Direction`.
        pub const MAX_SEARCH_DIST: i32 = 21;
        // stack: not translated, it uses Minecraft's `BlockPos`.
        // BNPortalShape.BNPortalShape(3): not translated, its class isn't translated (it uses Minecraft's `BlockGetter`).
        // BNPortalShape.getPortalBlocks(2): not translated, its class isn't translated (it uses Minecraft's `BlockGetter`).
        // BNPortalShape.calculateBottomLeft(1): not translated, its class isn't translated (it uses Minecraft's `BlockGetter`).
        // BNPortalShape.calculateWidth(0): not translated, its class isn't translated (it uses Minecraft's `BlockGetter`).
        // BNPortalShape.calculateHeight(0): not translated, its class isn't translated (it uses Minecraft's `BlockGetter`).
        // BNPortalShape.isFrame(1): not translated, it uses Minecraft's `BlockState`.
        // BNPortalShape.isEmpty(1): not translated, it uses Minecraft's `BlockState`.
        // BNPortalShape.isValid(0): not translated, its class isn't translated (it uses Minecraft's `BlockGetter`).
        // BNPortalShape.createPortalBlocks(1): not translated, its class isn't translated (it uses Minecraft's `BlockGetter`).
        // BNPortalShape.isComplete(0): not translated, its class isn't translated (it uses Minecraft's `BlockGetter`).
    }

    /// `BNPortalShapeProvider` (org/betterx/betternether/portals/BNPortalShapeProvider.java)
    pub mod bn_portal_shape_provider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BNPortalShapeProvider.bn_setShape(1): not translated, it has no body.
        // BNPortalShapeProvider.bn_getShape(0): not translated, it has no body.
    }

    /// `RecipesHelper` (org/betterx/betternether/recipes/RecipesHelper.java)
    pub mod recipes_helper {
        use super::*;
        pub static SHAPE_ROOF: LazyLock<Vec<String>> = LazyLock::new(|| vec!["# #".to_string(), "###".to_string(), " # ".to_string()]);
        pub static SHAPE_STAIR: LazyLock<Vec<String>> = LazyLock::new(|| vec!["#  ".to_string(), "## ".to_string(), "###".to_string()]);
        pub static SHAPE_SLAB: LazyLock<Vec<String>> = LazyLock::new(|| vec!["###".to_string()]);
        pub static SHAPE_BUTTON: LazyLock<Vec<String>> = LazyLock::new(|| vec!["#".to_string()]);
        pub static SHAPE_PLATE: LazyLock<Vec<String>> = LazyLock::new(|| vec!["##".to_string()]);
        pub static SHAPE_X2: LazyLock<Vec<String>> = LazyLock::new(|| vec!["##".to_string(), "##".to_string()]);
        pub static SHAPE_3X2: LazyLock<Vec<String>> = LazyLock::new(|| vec!["###".to_string(), "###".to_string()]);
        pub static SHAPE_COLORING: LazyLock<Vec<String>> = LazyLock::new(|| vec!["###".to_string(), "#I#".to_string(), "###".to_string()]);
        pub static SHAPE_ROUND: LazyLock<Vec<String>> = LazyLock::new(|| vec!["###".to_string(), "# #".to_string(), "###".to_string()]);
        pub static SHAPE_FIRE_BOWL: LazyLock<Vec<String>> = LazyLock::new(|| vec!["#I#".to_string(), " # ".to_string(), "L L".to_string()]);
        // RECIPES: not translated, it uses RecipeBuilder, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct RecipesHelper {
        }
        impl RecipesHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RecipesHelper.provideRecipes(1): not translated, it uses `Templates` (de.ambertation.wover.recipe.api.RecipeBuilder).
        // RecipesHelper.addProvider(1): not translated, it uses RecipeBuilder, which isn't translated (it's an interface).
        // RecipesHelper.makeRoofRecipe(2): not translated, it uses Minecraft's `Block`.
        // RecipesHelper.makeStairsRecipe(2): not translated, it uses Minecraft's `Block`.
        // RecipesHelper.makeSlabRecipe(2): not translated, it uses Minecraft's `Block`.
        // RecipesHelper.makeButtonRecipe(2): not translated, it uses Minecraft's `Block`.
        // RecipesHelper.makePlateRecipe(2): not translated, it uses Minecraft's `Block`.
        // RecipesHelper.makeSimpleRecipe2(5): not translated, it uses Minecraft's `Block`.
        // RecipesHelper.makeWallRecipe(2): not translated, it uses Minecraft's `Block`.
        // RecipesHelper.makeColoringRecipe(5): not translated, it uses Minecraft's `Block`.
        // RecipesHelper.makeRoundRecipe(4): not translated, it uses Minecraft's `Block`.
        // RecipesHelper.makeFireBowlRecipe(4): not translated, it uses Minecraft's `Block`.
    }

    /// `RecipesHelper.RecipeBuilder` (org/betterx/betternether/recipes/RecipesHelper.java)
    pub mod recipes_helper_recipe_builder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `BlockEntitiesRegistry` (org/betterx/betternether/registry/BlockEntitiesRegistry.java)
    pub mod block_entities_registry {
        use super::*;
        // CINCINNASITE_FORGE: not translated, it uses Minecraft's `BlockEntityType`.
        // NETHERRACK_FURNACE: not translated, it uses Minecraft's `BlockEntityType`.
        // CHEST_OF_DRAWERS: not translated, it uses Minecraft's `BlockEntityType`.
        // NETHER_BREWING_STAND: not translated, it uses Minecraft's `BlockEntityType`.
        #[derive(Clone, Debug)]
        pub struct BlockEntitiesRegistry {
        }
        impl BlockEntitiesRegistry {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BlockEntitiesRegistry.register(0): not translated, it uses Minecraft's `BlockEntityType`.
        // BlockEntitiesRegistry.RegisterBlockEntity(2): not translated, it uses Minecraft's `BlockEntityType`.
        // BlockEntitiesRegistry.getFurnaces(0): not translated, it uses Minecraft's `Block`.
    }

    /// `NetherBlockItemFactories` (org/betterx/betternether/registry/block/NetherBlockItemFactories.java)
    pub mod nether_block_item_factories {
        use super::*;
        // STACKS_TO_16: not translated, it uses `BlockItemFactory` (de.ambertation.wover.block.api.BlockDefinition).
        #[derive(Clone, Debug)]
        pub struct NetherBlockItemFactories {
        }
        impl NetherBlockItemFactories {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherCropBlocks` (org/betterx/betternether/registry/block/NetherCropBlocks.java)
    pub mod nether_crop_blocks {
        use super::*;
        // BLACK_APPLE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLACK_APPLE_SEED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // WHISPERING_GOURD: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherCropBlocks {
        }
        impl NetherCropBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherDecorBlocks` (org/betterx/betternether/registry/block/NetherDecorBlocks.java)
    pub mod nether_decor_blocks {
        use super::*;
        // CINCINNASITE_FRAME: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_POT: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BRICK_POT: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GEYSER: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // ROOF_TILE_NETHER_BRICKS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // ROOF_TILE_NETHER_BRICKS_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // ROOF_TILE_NETHER_BRICKS_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // ROOF_TILE_CINCINNASITE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // ROOF_TILE_CINCINNASITE_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // ROOF_TILE_CINCINNASITE_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherDecorBlocks {
        }
        impl NetherDecorBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherFactories` (org/betterx/betternether/registry/block/NetherFactories.java)
    pub mod nether_factories {
        use super::*;
        // AGAVE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // ANCHOR_TREE_SAPLING: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // ANCHOR_TREE_VINE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BARREL_CACTUS: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BLACK_APPLE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BLACK_APPLE_SEED: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BLACK_BUSH: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BLACK_VINE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BLUE_CRYING_OBSIDIAN: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BLUE_WEEPING_OBSIDIAN: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BN_BREWING_STAND: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BN_PANE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BN_PILLAR: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BN_POT_METAL: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BN_POT_STONE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BONE_MUSHROOM: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BROWN_LARGE_MUSHROOM: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CHEST_OF_DRAWERS: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_ANVIL: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_FORGE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_FRAME: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_PEDESTAL: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CINCINNASIT_PILLAR: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // EGG_PLANT: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // EYEBALL: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // EYEBALL_SMALL: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // EYE_SEED: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // EYE_VINE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // FARMLAND: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // FEATHER_FERN: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GEYSER: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GIANT_LUCIS: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GIANT_MOLD: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GIANT_MOLD_SAPLING: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GLOOMSCULK_CRYSTAL: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GLOOMSCULK_GEODE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GLOOMWISP_VINE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GLOOMWOOD_SAPLING: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GOLDEN_VINE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GRAY_MOLD: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // HOOK_MUSHROOM: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // INK_BUSH: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // INK_BUSH_SEED: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // JELLYFISH_MUSHROOM: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // JELLYFISH_MUSHROOM_SAPLING: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // LUCIS_MUSHROOM: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // LUCIS_SPORE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // LUMABUS_VINE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GOLDEN_LUMABUS_VINE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // MAGMA_FLOWER: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // MOLTEN_GLOOMSCULK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // MOLTEN_GLOOMWOOD_LOG: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // MOSS_COVER: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // MUSHROOM_FIR: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // MUSHROOM_FIR_SAPLING: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NEON_EQUISETUM: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NETHER_CACTUS: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NETHER_FURNACE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NETHER_GRASS_BONE_GRASS: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NETHER_GRASS_JUNGLE_PLANT: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NETHER_GRASS_NETHER_GRASS: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NETHER_GRASS_SEPIA_BONE_GRASS: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NETHER_GRASS_SWAMP_GRASS: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NETHER_MYCELIUM: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NETHER_REED: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NETHER_SAKURA_SAPLING: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // OBSIDIAN_GLASS: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // ORANGE_MUSHROOM: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // POTTED_PLANT: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // REDSTONE_ORE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // RED_LARGE_MUSHROOM: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // RED_MOLD: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // REEDS: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // RUBEUS_BARK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // RUBEUS_CONE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // RUBEUS_LOG: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // RUBEUS_SAPLING: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // SMALL_LANTERN_METAL: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // SMOKER: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // SOUL_GRASS: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // SOUL_LILY: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // SOUL_LILY_SAPLING: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // SOUL_SANDSTONE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // SOUL_VEIN: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // STALACTITE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // STALAGNATE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // STALAGNATE_BOWL: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // STALAGNATE_SEED: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // STATUE_RESPAWNER: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // TERRAIN: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // VANILLA_WEEPING_OBSIDIAN: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // VEINED_SAND: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // WART_ROOTS: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // WART_SEED: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // WHISPERING_GOURD_LANTERN: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // WHISPERING_GOURD_VINE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // WILLOW_BRANCH: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // WILLOW_SAPLING: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // WILLOW_TORCH: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // WILLOW_TRUNK: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BN_LEAVES: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // WILLOW_LEAVES: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // RUBEUS_LEAVES: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NETHER_SAKURA_LEAVES: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BN_OBSIDIAN: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BN_OBSIDIAN_BLUE_CRYING: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // WALL_PLANT_SHAPE_2_6: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_ORE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NETHER_RUBY_ORE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // NETHER_LAPIS_ORE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CRIMSON_BUTTON: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // IRON_BUTTON: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // BONE_PLATE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_PLATE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // CRIMSON_WOODLIKE_DOOR: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // FIRE_BOWL_METAL: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // FIRE_BOWL_STONE: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // LUMABUS_SEED: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        // GOLDEN_LUMABUS_SEED: not translated, it uses `BlockFactory` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherFactories {
        }
        impl NetherFactories {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherFunctionalBlocks` (org/betterx/betternether/registry/block/NetherFunctionalBlocks.java)
    pub mod nether_functional_blocks {
        use super::*;
        // CINCINNASITE_PEDESTAL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // PIG_STATUE_RESPAWNER: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLACKSTONE_FURNACE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BASALT_FURNACE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHERRACK_FURNACE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_FORGE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_BREWING_STAND: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_ANVIL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLACKSTONE_FURNACE_STONE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // BASALT_FURNACE_STONE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // NETHERRACK_FURNACE_STONE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherFunctionalBlocks {
        }
        impl NetherFunctionalBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherFunctionalBlocks.workstationStone(2): not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        pub fn ensure_loaded() {
        }
    }

    /// `NetherFurnitureBlocks` (org/betterx/betternether/registry/block/NetherFurnitureBlocks.java)
    pub mod nether_furniture_blocks {
        use super::*;
        // TRIMMED_CHEST_MATERIAL: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // TABURET_CINCINNASITE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CHAIR_CINCINNASITE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BAR_STOOL_CINCINNASITE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CHEST_OF_DRAWERS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // TRIMMED_MUSHROOM_FIR_CHEST: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherFurnitureBlocks {
        }
        impl NetherFurnitureBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherGlassBlocks` (org/betterx/betternether/registry/block/NetherGlassBlocks.java)
    pub mod nether_glass_blocks {
        use super::*;
        // QUARTZ_GLASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // QUARTZ_GLASS_FRAMED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // QUARTZ_GLASS_FRAMED_COLORED: not translated, it uses ColoredGlassMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // QUARTZ_GLASS_PANE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // QUARTZ_GLASS_PANE_COLORED: not translated, it uses ColoredGlassMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // QUARTZ_GLASS_FRAMED_PANE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // QUARTZ_GLASS_FRAMED_PANE_COLORED: not translated, it uses ColoredGlassMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        // QUARTZ_GLASS_COLORED: not translated, it uses ColoredGlassMaterial, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
        #[derive(Clone, Debug)]
        pub struct NetherGlassBlocks {
        }
        impl NetherGlassBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherKinds` (org/betterx/betternether/registry/block/NetherKinds.java)
    pub mod nether_kinds {
        use super::*;
        // OWN_MODEL: not translated, a raw Supplier.
        // STEM: not translated, it uses StemKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // TRUNK: not translated, it uses TrunkKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // POTTED_PLANT: not translated, it uses PottedPlantKind, which isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        #[derive(Clone, Debug)]
        pub struct NetherKinds {
        }
        impl NetherKinds {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherKinds.model(3): not translated, it uses `ResolveContext` (de.ambertation.wover.block.api.classify).
    }

    /// `NetherKinds.PottedPlantKind` (org/betterx/betternether/registry/block/NetherKinds.java)
    pub mod nether_kinds_potted_plant_kind {
        use super::*;
        // The class itself isn't translated: it extends `Kind` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // NetherKinds.PottedPlantKind.PottedPlantKind(0): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // NetherKinds.PottedPlantKind.PottedPlantKind(3): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // NetherKinds.PottedPlantKind.resolve(1): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
    }

    /// `NetherKinds.StemKind` (org/betterx/betternether/registry/block/NetherKinds.java)
    pub mod nether_kinds_stem_kind {
        use super::*;
        // The class itself isn't translated: it extends `Kind` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // NetherKinds.StemKind.StemKind(0): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // NetherKinds.StemKind.StemKind(6): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // NetherKinds.StemKind.resolve(1): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
    }

    /// `NetherKinds.TrunkKind` (org/betterx/betternether/registry/block/NetherKinds.java)
    pub mod nether_kinds_trunk_kind {
        use super::*;
        // The class itself isn't translated: it extends `Kind` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // NetherKinds.TrunkKind.TrunkKind(0): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // NetherKinds.TrunkKind.TrunkKind(5): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // NetherKinds.TrunkKind.hasItem(0): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
        // NetherKinds.TrunkKind.resolve(1): not translated, its class isn't translated (it extends `Kind` (de.ambertation.wover.block.api.classify)).
    }

    /// `NetherLeavesBlocks` (org/betterx/betternether/registry/block/NetherLeavesBlocks.java)
    pub mod nether_leaves_blocks {
        use super::*;
        // WILLOW_LEAVES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // RUBEUS_LEAVES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // ANCHOR_TREE_LEAVES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_SAKURA_LEAVES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOOMWOOD_LEAVES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOOMWOOD_BLEACHED_LEAVES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherLeavesBlocks {
        }
        impl NetherLeavesBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherLightBlocks` (org/betterx/betternether/registry/block/NetherLightBlocks.java)
    pub mod nether_light_blocks {
        use super::*;
        // NETHERITE_FIRE_BOWL_MATERIAL: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_LANTERN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_LANTERN_SMALL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // WHISPERING_GOURD_LANTERN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_FIRE_BOWL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BRICKS_FIRE_BOWL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHERITE_FIRE_BOWL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_FIRE_BOWL_SOUL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BRICKS_FIRE_BOWL_SOUL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHERITE_FIRE_BOWL_SOUL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherLightBlocks {
        }
        impl NetherLightBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherMaterials` (org/betterx/betternether/registry/block/NetherMaterials.java)
    pub mod nether_materials {
        use super::*;
        // WOOD: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // PLANT: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // GRASS: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // GRAY_GRASS: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // CROP_PLANT: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // HOE_PLANT: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // SEED: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // CACTUS: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // MUSHROOM: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // VINE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // STURDY_VINE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // MUSHROOM_STEM: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // GLOOMWOOD_TRANSITION: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // GLOOMWOOD_TRANSITION_LOG: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // VANILLA_WOOD: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // VANILLA_NETHER_WOOD: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_OPEN: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // NETHER_TERRAIN: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // NETHER_TERRAIN_COVER: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // NETHER_NYLIUM: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // PORTAL_OBSIDIAN: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // NETHER_ORE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // QUARTZ_GLASS: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // FRAMED_QUARTZ_GLASS: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // FRAMED_QUARTZ_GLASS_PANE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherMaterials {
        }
        impl NetherMaterials {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherMetalBlocks` (org/betterx/betternether/registry/block/NetherMetalBlocks.java)
    pub mod nether_metal_blocks {
        use super::*;
        // NETHER_RUBY: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_BLOCK: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_FORGED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_PILLAR: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_BRICKS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_BRICK_PLATE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_BUTTON: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_PLATE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_TILE_LARGE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_TILE_SMALL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_CARVED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_WALL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_BRICKS_PILLAR: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_BARS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_CHAIN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_RUBY_BLOCK: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_RUBY_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_RUBY_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherMetalBlocks {
        }
        impl NetherMetalBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherMushroomBlocks` (org/betterx/betternether/registry/block/NetherMushroomBlocks.java)
    pub mod nether_mushroom_blocks {
        use super::*;
        // ORANGE_MUSHROOM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // RED_MOLD: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GRAY_MOLD: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LUCIS_SPORE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GIANT_LUCIS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BONE_MUSHROOM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SMOKER: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // HOOK_MUSHROOM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // RED_LARGE_MUSHROOM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BROWN_LARGE_MUSHROOM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LUCIS_MUSHROOM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GIANT_MOLD: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // JELLYFISH_MUSHROOM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherMushroomBlocks {
        }
        impl NetherMushroomBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherObsidianBlocks` (org/betterx/betternether/registry/block/NetherObsidianBlocks.java)
    pub mod nether_obsidian_blocks {
        use super::*;
        // IMMOVEABLE: not translated, it uses `Flag` (de.ambertation.wover.block.api.classify).
        // OBSIDIAN_GLASS_MATERIAL: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // OBSIDIAN_GLASS_PANE_MATERIAL: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // BLUE_WEEPING_OBSIDIAN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // WEEPING_OBSIDIAN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLUE_CRYING_OBSIDIAN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // OBSIDIAN_BRICKS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // OBSIDIAN_BRICKS_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // OBSIDIAN_BRICKS_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // OBSIDIAN_TILE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // OBSIDIAN_TILE_SMALL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // OBSIDIAN_TILE_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // OBSIDIAN_TILE_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // OBSIDIAN_ROD_TILES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // OBSIDIAN_GLASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // OBSIDIAN_GLASS_PANE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLUE_OBSIDIAN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLUE_OBSIDIAN_BRICKS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLUE_OBSIDIAN_BRICKS_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLUE_OBSIDIAN_BRICKS_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLUE_OBSIDIAN_TILE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLUE_OBSIDIAN_TILE_SMALL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLUE_OBSIDIAN_TILE_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLUE_OBSIDIAN_TILE_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLUE_OBSIDIAN_ROD_TILES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLUE_OBSIDIAN_GLASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLUE_OBSIDIAN_GLASS_PANE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherObsidianBlocks {
        }
        impl NetherObsidianBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherOreBlocks` (org/betterx/betternether/registry/block/NetherOreBlocks.java)
    pub mod nether_ore_blocks {
        use super::*;
        // IRON_ORE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // DIAMOND_ORE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // CINCINNASITE_ORE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_RUBY_ORE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_LAPIS_ORE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_REDSTONE_ORE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherOreBlocks {
        }
        impl NetherOreBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherPlantBlocks` (org/betterx/betternether/registry/block/NetherPlantBlocks.java)
    pub mod nether_plant_blocks {
        use super::*;
        // EYE_SEED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SWAMP_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SOUL_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // JUNGLE_PLANT: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BONE_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // PALE_GLOOMGRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOOMGRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SEPIA_BONE_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLACK_BUSH: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // INK_BUSH: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // INK_BUSH_SEED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // EGG_PLANT: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // MAGMA_FLOWER: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // FEATHER_FERN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // MOSS_COVER: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NEON_EQUISETUM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // AGAVE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BARREL_CACTUS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_CACTUS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // JUNGLE_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SOUL_LILY: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // EYEBALL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // EYEBALL_SMALL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // POTTED_PLANT: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherPlantBlocks {
        }
        impl NetherPlantBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherRecipes` (org/betterx/betternether/registry/block/NetherRecipes.java)
    pub mod nether_recipes {
        use super::*;
        // CATEGORY: not translated, it uses Minecraft's `Codec`.
        // STALACTITE: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // SIMPLE_2X2: not translated, it uses `Template4` (de.ambertation.wover.core.api.template).
        // FURNACE: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // ROOF_TILE: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // FIRE_BOWL: not translated, it uses `Template3` (de.ambertation.wover.core.api.template).
        // STEM_TO_LOG: not translated, it uses `Template1` (de.ambertation.wover.core.api.template).
        // MOLTEN_GLOOMSCULK: not translated, it uses `Template0` (de.ambertation.wover.core.api.template).
        // GLOOMSCULK_LAMP: not translated, it uses `Template2` (de.ambertation.wover.core.api.template).
        // MOLTEN_GLOOMWOOD_LOG: not translated, it uses `Template2` (de.ambertation.wover.core.api.template).
        // TRANSITION_CONVERSION: not translated, it uses `Template2` (de.ambertation.wover.core.api.template).
        // TRIMMED_CHEST: not translated, it uses `Template2` (de.ambertation.wover.core.api.template).
        // STEMS_TO_BLOCK: not translated, it uses `Template4` (de.ambertation.wover.core.api.template).
        // ROOF: not translated, it uses `Template4` (de.ambertation.wover.core.api.template).
        #[derive(Clone, Debug)]
        pub struct NetherRecipes {
        }
        impl NetherRecipes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherRecipes.deferred(1): not translated, it uses `RecipeMaterial` (de.ambertation.wover.recipe.api).
        pub fn ensure_loaded() {
        }
        // NetherRecipes.stalactite(1): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherRecipes.simple2x2(4): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherRecipes.furnace(1): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherRecipes.roofTile(1): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherRecipes.fireBowl(3): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherRecipes.stemToLog(1): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherRecipes.moltenGloomsculk(0): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherRecipes.gloomsculkLamp(2): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherRecipes.moltenGloomwoodLog(2): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherRecipes.transitionConversion(2): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherRecipes.trimmedChest(2): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherRecipes.stemsToBlock(4): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherRecipes.roof(4): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
    }

    /// `NetherRecipes.FireBowl` (org/betterx/betternether/registry/block/NetherRecipes.java)
    pub mod nether_recipes_fire_bowl {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherRecipes.FireBowl.FireBowl(3): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.FireBowl.ring(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.FireBowl.inside(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.FireBowl.leg(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherRecipes.Furnace` (org/betterx/betternether/registry/block/NetherRecipes.java)
    pub mod nether_recipes_furnace {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherRecipes.Furnace.Furnace(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.Furnace.source(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherRecipes.GloomsculkLamp` (org/betterx/betternether/registry/block/NetherRecipes.java)
    pub mod nether_recipes_gloomsculk_lamp {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherRecipes.GloomsculkLamp.GloomsculkLamp(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.GloomsculkLamp.geode(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.GloomsculkLamp.crystal(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherRecipes.MoltenGloomwoodLog` (org/betterx/betternether/registry/block/NetherRecipes.java)
    pub mod nether_recipes_molten_gloomwood_log {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherRecipes.MoltenGloomwoodLog.MoltenGloomwoodLog(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.MoltenGloomwoodLog.planks(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.MoltenGloomwoodLog.log(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherRecipes.Roof` (org/betterx/betternether/registry/block/NetherRecipes.java)
    pub mod nether_recipes_roof {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherRecipes.Roof.Roof(4): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.Roof.ingredient(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.Roof.count(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.Roof.group(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.Roof.shape(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherRecipes.RoofTile` (org/betterx/betternether/registry/block/NetherRecipes.java)
    pub mod nether_recipes_roof_tile {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherRecipes.RoofTile.RoofTile(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.RoofTile.source(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherRecipes.Simple2x2` (org/betterx/betternether/registry/block/NetherRecipes.java)
    pub mod nether_recipes_simple2x2 {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherRecipes.Simple2x2.Simple2x2(4): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.Simple2x2.source(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.Simple2x2.count(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.Simple2x2.group(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.Simple2x2.category(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherRecipes.Stalactite` (org/betterx/betternether/registry/block/NetherRecipes.java)
    pub mod nether_recipes_stalactite {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherRecipes.Stalactite.Stalactite(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.Stalactite.source(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherRecipes.StemToLog` (org/betterx/betternether/registry/block/NetherRecipes.java)
    pub mod nether_recipes_stem_to_log {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherRecipes.StemToLog.StemToLog(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.StemToLog.log(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherRecipes.StemsToBlock` (org/betterx/betternether/registry/block/NetherRecipes.java)
    pub mod nether_recipes_stems_to_block {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherRecipes.StemsToBlock.StemsToBlock(4): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.StemsToBlock.stem(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.StemsToBlock.count(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.StemsToBlock.shaped(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.StemsToBlock.group(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherRecipes.TransitionConversion` (org/betterx/betternether/registry/block/NetherRecipes.java)
    pub mod nether_recipes_transition_conversion {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherRecipes.TransitionConversion.TransitionConversion(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.TransitionConversion.light(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.TransitionConversion.dark(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherRecipes.TrimmedChest` (org/betterx/betternether/registry/block/NetherRecipes.java)
    pub mod nether_recipes_trimmed_chest {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherRecipes.TrimmedChest.TrimmedChest(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.TrimmedChest.chest(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // NetherRecipes.TrimmedChest.strippedLog(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `NetherSaplingBlocks` (org/betterx/betternether/registry/block/NetherSaplingBlocks.java)
    pub mod nether_sapling_blocks {
        use super::*;
        // GIANT_MOLD_SAPLING: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // JELLYFISH_MUSHROOM_SAPLING: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SOUL_LILY_SAPLING: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherSaplingBlocks {
        }
        impl NetherSaplingBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherStoneBlocks` (org/betterx/betternether/registry/block/NetherStoneBlocks.java)
    pub mod nether_stone_blocks {
        use super::*;
        // NETHER_BRICK_TILE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // NETHER_BRICK_OPEN: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // BONE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // BONE_WOOD: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // SOUL_SANDSTONE_SHAPE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // SOUL_SANDSTONE_STONE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // BASALT: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // NETHERRACK: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // BLACKSTONE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // BLACKSTONE_STALACTITE_STONE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // GLOWSTONE_STALACTITE_STONE: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // NETHER_BRICK_TILE_LARGE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_BRICK_TILE_SMALL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_BRICK_WALL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_BRICK_TILE_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_BRICK_TILE_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BONE_BLOCK: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BONE_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BONE_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BONE_BUTTON: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BONE_PLATE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BONE_WALL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BONE_TILE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BONE_REED_DOOR: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BONE_CINCINNASITE_DOOR: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SOUL_SANDSTONE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SOUL_SANDSTONE_CUT: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SOUL_SANDSTONE_CUT_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SOUL_SANDSTONE_CUT_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SOUL_SANDSTONE_WALL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SOUL_SANDSTONE_SMOOTH: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SOUL_SANDSTONE_CHISELED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SOUL_SANDSTONE_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SOUL_SANDSTONE_SMOOTH_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SOUL_SANDSTONE_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SOUL_SANDSTONE_SMOOTH_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BASALT_BRICKS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BASALT_BRICKS_STAIRS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BASALT_BRICKS_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BASALT_BRICKS_WALL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BASALT_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHERRACK_STALACTITE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOWSTONE_STALACTITE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLACKSTONE_STALACTITE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BASALT_STALACTITE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BONE_STALACTITE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHERRACK_SLAB: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHERRACK_STAIR: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHERRACK_WALLS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherStoneBlocks {
        }
        impl NetherStoneBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherStoneBlocks.stone(2): not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        pub fn ensure_loaded() {
        }
    }

    /// `NetherSurvivalRules` (org/betterx/betternether/registry/block/NetherSurvivalRules.java)
    pub mod nether_survival_rules {
        use super::*;
        // GLIMMERWILD_GROUND: not translated, it uses Minecraft's `BlockPredicate`.
        // NETHERRACK: not translated, it uses Minecraft's `BlockPredicate`.
        // NETHER_MYCELIUM: not translated, it uses Minecraft's `BlockPredicate`.
        // SOUL_SAND: not translated, it uses Minecraft's `BlockPredicate`.
        // NETHER_GROUND: not translated, it uses Minecraft's `BlockPredicate`.
        // NETHER_GROUND_AND_SCULK: not translated, it uses Minecraft's `BlockPredicate`.
        // SCULK_LIKE: not translated, it uses Minecraft's `BlockPredicate`.
        // NETHERRACK_NYLIUM_AND_SCULK: not translated, it uses Minecraft's `BlockPredicate`.
        // NYLIUM: not translated, it uses Minecraft's `BlockPredicate`.
        // NETHER_SAND: not translated, it uses Minecraft's `BlockPredicate`.
        // SOUL_GROUND: not translated, it uses Minecraft's `BlockPredicate`.
        // SOUL_GROUND_OR_FARMLAND: not translated, it uses Minecraft's `BlockPredicate`.
        // SOIL_OR_LOGS: not translated, it uses Minecraft's `BlockPredicate`.
        // GRAVEL: not translated, it uses Minecraft's `BlockPredicate`.
        // MAGMA_SAND_OR_SCULK: not translated, it uses Minecraft's `BlockPredicate`.
        #[derive(Clone, Debug)]
        pub struct NetherSurvivalRules {
        }
        impl NetherSurvivalRules {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherSurvivalRules.orGlimmerwild(1): not translated, it uses Minecraft's `BlockPredicate`.
        // NetherSurvivalRules.boneBlocks(0): not translated, it uses Minecraft's `BlockPredicate`.
    }

    /// `NetherTerrainBlocks` (org/betterx/betternether/registry/block/NetherTerrainBlocks.java)
    pub mod nether_terrain_blocks {
        use super::*;
        // SCULK_ROCK: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // HOT_SCULK_ROCK: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // NETHERRACK_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_MYCELIUM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // JUNGLE_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // MUSHROOM_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SEPIA_MUSHROOM_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SWAMPLAND_GRASS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // FARMLAND: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // CEILING_MUSHROOMS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLEACHED_GLOOMSCULK: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // MOLTEN_GLOOMSCULK: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // VEINED_GLOOMSCULK: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOOMSCULK_GEODE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOOMSCULK_GEODE_CRYSTAL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOOMSCULK_LAMP: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOOMSCULK_CRYSTAL: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // VEINED_SAND: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // FARMLAND_MATERIAL: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        // VEINED_SAND_MATERIAL: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherTerrainBlocks {
        }
        impl NetherTerrainBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherVineBlocks` (org/betterx/betternether/registry/block/NetherVineBlocks.java)
    pub mod nether_vine_blocks {
        use super::*;
        // BLACK_VINE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // BLOOMING_VINE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GOLDEN_VINE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LUMABUS_VINE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GOLDEN_LUMABUS_VINE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOOMSCULK_VINE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOOMWISP_VINE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // SOUL_VEIN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // WHISPERING_GOURD_VINE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // ANCHOR_TREE_VINE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // EYE_VINE: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // LUMABUS_SEED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GOLDEN_LUMABUS_SEED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherVineBlocks {
        }
        impl NetherVineBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherWallPlantBlocks` (org/betterx/betternether/registry/block/NetherWallPlantBlocks.java)
    pub mod nether_wall_plant_blocks {
        use super::*;
        // WALL_MOSS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // WALL_MUSHROOM_BROWN: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // WALL_MUSHROOM_RED: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherWallPlantBlocks {
        }
        impl NetherWallPlantBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherWoodBlocks` (org/betterx/betternether/registry/block/NetherWoodBlocks.java)
    pub mod nether_wood_blocks {
        use super::*;
        // NETHER_REED_STEM: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // MAT_REED: not translated, it uses NetherReedMaterial, which isn't translated (it extends `RoofMaterial`).
        // MAT_STALAGNATE: not translated, it uses StalagnateMaterial, which isn't translated (it extends `RoofMaterial`).
        // MAT_WILLOW: not translated, it uses WillowMaterial, which isn't translated (it extends `RoofMaterial`).
        // MAT_WART: not translated, it uses WartMaterial, which isn't translated (it extends `RoofMaterial`).
        // WARPED_WOOD: not translated, it uses VanillaNetherWood, which isn't translated (it extends `VanillaFallback`).
        // CRIMSON_WOOD: not translated, it uses VanillaNetherWood, which isn't translated (it extends `VanillaFallback`).
        // OAK_WOOD: not translated, it uses VanillaWood, which isn't translated (it extends `VanillaFallback`).
        // SPRUCE_WOOD: not translated, it uses VanillaWood, which isn't translated (it extends `VanillaFallback`).
        // BIRCH_WOOD: not translated, it uses VanillaWood, which isn't translated (it extends `VanillaFallback`).
        // JUNGLE_WOOD: not translated, it uses VanillaWood, which isn't translated (it extends `VanillaFallback`).
        // ACACIA_WOOD: not translated, it uses VanillaWood, which isn't translated (it extends `VanillaFallback`).
        // DARK_OAK_WOOD: not translated, it uses VanillaWood, which isn't translated (it extends `VanillaFallback`).
        // CHERRY_WOOD: not translated, it uses VanillaWood, which isn't translated (it extends `VanillaFallback`).
        // BAMBOO_WOOD: not translated, it uses VanillaWood, which isn't translated (it extends `VanillaFallback`).
        // MANGROVE_WOOD: not translated, it uses VanillaWood, which isn't translated (it extends `VanillaFallback`).
        // MAT_RUBEUS: not translated, it uses RubeusMaterial, which isn't translated (it extends `NetherWoodenMaterial`).
        // MAT_MUSHROOM_FIR: not translated, it uses MushroomFirMaterial, which isn't translated (it extends `NetherWoodenMaterial`).
        // MAT_NETHER_MUSHROOM: not translated, it uses NetherMushroomMaterial, which isn't translated (it extends `NetherWoodenMaterial`).
        // MAT_ANCHOR_TREE: not translated, it uses AnchorTreeMaterial, which isn't translated (it extends `NetherWoodenMaterial`).
        // MAT_NETHER_SAKURA: not translated, it uses NetherSakuraMaterial, which isn't translated (it extends `NetherWoodenMaterial`).
        // MAT_GLOOMWOOD: not translated, it uses GloomwoodMaterial, which isn't translated (it extends `NetherWoodenMaterial`).
        // MAT_GLOOMWOOD_DARK: not translated, it uses GloomwoodDarkMaterial, which isn't translated (it extends `NetherWoodenMaterial`).
        // GLOOMWOOD_TRANSITION_STRIPPED_LOG: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOOMWOOD_TRANSITION_LOG: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOOMWOOD_DARK_MOLTEN_LOG: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // GLOOMWOOD_TRANSITION_PLANKS: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        // NETHER_REED_STEM_MATERIAL: not translated, it uses `MaterialInstance` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherWoodBlocks {
        }
        impl NetherWoodBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `EntityRenderRegistry` (org/betterx/betternether/registry/EntityRenderRegistry.java)
    pub mod entity_render_registry {
        use super::*;
        pub const DEFAULT_LAYER: &str = "main";
        // FIREFLY_MODEL: not translated, it uses Minecraft's `ModelLayerLocation`.
        // NAGA_MODEL: not translated, it uses Minecraft's `ModelLayerLocation`.
        // JUNGLE_SKELETON_MODEL: not translated, it uses Minecraft's `ModelLayerLocation`.
        // FLYING_PIG_MODEL: not translated, it uses Minecraft's `ModelLayerLocation`.
        // HYDROGEN_JELLYFISH_MODEL: not translated, it uses Minecraft's `ModelLayerLocation`.
        // SKULL_MODEL: not translated, it uses Minecraft's `ModelLayerLocation`.
        #[derive(Clone, Debug)]
        pub struct EntityRenderRegistry {
        }
        impl EntityRenderRegistry {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EntityRenderRegistry.registerMain(1): not translated, it uses Minecraft's `ModelLayerLocation`.
        // EntityRenderRegistry.register(0): not translated, it uses Minecraft's `EntityType`.
        // EntityRenderRegistry.registerRenderMob(2): not translated, it uses Minecraft's `EntityType`.
    }

    /// `NetherObjects` (org/betterx/betternether/registry/features/configured/NetherObjects.java)
    pub mod nether_objects {
        use super::*;
        // C: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // PATCH_BASALT_STALACTITE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_BASALT_STALAGMITE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_SMOKER: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        #[derive(Clone, Debug)]
        pub struct NetherObjects {
        }
        impl NetherObjects {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `NetherOres` (org/betterx/betternether/registry/features/configured/NetherOres.java)
    pub mod nether_ores {
        use super::*;
        // C: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // CINCINNASITE_ORE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // GLOOMWOOD_CEILING_DEBRIS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // NETHER_RUBY_ORE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // NETHER_RUBY_ORE_SOUL: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // NETHER_RUBY_ORE_LARGE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // NETHER_RUBY_ORE_RARE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // NETHER_LAPIS_ORE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // NETHER_REDSTONE_ORE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        #[derive(Clone, Debug)]
        pub struct NetherOres {
        }
        impl NetherOres {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `NetherTerrain` (org/betterx/betternether/registry/features/configured/NetherTerrain.java)
    pub mod nether_terrain {
        use super::*;
        // C: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // LAVA_PITS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // SCULK_VEIN: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        #[derive(Clone, Debug)]
        pub struct NetherTerrain {
        }
        impl NetherTerrain {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherTerrain.sculkVein(1): not translated, it uses Minecraft's `MultifaceGrowthFeature`.
    }

    /// `NetherTrees` (org/betterx/betternether/registry/features/configured/NetherTrees.java)
    pub mod nether_trees {
        use super::*;
        // C: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // RUBEUS_TREE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // GLOOMWOOD_TREE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // GLOOMWOOD_TREE_SOLITARY: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // MUSHROOM_FIR: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // STALAGNATE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // STALAGNATE_DOWN: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // GIANT_MOLD: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_BIG_RED_MUSHROOM: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_BIG_BROWN_MUSHROOM: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // WART_TREE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // WILLOW_TREE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // OLD_WILLOW_TREE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // ANCHOR_TREE_BRANCH: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // ANCHOR_TREE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // ANCHOR_TREE_ROOT: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // SAKURA_TREE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        #[derive(Clone, Debug)]
        pub struct NetherTrees {
        }
        impl NetherTrees {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `NetherVegetation` (org/betterx/betternether/registry/features/configured/NetherVegetation.java)
    pub mod nether_vegetation {
        use super::*;
        // C: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // BONEMEAL_NETHERRACK_MOSS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_NETHER_MYCELIUM: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_JUNGLE_GRASS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_MUSHROOM_GRASS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_SEPIA_MUSHROOM_GRASS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_SWAMPLAND_GRASS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_CEILING_MUSHROOMS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // VEGETATION_MUSHROOM_FORREST: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // JELLYFISH_MUSHROOM: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_JELLYFISH_MUSHROOM: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_BLACK_BUSH: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // WALL_LUCIS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // BONEMEAL_SOUL_SOIL: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // VEGETATION_GLOOMWOOD_BLEACHED: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // VEGETATION_GLOOMWOOD_SCULK: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // VEGETATION_MAGMA_LAND: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // GLOOMWISP_VINE_HEAD: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // GLOOMWISP_VINE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // VEGETATION_GRASSLANDS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // VEGETATION_GRAVEL_DESERT: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // VEGETATION_JUNGLE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // VEGETATION_POOR_GRASSLANDS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // VEGETATION_SOUL_PLAIN: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // VEGETATION_WART_FOREST: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // VEGETATION_WART_FOREST_EDGE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // VEGETATION_SWAMPLAND: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // VEGETATION_OLD_SWAMPLAND: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // VEGETATION_OLD_WARPED_WOODS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // NETHER_CACTUS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // WALL_MUSHROOM_RED_WITH_MOSS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // WALL_MUSHROOMS_WITH_MOSS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // WALL_MUSHROOMS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // WALL_JUNGLE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // WALL_UPSIDE_DOWN: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // NETHER_REED: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // WART_BUSH: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // WILLOW_BUSH: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // RUBEUS_BUSH: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // SAKURA_BUSH: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // SCULK_VEGETATION: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // HOOK_MUSHROOM: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // MOSS_COVER: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        #[derive(Clone, Debug)]
        pub struct NetherVegetation {
        }
        impl NetherVegetation {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherVegetation.setupBonemealFeatures(0): not translated, it uses NetherTerrainBlocks.NETHERRACK_MOSS, which isn't translated (it uses `BlockKey` (de.ambertation.wover.block.api.classify)).
    }

    /// `NetherVines` (org/betterx/betternether/registry/features/configured/NetherVines.java)
    pub mod nether_vines {
        use super::*;
        // C: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // LUMABUS_VINE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // GOLDEN_LUMABUS_VINE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // EYE_VINE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_LUMABUS_VINE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_LUMABUS_VINE_GLOOM: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_GOLDEN_LUMABUS_VINE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_GOLDEN_VINE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_GOLDEN_VINE_SPARSE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_EYE_VINE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_GLOOMSCULK_VINE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_BLACK_VINE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_TWISTING_VINES: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_BLOOMING_VINE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // NEON_EQUISETUM: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_NEON_EQUISETUM: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // WHISPERING_GOURD_VINE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PATCH_WHISPERING_GOURD_VINE: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        #[derive(Clone, Debug)]
        pub struct NetherVines {
        }
        impl NetherVines {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `NetherGlimmerwildFeatures` (org/betterx/betternether/registry/features/NetherGlimmerwildFeatures.java)
    pub mod nether_glimmerwild_features {
        use super::*;
        // C: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // WOOL_FLOWER_MEADOW: not translated, it uses Minecraft's `ResourceKey`.
        // MEADOW_TUFTS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // MEADOW_BLOOMS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // DESERT_PLANTS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // WOUND_PLANTS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // MUSHROOM_CAVE_PLANTS: not translated, it uses `FeatureKey` (de.ambertation.wover.feature.api.configured).
        // PLACED_MUSHROOM_CAVE_PLANTS: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // PLACED_DESERT_PLANTS: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // PLACED_WOUND_PLANTS: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // PLACED_MEADOW_TUFTS: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // PLACED_MEADOW_BLOOMS: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // RAINBOW_DESERT: not translated, it uses Minecraft's `ResourceKey`.
        // SHADOW_WOUND: not translated, it uses Minecraft's `ResourceKey`.
        // MUSHROOM_CAVE: not translated, it uses Minecraft's `ResourceKey`.
        #[derive(Clone, Debug)]
        pub struct NetherGlimmerwildFeatures {
        }
        impl NetherGlimmerwildFeatures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherObjectsPlaced` (org/betterx/betternether/registry/features/placed/NetherObjectsPlaced.java)
    pub mod nether_objects_placed {
        use super::*;
        // C: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // PATCH_TERRACOTTA_CLUMP: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BONES: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // JUNGLE_BONES: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // OBSIDIAN_CRYSTAL: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BONE_STALAGMITE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // STALACTITE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // STALAGMITE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BASALT_STALACTITE_2: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BASALT_STALACTITE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BASALT_STALACTITE_SPARSE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BASALT_STALAGMITE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BASALT_STALAGMITE_SPARSE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BLACKSTONE_STALACTITE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BLACKSTONE_STALAGMITE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SMOKER: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SMOKER_SPARSE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // WART_DEADWOOD: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SCULK_HIDDEN: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SCULK_TOP: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // FOREST_LITTER: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        #[derive(Clone, Debug)]
        pub struct NetherObjectsPlaced {
        }
        impl NetherObjectsPlaced {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `NetherOresPlaced` (org/betterx/betternether/registry/features/placed/NetherOresPlaced.java)
    pub mod nether_ores_placed {
        use super::*;
        // C: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // CINCINNASITE_ORE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GLOOMWOOD_CEILING_DEBRIS: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // NETHER_RUBY_ORE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // NETHER_RUBY_ORE_SOUL: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // NETHER_RUBY_ORE_LARGE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // NETHER_RUBY_ORE_RARE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // NETHER_LAPIS_ORE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // NETHER_REDSTONE_ORE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        #[derive(Clone, Debug)]
        pub struct NetherOresPlaced {
        }
        impl NetherOresPlaced {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `NetherTerrainPlaced` (org/betterx/betternether/registry/features/placed/NetherTerrainPlaced.java)
    pub mod nether_terrain_placed {
        use super::*;
        // C: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // LAVA_PIT: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // MAGMA_BLOBS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BASALT_OR_AIR: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // MARK: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // EXTEND_BASALT: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LAVA_PITS_SPARSE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LAVA_PITS_DENSE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LAVA_SWAMP: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LAVA_TERRACE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // FLOODED_LAVA_PIT_SURFACE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // FLOODED_LAVA_PIT: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // MOLTEN_GLOOMSCULK: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // MOLTEN_GLOOMSCULK_NEAR_LAVA: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GLOOMSCULK_GEODE_FLOOR: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GLOOMSCULK_GEODE_CEILING: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GLOOMSCULK_GEODE_ON_FLOOR: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GLOOMSCULK_CRYSTAL_FLOOR: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GLOOMSCULK_CRYSTAL_CEILING: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SCULK_VEIN_FLOOR: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SCULK_VEIN_WALL: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SCULK_VEIN_CEILING: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // REPLACE_SOUL_SANDSTONE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        #[derive(Clone, Debug)]
        pub struct NetherTerrainPlaced {
        }
        impl NetherTerrainPlaced {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `NetherTreesPlaced` (org/betterx/betternether/registry/features/placed/NetherTreesPlaced.java)
    pub mod nether_trees_placed {
        use super::*;
        // C: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // RUBEUS_TREE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GLOOMWOOD_TREE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GLOOMWOOD_TREE_EDGE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GLOOMWOOD_TREE_SOLITARY: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // MUSHROOM_FIR: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BIG_RED_MUSHROOM: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BIG_BROWN_MUSHROOM: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // WART_TREE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // WILLOW_TREE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SAKURA_TREE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // ANCHOR_TREE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // ANCHOR_TREE_BRANCH: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // ANCHOR_TREE_ROOT: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CRIMSON_GLOWING_TREE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // CRIMSON_PINE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // STALAGNATE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GIANT_MOLD: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // OLD_RED_MUSHROOM: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // OLD_BROWN_MUSHROOM: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SOUL_LILY: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // OLD_WILLOW_TREE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // ANCHOR_TREE_SPARSE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BIG_WARPED_TREE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        #[derive(Clone, Debug)]
        pub struct NetherTreesPlaced {
        }
        impl NetherTreesPlaced {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `NetherVegetationPlaced` (org/betterx/betternether/registry/features/placed/NetherVegetationPlaced.java)
    pub mod nether_vegetation_placed {
        use super::*;
        // C: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // VEGETATION_MUSHROOM_FORREST_EDGE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // JELLYFISH_MUSHROOM: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // PATCH_JELLYFISH_MUSHROOM: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // JELLYFISH_MUSHROOM_DENSE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BLACK_BUSH: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BLACK_BUSH_SPARSE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // WALL_LUCIS: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_BONE_REEF: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_SULFURIC_BONE_REEF: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_MAGMA_LAND: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_CRIMSON_GLOWING_WOODS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_GRASSLANDS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_GRAVEL_DESERT: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_JUNGLE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_MUSHROOM_FORREST: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_POOR_GRASSLANDS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_SOUL_PLAIN: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_WART_FOREST: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_WART_FOREST_EDGE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_SWAMPLAND: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_OLD_SWAMPLAND: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_OLD_WARPED_WOODS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // NETHER_CACTUS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // WALL_MUSHROOM_RED_WITH_MOSS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // WALL_MUSHROOMS_WITH_MOSS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // WALL_MUSHROOMS: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // WALL_JUNGLE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // WALL_UPSIDE_DOWN: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_GLOOMWOOD_BLEACHED: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // VEGETATION_GLOOMWOOD_SCULK: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GLOOMWISP_VINE_HEAD: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GLOOMWISP_VINE: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // NETHER_REED: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // WART_BUSH: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // WILLOW_BUSH: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // RUBEUS_BUSH: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SAKURA_BUSH: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // WART_CAP: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // SCULK_VEGETATION: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // HOOK_MUSHROOM: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // MOSS_COVER: not translated, it uses `PlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        #[derive(Clone, Debug)]
        pub struct NetherVegetationPlaced {
        }
        impl NetherVegetationPlaced {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `NetherVinesPlaced` (org/betterx/betternether/registry/features/placed/NetherVinesPlaced.java)
    pub mod nether_vines_placed {
        use super::*;
        // C: not translated, it uses `ModCore` (de.ambertation.wover.core.api).
        // PLACED_LUMABUS_VINE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // PLACED_LUMABUS_VINE_GLOOM: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // PLACED_GOLDEN_LUMABUS_VINE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // PLACED_EYE_VINE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // PLACED_NEON_EQUISETUM: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // PLACED_WHISPERING_GOURD_VINE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LUMABUS_VINE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GOLDEN_LUMABUS_VINE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GOLDEN_VINE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GOLDEN_VINE_SPARSE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // EYE_VINE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // GLOOMSCULK_VINE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // LUMABUS_VINE_SPARSE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BLACK_VINE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // BLOOMING_VINE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // TWISTING_VINES: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // NEON_EQUISETUM: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        // PATCH_WHISPERING_GOURD_VINE: not translated, it uses `BoundPlacedFeatureKey` (de.ambertation.wover.feature.api.placed).
        #[derive(Clone, Debug)]
        pub struct NetherVinesPlaced {
        }
        impl NetherVinesPlaced {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `NetherEquipmentItems` (org/betterx/betternether/registry/item/NetherEquipmentItems.java)
    pub mod nether_equipment_items {
        use super::*;
        // CINCINNASITE_SET: not translated, it uses NetherSet, which isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)).
        // NETHER_RUBY_SET: not translated, it uses NetherSet, which isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)).
        // CINCINNASITE_DIAMOND_SET: not translated, it uses DiamondSet, which isn't translated (it extends `NetherSet`).
        // FLAMING_RUBY_SET: not translated, it uses NetherSet, which isn't translated (it extends `EquipmentSet` (de.ambertation.wover.complex.api.equipment)).
        // CINCINNASITE_HAMMER: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // CINCINNASITE_HAMMER_DIAMOND: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // NETHER_RUBY_HAMMER: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // CINCINNASITE_EXCAVATOR: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // CINCINNASITE_EXCAVATOR_DIAMOND: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // NETHER_RUBY_EXCAVATOR: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherEquipmentItems {
        }
        impl NetherEquipmentItems {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherFoodItems` (org/betterx/betternether/registry/item/NetherFoodItems.java)
    pub mod nether_food_items {
        use super::*;
        // BLACK_APPLE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // STALAGNATE_BOWL: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // STALAGNATE_BOWL_WART: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // STALAGNATE_BOWL_MUSHROOM: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // STALAGNATE_BOWL_APPLE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // HOOK_MUSHROOM_COOKED: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherFoodItems {
        }
        impl NetherFoodItems {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherItemFactories` (org/betterx/betternether/registry/item/NetherItemFactories.java)
    pub mod nether_item_factories {
        use super::*;
        // CINCINNASITE: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // CINCINNASITE_INGOT: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // NETHER_RUBY: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // GLOWSTONE_PILE: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // LAPIS_PILE: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // AGAVE_LEAF: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // AGAVE_MEDICINE: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // HERBAL_MEDICINE: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // BLACK_APPLE: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // STALAGNATE_BOWL: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // STALAGNATE_BOWL_WART: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // STALAGNATE_BOWL_MUSHROOM: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // STALAGNATE_BOWL_APPLE: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // CINCINNASITE_HAMMER: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // CINCINNASITE_HAMMER_DIAMOND: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // NETHER_RUBY_HAMMER: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // CINCINNASITE_EXCAVATOR: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // CINCINNASITE_EXCAVATOR_DIAMOND: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        // NETHER_RUBY_EXCAVATOR: not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherItemFactories {
        }
        impl NetherItemFactories {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherItemFactories.factory(2): not translated, it uses `ItemCreator` (de.ambertation.wover.item.api.classify).
        pub fn ensure_loaded() {
        }
    }

    /// `NetherResourceItems` (org/betterx/betternether/registry/item/NetherResourceItems.java)
    pub mod nether_resource_items {
        use super::*;
        // CINCINNASITE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // CINCINNASITE_INGOT: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // NETHER_RUBY: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // GLOWSTONE_PILE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // LAPIS_PILE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // AGAVE_LEAF: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // AGAVE_MEDICINE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // HERBAL_MEDICINE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherResourceItems {
        }
        impl NetherResourceItems {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_loaded() {
        }
    }

    /// `NetherBiomes` (org/betterx/betternether/registry/NetherBiomes.java)
    pub mod nether_biomes {
        use super::*;
        // BONE_REEF: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // CRIMSON_GLOWING_WOODS: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // CRIMSON_PINEWOOD: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // FLOODED_DELTAS: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // GLOOMWOOD: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // GRAVEL_DESERT: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // MAGMA_LAND: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // NETHER_GRASSLANDS: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // NETHER_JUNGLE: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // NETHER_MUSHROOM_FORREST: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // NETHER_MUSHROOM_FORREST_EDGE: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // NETHER_SWAMPLAND: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // NETHER_SWAMPLAND_TERRACES: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // OLD_FUNGIWOODS: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // OLD_SWAMPLAND: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // OLD_WARPED_WOODS: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // POOR_NETHER_GRASSLANDS: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // SOUL_PLAIN: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // SULFURIC_BONE_REEF: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // UPSIDE_DOWN_FOREST: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // UPSIDE_DOWN_FOREST_CLEARED: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // WART_FOREST: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        // WART_FOREST_EDGE: not translated, it uses NetherBiomeKey, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct NetherBiomes {
        }
        impl NetherBiomes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherBiomes.register(0): not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // NetherBiomes.registerNumericProviders(0): not translated, it uses `NumericProviderRegistry` (de.ambertation.wover.surface.api.noise).NUMERIC_PROVIDER.
    }

    /// `NetherBlocks` (org/betterx/betternether/registry/NetherBlocks.java)
    pub mod nether_blocks {
        use super::*;
        // BLOCKS_REGISTRY: not translated, it uses `BlockRegistry` (de.ambertation.wover.block.api).
        #[derive(Clone, Debug)]
        pub struct NetherBlocks {
        }
        impl NetherBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherBlocks.getBlockRegistry(0): not translated, it uses `BlockRegistry` (de.ambertation.wover.block.api).
        // NetherBlocks.key(2): not translated, it's generic.
        // NetherBlocks.getModBlocks(0): not translated, it uses `Stream` (java.util.stream).
        // NetherBlocks.getModBlockItems(0): not translated, it uses `Stream` (java.util.stream).
        pub fn ensure_loaded() {
            nether_terrain_blocks::ensure_loaded();
            nether_stone_blocks::ensure_loaded();
            nether_metal_blocks::ensure_loaded();
            nether_ore_blocks::ensure_loaded();
            nether_obsidian_blocks::ensure_loaded();
            nether_glass_blocks::ensure_loaded();
            nether_wood_blocks::ensure_loaded();
            nether_leaves_blocks::ensure_loaded();
            nether_plant_blocks::ensure_loaded();
            nether_wall_plant_blocks::ensure_loaded();
            nether_crop_blocks::ensure_loaded();
            nether_sapling_blocks::ensure_loaded();
            nether_vine_blocks::ensure_loaded();
            nether_mushroom_blocks::ensure_loaded();
            nether_light_blocks::ensure_loaded();
            nether_furniture_blocks::ensure_loaded();
            nether_functional_blocks::ensure_loaded();
            nether_decor_blocks::ensure_loaded();
        }
    }

    /// `NetherClassification` (org/betterx/betternether/registry/NetherClassification.java)
    pub mod nether_classification {
        use super::*;
        // The class itself isn't translated: it implements `WoverBlocksEntrypoint` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // NetherClassification.registerClassification(0): not translated, its class isn't translated (it implements `WoverBlocksEntrypoint` (de.ambertation.wover.block.api.classify)).
        // NetherClassification.init(1): not translated, it uses `Class`.
    }

    /// `NetherEnchantments` (org/betterx/betternether/registry/NetherEnchantments.java)
    pub mod nether_enchantments {
        use super::*;
        // OBSIDIAN_BREAKER: not translated, it uses `EnchantmentKey` (de.ambertation.wover.enchantment.api).
        // RUBY_FIRE: not translated, it uses `EnchantmentKey` (de.ambertation.wover.enchantment.api).
        // OBSIDIAN_BLOCK_BREAK_SPEED: not translated, it uses Minecraft's `Holder`.
        #[derive(Clone, Debug)]
        pub struct NetherEnchantments {
        }
        impl NetherEnchantments {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_statically_loaded() {
        }
    }

    /// `NetherEntities` (org/betterx/betternether/registry/NetherEntities.java)
    pub mod nether_entities {
        use super::*;
        // ATTRIBUTES: not translated, it uses Minecraft's `EntityType`.
        // NETHER_ENTITIES: not translated, it uses `BCLEntityWrapper` (org.betterx.bclib.entity).
        // SPAWN_EGGS: not translated, it uses SpawnEgg, which isn't translated (it uses Minecraft's `EntityType`).
        // NAGA_PROJECTILE: not translated, it uses Minecraft's `EntityType`.
        // FIREFLY: not translated, it uses `BCLEntityWrapper` (org.betterx.bclib.entity).
        // HYDROGEN_JELLYFISH: not translated, it uses `BCLEntityWrapper` (org.betterx.bclib.entity).
        // NAGA: not translated, it uses `BCLEntityWrapper` (org.betterx.bclib.entity).
        // FLYING_PIG: not translated, it uses `BCLEntityWrapper` (org.betterx.bclib.entity).
        // JUNGLE_SKELETON: not translated, it uses `BCLEntityWrapper` (org.betterx.bclib.entity).
        // SKULL: not translated, it uses `BCLEntityWrapper` (org.betterx.bclib.entity).
        pub const MAX_FLOAT_HEIGHT: i32 = 7;
        // RULE_FLOAT_NOT_ABOVE_LAVA: not translated, it uses `SpawnRule` (org.betterx.bclib.interfaces).
        // RULE_FLOAT_ABOVE_LAVA: not translated, it uses `SpawnRule` (org.betterx.bclib.interfaces).
        #[derive(Clone, Debug)]
        pub struct NetherEntities {
        }
        impl NetherEntities {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherEntities.spawnEggs(0): not translated, it uses NetherEntities.SPAWN_EGGS, which isn't translated (it calls `Lists` (com.google.common.collect).newArrayList).
        // NetherEntities.register(9): not translated, it's generic.
        // NetherEntities.testSpawnAboveLava(3): not translated, it uses Minecraft's `LevelAccessor`.
        // NetherEntities.register(0): not translated, it uses Minecraft's `EntityType`.
        // NetherEntities.registerEntity(2): not translated, it uses Minecraft's `EntityType`.
        // NetherEntities.registerEntity(3): not translated, it uses Minecraft's `EntityType`.
        // NetherEntities.isNetherEntity(1): not translated, it uses Minecraft's `Entity`.
    }

    /// `NetherEntities.KnownSpawnTypes` (org/betterx/betternether/registry/NetherEntities.java)
    pub mod nether_entities_known_spawn_types {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // NetherEntities.KnownSpawnTypes.isVanilla(0): not translated, its class isn't translated (its constants carry values).
        // NetherEntities.KnownSpawnTypes.addSpawn(2): not translated, its class isn't translated (its constants carry values).
        // NetherEntities.KnownSpawnTypes.KnownSpawnTypes(4): not translated, its class isn't translated (its constants carry values).
        // NetherEntities.KnownSpawnTypes.KnownSpawnTypes(4): not translated, its class isn't translated (its constants carry values).
    }

    /// `NetherEntities.SpawnEgg` (org/betterx/betternether/registry/NetherEntities.java)
    pub mod nether_entities_spawn_egg {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `EntityType`. Its static parts are below.
        // NetherEntities.SpawnEgg.SpawnEgg(4): not translated, its class isn't translated (it uses Minecraft's `EntityType`).
        // NetherEntities.SpawnEgg.name(0): not translated, its class isn't translated (it uses Minecraft's `EntityType`).
        // NetherEntities.SpawnEgg.type(0): not translated, its class isn't translated (it uses Minecraft's `EntityType`).
        // NetherEntities.SpawnEgg.background(0): not translated, its class isn't translated (it uses Minecraft's `EntityType`).
        // NetherEntities.SpawnEgg.dots(0): not translated, its class isn't translated (it uses Minecraft's `EntityType`).
    }

    /// `NetherFeatures` (org/betterx/betternether/registry/NetherFeatures.java)
    pub mod nether_features {
        use super::*;
        // JELLYFISH_MUSHROOM: not translated, it uses JellyfishMushroomFeature, which isn't translated (it extends `ContextFeature`).
        // OBSIDIAN_CRYSTAL: not translated, it uses CrystalFeature, which isn't translated (it extends `NetherSurfaceFeature`).
        // WART_BUSH: not translated, it uses WartBushFeature, which isn't translated (it extends `ContextFeature`).
        // RUBEUS_TREE: not translated, it uses RubeusTreeFeature, which isn't translated (it extends `NonOverlappingFeature`).
        // GLOOMWOOD_TREE: not translated, it uses GloomwoodTreeFeature, which isn't translated (it extends `NonOverlappingFeature`).
        // MUSHROOM_FIR: not translated, it uses MushroomFirFeature, which isn't translated (it extends `ContextFeature`).
        // BIG_BROWN_MUSHROOM: not translated, it uses BigBrownMushroomFeature, which isn't translated (it extends `ContextFeature`).
        // RUBEUS_BUSH: not translated, it uses RubeusBushFeature, which isn't translated (it extends `ContextFeature`).
        // LUCIS: not translated, it uses LucisFeature, which isn't translated (it extends `ContextFeature`).
        // SOUL_LILY: not translated, it uses SoulLilyFeature, which isn't translated (it extends `ContextFeature`).
        // WART_TREE: not translated, it uses WartTreeFeature, which isn't translated (it extends `NonOverlappingFeature`).
        // WILLOW_BUSH: not translated, it uses WillowBushFeature, which isn't translated (it extends `ContextFeature`).
        // WILLOW_TREE: not translated, it uses WillowTreeFeature, which isn't translated (it extends `ContextFeature`).
        // OLD_WILLOW_TREE: not translated, it uses OldWillowTree, which isn't translated (it extends `NonOverlappingFeature`).
        // SAKURA_TREE: not translated, it uses NetherSakuraFeature, which isn't translated (it extends `ContextFeature`).
        // SAKURA_BUSH: not translated, it uses NetherSakuraBushFeature, which isn't translated (it extends `ContextFeature`).
        // ANCHOR_TREE_BRANCH: not translated, it uses AnchorTreeBranchFeature, which isn't translated (it extends `ContextFeature`).
        // ANCHOR_TREE: not translated, it uses AnchorTreeFeature, which isn't translated (it extends `ContextFeature`).
        // ANCHOR_TREE_ROOT: not translated, it uses AnchorTreeRootFeature, which isn't translated (it extends `ContextFeature`).
        // WART_CAP: not translated, it uses WartCapFeature, which isn't translated (it extends `ContextFeature`).
        // TWISTING_VINES: not translated, it uses TwistedVinesFeature, which isn't translated (it extends `ContextFeature`).
        // AVOID_NETHER_CITY: not translated, it uses Minecraft's `MapCodec`.
        #[derive(Clone, Debug)]
        pub struct NetherFeatures {
        }
        impl NetherFeatures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherFeatures.register(3): not translated, it's generic.
        // NetherFeatures.addDefaultFeatures(1): not translated, it uses NetherBiomeBuilder, which isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)).
        // NetherFeatures.addDefaultBNFeatures(1): not translated, it uses NetherBiomeBuilder, which isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)).
        // NetherFeatures.addDefaultOres(1): not translated, it uses NetherBiomeBuilder, which isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)).
        // NetherFeatures.register(0): not translated, it uses `WorldLifecycle` (de.ambertation.wover.events.api).SERVER_LEVEL_READY.
        // NetherFeatures.onWorldLoad(4): not translated, it uses Minecraft's `ServerLevel`.
    }

    /// `NetherGameRules` (org/betterx/betternether/registry/NetherGameRules.java)
    pub mod nether_game_rules {
        use super::*;
        // GLOOMWISP_DROPS_EXPERIENCE: not translated, it uses Minecraft's `GameRule`.
        // GENERATE_BLUE_RUINED_PORTALS: not translated, it uses Minecraft's `GameRule`.
        // GROW_LARGE_WILLOWS: not translated, it uses Minecraft's `GameRule`.
        // GROW_LARGE_ANCHOR_TREES: not translated, it uses Minecraft's `GameRule`.
        #[derive(Clone, Debug)]
        pub struct NetherGameRules {
        }
        impl NetherGameRules {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_statically_loaded() {
        }
    }

    /// `NetherItems` (org/betterx/betternether/registry/NetherItems.java)
    pub mod nether_items {
        use super::*;
        // ITEMS_REGISTRY: not translated, it uses `ItemRegistry` (de.ambertation.wover.item.api).
        // DEBUG_ITEMS: not translated, it uses DebugItem, which isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)).
        #[derive(Clone, Debug)]
        pub struct NetherItems {
        }
        impl NetherItems {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherItems.getItemRegistry(0): not translated, it uses `ItemRegistry` (de.ambertation.wover.item.api).
        // NetherItems.key(2): not translated, it's generic.
        pub fn default_settings() -> JProps {
            return JProps::new();
        }
        // NetherItems.addDebugItem(3): not translated, it uses Minecraft's `ResourceKey`.
        // NetherItems.debugItems(0): not translated, it uses DebugItem, which isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)).
        // NetherItems.buildItem(3): not translated, it uses Minecraft's `CompoundTag`.
        // NetherItems.fullLightRange(0): not translated, it uses Minecraft's `CompoundTag`.
        // NetherItems.buildCitySpawnerData(0): not translated, it uses Minecraft's `CompoundTag`.
        pub fn ensure_loaded() {
            nether_food_items::ensure_loaded();
            nether_equipment_items::ensure_loaded();
            nether_resource_items::ensure_loaded();
        }
    }

    /// `NetherItems.DebugItem` (org/betterx/betternether/registry/NetherItems.java)
    pub mod nether_items_debug_item {
        use super::*;
        // The class itself isn't translated: it uses `ItemCreator` (de.ambertation.wover.item.api.classify). Its static parts are below.
        // NetherItems.DebugItem.DebugItem(3): not translated, its class isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)).
        // NetherItems.DebugItem.path(0): not translated, its class isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)).
        // NetherItems.DebugItem.factory(0): not translated, its class isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)).
        // NetherItems.DebugItem.icon(0): not translated, its class isn't translated (it uses `ItemCreator` (de.ambertation.wover.item.api.classify)).
    }

    /// `NetherParticles` (org/betterx/betternether/registry/NetherParticles.java)
    pub mod nether_particles {
        use super::*;
        // BLUE_DRIPPING_OBSIDIAN_TEAR: not translated, it uses Minecraft's `SimpleParticleType`.
        // BLUE_FALLING_OBSIDIAN_TEAR: not translated, it uses Minecraft's `SimpleParticleType`.
        // BLUE_LANDING_OBSIDIAN_TEAR: not translated, it uses Minecraft's `SimpleParticleType`.
        // BLUE_DRIPPING_OBSIDIAN_WEEP: not translated, it uses Minecraft's `SimpleParticleType`.
        // BLUE_FALLING_OBSIDIAN_WEEP: not translated, it uses Minecraft's `SimpleParticleType`.
        // BLUE_LANDING_OBSIDIAN_WEEP: not translated, it uses Minecraft's `SimpleParticleType`.
        // DRIPPING_OBSIDIAN_WEEP: not translated, it uses Minecraft's `SimpleParticleType`.
        // FALLING_OBSIDIAN_WEEP: not translated, it uses Minecraft's `SimpleParticleType`.
        // LANDING_OBSIDIAN_WEEP: not translated, it uses Minecraft's `SimpleParticleType`.
        #[derive(Clone, Debug)]
        pub struct NetherParticles {
        }
        impl NetherParticles {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherParticles.register(0): not translated, it uses NetherParticles.BLUE_DRIPPING_OBSIDIAN_TEAR, which isn't translated (it uses Minecraft's `SimpleParticleType`).
    }

    /// `NetherPoiTypes` (org/betterx/betternether/registry/NetherPoiTypes.java)
    pub mod nether_poi_types {
        use super::*;
        // PIG_STATUE: not translated, it uses `WoverPoiType` (de.ambertation.wover.poi.api).
        #[derive(Clone, Debug)]
        pub struct NetherPoiTypes {
        }
        impl NetherPoiTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn register() {
        }
    }

    /// `NetherStructurePieces` (org/betterx/betternether/registry/NetherStructurePieces.java)
    pub mod nether_structure_pieces {
        use super::*;
        // NETHER_CITY_PIECE: not translated, it uses Minecraft's `StructurePieceType`.
        // CAVE_PIECE: not translated, it uses Minecraft's `StructurePieceType`.
        // DESTRUCTION_PIECE: not translated, it uses Minecraft's `StructurePieceType`.
        // ANCHOR_TREE_PIECE: not translated, it uses Minecraft's `StructurePieceType`.
        // LAVA_LAKE_PIECE: not translated, it uses Minecraft's `StructurePieceType`.
        #[derive(Clone, Debug)]
        pub struct NetherStructurePieces {
        }
        impl NetherStructurePieces {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn ensure_static_load() {
        }
    }

    /// `NetherStructures` (org/betterx/betternether/registry/NetherStructures.java)
    pub mod nether_structures {
        use super::*;
        // CITY_STRUCTURE: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // PYRAMIDS: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // GHAST_HIVE: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // SPAWN_ALTAR_LADDER: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // RESPAWN_POINTS: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // PILLARS: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // GARDENS: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // PORTALS: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // ALTARS: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // JUNGLE_TEMPLES: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        // MEGA_LAVA_LAKE: not translated, it uses `Simple` (de.ambertation.wover.structure.api.StructureKey).
        #[derive(Clone, Debug)]
        pub struct NetherStructures {
        }
        impl NetherStructures {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn register() {
            nether_structure_pieces::ensure_static_load();
        }
    }

    /// `NetherTags` (org/betterx/betternether/registry/NetherTags.java)
    pub mod nether_tags {
        use super::*;
        // BETTER_NETHER_DECORATIONS: not translated, it uses Minecraft's `TagKey`.
        // CITY_AVOIDS: not translated, it uses Minecraft's `TagKey`.
        // SOUL_GROUND_ITEM: not translated, it uses Minecraft's `TagKey`.
        // NETHER_FARMLAND: not translated, it uses Minecraft's `TagKey`.
        // GLIMMERWILD_GROUND: not translated, it uses Minecraft's `TagKey`.
        // GLIMMERWILD_RAINBOW_SAND: not translated, it uses Minecraft's `TagKey`.
        // GLIMMERWILD_SOILS: not translated, it uses Minecraft's `TagKey`.
        // NETHER_SAND: not translated, it uses Minecraft's `TagKey`.
        // NETHER_NYLIUM: not translated, it uses Minecraft's `TagKey`.
        // BETTER_NETHER: not translated, it uses Minecraft's `TagKey`.
        // FIREFLY_FLOWERS: not translated, it uses Minecraft's `TagKey`.
        // FLAMING_RUBY_ENCHANTABLE: not translated, it uses Minecraft's `TagKey`.
        // FLAMING_RUBY_PRIMARY: not translated, it uses Minecraft's `TagKey`.
        // OBSIDIAN_BREAKER_ENCHANTABLE: not translated, it uses Minecraft's `TagKey`.
        // OBSIDIAN_BREAKER_PRIMARY: not translated, it uses Minecraft's `TagKey`.
        // OBSIDIAN_BREAKER_MINEABLE: not translated, it uses Minecraft's `TagKey`.
        // NETHER_PICKAXES: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct NetherTags {
        }
        impl NetherTags {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn register() {
        }
    }

    /// `NetherTemplates` (org/betterx/betternether/registry/NetherTemplates.java)
    pub mod nether_templates {
        use super::*;
        // EMPTY_SLOT_BOWL: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // EMPTY_SLOT_BLOCK: not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // NETHER_BOWL_SMITHING_TEMPLATE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // FLAMING_RUBY_TEMPLATE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        // CINCINNASITE_DIAMOND_TEMPLATE: not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherTemplates {
        }
        impl NetherTemplates {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherTemplates.key(1): not translated, it uses `ItemKey` (de.ambertation.wover.item.api.classify).
        pub fn ensure_statically_loaded() {
        }
    }

    /// `SoundsRegistry` (org/betterx/betternether/registry/SoundsRegistry.java)
    pub mod sounds_registry {
        use super::*;
        // AMBIENT_MUSHROOM_FOREST: not translated, it uses Minecraft's `Holder`.
        // AMBIENT_GRAVEL_DESERT: not translated, it uses Minecraft's `Holder`.
        // AMBIENT_NETHER_JUNGLE: not translated, it uses Minecraft's `Holder`.
        // AMBIENT_SWAMPLAND: not translated, it uses Minecraft's `Holder`.
        // MUSIC_GLOOMWOOD: not translated, it uses Minecraft's `Holder`.
        // BLOCK_GLOOMWISP_CHIME: not translated, it uses Minecraft's `Holder`.
        // BLOCK_GLOOMWISP_BOON: not translated, it uses Minecraft's `Holder`.
        // MOB_FIREFLY_FLY: not translated, it uses Minecraft's `Holder`.
        // MOB_JELLYFISH: not translated, it uses Minecraft's `Holder`.
        // MOB_NAGA_IDLE: not translated, it uses Minecraft's `Holder`.
        // MOB_NAGA_ATTACK: not translated, it uses Minecraft's `Holder`.
        // MOB_SKULL_FLIGHT: not translated, it uses Minecraft's `Holder`.
        #[derive(Clone, Debug)]
        pub struct SoundsRegistry {
        }
        impl SoundsRegistry {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SoundsRegistry.register(1): not translated, it uses Minecraft's `SoundEvent`.
        // SoundsRegistry.registerHolder(1): not translated, it uses Minecraft's `Holder`.
        pub fn ensure_statically_loaded() {
        }
    }

    /// `BECreativeTabs` (org/betterx/betternether/tab/BECreativeTabs.java)
    pub mod be_creative_tabs {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BECreativeTabs {
        }
        impl BECreativeTabs {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BECreativeTabs.register(0): not translated, it uses BetterNether.C, which isn't translated (it uses `ModCore` (de.ambertation.wover.core.api)).
        // BECreativeTabs.isPlantLike(1): not translated, it uses Minecraft's `Block`.
    }

    /// `BoneReef` (org/betterx/betternether/world/biomes/BoneReef.java)
    pub mod bone_reef {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // BoneReef.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // BoneReef.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `CrimsonGlowingWoods` (org/betterx/betternether/world/biomes/CrimsonGlowingWoods.java)
    pub mod crimson_glowing_woods {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // NETHER_WART_BLOCK: not translated, it uses Minecraft's `MaterialRule`.
        // CRIMSON_NYLIUM: not translated, it uses Minecraft's `MaterialRule`.
        // CrimsonGlowingWoods.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // CrimsonGlowingWoods.spawnWeight(1): not translated, it's generic.
        // CrimsonGlowingWoods.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // CrimsonGlowingWoods.subBiomeOf(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `CrimsonPinewood` (org/betterx/betternether/world/biomes/CrimsonPinewood.java)
    pub mod crimson_pinewood {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // TERRAIN: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        // CrimsonPinewood.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // CrimsonPinewood.spawnWeight(1): not translated, it's generic.
        // CrimsonPinewood.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // CrimsonPinewood.subBiomeOf(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `FloodedDeltas` (org/betterx/betternether/world/biomes/FloodedDeltas.java)
    pub mod flooded_deltas {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // FloodedDeltas.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // FloodedDeltas.hasNetherCity(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // FloodedDeltas.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // FloodedDeltas.subBiomeOf(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `Gloomwood` (org/betterx/betternether/world/biomes/Gloomwood.java)
    pub mod gloomwood {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // CEILING_NETHERRACK: not translated, it uses `NoiseCondition` (de.ambertation.wover.surface.api.conditions).
        // CEILING_DEEP: not translated, it uses `NoiseCondition` (de.ambertation.wover.surface.api.conditions).
        // FLOOR_SCULK: not translated, it uses `NoiseCondition` (de.ambertation.wover.surface.api.conditions).
        // FLOOR_MOLTEN: not translated, it uses `NoiseCondition` (de.ambertation.wover.surface.api.conditions).
        // Gloomwood.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // Gloomwood.spawnWeight(1): not translated, it's generic.
        // Gloomwood.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `GravelDesert` (org/betterx/betternether/world/biomes/GravelDesert.java)
    pub mod gravel_desert {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // GRAVEL: not translated, it uses Minecraft's `MaterialRule`.
        // GravelDesert.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // GravelDesert.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // GravelDesert.spawnWeight(1): not translated, it's generic.
    }

    /// `MagmaLand` (org/betterx/betternether/world/biomes/MagmaLand.java)
    pub mod magma_land {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // MagmaLand.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // MagmaLand.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `NetherGrasslands` (org/betterx/betternether/world/biomes/NetherGrasslands.java)
    pub mod nether_grasslands {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // SOUL_SOIL: not translated, it uses Minecraft's `MaterialRule`.
        // SOUL_SAND: not translated, it uses Minecraft's `MaterialRule`.
        // MOSS: not translated, it uses Minecraft's `MaterialRule`.
        // BLUE: not translated, it uses Minecraft's `MaterialRule`.
        // LIGHT_BLUE: not translated, it uses Minecraft's `MaterialRule`.
        // CYAN: not translated, it uses Minecraft's `MaterialRule`.
        // GREEN: not translated, it uses Minecraft's `MaterialRule`.
        // LIME_GREEN: not translated, it uses Minecraft's `MaterialRule`.
        // YELLOW: not translated, it uses Minecraft's `MaterialRule`.
        // ORANGE: not translated, it uses Minecraft's `MaterialRule`.
        // RED: not translated, it uses Minecraft's `MaterialRule`.
        // PINK: not translated, it uses Minecraft's `MaterialRule`.
        // PURPLE: not translated, it uses Minecraft's `MaterialRule`.
        // MAGENTA: not translated, it uses Minecraft's `MaterialRule`.
        // BLACK: not translated, it uses Minecraft's `MaterialRule`.
        // NetherGrasslands.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // NetherGrasslands.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // NetherGrasslands.spawnWeight(1): not translated, it's generic.
    }

    /// `NetherJungle` (org/betterx/betternether/world/biomes/NetherJungle.java)
    pub mod nether_jungle {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // NetherJungle.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // NetherJungle.hasBNStructures(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // NetherJungle.spawnWeight(1): not translated, it's generic.
        // NetherJungle.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `NetherMushroomForest` (org/betterx/betternether/world/biomes/NetherMushroomForest.java)
    pub mod nether_mushroom_forest {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // NetherMushroomForest.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // NetherMushroomForest.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // NetherMushroomForest.withEdgeBiome(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `NetherMushroomForestEdge` (org/betterx/betternether/world/biomes/NetherMushroomForestEdge.java)
    pub mod nether_mushroom_forest_edge {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // NetherMushroomForestEdge.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // NetherMushroomForestEdge.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // NetherMushroomForestEdge.hasNetherCity(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `NetherSwampland` (org/betterx/betternether/world/biomes/NetherSwampland.java)
    pub mod nether_swampland {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // NetherSwampland.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // NetherSwampland.addCustomSwamplandBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // NetherSwampland.spawnWeight(1): not translated, it's generic.
        // NetherSwampland.hasDefaultOres(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // NetherSwampland.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `NetherSwamplandTerraces` (org/betterx/betternether/world/biomes/NetherSwamplandTerraces.java)
    pub mod nether_swampland_terraces {
        use super::*;
        // The class itself isn't translated: it extends `NetherSwampland`. Its static parts are below.
        // NetherSwamplandTerraces.addCustomSwamplandBuildData(1): not translated, its class isn't translated (it extends `NetherSwampland`).
        // NetherSwamplandTerraces.subBiomeOf(0): not translated, its class isn't translated (it extends `NetherSwampland`).
    }

    /// `OldFungiwoods` (org/betterx/betternether/world/biomes/OldFungiwoods.java)
    pub mod old_fungiwoods {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // OldFungiwoods.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // OldFungiwoods.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // OldFungiwoods.subBiomeOf(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `OldSwampland` (org/betterx/betternether/world/biomes/OldSwampland.java)
    pub mod old_swampland {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // OldSwampland.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // OldSwampland.spawnWeight(1): not translated, it's generic.
        // OldSwampland.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // OldSwampland.subBiomeOf(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `OldWarpedWoods` (org/betterx/betternether/world/biomes/OldWarpedWoods.java)
    pub mod old_warped_woods {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // OldWarpedWoods.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // OldWarpedWoods.spawnWeight(1): not translated, it's generic.
        // OldWarpedWoods.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // OldWarpedWoods.subBiomeOf(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `PoorNetherGrasslands` (org/betterx/betternether/world/biomes/PoorNetherGrasslands.java)
    pub mod poor_nether_grasslands {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // PoorNetherGrasslands.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // PoorNetherGrasslands.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // PoorNetherGrasslands.subBiomeOf(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `NetherGrasslandsNumericProvider` (org/betterx/betternether/world/biomes/providers/NetherGrasslandsNumericProvider.java)
    pub mod nether_grasslands_numeric_provider {
        use super::*;
        // The class itself isn't translated: it implements `NumericProvider` (de.ambertation.wover.surface.api.noise). Its static parts are below.
        // DEFAULT: not translated, it uses NetherGrasslandsNumericProvider, which isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub const SEED: i32 = 1852273249;
        // NetherGrasslandsNumericProvider.getNumber(1): not translated, its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
        // NetherGrasslandsNumericProvider.pcodec(0): not translated, its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
    }

    /// `NetherMushroomForestEdgeNumericProvider` (org/betterx/betternether/world/biomes/providers/NetherMushroomForestEdgeNumericProvider.java)
    pub mod nether_mushroom_forest_edge_numeric_provider {
        use super::*;
        // The class itself isn't translated: it implements `NumericProvider` (de.ambertation.wover.surface.api.noise). Its static parts are below.
        // DEFAULT: not translated, it uses NetherMushroomForestEdgeNumericProvider, which isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub const SEED: i32 = 1836279922;
        // NetherMushroomForestEdgeNumericProvider.getNumber(1): not translated, its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
        // NetherMushroomForestEdgeNumericProvider.pcodec(0): not translated, its class isn't translated (it implements `NumericProvider` (de.ambertation.wover.surface.api.noise)).
    }

    /// `SoulPlain` (org/betterx/betternether/world/biomes/SoulPlain.java)
    pub mod soul_plain {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // SOUL_SAND: not translated, it uses Minecraft's `MaterialRule`.
        // SOUL_SOIL: not translated, it uses Minecraft's `MaterialRule`.
        // SOUL_SANDSTONE: not translated, it uses Minecraft's `MaterialRule`.
        // LAVA: not translated, it uses Minecraft's `MaterialRule`.
        // SoulPlain.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // SoulPlain.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // SoulPlain.subBiomeOf(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `SulfuricBoneReef` (org/betterx/betternether/world/biomes/SulfuricBoneReef.java)
    pub mod sulfuric_bone_reef {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // SulfuricBoneReef.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // SulfuricBoneReef.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // SulfuricBoneReef.subBiomeOf(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `UpsideDownForest` (org/betterx/betternether/world/biomes/UpsideDownForest.java)
    pub mod upside_down_forest {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // CEILEING_MOSS: not translated, it uses Minecraft's `MaterialRule`.
        // NETHERRACK_MOSS: not translated, it uses Minecraft's `MaterialRule`.
        // NOISE_CEIL_LAYER: not translated, it uses Minecraft's `MaterialCondition`.
        // UpsideDownForest.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // UpsideDownForest.hasBNStructures(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // UpsideDownForest.hasBNFeatures(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // UpsideDownForest.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `UpsideDownForestCleared` (org/betterx/betternether/world/biomes/UpsideDownForestCleared.java)
    pub mod upside_down_forest_cleared {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // UpsideDownForestCleared.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // UpsideDownForestCleared.hasBNStructures(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // UpsideDownForestCleared.hasBNFeatures(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // UpsideDownForestCleared.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // UpsideDownForestCleared.subBiomeOf(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `NetherBiomeBuilder` (org/betterx/betternether/world/biomes/util/NetherBiomeBuilder.java)
    pub mod nether_biome_builder {
        use super::*;
        // The class itself isn't translated: it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder). Its static parts are below.
        pub const GEN_CHANCE_FACTOR: f32 = 1.25f32;
        // BEDROCK: not translated, it uses Minecraft's `MaterialRule`.
        // ON_FLOOR: not translated, it uses Minecraft's `MaterialCondition`.
        // UNDER_FLOOR: not translated, it uses Minecraft's `MaterialCondition`.
        // ON_CEILING: not translated, it uses Minecraft's `MaterialCondition`.
        // UNDER_CEILING: not translated, it uses Minecraft's `MaterialCondition`.
        // BEDROCK_BOTTOM: not translated, it uses Minecraft's `VerticalGradientCondition`.
        // BEDROCK_TOP: not translated, it uses Minecraft's `VerticalGradientCondition`.
        // NetherBiomeBuilder.NetherBiomeBuilder(2): not translated, its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)).
        // NetherBiomeBuilder.addVanillaStructures(0): not translated, its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)).
        // NetherBiomeBuilder.addVanillaFeatures(0): not translated, its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)).
        // NetherBiomeBuilder.addBNStructures(0): not translated, its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)).
        // NetherBiomeBuilder.configure(1): not translated, its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)).
        // NetherBiomeBuilder.createKey(1): not translated, it's generic.
        // NetherBiomeBuilder.registerBiomeData(1): not translated, its class isn't translated (it extends `AbstractWoverBiomeBuilder` (de.ambertation.wover.generator.api.biomesource.WoverBiomeBuilder)).
    }

    /// `NetherBiomeKey` (org/betterx/betternether/world/biomes/util/NetherBiomeKey.java)
    pub mod nether_biome_key {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // NetherBiomeKey.NetherBiomeKey(1): not translated, its class isn't translated (it's generic).
        // NetherBiomeKey.bootstrap(1): not translated, its class isn't translated (it's generic).
        // NetherBiomeKey.bootstrap(2): not translated, its class isn't translated (it's generic).
    }

    /// `WartForest` (org/betterx/betternether/world/biomes/WartForest.java)
    pub mod wart_forest {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // WartForest.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // WartForest.spawnWeight(1): not translated, it's generic.
        // WartForest.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // WartForest.withEdgeBiome(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
    }

    /// `WartForestEdge` (org/betterx/betternether/world/biomes/WartForestEdge.java)
    pub mod wart_forest_edge {
        use super::*;
        // The class itself isn't translated: it extends `NetherBiomeConfig` (org.betterx.betternether.world). Its static parts are below.
        // WartForestEdge.addCustomBuildData(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // WartForestEdge.surface(1): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // WartForestEdge.hasNetherCity(0): not translated, its class isn't translated (it extends `NetherBiomeConfig` (org.betterx.betternether.world)).
        // WartForestEdge.spawnWeight(1): not translated, it's generic.
    }

    /// `BNWorldGenerator` (org/betterx/betternether/world/BNWorldGenerator.java)
    pub mod bn_world_generator {
        use super::*;
        // The class itself isn't translated: it uses StructureGeneratorThreadContext, which isn't translated (it uses Minecraft's `MutableBlockPos`). Its static parts are below.
        pub fn on_mod_init() {
        }
    }

    /// `AnchorTreeBranchFeature` (org/betterx/betternether/world/features/AnchorTreeBranchFeature.java)
    pub mod anchor_tree_branch_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub static CURVE_X: LazyLock<Vec<f32>> = LazyLock::new(|| vec![9.0f32, 7.0f32, 1.5f32, 0.5f32, 3.0f32, 7.0f32]);
        pub static CURVE_Y: LazyLock<Vec<f32>> = LazyLock::new(|| vec![-20.0f32, -17.0f32, -12.0f32, -4.0f32, 0.0f32, 2.0f32]);
        pub const MIDDLE_Y: i32 = 10;
        // AnchorTreeBranchFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeBranchFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeBranchFeature.grow(6): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeBranchFeature.safeSetIfAir(4): not translated, it uses Minecraft's `LevelAccessor`.
        // AnchorTreeBranchFeature.safeSet(3): not translated, it uses Minecraft's `LevelAccessor`.
        // AnchorTreeBranchFeature.updateDistances(2): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeBranchFeature.updateSDFFrom(2): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeBranchFeature.line(9): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeBranchFeature.crown(7): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeBranchFeature.canReplace(1): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeBranchFeature.grow(3): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `AnchorTreeFeature` (org/betterx/betternether/world/features/AnchorTreeFeature.java)
    pub mod anchor_tree_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        // wallPlants: not translated, it uses Minecraft's `Block`.
        // FOLIAGE: not translated, it uses AnchorTreeBranchFeature, which isn't translated (it extends `ContextFeature`).
        pub const SHAPED_FOR_DEPTH: i32 = 128;
        pub const CITY_CLEARANCE: i32 = 23;
        // AnchorTreeFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        pub fn to_middle(val: i32) -> i32 {
            return val + (7 - (val & 15));
        }
        // AnchorTreeFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeFeature.grow(3): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeFeature.growFoliage(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeFeature.canGenerate(1): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeFeature.grow(7): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeFeature.buildBigCircle(11): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeFeature.drawLine(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeFeature.lerp(3): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeFeature.lerpCos(4): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeFeature.lcos(1): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeFeature.line(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeFeature.cylinder(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeFeature.circleLinesEnds(10): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeFeature.makeMushroom(5): not translated, it uses Minecraft's `ServerLevelAccessor`.
    }

    /// `AnchorTreeRootFeature` (org/betterx/betternether/world/features/AnchorTreeRootFeature.java)
    pub mod anchor_tree_root_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // LUCIS: not translated, it uses LucisFeature, which isn't translated (it extends `ContextFeature`).
        // AnchorTreeRootFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeRootFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeRootFeature.canReplace(1): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeRootFeature.buildLine(3): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeRootFeature.lerpCos(3): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeRootFeature.lcos(1): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeRootFeature.lineParable(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // AnchorTreeRootFeature.sphere(3): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `BigBrownMushroomFeature` (org/betterx/betternether/world/features/BigBrownMushroomFeature.java)
    pub mod big_brown_mushroom_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // MIDDLE: not translated, it uses Minecraft's `BlockState`.
        // BOTTOM: not translated, it uses Minecraft's `BlockState`.
        // TOP: not translated, it uses Minecraft's `BlockState`.
        // SIDE_N: not translated, it uses Minecraft's `BlockState`.
        // SIDE_S: not translated, it uses Minecraft's `BlockState`.
        // SIDE_E: not translated, it uses Minecraft's `BlockState`.
        // SIDE_W: not translated, it uses Minecraft's `BlockState`.
        // CORNER_N: not translated, it uses Minecraft's `BlockState`.
        // CORNER_W: not translated, it uses Minecraft's `BlockState`.
        // CORNER_E: not translated, it uses Minecraft's `BlockState`.
        // CORNER_S: not translated, it uses Minecraft's `BlockState`.
        // BigBrownMushroomFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        // BigBrownMushroomFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `CavesFeature` (org/betterx/betternether/world/features/CavesFeature.java)
    pub mod caves_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // caves: not translated, it uses StructureCaves, which isn't translated (it implements `IStructure`).
        // CavesFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // CavesFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // CavesFeature.onLoad(1): not translated, it uses CavesFeature.caves, which isn't translated (it uses StructureCaves, which isn't translated (it implements `IStructure`)).
        // CavesFeature.isInCave(3): not translated, it uses CavesFeature.caves, which isn't translated (it uses StructureCaves, which isn't translated (it implements `IStructure`)).
    }

    /// `GloomwoodTreeConfiguration` (org/betterx/betternether/world/features/configs/GloomwoodTreeConfiguration.java)
    pub mod gloomwood_tree_configuration {
        use super::*;
        // The class itself isn't translated: it extends `NaturalTreeConfiguration`. Its static parts are below.
        pub const SPACING: i32 = 9;
        pub const SOLITARY_BLEACHED_CHANCE: f32 = 0.33f32;
        // NATURAL: not translated, it uses GloomwoodTreeConfiguration, which isn't translated (it extends `NaturalTreeConfiguration`).
        // GloomwoodTreeConfiguration.codecFor(1): not translated, it uses Minecraft's `MapCodec`.
        // GloomwoodTreeConfiguration.GloomwoodTreeConfiguration(3): not translated, its class isn't translated (it extends `NaturalTreeConfiguration`).
        // GloomwoodTreeConfiguration.natural(0): not translated, it uses GloomwoodTreeConfiguration, which isn't translated (it extends `NaturalTreeConfiguration`).
        // GloomwoodTreeConfiguration.bleachedSometimes(1): not translated, it uses GloomwoodTreeConfiguration, which isn't translated (it extends `NaturalTreeConfiguration`).
    }

    /// `NaturalTreeConfiguration` (org/betterx/betternether/world/features/configs/NaturalTreeConfiguration.java)
    pub mod natural_tree_configuration {
        use super::*;
        pub static NATURAL: LazyLock<natural_tree_configuration::NaturalTreeConfiguration> = LazyLock::new(|| natural_tree_configuration::NaturalTreeConfiguration::new(true, 7));
        pub static USER: LazyLock<natural_tree_configuration::NaturalTreeConfiguration> = LazyLock::new(|| natural_tree_configuration::NaturalTreeConfiguration::new(false, 7));
        pub static NATURAL_LARGE: LazyLock<natural_tree_configuration::NaturalTreeConfiguration> = LazyLock::new(|| natural_tree_configuration::NaturalTreeConfiguration::new(true, 13));
        pub static USER_LARGE: LazyLock<natural_tree_configuration::NaturalTreeConfiguration> = LazyLock::new(|| natural_tree_configuration::NaturalTreeConfiguration::new(false, 13));
        #[derive(Clone, Debug)]
        pub struct NaturalTreeConfiguration {
            pub natural: bool,
            pub distance: i32,
            pub man_dist: i32,
        }
        impl NaturalTreeConfiguration {
            pub fn new(natural: bool, distance: i32) -> Self {
                let mut s = Self { natural: natural, distance: distance, man_dist: 0i32 };
                s.natural = natural;
                s.distance = distance;
                s.man_dist = ((((distance as f64) * 1.5f64).ceil()) as i32);
                s
            }
        }
        // NaturalTreeConfiguration.mapCodecFor(1): not translated, it's generic.
        pub fn natural() -> natural_tree_configuration::NaturalTreeConfiguration {
            return (*natural_tree_configuration::NATURAL).clone();
        }
        pub fn user_grown() -> natural_tree_configuration::NaturalTreeConfiguration {
            return (*natural_tree_configuration::USER).clone();
        }
        pub fn natural_large() -> natural_tree_configuration::NaturalTreeConfiguration {
            return (*natural_tree_configuration::NATURAL_LARGE).clone();
        }
        pub fn user_grown_large() -> natural_tree_configuration::NaturalTreeConfiguration {
            return (*natural_tree_configuration::USER_LARGE).clone();
        }
    }

    /// `ContextFeature` (org/betterx/betternether/world/features/ContextFeature.java)
    pub mod context_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // ContextFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // ContextFeature.place(5): not translated, it has no body.
    }

    /// `CrystalFeature` (org/betterx/betternether/world/features/CrystalFeature.java)
    pub mod crystal_feature {
        use super::*;
        // The class itself isn't translated: it extends `NetherSurfaceFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // PALETTES: not translated, it uses Minecraft's `Block`.
        pub static SQRT05: LazyLock<f64> = LazyLock::new(|| 0.5f64.sqrt());
        pub static MAX_ANGLE_X: LazyLock<f32> = LazyLock::new(|| (45.0f64.to_radians() as f32));
        pub static MAX_ANGLE_Y: LazyLock<f32> = LazyLock::new(|| ((std::f64::consts::PI * 2.0f64) as f32));
        pub static MAX_ANGLE_Z: LazyLock<f32> = LazyLock::new(|| (30.0f64.to_radians() as f32));
        // NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        // CrystalFeature.codec(0): not translated, its class isn't translated (it extends `NetherSurfaceFeature`).
        // CrystalFeature.generate(5): not translated, its class isn't translated (it extends `NetherSurfaceFeature`).
        // CrystalFeature.depth(4): not translated, its class isn't translated (it extends `NetherSurfaceFeature`).
        // CrystalFeature.isInside(4): not translated, its class isn't translated (it extends `NetherSurfaceFeature`).
        // CrystalFeature.isNotEdge(4): not translated, its class isn't translated (it extends `NetherSurfaceFeature`).
        // CrystalFeature.dodecahedronSDF(3): not translated, its class isn't translated (it extends `NetherSurfaceFeature`).
        // CrystalFeature.rigidNoise(2): not translated, its class isn't translated (it extends `NetherSurfaceFeature`).
        // CrystalFeature.getState(2): not translated, its class isn't translated (it extends `NetherSurfaceFeature`).
    }

    /// `GloomwoodTreeFeature` (org/betterx/betternether/world/features/GloomwoodTreeFeature.java)
    pub mod gloomwood_tree_feature {
        use super::*;
        // The class itself isn't translated: it extends `NonOverlappingFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub const MAX_CLEARANCE_SCAN: i32 = 40;
        pub const CROWN_MARGIN: i32 = 2;
        pub const MOLTEN_ROOT_CHANCE: i32 = 2;
        pub const ROOT_RADIUS: i32 = 2;
        pub const MIN_DEBRIS_DEPTH: i32 = 1;
        pub const MAX_DEBRIS_DEPTH: i32 = 4;
        pub const MOLTEN_COLUMN_CHANCE: f32 = 0.55f32;
        // MIN_CLEARANCE: not translated, it calls Size.values, which isn't in the mod's source.
        // GloomwoodTreeFeature.GloomwoodTreeFeature(1): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // GloomwoodTreeFeature.bleachedChance(0): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // GloomwoodTreeFeature.codec(0): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // GloomwoodTreeFeature.isStructure(1): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // GloomwoodTreeFeature.isGround(1): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // GloomwoodTreeFeature.grow(5): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // GloomwoodTreeFeature.moltenRoot(4): not translated, it uses Minecraft's `ServerLevelAccessor`.
        // GloomwoodTreeFeature.ancientDebrisRoots(4): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // GloomwoodTreeFeature.hasRoomToGrow(2): not translated, it uses Minecraft's `BlockGetter`.
        // GloomwoodTreeFeature.clearanceAbove(2): not translated, it uses Minecraft's `BlockGetter`.
        // GloomwoodTreeFeature.grow(3): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // GloomwoodTreeFeature.builder(3): not translated, it uses Size, which isn't translated (its constants carry values).
        // GloomwoodTreeFeature.palette(4): not translated, it uses TreePalette, which isn't translated (it's an interface).
    }

    /// `GloomwoodTreeFeature.Size` (org/betterx/betternether/world/features/GloomwoodTreeFeature.java)
    pub mod gloomwood_tree_feature_size {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // GloomwoodTreeFeature.Size.Size(21): not translated, its class isn't translated (its constants carry values).
    }

    /// `JellyfishMushroomFeature` (org/betterx/betternether/world/features/JellyfishMushroomFeature.java)
    pub mod jellyfish_mushroom_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // JellyfishMushroomFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        // JellyfishMushroomFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // JellyfishMushroomFeature.growHere(3): not translated, its class isn't translated (it extends `ContextFeature`).
        // JellyfishMushroomFeature.growSmall(2): not translated, its class isn't translated (it extends `ContextFeature`).
        // JellyfishMushroomFeature.growMedium(2): not translated, its class isn't translated (it extends `ContextFeature`).
        // JellyfishMushroomFeature.grow(3): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `LucisFeature` (org/betterx/betternether/world/features/LucisFeature.java)
    pub mod lucis_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CENTER: not translated, it uses Minecraft's `BlockState`.
        // SIDE: not translated, it uses Minecraft's `BlockState`.
        // CORNER: not translated, it uses Minecraft's `BlockState`.
        // LucisFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        // LucisFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // LucisFeature.grow(4): not translated, its class isn't translated (it extends `ContextFeature`).
        // LucisFeature.canReplace(1): not translated, its class isn't translated (it extends `ContextFeature`).
        // LucisFeature.canGenerate(2): not translated, its class isn't translated (it extends `ContextFeature`).
        // LucisFeature.grow(3): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `MushroomFirFeature` (org/betterx/betternether/world/features/MushroomFirFeature.java)
    pub mod mushroom_fir_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SIDE_BIG_N: not translated, it uses Minecraft's `BlockState`.
        // SIDE_BIG_W: not translated, it uses Minecraft's `BlockState`.
        // SIDE_BIG_E: not translated, it uses Minecraft's `BlockState`.
        // SIDE_SMALL_S: not translated, it uses Minecraft's `BlockState`.
        // SIDE_SMALL_N: not translated, it uses Minecraft's `BlockState`.
        // SIDE_SMALL_W: not translated, it uses Minecraft's `BlockState`.
        // SIDE_SMALL_E: not translated, it uses Minecraft's `BlockState`.
        // END: not translated, it uses Minecraft's `BlockState`.
        // BLUE_MYCELIUM: not translated, it uses Minecraft's `BlockState`.
        // SIDE_BIG_S: not translated, it uses Minecraft's `BlockState`.
        // TOP: not translated, it uses Minecraft's `BlockState`.
        // MIDDLE: not translated, it uses Minecraft's `BlockState`.
        // BOTTOM: not translated, it uses Minecraft's `BlockState`.
        // MushroomFirFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        // MushroomFirFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // MushroomFirFeature.grow(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // MushroomFirFeature.grow(3): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `NetherSakuraBushFeature` (org/betterx/betternether/world/features/NetherSakuraBushFeature.java)
    pub mod nether_sakura_bush_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherSakuraBushFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        // NetherSakuraBushFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // NetherSakuraBushFeature.setIfAir(3): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `NetherSakuraFeature` (org/betterx/betternether/world/features/NetherSakuraFeature.java)
    pub mod nether_sakura_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherSakuraFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        // NetherSakuraFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // NetherSakuraFeature.grow(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // NetherSakuraFeature.updateSDFFrom(2): not translated, its class isn't translated (it extends `ContextFeature`).
        // NetherSakuraFeature.updateDistances(2): not translated, its class isn't translated (it extends `ContextFeature`).
        // NetherSakuraFeature.crown(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // NetherSakuraFeature.canReplace(1): not translated, its class isn't translated (it extends `ContextFeature`).
        // NetherSakuraFeature.grow(3): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `NetherSurfaceFeature` (org/betterx/betternether/world/features/NetherSurfaceFeature.java)
    pub mod nether_surface_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // NetherSurfaceFeature.isValidSurface(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // NetherSurfaceFeature.generate(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // NetherSurfaceFeature.generate(5): not translated, it has no body.
        // NetherSurfaceFeature.minHeight(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // NetherSurfaceFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `NetherThreadDataStorage` (org/betterx/betternether/world/features/NetherThreadDataStorage.java)
    pub mod nether_thread_data_storage {
        use super::*;
        // WORLD_GENERATOR: not translated, it uses `ThreadLocal`.
        #[derive(Clone, Debug)]
        pub struct NetherThreadDataStorage {
        }
        impl NetherThreadDataStorage {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherThreadDataStorage.generatorForThread(0): not translated, it uses NetherThreadDataStorage.WORLD_GENERATOR, which isn't translated (it uses `ThreadLocal`).
    }

    /// `NonOverlappingFeature` (org/betterx/betternether/world/features/NonOverlappingFeature.java)
    pub mod non_overlapping_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // NonOverlappingFeature.NonOverlappingFeature(1): not translated, its class isn't translated (it extends `ContextFeature`).
        // NonOverlappingFeature.isStructure(1): not translated, it has no body.
        // NonOverlappingFeature.isGround(1): not translated, it has no body.
        // NonOverlappingFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // NonOverlappingFeature.grow(5): not translated, it has no body.
        // NonOverlappingFeature.noObjNear(3): not translated, its class isn't translated (it extends `ContextFeature`).
        // NonOverlappingFeature.isInside(3): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `OldWillowTree` (org/betterx/betternether/world/features/OldWillowTree.java)
    pub mod old_willow_tree {
        use super::*;
        // The class itself isn't translated: it extends `NonOverlappingFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub const CROWN_SQUASH: f32 = 0.70710677f32;
        pub static CURVE_X: LazyLock<Vec<f32>> = LazyLock::new(|| vec![9.0f32, 7.0f32, 1.5f32, 0.5f32, 3.0f32, 7.0f32]);
        pub static CURVE_Y: LazyLock<Vec<f32>> = LazyLock::new(|| vec![20.0f32, 17.0f32, 12.0f32, 4.0f32, 0.0f32, -2.0f32]);
        // wallPlants: not translated, it uses Minecraft's `Block`.
        // OldWillowTree.OldWillowTree(1): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // OldWillowTree.codec(0): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // OldWillowTree.grow(5): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // OldWillowTree.place(5): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // OldWillowTree.isStructure(1): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // OldWillowTree.isGround(1): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // OldWillowTree.line(9): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // OldWillowTree.sphere(3): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // OldWillowTree.crown(6): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // OldWillowTree.grow(3): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
    }

    /// `PathsFeature` (org/betterx/betternether/world/features/PathsFeature.java)
    pub mod paths_feature {
        use super::*;
        // The class itself isn't translated: it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // paths: not translated, it uses StructurePath, which isn't translated (it implements `IStructure`).
        // PathsFeature.codec(0): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // PathsFeature.place(4): not translated, its class isn't translated (it extends `DefaultFeature` (org.betterx.bclib.api.v2.levelgen.features.features)).
        // PathsFeature.onLoad(1): not translated, it uses PathsFeature.paths, which isn't translated (it uses StructurePath, which isn't translated (it implements `IStructure`)).
    }

    /// `RubeusBushFeature` (org/betterx/betternether/world/features/RubeusBushFeature.java)
    pub mod rubeus_bush_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // RubeusBushFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        // RubeusBushFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // RubeusBushFeature.setIfAir(3): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `RubeusTreeFeature` (org/betterx/betternether/world/features/RubeusTreeFeature.java)
    pub mod rubeus_tree_feature {
        use super::*;
        // The class itself isn't translated: it extends `NonOverlappingFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub static CURVE_X: LazyLock<Vec<f32>> = LazyLock::new(|| vec![9.0f32, 7.0f32, 1.5f32, 0.5f32, 3.0f32, 7.0f32]);
        pub static CURVE_Y: LazyLock<Vec<f32>> = LazyLock::new(|| vec![20.0f32, 17.0f32, 12.0f32, 4.0f32, 0.0f32, -2.0f32]);
        pub const MIDDLE_Y: i32 = 10;
        // RubeusTreeFeature.RubeusTreeFeature(1): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // RubeusTreeFeature.codec(0): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // RubeusTreeFeature.isStructure(1): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // RubeusTreeFeature.isGround(1): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // RubeusTreeFeature.place(5): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // RubeusTreeFeature.updateDistances(3): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // RubeusTreeFeature.grow(5): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // RubeusTreeFeature.line(9): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // RubeusTreeFeature.crown(7): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // RubeusTreeFeature.setCondition(6): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // RubeusTreeFeature.setIfAir(3): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // RubeusTreeFeature.setIfGroundOrAir(3): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // RubeusTreeFeature.setIfAirLeaves(3): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // RubeusTreeFeature.grow(3): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
    }

    /// `SoulLilyFeature` (org/betterx/betternether/world/features/SoulLilyFeature.java)
    pub mod soul_lily_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SoulLilyFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        // SoulLilyFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // SoulLilyFeature.growTree(3): not translated, its class isn't translated (it extends `ContextFeature`).
        // SoulLilyFeature.growSmall(2): not translated, its class isn't translated (it extends `ContextFeature`).
        // SoulLilyFeature.growMedium(2): not translated, its class isn't translated (it extends `ContextFeature`).
        // SoulLilyFeature.growBig(2): not translated, its class isn't translated (it extends `ContextFeature`).
        // SoulLilyFeature.isAirSides(2): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `TwistedVinesFeature` (org/betterx/betternether/world/features/TwistedVinesFeature.java)
    pub mod twisted_vines_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TwistedVinesFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        // TwistedVinesFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // TwistedVinesFeature.canPlaceAt(2): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `WartBushFeature` (org/betterx/betternether/world/features/WartBushFeature.java)
    pub mod wart_bush_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // DIRS: not translated, it uses Minecraft's `Direction`.
        // WartBushFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        // WartBushFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // WartBushFeature.setSeed(3): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `WartCapFeature` (org/betterx/betternether/world/features/WartCapFeature.java)
    pub mod wart_cap_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // INSIDE: not translated, it uses Minecraft's `BlockState`.
        // SKIN: not translated, it uses Minecraft's `BlockState`.
        // WartCapFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        // WartCapFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // WartCapFeature.isWall(2): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `WartTreeFeature` (org/betterx/betternether/world/features/WartTreeFeature.java)
    pub mod wart_tree_feature {
        use super::*;
        // The class itself isn't translated: it extends `NonOverlappingFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // WART_BLOCK: not translated, it uses Minecraft's `BlockState`.
        // HORIZONTAL: not translated, it uses Minecraft's `Direction`.
        // WartTreeFeature.WartTreeFeature(1): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // WartTreeFeature.codec(0): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // WartTreeFeature.grow(5): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // WartTreeFeature.PlaceRandomSeed(2): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // WartTreeFeature.isReplaceable(1): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // WartTreeFeature.isWart(1): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // WartTreeFeature.isStructure(1): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // WartTreeFeature.isGround(1): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
        // WartTreeFeature.grow(3): not translated, its class isn't translated (it extends `NonOverlappingFeature`).
    }

    /// `WillowBushFeature` (org/betterx/betternether/world/features/WillowBushFeature.java)
    pub mod willow_bush_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // WillowBushFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        // WillowBushFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // WillowBushFeature.setIfAir(3): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `WillowTreeFeature` (org/betterx/betternether/world/features/WillowTreeFeature.java)
    pub mod willow_tree_feature {
        use super::*;
        // The class itself isn't translated: it extends `ContextFeature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // HOR: not translated, it uses Minecraft's `Direction`.
        // WillowTreeFeature.codec(0): not translated, its class isn't translated (it extends `ContextFeature`).
        // WillowTreeFeature.place(5): not translated, its class isn't translated (it extends `ContextFeature`).
        // WillowTreeFeature.grow(3): not translated, its class isn't translated (it extends `ContextFeature`).
        // WillowTreeFeature.branch(7): not translated, its class isn't translated (it extends `ContextFeature`).
        // WillowTreeFeature.vine(3): not translated, its class isn't translated (it extends `ContextFeature`).
    }

    /// `NetherBiome` (org/betterx/betternether/world/NetherBiome.java)
    pub mod nether_biome {
        use super::*;
        // The class itself isn't translated: it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NetherBiome.NetherBiome(9): not translated, its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)).
        // NetherBiome.codec(0): not translated, its class isn't translated (it extends `WoverBiomeData` (de.ambertation.wover.generator.api.biomesource)).
    }

    /// `NetherBiomeConfig` (org/betterx/betternether/world/NetherBiomeConfig.java)
    pub mod nether_biome_config {
        use super::*;
        // NETHERRACK: not translated, it uses Minecraft's `MaterialRule`.
        // BEDROCK_BOTTOM: not translated, it uses Minecraft's `MaterialRule`.
        // BEDROCK_TOP: not translated, it uses Minecraft's `MaterialRule`.
        #[derive(Clone, Debug)]
        pub struct NetherBiomeConfig {
        }
        impl NetherBiomeConfig {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // NetherBiomeConfig.subBiomeOf(0): not translated, it uses Minecraft's `ResourceKey`.
            // NetherBiomeConfig.withEdgeBiome(0): not translated, it uses Minecraft's `ResourceKey`.
            pub fn has_vanilla_features(&self) -> bool {
                return true;
            }
            pub fn has_vanilla_ores(&self) -> bool {
                return true;
            }
            pub fn has_stalactites(&self) -> bool {
                return false;
            }
            pub fn has_default_ores(&self) -> bool {
                return true;
            }
            pub fn has_nether_city(&self) -> bool {
                return true;
            }
            pub fn has_vanilla_structures(&self) -> bool {
                return true;
            }
            pub fn has_bn_structures(&self) -> bool {
                return true;
            }
            pub fn has_bn_features(&self) -> bool {
                return true;
            }
            // NetherBiomeConfig.spawnWeight(1): not translated, it's generic.
            // NetherBiomeConfig.addCustomBuildData(1): not translated, it has no body.
            // NetherBiomeConfig.surface(1): not translated, it uses `BiomeSurfaceRuleBuilder` (de.ambertation.wover.biome.api.builder).
        }
    }

    /// `AvoidNetherCity` (org/betterx/betternether/world/structures/city/AvoidNetherCity.java)
    pub mod avoid_nether_city {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `PlacementFilter`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AvoidNetherCity.AvoidNetherCity(2): not translated, its class isn't translated (it implements Minecraft's `PlacementFilter`).
        // AvoidNetherCity.shouldPlace(3): not translated, its class isn't translated (it implements Minecraft's `PlacementFilter`).
        // AvoidNetherCity.codec(0): not translated, its class isn't translated (it implements Minecraft's `PlacementFilter`).
    }

    /// `BoundingBox2D` (org/betterx/betternether/world/structures/city/BoundingBox2D.java)
    pub mod bounding_box2_d {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BoundingBox2D {
            pub x1: i32,
            pub x2: i32,
            pub z1: i32,
            pub z2: i32,
        }
        impl BoundingBox2D {
            pub fn new_4(x1: i32, z1: i32, x2: i32, z2: i32) -> Self {
                let mut s = Self { x1: x1, x2: x2, z1: z1, z2: z2 };
                s.x1 = x1;
                s.x2 = x2;
                s.z1 = z1;
                s.z2 = z2;
                s
            }
            // BoundingBox2D.BoundingBox2D(3): not translated, it uses Minecraft's `BlockPos`.
            // BoundingBox2D.BoundingBox2D(1): not translated, it uses Minecraft's `BlockPos`.
            pub fn is_colliding(&self, bb: &bounding_box2_d::BoundingBox2D) -> bool {
                let col_x: bool = (bb.x1 < self.x2) && (self.x1 < bb.x2);
                let col_z: bool = (bb.z1 < self.z2) && (self.z1 < bb.z2);
                return col_x && col_z;
            }
            // BoundingBox2D.offset(1): not translated, it uses Minecraft's `BlockPos`.
            // BoundingBox2D.offsetNegative(1): not translated, it uses Minecraft's `BlockPos`.
            pub fn to_string(&self) -> String {
                return format!("{} {} {} {}", self.x1, self.z1, self.x2, self.z2);
            }
            // BoundingBox2D.rotate(1): not translated, it uses Minecraft's `Rotation`.
            // BoundingBox2D.getCenter(0): not translated, it uses Minecraft's `BlockPos`.
            pub fn get_side_x(&self) -> i32 {
                return self.x2 - self.x1;
            }
            pub fn get_side_z(&self) -> i32 {
                return self.z2 - self.z1;
            }
            pub fn get_min_x(&self) -> i32 {
                return self.x1;
            }
            pub fn get_max_x(&self) -> i32 {
                return self.x2;
            }
            pub fn get_min_z(&self) -> i32 {
                return self.z1;
            }
            pub fn get_max_z(&self) -> i32 {
                return self.z2;
            }
        }
    }

    /// `BuildingStructureProcessor` (org/betterx/betternether/world/structures/city/BuildingStructureProcessor.java)
    pub mod building_structure_processor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StructureProcessor`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // BuildingStructureProcessor.BuildingStructureProcessor(1): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // BuildingStructureProcessor.setState(2): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // BuildingStructureProcessor.processBlock(6): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // BuildingStructureProcessor.codec(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
    }

    /// `CityGenerator` (org/betterx/betternether/world/structures/city/CityGenerator.java)
    pub mod city_generator {
        use super::*;
        // The class itself isn't translated: it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)). Its static parts are below.
        // CityGenerator.CityGenerator(0): not translated, its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))).
        // CityGenerator.addBuildingToList(2): not translated, its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))).
        // CityGenerator.addBuildingToList(3): not translated, its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))).
        // CityGenerator.placeCenterBuilding(5): not translated, its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))).
        // CityGenerator.attachBuildings(3): not translated, its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))).
        // CityGenerator.closeRoads(3): not translated, its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))).
        // CityGenerator.generate(3): not translated, its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))).
        // CityGenerator.noCollisions(1): not translated, its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))).
        // CityGenerator.getBuildings(0): not translated, its class isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))).
    }

    /// `CityStructure` (org/betterx/betternether/world/structures/city/CityStructure.java)
    pub mod city_structure {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Structure`. Its static parts are below.
        // generator: not translated, it uses CityGenerator, which isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api))).
        pub const RADIUS: i32 = 64;
        pub const DEFAULT_HEIGHT: i32 = 35;
        pub const BASTION_REACH: i32 = 12;
        // CityStructure.CityStructure(1): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // CityStructure.initGenerator(0): not translated, it uses CityStructure.generator, which isn't translated (it uses CityGenerator, which isn't translated (it uses StructureCityBuilding, which isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)))).
        // CityStructure.findGenerationPoint(1): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // CityStructure.generatePieces(2): not translated, it uses Minecraft's `StructurePiecesBuilder`.
        // CityStructure.getCenter(4): not translated, it uses Minecraft's `BlockPos`.
        // CityStructure.buildingsWithin(3): not translated, it uses Minecraft's `ServerLevelAccessor`.
        // CityStructure.buildingsIntersect(3): not translated, it uses Minecraft's `ServerLevelAccessor`.
        // CityStructure.cityWithin(3): not translated, it uses Minecraft's `ServerLevelAccessor`.
        // CityStructure.intersects(3): not translated, it uses Minecraft's `ServerLevelAccessor`.
        // CityStructure.type(0): not translated, its class isn't translated (it extends Minecraft's `Structure`).
    }

    /// `NearbyBastions` (org/betterx/betternether/world/structures/city/NearbyBastions.java)
    pub mod nearby_bastions {
        use super::*;
        pub static BASTION: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "bastion_remnant"));
        #[derive(Clone, Debug)]
        pub struct NearbyBastions {
        }
        impl NearbyBastions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NearbyBastions.startsNear(3): not translated, it uses Minecraft's `GenerationStub`.
        // NearbyBastions.selectedAt(3): not translated, it uses Minecraft's `GenerationContext`.
    }

    /// `NearbyBastions.Selected` (org/betterx/betternether/world/structures/city/NearbyBastions.java)
    pub mod nearby_bastions_selected {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Holder`. Its static parts are below.
        // NearbyBastions.Selected.Selected(2): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // NearbyBastions.Selected.structure(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // NearbyBastions.Selected.stub(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
    }

    /// `CityPalette` (org/betterx/betternether/world/structures/city/palette/CityPalette.java)
    pub mod city_palette {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Block`. Its static parts are below.
        // CityPalette.CityPalette(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getName(0): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.putBlocks(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addFoundationBlocks(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addFoundationSlabs(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addFoundationStairs(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addFoundationWalls(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addRoofBlocks(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addRoofSlabs(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addRoofStairs(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addPlanksBlocks(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addPlanksSlabs(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addPlanksStairs(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addFences(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addWalls(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addGates(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addLogs(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addBark(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addStoneBlocks(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addStoneSlabs(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addStoneStairs(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addGlowingBlocks(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addWallLights(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addCeilingLights(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addFloorLights(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addDoors(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addTrapdoors(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addStonePlates(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addWoodPlates(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addGlassBlocks(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addGlassPanes(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.addPotsPanes(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getRandomBlock(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getFullState(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getSlabState(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getStairState(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.copySlab(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.copyStair(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.copyWall(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.copyFence(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.copyGate(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.copyPillar(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.copyLanternWall(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.copyLanternCeiling(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.copyDoor(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.copyTrapdoor(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.copyPane(2): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getFoundationBlock(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getFoundationSlab(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getFoundationStair(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getFoundationWall(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getRoofBlock(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getRoofSlab(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getRoofStair(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getPlanksBlock(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getPlanksSlab(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getPlanksStair(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getFence(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getWall(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getGate(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getLog(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getBark(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getStoneBlock(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getStoneSlab(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getStoneStair(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getGlowingBlock(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getWallLight(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getCeilingLight(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getFloorLight(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getDoor(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getTrapdoor(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getWoodenPlate(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getStonePlate(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getGlassBlock(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getGlassPane(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getPot(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
        // CityPalette.getPlant(1): not translated, its class isn't translated (it uses Minecraft's `Block`).
    }

    /// `Palettes` (org/betterx/betternether/world/structures/city/palette/Palettes.java)
    pub mod palettes {
        use super::*;
        // REGISTRY: not translated, it uses CityPalette, which isn't translated (it uses Minecraft's `Block`).
        // PALETTES: not translated, it uses CityPalette, which isn't translated (it uses Minecraft's `Block`).
        // EMPTY: not translated, it uses CityPalette, which isn't translated (it uses Minecraft's `Block`).
        // RED: not translated, it uses CityPalette, which isn't translated (it uses Minecraft's `Block`).
        #[derive(Clone, Debug)]
        pub struct Palettes {
        }
        impl Palettes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Palettes.register(1): not translated, it uses CityPalette, which isn't translated (it uses Minecraft's `Block`).
        // Palettes.getPalette(1): not translated, it uses CityPalette, which isn't translated (it uses Minecraft's `Block`).
        // Palettes.getRandom(1): not translated, it uses CityPalette, which isn't translated (it uses Minecraft's `Block`).
    }

    /// `StructureAvoidingProcessor` (org/betterx/betternether/world/structures/city/StructureAvoidingProcessor.java)
    pub mod structure_avoiding_processor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StructureProcessor`. Its static parts are below.
        // AVOIDED: not translated, it uses Minecraft's `TagKey`.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // StructureAvoidingProcessor.StructureAvoidingProcessor(1): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // StructureAvoidingProcessor.collect(3): not translated, it uses Minecraft's `BoundingBox`.
        // StructureAvoidingProcessor.isAvoided(2): not translated, it uses Minecraft's `Registry`.
        // StructureAvoidingProcessor.isEmpty(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // StructureAvoidingProcessor.claimed(1): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // StructureAvoidingProcessor.processBlock(6): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // StructureAvoidingProcessor.codec(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
    }

    /// `StructureCityBuilding` (org/betterx/betternether/world/structures/city/StructureCityBuilding.java)
    pub mod structure_city_building {
        use super::*;
        // The class itself isn't translated: it extends `StructureNBT` (de.ambertation.wover.structure.api). Its static parts are below.
        // AIR: not translated, it uses Minecraft's `BlockState`.
        // StructureCityBuilding.StructureCityBuilding(1): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.StructureCityBuilding(2): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.StructureCityBuilding(2): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.init(0): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.asBlockPos(1): not translated, it uses Minecraft's `BlockPos`.
        // StructureCityBuilding.getDir(1): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.getBoungingBox(0): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.mirrorRotation(1): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.placeInChunk(4): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.getEnds(0): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.getEndsCount(0): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.getOffsettedPos(1): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.getPos(1): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.getRotated(1): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.getRandomRotated(1): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.clone(0): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.rotated(2): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.getYOffset(0): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.getRotation(0): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.getMirror(0): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.setMirror(1): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.getBoundingBox(1): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
        // StructureCityBuilding.setRotation(1): not translated, its class isn't translated (it extends `StructureNBT` (de.ambertation.wover.structure.api)).
    }

    /// `IStructure` (org/betterx/betternether/world/structures/IStructure.java)
    pub mod i_structure {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IStructure.generate(5): not translated, it has no body.
    }

    /// `MegaLavaLakeStructure` (org/betterx/betternether/world/structures/lake/MegaLavaLakeStructure.java)
    pub mod mega_lava_lake_structure {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Structure`. Its static parts are below.
        pub const MIN_RADIUS: i32 = 10;
        pub const MAX_RADIUS: i32 = 18;
        pub const RIM_PROBE_REACH: f64 = 1.3f64;
        pub const MIN_DEPTH: i32 = 3;
        pub const MAX_DEPTH: i32 = 6;
        pub const MAX_RIM_DROP: i32 = 8;
        pub const REQUIRED_HEADROOM: i32 = 5;
        // MegaLavaLakeStructure.MegaLavaLakeStructure(1): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // MegaLavaLakeStructure.type(0): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // MegaLavaLakeStructure.findGenerationPoint(1): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // MegaLavaLakeStructure.rimColumns(3): not translated, it uses Minecraft's `BlockPos`.
    }

    /// `CavePiece` (org/betterx/betternether/world/structures/piece/CavePiece.java)
    pub mod cave_piece {
        use super::*;
        // The class itself isn't translated: it extends `CustomPiece`. Its static parts are below.
        // LAVA: not translated, it uses Minecraft's `BlockState`.
        // NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        // CavePiece.CavePiece(4): not translated, its class isn't translated (it extends `CustomPiece`).
        // CavePiece.CavePiece(2): not translated, its class isn't translated (it extends `CustomPiece`).
        // CavePiece.addAdditionalSaveData(2): not translated, its class isn't translated (it extends `CustomPiece`).
        // CavePiece.postProcess(7): not translated, its class isn't translated (it extends `CustomPiece`).
        // CavePiece.makeBoundingBox(2): not translated, it uses Minecraft's `BoundingBox`.
    }

    /// `CityPiece` (org/betterx/betternether/world/structures/piece/CityPiece.java)
    pub mod city_piece {
        use super::*;
        // The class itself isn't translated: it extends `CustomPiece`. Its static parts are below.
        // CityPiece.CityPiece(4): not translated, its class isn't translated (it extends `CustomPiece`).
        // CityPiece.CityPiece(2): not translated, its class isn't translated (it extends `CustomPiece`).
        // CityPiece.addAdditionalSaveData(2): not translated, its class isn't translated (it extends `CustomPiece`).
        // CityPiece.postProcess(7): not translated, its class isn't translated (it extends `CustomPiece`).
    }

    /// `CustomPiece` (org/betterx/betternether/world/structures/piece/CustomPiece.java)
    pub mod custom_piece {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `StructurePiece`. Its static parts are below.
        // CustomPiece.CustomPiece(3): not translated, its class isn't translated (it extends Minecraft's `StructurePiece`).
        // CustomPiece.CustomPiece(2): not translated, its class isn't translated (it extends Minecraft's `StructurePiece`).
    }

    /// `DestructionPiece` (org/betterx/betternether/world/structures/piece/DestructionPiece.java)
    pub mod destruction_piece {
        use super::*;
        // The class itself isn't translated: it extends `CustomPiece`. Its static parts are below.
        // DestructionPiece.DestructionPiece(2): not translated, its class isn't translated (it extends `CustomPiece`).
        // DestructionPiece.DestructionPiece(2): not translated, its class isn't translated (it extends `CustomPiece`).
        // DestructionPiece.addAdditionalSaveData(2): not translated, its class isn't translated (it extends `CustomPiece`).
        // DestructionPiece.postProcess(7): not translated, its class isn't translated (it extends `CustomPiece`).
        // DestructionPiece.makeBoundingBox(0): not translated, its class isn't translated (it extends `CustomPiece`).
    }

    /// `LavaLakePiece` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java)
    pub mod lava_lake_piece {
        use super::*;
        // The class itself isn't translated: it extends `CustomPiece`. Its static parts are below.
        // LAVA: not translated, it uses Minecraft's `BlockState`.
        // MAGMA: not translated, it uses Minecraft's `BlockState`.
        // NETHERRACK: not translated, it uses Minecraft's `BlockState`.
        pub const SHORE_WIDTH: i32 = 8;
        pub const APRON: f64 = 0.4f64;
        pub const APRON_FILL_DEPTH: i32 = 8;
        pub const BED_SHELL: i32 = 3;
        pub const HEADROOM: i32 = 5;
        pub const OUTLINE_WOBBLE: f64 = 0.18f64;
        pub const WISP_ONE_IN: i32 = 20;
        pub const MIN_WISP_HEIGHT: i32 = 2;
        pub const MAX_WISP_HEIGHT: i32 = 4;
        pub const MOLTEN_AT_WATER: f64 = 0.2f64;
        pub const MOLTEN_FADE: f64 = 0.4f64;
        pub const SCULK_PATCH_THRESHOLD: f64 = 0.305f64;
        pub const MAX_FLOWER_SHARE: f64 = 0.3f64;
        // LavaLakePiece.LavaLakePiece(4): not translated, its class isn't translated (it extends `CustomPiece`).
        // LavaLakePiece.LavaLakePiece(5): not translated, its class isn't translated (it extends `CustomPiece`).
        // LavaLakePiece.LavaLakePiece(2): not translated, its class isn't translated (it extends `CustomPiece`).
        // LavaLakePiece.addAdditionalSaveData(2): not translated, its class isn't translated (it extends `CustomPiece`).
        // LavaLakePiece.lobes(2): not translated, it calls Minecraft's `RandomSource`.create.
        // LavaLakePiece.makeBoundingBox(3): not translated, it uses Minecraft's `BoundingBox`.
        // LavaLakePiece.postProcess(7): not translated, its class isn't translated (it extends `CustomPiece`).
        // LavaLakePiece.shapeAt(2): not translated, its class isn't translated (it extends `CustomPiece`).
        // LavaLakePiece.overburden(4): not translated, its class isn't translated (it extends `CustomPiece`).
        // LavaLakePiece.isOpen(1): not translated, it uses Minecraft's `BlockState`.
        // LavaLakePiece.carveBowl(5): not translated, its class isn't translated (it extends `CustomPiece`).
        // LavaLakePiece.clearAbove(4): not translated, its class isn't translated (it extends `CustomPiece`).
        // LavaLakePiece.buildShore(6): not translated, its class isn't translated (it extends `CustomPiece`).
        // LavaLakePiece.fillDownToGround(5): not translated, its class isn't translated (it extends `CustomPiece`).
        // LavaLakePiece.findSurface(6): not translated, its class isn't translated (it extends `CustomPiece`).
        // LavaLakePiece.shoreGround(3): not translated, its class isn't translated (it extends `CustomPiece`).
        // LavaLakePiece.vegetate(5): not translated, its class isn't translated (it extends `CustomPiece`).
        // LavaLakePiece.placeWisp(3): not translated, its class isn't translated (it extends `CustomPiece`).
    }

    /// `LavaLakePiece.Lobe` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java)
    pub mod lava_lake_piece_lobe {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Lobe {
            pub offset_x: f64,
            pub offset_z: f64,
            pub radius: f64,
        }
        impl Lobe {
            pub fn new(offset_x: f64, offset_z: f64, radius: f64) -> Self {
                let mut s = Self { offset_x: offset_x, offset_z: offset_z, radius: radius };
                s.offset_x = offset_x;
                s.offset_z = offset_z;
                s.radius = radius;
                s
            }
            pub fn reach(&self) -> f64 {
                return (((self.offset_x * self.offset_x) + (self.offset_z * self.offset_z)).sqrt()) + self.radius;
            }
            pub fn offset_x(&self) -> f64 {
                return self.offset_x;
            }
            pub fn offset_z(&self) -> f64 {
                return self.offset_z;
            }
            pub fn radius(&self) -> f64 {
                return self.radius;
            }
        }
    }

    /// `LavaLakePiece.Shape` (org/betterx/betternether/world/structures/piece/LavaLakePiece.java)
    pub mod lava_lake_piece_shape {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Shape {
            pub t: f64,
            pub d_out: f64,
        }
        impl Shape {
            pub fn new(t: f64, d_out: f64) -> Self {
                let mut s = Self { t: t, d_out: d_out };
                s.t = t;
                s.d_out = d_out;
                s
            }
            pub fn t(&self) -> f64 {
                return self.t;
            }
            pub fn d_out(&self) -> f64 {
                return self.d_out;
            }
        }
    }

    /// `LegacyStructureAnchorTree` (org/betterx/betternether/world/structures/plants/LegacyStructureAnchorTree.java)
    pub mod legacy_structure_anchor_tree {
        use super::*;
        // The class itself isn't translated: it implements `IStructure` (org.betterx.betternether.world.structures). Its static parts are below.
        // NOISE: not translated, it creates a OpenSimplexNoise with a constructor that isn't translated.
        // LegacyStructureAnchorTree.generate(5): not translated, its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)).
        // LegacyStructureAnchorTree.canGenerate(1): not translated, its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)).
        // LegacyStructureAnchorTree.grow(6): not translated, its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)).
        // LegacyStructureAnchorTree.buildBigCircle(8): not translated, its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)).
        // LegacyStructureAnchorTree.buildLine(3): not translated, its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)).
        // LegacyStructureAnchorTree.lerp(3): not translated, its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)).
        // LegacyStructureAnchorTree.lerpCos(4): not translated, its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)).
        // LegacyStructureAnchorTree.lcos(1): not translated, its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)).
        // LegacyStructureAnchorTree.line(5): not translated, its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)).
        // LegacyStructureAnchorTree.cylinder(3): not translated, its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)).
        // LegacyStructureAnchorTree.circleLinesEnds(6): not translated, its class isn't translated (it implements `IStructure` (org.betterx.betternether.world.structures)).
        // LegacyStructureAnchorTree.makeMushroom(4): not translated, it uses Minecraft's `ServerLevelAccessor`.
    }

    /// `StructureCaves` (org/betterx/betternether/world/structures/StructureCaves.java)
    pub mod structure_caves {
        use super::*;
        // The class itself isn't translated: it implements `IStructure`. Its static parts are below.
        // StructureCaves.StructureCaves(1): not translated, its class isn't translated (it implements `IStructure`).
        // StructureCaves.generate(5): not translated, its class isn't translated (it implements `IStructure`).
        // StructureCaves.getHeight(2): not translated, its class isn't translated (it implements `IStructure`).
        // StructureCaves.getRigid(2): not translated, its class isn't translated (it implements `IStructure`).
        // StructureCaves.mix(3): not translated, its class isn't translated (it implements `IStructure`).
        // StructureCaves.opSmoothUnion(3): not translated, its class isn't translated (it implements `IStructure`).
        // StructureCaves.isInCave(4): not translated, its class isn't translated (it implements `IStructure`).
    }

    /// `StructureGeneratorThreadContext` (org/betterx/betternether/world/structures/StructureGeneratorThreadContext.java)
    pub mod structure_generator_thread_context {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `MutableBlockPos`. Its static parts are below.
        // StructureGeneratorThreadContext.clear(0): not translated, its class isn't translated (it uses Minecraft's `MutableBlockPos`).
    }

    /// `StructurePath` (org/betterx/betternether/world/structures/StructurePath.java)
    pub mod structure_path {
        use super::*;
        // The class itself isn't translated: it implements `IStructure`. Its static parts are below.
        // StructurePath.StructurePath(1): not translated, its class isn't translated (it implements `IStructure`).
        // StructurePath.generate(5): not translated, its class isn't translated (it implements `IStructure`).
        // StructurePath.getHeight(2): not translated, its class isn't translated (it implements `IStructure`).
        // StructurePath.getRigid(2): not translated, its class isn't translated (it implements `IStructure`).
        // StructurePath.isHeightValid(2): not translated, its class isn't translated (it implements `IStructure`).
        // StructurePath.makeLantern(2): not translated, its class isn't translated (it implements `IStructure`).
        // StructurePath.getRoadMaterial(3): not translated, its class isn't translated (it implements `IStructure`).
        // StructurePath.getSlabMaterial(3): not translated, its class isn't translated (it implements `IStructure`).
        // StructurePath.needsSlab(2): not translated, its class isn't translated (it implements `IStructure`).
    }

    /// `Altars` (org/betterx/betternether/world/structures/templates/Altars.java)
    pub mod altars {
        use super::*;
        // The class itself isn't translated: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // Altars.Altars(4): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // Altars.Altars(1): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // Altars.type(0): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
    }

    /// `Gardens` (org/betterx/betternether/world/structures/templates/Gardens.java)
    pub mod gardens {
        use super::*;
        // The class itself isn't translated: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // Gardens.Gardens(4): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // Gardens.Gardens(1): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // Gardens.type(0): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
    }

    /// `GhastHive` (org/betterx/betternether/world/structures/templates/GhastHive.java)
    pub mod ghast_hive {
        use super::*;
        // The class itself isn't translated: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // GhastHive.GhastHive(4): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // GhastHive.GhastHive(1): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // GhastHive.type(0): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
    }

    /// `JungleTemples` (org/betterx/betternether/world/structures/templates/JungleTemples.java)
    pub mod jungle_temples {
        use super::*;
        // The class itself isn't translated: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // JungleTemples.JungleTemples(4): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // JungleTemples.JungleTemples(1): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // JungleTemples.type(0): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
    }

    /// `Pillars` (org/betterx/betternether/world/structures/templates/Pillars.java)
    pub mod pillars {
        use super::*;
        // The class itself isn't translated: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // Pillars.Pillars(4): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // Pillars.Pillars(1): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // Pillars.type(0): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
    }

    /// `Portals` (org/betterx/betternether/world/structures/templates/Portals.java)
    pub mod portals {
        use super::*;
        // The class itself isn't translated: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // Portals.Portals(4): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // Portals.Portals(1): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // Portals.type(0): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
    }

    /// `Pyramids` (org/betterx/betternether/world/structures/templates/Pyramids.java)
    pub mod pyramids {
        use super::*;
        // The class itself isn't translated: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // Pyramids.Pyramids(4): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // Pyramids.Pyramids(1): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // Pyramids.type(0): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
    }

    /// `RespawnPoints` (org/betterx/betternether/world/structures/templates/RespawnPoints.java)
    pub mod respawn_points {
        use super::*;
        // The class itself isn't translated: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // RespawnPoints.RespawnPoints(4): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // RespawnPoints.RespawnPoints(1): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // RespawnPoints.type(0): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
    }

    /// `SpawnAltarLadder` (org/betterx/betternether/world/structures/templates/SpawnAltarLadder.java)
    pub mod spawn_altar_ladder {
        use super::*;
        // The class itself isn't translated: it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SpawnAltarLadder.SpawnAltarLadder(4): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // SpawnAltarLadder.SpawnAltarLadder(1): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
        // SpawnAltarLadder.type(0): not translated, its class isn't translated (it extends `RandomNbtStructure` (de.ambertation.wover.structure.api.structures.nbt)).
    }

    /// `LooseWood` (org/betterx/betternether/world/tree/build/LooseWood.java)
    pub mod loose_wood {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LooseWood {
        }
        impl LooseWood {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LooseWood.prune(1): not translated, `++` inside an expression.
        // LooseWood.lowestWoodLayer(1): not translated, it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`).
        // LooseWood.sweepUnattached(2): not translated, it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `TreeBuilder` (org/betterx/betternether/world/tree/build/TreeBuilder.java)
    pub mod tree_builder {
        use super::*;
        // The class itself isn't translated: it uses TrunkShape, which isn't translated (it's an interface). Its static parts are below.
        pub static DEBUG: LazyLock<bool> = LazyLock::new(|| std::env::var("betternether.tree.debug").is_ok_and(|v| v.eq_ignore_ascii_case("true")));
        pub const BARK_MARGIN: f32 = 2.0f32;
        // TreeBuilder.TreeBuilder(1): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.create(0): not translated, it uses Builder, which isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.build(2): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.build(3): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.settleColumnTops(1): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.world(4): not translated, it uses Minecraft's `BlockPos`.
        // TreeBuilder.commit(3): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.filterFor(2): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.grow(3): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.allocate(1): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.rasterizeWood(5): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.rasterizeFoliage(5): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.orderedAnchors(2): not translated, it uses `Skeleton` (org.betterx.betternether.world.tree.skeleton).
        // TreeBuilder.primaryAnchor(1): not translated, it uses `Skeleton` (org.betterx.betternether.world.tree.skeleton).
    }

    /// `TreeBuilder.Bounds` (org/betterx/betternether/world/tree/build/TreeBuilder.java)
    pub mod tree_builder_bounds {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Bounds {
            pub empty: bool,
            pub min_x: i32,
            pub min_y: i32,
            pub min_z: i32,
            pub max_x: i32,
            pub max_y: i32,
            pub max_z: i32,
        }
        impl Bounds {
            pub fn new() -> Self {
                let mut s = Self { empty: true, min_x: 0i32, min_y: 0i32, min_z: 0i32, max_x: 0i32, max_y: 0i32, max_z: 0i32 };
                s
            }
            pub fn include(&mut self, point: JVec3, radius: f32) {
                let low_x: i32 = ((((point.x - radius) as f64).floor()) as i32);
                let low_y: i32 = ((((point.y - radius) as f64).floor()) as i32);
                let low_z: i32 = ((((point.z - radius) as f64).floor()) as i32);
                let high_x: i32 = ((((point.x + radius) as f64).ceil()) as i32);
                let high_y: i32 = ((((point.y + radius) as f64).ceil()) as i32);
                let high_z: i32 = ((((point.z + radius) as f64).ceil()) as i32);
                if self.empty {
                    self.min_x = low_x;
                    self.min_y = low_y;
                    self.min_z = low_z;
                    self.max_x = high_x;
                    self.max_y = high_y;
                    self.max_z = high_z;
                    self.empty = false;
                } else {
                    self.min_x = self.min_x.min(low_x);
                    self.min_y = self.min_y.min(low_y);
                    self.min_z = self.min_z.min(low_z);
                    self.max_x = self.max_x.max(high_x);
                    self.max_y = self.max_y.max(high_y);
                    self.max_z = self.max_z.max(high_z);
                }
            }
        }
    }

    /// `TreeBuilder.Builder` (org/betterx/betternether/world/tree/build/TreeBuilder.java)
    pub mod tree_builder_builder {
        use super::*;
        // The class itself isn't translated: it uses TrunkShape, which isn't translated (it's an interface). Its static parts are below.
        // TreeBuilder.Builder.trunk(1): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.Builder.canopy(1): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.Builder.palette(1): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.Builder.bark(1): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.Builder.repair(1): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.Builder.decayBudget(1): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.Builder.maxCarves(1): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.Builder.pruneLooseWood(1): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.Builder.replaceable(1): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.Builder.writeFlags(1): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
        // TreeBuilder.Builder.build(0): not translated, its class isn't translated (it uses TrunkShape, which isn't translated (it's an interface)).
    }

    /// `TreeBuilder.CellFilter` (org/betterx/betternether/world/tree/build/TreeBuilder.java)
    pub mod tree_builder_cell_filter {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TreeBuilder.CellFilter.canOccupy(3): not translated, it has no body.
    }

    /// `TreeVoxels` (org/betterx/betternether/world/tree/build/TreeVoxels.java)
    pub mod tree_voxels {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        pub const EMPTY: i8 = 0i8;
        pub const LOG: i8 = 1i8;
        pub const LEAF: i8 = 2i8;
        pub static FACE_X: LazyLock<Vec<i32>> = LazyLock::new(|| vec![-1, 1, 0, 0, 0, 0]);
        pub static FACE_Y: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0, 0, -1, 1, 0, 0]);
        pub static FACE_Z: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0, 0, 0, 0, -1, 1]);
        // TreeVoxels.TreeVoxels(6): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.minX(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.minY(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.minZ(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.sizeX(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.sizeY(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.sizeZ(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.cellCount(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.inBounds(3): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.index(3): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.localX(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.localY(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.localZ(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.kindAt(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.kindAt(3): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.stateAt(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.set(3): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.setLog(4): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.setLeafIfEmpty(4): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.clear(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.isBuried(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.countWoodBreakingFoliage(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.count(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // TreeVoxels.forEach(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `TreeVoxels.CellVisitor` (org/betterx/betternether/world/tree/build/TreeVoxels.java)
    pub mod tree_voxels_cell_visitor {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TreeVoxels.CellVisitor.visit(5): not translated, it has no body.
    }

    /// `CanopyShape` (org/betterx/betternether/world/tree/canopy/CanopyShape.java)
    pub mod canopy_shape {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CanopyShape.volumeAt(3): not translated, it has no body.
        // CanopyShape.reach(0): not translated, it has no body.
    }

    /// `GhostCanopy` (org/betterx/betternether/world/tree/canopy/GhostCanopy.java)
    pub mod ghost_canopy {
        use super::*;
        // The class itself isn't translated: it implements `CanopyShape`. Its static parts are below.
        // GhostCanopy.GhostCanopy(1): not translated, its class isn't translated (it implements `CanopyShape`).
        pub fn builder() -> ghost_canopy_builder::Builder {
            return ghost_canopy_builder::Builder::new();
        }
        // GhostCanopy.volumeAt(3): not translated, its class isn't translated (it implements `CanopyShape`).
        // GhostCanopy.reach(0): not translated, its class isn't translated (it implements `CanopyShape`).
    }

    /// `GhostCanopy.Builder` (org/betterx/betternether/world/tree/canopy/GhostCanopy.java)
    pub mod ghost_canopy_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
            pub radius: f32,
            pub body_height: f32,
            pub dome_height: f32,
            pub skirt_width: f32,
            pub skirt_depth: f32,
            pub skirt_rise: f32,
            pub smoothing: f32,
            pub hem_wobble: f32,
            pub min_radius: f32,
            pub vertical_offset: f32,
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self { radius: 4.0f32, body_height: 6.0f32, dome_height: 3.0f32, skirt_width: 0.72f32, skirt_depth: 3.0f32, skirt_rise: 0.5f32, smoothing: 1.5f32, hem_wobble: 0.7f32, min_radius: 2.0f32, vertical_offset: 0.0f32 };
                s
            }
            // GhostCanopy.Builder.radius(1): not translated, `this` used as a value.
            // GhostCanopy.Builder.bodyHeight(1): not translated, `this` used as a value.
            // GhostCanopy.Builder.domeHeight(1): not translated, `this` used as a value.
            // GhostCanopy.Builder.skirt(3): not translated, `this` used as a value.
            // GhostCanopy.Builder.smoothing(1): not translated, `this` used as a value.
            // GhostCanopy.Builder.hemWobble(1): not translated, `this` used as a value.
            // GhostCanopy.Builder.minRadius(1): not translated, `this` used as a value.
            // GhostCanopy.Builder.verticalOffset(1): not translated, `this` used as a value.
            // GhostCanopy.Builder.build(0): not translated, it uses GhostCanopy, which isn't translated (it implements `CanopyShape`).
        }
    }

    /// `DecayRepair` (org/betterx/betternether/world/tree/decay/DecayRepair.java)
    pub mod decay_repair {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum DecayRepair {
            Prune,
            CarveBranch,
            Persist,
        }
        impl DecayRepair {
            pub const VALUES: [DecayRepair; 3] = [DecayRepair::Prune, DecayRepair::CarveBranch, DecayRepair::Persist, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    DecayRepair::Prune => "PRUNE",
                    DecayRepair::CarveBranch => "CARVE_BRANCH",
                    DecayRepair::Persist => "PERSIST",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "PRUNE" => DecayRepair::Prune,
                    "CARVE_BRANCH" => DecayRepair::CarveBranch,
                    "PERSIST" => DecayRepair::Persist,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `LeafDecay` (org/betterx/betternether/world/tree/decay/LeafDecay.java)
    pub mod leaf_decay {
        use super::*;
        pub const VANILLA_BUDGET: i32 = 6;
        pub static UNREACHED: LazyLock<i32> = LazyLock::new(|| i32::MAX);
        pub static FACE_X: LazyLock<Vec<i32>> = LazyLock::new(|| vec![-1, 1, 0, 0, 0, 0]);
        pub static FACE_Y: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0, 0, -1, 1, 0, 0]);
        pub static FACE_Z: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0, 0, 0, 0, -1, 1]);
        #[derive(Clone, Debug)]
        pub struct LeafDecay {
        }
        impl LeafDecay {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LeafDecay.solve(2): not translated, it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`).
        // LeafDecay.solve(5): not translated, it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`).
        // LeafDecay.sweep(1): not translated, it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`).
        // LeafDecay.worstReachableLeaf(4): not translated, it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`).
        // LeafDecay.carveTowardsWood(5): not translated, it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`).
        // LeafDecay.stepTowardsWood(3): not translated, it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`).
        // LeafDecay.finish(5): not translated, it uses TreeVoxels, which isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `LeafDecay.BranchFactory` (org/betterx/betternether/world/tree/decay/LeafDecay.java)
    pub mod leaf_decay_branch_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LeafDecay.BranchFactory.at(3): not translated, it has no body.
    }

    /// `LeafDecay.Result` (org/betterx/betternether/world/tree/decay/LeafDecay.java)
    pub mod leaf_decay_result {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Result_ {
            pub leaves_kept: i32,
            pub leaves_pruned: i32,
            pub branch_cells: i32,
            pub leaves_persist: i32,
        }
        impl Result_ {
            pub fn new(leaves_kept: i32, leaves_pruned: i32, branch_cells: i32, leaves_persist: i32) -> Self {
                let mut s = Self { leaves_kept: leaves_kept, leaves_pruned: leaves_pruned, branch_cells: branch_cells, leaves_persist: leaves_persist };
                s.leaves_kept = leaves_kept;
                s.leaves_pruned = leaves_pruned;
                s.branch_cells = branch_cells;
                s.leaves_persist = leaves_persist;
                s
            }
            pub fn is_clean(&self) -> bool {
                return ((self.leaves_pruned == 0) && (self.branch_cells == 0)) && (self.leaves_persist == 0);
            }
            pub fn leaves_kept(&self) -> i32 {
                return self.leaves_kept;
            }
            pub fn leaves_pruned(&self) -> i32 {
                return self.leaves_pruned;
            }
            pub fn branch_cells(&self) -> i32 {
                return self.branch_cells;
            }
            pub fn leaves_persist(&self) -> i32 {
                return self.leaves_persist;
            }
        }
    }

    /// `Crackle` (org/betterx/betternether/world/tree/math/Crackle.java)
    pub mod crackle {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Crackle {
        }
        impl Crackle {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Crackle.bark(3): not translated, it uses Displacement, which isn't translated (it's an interface).
        // Crackle.wobble(3): not translated, it uses Displacement, which isn't translated (it's an interface).
    }

    /// `Spline` (org/betterx/betternether/world/tree/math/Spline.java)
    pub mod spline {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Spline {
        }
        impl Spline {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn line(from: JVec3, to: JVec3, points: i32) -> Vec<JVec3> {
            let mut result: Vec<JVec3> = Vec::with_capacity((points.max(2)) as usize);
            let last: i32 = (points.max(2)) - 1;
            for i in 0..=last {
                let t: f32 = (i as f32) / (last as f32);
                result.push(JVec3::new(from.x + ((to.x - from.x) * t), from.y + ((to.y - from.y) * t), from.z + ((to.z - from.z) * t)));
            }
            return result.to_vec();
        }
        pub fn smooth(control: &[JVec3], subdivisions: i32) -> Vec<JVec3> {
            if ((control.len() as i32) >= 3) && (subdivisions > 1) {
                let mut result: Vec<JVec3> = Vec::with_capacity(((control.len() as i32) * subdivisions) as usize);
                {
                    let mut i: i32 = 0;
                    while i < ((control.len() as i32) - 1) {
                        let p0: JVec3 = control[((i - 1).max(0)) as usize];
                        let p1: JVec3 = control[i as usize];
                        let p2: JVec3 = control[(i + 1) as usize];
                        let p3: JVec3 = control[((i + 2).min((control.len() as i32) - 1)) as usize];
                        for s in 0..subdivisions {
                            result.push(spline::catmull_rom_5(p0, p1, p2, p3, (s as f32) / (subdivisions as f32)));
                        }
                        i += 1;
                    }
                }
                result.push(control[((control.len() as i32) - 1) as usize]);
                return result.to_vec();
            } else {
                return control.to_vec();
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        pub fn catmull_rom_5(p0: JVec3, p1: JVec3, p2: JVec3, p3: JVec3, t: f32) -> JVec3 {
            let t2: f32 = t * t;
            let t3: f32 = t2 * t;
            return JVec3::new(spline::catmull_rom_7(p0.x, p1.x, p2.x, p3.x, t, t2, t3), spline::catmull_rom_7(p0.y, p1.y, p2.y, p3.y, t, t2, t3), spline::catmull_rom_7(p0.z, p1.z, p2.z, p3.z, t, t2, t3));
        }
        pub fn catmull_rom_7(a: f32, b: f32, c: f32, d: f32, t: f32, t2: f32, t3: f32) -> f32 {
            return 0.5f32 * ((((2.0f32 * b) + (((-a) + c) * t)) + (((((2.0f32 * a) - (5.0f32 * b)) + (4.0f32 * c)) - d) * t2)) + (((((-a) + (3.0f32 * b)) - (3.0f32 * c)) + d) * t3));
        }
        pub fn jitter(spline: &[JVec3], random: JRandom, amount: f32) {
            if ((spline.len() as i32) >= 3) && (!(amount <= 0.0f32)) {
                {
                    let mut i: i32 = 1;
                    while i < ((spline.len() as i32) - 1) {
                        let window: f32 = ((((std::f64::consts::PI * (i as f64)) / (((spline.len() as i32) - 1) as f64)).sin()) as f32);
                        let scale: f32 = amount * window;
                        { let joml_0 = spline[i as usize].add3(((random.next_float() * 2.0f32) - 1.0f32) * scale, (((random.next_float() * 2.0f32) - 1.0f32) * scale) * 0.35f32, ((random.next_float() * 2.0f32) - 1.0f32) * scale); spline[i as usize] = joml_0; joml_0 };
                        i += 1;
                    }
                }
            }
        }
    }

    /// `Volume` (org/betterx/betternether/world/tree/math/Volume.java)
    pub mod volume {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Volume.distance(3): not translated, it has no body.
        // Volume.contains(3): not translated, its class isn't translated (it's an interface).
        // Volume.union(1): not translated, its class isn't translated (it's an interface).
        // Volume.intersect(1): not translated, its class isn't translated (it's an interface).
        // Volume.subtract(1): not translated, its class isn't translated (it's an interface).
        // Volume.smoothUnion(2): not translated, its class isn't translated (it's an interface).
        // Volume.translate(3): not translated, its class isn't translated (it's an interface).
        // Volume.scale(3): not translated, its class isn't translated (it's an interface).
        // Volume.expand(1): not translated, its class isn't translated (it's an interface).
        // Volume.displace(1): not translated, its class isn't translated (it's an interface).
    }

    /// `Volume.Displacement` (org/betterx/betternether/world/tree/math/Volume.java)
    pub mod volume_displacement {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Volume.Displacement.at(3): not translated, it has no body.
    }

    /// `Volumes` (org/betterx/betternether/world/tree/math/Volumes.java)
    pub mod volumes {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Volumes {
        }
        impl Volumes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Volumes.sphere(1): not translated, it uses Volume, which isn't translated (it's an interface).
        // Volumes.ellipsoid(3): not translated, it uses Volume, which isn't translated (it's an interface).
        // Volumes.cylinder(2): not translated, it uses Volume, which isn't translated (it's an interface).
        // Volumes.capsule(4): not translated, it uses Volume, which isn't translated (it's an interface).
        // Volumes.box(3): not translated, it uses Volume, which isn't translated (it's an interface).
        // Volumes.below(1): not translated, it uses Volume, which isn't translated (it's an interface).
    }

    /// `LeafMarks` (org/betterx/betternether/world/tree/palette/LeafMarks.java)
    pub mod leaf_marks {
        use super::*;
        // The class itself isn't translated: it implements `LeafSource`. Its static parts are below.
        // LeafMarks.LeafMarks(1): not translated, its class isn't translated (it implements `LeafSource`).
        // LeafMarks.of(1): not translated, it uses Minecraft's `BlockState`.
        // LeafMarks.at(1): not translated, its class isn't translated (it implements `LeafSource`).
    }

    /// `LeafMarks.Builder` (org/betterx/betternether/world/tree/palette/LeafMarks.java)
    pub mod leaf_marks_builder {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        // LeafMarks.Builder.Builder(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // LeafMarks.Builder.mark(2): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // LeafMarks.Builder.mark(3): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // LeafMarks.Builder.markOnPrimaryLobe(2): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // LeafMarks.Builder.eyes(6): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // LeafMarks.Builder.eyesOnPrimaryLobe(6): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // LeafMarks.Builder.addEyes(7): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // LeafMarks.Builder.build(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `LeafMarks.Mark` (org/betterx/betternether/world/tree/palette/LeafMarks.java)
    pub mod leaf_marks_mark {
        use super::*;
        // The class itself isn't translated: it uses Volume, which isn't translated (it's an interface). Its static parts are below.
        // LeafMarks.Mark.Mark(4): not translated, its class isn't translated (it uses Volume, which isn't translated (it's an interface)).
        // LeafMarks.Mark.region(0): not translated, its class isn't translated (it uses Volume, which isn't translated (it's an interface)).
        // LeafMarks.Mark.block(0): not translated, its class isn't translated (it uses Volume, which isn't translated (it's an interface)).
        // LeafMarks.Mark.chance(0): not translated, its class isn't translated (it uses Volume, which isn't translated (it's an interface)).
        // LeafMarks.Mark.primaryOnly(0): not translated, its class isn't translated (it uses Volume, which isn't translated (it's an interface)).
    }

    /// `LogBands` (org/betterx/betternether/world/tree/palette/LogBands.java)
    pub mod log_bands {
        use super::*;
        // The class itself isn't translated: it implements `LogSource`. Its static parts are below.
        pub const NOISE_SCALE: f64 = 0.35f64;
        // LogBands.LogBands(1): not translated, its class isn't translated (it implements `LogSource`).
        // LogBands.builder(0): not translated, it uses Builder, which isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)).
        // LogBands.at(1): not translated, its class isn't translated (it implements `LogSource`).
        // LogBands.topOfColumn(1): not translated, its class isn't translated (it implements `LogSource`).
        // LogBands.sameAxis(2): not translated, its class isn't translated (it implements `LogSource`).
        // LogBands.seamLayer(3): not translated, its class isn't translated (it implements `LogSource`).
        // LogBands.choice(3): not translated, its class isn't translated (it implements `LogSource`).
    }

    /// `LogBands.Band` (org/betterx/betternether/world/tree/palette/LogBands.java)
    pub mod log_bands_band {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        // LogBands.Band.Band(3): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // LogBands.Band.upTo(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // LogBands.Band.block(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // LogBands.Band.seam(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `LogBands.Builder` (org/betterx/betternether/world/tree/palette/LogBands.java)
    pub mod log_bands_builder {
        use super::*;
        // The class itself isn't translated: it uses Band, which isn't translated (it uses Minecraft's `BlockState`). Its static parts are below.
        // LogBands.Builder.band(2): not translated, its class isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)).
        // LogBands.Builder.seam(2): not translated, its class isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)).
        // LogBands.Builder.top(1): not translated, its class isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)).
        // LogBands.Builder.jitter(2): not translated, its class isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)).
        // LogBands.Builder.seamSeed(1): not translated, its class isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)).
        // LogBands.Builder.applyAxis(1): not translated, its class isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)).
        // LogBands.Builder.build(0): not translated, its class isn't translated (it uses Band, which isn't translated (it uses Minecraft's `BlockState`)).
    }

    /// `TreePalette` (org/betterx/betternether/world/tree/palette/TreePalette.java)
    pub mod tree_palette {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TreePalette.log(1): not translated, it has no body.
        // TreePalette.leaf(1): not translated, it has no body.
        // TreePalette.topOfColumn(1): not translated, its class isn't translated (it's an interface).
        // TreePalette.of(2): not translated, its body couldn't be read: an anonymous class.
    }

    /// `TreePalette.LeafQuery` (org/betterx/betternether/world/tree/palette/TreePalette.java)
    pub mod tree_palette_leaf_query {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // TreePalette.LeafQuery.LeafQuery(6): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreePalette.LeafQuery.world(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreePalette.LeafQuery.local(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreePalette.LeafQuery.anchor(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreePalette.LeafQuery.toAnchor(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreePalette.LeafQuery.primary(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreePalette.LeafQuery.random(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `TreePalette.LeafSource` (org/betterx/betternether/world/tree/palette/TreePalette.java)
    pub mod tree_palette_leaf_source {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TreePalette.LeafSource.at(1): not translated, it has no body.
    }

    /// `TreePalette.LogQuery` (org/betterx/betternether/world/tree/palette/TreePalette.java)
    pub mod tree_palette_log_query {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // TreePalette.LogQuery.LogQuery(6): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreePalette.LogQuery.world(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreePalette.LogQuery.local(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreePalette.LogQuery.heightAbove(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreePalette.LogQuery.axis(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreePalette.LogQuery.carved(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreePalette.LogQuery.random(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `TreePalette.LogSource` (org/betterx/betternether/world/tree/palette/TreePalette.java)
    pub mod tree_palette_log_source {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TreePalette.LogSource.at(1): not translated, it has no body.
        // TreePalette.LogSource.topOfColumn(1): not translated, its class isn't translated (it's an interface).
    }

    /// `Anchor` (org/betterx/betternether/world/tree/skeleton/Anchor.java)
    pub mod anchor {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Anchor {
            pub position: Option<JVec3>,
            pub scale: f32,
        }
        impl Anchor {
            pub fn new_2(position: JVec3, scale: f32) -> Self {
                let mut s = Self { position: Some(position), scale: scale };
                s.position = Some(position);
                s.scale = scale;
                s
            }
            pub fn new_1(position: JVec3) -> Self {
                let mut s = Self::new_2(position, 1.0f32);
                s
            }
            pub fn position(&self) -> Option<JVec3> {
                return self.position.clone();
            }
            pub fn scale(&self) -> f32 {
                return self.scale;
            }
        }
    }

    /// `ForkingTrunk` (org/betterx/betternether/world/tree/skeleton/ForkingTrunk.java)
    pub mod forking_trunk {
        use super::*;
        // The class itself isn't translated: it implements `TrunkShape`. Its static parts are below.
        // ForkingTrunk.ForkingTrunk(1): not translated, its class isn't translated (it implements `TrunkShape`).
        pub fn builder() -> forking_trunk_builder::Builder {
            return forking_trunk_builder::Builder::new();
        }
        // ForkingTrunk.build(2): not translated, its class isn't translated (it implements `TrunkShape`).
        // ForkingTrunk.anchorsAlong(1): not translated, its class isn't translated (it implements `TrunkShape`).
    }

    /// `ForkingTrunk.Builder` (org/betterx/betternether/world/tree/skeleton/ForkingTrunk.java)
    pub mod forking_trunk_builder {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Builder {
            pub min_height: i32,
            pub max_height: i32,
            pub fork_at: f32,
            pub min_arms: i32,
            pub max_arms: i32,
            pub spread: f32,
            pub lean: f32,
            pub jitter: f32,
            pub base_radius: f32,
            pub tip_radius: f32,
            pub anchors_per_arm: i32,
        }
        impl Builder {
            pub fn new() -> Self {
                let mut s = Self { min_height: 9, max_height: 14, fork_at: 0.55f32, min_arms: 2, max_arms: 3, spread: 3.0f32, lean: 0.8f32, jitter: 0.6f32, base_radius: 1.4f32, tip_radius: 0.7f32, anchors_per_arm: 2 };
                s
            }
            // ForkingTrunk.Builder.height(2): not translated, `this` used as a value.
            // ForkingTrunk.Builder.forkAt(1): not translated, `this` used as a value.
            // ForkingTrunk.Builder.arms(2): not translated, `this` used as a value.
            // ForkingTrunk.Builder.spread(1): not translated, `this` used as a value.
            // ForkingTrunk.Builder.lean(1): not translated, `this` used as a value.
            // ForkingTrunk.Builder.jitter(1): not translated, `this` used as a value.
            // ForkingTrunk.Builder.radius(2): not translated, `this` used as a value.
            // ForkingTrunk.Builder.anchorsPerArm(1): not translated, `this` used as a value.
            // ForkingTrunk.Builder.build(0): not translated, it uses ForkingTrunk, which isn't translated (it implements `TrunkShape`).
        }
    }

    /// `Segment` (org/betterx/betternether/world/tree/skeleton/Segment.java)
    pub mod segment {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Segment {
            pub from: JVec3,
            pub to: JVec3,
            pub radius_from: f32,
            pub radius_to: f32,
        }
        impl Segment {
            pub fn new(from: JVec3, to: JVec3, radius_from: f32, radius_to: f32) -> Self {
                let mut s = Self { from: from, to: to, radius_from: radius_from, radius_to: radius_to };
                s.from = from;
                s.to = to;
                s.radius_from = radius_from;
                s.radius_to = radius_to;
                s
            }
            // Segment.toVolume(0): not translated, it uses Volume, which isn't translated (it's an interface).
            pub fn max_radius(&self) -> f32 {
                return self.radius_from.max(self.radius_to);
            }
            // Segment.axis(0): not translated, it uses Minecraft's `Axis`.
            pub fn from(&self) -> JVec3 {
                return self.from;
            }
            pub fn to(&self) -> JVec3 {
                return self.to;
            }
            pub fn radius_from(&self) -> f32 {
                return self.radius_from;
            }
            pub fn radius_to(&self) -> f32 {
                return self.radius_to;
            }
        }
    }

    /// `TrunkShape` (org/betterx/betternether/world/tree/skeleton/TrunkShape.java)
    pub mod trunk_shape {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TrunkShape.build(2): not translated, it has no body.
    }

    /// `TreeSpace` (org/betterx/betternether/world/tree/TreeSpace.java)
    pub mod tree_space {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // TreeSpace.TreeSpace(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreeSpace.of(2): not translated, it uses TreeSpace, which isn't translated (it uses Minecraft's `BlockPos`).
        // TreeSpace.unbounded(1): not translated, it uses TreeSpace, which isn't translated (it uses Minecraft's `BlockPos`).
        // TreeSpace.fitRadius(3): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreeSpace.fitSegment(3): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreeSpace.canWrite(2): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreeSpace.origin(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // TreeSpace.zone(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `NetherAdvancementDataProvider` (org/betterx/datagen/betternether/advancements/NetherAdvancementDataProvider.java)
    pub mod nether_advancement_data_provider {
        use super::*;
        // The class itself isn't translated: it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen). Its static parts are below.
        // NetherAdvancementDataProvider.NetherAdvancementDataProvider(2): not translated, its class isn't translated (it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)).
        // NetherAdvancementDataProvider.bootstrap(1): not translated, its class isn't translated (it extends `AdvancementDataProvider` (org.betterx.bclib.api.v3.datagen)).
    }

    /// `BetterNetherDatagen` (org/betterx/datagen/betternether/BetterNetherDatagen.java)
    pub mod better_nether_datagen {
        use super::*;
        // The class itself isn't translated: it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api). Its static parts are below.
        // BetterNetherDatagen.onInitializeProviders(1): not translated, its class isn't translated (it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api)).
        // BetterNetherDatagen.onBuildRegistry(1): not translated, its class isn't translated (it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api)).
        // BetterNetherDatagen.modCore(0): not translated, its class isn't translated (it extends `WoverDataGenEntryPoint` (de.ambertation.wover.datagen.api)).
    }

    /// `NetherBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherBlockDefinitions.java)
    pub mod nether_block_definitions {
        use super::*;
        // The class itself isn't translated: it implements `BlockDefinitionsEntrypoint` (de.ambertation.wover.block.api.classify). Its static parts are below.
        // NetherBlockDefinitions.defineBlocks(1): not translated, its class isn't translated (it implements `BlockDefinitionsEntrypoint` (de.ambertation.wover.block.api.classify)).
    }

    /// `NetherCropBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherCropBlockDefinitions.java)
    pub mod nether_crop_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct NetherCropBlockDefinitions {
        }
        impl NetherCropBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherCropBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherCropBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherDecorBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherDecorBlockDefinitions.java)
    pub mod nether_decor_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        // POT_TAGS: not translated, it uses Minecraft's `TagKey`.
        // CINCINNASITE_POT_TAGS: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct NetherDecorBlockDefinitions {
        }
        impl NetherDecorBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherDecorBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherDecorBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherFunctionalBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherFunctionalBlockDefinitions.java)
    pub mod nether_functional_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        // ANVIL_TAGS: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct NetherFunctionalBlockDefinitions {
        }
        impl NetherFunctionalBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherFunctionalBlockDefinitions.furnace(4): not translated, it's generic.
        // NetherFunctionalBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherFunctionalBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherFurnitureBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherFurnitureBlockDefinitions.java)
    pub mod nether_furniture_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        // NO_LOOT: not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        #[derive(Clone, Debug)]
        pub struct NetherFurnitureBlockDefinitions {
        }
        impl NetherFurnitureBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherFurnitureBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherFurnitureBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherGlassBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherGlassBlockDefinitions.java)
    pub mod nether_glass_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct NetherGlassBlockDefinitions {
        }
        impl NetherGlassBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherGlassBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherGlassBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherLeavesBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherLeavesBlockDefinitions.java)
    pub mod nether_leaves_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct NetherLeavesBlockDefinitions {
        }
        impl NetherLeavesBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherLeavesBlockDefinitions.leaves(5): not translated, it's generic.
        // NetherLeavesBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherLeavesBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherLightBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherLightBlockDefinitions.java)
    pub mod nether_light_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct NetherLightBlockDefinitions {
        }
        impl NetherLightBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherLightBlockDefinitions.fireBowl(7): not translated, it's generic.
        // NetherLightBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherLightBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherMetalBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherMetalBlockDefinitions.java)
    pub mod nether_metal_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        // NO_RECIPE: not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        #[derive(Clone, Debug)]
        pub struct NetherMetalBlockDefinitions {
        }
        impl NetherMetalBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherMetalBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherMetalBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherMushroomBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherMushroomBlockDefinitions.java)
    pub mod nether_mushroom_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct NetherMushroomBlockDefinitions {
        }
        impl NetherMushroomBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherMushroomBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherMushroomBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherObsidianBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherObsidianBlockDefinitions.java)
    pub mod nether_obsidian_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct NetherObsidianBlockDefinitions {
        }
        impl NetherObsidianBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherObsidianBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherObsidianBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherOreBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherOreBlockDefinitions.java)
    pub mod nether_ore_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct NetherOreBlockDefinitions {
        }
        impl NetherOreBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherOreBlockDefinitions.ore(7): not translated, it's generic.
        // NetherOreBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherPlantBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherPlantBlockDefinitions.java)
    pub mod nether_plant_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        // GLIMMERWILD_GRASSES: not translated, it uses `BlockKey` (de.ambertation.wover.block.api.classify).
        #[derive(Clone, Debug)]
        pub struct NetherPlantBlockDefinitions {
        }
        impl NetherPlantBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherPlantBlockDefinitions.grass(6): not translated, it's generic.
        // NetherPlantBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherPlantBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherSaplingBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherSaplingBlockDefinitions.java)
    pub mod nether_sapling_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct NetherSaplingBlockDefinitions {
        }
        impl NetherSaplingBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherSaplingBlockDefinitions.sapling(5): not translated, it's generic.
        // NetherSaplingBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherSaplingBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherStoneBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherStoneBlockDefinitions.java)
    pub mod nether_stone_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        // NO_RECIPE: not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // SOUL_SANDSTONE_TERRAIN_TAGS: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct NetherStoneBlockDefinitions {
        }
        impl NetherStoneBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherStoneBlockDefinitions.stalactite(4): not translated, it uses `BlockDefinition` (de.ambertation.wover.block.api).
        // NetherStoneBlockDefinitions.simple2x2(3): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherStoneBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherStoneBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherTerrainBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherTerrainBlockDefinitions.java)
    pub mod nether_terrain_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        pub const FUEL_DESCRIPTION: &str = "tooltip.betternether.fuel";
        // BLEACHED_GLOOMSCULK_TAGS: not translated, it uses Minecraft's `TagKey`.
        // MOLTEN_GLOOMSCULK_TAGS: not translated, it uses Minecraft's `TagKey`.
        // VEINED_GLOOMSCULK_TAGS: not translated, it uses Minecraft's `TagKey`.
        // MYCELIUM_TAGS: not translated, it uses Minecraft's `TagKey`.
        #[derive(Clone, Debug)]
        pub struct NetherTerrainBlockDefinitions {
        }
        impl NetherTerrainBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherTerrainBlockDefinitions.terrain(3): not translated, it uses `BlockDefinition` (de.ambertation.wover.block.api).
        // NetherTerrainBlockDefinitions.crystal(2): not translated, it uses `BlockDefinition` (de.ambertation.wover.block.api).
        // NetherTerrainBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherTerrainBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherVineBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherVineBlockDefinitions.java)
    pub mod nether_vine_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        // NO_LOOT: not translated, it uses `LootOutputTrait` (de.ambertation.wover.block.impl.classify).
        #[derive(Clone, Debug)]
        pub struct NetherVineBlockDefinitions {
        }
        impl NetherVineBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherVineBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherVineBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherWallPlantBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherWallPlantBlockDefinitions.java)
    pub mod nether_wall_plant_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct NetherWallPlantBlockDefinitions {
        }
        impl NetherWallPlantBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherWallPlantBlockDefinitions.wallPlant(3): not translated, it uses `BlockDefinition` (de.ambertation.wover.block.api).
        // NetherWallPlantBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherWallPlantBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherWoodBlockDefinitions` (org/betterx/datagen/betternether/blocks/NetherWoodBlockDefinitions.java)
    pub mod nether_wood_block_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct NetherWoodBlockDefinitions {
        }
        impl NetherWoodBlockDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherWoodBlockDefinitions.transitionConversion(2): not translated, it uses `RecipeOutputTrait` (de.ambertation.wover.block.impl.classify).
        // NetherWoodBlockDefinitions.model(1): not translated, a raw Supplier.
        // NetherWoodBlockDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherEnchantmentProvider` (org/betterx/datagen/betternether/enchantments/NetherEnchantmentProvider.java)
    pub mod nether_enchantment_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverEnchantmentProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // NetherEnchantmentProvider.NetherEnchantmentProvider(1): not translated, its class isn't translated (it extends `WoverEnchantmentProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherEnchantmentProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverEnchantmentProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `NetherEnchantmentTagProvider` (org/betterx/datagen/betternether/enchantments/NetherEnchantmentTagProvider.java)
    pub mod nether_enchantment_tag_provider {
        use super::*;
        // The class itself isn't translated: it extends `ForEnchantments` (de.ambertation.wover.datagen.api.WoverTagProvider). Its static parts are below.
        // NetherEnchantmentTagProvider.NetherEnchantmentTagProvider(1): not translated, its class isn't translated (it extends `ForEnchantments` (de.ambertation.wover.datagen.api.WoverTagProvider)).
        // NetherEnchantmentTagProvider.prepareTags(1): not translated, its class isn't translated (it extends `ForEnchantments` (de.ambertation.wover.datagen.api.WoverTagProvider)).
    }

    /// `NetherEntityTypeTagProvider` (org/betterx/datagen/betternether/entity/NetherEntityTypeTagProvider.java)
    pub mod nether_entity_type_tag_provider {
        use super::*;
        // The class itself isn't translated: it extends `ForEntityTypes` (de.ambertation.wover.datagen.api.WoverTagProvider). Its static parts are below.
        // NetherEntityTypeTagProvider.NetherEntityTypeTagProvider(1): not translated, its class isn't translated (it extends `ForEntityTypes` (de.ambertation.wover.datagen.api.WoverTagProvider)).
        // NetherEntityTypeTagProvider.prepareTags(1): not translated, its class isn't translated (it extends `ForEntityTypes` (de.ambertation.wover.datagen.api.WoverTagProvider)).
    }

    /// `GlimmerwildBiomeModificationProvider` (org/betterx/datagen/betternether/glimmerwild/GlimmerwildBiomeModificationProvider.java)
    pub mod glimmerwild_biome_modification_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api). Its static parts are below.
        // GlimmerwildBiomeModificationProvider.GlimmerwildBiomeModificationProvider(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
        // GlimmerwildBiomeModificationProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
    }

    /// `GlimmerwildBlockTagProvider` (org/betterx/datagen/betternether/glimmerwild/GlimmerwildBlockTagProvider.java)
    pub mod glimmerwild_block_tag_provider {
        use super::*;
        // The class itself isn't translated: it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider). Its static parts are below.
        // GlimmerwildBlockTagProvider.GlimmerwildBlockTagProvider(1): not translated, its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)).
        // GlimmerwildBlockTagProvider.shouldAdd(1): not translated, its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)).
        // GlimmerwildBlockTagProvider.prepareTags(1): not translated, its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)).
    }

    /// `GlimmerwildFeatureProvider` (org/betterx/datagen/betternether/glimmerwild/GlimmerwildFeatureProvider.java)
    pub mod glimmerwild_feature_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        // GlimmerwildFeatureProvider.GlimmerwildFeatureProvider(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // GlimmerwildFeatureProvider.bootstrapFeatures(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // GlimmerwildFeatureProvider.bootstrapPlaced(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
    }

    /// `NetherDebugItemDefinitions` (org/betterx/datagen/betternether/items/NetherDebugItemDefinitions.java)
    pub mod nether_debug_item_definitions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct NetherDebugItemDefinitions {
        }
        impl NetherDebugItemDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherDebugItemDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherEquipmentItemDefinitions` (org/betterx/datagen/betternether/items/NetherEquipmentItemDefinitions.java)
    pub mod nether_equipment_item_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct NetherEquipmentItemDefinitions {
        }
        impl NetherEquipmentItemDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherEquipmentItemDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherFoodItemDefinitions` (org/betterx/datagen/betternether/items/NetherFoodItemDefinitions.java)
    pub mod nether_food_item_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct NetherFoodItemDefinitions {
        }
        impl NetherFoodItemDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherFoodItemDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherItemDefinitions` (org/betterx/datagen/betternether/items/NetherItemDefinitions.java)
    pub mod nether_item_definitions {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct NetherItemDefinitions {
        }
        impl NetherItemDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherItemDefinitions.registerIngredient(4): not translated, it uses Minecraft's `Item`.
        // NetherItemDefinitions.registerTool(3): not translated, it uses Minecraft's `Item`.
        // NetherItemDefinitions.registerCompatTool(4): not translated, it uses Minecraft's `Item`.
        // NetherItemDefinitions.registerFood(3): not translated, it uses Minecraft's `Item`.
        // NetherItemDefinitions.registerFood(4): not translated, it uses Minecraft's `Item`.
        // NetherItemDefinitions.registerMedicine(5): not translated, it uses Minecraft's `Item`.
        // NetherItemDefinitions.makeEgg(5): not translated, it uses Minecraft's `Item`.
    }

    /// `NetherResourceItemDefinitions` (org/betterx/datagen/betternether/items/NetherResourceItemDefinitions.java)
    pub mod nether_resource_item_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct NetherResourceItemDefinitions {
        }
        impl NetherResourceItemDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherResourceItemDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherSpawnEggDefinitions` (org/betterx/datagen/betternether/items/NetherSpawnEggDefinitions.java)
    pub mod nether_spawn_egg_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct NetherSpawnEggDefinitions {
        }
        impl NetherSpawnEggDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherSpawnEggDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherTemplatesDefinitions` (org/betterx/datagen/betternether/items/NetherTemplatesDefinitions.java)
    pub mod nether_templates_definitions {
        use super::*;
        pub static DEFINED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct NetherTemplatesDefinitions {
        }
        impl NetherTemplatesDefinitions {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NetherTemplatesDefinitions.define(1): not translated, it uses `DefinitionContext` (de.ambertation.wover.item.api.classify).
        // NetherTemplatesDefinitions.smithingTemplate(2): not translated, it uses `ItemDefinition` (de.ambertation.wover.item.api.classify).
    }

    /// `NetherBlockTagDataProvider` (org/betterx/datagen/betternether/NetherBlockTagDataProvider.java)
    pub mod nether_block_tag_data_provider {
        use super::*;
        // The class itself isn't translated: it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider). Its static parts are below.
        // NetherBlockTagDataProvider.NetherBlockTagDataProvider(1): not translated, its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)).
        // NetherBlockTagDataProvider.prepareTags(1): not translated, its class isn't translated (it extends `ForBlocks` (de.ambertation.wover.datagen.api.WoverTagProvider)).
    }

    /// `NetherItemTagDataProvider` (org/betterx/datagen/betternether/NetherItemTagDataProvider.java)
    pub mod nether_item_tag_data_provider {
        use super::*;
        // The class itself isn't translated: it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider). Its static parts are below.
        // NetherItemTagDataProvider.NetherItemTagDataProvider(1): not translated, its class isn't translated (it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider)).
        // NetherItemTagDataProvider.prepareTags(1): not translated, its class isn't translated (it extends `ForItems` (de.ambertation.wover.datagen.api.WoverTagProvider)).
    }

    /// `NetherModelProvider` (org/betterx/datagen/betternether/NetherModelProvider.java)
    pub mod nether_model_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // HAND_AUTHORED_ROOT: not translated, it calls NetherModelProvider.resolveHandAuthoredRoot, which isn't translated.
        // NetherModelProvider.resolveHandAuthoredRoot(0): not translated, it calls `Path` (java.nio.file).of.
        // NetherModelProvider.shipsHandAuthoredModel(1): not translated, `null` (Rust has no null: this needs an Option).
        // NetherModelProvider.bootstrapItemModels(1): not translated, its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherModelProvider.generateTrimmableArmor(3): not translated, its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherModelProvider.generateTrimmableArmor(4): not translated, its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherModelProvider.bootstrapBlockStateModels(1): not translated, its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherModelProvider.NetherModelProvider(1): not translated, its class isn't translated (it extends `WoverModelProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `FlatLevelPresetsDataProvider` (org/betterx/datagen/betternether/presets/FlatLevelPresetsDataProvider.java)
    pub mod flat_level_presets_data_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api). Its static parts are below.
        // BN_FLAT: not translated, it uses Minecraft's `ResourceKey`.
        // FlatLevelPresetsDataProvider.FlatLevelPresetsDataProvider(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
        // FlatLevelPresetsDataProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
    }

    /// `NetherBlockRecipesProvider` (org/betterx/datagen/betternether/recipes/NetherBlockRecipesProvider.java)
    pub mod nether_block_recipes_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // NetherBlockRecipesProvider.NetherBlockRecipesProvider(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherBlockRecipesProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherBlockRecipesProvider.registerSmithing(1): not translated, it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder).
        // NetherBlockRecipesProvider.registerSmelting(1): not translated, it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder).
        // NetherBlockRecipesProvider.registerBlasting(1): not translated, it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder).
        // NetherBlockRecipesProvider.registerStoneCutting(1): not translated, it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder).
        // NetherBlockRecipesProvider.withTemplates(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `NetherBrewingRecipes` (org/betterx/datagen/betternether/recipes/NetherBrewingRecipes.java)
    pub mod nether_brewing_recipes {
        use super::*;
        // The class itself isn't translated: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // NetherBrewingRecipes.NetherBrewingRecipes(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherBrewingRecipes.bootstrap(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherBrewingRecipes.brewingRecipe(5): not translated, it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder).
        // NetherBrewingRecipes.potionOutput(1): not translated, it uses Minecraft's `ItemStackTemplate`.
    }

    /// `NetherChestLootTableProvider` (org/betterx/datagen/betternether/recipes/NetherChestLootTableProvider.java)
    pub mod nether_chest_loot_table_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // NetherChestLootTableProvider.NetherChestLootTableProvider(1): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.boostrap(2): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.surpriseItem(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.ghastHive(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.books(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.workstations(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.armor(4): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.simpleTemplates(2): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.flamingTemplate(2): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.allTemplates(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.bonusSwords(2): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.simpleSwords(1): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.hoes(1): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.enchantedTools(1): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.netherObsidian(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.netherFiller(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.superBonusOreLoot(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.bonusOreLoot(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.simpleOreLoot(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherChestLootTableProvider.simpleCityLoot(0): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `NetherCraftingRecipes` (org/betterx/datagen/betternether/recipes/NetherCraftingRecipes.java)
    pub mod nether_crafting_recipes {
        use super::*;
        // The class itself isn't translated: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // NetherCraftingRecipes.NetherCraftingRecipes(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherCraftingRecipes.bootstrap(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `NetherEntityLootTableProvider` (org/betterx/datagen/betternether/recipes/NetherEntityLootTableProvider.java)
    pub mod nether_entity_loot_table_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // NetherEntityLootTableProvider.NetherEntityLootTableProvider(1): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherEntityLootTableProvider.boostrap(2): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherEntityLootTableProvider.enchantments(1): not translated, it uses Minecraft's `HolderGetter`.
        // NetherEntityLootTableProvider.nagaPlayerKill(1): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherEntityLootTableProvider.killLoot(4): not translated, its class isn't translated (it extends `WoverLootTableProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `NetherItemRecipeProvider` (org/betterx/datagen/betternether/recipes/NetherItemRecipeProvider.java)
    pub mod nether_item_recipe_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // NetherItemRecipeProvider.NetherItemRecipeProvider(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherItemRecipeProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherItemRecipeProvider.registerShapeLess(1): not translated, it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder).
        // NetherItemRecipeProvider.registerSmelting(1): not translated, it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder).
    }

    /// `NetherLootAdditionProvider` (org/betterx/datagen/betternether/recipes/NetherLootAdditionProvider.java)
    pub mod nether_loot_addition_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverLootAdditionProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // NetherLootAdditionProvider.NetherLootAdditionProvider(1): not translated, its class isn't translated (it extends `WoverLootAdditionProvider` (de.ambertation.wover.datagen.api.provider)).
        // NetherLootAdditionProvider.bootstrap(2): not translated, its class isn't translated (it extends `WoverLootAdditionProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `VanillaExcavatorsRecipes` (org/betterx/datagen/betternether/recipes/VanillaExcavatorsRecipes.java)
    pub mod vanilla_excavators_recipes {
        use super::*;
        // The class itself isn't translated: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // VanillaExcavatorsRecipes.VanillaExcavatorsRecipes(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // VanillaExcavatorsRecipes.makeExcavatorRecipe(4): not translated, it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder).
        // VanillaExcavatorsRecipes.bootstrap(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `VanillaHammersRecipes` (org/betterx/datagen/betternether/recipes/VanillaHammersRecipes.java)
    pub mod vanilla_hammers_recipes {
        use super::*;
        // The class itself isn't translated: it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider). Its static parts are below.
        // VanillaHammersRecipes.VanillaHammersRecipes(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
        // VanillaHammersRecipes.makeHammerRecipe(4): not translated, it uses `Context` (de.ambertation.wover.recipe.api.RecipeBuilder).
        // VanillaHammersRecipes.bootstrap(1): not translated, its class isn't translated (it extends `WoverRecipeProvider` (de.ambertation.wover.datagen.api.provider)).
    }

    /// `ObjectFeatureDataProvider` (org/betterx/datagen/betternether/worldgen/features/ObjectFeatureDataProvider.java)
    pub mod object_feature_data_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        // ObjectFeatureDataProvider.ObjectFeatureDataProvider(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // ObjectFeatureDataProvider.bootstrapFeatures(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // ObjectFeatureDataProvider.bootstrapPlaced(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
    }

    /// `OreFeatureDataProvider` (org/betterx/datagen/betternether/worldgen/features/OreFeatureDataProvider.java)
    pub mod ore_feature_data_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        // OreFeatureDataProvider.OreFeatureDataProvider(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // OreFeatureDataProvider.bootstrapFeatures(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // OreFeatureDataProvider.bootstrapPlaced(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
    }

    /// `PlacedTreeFeatureDataProvider` (org/betterx/datagen/betternether/worldgen/features/PlacedTreeFeatureDataProvider.java)
    pub mod placed_tree_feature_data_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        pub const GROVE_SCALE_XZ: f32 = 2.0f32;
        pub const GROVE_SCALE_Y: f32 = 1.0f32;
        pub const GROVE_CORE: f64 = 0.25f64;
        pub const GROVE_EDGE: f64 = 0.05f64;
        // PlacedTreeFeatureDataProvider.PlacedTreeFeatureDataProvider(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // PlacedTreeFeatureDataProvider.bootstrapFeatures(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // PlacedTreeFeatureDataProvider.bootstrapPlaced(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
    }

    /// `TerrainFeatureDataProvider` (org/betterx/datagen/betternether/worldgen/features/TerrainFeatureDataProvider.java)
    pub mod terrain_feature_data_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        // TerrainFeatureDataProvider.TerrainFeatureDataProvider(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // TerrainFeatureDataProvider.bootstrapFeatures(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // TerrainFeatureDataProvider.bootstrapPlaced(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
    }

    /// `VegetationFeatureDataProvider` (org/betterx/datagen/betternether/worldgen/features/VegetationFeatureDataProvider.java)
    pub mod vegetation_feature_data_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        // VegetationFeatureDataProvider.VegetationFeatureDataProvider(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // VegetationFeatureDataProvider.bootstrapFeatures(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // VegetationFeatureDataProvider.bootstrapPlaced(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
    }

    /// `VineFeatureDataProvider` (org/betterx/datagen/betternether/worldgen/features/VineFeatureDataProvider.java)
    pub mod vine_feature_data_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        // VineFeatureDataProvider.VineFeatureDataProvider(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // VineFeatureDataProvider.bootstrapFeatures(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // VineFeatureDataProvider.bootstrapPlaced(1): not translated, its class isn't translated (it extends `WoverFeatureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
    }

    /// `NetherBiomeModificationProvider` (org/betterx/datagen/betternether/worldgen/NetherBiomeModificationProvider.java)
    pub mod nether_biome_modification_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api). Its static parts are below.
        // NetherBiomeModificationProvider.NetherBiomeModificationProvider(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
        // NetherBiomeModificationProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverRegistryContentProvider` (de.ambertation.wover.datagen.api)).
    }

    /// `NetherBiomesProvider` (org/betterx/datagen/betternether/worldgen/NetherBiomesProvider.java)
    pub mod nether_biomes_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverBiomeProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        // NetherBiomesProvider.NetherBiomesProvider(1): not translated, its class isn't translated (it extends `WoverBiomeProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // NetherBiomesProvider.bootstrap(1): not translated, its class isn't translated (it extends `WoverBiomeProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // NetherBiomesProvider.register(3): not translated, it's generic.
    }

    /// `NetherBiomeTagProvider` (org/betterx/datagen/betternether/worldgen/NetherBiomeTagProvider.java)
    pub mod nether_biome_tag_provider {
        use super::*;
        // The class itself isn't translated: it extends `ForBiomes` (de.ambertation.wover.datagen.api.WoverTagProvider). Its static parts are below.
        // WAYSTONES_BLACKSTONE_WAYSTONE: not translated, it uses Minecraft's `TagKey`.
        // NetherBiomeTagProvider.NetherBiomeTagProvider(1): not translated, its class isn't translated (it extends `ForBiomes` (de.ambertation.wover.datagen.api.WoverTagProvider)).
        // NetherBiomeTagProvider.prepareTags(1): not translated, its class isn't translated (it extends `ForBiomes` (de.ambertation.wover.datagen.api.WoverTagProvider)).
    }

    /// `StructureDataProvider` (org/betterx/datagen/betternether/worldgen/StructureDataProvider.java)
    pub mod structure_data_provider {
        use super::*;
        // The class itself isn't translated: it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi). Its static parts are below.
        pub const CITY_SPACING: i32 = 64;
        pub const MEGA_LAVA_LAKE_SPACING: i32 = 24;
        // StructureDataProvider.StructureDataProvider(1): not translated, its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // StructureDataProvider.bootstrapSturctures(1): not translated, its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // StructureDataProvider.bootstrapSets(1): not translated, its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // StructureDataProvider.bootstrapPools(1): not translated, its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // StructureDataProvider.bootstrapProcessors(1): not translated, its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
        // StructureDataProvider.prepareBiomeTags(1): not translated, its class isn't translated (it extends `WoverStructureProvider` (de.ambertation.wover.datagen.api.provider.multi)).
    }

    /// JOML's Vector3f as a value (Java's methods change the vector; the translation writes the result back).
    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    pub struct JVec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    impl JVec3 {
        pub const fn new(x: f32, y: f32, z: f32) -> Self {
            Self { x, y, z }
        }
        pub fn add(self, o: JVec3) -> Self {
            Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
        }
        pub fn add3(self, x: f32, y: f32, z: f32) -> Self {
            Self::new(self.x + x, self.y + y, self.z + z)
        }
        pub fn sub(self, o: JVec3) -> Self {
            Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
        }
        pub fn mul_s(self, s: f32) -> Self {
            Self::new(self.x * s, self.y * s, self.z * s)
        }
        pub fn mul_v(self, o: JVec3) -> Self {
            Self::new(self.x * o.x, self.y * o.y, self.z * o.z)
        }
        pub fn negate(self) -> Self {
            Self::new(-self.x, -self.y, -self.z)
        }
        pub fn dot(self, o: JVec3) -> f32 {
            self.x * o.x + self.y * o.y + self.z * o.z
        }
        pub fn length(self) -> f32 {
            self.dot(self).sqrt()
        }
        pub fn length_squared(self) -> f32 {
            self.dot(self)
        }
        /// JOML's normalize: divides by the length (a zero vector becomes NaNs, as in Java).
        pub fn normalize(self) -> Self {
            let inv = 1.0 / self.length();
            self.mul_s(inv)
        }
        pub fn cross(self, o: JVec3) -> Self {
            Self::new(self.y * o.z - self.z * o.y, self.z * o.x - self.x * o.z, self.x * o.y - self.y * o.x)
        }
        /// JOML's lerp: this + (other - this) * t.
        pub fn lerp(self, o: JVec3, t: f32) -> Self {
            Self::new(self.x + (o.x - self.x) * t, self.y + (o.y - self.y) * t, self.z + (o.z - self.z) * t)
        }
        pub fn distance(self, o: JVec3) -> f32 {
            self.sub(o).length()
        }
    }
    
    /// JOML's Quaternionf (x, y, z, w).
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JQuat {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
    impl Default for JQuat {
        fn default() -> Self {
            Self::IDENTITY
        }
    }
    impl JQuat {
        pub const IDENTITY: JQuat = JQuat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
            Self { x, y, z, w }
        }
        /// `rotationX(angle)`: a turn about +X by `angle` radians (Axis.XP.rotation).
        pub fn rotation_x(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(s, 0.0, 0.0, c)
        }
        pub fn rotation_y(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(0.0, s, 0.0, c)
        }
        pub fn rotation_z(a: f32) -> Self {
            let (s, c) = (a * 0.5).sin_cos();
            Self::new(0.0, 0.0, s, c)
        }
        pub fn normalize(self) -> Self {
            let inv = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
            Self::new(self.x * inv, self.y * inv, self.z * inv, self.w * inv)
        }
        /// this * q.
        pub fn mul(self, q: JQuat) -> Self {
            Self::new(
                self.w * q.x + self.x * q.w + self.y * q.z - self.z * q.y,
                self.w * q.y - self.x * q.z + self.y * q.w + self.z * q.x,
                self.w * q.z + self.x * q.y - self.y * q.x + self.z * q.w,
                self.w * q.w - self.x * q.x - self.y * q.y - self.z * q.z,
            )
        }
        pub fn rotate_x(self, a: f32) -> Self {
            self.mul(Self::rotation_x(a))
        }
        pub fn rotate_y(self, a: f32) -> Self {
            self.mul(Self::rotation_y(a))
        }
        pub fn rotate_z(self, a: f32) -> Self {
            self.mul(Self::rotation_z(a))
        }
    }
    
    /// JOML's Matrix3f, column-major, from three column vectors.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JMat3 {
        pub m: [f32; 9],
    }
    impl JMat3 {
        pub fn from_cols(c0: JVec3, c1: JVec3, c2: JVec3) -> Self {
            Self { m: [c0.x, c0.y, c0.z, c1.x, c1.y, c1.z, c2.x, c2.y, c2.z] }
        }
    }
    
    /// JOML's Matrix4f: column-major, `m[col * 4 + row]`; `a.mul(b)` is a * b, and translate/rotate/scale
    /// multiply on the right, as JOML does.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct JMat4 {
        pub m: [f32; 16],
    }
    impl Default for JMat4 {
        fn default() -> Self {
            Self::IDENTITY
        }
    }
    impl JMat4 {
        pub const IDENTITY: JMat4 = JMat4 { m: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0] };
        pub fn identity() -> Self {
            Self::IDENTITY
        }
        fn at(&self, row: usize, col: usize) -> f32 {
            self.m[col * 4 + row]
        }
        pub fn mul(self, r: JMat4) -> Self {
            let mut out = [0.0f32; 16];
            for col in 0..4 {
                for row in 0..4 {
                    let mut v = 0.0;
                    for k in 0..4 {
                        v += self.at(row, k) * r.at(k, col);
                    }
                    out[col * 4 + row] = v;
                }
            }
            Self { m: out }
        }
        pub fn translation(x: f32, y: f32, z: f32) -> Self {
            let mut t = Self::IDENTITY;
            t.m[12] = x;
            t.m[13] = y;
            t.m[14] = z;
            t
        }
        pub fn translate(self, x: f32, y: f32, z: f32) -> Self {
            self.mul(Self::translation(x, y, z))
        }
        pub fn translate_v(self, v: JVec3) -> Self {
            self.translate(v.x, v.y, v.z)
        }
        pub fn scaling(x: f32, y: f32, z: f32) -> Self {
            let mut t = Self::IDENTITY;
            t.m[0] = x;
            t.m[5] = y;
            t.m[10] = z;
            t
        }
        pub fn scale(self, x: f32, y: f32, z: f32) -> Self {
            self.mul(Self::scaling(x, y, z))
        }
        pub fn scale_s(self, s: f32) -> Self {
            self.scale(s, s, s)
        }
        /// The rotation a unit quaternion stands for.
        pub fn rotation(q: JQuat) -> Self {
            let (x, y, z, w) = (q.x, q.y, q.z, q.w);
            let (xx, yy, zz) = (x * x, y * y, z * z);
            let (xy, xz, yz, wx, wy, wz) = (x * y, x * z, y * z, w * x, w * y, w * z);
            Self {
                m: [
                    1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0,
                    2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0,
                    2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0,
                    0.0, 0.0, 0.0, 1.0,
                ],
            }
        }
        pub fn rotate(self, q: JQuat) -> Self {
            self.mul(Self::rotation(q))
        }
        pub fn rotate_x(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_x(a))
        }
        pub fn rotate_y(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_y(a))
        }
        pub fn rotate_z(self, a: f32) -> Self {
            self.rotate(JQuat::rotation_z(a))
        }
        /// JOML's rotateXYZ: about X, then Y, then Z.
        pub fn rotate_xyz(self, x: f32, y: f32, z: f32) -> Self {
            self.rotate_x(x).rotate_y(y).rotate_z(z)
        }
        /// JOML's rotateAround(q, ox, oy, oz): about the point (ox, oy, oz).
        pub fn rotate_around(self, q: JQuat, ox: f32, oy: f32, oz: f32) -> Self {
            self.translate(ox, oy, oz).rotate(q).translate(-ox, -oy, -oz)
        }
        /// The general inverse (JOML's invert); a singular matrix gives infinities, as in Java.
        pub fn invert(self) -> Self {
            let a = &self.m;
            let mut inv = [0.0f32; 16];
            inv[0] = a[5] * a[10] * a[15] - a[5] * a[11] * a[14] - a[9] * a[6] * a[15] + a[9] * a[7] * a[14] + a[13] * a[6] * a[11] - a[13] * a[7] * a[10];
            inv[4] = -a[4] * a[10] * a[15] + a[4] * a[11] * a[14] + a[8] * a[6] * a[15] - a[8] * a[7] * a[14] - a[12] * a[6] * a[11] + a[12] * a[7] * a[10];
            inv[8] = a[4] * a[9] * a[15] - a[4] * a[11] * a[13] - a[8] * a[5] * a[15] + a[8] * a[7] * a[13] + a[12] * a[5] * a[11] - a[12] * a[7] * a[9];
            inv[12] = -a[4] * a[9] * a[14] + a[4] * a[10] * a[13] + a[8] * a[5] * a[14] - a[8] * a[6] * a[13] - a[12] * a[5] * a[10] + a[12] * a[6] * a[9];
            inv[1] = -a[1] * a[10] * a[15] + a[1] * a[11] * a[14] + a[9] * a[2] * a[15] - a[9] * a[3] * a[14] - a[13] * a[2] * a[11] + a[13] * a[3] * a[10];
            inv[5] = a[0] * a[10] * a[15] - a[0] * a[11] * a[14] - a[8] * a[2] * a[15] + a[8] * a[3] * a[14] + a[12] * a[2] * a[11] - a[12] * a[3] * a[10];
            inv[9] = -a[0] * a[9] * a[15] + a[0] * a[11] * a[13] + a[8] * a[1] * a[15] - a[8] * a[3] * a[13] - a[12] * a[1] * a[11] + a[12] * a[3] * a[9];
            inv[13] = a[0] * a[9] * a[14] - a[0] * a[10] * a[13] - a[8] * a[1] * a[14] + a[8] * a[2] * a[13] + a[12] * a[1] * a[10] - a[12] * a[2] * a[9];
            inv[2] = a[1] * a[6] * a[15] - a[1] * a[7] * a[14] - a[5] * a[2] * a[15] + a[5] * a[3] * a[14] + a[13] * a[2] * a[7] - a[13] * a[3] * a[6];
            inv[6] = -a[0] * a[6] * a[15] + a[0] * a[7] * a[14] + a[4] * a[2] * a[15] - a[4] * a[3] * a[14] - a[12] * a[2] * a[7] + a[12] * a[3] * a[6];
            inv[10] = a[0] * a[5] * a[15] - a[0] * a[7] * a[13] - a[4] * a[1] * a[15] + a[4] * a[3] * a[13] + a[12] * a[1] * a[7] - a[12] * a[3] * a[5];
            inv[14] = -a[0] * a[5] * a[14] + a[0] * a[6] * a[13] + a[4] * a[1] * a[14] - a[4] * a[2] * a[13] - a[12] * a[1] * a[6] + a[12] * a[2] * a[5];
            inv[3] = -a[1] * a[6] * a[11] + a[1] * a[7] * a[10] + a[5] * a[2] * a[11] - a[5] * a[3] * a[10] - a[9] * a[2] * a[7] + a[9] * a[3] * a[6];
            inv[7] = a[0] * a[6] * a[11] - a[0] * a[7] * a[10] - a[4] * a[2] * a[11] + a[4] * a[3] * a[10] + a[8] * a[2] * a[7] - a[8] * a[3] * a[6];
            inv[11] = -a[0] * a[5] * a[11] + a[0] * a[7] * a[9] + a[4] * a[1] * a[11] - a[4] * a[3] * a[9] - a[8] * a[1] * a[7] + a[8] * a[3] * a[5];
            inv[15] = a[0] * a[5] * a[10] - a[0] * a[6] * a[9] - a[4] * a[1] * a[10] + a[4] * a[2] * a[9] + a[8] * a[1] * a[6] - a[8] * a[2] * a[5];
            let det = a[0] * inv[0] + a[1] * inv[4] + a[2] * inv[8] + a[3] * inv[12];
            let d = 1.0 / det;
            for v in inv.iter_mut() {
                *v *= d;
            }
            Self { m: inv }
        }
        /// The point v moved by this matrix (w = 1).
        pub fn transform_position(&self, v: JVec3) -> JVec3 {
            JVec3::new(
                self.at(0, 0) * v.x + self.at(0, 1) * v.y + self.at(0, 2) * v.z + self.at(0, 3),
                self.at(1, 0) * v.x + self.at(1, 1) * v.y + self.at(1, 2) * v.z + self.at(1, 3),
                self.at(2, 0) * v.x + self.at(2, 1) * v.y + self.at(2, 2) * v.z + self.at(2, 3),
            )
        }
        /// The direction v turned by this matrix (w = 0).
        pub fn transform_direction(&self, v: JVec3) -> JVec3 {
            JVec3::new(
                self.at(0, 0) * v.x + self.at(0, 1) * v.y + self.at(0, 2) * v.z,
                self.at(1, 0) * v.x + self.at(1, 1) * v.y + self.at(1, 2) * v.z,
                self.at(2, 0) * v.x + self.at(2, 1) * v.y + self.at(2, 2) * v.z,
            )
        }
        /// JOML's set3x3: the upper-left 3x3 from `m`, the rest kept.
        pub fn set3x3(self, m: JMat3) -> Self {
            let mut out = self;
            for col in 0..3 {
                for row in 0..3 {
                    out.m[col * 4 + row] = m.m[col * 3 + row];
                }
            }
            out
        }
        /// JOML's transpose3x3: the upper-left 3x3 transposed, the rest kept.
        pub fn transpose3x3(self) -> Self {
            let mut out = self;
            for col in 0..3 {
                for row in 0..3 {
                    out.m[col * 4 + row] = self.m[row * 4 + col];
                }
            }
            out
        }
        pub fn get_translation(&self) -> JVec3 {
            JVec3::new(self.m[12], self.m[13], self.m[14])
        }
    }
    
    /// Minecraft's PoseStack: a stack of matrices (normals aren't kept: Horizon's hand renderer takes one matrix).
    #[derive(Clone, Debug)]
    pub struct JPoseStack {
        pub stack: Vec<JMat4>,
    }
    impl Default for JPoseStack {
        fn default() -> Self {
            Self { stack: vec![JMat4::IDENTITY] }
        }
    }
    impl JPoseStack {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn of(m: JMat4) -> Self {
            Self { stack: vec![m] }
        }
        fn top(&mut self) -> &mut JMat4 {
            self.stack.last_mut().expect("PoseStack is empty")
        }
        pub fn push_pose(&mut self) {
            let m = *self.top();
            self.stack.push(m);
        }
        pub fn pop_pose(&mut self) {
            self.stack.pop();
        }
        /// Writes the top matrix (Java changed `pose.last().pose()` in place).
        pub fn set_last(&mut self, m: JMat4) {
            *self.top() = m;
        }
        pub fn last_pose(&self) -> JMat4 {
            *self.stack.last().expect("PoseStack is empty")
        }
        pub fn set_identity(&mut self) {
            *self.top() = JMat4::IDENTITY;
        }
        pub fn translate(&mut self, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.translate(x, y, z);
        }
        pub fn scale(&mut self, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.scale(x, y, z);
        }
        pub fn mul_pose(&mut self, q: JQuat) {
            let m = self.top();
            *m = m.rotate(q);
        }
        pub fn mul_pose_matrix(&mut self, r: JMat4) {
            let m = self.top();
            *m = m.mul(r);
        }
        pub fn rotate_around(&mut self, q: JQuat, x: f32, y: f32, z: f32) {
            let m = self.top();
            *m = m.rotate_around(q, x, y, z);
        }
    }
    
    #[cfg(test)]
    mod joml_tests {
        use super::*;
    
        fn close(a: JVec3, b: JVec3) -> bool {
            (a.x - b.x).abs() < 1e-5 && (a.y - b.y).abs() < 1e-5 && (a.z - b.z).abs() < 1e-5
        }
    
        #[test]
        fn rotations_turn_axes_as_joml_does() {
            let q = JQuat::rotation_x(std::f32::consts::FRAC_PI_2);
            // A positive turn about X takes +Y to +Z (right-handed), as JOML's rotationX.
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(0.0, 1.0, 0.0)), JVec3::new(0.0, 0.0, 1.0)));
            let q = JQuat::rotation_y(std::f32::consts::FRAC_PI_2);
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(0.0, 0.0, 1.0)), JVec3::new(1.0, 0.0, 0.0)));
            let q = JQuat::rotation_z(std::f32::consts::FRAC_PI_2);
            assert!(close(JMat4::rotation(q).transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(0.0, 1.0, 0.0)));
        }
    
        #[test]
        fn translate_then_rotate_multiplies_on_the_right() {
            // JOML: new Matrix4f().translate(1, 0, 0).rotateZ(90) moves (1, 0, 0) to (1, 1, 0).
            let m = JMat4::identity().translate(1.0, 0.0, 0.0).rotate_z(std::f32::consts::FRAC_PI_2);
            assert!(close(m.transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(1.0, 1.0, 0.0)));
        }
    
        #[test]
        fn invert_undoes() {
            let m = JMat4::identity().translate(1.0, 2.0, 3.0).rotate_x(0.7).rotate_y(-0.3).scale(2.0, 0.5, 1.5);
            let p = JVec3::new(0.3, -1.2, 4.0);
            assert!(close(m.invert().transform_position(m.transform_position(p)), p));
            assert!(close(m.mul(m.invert()).transform_position(p), p));
        }
    
        #[test]
        fn rotate_around_keeps_its_point() {
            let m = JMat4::identity().rotate_around(JQuat::rotation_y(1.1), 0.5, 0.25, 0.0);
            assert!(close(m.transform_position(JVec3::new(0.5, 0.25, 0.0)), JVec3::new(0.5, 0.25, 0.0)));
        }
    
        #[test]
        fn pose_stack_push_pop() {
            let mut p = JPoseStack::new();
            p.translate(1.0, 0.0, 0.0);
            p.push_pose();
            p.scale(2.0, 2.0, 2.0);
            assert!(close(p.last_pose().transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(3.0, 0.0, 0.0)));
            p.pop_pose();
            assert!(close(p.last_pose().transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(2.0, 0.0, 0.0)));
        }
    
        #[test]
        fn quaternion_matrix_agrees_with_axis_turns() {
            let q = JQuat::rotation_x(0.4).mul(JQuat::rotation_y(-0.9));
            let a = JMat4::rotation(q);
            let b = JMat4::identity().rotate_x(0.4).rotate_y(-0.9);
            let p = JVec3::new(1.0, 2.0, 3.0);
            assert!(close(a.transform_position(p), b.transform_position(p)));
        }
    
        #[test]
        fn set3x3_and_transpose() {
            let m3 = JMat3::from_cols(JVec3::new(0.0, 1.0, 0.0), JVec3::new(-1.0, 0.0, 0.0), JVec3::new(0.0, 0.0, 1.0));
            let m = JMat4::identity().set3x3(m3);
            assert!(close(m.transform_position(JVec3::new(1.0, 0.0, 0.0)), JVec3::new(0.0, 1.0, 0.0)));
            assert!(close(m.transpose3x3().transform_position(JVec3::new(0.0, 1.0, 0.0)), JVec3::new(1.0, 0.0, 0.0)));
        }
    }

    
    // ---- Java regular expressions (java.util.regex), a small backtracking engine ----
    // Bundles can't add the regex crate, so this covers what mods use: literals, `.`, classes with ranges and
    // \d \s \w, groups (capturing, `(?:`, `(?<name>`), lookahead, alternation, greedy/lazy quantifiers, ^ $ \b.
    // Positions are in chars, like the translated String methods.
    
    #[derive(Clone, Debug)]
    enum JClassItem {
        Range(char, char),
        Digit(bool),
        Space(bool),
        Word(bool),
    }
    
    #[derive(Clone, Debug)]
    enum JNode {
        Char(char),
        Any,
        Class(Vec<JClassItem>, bool),
        Start,
        End,
        WordB(bool),
        Group(Box<JNode>, Option<usize>),
        Look(Box<JNode>, bool),
        Alt(Vec<JNode>),
        Concat(Vec<JNode>),
        Repeat(Box<JNode>, u32, u32, bool),
        Backref(usize),
    }
    
    type JCaps = Vec<Option<(usize, usize)>>;
    
    /// A compiled `Pattern`.
    #[derive(Clone, Debug)]
    pub struct JRegex {
        node: JNode,
        groups: usize,
        names: Vec<(String, usize)>,
        ci: bool,
    }
    
    fn j_is_word(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
    
    fn j_is_space(c: char) -> bool {
        matches!(c, ' ' | '\t' | '\n' | '\u{0B}' | '\u{0C}' | '\r')
    }
    
    struct JRegexParser<'a> {
        p: &'a [char],
        i: usize,
        groups: usize,
        names: Vec<(String, usize)>,
    }
    
    impl JRegexParser<'_> {
        fn peek(&self) -> Option<char> {
            self.p.get(self.i).copied()
        }
        fn eat(&mut self, c: char) -> bool {
            if self.peek() == Some(c) {
                self.i += 1;
                true
            } else {
                false
            }
        }
        fn alt(&mut self) -> JNode {
            let mut arms = vec![self.concat()];
            while self.eat('|') {
                arms.push(self.concat());
            }
            if arms.len() == 1 { arms.pop().unwrap() } else { JNode::Alt(arms) }
        }
        fn concat(&mut self) -> JNode {
            let mut items = Vec::new();
            while let Some(c) = self.peek() {
                if c == '|' || c == ')' {
                    break;
                }
                let atom = self.atom();
                items.push(self.quantified(atom));
            }
            JNode::Concat(items)
        }
        fn number(&mut self) -> Option<u32> {
            let start = self.i;
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.i += 1;
            }
            if start == self.i { None } else { self.p[start..self.i].iter().collect::<String>().parse().ok() }
        }
        fn quantified(&mut self, atom: JNode) -> JNode {
            let (min, max) = match self.peek() {
                Some('*') => (0, u32::MAX),
                Some('+') => (1, u32::MAX),
                Some('?') => (0, 1),
                Some('{') => {
                    let save = self.i;
                    self.i += 1;
                    let Some(lo) = self.number() else {
                        self.i = save;
                        return atom;
                    };
                    let hi = if self.eat(',') { self.number().unwrap_or(u32::MAX) } else { lo };
                    if !self.eat('}') {
                        panic!("bad repetition in regex");
                    }
                    self.i -= 1;
                    (lo, hi)
                }
                _ => return atom,
            };
            self.i += 1;
            let greedy = !self.eat('?');
            if greedy {
                self.eat('+');
            }
            self.quantified(JNode::Repeat(Box::new(atom), min, max, greedy))
        }
        fn escape(&mut self, in_class: bool) -> JClassItem {
            let c = self.peek().expect("regex ends with a backslash");
            self.i += 1;
            let lit = |c: char| JClassItem::Range(c, c);
            match c {
                'd' => JClassItem::Digit(true),
                'D' => JClassItem::Digit(false),
                's' => JClassItem::Space(true),
                'S' => JClassItem::Space(false),
                'w' => JClassItem::Word(true),
                'W' => JClassItem::Word(false),
                't' => lit('\t'),
                'n' => lit('\n'),
                'r' => lit('\r'),
                'f' => lit('\u{0C}'),
                'e' => lit('\u{1B}'),
                '0' => lit('\0'),
                'u' => {
                    let hex: String = self.p[self.i..self.i + 4].iter().collect();
                    self.i += 4;
                    lit(char::from_u32(u32::from_str_radix(&hex, 16).expect("bad \\u escape")).unwrap_or('\u{FFFD}'))
                }
                'x' => {
                    let hex: String = self.p[self.i..self.i + 2].iter().collect();
                    self.i += 2;
                    lit(char::from_u32(u32::from_str_radix(&hex, 16).expect("bad \\x escape")).unwrap_or('\u{FFFD}'))
                }
                c if !in_class && c.is_ascii_alphabetic() => panic!("unsupported regex escape \\{c}"),
                c => lit(c),
            }
        }
        fn class(&mut self) -> JNode {
            let negated = self.eat('^');
            let mut items = Vec::new();
            let mut first = true;
            loop {
                let c = self.peek().expect("unclosed [ in regex");
                if c == ']' && !first {
                    self.i += 1;
                    break;
                }
                first = false;
                if c == '[' {
                    panic!("nested character classes aren't supported");
                }
                self.i += 1;
                let item = if c == '\\' { self.escape(true) } else { JClassItem::Range(c, c) };
                // a-z
                if let JClassItem::Range(lo, _) = item
                    && self.peek() == Some('-')
                    && self.p.get(self.i + 1).is_some_and(|&n| n != ']')
                {
                    self.i += 1;
                    let hc = self.peek().unwrap();
                    self.i += 1;
                    let hi = if hc == '\\' {
                        match self.escape(true) {
                            JClassItem::Range(h, _) => h,
                            _ => panic!("bad range in regex"),
                        }
                    } else {
                        hc
                    };
                    items.push(JClassItem::Range(lo, hi));
                    continue;
                }
                items.push(item);
            }
            JNode::Class(items, negated)
        }
        fn atom(&mut self) -> JNode {
            let c = self.peek().unwrap();
            self.i += 1;
            match c {
                '.' => JNode::Any,
                '^' => JNode::Start,
                '$' => JNode::End,
                '[' => self.class(),
                '(' => {
                    let mut index = None;
                    let mut look = None;
                    if self.eat('?') {
                        if self.eat(':') {
                        } else if self.eat('=') {
                            look = Some(false);
                        } else if self.eat('!') {
                            look = Some(true);
                        } else if self.eat('<') {
                            let start = self.i;
                            while self.peek().is_some_and(|c| c != '>') {
                                self.i += 1;
                            }
                            let name: String = self.p[start..self.i].iter().collect();
                            self.i += 1;
                            self.groups += 1;
                            self.names.push((name, self.groups));
                            index = Some(self.groups);
                        } else {
                            panic!("unsupported regex group (?{:?}", self.peek());
                        }
                    } else {
                        self.groups += 1;
                        index = Some(self.groups);
                    }
                    let inner = self.alt();
                    if !self.eat(')') {
                        panic!("unclosed ( in regex");
                    }
                    match look {
                        Some(neg) => JNode::Look(Box::new(inner), neg),
                        None => JNode::Group(Box::new(inner), index),
                    }
                }
                '\\' => match self.peek() {
                    Some('b') => {
                        self.i += 1;
                        JNode::WordB(true)
                    }
                    Some('B') => {
                        self.i += 1;
                        JNode::WordB(false)
                    }
                    Some('A') => {
                        self.i += 1;
                        JNode::Start
                    }
                    Some('z') | Some('Z') => {
                        self.i += 1;
                        JNode::End
                    }
                    Some(d) if d.is_ascii_digit() && d != '0' => {
                        self.i += 1;
                        JNode::Backref(d as usize - '0' as usize)
                    }
                    Some('Q') => {
                        self.i += 1;
                        let mut lits = Vec::new();
                        while self.i < self.p.len() && !(self.p[self.i] == '\\' && self.p.get(self.i + 1) == Some(&'E')) {
                            lits.push(JNode::Char(self.p[self.i]));
                            self.i += 1;
                        }
                        self.i = (self.i + 2).min(self.p.len());
                        JNode::Concat(lits)
                    }
                    _ => match self.escape(false) {
                        JClassItem::Range(c, _) => JNode::Char(c),
                        other => JNode::Class(vec![other], false),
                    },
                },
                c => JNode::Char(c),
            }
        }
    }
    
    impl JRegex {
        /// `Pattern.compile(p)`. Java throws PatternSyntaxException on a bad pattern; this panics.
        pub fn new(p: &str) -> Self {
            Self::with_flags(p, 0)
        }
        /// `Pattern.compile(p, flags)`: CASE_INSENSITIVE (2) is honoured.
        pub fn with_flags(p: &str, flags: i32) -> Self {
            let chars: Vec<char> = p.chars().collect();
            let (chars, ci) = match chars.strip_prefix(&['(', '?', 'i', ')']) {
                Some(rest) => (rest.to_vec(), true),
                None => (chars, flags & 2 != 0),
            };
            let mut parser = JRegexParser { p: &chars, i: 0, groups: 0, names: Vec::new() };
            let node = parser.alt();
            if parser.i != chars.len() {
                panic!("unmatched ) in regex {p:?}");
            }
            JRegex { node, groups: parser.groups, names: parser.names, ci }
        }
    
        fn same(&self, a: char, b: char) -> bool {
            a == b || self.ci && (j_lower_char(a) == j_lower_char(b) || j_upper_char(a) == j_upper_char(b))
        }
    
        fn in_class(&self, items: &[JClassItem], c: char) -> bool {
            items.iter().any(|it| match *it {
                JClassItem::Range(lo, hi) => {
                    (lo..=hi).contains(&c) || self.ci && ((lo..=hi).contains(&j_lower_char(c)) || (lo..=hi).contains(&j_upper_char(c)))
                }
                JClassItem::Digit(yes) => c.is_ascii_digit() == yes,
                JClassItem::Space(yes) => j_is_space(c) == yes,
                JClassItem::Word(yes) => j_is_word(c) == yes,
            })
        }
    
        fn m(&self, n: &JNode, s: &[char], i: usize, caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            match n {
                JNode::Char(c) => i < s.len() && self.same(s[i], *c) && k(i + 1, caps),
                JNode::Any => i < s.len() && !matches!(s[i], '\n' | '\r' | '\u{85}' | '\u{2028}' | '\u{2029}') && k(i + 1, caps),
                JNode::Class(items, neg) => i < s.len() && self.in_class(items, s[i]) != *neg && k(i + 1, caps),
                JNode::Start => i == 0 && k(i, caps),
                JNode::End => (i == s.len() || i + 1 == s.len() && s[i] == '\n') && k(i, caps),
                JNode::WordB(yes) => {
                    let before = i > 0 && j_is_word(s[i - 1]);
                    let after = i < s.len() && j_is_word(s[i]);
                    (before != after) == *yes && k(i, caps)
                }
                JNode::Group(inner, index) => match *index {
                    None => self.m(inner, s, i, caps, k),
                    Some(g) => self.m(inner, s, i, caps, &mut |j, caps: &mut JCaps| {
                        let prev = caps[g];
                        caps[g] = Some((i, j));
                        if k(j, caps) {
                            return true;
                        }
                        caps[g] = prev;
                        false
                    }),
                },
                JNode::Look(inner, neg) => {
                    let mut probe = caps.clone();
                    let found = self.m(inner, s, i, &mut probe, &mut |_, _| true);
                    if found == *neg {
                        return false;
                    }
                    if !neg {
                        *caps = probe;
                    }
                    k(i, caps)
                }
                JNode::Alt(arms) => arms.iter().any(|a| self.m(a, s, i, caps, k)),
                JNode::Concat(items) => self.seq(items, s, i, caps, k),
                JNode::Repeat(inner, min, max, greedy) => self.rep(inner, *min, *max, *greedy, 0, s, i, caps, k),
                JNode::Backref(g) => match caps.get(*g).copied().flatten() {
                    Some((a, b)) => {
                        let len = b - a;
                        i + len <= s.len() && (0..len).all(|d| self.same(s[a + d], s[i + d])) && k(i + len, caps)
                    }
                    None => false,
                },
            }
        }
    
        fn seq(&self, items: &[JNode], s: &[char], i: usize, caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            match items.split_first() {
                None => k(i, caps),
                Some((first, rest)) => self.m(first, s, i, caps, &mut |j, caps: &mut JCaps| self.seq(rest, s, j, caps, k)),
            }
        }
    
        #[allow(clippy::too_many_arguments)]
        fn rep(&self, n: &JNode, min: u32, max: u32, greedy: bool, count: u32, s: &[char], i: usize, caps: &mut JCaps,
               k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            let more = |caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool| {
                count < max
                    && self.m(n, s, i, caps, &mut |j, caps: &mut JCaps| {
                        // An empty pass can't make progress: stop repeating once the minimum is met.
                        (j != i || count < min) && self.rep(n, min, max, greedy, count + 1, s, j, caps, k)
                    })
            };
            if count < min {
                return more(caps, k);
            }
            if greedy { more(caps, k) || k(i, caps) } else { k(i, caps) || more(caps, k) }
        }
    
        /// A match starting exactly at `i`; `whole`: it must also end at the end of the text.
        fn at(&self, s: &[char], i: usize, whole: bool) -> Option<JCaps> {
            let mut caps: JCaps = vec![None; self.groups + 1];
            let mut end = None;
            let ok = self.m(&self.node, s, i, &mut caps, &mut |j, _| {
                if whole && j != s.len() {
                    return false;
                }
                end = Some(j);
                true
            });
            if !ok {
                return None;
            }
            caps[0] = Some((i, end.unwrap()));
            Some(caps)
        }
    
        fn find_from(&self, s: &[char], from: usize) -> Option<JCaps> {
            (from..=s.len()).find_map(|i| self.at(s, i, false))
        }
    
        /// `String.matches` / `Pattern.matches`: the whole text.
        pub fn matches(&self, s: &str) -> bool {
            let cs: Vec<char> = s.chars().collect();
            self.at(&cs, 0, true).is_some()
        }
    
        /// `Pattern.matcher(s).find()` as a test.
        pub fn is_found(&self, s: &str) -> bool {
            let cs: Vec<char> = s.chars().collect();
            self.find_from(&cs, 0).is_some()
        }
    
        fn expand(&self, s: &[char], caps: &JCaps, rep: &str, out: &mut String) {
            let r: Vec<char> = rep.chars().collect();
            let mut i = 0;
            while i < r.len() {
                match r[i] {
                    '\\' if i + 1 < r.len() => {
                        out.push(r[i + 1]);
                        i += 2;
                    }
                    '$' if i + 1 < r.len() && r[i + 1].is_ascii_digit() => {
                        let g = r[i + 1] as usize - '0' as usize;
                        if let Some((a, b)) = caps.get(g).copied().flatten() {
                            out.extend(&s[a..b]);
                        } else if g > self.groups {
                            panic!("No group {g}");
                        }
                        i += 2;
                    }
                    '$' if r.get(i + 1) == Some(&'{') => {
                        let close = r[i..].iter().position(|&c| c == '}').expect("bad ${name} in replacement") + i;
                        let name: String = r[i + 2..close].iter().collect();
                        let g = self.names.iter().find(|(n, _)| *n == name).map(|(_, g)| *g).expect("no group with that name");
                        if let Some((a, b)) = caps[g] {
                            out.extend(&s[a..b]);
                        }
                        i = close + 1;
                    }
                    c => {
                        out.push(c);
                        i += 1;
                    }
                }
            }
        }
    
        fn replace(&self, s: &str, rep: &str, all: bool) -> String {
            let cs: Vec<char> = s.chars().collect();
            let mut out = String::new();
            let (mut pos, mut from) = (0, 0);
            while from <= cs.len() {
                let Some(caps) = self.find_from(&cs, from) else { break };
                let (a, b) = caps[0].unwrap();
                out.extend(&cs[pos..a]);
                self.expand(&cs, &caps, rep, &mut out);
                pos = b;
                from = if b == a { b + 1 } else { b };
                if b == a && a < cs.len() {
                    out.push(cs[a]);
                    pos = a + 1;
                }
                if !all {
                    break;
                }
            }
            if pos <= cs.len() {
                out.extend(&cs[pos..]);
            }
            out
        }
    
        /// `String.replaceAll` / `Matcher.replaceAll`, with Java's `$1` and `\$` in the replacement.
        pub fn replace_all(&self, s: &str, rep: &str) -> String {
            self.replace(s, rep, true)
        }
    
        pub fn replace_first(&self, s: &str, rep: &str) -> String {
            self.replace(s, rep, false)
        }
    
        /// `String.split(regex, limit)` / `Pattern.split`, with Java's rules for empty parts.
        pub fn split(&self, s: &str, limit: i32) -> Vec<String> {
            let cs: Vec<char> = s.chars().collect();
            let mut parts: Vec<String> = Vec::new();
            let (mut pos, mut from) = (0, 0);
            while from <= cs.len() {
                if limit > 0 && parts.len() as i32 == limit - 1 {
                    break;
                }
                let Some(caps) = self.find_from(&cs, from) else { break };
                let (a, b) = caps[0].unwrap();
                if b == 0 {
                    // A zero-width match at the start makes no leading empty part.
                    from = 1;
                    continue;
                }
                if a == b && a >= cs.len() {
                    break;
                }
                parts.push(cs[pos..a].iter().collect());
                pos = b;
                from = if a == b { b + 1 } else { b };
            }
            parts.push(cs[pos.min(cs.len())..].iter().collect());
            if limit == 0 {
                while parts.len() > 1 && parts.last().is_some_and(|p| p.is_empty()) {
                    parts.pop();
                }
                if parts.len() == 1 && parts[0].is_empty() && !s.is_empty() {
                    parts.clear();
                }
            }
            parts
        }
    }
    
    /// A `Matcher`: a regex over one text, with the last match.
    #[derive(Clone, Debug)]
    pub struct JMatcher {
        re: JRegex,
        text: Vec<char>,
        from: usize,
        caps: Option<JCaps>,
    }
    
    impl JMatcher {
        pub fn new(re: &JRegex, s: &str) -> Self {
            JMatcher { re: re.clone(), text: s.chars().collect(), from: 0, caps: None }
        }
        pub fn find(&mut self) -> bool {
            if self.from > self.text.len() {
                self.caps = None;
                return false;
            }
            self.caps = self.re.find_from(&self.text, self.from);
            if let Some((a, b)) = self.caps.as_ref().and_then(|c| c[0]) {
                self.from = if a == b { b + 1 } else { b };
                true
            } else {
                self.from = self.text.len() + 1;
                false
            }
        }
        pub fn find_at(&mut self, start: i32) -> bool {
            self.from = start as usize;
            self.find()
        }
        pub fn matches(&mut self) -> bool {
            self.caps = self.re.at(&self.text, 0, true);
            self.caps.is_some()
        }
        pub fn looking_at(&mut self) -> bool {
            self.caps = self.re.at(&self.text, 0, false);
            self.caps.is_some()
        }
        fn span(&self, g: usize) -> Option<(usize, usize)> {
            let caps = self.caps.as_ref().expect("No match found");
            assert!(g < caps.len(), "No group {g}");
            caps[g]
        }
        /// `group(n)`: null (None) when that group took no part in the match.
        pub fn group(&self, g: i32) -> Option<String> {
            self.span(g as usize).map(|(a, b)| self.text[a..b].iter().collect())
        }
        pub fn group_named(&self, name: &str) -> Option<String> {
            let g = self.re.names.iter().find(|(n, _)| n == name).map(|(_, g)| *g).expect("No group with that name");
            self.group(g as i32)
        }
        pub fn start(&self, g: i32) -> i32 {
            self.span(g as usize).map_or(-1, |(a, _)| a as i32)
        }
        pub fn end(&self, g: i32) -> i32 {
            self.span(g as usize).map_or(-1, |(_, b)| b as i32)
        }
        pub fn group_count(&self) -> i32 {
            self.re.groups as i32
        }
        pub fn replace_all(&self, rep: &str) -> String {
            self.re.replace_all(&self.text.iter().collect::<String>(), rep)
        }
    }
    
    #[cfg(test)]
    mod jregex_tests {
        use super::*;
    
        #[test]
        fn groups_and_lazy() {
            let re = JRegex::new("^(?:\\S )?(?:VERY SPECIAL|COMMON|RARE|EPIC)(?: DUNGEON)?(?: (.+?))?(?: \\S)?$");
            let mut m = JMatcher::new(&re, "EPIC DUNGEON SWORD");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("SWORD"));
            let mut m = JMatcher::new(&re, "a RARE FISHING ROD b");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("FISHING ROD"));
            let mut m = JMatcher::new(&re, "COMMON");
            assert!(m.matches());
            assert_eq!(m.group(1), None);
        }
    
        #[test]
        fn classes_counts_find() {
            let re = JRegex::new("server/[0-9A-Fa-f]{8}/(.+)");
            let mut m = JMatcher::new(&re, "server/0aF3b2c1/pack.zip");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("pack.zip"));
            assert!(!JRegex::new("server/[0-9A-Fa-f]{8}/(.+)").matches("server/xyz/a"));
            let re = JRegex::new("\"id\"\\s*:\\s*\"([a-z0-9_.-]+:[a-z0-9_./-]+)\"");
            let mut m = JMatcher::new(&re, "{\"id\": \"minecraft:item/a\", \"id\":\"x:y\"}");
            assert!(m.find());
            assert_eq!(m.group(1).as_deref(), Some("minecraft:item/a"));
            assert!(m.find());
            assert_eq!(m.group(1).as_deref(), Some("x:y"));
            assert!(!m.find());
        }
    
        #[test]
        fn split_and_replace() {
            assert_eq!(JRegex::new("\\s*,\\s*").split("a , b,c,,", 0), vec!["a", "b", "c"]);
            assert_eq!(JRegex::new(",").split("a,b,c", 2), vec!["a", "b,c"]);
            assert_eq!(JRegex::new("").split("abc", 0), vec!["a", "b", "c"]);
            assert_eq!(JRegex::new("(\\w+)@(\\w+)").replace_all("x a@b y c@d", "$2.$1"), "x b.a y d.c");
            assert_eq!(JRegex::new("a*").replace_all("baaac", "-"), "-b--c-");
            assert!(JRegex::with_flags("abc", 2).matches("AbC"));
        }
    }

    
    // ---- Gson (com.google.gson): JSON values, a reader and Gson's writer ----
    // JsonElement, JsonObject, JsonArray and JsonPrimitive are all one Rust type. Numbers keep their text, as
    // Gson's LazilyParsedNumber does. Objects keep their insertion order, as Gson's LinkedTreeMap does.
    
    #[derive(Clone, Debug, PartialEq)]
    pub enum JJson {
        Null,
        Bool(bool),
        Num(String),
        Str(String),
        Arr(Vec<JJson>),
        Obj(Vec<(String, JJson)>),
    }
    
    /// A float or double as Java prints it (Float.toString / Double.toString).
    pub fn j_java_num(x: f64, float: bool) -> String {
        if x.is_nan() {
            return "NaN".into();
        }
        if x.is_infinite() {
            return if x > 0.0 { "Infinity".into() } else { "-Infinity".into() };
        }
        let a = x.abs();
        if x == 0.0 {
            return if x.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
        }
        let digits = if float { format!("{:e}", x as f32) } else { format!("{:e}", x) };
        let (mant, exp) = digits.split_once('e').unwrap();
        let exp: i32 = exp.parse().unwrap();
        if (1e-3..1e7).contains(&a) {
            let plain = if float { format!("{}", x as f32) } else { format!("{}", x) };
            if plain.contains('.') { plain } else { plain + ".0" }
        } else {
            let mant = if mant.contains('.') { mant.to_string() } else { format!("{mant}.0") };
            format!("{mant}E{exp}")
        }
    }
    
    struct JJsonReader<'a> {
        s: &'a [char],
        i: usize,
    }
    
    impl JJsonReader<'_> {
        fn ws(&mut self) {
            while self.i < self.s.len() && self.s[self.i].is_whitespace() {
                self.i += 1;
            }
        }
        fn fail(&self, what: &str) -> ! {
            panic!("JsonSyntaxException: {what} at {}", self.i)
        }
        fn value(&mut self) -> JJson {
            self.ws();
            match self.s.get(self.i) {
                Some('{') => {
                    self.i += 1;
                    let mut members = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&'}') {
                        self.i += 1;
                        return JJson::Obj(members);
                    }
                    loop {
                        self.ws();
                        let k = match self.s.get(self.i) {
                            Some('"') | Some('\'') => self.string(),
                            _ => self.bare(),
                        };
                        self.ws();
                        if !matches!(self.s.get(self.i), Some(':') | Some('=')) {
                            self.fail("expected ':'");
                        }
                        self.i += 1;
                        let v = self.value();
                        // A repeated name: Gson keeps the last one.
                        members.retain(|(n, _): &(String, JJson)| *n != k);
                        members.push((k, v));
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some('}') => {
                                self.i += 1;
                                return JJson::Obj(members);
                            }
                            _ => self.fail("expected ',' or '}'"),
                        }
                    }
                }
                Some('[') => {
                    self.i += 1;
                    let mut items = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&']') {
                        self.i += 1;
                        return JJson::Arr(items);
                    }
                    loop {
                        items.push(self.value());
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some(']') => {
                                self.i += 1;
                                return JJson::Arr(items);
                            }
                            _ => self.fail("expected ',' or ']'"),
                        }
                    }
                }
                Some('"') | Some('\'') => JJson::Str(self.string()),
                Some(_) => {
                    let word = self.bare();
                    match word.as_str() {
                        "true" => JJson::Bool(true),
                        "false" => JJson::Bool(false),
                        "null" => JJson::Null,
                        w if w.parse::<f64>().is_ok() => JJson::Num(w.to_string()),
                        // Gson is lenient: an unquoted word is a string.
                        w => JJson::Str(w.to_string()),
                    }
                }
                None => self.fail("end of input"),
            }
        }
        fn bare(&mut self) -> String {
            let start = self.i;
            while self.i < self.s.len() && !matches!(self.s[self.i], ',' | ':' | '=' | ';' | '}' | ']' | '{' | '[' | '"' | '\'') && !self.s[self.i].is_whitespace() {
                self.i += 1;
            }
            if start == self.i {
                self.fail("expected a value");
            }
            self.s[start..self.i].iter().collect()
        }
        fn string(&mut self) -> String {
            let q = self.s[self.i];
            self.i += 1;
            let mut out = String::new();
            loop {
                let Some(&c) = self.s.get(self.i) else { self.fail("unterminated string") };
                self.i += 1;
                match c {
                    c if c == q => return out,
                    '\\' => {
                        let Some(&e) = self.s.get(self.i) else { self.fail("unterminated string") };
                        self.i += 1;
                        match e {
                            'n' => out.push('\n'),
                            't' => out.push('\t'),
                            'r' => out.push('\r'),
                            'b' => out.push('\u{8}'),
                            'f' => out.push('\u{c}'),
                            'u' => {
                                let hex: String = self.s[self.i..(self.i + 4).min(self.s.len())].iter().collect();
                                self.i += 4;
                                let unit = u16::from_str_radix(&hex, 16).unwrap_or_else(|_| self.fail("bad \\u escape"));
                                // A surrogate pair: two escapes make one char.
                                if (0xD800..0xDC00).contains(&unit) && self.s.get(self.i) == Some(&'\\') && self.s.get(self.i + 1) == Some(&'u') {
                                    let hex2: String = self.s[self.i + 2..(self.i + 6).min(self.s.len())].iter().collect();
                                    if let Ok(low) = u16::from_str_radix(&hex2, 16) {
                                        self.i += 6;
                                        out.extend(char::decode_utf16([unit, low]).map(|r| r.unwrap_or('\u{FFFD}')));
                                        continue;
                                    }
                                }
                                out.push(char::from_u32(unit as u32).unwrap_or('\u{FFFD}'));
                            }
                            other => out.push(other),
                        }
                    }
                    c => out.push(c),
                }
            }
        }
    }
    
    impl JJson {
        /// JsonParser.parseString: panics (Java throws JsonSyntaxException) on bad JSON.
        pub fn parse(s: &str) -> JJson {
            let chars: Vec<char> = s.chars().collect();
            let mut r = JJsonReader { s: &chars, i: 0 };
            let v = r.value();
            r.ws();
            if r.i != chars.len() {
                r.fail("more after the value");
            }
            v
        }
        pub fn object() -> JJson {
            JJson::Obj(Vec::new())
        }
        pub fn array() -> JJson {
            JJson::Arr(Vec::new())
        }
        pub fn int(v: i64) -> JJson {
            JJson::Num(v.to_string())
        }
        pub fn float(v: f32) -> JJson {
            JJson::Num(j_java_num(v as f64, true))
        }
        pub fn double(v: f64) -> JJson {
            JJson::Num(j_java_num(v, false))
        }
        pub fn str(v: &str) -> JJson {
            JJson::Str(v.to_string())
        }
        pub fn is_json_object(&self) -> bool {
            matches!(self, JJson::Obj(_))
        }
        pub fn is_json_array(&self) -> bool {
            matches!(self, JJson::Arr(_))
        }
        pub fn is_json_null(&self) -> bool {
            matches!(self, JJson::Null)
        }
        pub fn is_json_primitive(&self) -> bool {
            matches!(self, JJson::Bool(_) | JJson::Num(_) | JJson::Str(_))
        }
        pub fn is_string(&self) -> bool {
            matches!(self, JJson::Str(_))
        }
        pub fn is_number(&self) -> bool {
            matches!(self, JJson::Num(_))
        }
        pub fn is_boolean(&self) -> bool {
            matches!(self, JJson::Bool(_))
        }
        /// getAsJsonObject() / getAsJsonArray(): Java throws IllegalStateException on the wrong kind.
        pub fn as_object(&self) -> JJson {
            assert!(self.is_json_object(), "Not a JSON Object: {self}");
            self.clone()
        }
        pub fn as_array(&self) -> JJson {
            assert!(self.is_json_array(), "Not a JSON Array: {self}");
            self.clone()
        }
        fn members(&self) -> &Vec<(String, JJson)> {
            match self {
                JJson::Obj(m) => m,
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        /// The (name, value) pairs of an object, as entrySet() walks them.
        pub fn entries(&self) -> impl Iterator<Item = (&String, &JJson)> {
            self.members().iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &String> {
            self.members().iter().map(|(k, _)| k)
        }
        pub fn elements(&self) -> impl Iterator<Item = &JJson> {
            match self {
                JJson::Arr(a) => a.iter(),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn has(&self, k: &str) -> bool {
            self.members().iter().any(|(n, _)| n == k)
        }
        /// JsonObject.get: None (null) when absent.
        pub fn get(&self, k: &str) -> Option<JJson> {
            self.members().iter().find(|(n, _)| n == k).map(|(_, v)| v.clone())
        }
        /// JsonObject.getAsJsonObject(name) / getAsJsonArray(name): null when absent, a cast that throws on the wrong kind.
        pub fn member_object(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_object())
        }
        pub fn member_array(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_array())
        }
        /// JsonArray.get(i).
        pub fn at(&self, i: i32) -> JJson {
            match self {
                JJson::Arr(a) => a.get(i as usize).cloned().unwrap_or_else(|| panic!("Index {i} out of bounds for length {}", a.len())),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn size(&self) -> i32 {
            match self {
                JJson::Arr(a) => a.len() as i32,
                JJson::Obj(m) => m.len() as i32,
                _ => panic!("size() of a JSON primitive"),
            }
        }
        pub fn is_empty(&self) -> bool {
            self.size() == 0
        }
        /// The single value of a one-element array, as Gson's JsonArray.getAsX does.
        fn single(&self) -> &JJson {
            match self {
                JJson::Arr(a) if a.len() == 1 => &a[0],
                JJson::Arr(_) => panic!("Array must have size 1"),
                other => other,
            }
        }
        pub fn as_string(&self) -> String {
            match self.single() {
                JJson::Str(s) => s.clone(),
                JJson::Num(n) => n.clone(),
                JJson::Bool(b) => b.to_string(),
                other => panic!("UnsupportedOperationException: {other} as a string"),
            }
        }
        fn number_text(&self) -> String {
            match self.single() {
                JJson::Num(n) | JJson::Str(n) => n.trim().to_string(),
                other => panic!("UnsupportedOperationException: {other} as a number"),
            }
        }
        pub fn as_f64(&self) -> f64 {
            let t = self.number_text();
            t.parse().unwrap_or_else(|_| panic!("NumberFormatException: {t}"))
        }
        pub fn as_float(&self) -> f32 {
            self.as_f64() as f32
        }
        pub fn as_double(&self) -> f64 {
            self.as_f64()
        }
        /// getAsInt / getAsLong: a whole number, else the number cut to one.
        pub fn as_long(&self) -> i64 {
            let t = self.number_text();
            t.parse::<i64>().unwrap_or_else(|_| t.parse::<f64>().map(|f| f as i64).unwrap_or_else(|_| panic!("NumberFormatException: {t}")))
        }
        pub fn as_int(&self) -> i32 {
            self.as_long() as i32
        }
        pub fn as_bool(&self) -> bool {
            match self.single() {
                JJson::Bool(b) => *b,
                // Gson: Boolean.parseBoolean of the text
                other => other.as_string().eq_ignore_ascii_case("true"),
            }
        }
        pub fn add(&mut self, k: &str, v: JJson) {
            match self {
                JJson::Obj(m) => {
                    m.retain(|(n, _)| n != k);
                    m.push((k.to_string(), v));
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn remove(&mut self, k: &str) -> Option<JJson> {
            match self {
                JJson::Obj(m) => {
                    let i = m.iter().position(|(n, _)| n == k)?;
                    Some(m.remove(i).1)
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn push(&mut self, v: JJson) {
            match self {
                JJson::Arr(a) => a.push(v),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        fn write(&self, out: &mut String, pretty: bool, html: bool, depth: usize) {
            let nl = |out: &mut String, d: usize| {
                if pretty {
                    out.push('\n');
                    out.push_str(&"  ".repeat(d));
                }
            };
            match self {
                JJson::Null => out.push_str("null"),
                JJson::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
                JJson::Num(n) => out.push_str(n),
                JJson::Str(s) => j_json_quote(s, html, out),
                JJson::Arr(a) => {
                    out.push('[');
                    for (i, v) in a.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !a.is_empty() {
                        nl(out, depth);
                    }
                    out.push(']');
                }
                JJson::Obj(m) => {
                    out.push('{');
                    for (i, (k, v)) in m.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        j_json_quote(k, html, out);
                        out.push(':');
                        if pretty {
                            out.push(' ');
                        }
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !m.is_empty() {
                        nl(out, depth);
                    }
                    out.push('}');
                }
            }
        }
    }
    
    fn j_json_quote(s: &str, html: bool, out: &mut String) {
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '\u{8}' => out.push_str("\\b"),
                '\u{c}' => out.push_str("\\f"),
                '<' | '>' | '&' | '=' | '\'' if html => out.push_str(&format!("\\u{:04x}", c as u32)),
                '\u{2028}' | '\u{2029}' => out.push_str(&format!("\\u{:04x}", c as u32)),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
    }
    
    /// JsonElement.toString: compact, without Gson's HTML escaping.
    impl std::fmt::Display for JJson {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut out = String::new();
            self.write(&mut out, false, false, 0);
            f.write_str(&out)
        }
    }
    
    /// A Gson (from `new Gson()` or a GsonBuilder): how it writes.
    #[derive(Clone, Copy, Debug)]
    pub struct JGson {
        pub pretty: bool,
        pub html: bool,
        pub nulls: bool,
    }
    
    impl Default for JGson {
        fn default() -> Self {
            JGson { pretty: false, html: true, nulls: false }
        }
    }
    
    impl JGson {
        /// Gson.toJson.
        pub fn to_json(&self, v: &JJson) -> String {
            let mut out = String::new();
            self.strip(v).write(&mut out, self.pretty, self.html, 0);
            out
        }
        /// Without serializeNulls, Gson leaves out null fields of objects.
        fn strip(&self, v: &JJson) -> JJson {
            match v {
                JJson::Obj(m) => JJson::Obj(m.iter().filter(|(_, x)| self.nulls || !x.is_json_null()).map(|(k, x)| (k.clone(), self.strip(x))).collect()),
                JJson::Arr(a) => JJson::Arr(a.iter().map(|x| self.strip(x)).collect()),
                other => other.clone(),
            }
        }
    }
    
    #[cfg(test)]
    mod jjson_tests {
        use super::*;
    
        #[test]
        fn read_and_write() {
            let j = JJson::parse("{\"a\": 1, \"b\": [1.5, \"x\", true, null], \"c\": {\"d\": -2e3}}");
            assert!(j.has("a") && !j.has("z"));
            assert_eq!(j.get("a").unwrap().as_int(), 1);
            assert_eq!(j.member_array("b").unwrap().at(0).as_float(), 1.5);
            assert_eq!(j.member_array("b").unwrap().size(), 4);
            assert_eq!(j.member_object("c").unwrap().get("d").unwrap().as_double(), -2000.0);
            assert_eq!(j.get("z"), None);
            assert_eq!(j.to_string(), "{\"a\":1,\"b\":[1.5,\"x\",true,null],\"c\":{\"d\":-2e3}}");
            let g = JGson { pretty: true, ..JGson::default() };
            assert_eq!(g.to_json(&JJson::parse("{\"a\":[1,2],\"b\":{},\"n\":null,\"s\":\"<x>\"}")),
                       "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": {},\n  \"s\": \"\\u003cx\\u003e\"\n}");
        }
    
        #[test]
        fn java_numbers() {
            assert_eq!(j_java_num(1.0, true), "1.0");
            assert_eq!(j_java_num(0.1, true), "0.1");
            assert_eq!(j_java_num(1.0e-4, false), "1.0E-4");
            assert_eq!(j_java_num(12345678.0, false), "1.2345678E7");
            assert_eq!(j_java_num(-0.75, false), "-0.75");
        }
    }

    
    // ---- java.util.Properties: `key=value` text files ----
    
    #[derive(Clone, Debug, Default)]
    pub struct JProps(pub std::collections::BTreeMap<String, String>);
    
    fn j_props_unescape(s: &str) -> String {
        let mut out = String::new();
        let mut it = s.chars().peekable();
        while let Some(c) = it.next() {
            if c != '\\' {
                out.push(c);
                continue;
            }
            match it.next() {
                Some('t') => out.push('\t'),
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('f') => out.push('\u{c}'),
                Some('u') => {
                    let hex: String = (0..4).filter_map(|_| it.next()).collect();
                    out.push(u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32).unwrap_or('\u{FFFD}'));
                }
                Some(o) => out.push(o),
                None => {}
            }
        }
        out
    }
    
    fn j_props_escape(s: &str, key: bool) -> String {
        let mut out = String::new();
        for (i, c) in s.chars().enumerate() {
            match c {
                '\\' => out.push_str("\\\\"),
                '\t' => out.push_str("\\t"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\u{c}' => out.push_str("\\f"),
                '=' | ':' | '#' | '!' => {
                    out.push('\\');
                    out.push(c);
                }
                ' ' if key || i == 0 => out.push_str("\\ "),
                c if (c as u32) < 0x20 || (c as u32) > 0x7e => out.push_str(&format!("\\u{:04X}", c as u32)),
                c => out.push(c),
            }
        }
        out
    }
    
    impl JProps {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn get_property(&self, k: &str) -> Option<String> {
            self.0.get(k).cloned()
        }
        pub fn get_property_or(&self, k: &str, default: &str) -> String {
            self.0.get(k).cloned().unwrap_or_else(|| default.to_string())
        }
        pub fn set_property(&mut self, k: &str, v: &str) -> Option<String> {
            self.0.insert(k.to_string(), v.to_string())
        }
        pub fn contains_key(&self, k: &str) -> bool {
            self.0.contains_key(k)
        }
        pub fn remove(&mut self, k: &str) -> Option<String> {
            self.0.remove(k)
        }
        pub fn names(&self) -> JSet<String> {
            JSet::from_iter(self.0.keys().cloned())
        }
        /// Properties.load: `key=value`, `key: value` or `key value`, `#`/`!` comments, `\` continuations.
        pub fn load(&mut self, text: &str) {
            let mut logical = String::new();
            for raw in text.lines() {
                let line = raw.trim_start();
                if logical.is_empty() && (line.is_empty() || line.starts_with('#') || line.starts_with('!')) {
                    continue;
                }
                let trailing = line.chars().rev().take_while(|&c| c == '\\').count();
                if trailing % 2 == 1 {
                    logical.push_str(&line[..line.len() - 1]);
                    continue;
                }
                logical.push_str(line);
                let chars: Vec<char> = logical.chars().collect();
                let mut i = 0;
                while i < chars.len() && !(matches!(chars[i], '=' | ':') || chars[i].is_whitespace()) {
                    i += if chars[i] == '\\' { 2 } else { 1 };
                }
                let key: String = chars[..i.min(chars.len())].iter().collect();
                let mut j = i;
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                if j < chars.len() && matches!(chars[j], '=' | ':') {
                    j += 1;
                }
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                let value: String = chars[j.min(chars.len())..].iter().collect();
                self.0.insert(j_props_unescape(&key), j_props_unescape(&value));
                logical.clear();
            }
        }
        /// Properties.store: the comment, then `key=value` lines (Java also writes the date, left out here).
        pub fn store_text(&self, comment: Option<&str>) -> String {
            let mut out = String::new();
            if let Some(c) = comment {
                for l in c.lines() {
                    out.push('#');
                    out.push_str(l);
                    out.push('\n');
                }
            }
            for (k, v) in &self.0 {
                out.push_str(&j_props_escape(k, true));
                out.push('=');
                out.push_str(&j_props_escape(v, false));
                out.push('\n');
            }
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "betternether");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
