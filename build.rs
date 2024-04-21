fn main() { check_enabled_features(); }

fn check_enabled_features() {
    let vars = std::env::vars();
    let mut has_features = false;

    for (name, _value) in vars {
        if name == "CARGO_FEATURE_DEFAULT" {
            continue;
        }

        if name == "feature_warning" {
            continue;
        }

        if name.starts_with("CARGO_FEATURE_") {
            has_features = true;
            break;
        }
    }

    println!("cargo:rustc-env=HAS_FEATURES={has_features}");
}
