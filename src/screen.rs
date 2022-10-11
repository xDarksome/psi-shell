use crate::process;

fn brightnessctl(diff: Option<i8>) -> Result<u8, String> {
    let diff = match diff {
        Some(v) if v > 0 => format!("set +{v}%"),
        Some(v) => format!("set {}%-", v.abs()),
        None => String::new(),
    };
    let output: String = process::exec(&format!("brightnessctl {diff} -m"))?;

    output
        .split(',')
        .nth(3)
        .ok_or_else(|| format!("{output}: missing 4th column"))?
        .trim_end_matches('%')
        .parse()
        .map_err(|e| format!("{output}: invalid brightness percent: {e}"))
}

pub fn brightness() -> Result<u8, String> { brightnessctl(None) }
pub fn change_brightness(diff: i8) -> Result<u8, String> { brightnessctl(Some(diff)) }
