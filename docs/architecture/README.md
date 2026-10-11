# Architecture

The architecture is described as code with [LikeC4](https://likec4.dev) in [`model.c4`](model.c4):
the model (people, tools, cluster, namespaces, components, relationships) and the views generated from it.

| View | What it shows |
|---|---|
| `index` | System landscape |
| `cluster` | Namespaces and components inside the kind cluster |
| `app` | Hexagonal layers of `scratchplate-app` |
| `welcomeFlow` | Sequence: token retrieval, Istio JWT check, Spring re-validation |

## Work on the diagrams

Requires Node.js 20+ and npm.

```bash
cd docs/architecture
npm install
npm run start        # live preview at http://localhost:5173
npm run validate     # check the model
npm run build        # static site in dist/
npm run export:png   # PNGs in diagrams/
```

Edit `model.c4` when the infrastructure or the apps change (a new namespace, a new app, a new flow) and keep it in sync with `infra/` and `apps/`.
