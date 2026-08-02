# 12-Week Roadmap

This roadmap is course-driven. Each week starts with concepts, then uses Myman as the lab.

## Success Criteria

By the end of week 12, you should be able to:

- Explain the path from `Composer` submit to SQLite write.
- Explain how frontend request objects reach Rust Tauri commands.
- Read and modify a small React or Rust module without guessing.
- Add or adjust a small test.
- Run `npm run verify` and interpret failures.
- Decide whether a change needs a database migration.

## Weekly Template

Each week follows the same rhythm:

1. Concept lesson: learn the topic away from code.
2. Code reading: inspect the Myman modules that use the topic.
3. Exercise: complete one small, verifiable task.
4. Verification: run the smallest useful command, then the full baseline if code changed.
5. Retrospective: write what you understand and what remains unclear.

## Week 1 - Environment, CLI, Git, Project Shape

Goal: know what Vite, React, Tauri, Rust, SQLite, and tests do in this project.

Read:

- `package.json`
- `README.md`
- `src/main.tsx`
- `src-tauri/src/main.rs`
- `src-tauri/src/lib.rs`

Practice:

- Draw the module map in your own words.
- Explain what `npm run verify` runs and why each step exists.

Done when:

- You can explain the difference between frontend, Tauri bridge, Rust backend, and SQLite.
- You can run a verification command and understand the rough meaning of the output.

## Week 2 - TypeScript Types and Frontend Contracts

Goal: understand `Entity`, `EntityType`, request objects, and why type boundaries matter.

Read:

- `src/types.ts`
- `src/lib/entityTypes.ts`
- `src/lib/entities.ts`

Practice:

- Write a one-page explanation of the data shape used by one entity.
- Explain why `listEntities` and `searchEntities` use request objects.

Done when:

- You can identify which fields come from the database and which are UI-only state.

## Week 3 - React Components and State

Goal: understand `App`, `Composer`, and `EntityCard` responsibilities.

Read:

- `src/App.tsx`
- `src/components/Composer.tsx`
- `src/components/EntityCard.tsx`

Practice:

- Trace one create flow from typing in the form to refreshing the entity list.

Done when:

- You can explain what state lives in `App` and what state lives in `Composer`.

## Week 4 - Frontend Tests

Goal: understand Vitest, Testing Library, user events, and API mocking.

Read:

- `vitest.config.ts`
- `src/test/setup.ts`
- `src/App.test.tsx`
- `src/components/Composer.test.tsx`
- `src/components/EntityCard.test.tsx`

Practice:

- Add or rewrite one UI test in red-green style: make it fail first, then pass.

Done when:

- You can explain why App tests mock the API layer instead of calling Tauri.

## Week 5 - Rust Basics

Goal: understand enum, struct, module, `Result`, and `Option` in project context.

Read:

- `src-tauri/src/models.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/commands.rs`

Practice:

- Explain how Rust `EntityType` protects the project from invalid entity types.

Done when:

- You can read a Rust function signature and identify input, output, and failure type.

## Week 6 - SQLite and Data Modeling

Goal: understand shared entity modeling, tags, contents, links, and FTS table purpose.

Read:

- `src-tauri/migrations/0001_initial.sql`
- `src-tauri/src/entities.rs`

Practice:

- Pick one query in `entities.rs` and explain joins, filters, ordering, and indexes.

Done when:

- You can explain why notes, tasks, events, knowledge entries, and files share `entities`.

## Week 7 - Tauri Frontend-to-Rust Communication

Goal: understand `invoke`, command handlers, and request/response serialization.

Read:

- `src/lib/entities.ts`
- `src-tauri/src/commands.rs`
- `src-tauri/src/models.rs`

Practice:

- Trace `searchEntities` from React call to Rust command to SQLite query.

Done when:

- You can explain command names, request objects, and serde camelCase conversion.

## Week 8 - CRUD and Transactions

Goal: understand create, update, archive, transactions, tag replacement, and search index refresh.

Read:

- CRUD functions in `src-tauri/src/entities.rs`
- Rust tests at the bottom of `entities.rs`

Practice:

- Add a Rust behavior test for one edge case before changing behavior.

Done when:

- You can explain why create/update use transactions.

## Week 9 - Search System

Goal: understand SQLite FTS5, AND/OR search, tag/type filters, ranking, and Chinese search limitations.

Read:

- `search_entities` in `src-tauri/src/entities.rs`
- search tests in `entities.rs`
- frontend search flow in `src/App.tsx`

Practice:

- Write a search scenario test or explain one existing test line by line.

Done when:

- You can explain why Chinese search needs special design later.

## Week 10 - Migrations and Compatibility

Goal: understand `schema_migrations` and why persistent data changes need care.

Read:

- `src-tauri/src/db.rs`
- `src-tauri/migrations/0001_initial.sql`

Practice:

- Design a fake migration for adding a task status field. Do not implement it yet.
- Write what tests would prove it works for old and new databases.

Done when:

- You can decide whether a change is code-only or database-shape-changing.

## Week 11 - Independent Small Change

Goal: complete one low-risk change with a test.

Choose one:

- Improve backend error messages.
- Add a visible filter summary in the UI.
- Improve empty-state copy.
- Add one missing test for an existing behavior.

Done when:

- The change is small.
- A test was added or updated.
- `npm run verify` passes.

## Week 12 - Final Review and Small Assessment

Goal: explain the architecture and complete a small verified change.

Assessment:

- Explain frontend, Tauri, Rust, SQLite, and tests in one coherent flow.
- Trace `Composer` submit to SQLite write.
- Trace search from input to FTS query.
- Modify or add one small test.
- Run `npm run verify`.

Done when:

- You can describe what to learn next without needing the whole project re-explained.
