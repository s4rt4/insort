# Insort

A small, fast, **native desktop app for sorting PDF invoices** — built with [Tauri v2](https://tauri.app/).

Open a folder full of invoice PDFs, preview them, set each one's date, reorder them, and tick them off as you process them. No browser, no PHP, no server — just a ~30 MB native Windows `.exe`.

Insort is a native rewrite of an older PHP web app (`invoice-app`), turned into a real installable application.

## Features

- 📂 **Native folder picker** — pick a folder of PDFs via the OS dialog
- 📄 **PDF preview** — renders inline using WebView2's built-in viewer (zoom, print, rotate)
- 🗓️ **Date picker (1–31)** — tag each invoice with its day of the month; the list **auto-sorts by date**
- ↕️ **Drag to reorder** — fine-tune order with a grip handle; manual order is preserved across auto-sorts
- ✅ **Processed checkbox** — strike through invoices as you finish them
- 🔎 **Filename filter** — quickly narrow the list
- ↔️ **Resizable sidebar** — adjust the file list width
- 🎨 **4 pastel light themes** — saved between sessions

## Download

Grab the latest build from the [**Releases**](https://github.com/s4rt4/insort/releases) page:

| File | Use |
|------|-----|
| `Insort_x.y.z_x64-setup.exe` | NSIS installer (recommended) |
| `Insort_x.y.z_x64_en-US.msi` | MSI installer |
| `insort.exe` | Portable — no install, just run |

> Windows only. Requires the [WebView2 runtime](https://developer.microsoft.com/microsoft-edge/webview2/) (already present on Windows 10/11).

## Usage

1. Click **Open Folder** and choose the folder containing your invoice PDFs.
2. Click an invoice to preview it.
3. Use the calendar button to set each invoice's date (1–31). The list re-sorts by date automatically.
4. Drag rows by the grip handle to fine-tune order (e.g. same-date invoices).
5. Tick the checkbox to mark an invoice as processed.

## Build from source

Requires [Node.js](https://nodejs.org/), [Rust](https://rustup.rs/), and the MSVC build tools.

```bash
npm install

# Run in dev
npm run tauri dev

# Produce release binaries (exe + NSIS + MSI)
npm run tauri build
```

Outputs land in `src-tauri/target/release/`:

- `insort.exe` — portable executable
- `bundle/nsis/Insort_x.y.z_x64-setup.exe` — NSIS installer
- `bundle/msi/Insort_x.y.z_x64_en-US.msi` — MSI installer

## Tech

- **Tauri v2** with a plain static frontend (`src/index.html`, no bundler), `withGlobalTauri`
- **Rust** backend commands (`list_pdfs`, `read_pdf`) reading files directly via `std::fs`
- **Bootstrap 5.3** + **SortableJS** vendored locally (fully offline)

## License

[MIT](LICENSE) © s4rt4
