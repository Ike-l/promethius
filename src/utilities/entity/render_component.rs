use super::{
    raw_render_component::RawRenderComponent, FloatPrecision
};

use cgmath::{
    Array, Deg, InnerSpace, Matrix3, Matrix4, Quaternion, Rotation3, SquareMatrix, Vector3, Vector4
};

use log::warn;

#[derive(Debug)]
pub struct InstanceRenderComponent {
    pub visible: bool,

    pub local_translation: Vector3<FloatPrecision>,
    pub global_translation: Vector3<FloatPrecision>,

    pub local_rotation: Quaternion<FloatPrecision>,
    pub global_rotation: Quaternion<FloatPrecision>,

    pub local_scale: Vector3<FloatPrecision>,
    pub global_scale: Vector3<FloatPrecision>,

    pub tint: Vector4<FloatPrecision>,
    pub highlight: Vector4<FloatPrecision>,
}

impl Default for InstanceRenderComponent {
    fn default() -> Self {
        Self {
            visible: true,

            local_translation: Vector3::from_value(0.0),
            global_translation: Vector3::from_value(0.0),

            local_rotation: Quaternion::from(Matrix3::identity()),
            global_rotation: Quaternion::from(Matrix3::identity()),

            local_scale: Vector3::from_value(1.0),
            global_scale: Vector3::from_value(1.0),

            tint: Vector4::from_value(1.0), 
            highlight: Vector4::from_value(0.0), 
        }
    }
}

impl InstanceRenderComponent {
    pub fn to_raw(&self) -> RawRenderComponent {
        let model = self.model_matrix();

        RawRenderComponent::new(model.into(), self.tint.into(), self.highlight.into())
    }

    pub fn model_matrix(&self) -> Matrix4<FloatPrecision> {
        Matrix4::from_translation(self.global_translation) *
        Matrix4::from(self.global_rotation) *
        Matrix4::from_nonuniform_scale(self.global_scale.x, self.global_scale.y, self.global_scale.z) *
        Matrix4::from_translation(self.local_translation) *
        Matrix4::from(self.local_rotation) *
        Matrix4::from_nonuniform_scale(self.local_scale.x, self.local_scale.y, self.local_scale.z)
    }

    pub fn local_rotate(&mut self, angle: &Deg<FloatPrecision>, axis: &Vector3<FloatPrecision>) {
        let rotation_quat = Quaternion::from_axis_angle(axis.normalize(), *angle);
        self.local_rotation = rotation_quat * self.local_rotation;
    }

    pub fn global_rotate(&mut self, angle: &Deg<FloatPrecision>, axis: &Vector3<FloatPrecision>) {
        let rotation_quat = Quaternion::from_axis_angle(axis.normalize(), *angle);
        self.global_rotation = rotation_quat * self.global_rotation;
    }

	pub fn model_vertex(&self, vertex: Vector4<FloatPrecision>) -> Vector4<FloatPrecision> {
		if vertex.w != 1.0 { warn!("Vertex taken as direction, translations won't apply") }
		self.model_matrix() * vertex
	}
}

#[cfg(test)]
mod tests {
	use cgmath::{Deg, Vector3, Vector4};

	use super::{InstanceRenderComponent, FloatPrecision};

	const EPSILON: f32 = 0.000001;
	fn approx_equal(got: Vector3<FloatPrecision>, expected: Vector3<FloatPrecision>) {
		assert!(
			(got.x - expected.x).abs() < EPSILON &&
			(got.y - expected.y).abs() < EPSILON &&
			(got.z - expected.z).abs() < EPSILON,
			"{}", &format!("Expected: {:?}, Got: {:?}", expected, got)
		)
	}

    #[test]
    fn rotating() {
        let mut r = InstanceRenderComponent::default();
		r.global_rotate(&Deg(90.0), &Vector3::unit_y());
		let v = r.model_matrix() * Vector4::unit_x();
		approx_equal(v.truncate(), -Vector3::unit_z());

		r.local_rotate(&Deg(45.0), &Vector3::unit_y());
		let v = r.model_matrix() * Vector4::unit_x();
		approx_equal(v.truncate(), Vector3::new(-0.7071068, 0.0, -0.7071068));
	}

	#[test]
	fn local_rotating() {
		let mut r = InstanceRenderComponent::default();
		r.local_rotate(&Deg(90.0), &Vector3::unit_y());
		let v = r.model_matrix() * Vector4::unit_x();
		approx_equal(v.truncate(), -Vector3::unit_z());
	}

	#[test]
	fn scaling() {
		let mut r = InstanceRenderComponent::default();
		r.global_scale = Vector3::new(2.0, 2.0, 2.0);
		let v = r.model_matrix() * Vector4::unit_x();
		approx_equal(v.truncate(), Vector3::new(2.0, 0.0, 0.0));
		r.local_scale *= 2.0;
		let v = r.model_matrix() * Vector4::new(1.0, 1.0, 1.0, 0.0);
		approx_equal(v.truncate(), Vector3::new(4.0, 4.0, 4.0));
	}

	#[test]
	fn translating() {
		let mut r = InstanceRenderComponent::default();
		r.global_translation = Vector3::unit_x();
		// w = 1.0 allows translations
		let v = r.model_matrix() * Vector4::new(1.0, 0.0, 0.0, 1.0);
		approx_equal(v.truncate(), Vector3::new(2.0, 0.0, 0.0));
		r.local_translation = Vector3::unit_y();
		let v = r.model_matrix() * Vector4::new(0.0, 0.0, 0.0, 1.0);
		approx_equal(v.truncate(), Vector3::new(1.0, 1.0, 0.0));
	}

	#[test]
	fn all() {
		let mut r = InstanceRenderComponent::default();
		r.local_scale = Vector3::new(2.0, 0.5, 1.0);
		r.local_rotate(&Deg(90.0), &Vector3::unit_z());
		r.local_translation = Vector3::new(3.0, 2.0, 0.0);
		r.global_scale = Vector3::new(1.0, 1.0, 3.0);
		r.global_rotate(&Deg(45.0), &Vector3::unit_x());
		r.global_translation = Vector3::new(0.0, 0.0, 1.0);

		// w = 0.0 -> direction -> translation doesn't apply
		// w = 1.0 -> position -> translation doesn't apply
		let v = r.model_vertex(Vector4::new(1.0, 0.0, 0.0, 1.0));
		approx_equal(v.truncate(), Vector3::new(3.0, 2.8284268, 3.8284273));
	}
}

