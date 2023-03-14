use super::{Ellipsoid, NaifId};
pub struct Mercury;
impl NaifId for Mercury {
    fn id() -> i32 {
        199i32
    }
}
impl Ellipsoid for Mercury {
    fn max_equatorial_radius() -> f64 {
        2440.53f64
    }
    fn min_equatorial_radius() -> f64 {
        2440.53f64
    }
    fn polar_radius() -> f64 {
        2438.26f64
    }
    fn mean_radius() -> f64 {
        2439.7733333333335f64
    }
}
pub struct Venus;
impl NaifId for Venus {
    fn id() -> i32 {
        299i32
    }
}
impl Ellipsoid for Venus {
    fn max_equatorial_radius() -> f64 {
        6051.8f64
    }
    fn min_equatorial_radius() -> f64 {
        6051.8f64
    }
    fn polar_radius() -> f64 {
        6051.8f64
    }
    fn mean_radius() -> f64 {
        6051.8f64
    }
}
pub struct Earth;
impl NaifId for Earth {
    fn id() -> i32 {
        399i32
    }
}
impl Ellipsoid for Earth {
    fn max_equatorial_radius() -> f64 {
        6378.1366f64
    }
    fn min_equatorial_radius() -> f64 {
        6378.1366f64
    }
    fn polar_radius() -> f64 {
        6356.7519f64
    }
    fn mean_radius() -> f64 {
        6371.008366666666f64
    }
}
pub struct Mars;
impl NaifId for Mars {
    fn id() -> i32 {
        499i32
    }
}
impl Ellipsoid for Mars {
    fn max_equatorial_radius() -> f64 {
        3396.19f64
    }
    fn min_equatorial_radius() -> f64 {
        3396.19f64
    }
    fn polar_radius() -> f64 {
        3376.2f64
    }
    fn mean_radius() -> f64 {
        3389.5266666666666f64
    }
}
pub struct Jupiter;
impl NaifId for Jupiter {
    fn id() -> i32 {
        599i32
    }
}
impl Ellipsoid for Jupiter {
    fn max_equatorial_radius() -> f64 {
        71492f64
    }
    fn min_equatorial_radius() -> f64 {
        71492f64
    }
    fn polar_radius() -> f64 {
        66854f64
    }
    fn mean_radius() -> f64 {
        69946f64
    }
}
pub struct Saturn;
impl NaifId for Saturn {
    fn id() -> i32 {
        699i32
    }
}
impl Ellipsoid for Saturn {
    fn max_equatorial_radius() -> f64 {
        60268f64
    }
    fn min_equatorial_radius() -> f64 {
        60268f64
    }
    fn polar_radius() -> f64 {
        54364f64
    }
    fn mean_radius() -> f64 {
        58300f64
    }
}
pub struct Uranus;
impl NaifId for Uranus {
    fn id() -> i32 {
        799i32
    }
}
impl Ellipsoid for Uranus {
    fn max_equatorial_radius() -> f64 {
        25559f64
    }
    fn min_equatorial_radius() -> f64 {
        25559f64
    }
    fn polar_radius() -> f64 {
        24973f64
    }
    fn mean_radius() -> f64 {
        25363.666666666668f64
    }
}
pub struct Neptune;
impl NaifId for Neptune {
    fn id() -> i32 {
        899i32
    }
}
impl Ellipsoid for Neptune {
    fn max_equatorial_radius() -> f64 {
        24764f64
    }
    fn min_equatorial_radius() -> f64 {
        24764f64
    }
    fn polar_radius() -> f64 {
        24341f64
    }
    fn mean_radius() -> f64 {
        24623f64
    }
}
pub struct Pluto;
impl NaifId for Pluto {
    fn id() -> i32 {
        999i32
    }
}
impl Ellipsoid for Pluto {
    fn max_equatorial_radius() -> f64 {
        1188.3f64
    }
    fn min_equatorial_radius() -> f64 {
        1188.3f64
    }
    fn polar_radius() -> f64 {
        1188.3f64
    }
    fn mean_radius() -> f64 {
        1188.3f64
    }
}
