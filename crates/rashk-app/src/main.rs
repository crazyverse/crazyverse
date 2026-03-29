use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rashk", about = "Decentralized capability runtime")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show runtime status
    Status,
    /// Identity management
    Identity {
        #[command(subcommand)]
        action: IdentityAction,
    },
    /// Module management
    Module {
        #[command(subcommand)]
        action: ModuleAction,
    },
    /// Mesh network management
    Mesh {
        #[command(subcommand)]
        action: MeshAction,
    },
}

#[derive(Subcommand)]
enum IdentityAction {
    /// Show local identity
    Show,
    /// Generate a new identity
    Generate,
}

#[derive(Subcommand)]
enum ModuleAction {
    /// List installed modules
    List,
    /// Install a module from a .wasm file
    Install { path: String },
    /// Uninstall a module
    Uninstall { id: String },
}

#[derive(Subcommand)]
enum MeshAction {
    /// Show mesh status and connected peers
    Status,
    /// Discover peers on the network
    Discover,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rashk=info".parse().unwrap()),
        )
        .init();

    let cli = Cli::parse();

    // --- Composition root: wire adapters ---
    let data_dir = dirs_home().join(".rashk");
    std::fs::create_dir_all(&data_dir)?;

    let store = rashk_store::SqliteStore::open(
        data_dir.join("rashk.db").to_str().unwrap(),
    )?;

    let identity_path = data_dir.join("identity.key");
    let identity = rashk_identity::LocalIdentity::load_or_generate(&identity_path)?;
    let node_id = {
        use rashk_core::ports::IdentityPort;
        identity.local_identity().await?.node_id
    };

    let mesh = rashk_mesh::QuicMesh::new(node_id.clone());

    let wasm_dir = data_dir.join("modules");
    std::fs::create_dir_all(&wasm_dir)?;
    let wasm_host = rashk_wasm::WasmHost::new(wasm_dir.to_str().unwrap())?;

    // --- Dispatch ---
    match cli.command {
        None | Some(Commands::Status) => {
            println!("rashk v{}", env!("CARGO_PKG_VERSION"));
            println!("node:    {node_id}");
            println!("store:   {}", data_dir.join("rashk.db").display());
            println!("modules: {}", wasm_dir.display());

            use rashk_core::ports::{MeshPort, ModuleRegistry, Store};
            let record_count = store.count(&rashk_core::Query::default()).await?;
            let module_count = wasm_host.list_installed().await?.len();
            let peer_count = mesh.peers().await?.len();
            println!("records: {record_count}");
            println!("modules: {module_count}");
            println!("peers:   {peer_count}");
        }
        Some(Commands::Identity { action }) => {
            use rashk_core::ports::IdentityPort;
            match action {
                IdentityAction::Show => {
                    let id = identity.local_identity().await?;
                    println!("node_id:    {}", id.node_id);
                    println!("public_key: {}", hex::encode(&id.public_key));
                    if let Some(did) = &id.did {
                        println!("did:        {did}");
                    }
                }
                IdentityAction::Generate => {
                    let new_identity = rashk_identity::LocalIdentity::generate();
                    new_identity.save(&identity_path)?;
                    let id = {
                        use rashk_core::ports::IdentityPort;
                        new_identity.local_identity().await?
                    };
                    println!("generated new identity: {}", id.node_id);
                    println!("public_key: {}", hex::encode(&id.public_key));
                    println!("saved to: {}", identity_path.display());
                }
            }
        }
        Some(Commands::Module { action }) => {
            use rashk_core::ports::ModuleRegistry;
            match action {
                ModuleAction::List => {
                    let modules = wasm_host.list_installed().await?;
                    if modules.is_empty() {
                        println!("no modules installed");
                    }
                    for m in &modules {
                        println!("{} v{} — {}", m.id, m.version, m.description);
                    }
                }
                ModuleAction::Install { path } => {
                    let bytes = std::fs::read(&path)?;
                    let manifest = rashk_core::module::ModuleManifest {
                        id: rashk_core::module::ModuleId::new("local", &path),
                        version: "0.1.0".into(),
                        description: format!("Installed from {path}"),
                        kind: rashk_core::module::ModuleKind::Tool,
                        required_capabilities: vec![],
                        exports: vec![],
                        imports: vec![],
                        ui_entry: None,
                        publisher_key: None,
                        signature: None,
                        wasm_size: Some(bytes.len() as u64),
                        wasm_hash: None,
                        published_at: None,
                        metadata: serde_json::json!({}),
                    };
                    let id = wasm_host.install(&manifest, &bytes).await?;
                    println!("installed: {id}");
                }
                ModuleAction::Uninstall { id } => {
                    wasm_host
                        .uninstall(&rashk_core::module::ModuleId(id.clone()))
                        .await?;
                    println!("uninstalled: {id}");
                }
            }
        }
        Some(Commands::Mesh { action }) => {
            use rashk_core::ports::MeshPort;
            match action {
                MeshAction::Status => {
                    let local = mesh.local_node().await?;
                    let peers = mesh.peers().await?;
                    println!("local node: {local}");
                    println!("peers:      {}", peers.len());
                    for p in &peers {
                        println!("  {} — {:?}", p.node_id, p.capabilities);
                    }
                }
                MeshAction::Discover => {
                    let peers = mesh.discover().await?;
                    if peers.is_empty() {
                        println!("no peers discovered (mesh not yet wired)");
                    }
                    for p in &peers {
                        println!("  {} — {:?}", p.node_id, p.addresses);
                    }
                }
            }
        }
    }

    Ok(())
}

fn dirs_home() -> std::path::PathBuf {
    dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."))
}
