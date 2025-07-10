# PTY Testing Patterns and Best Practices

## Temp Directory Management

### ✅ CORRECT: Use tempfile crate
```rust
use tempfile::TempDir;

let temp_dir = TempDir::new().unwrap();
copy_fixture_to_dir(temp_dir.path(), "fixture.txt", "target.txt");

let mut child = pty.spawn_command(
    &mut Command::new("vim")
        .arg("target.txt")
        .current_dir(temp_dir.path()),
).unwrap();
```

### ❌ WRONG: Manual temp paths or current directory
```rust
// Don't do this
let temp_file = format!("test_{}.txt", process::id());
let mut child = pty.spawn_command(&mut Command::new("vim").arg(&temp_file)).unwrap();
```

## Fixture Usage

### ✅ CORRECT: Copy fixtures to temp directory
```rust
fn copy_fixture_to_dir(dir: &Path, fixture_name: &str, target_name: &str) {
    let fixture_path = Path::new("tests/fixtures").join(fixture_name);
    let target_path = dir.join(target_name);
    fs::copy(&fixture_path, &target_path).unwrap();
}

// Usage
copy_fixture_to_dir(temp_dir.path(), "multiline_content.txt", "test_visual.txt");
```

### ❌ WRONG: Inline content creation
```rust
// Don't do this
let content = "First line\nSecond line\nThird line";
create_file_in_dir(temp_dir.path(), "test.txt", content);
```

## Snapshot Testing

### ✅ CORRECT: Inline snapshots only
```rust
let snapshot = pty.get_snapshot();
insta::assert_snapshot!(snapshot, @r"
First line of text                      \n
Second line with more content           \n
Third line                              \n
~                                       \n
-- VISUAL --                            \n
");
```

### ❌ WRONG: External snapshots or string matching
```rust
// Don't do this
insta::assert_snapshot!(snapshot); // Creates external .snap files
assert!(snapshot.contains("some text")); // Fragile string matching
```

## Regenerating Snapshots

1. Use empty inline format to force regeneration:
```rust
insta::assert_snapshot!(snapshot, @"");
```

2. Run tests to generate content:
```bash
cargo test --test test_name
```

3. Accept new snapshots:
```bash
cargo insta accept
```

## Stable Filename Patterns

### ✅ CORRECT: Consistent, meaningful names
- `test_visual.txt`
- `test_split.txt` 
- `tab1.txt`, `tab2.txt`, `tab3.txt`
- `test_macro.txt`

### ❌ WRONG: Process IDs or random names
- `test_visual_1234.txt`
- `temp_file_567.txt`
- Dynamic names that change between runs

## Test Structure Template

```rust
#[test]
fn test_vim_feature_pty() {
    let temp_dir = TempDir::new().unwrap();
    copy_fixture_to_dir(temp_dir.path(), "source_fixture.txt", "test_file.txt");

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("test_file.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    // Test operations
    pty.send_input_str("v").unwrap();
    sleep(Duration::from_millis(100));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    Expected content here...
    ");

    pty.send_input_str("\x1b:q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
    // TempDir automatically cleans up
}
```

## Key Benefits

1. **Deterministic**: Same fixtures + same filenames = stable tests
2. **Isolated**: Each test runs in its own temp directory
3. **Cross-platform**: `tempfile` handles OS differences
4. **Maintainable**: Inline snapshots visible in source code
5. **Reliable**: Real fixtures + real applications = accurate validation