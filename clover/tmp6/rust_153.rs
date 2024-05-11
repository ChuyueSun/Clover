
fn strongest_extension(class_name: &str, extensions: Vec<&str>) -> String {
    let calc_strength = |extension: &str| {
        extension.chars().fold(0, |acc, c| {
            if c.is_uppercase() {
                acc + 1
            } else if c.is_lowercase() {
                acc - 1
            } else {
                acc
            }
        })
    };

    let mut strongest: Option<(&str, i32)> = None;

    for extension in extensions.iter() {
        let strength = calc_strength(extension);

        if strongest
            .as_ref()
            .map_or(true, |&(_, s)| strength > s || (strength == s && extension < &strongest.unwrap().0))
        {
            strongest = Some((extension, strength));
        }
    }

    format!("{}.{}", class_name, strongest.map_or("", |(ext, _)| ext))
}

//Example Usage
fn main() {
    let strongest = strongest_extension("Slices", vec!["SErviNGSliCes", "Cheese", "StuFfed"]);
    println!("{}", strongest); // Should display "Slices.SErviNGSliCes"
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
