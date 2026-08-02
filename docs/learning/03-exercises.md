# Exercises and Assessments

Exercises should be small enough to finish in one study session.

## Rules

- One exercise should teach one thing.
- If code changes, add or update a test when practical.
- Run the smallest useful command after each exercise.
- Record the result in `01-weekly-log.md`.

## Week 1 Exercises

1. Explain `npm run verify`.
   - Output: short note.
   - Check: you can list every command it runs.

2. Draw the module map.
   - Output: text diagram or bullet list.
   - Check: include frontend, Tauri commands, Rust backend, SQLite, tests.

## Week 2 Exercises

1. Explain `Entity`.
   - Output: field-by-field note.
   - Check: identify which fields are optional.

2. Compare create and update request types.
   - Output: short comparison.
   - Check: explain why create has `entityType` and update has `id`.

## Week 3 Exercises

1. Trace create flow.
   - Output: numbered steps from form submit to refresh.
   - Check: include `Composer`, `App`, API layer.

2. Explain component ownership.
   - Output: table of state and owner.
   - Check: identify which component owns `query`, `editing`, and form body.

## Week 4 Exercises

1. Read a test aloud.
   - Output: line-by-line explanation.
   - Check: explain setup, action, assertion.

2. Add a UI test.
   - Output: one small test.
   - Check: `npm run test:run` passes.

## Week 5 Exercises

1. Explain Rust `EntityType`.
   - Output: note on enum variants and serialization.
   - Check: explain invalid type behavior.

2. Explain one Rust function signature.
   - Output: function name, input, output, possible error.
   - Check: no unexplained type remains.

## Week 6 Exercises

1. Explain schema tables.
   - Output: one sentence per major table.
   - Check: include `entities`, `entity_contents`, `tags`, `entity_tags`, `search_index`.

2. Explain one query.
   - Output: SQL walkthrough.
   - Check: identify joins, filters, grouping, order, limit.

## Week 7 Exercises

1. Trace `searchEntities`.
   - Output: full data path.
   - Check: include React call, Tauri command, Rust request, SQL query.

2. Explain camelCase mapping.
   - Output: short note.
   - Check: explain frontend `entityType` to Rust `entity_type`.

## Week 8 Exercises

1. Explain transaction use.
   - Output: note on why create/update need all-or-nothing writes.
   - Check: mention tags and search index.

2. Add or inspect one Rust test.
   - Output: test explanation or test change.
   - Check: `cargo test` passes.

## Week 9 Exercises

1. Explain AND vs OR search.
   - Output: example queries and results.
   - Check: connect frontend mode to Rust `SearchMode`.

2. Explain Chinese search limitation.
   - Output: short note.
   - Check: mention tokenization.

## Week 10 Exercises

1. Design a fake migration.
   - Output: migration idea and rollback risk.
   - Check: explain old database and new database behavior.

2. Write migration test plan.
   - Output: test cases only.
   - Check: include empty DB and legacy DB.

## Week 11 Exercises

Pick one low-risk change:

- Improve error text.
- Add filter summary.
- Improve empty state.
- Add a missing test.

Check:

- The change is small.
- A test is added or updated where practical.
- `npm run verify` passes.

## Week 12 Assessment

Final tasks:

1. Explain `Composer` submit to SQLite write.
2. Explain search input to FTS query.
3. Modify or add one small test.
4. Run `npm run verify`.
5. Write the next learning plan.
