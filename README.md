# starcoin-graphd

A minimal DAG visualization module for Starcoin, supporting GHOSTDAG coloring and structure analysis.

This library extracts block headers and GHOSTDAG data from a `ChainReader` and constructs a renderable DAG, with support for:

- Blue / Red / Unknown node coloring
- Edge identification with selected-parent marking
- Node score extraction
- Visualization-ready structures (e.g., for JSON or DOT output)

---

## 🔧 Features

- `ChainReader` and `ChainReaderExt` abstraction
-  DAG graph provider via `DagGraphProvider` trait
- `DagGraph` struct with renderable node/edge data
- `MockChainReader` for offline tests and development
