# srvcs-www

## Name

| Field | Value |
| --- | --- |
| Service | `srvcs-www` |
| Slug | `www` |
| Repository | `srvcs/www` |
| Package | `srvcs-www` |
| Kind | `website` |

## Function

website: public srvcs.cloud site and service catalog

## Dependencies

None.

## API

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/` | Website HTML or JSON identity |
| `GET` | `/services.json` | Service catalog and dependency graph data |
| `GET` | `/assets/srvcs-logo.png` | Logo asset |
| `GET` | `/assets/srvcs-social.png` | Social preview asset |
| `GET` | `/site.webmanifest` | Web app manifest |
| `GET` | `/robots.txt` | Crawler policy |
| `GET` | `/sitemap.xml` | Sitemap |
| `GET` | `/healthz` | Liveness probe |
| `GET` | `/readyz` | Readiness probe |
| `GET` | `/metrics` | Prometheus metrics |
| `GET` | `/openapi.json` | OpenAPI document |

## Inputs

This service accepts an empty or ignored request body.

## Outputs

| Name | Type |
| --- | --- |
| `service` | `string` |

## Configuration

| Variable | Default | Purpose |
| --- | --- | --- |
| `SRVCS_BIND_ADDR` | `0.0.0.0:8080` | Bind address |
| `SRVCS_ENV` | `development` | Environment label for logs |
| `RUST_LOG` | `info,tower_http=info` | Tracing filter |

## Error Behavior

- `422` means the request could not be evaluated for the documented input shape.
- `503` means a required dependency was unavailable or returned an unexpected response.
- Dependency validation errors are forwarded when this service delegates validation.

## Local Checks

```sh
nix flake check -L
nix develop -c sh -euc 'cargo fmt --check; cargo clippy --all-targets -- -D warnings; cargo test'
nix build .#default -L
```

The Linux container is exposed as `.#container`. On Apple Silicon, use
`linux/arm64` for the practical local check; CI builds the release image on
native `x86_64-linux`.

```sh
docker run --rm --platform linux/arm64 -v "$PWD":/workspace -w /workspace nixos/nix:latest \
  sh -lc 'nix --extra-experimental-features "nix-command flakes" build .#container -L --out-link oci-image && cp -L oci-image image.tar.gz'
```

If Docker rejects Nix sandbox setup with a seccomp error, retry with
`--privileged` and add `--option sandbox false --option filter-syscalls false` to
the Nix command.

## Preview Workflow

Previews are maintainer opt-in.

1. Open a PR from a branch inside `srvcs/www`.
2. Add the `deploy-preview` label.
3. CI publishes `ghcr.io/srvcs/www:pr-<number>-<sha>`.
4. `srvcs/infra` deploys `https://www-pr-<number>.srvcs.cloud`.
5. Removing the label or closing the PR asks infra to destroy the preview namespace.

Fork PRs do not receive previews automatically. Move the reviewed change to a
branch inside `srvcs/www` before adding `deploy-preview`.

## Production Promotion

Production is promoted through `srvcs/infra`, not directly from this repository.

1. Merge the `srvcs/www` PR into `main`.
2. CI publishes `ghcr.io/srvcs/www:<main-commit-sha>`.
3. CI asks `srvcs/infra` to open or update a promotion PR.
4. Merge the `srvcs/infra` promotion PR.
5. Run the manual infra deploy workflow:

```sh
gh workflow run deploy-prod.yml --repo srvcs/infra --ref main
```

This split keeps website source, image publishing, production desired state, and
live deployment as separate steps.

## Metadata

Machine-readable service metadata lives in `srvcs.yaml`. The website catalog is
served from `static/services.json` and should be regenerated from service
metadata whenever service docs change.

See the [srvcs service standard](https://github.com/srvcs/platform/blob/main/STANDARD.md)
for the full operational contract.
