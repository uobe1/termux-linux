Foremost Rule ↓
# > 用户高于一切 <
Main Rule ↑
<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->
<!-- SPEC2025:START -->
# Spec 2025
## RuleF
- **MUST**使用中文交流沟通
  - **禁止**英文交流沟通
- **禁止**使用 Agent 和 Task 工具
- **尽量不要**使用Shell命令进行文件读写
  - **可以**使用tool代替Shell命令，Tool列表如下:
    ```List
    - Edit
    - exit_plan_mode
    - glob
    - list_directory
    - multi_edit
    - plan
    - read plan
    - read_file
    - read_many_files
    - save_memory
    - Search # 无法使用则手动使用ripgrep搜索
    - Shell # 尽量不要用它进行文件读写
    - task # 禁止使用
    - web_fetch
    - web_search
    - write_file
    - xml_escape
    ```

## OpenSpecB
- **MUST**按顺序开发
  - **禁止**乱序开发
  - **禁止**跳序开发
- **ONLY**开发对应任务时查验对应代码实现状态
  - **禁止**未开发任务时查验代码实现状态
  - **禁止**开发任务时查验非与其对应的代码实现状态
- 当完成单个任务(如: 1.1 2.3 4.1)的开发后:
  **MUST**更新 `task.md` 以反映修改
    - **禁止**不更新 `task.md` 直接提交代码
  **MUST**使用 `git add . && git commit -m "fact: " -m "other..." ` 提交修改，如果提交到达上限则使用 `git push`
    - **禁止**提交代码后不向用户报告当前工作
  **MUST**暂停开发，向用户报告当前工作(已完成步骤内容、完成的步骤和正在开发的步骤)并征求**用户的**下一步意见
      - **禁止**不向用户征求下一步意见就继续开发

> Keep this managed block <
<!-- SPEC2025:END -->

<!-- IFLOW.md -->

<!-- SPEC2025:START -->
# Spec 2025
## RuleF
- **MUST**使用中文交流沟通
  - **禁止**英文交流沟通
- **禁止**使用 Agent 和 Task 工具
- **尽量不要**使用Shell命令进行文件读写
  - **可以**使用tool代替Shell命令，Tool列表如下:
    ```List
    - Edit
    - exit_plan_mode
    - glob
    - list_directory
    - multi_edit
    - plan
    - read plan
    - read_file
    - read_many_files
    - save_memory
    - Search # 无法使用则手动使用ripgrep搜索
    - Shell # 尽量不要用它进行文件读写
    - task # 禁止使用
    - web_fetch
    - web_search
    - write_file
    - xml_escape
    ```

## OpenSpecB
- **MUST**按顺序开发
  - **禁止**乱序开发
  - **禁止**跳序开发
- **ONLY**开发对应任务时查验对应代码实现状态
  - **禁止**未开发任务时查验代码实现状态
  - **禁止**开发任务时查验非与其对应的代码实现状态
- 当完成单个任务(如: 1.1 2.3 4.1)的开发后:
  **MUST**更新 `task.md` 以反映修改
    - **禁止**不更新 `task.md` 直接提交代码
  **MUST**使用 `git add . && git commit -m "fact: " -m "other..." ` 提交修改，如果提交到达上限则使用 `git push`
    - **禁止**提交代码后不向用户报告当前工作
  **MUST**暂停开发，向用户报告当前工作(已完成步骤内容、完成的步骤和正在开发的步骤)并征求**用户的**下一步意见
      - **禁止**不向用户征求下一步意见就继续开发

> Keep this managed block <
<!-- SPEC2025:END -->