# Myman Learning Plan

Myman is the learning project for building a local-first desktop assistant. The learning flow is course-driven first, then verified through small project tasks inside the real codebase.

## Learning System

- Target project: `D:\fun\Myman`
- Learner profile: early beginner
- Time budget: 6-8 hours per week
- First milestone: understand the main project flow and complete small changes with tests
- Tooling: Codex acts as reading coach, question generator, reviewer, and recap partner

## Learning Documents

- [12-week roadmap](docs/learning/00-roadmap.md)
- [Weekly learning log](docs/learning/01-weekly-log.md)
- [Concept notebook](docs/learning/02-concepts.md)
- [Exercises and assessments](docs/learning/03-exercises.md)
- [Question backlog](docs/learning/04-questions.md)

## Weekly Workflow

1. Concept lesson, about 1.5 hours.
2. Code reading, about 1.5 hours.
3. Small exercise, about 2 hours.
4. Verification, about 1 hour.
5. Retrospective, about 30-60 minutes.

## Verification Commands

Run focused checks while learning:

```powershell
npm run test:run
cargo test
```

Run the full baseline before considering a change complete:

```powershell
npm run verify
npm run tauri:build:no-bundle
```

## Codex Reading Prompt

```text
Read this file with me as if I am an early beginner:
1. Explain the file's role in the project.
2. Explain the key types and functions.
3. Connect them with one real data flow.
4. Ask me 3 questions to check my understanding.
```

## Rule

Every study session should produce something visible: a note, an exercise result, a test, a command output, or a clear question.
