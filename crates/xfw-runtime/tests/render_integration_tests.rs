use std::{env, path::PathBuf};

use tempfile::TempDir;
use tracing_subscriber::fmt;

use std::sync::Mutex;
use xfw_cli::RuntimeConfig;
use xfw_runtime::Runtime;

fn set_env_vars(pairs: &[(&str, &str)]) {
    unsafe {
        for (key, value) in pairs {
            env::set_var(key, value);
        }
    }
}

fn remove_env_vars(keys: &[&str]) {
    unsafe {
        for key in keys {
            env::remove_var(key);
        }
    }
}

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

fn write_config_dirty_changed(temp_dir: &TempDir) -> PathBuf {
    let path = temp_dir.path().join("config_dirty_changed.lua");
    let script = include_str!("fixtures/render_with-state-changed.lua");
    std::fs::write(&path, script).unwrap();
    path
}

fn write_config_dirty_complex(temp_dir: &TempDir) -> PathBuf {
    let path = temp_dir.path().join("config_dirty_complex.lua");
    let script = include_str!("fixtures/render_dirty_complex.lua");
    std::fs::write(&path, script).unwrap();
    path
}

fn write_config_dirty_complex_changed(temp_dir: &TempDir) -> PathBuf {
    let path = temp_dir.path().join("config_dirty_complex_changed.lua");
    let script = include_str!("fixtures/render_dirty_complex_changed.lua");
    std::fs::write(&path, script).unwrap();
    path
}

fn write_config_dirty_complex_visibility(temp_dir: &TempDir) -> PathBuf {
    let path = temp_dir.path().join("config_dirty_complex_visibility.lua");
    let script = include_str!("fixtures/render_dirty_complex_visibility.lua");
    std::fs::write(&path, script).unwrap();
    path
}

fn write_config_dirty_complex_multi(temp_dir: &TempDir) -> PathBuf {
    let path = temp_dir.path().join("config_dirty_complex_multi.lua");
    let script = include_str!("fixtures/render_dirty_complex_multi.lua");
    std::fs::write(&path, script).unwrap();
    path
}

fn write_config_with_fixture(temp_dir: &TempDir, filename: &str, fixture: &str) -> PathBuf {
    let path = temp_dir.path().join(filename);
    let script = match fixture {
        "render_complex" => include_str!("fixtures/render_complex.lua"),
        "render_complex_alt" => include_str!("fixtures/render_complex_alt.lua"),
        "render_with_state" => include_str!("fixtures/render_with_state.lua"),
        "render_with-state-changed" => include_str!("fixtures/render_with-state-changed.lua"),
        "render_dirty_complex" => include_str!("fixtures/render_dirty_complex.lua"),
        "render_dirty_complex_changed" => include_str!("fixtures/render_dirty_complex_changed.lua"),
        "render_dirty_complex_visibility" => {
            include_str!("fixtures/render_dirty_complex_visibility.lua")
        }
        "render_dirty_complex_multi" => include_str!("fixtures/render_dirty_complex_multi.lua"),
        _ => panic!("Unknown fixture: {}", fixture),
    };
    std::fs::write(&path, script).unwrap();
    path
}

fn dump_path(root: &PathBuf, name: &str) -> PathBuf {
    root.join("target").join("xfw-runtime-dumps").join(name)
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
        entrypoint: config_path.clone(),
    };
    let mut runtime = Runtime::new(config).unwrap();

    let (_width, _height, data) = runtime.render_once().unwrap();
    assert!(data.iter().any(|b| *b != 0));

    let dump_path_full = root
        .join("target")
        .join("xfw-runtime-dumps")
        .join("dirty_rect_full.png");
    assert!(dump_path_full.exists());

    std::fs::write(
        config_path.as_os_str(),
        include_str!("fixtures/render_with-state-changed.lua"),
    )
    .unwrap();

    unsafe {
        std::env::remove_var("XFW_RUNTIME_DUMP");
        std::env::set_var("XFW_RUNTIME_DUMP_NAME", "dirty_rect_partial.png");
        std::env::set_var("XFW_RUNTIME_DUMP", "1");
    }

    let config2 = RuntimeConfig {
        entrypoint: config_path,
    };
    let mut runtime2 = Runtime::new(config2).unwrap();
    runtime2.render_once().unwrap();

    let dump_path_partial = root
        .join("target")
        .join("xfw-runtime-dumps")
        .join("dirty_rect_partial.png");
    assert!(dump_path_partial.exists());

    let full_bytes = std::fs::read(&dump_path_full).unwrap();
    let partial_bytes = std::fs::read(&dump_path_partial).unwrap();
    assert_ne!(
        full_bytes, partial_bytes,
        "PNG should be different after config change"
    );

    std::env::set_current_dir(old_dir).unwrap();
    unsafe {
        std::env::remove_var("XFW_RUNTIME_DUMP");
        std::env::remove_var("XFW_RUNTIME_DUMP_NAME");
    }
}

