<div align="center">

<pre>
▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄   ▄▄▄▄▄  ▄▄▄▄▄▄▄ ▄▄▄      ▄▄▄
███▀▀███▄ ███▀▀███▄  ███  █████▀▀▀ ████▄  ▄████
███▄▄███▀ ███▄▄███▀  ███   ▀████▄  ███▀████▀███
███▀▀▀▀   ███▀▀██▄   ███     ▀████ ███  ▀▀  ███
███       ███  ▀███ ▄███▄ ███████▀ ███      ███
                    T O O L B O X
</pre>

[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](Cargo.toml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.78+-orange.svg)](https://www.rust-lang.org/)
[![Angular](https://img.shields.io/badge/Angular-18+-red.svg)](https://angular.dev/)
[![Docker](https://img.shields.io/badge/Docker-Compose-2496ED?logo=docker&logoColor=white)](https://docs.docker.com/compose/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

**A self-hosted, open-source converter and utility toolbox.**
Built with Angular and Rust. No telemetry. No accounts. Just tools.

</div>

---

## Overview

Most online file conversion tools share the same problems: intrusive ads, opaque privacy policies, and files that quietly pass through third-party servers you've never heard of. **Prism** solves this by running entirely on your own hardware.

Upload a file, get a file back. Nothing leaves your server.

**Why Prism?**

- **Private by design** — your files never touch an external service.
- **Fast** — conversion logic is written in Rust, processing happens in memory with no temp-file overhead.
- **Self-contained** — one `docker compose up` and you're running. No cloud accounts, no API keys, no subscriptions.
- **Extensible** — the tool pipeline is modular; adding a new conversion type is straightforward.

> ⚠️ Prism is in early development. The API surface may change between releases.

---

## Installation

### Prerequisites

Make sure you have the following installed before proceeding:

| Tool | Version | Notes |
|------|---------|-------|
| [Rust](https://rustup.rs/) | 1.78+ | Install via `rustup` |
| [Node.js](https://nodejs.org/) | 20+ | Required for the Angular frontend |
| [Docker](https://docs.docker.com/get-docker/) | 24+ | Optional, recommended for production |

---

### Running Locally (Development)

**1. Clone the repository**

```bash
git clone https://github.com/[owner]/prism.git
cd prism
```

**2. Start the backend**

```bash
cd backend
cargo run
```

The API will be available at `http://localhost:8080`. You can verify it is running:

```bash
curl http://localhost:8080/api/health
# → Healthy
```

**3. Start the frontend**

```bash
cd frontend
npm install
npm run start
```

The UI will be available at `http://localhost:4200`.

---

### Running with Docker Compose

> Docker Compose support is on the roadmap. The instructions below reflect the planned setup.

```bash
docker compose up --build
```

This will start both the Rust backend and the Angular frontend. By default:
- Backend: `http://localhost:8080`
- Frontend: `http://localhost:4200`

To run in detached mode:

```bash
docker compose up -d
```

---

## API Reference & Examples

The backend exposes a simple REST API. All endpoints accept `multipart/form-data` with two fields:

| Field | Type | Description |
|-------|------|-------------|
| `options` | JSON string | Serialised options object specific to the tool |
| `file` | Binary | The file to process |

All responses follow a consistent envelope:

```json
{
  "ok": true,
  "data": { ... },
  "error": null
}
```

On failure:

```json
{
  "ok": false,
  "data": null,
  "error": {
    "code": "BAD_REQUEST",
    "message": "Invalid input: unsupported format"
  }
}
```

---

### Image Conversion

**`POST /api/image/convert`**

Converts an image to a target format. Supported targets: `png`, `jpg`, `webp`, `bmp`.

**Request fields:**

| Field | Type | Values |
|-------|------|--------|
| `options` | JSON | `{ "target": "png" \| "jpg" \| "webp" \| "bmp" }` |
| `file` | Binary | Source image (PNG, JPEG, WEBP, BMP, GIF, AVIF, etc.) |

**Example — convert a PNG to WebP using `curl`:**

```bash
curl -X POST http://localhost:8080/api/image/convert \
  -F 'options={"target":"webp"}' \
  -F 'file=@photo.png' \
  --output result.webp
```

**Example — convert from JavaScript (browser):**

```javascript
const formData = new FormData();
formData.append("options", JSON.stringify({ target: "jpg" }));
formData.append("file", fileInput.files[0]);

const response = await fetch("http://localhost:8080/api/image/convert", {
  method: "POST",
  body: formData,
});

const blob = await response.blob();
const url = URL.createObjectURL(blob);
```

The response body is the raw converted image bytes with the appropriate `Content-Type` header (e.g. `image/webp`). There is no JSON wrapper — the file is returned directly.

---

### Health Check

**`GET /api/health`**

Returns a plain-text `Healthy` response. Useful for container orchestration liveness probes.

```bash
curl http://localhost:8080/api/health
# → Healthy
```

---

### [Audio Conversion — Coming Soon]

**`POST /api/audio/convert`** *(planned)*

Will transcode between common audio formats server-side. Supported targets will include `mp3`, `wav`, `flac`, and `ogg`.

---

## Contributing

Contributions are welcome — whether that's a bug report, a new tool, or a documentation fix.

### Getting Started

1. Fork the repository and create a feature branch:

```bash
git checkout -b feat/my-new-tool
```

2. Make your changes. Keep commits focused and atomic.

3. Open a pull request against `main` with a clear description of what you changed and why.

### Code Style

**Rust (backend)**

- Follow standard `rustfmt` formatting — run `cargo fmt` before committing.
- Run `cargo clippy` and resolve all warnings before opening a PR.
- New tools should follow the existing pattern: a dedicated module under `src/tools/`, with options deserialized via `serde` and errors mapped to `AppError`.

**TypeScript (frontend)**

- Follow the Angular style guide.
- Run `npm run lint` before committing.

### Reporting Issues

- Search existing issues before opening a new one.
- For bugs, include: OS, Rust/Node version, the request you made, and the response you received.
- For feature requests, describe the use case — not just the implementation.

### What We're Looking For

- New conversion tools (audio, PDF, video — see Roadmap).
- Improved error messages.
- Performance improvements with benchmarks.
- Frontend UI contributions (Angular 18+).

---

## Roadmap

Track progress and planned features:

- [x] Image conversion (PNG, JPEG, WEBP, BMP)
- [x] Health check endpoint
- [x] Consistent JSON error/success envelope
- [ ] Image resizing and optimisation
- [ ] Audio conversion (MP3, WAV, FLAC, OGG)
- [ ] Frontend UI (Angular 18)
- [ ] Docker Compose setup
- [ ] PDF tools (merge, split, compress)
- [ ] Batch processing (multiple files in one request)
- [ ] [File size and dimension validation per tool]
- [ ] [Rate limiting for public deployments]

Have an idea for a tool? [Open a feature request](../../issues/new).

---

## License

Prism is open-source software, released under the [MIT License](LICENSE).

You are free to use, modify, and distribute it — including for commercial purposes — with attribution.

---

<div align="center">
<sub>Built by <a href="https://github.com/v1kyc">v1kyc</a> & Radek · © 2026 · <a href="LICENSE">MIT License</a></sub>
</div>
