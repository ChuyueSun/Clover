
fn strongest_extension(class_name: &str, extensions: Vec<&str>) -> String {
    let mut max_strength = i32::MIN;
    let mut strongest_ext = "";

    for ext in extensions.iter() {
        let (cap, sm) = ext.chars().fold((0, 0), |(cap, sm), c| {
            if c.is_ascii_uppercase() {
                (cap + 1, sm)
            } else if c.is_ascii_lowercase() {
                (cap, sm + 1)
            } else {
                (cap, sm)
            }
        });
        let strength = cap - sm;
        if strength > max_strength {
            max_strength = strength;
            strongest_ext = ext;
        }
    }

    format!("{}.{}", class_name, strongest_ext)
}

fn main() {
    let class_name = "Slices";
    let extensions = vec!["SErviNGSliCes", "Cheese", "StuFfed"];
    println!("{}", strongest_extension(class_name, extensions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strongest_extension() {
        assert_eq!(
            strongest_extension("Watashi", vec!["tEN", "niNE", "eIGHt8OKe"]),
            "Watashi.eIGHt8OKe"
        );
        assert_eq!(
            strongest_extension("Boku123", vec!["nani", "NazeDa", "YEs.WeCaNe", "32145tggg"]),
            "Boku123.YEs.WeCaNe"
        );
        assert_eq!(
            strongest_extension(
                "__YESIMHERE",
                vec!["t", "eMptY", "(nothing", "zeR00", "NuLl__", "123NoooneB321"]
            ),
            "__YESIMHERE.NuLl__"
        );
        assert_eq!(
            strongest_extension("K", vec!["Ta", "TAR", "t234An", "cosSo"]),
            "K.TAR"
        );
        assert_eq!(
            strongest_extension("__HAHA", vec!["Tab", "123", "781345", "-_-"]),
            "__HAHA.123"
        );
        assert_eq!(
            strongest_extension(
                "YameRore",
                vec!["HhAas", "okIWILL123", "WorkOut", "Fails", "-_-"]
            ),
            "YameRore.okIWILL123"
        );
        assert_eq!(
            strongest_extension("finNNalLLly", vec!["Die", "NowW", "Wow", "WoW"]),
            "finNNalLLly.WoW"
        );
        assert_eq!(strongest_extension("_", vec!["Bb", "91245"]), "_.Bb");
        assert_eq!(strongest_extension("Sp", vec!["671235", "Bb"]), "Sp.671235");
    }

}
