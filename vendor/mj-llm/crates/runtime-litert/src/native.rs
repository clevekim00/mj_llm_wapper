use crate::RuntimeError;
use mj_llm_core::{
    embedding::{Embedding, EmbeddingSpace},
    pin::VerifiedModel,
};
use std::{
    ffi::{CString, c_char, c_void},
    marker::PhantomData,
    path::Path,
    ptr::NonNull,
    rc::Rc,
};

unsafe extern "C" {
    fn mj_embedding_open(model: *const c_char, cache: *const c_char, out: *mut *mut c_void) -> i32;
    fn mj_embedding_compute(
        handle: *mut c_void,
        kind: u32,
        data: *const u8,
        size: usize,
        output: *mut f32,
        capacity: usize,
        written: *mut usize,
    ) -> i32;
    fn mj_embedding_close(handle: *mut c_void);
}

pub(super) struct NativeEngine {
    handle: NonNull<c_void>,
    _thread_confined: PhantomData<Rc<()>>,
}
impl NativeEngine {
    pub(super) fn load(model: &VerifiedModel, cache: &Path) -> Result<Self, RuntimeError> {
        // Re-verify immediately before SDK load, including changes since proof creation.
        let model = VerifiedModel::open(model.path())?;
        std::fs::create_dir_all(cache)?;
        let cache = cache.canonicalize()?;
        let to_c = |path: &Path| {
            CString::new(path.to_str().ok_or(RuntimeError::InvalidInput)?)
                .map_err(|_| RuntimeError::InvalidInput)
        };
        let path = to_c(model.path())?;
        let cache = to_c(&cache)?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: both strings and writable out pointer live through this synchronous call.
        // C++ copies path settings; catches exceptions; transfers one owning opaque handle on success.
        let status = unsafe { mj_embedding_open(path.as_ptr(), cache.as_ptr(), &mut raw) };
        if status != 0 {
            return Err(RuntimeError::Native(status));
        }
        let handle = NonNull::new(raw).ok_or(RuntimeError::Native(4))?;
        Ok(Self {
            handle,
            _thread_confined: PhantomData,
        })
    }
    pub(super) fn compute(&mut self, kind: u32, bytes: &[u8]) -> Result<Embedding, RuntimeError> {
        let space = EmbeddingSpace::n0();
        let mut output = vec![0.0; space.dimension()];
        let mut written = 0;
        // SAFETY: exclusive access prevents overlapping calls or destruction. Input is borrowed
        // for this call; C++ checks output capacity and copies values before freeing its response.
        let status = unsafe {
            mj_embedding_compute(
                self.handle.as_ptr(),
                kind,
                bytes.as_ptr(),
                bytes.len(),
                output.as_mut_ptr(),
                output.len(),
                &mut written,
            )
        };
        if status != 0 {
            return Err(RuntimeError::Native(status));
        }
        if written != output.len() {
            return Err(RuntimeError::Native(4));
        }
        Ok(Embedding::new(space, output)?)
    }
}
impl Drop for NativeEngine {
    fn drop(&mut self) {
        // SAFETY: this is the unique owning handle, no asynchronous calls/callbacks exist,
        // and !Send/!Sync confines destruction to the SDK's owner thread.
        unsafe { mj_embedding_close(self.handle.as_ptr()) };
    }
}

unsafe extern "C" {
    fn mj_generation_open(model: *const c_char, cache: *const c_char, out: *mut *mut c_void)
    -> i32;
    fn mj_generation_run(
        handle: *mut c_void,
        message: *const c_char,
        system: *const c_char,
        history: *const c_char,
        max_tokens: u32,
        output: *mut u8,
        capacity: usize,
        written: *mut usize,
    ) -> i32;
    fn mj_generation_close(handle: *mut c_void);
}

pub(crate) struct NativeGenerator {
    handle: NonNull<c_void>,
    _thread_confined: PhantomData<Rc<()>>,
}
impl NativeGenerator {
    pub(crate) fn load(
        model: &mj_llm_core::pin::VerifiedGenerationModel,
        cache: &Path,
    ) -> Result<Self, RuntimeError> {
        let model = mj_llm_core::pin::VerifiedGenerationModel::open(model.path())?;
        std::fs::create_dir_all(cache)?;
        let cache = cache.canonicalize()?;
        let to_c = |path: &Path| {
            CString::new(path.to_str().ok_or(RuntimeError::InvalidInput)?)
                .map_err(|_| RuntimeError::InvalidInput)
        };
        let path = to_c(model.path())?;
        let cache = to_c(&cache)?;
        let mut raw = std::ptr::null_mut();
        // SAFETY: synchronous C call receives valid strings and a writable handle slot.
        // Bridge catches exceptions and transfers unique ownership only on success.
        let status = unsafe { mj_generation_open(path.as_ptr(), cache.as_ptr(), &mut raw) };
        if status != 0 {
            return Err(RuntimeError::Native(status));
        }
        Ok(Self {
            handle: NonNull::new(raw).ok_or(RuntimeError::Native(4))?,
            _thread_confined: PhantomData,
        })
    }
    pub(crate) fn generate_messages(
        &mut self,
        messages: &[serde_json::Value],
        max_tokens: u32,
    ) -> Result<serde_json::Value, RuntimeError> {
        let to_native = |m: &serde_json::Value| serde_json::json!({"role":m["role"],"content":[{"type":"text","text":m["content"]}]});
        let has_system = messages[0]["role"] == "system";
        let system = CString::new(if has_system {
            to_native(&messages[0]).to_string()
        } else {
            String::new()
        })
        .map_err(|_| RuntimeError::InvalidInput)?;
        let history = CString::new(
            serde_json::Value::Array(
                messages[usize::from(has_system)..messages.len() - 1]
                    .iter()
                    .map(to_native)
                    .collect(),
            )
            .to_string(),
        )
        .map_err(|_| RuntimeError::InvalidInput)?;
        // The pinned Qwen3 artifact requires its documented /no_think control
        // (also used by the source N0 probe), in addition to SDK thinking settings.
        // Keep stored user text unchanged; this suffix is model preprocessing only.
        let mut last = messages.last().ok_or(RuntimeError::InvalidInput)?.clone();
        last["content"] = serde_json::Value::String(format!(
            "{} /no_think",
            last["content"].as_str().ok_or(RuntimeError::InvalidInput)?
        ));
        let message =
            CString::new(to_native(&last).to_string()).map_err(|_| RuntimeError::InvalidInput)?;
        let mut output = vec![0u8; 65536];
        let mut written = 0;
        // SAFETY: exclusive handle, live UTF-8 JSON, initialized writable output capacity.
        // Bridge copies JSON before freeing the response and returns only after generation ends.
        let status = unsafe {
            mj_generation_run(
                self.handle.as_ptr(),
                message.as_ptr(),
                system.as_ptr(),
                history.as_ptr(),
                max_tokens,
                output.as_mut_ptr(),
                output.len(),
                &mut written,
            )
        };
        if status != 0 {
            return Err(RuntimeError::Native(status));
        }
        if written == 0 || written > output.len() {
            return Err(RuntimeError::Native(4));
        }
        serde_json::from_slice(&output[..written]).map_err(|_| RuntimeError::Native(4))
    }
}
impl Drop for NativeGenerator {
    fn drop(&mut self) {
        // SAFETY: unique thread-confined owner; all generation calls are synchronous and finished.
        unsafe { mj_generation_close(self.handle.as_ptr()) };
    }
}
