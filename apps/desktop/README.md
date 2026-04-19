# Zaroxi Desktop App

This is the Tauri 2-based desktop application for Zaroxi Studio.

## Quick Start

```bash
# Navigate to the desktop app directory
cd apps/desktop

# Install dependencies
npm install

# Build Rust dependencies (from the root directory)
cd ../..
cargo build --workspace

# Return to desktop app and start development
cd apps/desktop
npm run tauri dev
```

## Prerequisites

- Node.js 18+ and npm
- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Tauri CLI will be installed locally via devDependencies

## Setup

1. Make sure you're in the correct directory:
   ```bash
   pwd  # Should show: .../zaroxi/apps/desktop
   ```

2. Install npm dependencies:
   ```bash
   npm install
   ```

3. Build the Rust workspace:
   ```bash
   cd ../..
   cargo build --workspace
   cd apps/desktop
   ```

## Development

Run the app in development mode:
```bash
npm run tauri dev
```

This will start:
- Frontend development server on http://localhost:1420
- Tauri application with hot reload

For frontend-only development (without Tauri):
```bash
npm run dev
```

## Building for Production

```bash
npm run tauri build
```

The built application will be in `src-tauri/target/release/`.

## Project Structure

- `src/` - React TypeScript frontend
- `src-tauri/` - Rust backend
- `public/` - Static assets

## Troubleshooting

### Common Issues

1. **"package.json not found" error**: You're not in the `apps/desktop` directory
   ```bash
   cd apps/desktop
   ```

2. **"npm install" fails**: Check Node.js version (should be 18+)
   ```bash
   node --version
   ```

3. **Rust dependencies not found**: Build from the root
   ```bash
   cd ../..
   cargo build --workspace
   cd apps/desktop
   ```

4. **Tauri not found**: It's installed as a dev dependency, no need for global install

### Development Tips

- Use `npm run dev` for frontend-only development (without Tauri)
- Use `npm run tauri dev` for full application with Rust backend
- Check browser console for frontend errors
- Check terminal for Rust backend errors
- If you see "Initializing..." forever, check the browser console for errors

## First Run Checklist

1. ✅ Navigate to `apps/desktop`
2. ✅ Run `npm install`
3. ✅ Run `cd ../.. && cargo build --workspace`
4. ✅ Run `cd apps/desktop && npm run tauri dev`

If you still have issues, run the setup check:
```bash
node check-setup.js
```
