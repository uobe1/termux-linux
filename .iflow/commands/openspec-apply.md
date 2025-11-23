---
name: /openspec-apply
id: openspec-apply
category: OpenSpec
description: Implement an approved OpenSpec change and keep tasks in sync. [提案开发]
---
<!-- OPENSPEC:START -->
**Guardrails**
- Favor straightforward, minimal implementations first and add complexity only when it is requested or clearly required.
- Keep changes tightly scoped to the requested outcome.
- Refer to `openspec/AGENTS.md` (located inside the `openspec/` directory—run `ls openspec` or `openspec update` if you don't see it) if you need additional OpenSpec conventions or clarifications.

**Steps**
Track these steps as TODOs and complete them one by one.
1. 1. 阅读 changes/<id>/proposal.md、design.md（如存在）和 tasks.md 文件，以确认工作范围、验收标准和当前开发进度
2. Work(Develop) through tasks sequentially, keeping edits minimal and focused on the requested change.
  * !{openspec list} is active proposal, and you need to develop it
3. Confirm completion before updating statuses—make sure every item in `tasks.md` is finished.
4. 在完成一项任务后更新清单，确保完成的任务任务标记为 `- [x]` 并向用户反映实际开发情况
5. Reference `openspec list` or `openspec show <item>` when additional context is required.

**Reference**
- Use `openspec show <id> --json --deltas-only` if you need additional context from the proposal while implementing.
<!-- OPENSPEC:END -->
