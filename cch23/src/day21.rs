use axum::extract::Path;
use keshvar::CountryIterator;
use s2::{cellid::CellID, latlng::LatLng, point::Point};

pub async fn coords(Path(binary): Path<String>) -> String {
    let latlng = s2_to_latlng(&binary);
    latlng_to_dms(latlng)
}
pub async fn country(Path(binary): Path<String>) -> String {
    dbg!(&binary);
    let latlng = s2_to_latlng(&binary);
    latlng_to_country(latlng)
}
fn latlng_to_country(latlng: LatLng) -> String {
    let lat = latlng.lat.deg();
    let lon = latlng.lng.deg();

    dbg!(lat, lon);
    let mut country = None;
    let cs = CountryIterator::new();
    for c in cs{
        let g = c.geo()
;        if g.min_latitude() < lat
            && lat < g.max_latitude()
            && g.min_longitude() < lon
            && lon < g.max_longitude()
        {
            country = Some(c);
            break;
        }
    }
    dbg!(country
        .expect("bounding country")
        .unofficial_name_list()[0]
        .to_string())
}

fn s2_to_latlng(binary: &str) -> LatLng {
    let cellid = u64::from_str_radix(binary, 2).expect("64 bigits");
    let token = format!("{cellid:x}");
    let cellid = CellID::from_token(&token);
    let point: Point = cellid.into();
    point.into()
}

fn latlng_to_dms(latlng: LatLng) -> String {
    let lat = latlng.lat;
    let ns = if lat.deg() < 0.0 { "S" } else { "N" };
    let (lad, lam, las) = float_to_dms(lat.abs().deg());

    let lon = latlng.lng;
    let ew = if lon.deg() < 0.0 { "W" } else { "E" };
    let (lod, lom, los) = float_to_dms(lon.abs().deg());

    format!("{lad}°{lam}'{las:.3}''{ns} {lod}°{lom}'{los:.3}''{ew}")
}

fn float_to_dms(f: f64) -> (f64, f64, f64) {
    let d = f.floor();
    let m = ((f - d) * 60.0).floor();
    let s = ((f - d) * 3600.0) - m * 60.0;
    (d, m, s)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn c83() {
        assert_eq!(
            "83°39'54.324''N 30°37'40.584''W",
            latlng_to_dms(s2_to_latlng(
                "0100111110010011000110011001010101011111000010100011110001011011"
            ))
        )
    }

    #[test]
    fn c18() {
        assert_eq!(
            "18°54'55.944''S 47°31'17.976''E",
            latlng_to_dms(s2_to_latlng(
                "0010000111110000011111100000111010111100000100111101111011000101"
            ))
        )
    }

    #[test]
    fn mada() {
        assert_eq!(
            "Madagascar",
            latlng_to_country(s2_to_latlng(
                "0010000111110000011111100000111010111100000100111101111011000101"
            ))
        )
    }
}
