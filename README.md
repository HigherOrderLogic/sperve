# sperve

A simple static file server.

## Environment variables

The app's behaviour can be configured via environment variables.

| Name | Default | Description |
|------|---------|-------------|
| `DIR` | `.` | The directory to serve |
| `HOST` | `0.0.0.0` | The host that the app will listen to |
| `PORT` | `8080` | The port that the app will listen to |
| `BROTLI` | *None* | When specified, the app will try to compress large files with brotli algorithm if client accepts brotli encoding |
| `GZIP` | *None* | When specified, the app will try to compress large files with gzip algorithm if client accepts gzip encoding |
| `SPA` | *None* | When specified, the app will run in "SPA mode", meaning that it will fallback to `index.html` for file that doesn't exist |