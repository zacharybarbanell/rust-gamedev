use std::{
    mem::{size_of, size_of_val, MaybeUninit},
    ptr::null,
};

//use glam::{Mat3, Mat4, Vec3};
use gl_from_raw_window_handle::{GlConfig, GlContext};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

use graphics::shaders::ShaderPair;

mod graphics;

struct GlobalData {
    window: Window,
    context: GlContext,
    shader_program: ShaderPair,
    VAO: u32,
}

struct App {
    data: Option<GlobalData>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[cfg(debug_assertions)]
        {
            assert!(self.data.is_none());
        }

        self.data = Some(setup(event_loop));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                render(self.data.as_ref().unwrap());
                self.data.as_ref().unwrap().window.request_redraw();
            }
            _ => (),
        }
    }
}

fn size_of_element<T>(_: &[T]) -> i32 {
    return size_of::<T>() as i32;
}

fn setup(event_loop: &ActiveEventLoop) -> GlobalData {
    let window = event_loop
        .create_window(WindowAttributes::default())
        //    .with_inner_size(PhysicalSize {width: 768, height: 576})
        .unwrap();

    let context = unsafe { GlContext::create(&window, GlConfig::default()).unwrap() };

    unsafe { context.make_current() };

    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);

    let PhysicalSize { width, height } = window.inner_size();

    unsafe {
        gl::Viewport(0, 0, width as i32, height as i32);
    }

    let shader_program = shader_pair!("shaders/basic");
    //let shader_program = ShaderPair::new(V,F);

    let VAO = make_triangle_vao();
    
    println!("{}", VAO);

    return GlobalData {
        window,
        context,
        shader_program,
        VAO,
    };
}

fn make_triangle_vao() -> u32 {
    let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let mut VAO = MaybeUninit::uninit();
    let mut VBO = MaybeUninit::uninit();

    unsafe {
        gl::GenVertexArrays(1, VAO.as_mut_ptr());
        gl::GenBuffers(1, VBO.as_mut_ptr());

        gl::BindVertexArray(VAO.assume_init());

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO.assume_init());
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&vertices) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * size_of_element(&vertices),
            null(),
        );
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

        global_data.shader_program.use_shader();
        gl::BindVertexArray(global_data.VAO);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }

    global_data.context.swap_buffers();
    //global_data.context.make_not_current();
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App { data: None };

    event_loop.run_app(&mut app);
}