#[test]
fn test_runtime_dirty_rect_nested() {
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
        std::env::set_var("XFW_RUNTIME_DUMP_NAME", "dirty_nested_full.png");
    }

    let temp_dir = TempDir::new().unwrap();
    let config_path = write_config_dirty_complex(&temp_dir);

    let config = RuntimeConfig {
        entrypoint: config_path.clone(),
    };
    let mut runtime = Runtime::new(config).unwrap();

    let (width, height, data) = runtime.render_once().unwrap();
    assert!(data.iter().any(|b| *b != 0));

    let dump_path_full = root
        .join("target")
        .join("xfw-runtime-dumps")
        .join("dirty_nested_full.png");
    assert!(dump_path_full.exists());
    let full_bytes = std::fs::read(&dump_path_full).unwrap();

    std::fs::write(
        config_path.as_os_str(),
        include_str!("fixtures/render_dirty_complex_changed.lua"),
    )
    .unwrap();

    unsafe {
        std::env::remove_var("XFW_RUNTIME_DUMP");
        std::env::set_var("XFW_RUNTIME_DUMP_NAME", "dirty_nested_partial.png");
        std::env::set_var("XFW_RUNTIME_DUMP", "1");
    }

    let config2 = RuntimeConfig {
        entrypoint: config_path,
    };
    let mut runtime2 = Runtime::new(config2).unwrap();

    let (width, height, data) = runtime2.render_once().unwrap();
    assert!(data.iter().any(|b| *b != 0));

    let dump_path_partial = root
        .join("target")
        .join("xfw-runtime-dumps")
        .join("dirty_nested_partial.png");
    assert!(dump_path_partial.exists());
    let partial_bytes = std::fs::read(&dump_path_partial).unwrap();

    assert_ne!(
        full_bytes, partial_bytes,
        "Dirty rect render should produce different output"
    );

    std::env::set_current_dir(old_dir).unwrap();
    unsafe {
        std::env::remove_var("XFW_RUNTIME_DUMP");
        std::env::remove_var("XFW_RUNTIME_DUMP_NAME");
    }
}

#[test]
fn test_runtime_dirty_rect_visibility() {
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
        std::env::set_var("XFW_RUNTIME_DUMP_NAME", "dirty_visibility_full.png");
    }

    let temp_dir = TempDir::new().unwrap();
    let config_path = write_config_dirty_complex(&temp_dir);

    let config = RuntimeConfig {
        entrypoint: config_path.clone(),
    };
    let mut runtime = Runtime::new(config).unwrap();

    let (width, height, data) = runtime.render_once().unwrap();
    assert!(data.iter().any(|b| *b != 0));

    let dump_path_full = root
        .join("target")
        .join("xfw-runtime-dumps")
        .join("dirty_visibility_full.png");
    assert!(dump_path_full.exists());
    let full_bytes = std::fs::read(&dump_path_full).unwrap();

    std::fs::write(
        config_path.as_os_str(),
        include_str!("fixtures/render_dirty_complex_visibility.lua"),
    )
    .unwrap();

    unsafe {
        std::env::remove_var("XFW_RUNTIME_DUMP");
        std::env::set_var("XFW_RUNTIME_DUMP_NAME", "dirty_visibility_hidden.png");
        std::env::set_var("XFW_RUNTIME_DUMP", "1");
    }

    let config2 = RuntimeConfig {
        entrypoint: config_path,
    };
    let mut runtime2 = Runtime::new(config2).unwrap();

    let (width, height, data) = runtime2.render_once().unwrap();
    assert!(data.iter().any(|b| *b != 0));

    let dump_path_hidden = root
        .join("target")
        .join("xfw-runtime-dumps")
        .join("dirty_visibility_hidden.png");
    assert!(dump_path_hidden.exists());
    let hidden_bytes = std::fs::read(&dump_path_hidden).unwrap();

    assert_ne!(
        full_bytes, hidden_bytes,
        "Visibility change should produce different output"
    );

    std::env::set_current_dir(old_dir).unwrap();
    unsafe {
        std::env::remove_var("XFW_RUNTIME_DUMP");
        std::env::remove_var("XFW_RUNTIME_DUMP_NAME");
    }
}

