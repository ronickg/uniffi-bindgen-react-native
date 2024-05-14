mod cpp_bindings;
mod rust_crate;
mod typescript;

use anyhow::{Ok, Result};
use camino::Utf8PathBuf;
use clap::Args;

use crate::{
    bootstrap::{Bootstrap, TestRunnerCmd},
    util::build_root,
};

use self::{cpp_bindings::CppBindingArg, rust_crate::CrateArg, typescript::EntryArg};

#[derive(Debug, Args)]
pub(crate) struct RunCmd {
    /// Clean the crate before starting.
    #[clap(long, short = 'c')]
    clean: bool,

    /// The crate to be bound to hermes
    #[command(flatten)]
    crate_: Option<CrateArg>,

    #[command(flatten)]
    cpp_binding: Option<CppBindingArg>,

    /// The Javascript or Typescript file.
    #[command(flatten)]
    js_file: EntryArg,
}

impl RunCmd {
    pub(crate) fn run(&self) -> Result<()> {
        TestRunnerCmd.ensure_ready()?;
        let so_file = self.prepare_library_path()?;

        let js_file = self.js_file.prepare()?;
        TestRunnerCmd.run(&js_file, so_file.as_ref())?;
        Ok(())
    }

    fn prepare_library_path(&self) -> Result<Option<Utf8PathBuf>> {
        let clean = self.clean;
        let (release, info) = if let Some(crate_) = &self.crate_ {
            (crate_.release, Some(crate_.cargo_build(clean)?))
        } else {
            (false, None)
        };

        match (&info, &self.cpp_binding) {
            (Some(crate_), Some(cpp)) => {
                let crate_lib = crate_.library_path(release);
                let target_dir = crate_lib
                    .parent()
                    .expect("target directory is parent of library file");
                let lib_name = crate_.library_name.as_str();
                let so_file = cpp.compile_with_crate(clean, target_dir, lib_name)?;
                Ok(Some(so_file))
            }
            (None, Some(cpp)) => {
                let so_file = cpp.compile_without_crate(clean)?;
                Ok(Some(so_file))
            }
            #[allow(unreachable_code)]
            #[allow(unused)]
            (Some(crate_), None) => {
                let crate_lib = crate_.library_path(release);
                let target_dir = crate_lib
                    .parent()
                    .expect("target directory is parent of library file");
                let lib_name = crate_.library_name.as_str();
                unimplemented!("Not yet able to generate cpp and js from a crate");

                let cpp_file = build_root()?.join(lib_name).join("extension.cpp");
                let cpp = CppBindingArg::new(cpp_file);
                let so_file = cpp.compile_with_crate(clean, target_dir, lib_name)?;
                Ok(Some(so_file))
            }
            (_, _) => Ok(None),
        }
    }
}