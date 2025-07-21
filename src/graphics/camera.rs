use glam::{Mat3, Mat4, Vec3};

const NEAR_CLIPPING_PLANE: f32 = 0.000001;
const FAR_CLIPPING_PLANE: f32 = 100000.0;

pub fn make_view_mat(cam_pos: &Vec3, cam_facing: &Vec3) -> Mat4 {
    let translation_mat = Mat4::from_translation(*cam_pos * -1f32);

    let cam_z = cam_facing.normalize();
    let cam_y = (Vec3::Y - Vec3::Y.project_onto(cam_z)).normalize();
    let cam_x = cam_y.cross(cam_z);

    let rotation_mat = Mat4::from_mat3(Mat3::from_cols(cam_x, cam_y, cam_z).transpose().inverse());

    return rotation_mat * translation_mat;
}

pub fn make_ortho_proj_mat(height: f32, aspect_ratio: f32) -> Mat4 {
    todo!();
}