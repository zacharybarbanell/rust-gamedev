pub struct ShaderPair {
    program_id: u32
}

impl ShaderPair {
    pub fn new(vertex: &[u8], fragment: &[u8]) -> Self {
        let vertex_pointer = vertex.as_ptr() as *const i8;
        let vertex_len = vertex.len() as i32;
        
        let vertex_id = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        unsafe {
            gl::ShaderSource(vertex_id, 1, &vertex_pointer, &vertex_len);
            gl::CompileShader(vertex_id);

            #[cfg(debug_assertions)]
            {
                let mut success = 0;
                gl::GetShaderiv(vertex_id, gl::COMPILE_STATUS, &mut success);
                assert!(success != 0);
            }
        }

        let fragment_pointer = fragment.as_ptr() as *const i8;
        let fragment_len = fragment.len() as i32;
        
        let fragment_id = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        unsafe {
            gl::ShaderSource(fragment_id, 1, &fragment_pointer, &fragment_len);
            gl::CompileShader(fragment_id);

            #[cfg(debug_assertions)]
            {
                let mut success = 0;
                gl::GetShaderiv(fragment_id, gl::COMPILE_STATUS, &mut success);
                assert!(success != 0);
            }
        }
        
        let  program_id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program_id, vertex_id);
            gl::AttachShader(program_id, fragment_id);
            gl::LinkProgram(program_id);
            gl::DeleteShader(vertex_id);
            gl::DeleteShader(fragment_id);
    
            #[cfg(debug_assertions)]
            {
                let mut success = 0;
                gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
                assert!(success != 0);
            }
        }
        
        return ShaderPair { program_id };
    }

    pub unsafe fn use_shader(&self) {
        gl::UseProgram(self.program_id);
    }
}

#[macro_export]
macro_rules! shader_pair {
    ($path:expr) => {
        shader_pair!(concat!($path, ".vs"), concat!($path, ".fs"))
    };
    ($vertex_path:expr, $fragment_path:expr) => {
        ShaderPair::new(include_bytes!($vertex_path), include_bytes!($fragment_path))
    };
}