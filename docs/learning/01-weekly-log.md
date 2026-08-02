# Weekly Learning Log

Use this file as the running record. Add one section per study session or per week.

## How To Use

- Keep entries short and concrete.
- Record commands exactly.
- Write unclear points as questions, not vague feelings.
- Prefer "I can explain X" over "I watched/read X".

## Template

```markdown
## Week N - Topic

- Date:
- Time spent:
- Goal:
- Files read:
- Concepts studied:
- Exercise completed:
- Commands run:
- Result:
- What I can explain now:
- What is still unclear:
- Next small task:
```

## Week 0 - Beginner Bridge: CLI, Project Shape, and Entrypoints

- Date: 2026-07-02
- Time spent: Not recorded
- Goal: Build enough command-line and project-structure knowledge to follow Week 1 without assuming JavaScript, React, Tauri, or Rust experience.
- Files read: `package.json`, `src/main.tsx`, `src/components/Composer.test.tsx`, `src-tauri/tauri.conf.json`, `src-tauri/src/main.rs`, `src-tauri/src/lib.rs`
- Concepts studied: Current directory and relative paths; files versus folders; npm script mapping; frontend tests versus builds; frontend-only versus full Tauri startup; frontend and Rust entrypoints; basic test assertion reading.
- Exercise completed: Drew and correctly completed the startup path from `npm run tauri:dev` through the frontend and Rust entrypoints; interpreted one real `Composer` test assertion.
- Commands run: `Get-Location`; `Get-ChildItem`; `Get-Content -Encoding UTF8 .\src\components\Composer.test.tsx`; `npm run test:run`; `npm run build`; `Select-String -Path .\src-tauri\src\lib.rs -Pattern "pub fn run","setup","invoke_handler"`
- Result: Frontend tests passed (`3` files, `6` tests); frontend production build succeeded and generated `dist` output.
- What I can explain now: How npm script names map to underlying commands; where frontend and Rust code live; the roles of `src/main.tsx`, `src-tauri/src/main.rs`, and `src-tauri/src/lib.rs`; the difference between testing, building, frontend development, and full Tauri development.
- What is still unclear: No unresolved question recorded. TypeScript test syntax and Rust syntax have only been introduced at a basic reading level.
- Next small task: Start Week 1 by running `npm run tauri:dev` and identify which output comes from the frontend and which comes from Tauri/Rust.

## Week 1 - Environment, CLI, Git, Project Shape

- Date:
- Time spent:
- Goal:
- Files read:
- Concepts studied:
- Exercise completed:
- Commands run:
- Result:
- What I can explain now:
- What is still unclear:
- Next small task:

## Week 2 - TypeScript Types and Frontend Contracts

- Date:
- Time spent:
- Goal:
- Files read:
- Concepts studied:
- Exercise completed:
- Commands run:
- Result:
- What I can explain now:
- What is still unclear:
- Next small task:

## Week 3 - React Components and State

- Date:
- Time spent:
- Goal:
- Files read:
- Concepts studied:
- Exercise completed:
- Commands run:
- Result:
- What I can explain now:
- What is still unclear:
- Next small task:

## Week 4 - Frontend Tests

- Date:
- Time spent:
- Goal:
- Files read:
- Concepts studied:
- Exercise completed:
- Commands run:
- Result:
- What I can explain now:
- What is still unclear:
- Next small task:

## Week 5 - Rust Basics

- Date:
- Time spent:
- Goal:
- Files read:
- Concepts studied:
- Exercise completed:
- Commands run:
- Result:
- What I can explain now:
- What is still unclear:
- Next small task:

## Week 6 - SQLite and Data Modeling

- Date:
- Time spent:
- Goal:
- Files read:
- Concepts studied:
- Exercise completed:
- Commands run:
- Result:
- What I can explain now:
- What is still unclear:
- Next small task:

## Week 7 - Tauri Frontend-to-Rust Communication

- Date:
- Time spent:
- Goal:
- Files read:
- Concepts studied:
- Exercise completed:
- Commands run:
- Result:
- What I can explain now:
- What is still unclear:
- Next small task:

## Week 8 - CRUD and Transactions

- Date:
- Time spent:
- Goal:
- Files read:
- Concepts studied:
- Exercise completed:
- Commands run:
- Result:
- What I can explain now:
- What is still unclear:
- Next small task:

## Week 9 - Search System

- Date:
- Time spent:
- Goal:
- Files read:
- Concepts studied:
- Exercise completed:
- Commands run:
- Result:
- What I can explain now:
- What is still unclear:
- Next small task:

## Week 10 - Migrations and Compatibility

- Date:
- Time spent:
- Goal:
- Files read:
- Concepts studied:
- Exercise completed:
- Commands run:
- Result:
- What I can explain now:
- What is still unclear:
- Next small task:

## Week 11 - Independent Small Change

- Date:
- Time spent:
- Goal:
- Files read:
- Concepts studied:
- Exercise completed:
- Commands run:
- Result:
- What I can explain now:
- What is still unclear:
- Next small task:

## Week 12 - Final Review and Small Assessment

- Date:
- Time spent:
- Goal:
- Files read:
- Concepts studied:
- Exercise completed:
- Commands run:
- Result:
- What I can explain now:
- What is still unclear:
- Next small task:
