# srvcs-www

The `srvcs.cloud` website service.

This is a stamped srvcs microservice that serves the public website from
`static/index.html` while keeping the shared operational surface:

- `GET /` returns HTML for browser requests and JSON identity for API callers.
- `GET /healthz`, `GET /readyz`, `GET /metrics`, and `GET /openapi.json` follow
  the srvcs service standard.

## Local checks

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

See [`srvcs/platform`](https://github.com/srvcs/platform) for the shared service
standard and CI workflow.

## Preview workflow

Previews are maintainer opt-in.

1. Open a PR from a branch inside `srvcs/www`.
2. Add the `deploy-preview` label:

   ```sh
   gh pr edit <pr-number> --repo srvcs/www --add-label deploy-preview
   ```

3. CI publishes `ghcr.io/srvcs/www:pr-<number>-<sha>`.
4. `srvcs/infra` deploys the preview and comments the URL on the PR:

   ```text
   https://www-pr-<number>.srvcs.cloud
   ```

Removing the label or closing the PR asks infra to destroy the preview
namespace.

Fork PRs do not receive previews automatically. Move the reviewed change to a
branch inside `srvcs/www` before adding `deploy-preview`.

## Production promotion

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
