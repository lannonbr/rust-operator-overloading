use std::ops::Sub;

#[derive(Debug)]
struct StatsEntry {
    /// how many modules have "type": "module" in their package.json
    type_module: i32,

    /// how many packages have an "exports" field and a "require" subfield within it
    exports_require: i32,

    /// how many packages have an "exports" field, but no "require" subfield
    exports_no_require: i32,

    timestamp: Option<String>,
}

impl Sub for StatsEntry {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            type_module: self.type_module - rhs.type_module,
            exports_require: self.exports_require - rhs.exports_require,
            exports_no_require: self.exports_no_require - rhs.exports_no_require,
            timestamp: None,
        }
    }
}

fn main() {
    let a = StatsEntry {
        type_module: 20,
        exports_require: 5,
        exports_no_require: 8,
        timestamp: Some("2022-01-04T12:00:00".into()),
    };

    let b = StatsEntry {
        type_module: 30,
        exports_require: 5,
        exports_no_require: 8,
        timestamp: Some("2022-01-14T12:00:00".into()),
    };

    dbg!(b - a);
}
