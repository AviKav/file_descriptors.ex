use rustler::{Binary, Encoder, Env, Error, NifResult, ResourceArc, Term};
use std::{fs::File, os::fd::AsRawFd};

mod atoms {
    rustler::atoms! {
        fd,
        file
    }
}

rustler::init!("Elixir.FileDescriptors", [_open], load = load);
fn load(env: Env, _: Term) -> bool {
    rustler::resource!(FileResource, env);
    true
}

struct FileResource {
    file: File,
}

fn wrap_err<'a, T: 'static + Encoder>(val: T) -> Error {
    Error::Term(Box::new(val))
}

fn binary_as_utf8(binary: Binary) -> NifResult<&str> {
    std::str::from_utf8(binary.as_slice()).map_err(|e| wrap_err(e.valid_up_to()))
}

fn monadic_open(path: &str) -> NifResult<File> {
    File::open(path).map_err(|e| wrap_err(e.to_string()))
}

#[rustler::nif(schedule = "DirtyIo")]
pub fn _open<'a>(env: Env<'a>, file_path: Binary) -> NifResult<Term<'a>> {
    NifResult::Ok(file_path)
        .and_then(binary_as_utf8)
        .and_then(monadic_open)
        .and_then(|file: File| {
            let file_resource: ResourceArc<FileResource> =
                ResourceArc::new(FileResource { file: file });

            Ok(Term::map_new(env))
                .and_then(|map| map.map_put(atoms::fd(), file_resource.file.as_raw_fd()))
                .and_then(|map| map.map_put(atoms::file(), file_resource))
        })
}
