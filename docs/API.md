# HTTP API reference

**Version:** 0.1.0 (Milestone 1)
**Base URL:** `http://127.0.0.1:3000` (configurable, see the README)
**Format:** JSON (`application/json`)

This document is the contract for HTTP clients. It grows with each milestone;
endpoints planned for later milestones are listed in [ROADMAP.md](ROADMAP.md),
not here, so this page only ever describes behaviour that exists.

## Contents

1. [Conventions](#conventions)
2. [Endpoints](#endpoints)
   * [`GET /health`](#get-health)
3. [Framework-level responses](#framework-level-responses)
4. [Changelog](#changelog)

---

## Conventions

* Successful responses use `2xx` status codes; client mistakes use `4xx`;
  server faults use `5xx`.
* Response bodies are JSON unless stated otherwise.
* Paths are case-sensitive.

## Endpoints

### `GET /health`

Liveness probe. Reports that the process is running and able to serve HTTP.
It performs no I/O, has no side effects and is safe to call frequently.

**Request**

* No parameters.
* No request body.

**Responses**

| Status | Meaning | Body |
|--------|---------|------|
| `200 OK` | The service is up. | [`Health`](#health-object) |

**Response headers**

| Header | Value |
|--------|-------|
| `content-type` | `application/json` |

#### Health object

| Field | Type | Description |
|-------|------|-------------|
| `status` | string | Always `"ok"` while the service can answer. |
| `version` | string | Crate version of the running build (semantic version). |

**Example**

```bash
curl -i http://127.0.0.1:3000/health
```

```http
HTTP/1.1 200 OK
content-type: application/json

{"status":"ok","version":"0.1.0"}
```

**Idempotency:** safe and idempotent (`GET`).

---

## Framework-level responses

These are produced by axum's router before any of our handlers run. They are
covered by tests but their bodies are the framework defaults; a consistent
JSON error format is planned for Milestone 3.

| Situation | Status | Example |
|-----------|--------|---------|
| No route matches the path | `404 Not Found` | `GET /nope` |
| The path exists but the method is not registered | `405 Method Not Allowed` | `POST /health` |

## Changelog

| Version | Change |
|---------|--------|
| 0.1.0 | Initial release: `GET /health`. |
