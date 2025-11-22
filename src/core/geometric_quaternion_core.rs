use crate::core::types::Quaternion;
use std::f64::consts::PI;

impl Quaternion {
    /// Create a new quaternion
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self { w, x, y, z }
    }

    /// Create a quaternion from axis-angle representation
    pub fn from_axis_angle(axis: [f64; 3], angle_rad: f64) -> Self {
        let half_angle = angle_rad / 2.0;
        let sin_half = half_angle.sin();
        let [x, y, z] = axis;
        let norm = (x * x + y * y + z * z).sqrt();

        if norm < 1e-10 {
            return Self::identity();
        }

        let s = sin_half / norm;
        Self {
            w: half_angle.cos(),
            x: x * s,
            y: y * s,
            z: z * s,
        }
    }

    /// Get the identity quaternion
    pub fn identity() -> Self {
        Self {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Quaternion multiplication (Hamilton product)
    pub fn multiply(&self, other: &Self) -> Self {
        Self {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
        }
    }

    /// Quaternion conjugate
    pub fn conjugate(&self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    /// Quaternion norm (length)
    pub fn norm(&self) -> f64 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalize the quaternion
    pub fn normalize(&self) -> Self {
        let n = self.norm();
        if n < 1e-10 {
            return Self::identity();
        }
        Self {
            w: self.w / n,
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }

    /// Rotate a 3D vector using this quaternion
    pub fn rotate_vector(&self, v: [f64; 3]) -> [f64; 3] {
        let q = self.normalize();
        let [x, y, z] = v;

        // Convert vector to pure quaternion
        let p = Quaternion::new(0.0, x, y, z);

        // Rotate: p' = q * p * q^-1
        let q_conj = q.conjugate();
        let p_rotated = q.multiply(&p).multiply(&q_conj);

        [p_rotated.x, p_rotated.y, p_rotated.z]
    }

    /// Convert to Euler angles (roll, pitch, yaw)
    pub fn to_euler(&self) -> (f64, f64, f64) {
        // Roll (x-axis rotation)
        let sinr_cosp = 2.0 * (self.w * self.x + self.y * self.z);
        let cosr_cosp = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);
        let roll = sinr_cosp.atan2(cosr_cosp);

        // Pitch (y-axis rotation)
        let sinp = 2.0 * (self.w * self.y - self.z * self.x);
        let pitch = if sinp.abs() >= 1.0 {
            (PI / 2.0).copysign(sinp) // Use 90 degrees if out of range
        } else {
            sinp.asin()
        };

        // Yaw (z-axis rotation)
        let siny_cosp = 2.0 * (self.w * self.z + self.x * self.y);
        let cosy_cosp = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);
        let yaw = siny_cosp.atan2(cosy_cosp);

        (roll, pitch, yaw)
    }

    /// Spherical linear interpolation between two quaternions
    pub fn slerp(&self, other: &Self, t: f64) -> Self {
        let mut dot = self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z;

        // If the dot product is negative, the quaternions have opposite handedness
        // and slerp won't take the shorter path. Fix by reversing one quaternion.
        let mut other = *other;
        if dot < 0.0 {
            other = Quaternion::new(-other.w, -other.x, -other.y, -other.z);
            dot = -dot;
        }

        const DOT_THRESHOLD: f64 = 0.9995;
        if dot > DOT_THRESHOLD {
            // If the inputs are too close, linearly interpolate and normalize
            let result = Quaternion {
                w: self.w + t * (other.w - self.w),
                x: self.x + t * (other.x - self.x),
                y: self.y + t * (other.y - self.y),
                z: self.z + t * (other.z - self.z),
            };
            return result.normalize();
        }

        // Since dot is in range [0, DOT_THRESHOLD], acos is safe
        let theta_0 = dot.acos();
        let theta = theta_0 * t;
        let sin_theta = theta.sin();
        let sin_theta_0 = theta_0.sin();

        let s1 = (theta_0 - theta).cos() - dot * sin_theta / sin_theta_0;
        let s2 = sin_theta / sin_theta_0;

        Quaternion {
            w: self.w * s1 + other.w * s2,
            x: self.x * s1 + other.x * s2,
            y: self.y * s1 + other.y * s2,
            z: self.z * s1 + other.z * s2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn test_quaternion_creation() {
        let q = Quaternion::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(q.w, 1.0);
        assert_eq!(q.x, 0.0);
        assert_eq!(q.y, 0.0);
        assert_eq!(q.z, 0.0);
    }

    #[test]
    fn test_quaternion_multiplication() {
        let q1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let q2 = Quaternion::new(5.0, 6.0, 7.0, 8.0);
        let result = q1.multiply(&q2);

        // Expected result from Hamilton product
        assert_eq!(result.w, -60.0);
        assert_eq!(result.x, 12.0);
        assert_eq!(result.y, 30.0);
        assert_eq!(result.z, 24.0);
    }

    #[test]
    fn test_quaternion_rotation() {
        // 90-degree rotation around Z-axis
        let angle = FRAC_PI_2; // 90 degrees in radians
        let axis = [0.0, 0.0, 1.0]; // Z-axis
        let q = Quaternion::from_axis_angle(axis, angle);

        // Rotate vector [1, 0, 0] 90 degrees around Z-axis
        let v = [1.0, 0.0, 0.0];
        let rotated = q.rotate_vector(v);

        // Should rotate to [0, 1, 0]
        assert_relative_eq!(rotated[0], 0.0, epsilon = 1e-10);
        assert_relative_eq!(rotated[1], 1.0, epsilon = 1e-10);
        assert_relative_eq!(rotated[2], 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_quaternion_slerp() {
        // Identity quaternion
        let q1 = Quaternion::identity();
        // 180-degree rotation around X-axis
        let q2 = Quaternion::from_axis_angle([1.0, 0.0, 0.0], std::f64::consts::PI);

        // Halfway interpolation should give 90-degree rotation
        let q_mid = q1.slerp(&q2, 0.5);
        let expected = Quaternion::from_axis_angle([1.0, 0.0, 0.0], std::f64::consts::FRAC_PI_2);

        assert_relative_eq!(q_mid.w, expected.w, epsilon = 1e-10);
        assert_relative_eq!(q_mid.x, expected.x, epsilon = 1e-10);
        assert_relative_eq!(q_mid.y, expected.y, epsilon = 1e-10);
        assert_relative_eq!(q_mid.z, expected.z, epsilon = 1e-10);
    }
}
