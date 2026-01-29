# Generating Extension Icons

Since ImageMagick is not installed, you have two options:

## Option 1: Install ImageMagick (Recommended)
```bash
brew install imagemagick
```

Then run:
```bash
cd browser-extension/icons
for size in 16 32 48 128; do
  convert icon128.svg -resize ${size}x${size} icon${size}.png
done
```

## Option 2: Use Online Tools
1. Go to https://redketchup.io/icon-converter
2. Upload `icon128.svg`
3. Generate icons in sizes: 16x16, 32x32, 48x48, 128x128
4. Download and save them in `browser-extension/icons/`

## Option 3: Use the SVG Directly (Temporary)
For development, you can temporarily use the SVG:

Update `manifest.json`:
```json
"action": {
  "default_icon": {
    "16": "icons/icon128.svg",
    "32": "icons/icon128.svg",
    "48": "icons/icon128.svg",
    "128": "icons/icon128.svg"
  }
}
```

Note: PNG icons are required for Chrome Web Store submission.
