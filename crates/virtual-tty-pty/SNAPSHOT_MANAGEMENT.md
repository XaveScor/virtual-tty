# Snapshot Management Guidelines

## Rule: ALL Snapshots Must Be Inline

Every snapshot assertion MUST use the inline `@"content"` format. No external `.snap` files are allowed.

### ✅ CORRECT: Inline snapshots
```rust
insta::assert_snapshot!(snapshot, @r"
Line 1                                  \n
Line 2                                  \n
~                                       \n
-- VISUAL --                            \n
");
```

### ❌ FORBIDDEN: External snapshots
```rust
insta::assert_snapshot!(snapshot); // Creates external .snap files - NOT ALLOWED
```

## Converting External to Inline

If external `.snap` files exist, convert them using this process:

1. **Delete external files**:
```bash
rm tests/snapshots/*.snap
```

2. **Update test calls**:
```rust
// Change from:
insta::assert_snapshot!(snapshot);

// To:
insta::assert_snapshot!(snapshot, @"");
```

3. **Regenerate**:
```bash
cargo test --test test_name
cargo insta accept
```

## Snapshot Content Rules

1. **Small terminal sizes**: Use 40x10 or similar for readable snapshots
2. **Real fixture content**: Snapshots should contain actual fixture data
3. **Stable filenames**: Consistent names ensure stable snapshots
4. **Complete validation**: Snapshots capture entire terminal state

## Benefits of Inline Snapshots

- **Visible in source**: No need to open external files
- **Version control friendly**: Changes visible in diffs
- **Self-contained**: All test data in one place
- **Easier review**: Snapshot content in pull requests
- **No file management**: No external snapshot directory to maintain

## Example: Complete Inline Test

```rust
#[test]
fn test_vim_visual_mode_pty() {
    let temp_dir = TempDir::new().unwrap();
    copy_fixture_to_dir(temp_dir.path(), "multiline_content.txt", "test_visual.txt");

    let mut pty = PtyAdapter::new(40, 10);
    let mut child = pty
        .spawn_command(
            &mut Command::new("vim")
                .arg("test_visual.txt")
                .current_dir(temp_dir.path()),
        )
        .unwrap();
    sleep(Duration::from_millis(500));

    pty.send_input_str("v").unwrap();
    sleep(Duration::from_millis(100));

    let snapshot = pty.get_snapshot();
    insta::assert_snapshot!(snapshot, @r"
    First line of text                      \n
    Second line with more content           \n
    Third line                              \n
    Fourth line                             \n
    Fifth line                              \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    ~                                       \n
    -- VISUAL --                            \n
    ");

    pty.send_input_str("\x1b:q!\n").unwrap();
    child.wait().unwrap();
    pty.wait_for_completion();
}
```

## Verification

After making changes, verify no external snapshots exist:
```bash
find . -name "*.snap" | wc -l  # Should return 0
```