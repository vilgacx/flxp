# flxp
```
a simple CLI based file/paste sharing application, developed in rust lang.
```

### Installation
- you can directly install the application here. If using Unix then use this command: ```chmod+x ./flxp && sudo mv ./flxp /usr/bin/``` to access it globally.
- or use the methods below to compile it yourself.

- #### Requirements
  - cargo
  - git
- #### Setup
  - ```git clone https://github.com/xorvet/flxp.git ```
  - ```cd flxp```
  - ```cargo build```
  - ```chmod+x ./target/debug/flxp && sudo cp ./target/debug/flxp /usr/bin/``` (to make it globally accessible [unix only])

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

```
It's probably my first rust based project so for any suggestions & queries DM on discord:
xorvet#2190
```
