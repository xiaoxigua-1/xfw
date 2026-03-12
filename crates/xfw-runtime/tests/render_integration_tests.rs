use std::path::PathBuf;

use tempfile::TempDir;
use tracing_subscriber::fmt;

use std::sync::Mutex;
use xfw_cli::RuntimeConfig;
use xfw_runtime::Runtime;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn write_config(temp_dir: &TempDir) -> PathBuf {
    let path = temp_dir.path().join("config.lua");
    let script = include_str!("fixtures/render_complex.lua");
    std::fs::write(&path, script).unwrap();
    path
}

fn write_config_alt(temp_dir: &TempDir) -> PathBuf {
    let path = temp_dir.path().join("config_alt.lua");
    let script = include_str!("fixtures/render_complex_alt.lua");
    std::fs::write(&path, script).unwrap();
    path
}

fn write_config_with_state(temp_dir: &TempDir) -> PathBuf {
    let path = temp_dir.path().join("config_state.lua");
    let script = include_str!("fixtures/render_with_state.lua");
    std::fs::write(&path, script).unwrap();
    path
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

#[test]
fn test_runtime_render_once_produces_pixels() {
    let _env_guard = ENV_LOCK.lock().unwrap();
    let _ = fmt()
        .with_test_writer()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();
    let root = workspace_root();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&root).unwrap();

    unsafe {
        std::env::set_var("XFW_RUNTIME_DUMP", "1");
        std::env::set_var("XFW_RUNTIME_DUMP_NAME", "frame.png");
        std::env::set_var("XFW_RENDER_DEBUG", "1");
    }

    let temp_dir = TempDir::new().unwrap();
    let config_path = write_config(&temp_dir);

    let config = RuntimeConfig {
        entrypoint: config_path,
    };
    let mut runtime = Runtime::new(config).unwrap();
    let (width, height, data) = runtime.render_once().unwrap();

    let dump_path = root
        .join("target")
        .join("xfw-runtime-dumps")
        .join("frame.png");
    let dump_metadata = std::fs::metadata(&dump_path).unwrap();

    std::env::set_current_dir(old_dir).unwrap();
    unsafe {
        std::env::remove_var("XFW_RUNTIME_DUMP");
        std::env::remove_var("XFW_RUNTIME_DUMP_NAME");
        std::env::remove_var("XFW_RENDER_DEBUG");
    }

    assert_eq!(data.len(), width as usize * height as usize * 4);
    assert!(data.iter().any(|b| *b != 0));
    assert!(dump_metadata.len() > 0);
}

#[test]
fn test_runtime_render_once_produces_pixels_alt() {
    let _env_guard = ENV_LOCK.lock().unwrap();
    let _ = fmt()
        .with_test_writer()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();
    let root = workspace_root();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&root).unwrap();

    unsafe {
        std::env::set_var("XFW_RUNTIME_DUMP", "1");
        std::env::set_var("XFW_RUNTIME_DUMP_NAME", "frame_alt.png");
        std::env::set_var("XFW_RENDER_DEBUG", "1");
    }

    let temp_dir = TempDir::new().unwrap();
    let config_path = write_config_alt(&temp_dir);

    let config = RuntimeConfig {
        entrypoint: config_path,
    };
    let mut runtime = Runtime::new(config).unwrap();
    let (width, height, data) = runtime.render_once().unwrap();

    let dump_path = root
        .join("target")
        .join("xfw-runtime-dumps")
        .join("frame_alt.png");
    let dump_metadata = std::fs::metadata(&dump_path).unwrap();

    std::env::set_current_dir(old_dir).unwrap();
    unsafe {
        std::env::remove_var("XFW_RUNTIME_DUMP");
        std::env::remove_var("XFW_RUNTIME_DUMP_NAME");
        std::env::remove_var("XFW_RENDER_DEBUG");
    }

    assert_eq!(data.len(), width as usize * height as usize * 4);
    assert!(data.iter().any(|b| *b != 0));
    assert!(dump_metadata.len() > 0);
}

#[test]
fn test_runtime_dirty_rect_pipeline() {
    let _env_guard = ENV_LOCK.lock().unwrap();
    let _ = fmt()
        .with_test_writer()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();
    let root = workspace_root();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&root).unwrap();

    unsafe {
        std::env::set_var("XFW_RUNTIME_DUMP", "1");
        std::env::set_var("XFW_RUNTIME_DUMP_NAME", "dirty_rect_full.png");
    }

    let temp_dir = TempDir::new().unwrap();
    let config_path = write_config_with_state(&temp_dir);

    let config = RuntimeConfig {
        entrypoint: config_path,
    };
    let mut runtime = Runtime::new(config).unwrap();

    let (width, height, data) = runtime.render_once().unwrap();
    assert_eq!(data.len(), width as usize * height as usize * 4);
    assert!(data.iter().any(|b| *b != 0));

    let dump_path_full = root
        .join("target")
        .join("xfw-runtime-dumps")
        .join("dirty_rect_full.png");
    assert!(dump_path_full.exists());

    unsafe {
        std::env::remove_var("XFW_RUNTIME_DUMP");
        std::env::set_var("XFW_RUNTIME_DUMP_NAME", "dirty_rect_partial.png");
        std::env::set_var("XFW_RUNTIME_DUMP", "1");
    }

    runtime.on_state_change("items").unwrap();

    let dump_path_partial = root
        .join("target")
        .join("xfw-runtime-dumps")
        .join("dirty_rect_partial.png");
    assert!(dump_path_partial.exists());

    std::env::set_current_dir(old_dir).unwrap();
    unsafe {
        std::env::remove_var("XFW_RUNTIME_DUMP");
        std::env::remove_var("XFW_RUNTIME_DUMP_NAME");
    }
}
