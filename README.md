# flxp
```
a simple CLI based file/paste sharing application, written in rust.
```

### Installation

- Requirements: cargo

```
cargo install --git https://github.com/xorvet/flxp
```

### Usage
- **Paste**
  - Set Pastebin API key: ```flxp --pbk <YOUR_PASTEBIN_API_KEY>```
  - Upload: ```flxp -p <TEXT>```
- **File**
  - Upload: ```flxp -f <PATH_TO_FILE>```

#### Feature to Add:
- File/Paste customization (e.g. : expiration, etc..).
- Encryption.
- Downloading.
- Help/Usage.
