pub fn parse_color(args: &[String]) -> anyhow::Result<Vec<u8>> {
    let mut result = Vec::new();

    for hex in args {
        if hex.len() != 6 {
            anyhow::bail!("Invalid hex color: {}", hex);
        }

        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;

        result.extend_from_slice(&[r, g, b]);
    }

    Ok(result)
}
