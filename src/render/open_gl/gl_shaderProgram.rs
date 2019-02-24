extern crate gl;
extern crate uuid;
extern crate regex;


use self::gl::types::*;

use core::{Uniform, UniformItem};
use std::ffi::{CString};
use std::ptr;
use std::str;
use helpers::{find_file, read_to_string};
use super::gl_texture::{GLTextureIDs, GLTexture};
use super::BindContext;
use self::regex::Regex;


lazy_static! {
	static ref RE_INCLUDE: Regex = Regex::new(r"#include\s+<(\S+)>").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgramType {
	None,
	Vertex,
	Fragment,
}


#[derive(Debug)]
pub struct UniformLocation {
	location: i32,
	texture_slot: i32,
}

#[derive(Debug)]
pub struct GLShaderProgramID {
	pub fs_source: String,
	pub vs_source: String,
	pub id: GLuint,
	pub uniform_locations: Vec<UniformLocation>,
}

impl Drop for GLShaderProgramID {
	fn drop(&mut self) {
		println!("delete program");
		gl_call!({
			gl::DeleteProgram(self.id);
		});
	}
}


pub fn set_uniform(uniform: &mut Uniform, loc: &UniformLocation, texture_store: &mut GLTextureIDs) {
	if loc.location == -1 {return}

	match uniform {
		Uniform::Vector2(data) => {
			gl_call!({
				gl::Uniform2fv(loc.location, 1, &data.x as *const f32);
			});
		}
		Uniform::Vector3(data) => {
			gl_call!({
				gl::Uniform3fv(loc.location, 1, &data.x as *const f32);
			});
		}
		Uniform::Vector4(data) => {
			gl_call!({
				gl::Uniform4fv(loc.location, 1, &data.x as *const f32);
			});
		}
		Uniform::Matrix3f(data) => {
			gl_call!({
				gl::UniformMatrix3fv(loc.location, 1, gl::FALSE, &data.elements[0] as *const f32);
			});
		}
		Uniform::Matrix4f(data) => {
			gl_call!({
				gl::UniformMatrix4fv(loc.location, 1, gl::FALSE, &data.elements[0] as *const f32);
			});
		}
		Uniform::Float(data) => {
			gl_call!({
				gl::Uniform1f(loc.location, *data);
			});
		}
		Uniform::Int(data) => {
			gl_call!({
				gl::Uniform1i(loc.location, *data);
			});
		}
		Uniform::UInt(data) => {
			gl_call!({
				gl::Uniform1ui(loc.location, *data);
			});
		}

		Uniform::ArrVector2(data) => {
			gl_call!({
				gl::Uniform2fv(loc.location, data.len() as i32, &data[0].x as *const f32);
			});
		}
		Uniform::ArrVector3(data) => {
			gl_call!({
				gl::Uniform3fv(loc.location, data.len() as i32, &data[0].x as *const f32);
			});
		}
		Uniform::ArrVector4(data) => {
			gl_call!({
				gl::Uniform4fv(loc.location, data.len() as i32, &data[0].x as *const f32);
			});
		}
		Uniform::ArrMatrix3f(data) => {
			gl_call!({
				gl::UniformMatrix3fv(loc.location, data.len() as i32, gl::FALSE, &data[0].elements[0] as *const f32);
			});
		}
		Uniform::ArrMatrix4f(data) => {
			gl_call!({
				gl::UniformMatrix4fv(loc.location, data.len() as i32, gl::FALSE, &data[0].elements[0] as *const f32);
			});
		}

		Uniform::ArrFloat(data) => {
			gl_call!({
				gl::Uniform1fv(loc.location, data.len() as i32,  &data[0] as *const f32);
			});
		}
		Uniform::ArrInt(data) => {
			gl_call!({
				gl::Uniform1iv(loc.location, data.len() as i32,  &data[0] as *const i32);
			});
		}

		Uniform::ArrUInt(data) => {
			gl_call!({
				gl::Uniform1iv(loc.location, data.len() as i32,  *&data[0] as i32 as *const i32);
			});
		}

		Uniform::Texture2D(data) => {
			gl_call!({
				gl::ActiveTexture(gl::TEXTURE0 + loc.texture_slot as u32);
			});
			match data {
				Some(ref mut texture) => {
					let mut texture = texture.lock().unwrap();
					texture.bind(texture_store);
				}
				None => {
					gl_call!({
						gl::BindTexture(gl::TEXTURE_2D, 0);
					});
				}
			}
		}
	};
}


pub fn set_uniforms(uniforms: &mut [UniformItem], shader_program: &GLShaderProgramID, texture_store: &mut GLTextureIDs) {
	uniforms
		.iter_mut()
		.enumerate()
		.for_each(|(i, uniform_i)| {
			match uniform_i.uniform {
				Uniform::Texture2D( _ ) => {
					set_uniform(&mut uniform_i.uniform, &shader_program.uniform_locations[i], texture_store);
				}
				_=> {
					if uniform_i.need_update {
						set_uniform(&mut uniform_i.uniform, &shader_program.uniform_locations[i], texture_store);
						uniform_i.need_update = false;
					}
				}
			}

		});
}


pub fn read_shader_file(search_dirs: &Vec<&str>, path: &str) -> String {
	let path = path.to_string() + ".glsl";

	let p = find_file(&["src/render/open_gl/shaders"], &path).unwrap();
	let mut code = read_to_string(&p);

	while let Some(cap) = RE_INCLUDE.captures(&code.clone()) {
		let include_data = read_shader_file(search_dirs, &cap[1]);
		code = code.replace(&cap[0], &include_data);
	}

	code
}


fn set_definitions_fragment(code: &String, bind_context: &mut BindContext) -> String {

	let core_definitions = format!("#define NUM_POINT_LIGHTS {}\n", bind_context.render_settings.num_point_lights);

	let definitions: String = bind_context.definitions.iter()
		.filter(|e| e.0 == ProgramType::Fragment )
		.map(|e| {
			format!("#define {} {}\n", e.1, e.2)
		})
		.collect();
	
	format!("#version 330 core\n{}\n{}\n{}",core_definitions,  definitions, code)
}


fn set_definitions_vertex(code: &String, bind_context: &mut BindContext) -> String {	
	format!("#version 330 core\n{}", code)
}



pub fn get_program(src: &str, bind_context: &mut BindContext) -> GLShaderProgramID {
	let code = read_shader_file(&vec!("src/render/open_gl/shaders"), src);

	let mut shader_program = GLShaderProgramID {
		fs_source: String::from(""),
		vs_source: String::from(""),
		id: 0,
		uniform_locations: Vec::new(),
	};

	let mut write_to_prog = ProgramType::None;

	for line in code.lines() {
		if line.starts_with("#<vertex>") {
			write_to_prog = ProgramType::Vertex;
		} else if line.starts_with("#<fragment>") {
			write_to_prog = ProgramType::Fragment;
		} else {
			match write_to_prog {
				ProgramType::Vertex => {
					shader_program.vs_source += line;
					shader_program.vs_source += "\n";
				}
				ProgramType::Fragment => {
					shader_program.fs_source += line;
					shader_program.fs_source += "\n";
				}
				_ => {}
			}
		}
	}

	shader_program.fs_source = set_definitions_fragment(&shader_program.fs_source, bind_context);
	shader_program.vs_source = set_definitions_vertex(&shader_program.vs_source, bind_context);

	println!("========================================================");
	println!("{}", shader_program.vs_source);
	println!("========================================================");
	println!("{}", shader_program.fs_source);
	println!("========================================================");

	shader_program
}


pub fn compile_shader_program(src: &str, uniforms: &mut [UniformItem], bind_context: &mut BindContext ) -> GLShaderProgramID {
	println!("compile shader: {}", src);

	let mut program = get_program(src, bind_context);
	let id;
	// let fs_source = &program.fs_source;

	gl_call!({
		id = gl::CreateProgram();
		program.id = id;

		let vs = compile_shader(gl::VERTEX_SHADER, &program.vs_source[..], src);
		let fs = compile_shader(gl::FRAGMENT_SHADER, &program.fs_source[..], src);

		gl::AttachShader(id, fs);
		gl::AttachShader(id, vs);

		gl::LinkProgram(id);
		gl::ValidateProgram(id);

		let mut info_log = Vec::with_capacity(512);
		info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
		let mut success = gl::FALSE as GLint;
		gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
		if success != gl::TRUE as GLint {
			gl::GetProgramInfoLog(
				id,
				512,
				ptr::null_mut(),
				info_log.as_mut_ptr() as *mut GLchar,
			);
			// println!("{}", str::from_utf8_unchecked(&info_log));
			println!(
				"ERROR::SHADER::PROGRAM::COMPILATION_FAILED: {}\n{}",
				src,
				str::from_utf8(&info_log).unwrap()
			);
		}

		// TODO - release remove shaders
		gl::DeleteShader(vs);
		gl::DeleteShader(fs);
	});

	gl_call!({
		gl::UseProgram(program.id);
	});

	// let uniforms = material.get_uniforms();
	let mut uniform_locations = Vec::<UniformLocation>::with_capacity(uniforms.len());

	let mut location;
	let mut texture_slot = 0;
	let mut c_name;

	for uniform in uniforms.iter() {
		c_name = CString::new(uniform.name.as_bytes()).unwrap();

		match uniform.uniform {
			Uniform::Texture2D(_) => {
				gl_call!({
					location = gl::GetUniformLocation(program.id, c_name.as_ptr());
					gl::Uniform1i(location, texture_slot as i32);
				});

				uniform_locations.push(UniformLocation{location, texture_slot});
				texture_slot +=1;
			}
			_ => {
				gl_call!({
					location = gl::GetUniformLocation(program.id, c_name.as_ptr());
				});
				uniform_locations.push(UniformLocation{location, texture_slot: -1});
			}
		};

	}

	program.uniform_locations = uniform_locations;
	set_uniforms(uniforms, &program, bind_context.gl_texture_ids);
	program
}



pub fn compile_shader(t: GLenum, src: &str, src_path: &str) -> u32 {
	let id;

	gl_call!({
		id = gl::CreateShader(t);
		let c_str_frag = CString::new(src[..].as_bytes()).unwrap();


		let mut success = gl::FALSE as GLint;
		let mut info_log = Vec::with_capacity(1024);
		info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character

		gl::ShaderSource(id, 1, &c_str_frag.as_ptr(), ptr::null());
		gl::CompileShader(id);

		// check for shader compile errors
		gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
		if success != gl::TRUE as GLint {
			gl::GetShaderInfoLog(
				id,
				1024,
				ptr::null_mut(),
				info_log.as_mut_ptr() as *mut GLchar,
			);
			// println!("{}", str::from_utf8(&info_log).unwrap());
			match t {
				gl::FRAGMENT_SHADER => println!(
					"ERROR::SHADER::FRAGMENT::COMPILATION_FAILED: {}\n{}",
					src_path,
					str::from_utf8(&info_log).unwrap()
				),
				gl::VERTEX_SHADER => println!(
					"ERROR::SHADER::VERTEX::COMPILATION_FAILED: {}\n{}",
					src_path,
					str::from_utf8(&info_log).unwrap()
				),
				_ => println!(
					"ERROR::SHADER::?::COMPILATION_FAILED: {}\n{}",
					src_path,
					str::from_utf8(&info_log).unwrap()
				),
			};
			gl::DeleteShader(id);
			panic!();
		}
	});

	id
}



