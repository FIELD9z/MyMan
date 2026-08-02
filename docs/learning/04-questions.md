# Question Backlog

Use this file for unclear points. Each question should be concrete enough to answer.

## How To Ask

Bad:

- I do not understand Rust.

Better:

- In `update_entity`, why does the function return `Result<Entity, String>` instead of just `Entity`?

## Open Questions

### Project Structure

- 

### TypeScript / React

- 

### Testing

- 

### Rust

- 

### SQLite

- 

### Tauri

- 

## Answered Questions

Move questions here after they are answered.

### PowerShell Encoding

Question:

- Why does Chinese text appear garbled when reading Myman files with `Get-Content`?

Answer:

- PowerShell may decode the UTF-8 file using a different default encoding. Specify UTF-8 explicitly with `Get-Content -Encoding UTF8 <path>`.

Evidence:

- `Get-Content -Encoding UTF8 .\src\components\Composer.test.tsx` displayed the Chinese text correctly.

### Example

Question:

- Why does `Composer` call `onSave` instead of importing the API directly?

Answer:

- It keeps the form reusable and testable. `Composer` owns form state, while `App` owns persistence and refresh behavior.

Evidence:

- `src/components/Composer.tsx`
- `src/App.tsx`

## Codex Question Prompt

```text
I am following the Myman learning roadmap. Please answer this question:

Question:
[write the specific question]

Use this format:
1. Direct answer
2. Related Myman file or function
3. One small example
4. One question to check my understanding
```
