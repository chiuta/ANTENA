//! ANTENA core: wavelength helpers and the NEC `GW` (straight wire) card.
//!
//! Basis: NEC-2 manual (LLNL, public), free-space wavelength.

use std::fmt;

/// Speed of light in vacuum, m/s.
pub const C_M_S: f64 = 299_792_458.0;

/// Free-space wavelength in metres for a frequency in MHz.
pub fn wavelength_m(freq_mhz: f64) -> f64 {
    C_M_S / (freq_mhz * 1e6)
}

/// Cut length of a half-wave dipole (0.95 end-effect factor), metres.
pub fn half_wave_dipole_m(freq_mhz: f64) -> f64 {
    0.95 * wavelength_m(freq_mhz) / 2.0
}

/// Longest segment length (λ/10 rule of thumb for the method of moments), metres.
pub fn max_segment_length_m(freq_mhz: f64) -> f64 {
    wavelength_m(freq_mhz) / 10.0
}

/// A straight wire from a `GW` card.
#[derive(Debug, Clone, PartialEq)]
pub struct Wire {
    pub tag: u32,
    pub segments: u32,
    pub start: [f64; 3],
    pub end: [f64; 3],
    pub radius: f64,
}

impl Wire {
    /// Wire length in metres.
    pub fn length(&self) -> f64 {
        self.start
            .iter()
            .zip(self.end.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum::<f64>()
            .sqrt()
    }

    /// Length of one segment in metres.
    pub fn segment_length(&self) -> f64 {
        self.length() / f64::from(self.segments.max(1))
    }
}

/// Why a card could not be parsed.
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    NotGw,
    MissingField(usize),
    BadNumber(usize),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::NotGw => write!(f, "not a GW card"),
            ParseError::MissingField(i) => write!(f, "missing field {i}"),
            ParseError::BadNumber(i) => write!(f, "field {i} is not a number"),
        }
    }
}

impl std::error::Error for ParseError {}

/// Parse `GW tag segments x1 y1 z1 x2 y2 z2 radius` (spaces or commas).
pub fn parse_gw(line: &str) -> Result<Wire, ParseError> {
    let f: Vec<&str> = line
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .collect();
    if f.first().map(|s| s.to_ascii_uppercase()) != Some("GW".to_string()) {
        return Err(ParseError::NotGw);
    }
    let get = |i: usize| f.get(i).copied().ok_or(ParseError::MissingField(i));
    let int = |i: usize| get(i)?.parse::<u32>().map_err(|_| ParseError::BadNumber(i));
    let num = |i: usize| get(i)?.parse::<f64>().map_err(|_| ParseError::BadNumber(i));
    Ok(Wire {
        tag: int(1)?,
        segments: int(2)?,
        start: [num(3)?, num(4)?, num(5)?],
        end: [num(6)?, num(7)?, num(8)?],
        radius: num(9)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dipole_20m_band() {
        assert!((half_wave_dipole_m(14.2) - 10.03).abs() < 0.05);
        assert!((wavelength_m(300.0) - 0.999_308).abs() < 1e-5);
        assert!((max_segment_length_m(14.2) - 2.111).abs() < 0.01);
    }

    #[test]
    fn gw_card() {
        let w = parse_gw("GW 1,21, 0,0,-5, 0,0,5, 1.0E-3").unwrap();
        assert_eq!(w.tag, 1);
        assert_eq!(w.segments, 21);
        assert!((w.length() - 10.0).abs() < 1e-12);
        assert!((w.segment_length() - 10.0 / 21.0).abs() < 1e-12);
        assert!((w.radius - 0.001).abs() < 1e-12);
    }

    #[test]
    fn gw_errors() {
        assert_eq!(parse_gw("GE 0"), Err(ParseError::NotGw));
        assert_eq!(parse_gw("GW 1 21"), Err(ParseError::MissingField(3)));
        assert_eq!(
            parse_gw("GW 1 x 0 0 0 0 0 1 0.1"),
            Err(ParseError::BadNumber(2))
        );
    }
}
