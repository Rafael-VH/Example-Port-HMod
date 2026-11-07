// JeremySeq's Damage Indicator, ported to Horizon by horizon-port from jdamageindicators-fabric-26.3-2.3.0.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_jeremyseqsdamageindicators). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "jeremyseqsdamageindicators";
pub const NAME: &str = "JeremySeq's Damage Indicator";
pub const VERSION: &str = "2.3.0";
pub const DESCRIPTION: &str = "Adds a directional damage indicator.";

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

    /// `PlatformMethods` (architectury_inject_jeremyseqsdamageindicators_common_5caaefe61f3049e5878994c40a937c70_b58446b0181d852d654ff8c157759ad48016b6fe8fa8b22304414a21f0766539jdamageindicatorscommon263230devjar/PlatformMethods.java)
    pub mod platform_methods {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PlatformMethods {
        }
        impl PlatformMethods {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn get_current_target() -> String {
            return "fabric".to_string();
        }
    }

    /// `DamageIndicatorsConfig` (dev/jeremyseq/config/DamageIndicatorsConfig.java)
    pub mod damage_indicators_config {
        use super::*;
        pub static CONFIG_PATH: LazyLock<std::path::PathBuf> = LazyLock::new(|| std::path::PathBuf::from("config").join("jeremyseqsdamageindicators.toml"));
        pub static INSTANCE: LazyLock<JCell<damage_indicators_config::DamageIndicatorsConfig>> = LazyLock::new(|| JCell::new(damage_indicators_config::DamageIndicatorsConfig::new()));
        pub static LAST_MODIFIED_TIME: JCell<i64> = JCell::new(0i64);
        #[derive(Clone, Debug)]
        pub struct DamageIndicatorsConfig {
            pub enable_directional_indicator: bool,
            pub directional_indicator_texture: String,
            pub directional_indicator_distance: i32,
            pub directional_indicator_color: String,
            pub directional_indicator_alpha: i32,
            pub enable_fade_out: bool,
            pub indicate_time: i32,
            pub enable_hitmarker: bool,
            pub hitmarker_duration: i32,
            pub hitmarker_size: i32,
            pub hitmarker_thickness: f32,
            pub hitmarker_gap: i32,
            pub hitmarker_color: String,
            pub hitmarker_crit_color: String,
            pub hitmarker_kill_color: String,
            pub enable_damage_text: bool,
            pub enable_health_bars: bool,
            pub only_show_target_health_bar: bool,
            pub enable_blood_overlay: bool,
        }
        impl DamageIndicatorsConfig {
            pub fn new() -> Self {
                let mut s = Self { enable_directional_indicator: true, directional_indicator_texture: "0".to_string(), directional_indicator_distance: 0, directional_indicator_color: "#CE0025".to_string(), directional_indicator_alpha: 255, enable_fade_out: true, indicate_time: 25, enable_hitmarker: true, hitmarker_duration: 5, hitmarker_size: 10, hitmarker_thickness: 1.75f32, hitmarker_gap: 3, hitmarker_color: "#FFFFFF".to_string(), hitmarker_crit_color: "#FFFF00".to_string(), hitmarker_kill_color: "#FF0000".to_string(), enable_damage_text: true, enable_health_bars: false, only_show_target_health_bar: true, enable_blood_overlay: false };
                s
            }
            // DamageIndicatorsConfig.getDirectionalIndicatorColor(0): not translated, it uses `Color` (java.awt).
            // DamageIndicatorsConfig.getHitmarkerColor(0): not translated, it uses `Color` (java.awt).
            // DamageIndicatorsConfig.getHitmarkerCritColor(0): not translated, it uses `Color` (java.awt).
            // DamageIndicatorsConfig.getHitmarkerKillColor(0): not translated, it uses `Color` (java.awt).
            /// Reads each option kept with Horizon's `native_setting` (one key per field); missing or
            /// unreadable ones keep their value.
            pub fn load_settings(&mut self, get: impl Fn(&str) -> Option<String>) {
                if let Some(v) = get("enableDirectionalIndicator").and_then(|v| v.trim().parse::<bool>().ok()) {
                    self.enable_directional_indicator = v;
                }
                if let Some(v) = get("directionalIndicatorTexture").and_then(|v| v.trim().parse::<String>().ok()) {
                    self.directional_indicator_texture = v;
                }
                if let Some(v) = get("directionalIndicatorDistance").and_then(|v| v.trim().parse::<i32>().ok()) {
                    self.directional_indicator_distance = v;
                }
                if let Some(v) = get("directionalIndicatorColor").and_then(|v| v.trim().parse::<String>().ok()) {
                    self.directional_indicator_color = v;
                }
                if let Some(v) = get("directionalIndicatorAlpha").and_then(|v| v.trim().parse::<i32>().ok()) {
                    self.directional_indicator_alpha = v;
                }
                if let Some(v) = get("enableFadeOut").and_then(|v| v.trim().parse::<bool>().ok()) {
                    self.enable_fade_out = v;
                }
                if let Some(v) = get("indicateTime").and_then(|v| v.trim().parse::<i32>().ok()) {
                    self.indicate_time = v;
                }
                if let Some(v) = get("enableHitmarker").and_then(|v| v.trim().parse::<bool>().ok()) {
                    self.enable_hitmarker = v;
                }
                if let Some(v) = get("hitmarkerDuration").and_then(|v| v.trim().parse::<i32>().ok()) {
                    self.hitmarker_duration = v;
                }
                if let Some(v) = get("hitmarkerSize").and_then(|v| v.trim().parse::<i32>().ok()) {
                    self.hitmarker_size = v;
                }
                if let Some(v) = get("hitmarkerThickness").and_then(|v| v.trim().parse::<f32>().ok()) {
                    self.hitmarker_thickness = v;
                }
                if let Some(v) = get("hitmarkerGap").and_then(|v| v.trim().parse::<i32>().ok()) {
                    self.hitmarker_gap = v;
                }
                if let Some(v) = get("hitmarkerColor").and_then(|v| v.trim().parse::<String>().ok()) {
                    self.hitmarker_color = v;
                }
                if let Some(v) = get("hitmarkerCritColor").and_then(|v| v.trim().parse::<String>().ok()) {
                    self.hitmarker_crit_color = v;
                }
                if let Some(v) = get("hitmarkerKillColor").and_then(|v| v.trim().parse::<String>().ok()) {
                    self.hitmarker_kill_color = v;
                }
                if let Some(v) = get("enableDamageText").and_then(|v| v.trim().parse::<bool>().ok()) {
                    self.enable_damage_text = v;
                }
                if let Some(v) = get("enableHealthBars").and_then(|v| v.trim().parse::<bool>().ok()) {
                    self.enable_health_bars = v;
                }
                if let Some(v) = get("onlyShowTargetHealthBar").and_then(|v| v.trim().parse::<bool>().ok()) {
                    self.only_show_target_health_bar = v;
                }
                if let Some(v) = get("enableBloodOverlay").and_then(|v| v.trim().parse::<bool>().ok()) {
                    self.enable_blood_overlay = v;
                }
            }
            /// Keeps every option with Horizon's `native_set_setting`.
            pub fn save_settings(&self, mut set: impl FnMut(&str, &str)) {
                set("enableDirectionalIndicator", &self.enable_directional_indicator.to_string());
                set("directionalIndicatorTexture", &self.directional_indicator_texture.to_string());
                set("directionalIndicatorDistance", &self.directional_indicator_distance.to_string());
                set("directionalIndicatorColor", &self.directional_indicator_color.to_string());
                set("directionalIndicatorAlpha", &self.directional_indicator_alpha.to_string());
                set("enableFadeOut", &self.enable_fade_out.to_string());
                set("indicateTime", &self.indicate_time.to_string());
                set("enableHitmarker", &self.enable_hitmarker.to_string());
                set("hitmarkerDuration", &self.hitmarker_duration.to_string());
                set("hitmarkerSize", &self.hitmarker_size.to_string());
                set("hitmarkerThickness", &self.hitmarker_thickness.to_string());
                set("hitmarkerGap", &self.hitmarker_gap.to_string());
                set("hitmarkerColor", &self.hitmarker_color.to_string());
                set("hitmarkerCritColor", &self.hitmarker_crit_color.to_string());
                set("hitmarkerKillColor", &self.hitmarker_kill_color.to_string());
                set("enableDamageText", &self.enable_damage_text.to_string());
                set("enableHealthBars", &self.enable_health_bars.to_string());
                set("onlyShowTargetHealthBar", &self.only_show_target_health_bar.to_string());
                set("enableBloodOverlay", &self.enable_blood_overlay.to_string());
            }
            /// The on/off options, for the settings page: key and label.
            pub const TOGGLES: &[(&str, &str)] = &[
                ("enableDirectionalIndicator", "Enable directional indicator"),
                ("enableFadeOut", "Enable fade out"),
                ("enableHitmarker", "Enable hitmarker"),
                ("enableDamageText", "Enable damage text"),
                ("enableHealthBars", "Enable health bars"),
                ("onlyShowTargetHealthBar", "Only show target health bar"),
                ("enableBloodOverlay", "Enable blood overlay"),
            ];
            /// An on/off option's value.
            pub fn flag(&self, key: &str) -> bool {
                match key {
                    "enableDirectionalIndicator" => self.enable_directional_indicator,
                    "enableFadeOut" => self.enable_fade_out,
                    "enableHitmarker" => self.enable_hitmarker,
                    "enableDamageText" => self.enable_damage_text,
                    "enableHealthBars" => self.enable_health_bars,
                    "onlyShowTargetHealthBar" => self.only_show_target_health_bar,
                    "enableBloodOverlay" => self.enable_blood_overlay,
                    _ => false,
                }
            }
            /// Flips an on/off option.
            pub fn toggle(&mut self, key: &str) {
                match key {
                    "enableDirectionalIndicator" => self.enable_directional_indicator = !self.enable_directional_indicator,
                    "enableFadeOut" => self.enable_fade_out = !self.enable_fade_out,
                    "enableHitmarker" => self.enable_hitmarker = !self.enable_hitmarker,
                    "enableDamageText" => self.enable_damage_text = !self.enable_damage_text,
                    "enableHealthBars" => self.enable_health_bars = !self.enable_health_bars,
                    "onlyShowTargetHealthBar" => self.only_show_target_health_bar = !self.only_show_target_health_bar,
                    "enableBloodOverlay" => self.enable_blood_overlay = !self.enable_blood_overlay,
                    _ => {}
                }
            }
            /// The number options the mod's own screen has sliders for: key, label, min, max, step.
            pub const NUMBERS: &[(&str, &str, f64, f64, f64)] = &[
            ];
            /// A number option's value.
            pub fn number(&self, key: &str) -> f64 {
                match key {
                    _ => 0.0,
                }
            }
            /// Moves a number option `steps` slider steps, kept in its range and on its steps, as the slider would.
            pub fn nudge(&mut self, key: &str, steps: i32) {
                let Some(&(_, _, min, max, step)) = Self::NUMBERS.iter().find(|n| n.0 == key) else {
                    return;
                };
                let v = (((self.number(key) + steps as f64 * step - min) / step).round() * step + min).clamp(min, max);
                match key {
                    _ => {}
                }
            }
            /// How a number option reads on the page: as many decimals as its step has.
            pub fn number_text(&self, key: &str) -> String {
                let step = Self::NUMBERS.iter().find(|n| n.0 == key).map_or(1.0, |n| n.4);
                let decimals = if step >= 1.0 { 0 } else { (-step.log10()).ceil() as usize };
                format!("{:.*}", decimals, self.number(key))
            }
        }
        // DamageIndicatorsConfig.parseHex(3): not translated, it uses `Color` (java.awt).
        // DamageIndicatorsConfig.load(0): not translated, it calls DamageIndicatorsConfig.load, which isn't translated.
        // DamageIndicatorsConfig.load(1): not translated, it uses `CommentedFileConfig` (com.electronwill.nightconfig.core.file).
        // DamageIndicatorsConfig.save(0): not translated, unknown static DamageIndicators.LOGGER.
        // DamageIndicatorsConfig.writeWithComments(1): not translated, it uses `CommentedFileConfig` (com.electronwill.nightconfig.core.file).
        // DamageIndicatorsConfig.reloadIfChanged(0): not translated, it calls `Files` (java.nio.file).getLastModifiedTime.
        static SETTINGS_LOADED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
        /// Loads DamageIndicatorsConfig.INSTANCE from Horizon's settings the first time it's asked.
        pub fn ensure_loaded(get: impl Fn(&str) -> Option<String>) {
            if !SETTINGS_LOADED.swap(true, std::sync::atomic::Ordering::SeqCst) {
                INSTANCE.with(|c| c.load_settings(get));
            }
        }
    }

    /// `CritTracker` (dev/jeremyseq/CritTracker.java)
    pub mod crit_tracker {
        use super::*;
        pub static LAST_ATTACK_WAS_CRIT: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct CritTracker {
        }
        impl CritTracker {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `DamageIndicators` (dev/jeremyseq/DamageIndicators.java)
    pub mod damage_indicators {
        use super::*;
        // The class itself isn't translated: another class has the same name, in a different package. Its static parts are below.
        pub const MOD_ID: &str = "jeremyseqsdamageindicators";
        // LOGGER: the logger. Log calls become comments.
        // DamageIndicators.init(0): not translated, it calls DamageIndicatorsConfig.load, which isn't translated.
    }

    /// `DamageTextHandler` (dev/jeremyseq/damage_text/DamageTextHandler.java)
    pub mod damage_text_handler {
        use super::*;
        pub const SELF_DAMAGE_TEXT: bool = false;
        // damageTexts: not translated, it uses DamageText, which isn't translated (it uses Minecraft's `DamageSource`).
        #[derive(Clone, Debug)]
        pub struct DamageTextHandler {
        }
        impl DamageTextHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DamageTextHandler.getDamageTexts(0): not translated, it uses DamageText, which isn't translated (it uses Minecraft's `DamageSource`).
    }

    /// `DamageTextHandler.DamageText` (dev/jeremyseq/damage_text/DamageTextHandler.java)
    pub mod damage_text_handler_damage_text {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `DamageSource`. Its static parts are below.
        // DamageTextHandler.DamageText.DamageText(5): not translated, its class isn't translated (it uses Minecraft's `DamageSource`).
        // DamageTextHandler.DamageText.getX(0): not translated, its class isn't translated (it uses Minecraft's `DamageSource`).
        // DamageTextHandler.DamageText.getY(0): not translated, its class isn't translated (it uses Minecraft's `DamageSource`).
        // DamageTextHandler.DamageText.getZ(0): not translated, its class isn't translated (it uses Minecraft's `DamageSource`).
        // DamageTextHandler.DamageText.getDamage(0): not translated, its class isn't translated (it uses Minecraft's `DamageSource`).
        // DamageTextHandler.DamageText.getSource(0): not translated, its class isn't translated (it uses Minecraft's `DamageSource`).
        // DamageTextHandler.DamageText.getTimestamp(0): not translated, its class isn't translated (it uses Minecraft's `DamageSource`).
        // DamageTextHandler.DamageText.getDuration(0): not translated, its class isn't translated (it uses Minecraft's `DamageSource`).
    }

    /// `DamageTextRenderer` (dev/jeremyseq/damage_text/DamageTextRenderer.java)
    pub mod damage_text_renderer {
        use super::*;
        pub const FULL_BRIGHT: i32 = 15728880;
        #[derive(Clone, Debug)]
        pub struct DamageTextRenderer {
        }
        impl DamageTextRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DamageTextRenderer.render(2): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // DamageTextRenderer.hasLineOfSight(5): not translated, it uses Minecraft's `LivingEntity`.
        // DamageTextRenderer.getDamageColor(2): not translated, it uses `Color` (java.awt).
    }

    /// `HealthBarRenderer` (dev/jeremyseq/damage_text/HealthBarRenderer.java)
    pub mod health_bar_renderer {
        use super::*;
        pub const FULL_BRIGHT: i32 = 15728880;
        #[derive(Clone, Debug)]
        pub struct HealthBarRenderer {
        }
        impl HealthBarRenderer {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // HealthBarRenderer.render(2): not translated, it uses Minecraft's `SubmitNodeCollector`.
        // HealthBarRenderer.hasLineOfSight(6): not translated, it uses Minecraft's `LivingEntity`.
        // HealthBarRenderer.renderHealthBar(3): not translated, it uses Minecraft's `SubmitNodeCollector`.
    }

    /// `ClientTickHandler` (dev/jeremyseq/events/ClientTickHandler.java)
    pub mod client_tick_handler {
        use super::*;
        // LISTENERS: not translated, a function kept in a variable or field (only parameters can be).
        #[derive(Clone, Debug)]
        pub struct ClientTickHandler {
        }
        impl ClientTickHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClientTickHandler.register(1): not translated, it uses ClientTickHandler.LISTENERS, which isn't translated (a function kept in a variable or field (only parameters can be)).
        // ClientTickHandler.tick(0): not translated, it uses ClientTickHandler.LISTENERS, which isn't translated (a function kept in a variable or field (only parameters can be)).
    }

    /// `ConfigReloader` (dev/jeremyseq/events/ConfigReloader.java)
    pub mod config_reloader {
        use super::*;
        pub static RELOAD_CHECK_COUNTER: JCell<i32> = JCell::new(0);
        #[derive(Clone, Debug)]
        pub struct ConfigReloader {
        }
        impl ConfigReloader {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ConfigReloader.register(0): not translated, it calls ClientTickHandler.register, which isn't translated.
        // ConfigReloader.onClientTick(0): not translated, it calls DamageIndicatorsConfig.reloadIfChanged, which isn't translated.
    }

    /// `DamageHandler` (dev/jeremyseq/events/DamageHandler.java)
    pub mod damage_handler {
        use super::*;
        pub static LAST_HEALTH: JCell<f32> = JCell::new(-1.0f32);
        pub static LAST_HEALTH_FOR_OTHER_ENTITIES: LazyLock<JMap<i32, f32>> = LazyLock::new(|| JMap::new());
        #[derive(Clone, Debug)]
        pub struct DamageHandler {
        }
        impl DamageHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DamageHandler.register(0): not translated, it calls ClientTickHandler.register, which isn't translated.
        // DamageHandler.onClientTick(0): not translated, it calls Minecraft's `Minecraft`.getInstance.
    }

    /// `DamageIndicators` (dev/jeremyseq/fabric/DamageIndicators.java)
    pub mod damage_indicators_2 {
        use super::*;
        // The class itself isn't translated: another class has the same name, in a different package. Its static parts are below.
        // DamageIndicators.onInitialize(0): not translated, its class isn't translated (another class has the same name, in a different package).
    }

    /// `DamageIndicatorsClient` (dev/jeremyseq/fabric/DamageIndicatorsClient.java)
    pub mod damage_indicators_client {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        // DamageIndicatorsClient.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
    }

    /// `FabricClientEvents` (dev/jeremyseq/fabric/FabricClientEvents.java)
    pub mod fabric_client_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct FabricClientEvents {
        }
        impl FabricClientEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FabricClientEvents.register(0): not translated, it uses Fabric's `LevelRenderEvents`.COLLECT_SUBMITS.
    }

    /// `FabricClientTickHandler` (dev/jeremyseq/fabric/FabricClientTickHandler.java)
    pub mod fabric_client_tick_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct FabricClientTickHandler {
        }
        impl FabricClientTickHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FabricClientTickHandler.register(0): not translated, it uses Fabric's `ClientTickEvents`.END_CLIENT_TICK.
    }

    /// `DirectionalIndicatorImages` (dev/jeremyseq/overlays/DirectionalIndicatorImages.java)
    pub mod directional_indicator_images {
        use super::*;
        pub const FOLDER: &str = "textures/directional_indicators";
        pub static IMAGES: LazyLock<JCell<JMap<String, JIdentifier>>> = LazyLock::new(|| JCell::new(JMap::new()));
        #[derive(Clone, Debug)]
        pub struct DirectionalIndicatorImages {
        }
        impl DirectionalIndicatorImages {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DirectionalIndicatorImages.reload(1): not translated, it uses Minecraft's `ResourceManager`.
        pub fn get(name: &str) -> JIdentifier {
            return ({ let a_0 = name.to_string(); directional_indicator_images::IMAGES.with(|o| o.get(&a_0).cloned()) }).expect("null");
        }
        pub fn get_all() -> JMap<String, JIdentifier> {
            return directional_indicator_images::IMAGES.clone();
        }
    }

    /// `IndicatorOverlay` (dev/jeremyseq/overlays/IndicatorOverlay.java)
    pub mod indicator_overlay {
        use super::*;
        pub static BLOOD_OVERLAY: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("jeremyseqsdamageindicators", "textures/overlays/blood_overlay.png"));
        pub const SUBPIXEL: i32 = 8;
        // damageSource: not translated, it uses Minecraft's `Vec3`.
        pub static SHOW_DIRECTIONAL: JCell<bool> = JCell::new(false);
        pub static COUNTER: JCell<f32> = JCell::new(0.0f32);
        pub static SHOW_HITMARKER: JCell<bool> = JCell::new(false);
        pub static HITMARKER_COUNTER: JCell<f32> = JCell::new(0.0f32);
        pub static WAS_KILL: JCell<bool> = JCell::new(false);
        pub static WAS_CRIT: JCell<bool> = JCell::new(false);
        pub static SHOW_BLOOD: JCell<bool> = JCell::new(false);
        pub static BLOOD_ALPHA: JCell<f32> = JCell::new(0.0f32);
        pub const BLOOD_FADE_SPEED: f32 = 0.02f32;
        #[derive(Clone, Debug)]
        pub struct IndicatorOverlay {
        }
        impl IndicatorOverlay {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // IndicatorOverlay.triggerOverlay(1): not translated, it uses Minecraft's `Vec3`.
        pub fn trigger_hitmarker(was_kill: bool, was_crit: bool) {
            indicator_overlay::SHOW_HITMARKER.set(true);
            indicator_overlay::HITMARKER_COUNTER.set(0.0f32);
            indicator_overlay::WAS_KILL.set(was_kill);
            indicator_overlay::WAS_CRIT.set(was_crit);
        }
        // IndicatorOverlay.render(4): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // IndicatorOverlay.calculateFinalAngle(3): not translated, it uses Minecraft's `Vec3`.
        // IndicatorOverlay.drawX(7): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // IndicatorOverlay.drawRotatedBar(8): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // IndicatorOverlay.fillFloat(6): not translated, it uses Minecraft's `GuiGraphicsExtractor`.
        // IndicatorOverlay.withAlpha(2): not translated, it uses `Color` (java.awt).
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "jeremyseqsdamageindicators");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }

    #[test]
    fn settings_round_trip() {
        let key = java::damage_indicators_config::DamageIndicatorsConfig::TOGGLES[0].0;
        let mut c = java::damage_indicators_config::DamageIndicatorsConfig::new();
        c.toggle(key);
        let mut kept = std::collections::HashMap::new();
        c.save_settings(|k, v| {
            kept.insert(k.to_string(), v.to_string());
        });
        let mut d = java::damage_indicators_config::DamageIndicatorsConfig::new();
        d.load_settings(|k| kept.get(k).cloned());
        assert_eq!(d.flag(key), c.flag(key));
        assert_ne!(d.flag(key), java::damage_indicators_config::DamageIndicatorsConfig::new().flag(key));
    }
}
