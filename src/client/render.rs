use libc::intptr_t;
use crate::client as cl;
use cl::uiImport_t;


#[derive(Debug, Default)]
pub struct Shader {
    name : String,
    handle : u32,
}

impl Shader {
    pub fn register( name : &str ) -> Self {
        let h = unsafe{register_shader_nomip(name)};
        
        Shader {
            name : name.to_owned(),
            handle : h,
        }
    }

    pub fn draw(&self, x : i32, y : i32, w : i32, h : i32) {
        unsafe{draw_stretch_pic(x as f32, y as f32, w as f32, h as f32, 0.0, 0.0, 1.0, 1.0, self.handle)}
    }
}

unsafe fn register_shader_nomip( name : &str ) -> u32 {
    let c_name = cl::create_cstring(name);
    cl::SYSCALL(uiImport_t::UI_R_REGISTERSHADERNOMIP as intptr_t, c_name.as_ptr()) as u32
}

unsafe fn draw_stretch_pic(x : f32, y : f32, w : f32, h : f32, s1 : f32, t1 : f32, s2 : f32, t2 : f32, shader : u32) {
    let fi_x = cl::floatint_t { f : x };
    let fi_y = cl::floatint_t { f : y };
    let fi_w = cl::floatint_t { f : w };
    let fi_h = cl::floatint_t { f : h };
    let fi_s1 = cl::floatint_t { f : s1 };
    let fi_t1 = cl::floatint_t { f : t1 };
    let fi_s2 = cl::floatint_t { f : s2 };
    let fi_t2 = cl::floatint_t { f : t2 };
    
    cl::SYSCALL(uiImport_t::UI_R_DRAWSTRETCHPIC as intptr_t, fi_x, fi_y, fi_w, fi_h, fi_s1, fi_t1, fi_s2, fi_t2, shader);
}
