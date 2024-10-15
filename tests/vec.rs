#[cfg(test)]
mod tests {
    use wasm::math::vec::Vec3;

    const X: f64 = 1.0;
    const Y: f64 = 1.0;
    const Z: f64 = 1.0;

    #[test]
    fn test_vec3_new() {
        let vec = Vec3::new(X, Y, Z);
        assert_eq!(vec.x, X);
        assert_eq!(vec.y, Y);
        assert_eq!(vec.z, Z);
    }

    #[test]
    fn test_vec3_neg() {
        let vec = Vec3::new(X, Y, Z);
        let neg = -vec;
        assert_eq!(neg.x, -X);
        assert_eq!(neg.y, -Y);
        assert_eq!(neg.z, -Z);
    }

    #[test]
    fn test_vec3_mul_f64() {
        let vec = Vec3::new(X, Y, Z);
        let fact = 2.0;
        let mul = vec * fact;
        assert_eq!(mul.x, X * fact);
        assert_eq!(mul.y, Y * fact);
        assert_eq!(mul.z, Z * fact);
    }

    #[test]
    fn test_vec3_sub() {
        let vec = Vec3::new(X, Y, Z);
        let other = Vec3::new(X + 2.0, Y - 1.0, Z);
        let sub = vec - other;
        assert_eq!(sub.x, -2.0);
        assert_eq!(sub.y, 1.0);
        assert_eq!(sub.z, 0.0);
    }

    #[test]
    fn test_vec3_norm() {
        let mut vec = Vec3::new(X, Y, Z);
        assert_eq!(vec.norm(), 3.0_f64.sqrt());
        vec = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(vec.norm(), 1.0_f64);
        vec = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(vec.norm(), 1.0_f64);
        vec = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(vec.norm(), 1.0_f64);
        vec = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(vec.norm(), 0.0_f64);
    }

    #[test]
    fn test_vec3_normalize() {
        let vec = Vec3::new(X, Y, Z);
        assert_eq!(vec.normalize().norm(), 1.0_f64.sqrt());
    }
}
