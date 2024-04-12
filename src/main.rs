use std::{mem::{size_of, size_of_val, MaybeUninit}, ptr::null};

use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder}
};
use raw_gl_context::{GlConfig, GlContext};

const V: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;
void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
"#;

const F: &str = r#"
#version 330 core
out vec4 FragColor;
void main()
{
FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
"#;

struct GlobalData {
    window: Window,
    context: GlContext,
    shader_program: u32,
    VAO: u32
}

fn size_of_element<T>(_: &[T]) -> i32 {
    return size_of::<T>() as i32;
}

fn setup() -> (EventLoop<()>, GlobalData) {

    let event_loop = EventLoop::new().unwrap();
    
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    
    let context = GlContext::create(&window, GlConfig::default()).unwrap();
    
    context.make_current();

    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    let PhysicalSize { width , height } = window.inner_size();

    unsafe { gl::Viewport( 0, 0, width as i32, height as i32); }

    let shader_program = build_shaders(V, F);

    let VAO = make_triangle_vao();

    return (event_loop, GlobalData {
        window,
        context,
        shader_program,
        VAO
    });

}

fn build_shaders(v: &str, f: &str) -> u32 {
    
    let VV = v.to_string().into_bytes();
    let VP = VV.as_ptr() as *const i8;
    let VVLen = VV.len() as i32;

    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    unsafe {
        gl::ShaderSource(vertex_shader, 1, &VP, &VVLen);
        gl::CompileShader(vertex_shader);

        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        assert!(success != 0);
    }

    let FF = f.to_string().into_bytes();
    let FP = FF.as_ptr() as *const i8;
    let FFLen = FF.len() as i32;

    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    unsafe {
        gl::ShaderSource(fragment_shader, 1, &FP, &FFLen);
        gl::CompileShader(fragment_shader);

        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        assert!(success != 0);
    }

    let shader_program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        assert!(success != 0);
    }

    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(vertex_shader);
    };

    return shader_program;
}

fn make_triangle_vao() -> u32 {
    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];
    
    let mut VAO = MaybeUninit::uninit();
    let mut VBO = MaybeUninit::uninit();

    unsafe {
        gl::GenVertexArrays(1, VAO.as_mut_ptr());
        gl::GenBuffers(1, VBO.as_mut_ptr());

        gl::BindVertexArray(VAO.assume_init());

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO.assume_init());
        gl::BufferData(gl::ARRAY_BUFFER, size_of_val(&vertices) as isize, vertices.as_ptr().cast(), gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * size_of_element(&vertices), null());
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::BindVertexArray(0);
    }

    return unsafe { VAO.assume_init() };
}

fn render(global_data: &GlobalData) {
    //global_data.context.make_current();
    
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::UseProgram(global_data.shader_program);
        gl::BindVertexArray(global_data.VAO);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }

    global_data.context.swap_buffers();
    //global_data.context.make_not_current();
}

fn main() {
    let (event_loop, global_data) = setup();

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop.run(move |event, elwt| 
        {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    elwt.exit();
                },
                Event::AboutToWait => {
                    render(&global_data);
                },
                _ => ()
            }
        }
    ).unwrap();
}
