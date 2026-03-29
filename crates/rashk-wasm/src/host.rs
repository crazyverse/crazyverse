use async_trait::async_trait;
use rashk_core::error::RashkError;
use rashk_core::module::{ModuleId, ModuleManifest};
use rashk_core::ports::{ModuleRegistry, ModuleRuntime};
use std::collections::HashMap;
use std::sync::Mutex;

/// WASM module host backed by wasmtime.
pub struct WasmHost {
    engine: wasmtime::Engine,
    modules: Mutex<HashMap<String, LoadedModule>>,
    manifests: Mutex<HashMap<String, ModuleManifest>>,
    wasm_dir: String,
}

struct LoadedModule {
    module: wasmtime::Module,
}

impl WasmHost {
    pub fn new(wasm_dir: &str) -> Result<Self, RashkError> {
        let mut config = wasmtime::Config::new();
        config.async_support(true);
        config.wasm_component_model(true);

        let engine =
            wasmtime::Engine::new(&config).map_err(|e| RashkError::Module(e.to_string()))?;

        Ok(Self {
            engine,
            modules: Mutex::new(HashMap::new()),
            manifests: Mutex::new(HashMap::new()),
            wasm_dir: wasm_dir.to_string(),
        })
    }
}

#[async_trait]
impl ModuleRegistry for WasmHost {
    async fn list_installed(&self) -> Result<Vec<ModuleManifest>, RashkError> {
        let manifests = self.manifests.lock().unwrap();
        Ok(manifests.values().cloned().collect())
    }

    async fn install(
        &self,
        manifest: &ModuleManifest,
        wasm_bytes: &[u8],
    ) -> Result<ModuleId, RashkError> {
        let module = wasmtime::Module::new(&self.engine, wasm_bytes)
            .map_err(|e| RashkError::Module(e.to_string()))?;

        let id = manifest.id.clone();
        let path = format!("{}/{}.wasm", self.wasm_dir, id.0.replace('/', "_"));
        std::fs::write(&path, wasm_bytes)
            .map_err(|e| RashkError::Module(format!("write wasm: {e}")))?;

        self.modules
            .lock()
            .unwrap()
            .insert(id.0.clone(), LoadedModule { module });
        self.manifests
            .lock()
            .unwrap()
            .insert(id.0.clone(), manifest.clone());

        Ok(id)
    }

    async fn uninstall(&self, id: &ModuleId) -> Result<(), RashkError> {
        self.modules.lock().unwrap().remove(&id.0);
        self.manifests.lock().unwrap().remove(&id.0);
        let path = format!("{}/{}.wasm", self.wasm_dir, id.0.replace('/', "_"));
        let _ = std::fs::remove_file(&path);
        Ok(())
    }

    async fn get_manifest(&self, id: &ModuleId) -> Result<Option<ModuleManifest>, RashkError> {
        Ok(self.manifests.lock().unwrap().get(&id.0).cloned())
    }
}

#[async_trait]
impl ModuleRuntime for WasmHost {
    async fn load(&self, id: &ModuleId) -> Result<(), RashkError> {
        if self.modules.lock().unwrap().contains_key(&id.0) {
            return Ok(());
        }
        let path = format!("{}/{}.wasm", self.wasm_dir, id.0.replace('/', "_"));
        let bytes =
            std::fs::read(&path).map_err(|e| RashkError::Module(format!("read wasm: {e}")))?;
        let module = wasmtime::Module::new(&self.engine, &bytes)
            .map_err(|e| RashkError::Module(e.to_string()))?;
        self.modules
            .lock()
            .unwrap()
            .insert(id.0.clone(), LoadedModule { module });
        Ok(())
    }

    async fn call(
        &self,
        id: &ModuleId,
        _function: &str,
        _input: serde_json::Value,
    ) -> Result<serde_json::Value, RashkError> {
        let modules = self.modules.lock().unwrap();
        if !modules.contains_key(&id.0) {
            return Err(RashkError::NotFound(format!("module not loaded: {}", id)));
        }
        // TODO: instantiate with WASI + host functions, call the export
        Ok(serde_json::json!({"status": "not_yet_implemented"}))
    }

    async fn unload(&self, id: &ModuleId) -> Result<(), RashkError> {
        self.modules.lock().unwrap().remove(&id.0);
        Ok(())
    }

    async fn loaded(&self) -> Result<Vec<ModuleId>, RashkError> {
        Ok(self
            .modules
            .lock()
            .unwrap()
            .keys()
            .map(|k| ModuleId(k.clone()))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_host_creates() {
        let dir = std::env::temp_dir().join("rashk_wasm_test");
        let _ = std::fs::create_dir_all(&dir);
        let host = WasmHost::new(dir.to_str().unwrap()).unwrap();
        let installed = host.list_installed().await.unwrap();
        assert!(installed.is_empty());
    }
}