#[test]
fn test_runtime_dirty_rect_multiple_changes() {
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
        std::env::set_var("XFW_RUNTIME_DUMP_NAME", "dirty_multi_full.png");
    }

    let temp_dir = TempDir::new().unwrap();
    let config_path = write_config_dirty_complex(&temp_dir);

    let config = RuntimeConfig {
        entrypoint: config_path.clone(),
    };
    let mut runtime = Runtime::new(config).unwrap();

    let (width, height, data) = runtime.render_once().unwrap();
    assert!(data.iter().any(|b| *b != 0));

    unsafe {
        std::env::remove_var("XFW_RUNTIME_DUMP");
        std::env::set_var("XFW_RUNTIME_DUMP_NAME", "dirty_multi_change1.png");
        std::env::set_var("XFW_RUNTIME_DUMP", "1");
    }

    let dump_path_full = root
        .join("target")
        .join("xfw-runtime-dumps")
        .join("dirty_multi_full.png");
    assert!(dump_path_full.exists());
    let full_bytes = std::fs::read(&dump_path_full).unwrap();

    std::fs::write(
        config_path.as_os_str(),
        include_str!("fixtures/render_dirty_complex_multi.lua"),
    )
    .unwrap();

    unsafe {
        std::env::remove_var("XFW_RUNTIME_DUMP");
        std::env::set_var("XFW_RUNTIME_DUMP_NAME", "dirty_multi_change2.png");
        std::env::set_var("XFW_RUNTIME_DUMP", "1");
    }

    let config2 = RuntimeConfig {
        entrypoint: config_path,
    };
    let mut runtime2 = Runtime::new(config2).unwrap();

    let (width, height, data) = runtime2.render_once().unwrap();
    assert!(data.iter().any(|b| *b != 0));

    let dump_path = root
        .join("target")
        .join("xfw-runtime-dumps")
        .join("dirty_multi_change2.png");
    assert!(dump_path.exists());
    let change_bytes = std::fs::read(&dump_path).unwrap();

    assert_ne!(
        full_bytes, change_bytes,
        "Multiple changes should produce different output"
    );

    std::env::set_current_dir(old_dir).unwrap();
    unsafe {
        std::env::remove_var("XFW_RUNTIME_DUMP");
        std::env::remove_var("XFW_RUNTIME_DUMP_NAME");
    }
}

#[test]
fn test_runtime_manual_dirty_rect() {
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
        std::env::set_var("XFW_RUNTIME_DUMP_NAME", "manual_dirty_full.png");
    }

    let temp_dir = TempDir::new().unwrap();
    let config_path = write_config_alt(&temp_dir);

    let config = RuntimeConfig {
        entrypoint: config_path.clone(),
    };
    let mut runtime = Runtime::new(config).unwrap();

    let (_width, _height, data) = runtime.render_once().unwrap();
    assert!(data.iter().any(|b| *b != 0));

    let dump_path_full = root
        .join("target")
        .join("xfw-runtime-dumps")
        .join("manual_dirty_full.png");
    assert!(dump_path_full.exists());

    unsafe {
        std::env::remove_var("XFW_RUNTIME_DUMP");
        std::env::set_var("XFW_RUNTIME_DUMP_NAME", "manual_dirty_partial.png");
        std::env::set_var("XFW_RUNTIME_DUMP", "1");
    }

    let dirty_rect = xfw_layout::Rect {
        x: 50.0,
        y: 50.0,
        width: 100.0,
        height: 100.0,
    };
    runtime.render_with_dirty_rect(dirty_rect).unwrap();

    let dump_path_partial = root
        .join("target")
        .join("xfw-runtime-dumps")
        .join("manual_dirty_partial.png");
    assert!(dump_path_partial.exists());

    let full_size = std::fs::metadata(&dump_path_full).unwrap().len();
    let partial_size = std::fs::metadata(&dump_path_partial).unwrap().len();
    assert_ne!(full_size, partial_size, "PNG sizes should be different");

    std::env::set_current_dir(old_dir).unwrap();
    unsafe {
        std::env::remove_var("XFW_RUNTIME_DUMP");
        std::env::remove_var("XFW_RUNTIME_DUMP_NAME");
    }
}
