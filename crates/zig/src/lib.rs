use anyhow::Result;
use wit_bindgen_core::wit_parser::{Function, InterfaceId, Resolve, TypeId, WorldId, WorldKey, Type};
use wit_bindgen_core::{Files, Source, WorldGenerator};
use heck::ToSnakeCase;

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Opts {}
impl Opts {
    pub fn build(self) -> Box<dyn WorldGenerator> {
        Box::new(Zig {
            opts: self.clone(),
            ..Zig::default()
        })
    }
}

#[derive(Default)]
struct Zig {
    opts: Opts,
    src: Source,
}

impl WorldGenerator for Zig {
    // How many times are these functions supposd to be called?
    fn import_interface(
        &mut self,
        resolve: &Resolve,
        name: &WorldKey,
        id: InterfaceId,
        _files: &mut Files,
    ) -> Result<()> {
        let name_raw = &resolve.name_world_key(name);
        self.src
            .push_str(&format!("// Import functions from {name_raw}\n"));

        for (name, func) in resolve.interfaces[id].functions.iter() {
            let fn_name_snake = func.name.to_snake_case();

            self.src.push_str(&format!("extern \"todo-namespace\" fn {fn_name_snake}("));
            for (param_name, param_type) in func.params.iter(){
                let param_name_snake = param_name.to_snake_case();
                let param_type_zig = get_ty(param_type);
                self.src.push_str(&format!("{param_name_snake}: {param_type_zig}"));
            }
            self.src.push_str(") ");
            // TODO: do return type here <02-11-24, Max Schulte> //
            match func.results.len() {
                0 => {
                    self.src.push_str("void");
                },
                1 => {
                    self.src.push_str(&get_ty(func.results.iter_types().next().unwrap()));
                },
                _ => {
                    panic!("have not implemented tuple return types");
                }
            }
            self.src.push_str(";\n");
        }
        println!("{}", self.src.as_str());
        Ok(())
    }
    fn export_interface(
        &mut self,
        _resolve: &Resolve,
        _name: &WorldKey,
        _iface: InterfaceId,
        _files: &mut Files,
    ) -> Result<()> {
        panic!("export_interface not implemented");
    }
    fn import_funcs(
        &mut self,
        _resolve: &Resolve,
        _world: WorldId,
        _funcs: &[(&str, &Function)],
        _files: &mut Files,
    ) {
        panic!("import_funcs not implemented");
    }
    fn export_funcs(
        &mut self,
        _resolve: &Resolve,
        _world: WorldId,
        _funcs: &[(&str, &Function)],
        _files: &mut Files,
    ) -> Result<()> {
        panic!("export_funcs not implemented");
    }
    fn import_types(
        &mut self,
        _resolve: &Resolve,
        _world: WorldId,
        _types: &[(&str, TypeId)],
        _files: &mut Files,
    ) {
        panic!("import_types not implemented");
    }
    fn finish(&mut self, _resolve: &Resolve, _world: WorldId, _files: &mut Files) -> Result<()> {
        Ok(())
    }


}
    // Utilities
    fn get_ty(ty: &Type) -> String {
        match ty {
            Type::Bool => "bool",
            Type::U8 | Type::Char => "u8",
            Type::U16 => "u16",
            Type::U32 => "u32",
            Type::U64 => "u64",
            Type::S8 => "i8",
            Type::S16 => "i16",
            Type::S32 => "i32",
            Type::S64 => "i64",
            Type::F32 => "f32",
            Type::F64 => "f64",
            _ => panic!("unsupported type: {:?}", ty)
        }.into()
    }
