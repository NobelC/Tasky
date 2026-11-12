# Tasky

**Project and Task manager**

---

## Requirements

- Rust (edition 2024)

---

## Build and run

```sh
cargo run
cargo build --release
cargo test
```

---

## Structure

```
tasky/
├── Cargo.toml
├── .editorconfig
├── .gitignore
├── LICENSE
├── README.md
├── doc/
│   └── architecture.md
│
├── tests/                           # integración
│   └── create_project.rs
│
└── src/
    ├── main.rs
    ├── app.rs
    ├── event.rs
    │
    ├── tui.rs
    ├── tui/
    │   ├── internal_section.rs
    │   └── drawing.rs               # ex utils_tui.rs: esquinas, cálculo de área
    │
    ├── backend.rs
    ├── backend/
    │   ├── project.rs               # datos crudos, siempre válidos
    │   ├── task.rs                  # datos crudos, siempre válidos
    │   ├── project_manager.rs       # validación, alta/baja/edición
    │   └── project_manager/
    │       └── manipulation_data.rs # funciones "sucias": parseo, cálculo de status, etc.
    │
    ├── persistence.rs               # capa SQLite (rusqlite)
    ├── persistence/
    │   ├── schema.rs                # creación/migración de tablas
    │   ├── project_repo.rs          # CRUD de proyectos contra la DB
    │   └── task_repo.rs             # CRUD de tareas contra la DB
    │
    ├── utilities.rs
    └── utilities/
        └── format.rs                # helpers genéricos (fechas, etc.)
```

---

## License

**GPL-3.0-only** — see [LICENSE](LICENSE).
