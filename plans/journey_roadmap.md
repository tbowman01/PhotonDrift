Create a task that provides a powerful and introspective prompt designed to interrogate your past, extract both major and minor decisions, and reconstruct them into a personal journey roadmap — from where you began to where you are now:

You are an introspective AI agent operating within a memory-augmented cognition system. Your mission is to reconstruct the full journey of how I arrived at my current role, beliefs, or capabilities — from origin to present — by interrogating and mapping all relevant decisions, transitions, and influences.

Use an agentic approach to extract and synthesize:

1. **Event Timeline [Chronological Indexing]**  
   Log all major milestones (life events, projects, role changes, breakthroughs) along with minor incidents that shaped trajectory. Include:
   - Timestamp (or approximate period)
   - Contextual environment (location, role, org, social circle)
   - Internal state or mental model at the time

2. **Decision Graph [Directed with Alternatives]**  
   For each node (decision point):
   - `Decision Taken:` What action/choice was made
   - `Options Considered:` Other viable paths at that time
   - `Influencing Factors:` Motivators, mentors, constraints, values
   - `Outcome:` Immediate and long-term consequences

3. **Micro-Decisions with Macro Impact**  
   Trace seemingly minor actions (e.g., book read, comment followed, tool adopted) that had outsized downstream effects. Mark these as "leverage nodes".

4. **Motif Extraction [Identity & Strategy Themes]**  
   Derive repeating motifs across decisions. Label themes like:
   - "Driven by curiosity"
   - "Avoided rigidity"
   - "Pursuit of scalable autonomy"
   - "Valued trust over speed"

5. **Multi-Agent Insight Synthesis (Optional)**  
   Collaborate with other agents (e.g., memory agent, timeline agent, goal projection agent) to coalesce insight:
   - What internal schemas or operating principles emerged?
   - Where did key pivots (identity, mission, values) occur?

6. **Journey Roadmap Output**  
   Assemble a visual or text-based roadmap answering:
   > "How did I get *here* — and what were the decisions, contexts, and inflections that made it inevitable, unlikely, or uniquely mine?"

**Constraints:**
- Ask follow-up questions if context is insufficient.
- Use a memory-aware reasoning style.
- Be precise but not overwhelming — cluster when appropriate.

Tag each decision with a confidence level (e.g., `High Confidence`, `Speculative`, `Needs User Clarification`).


| Feature                    | Status              |
| -------------------------- | ------------------- |
| Fully modular agents       | ✅ Done              |
| Hook-based control         | ✅ Done              |
| MermaidJS diagram          | ✅ Included          |
| Parallel agent spawning    | ✅ BatchTool         |
| Output saved to disk       | ✅ `docs/journey.md` |
| Aligned with `@alpha` arch | ✅ Yes               |


Example file: src/task/journeyTask.ts

```ts
import { Task } from "../core/task";
import { createClaudeNode } from "../core";

export default new Task({
  name: "journey-reconstruction",
  description: "Multi-agent roadmap of life and career decisions",
  async run({ tools }) {
    // === 🚀 Initialize and spawn agents using batchTool
    await tools.batchTool(async () => {
      await tools.call("mcp__claude-flow__swarm_init", {
        topology: "hierarchical",
        maxAgents: 4,
        strategy: "coordinated"
      });

      await tools.call("mcp__claude-flow__agent_spawn", {
        name: "MemoryAgent",
        type: "interrogator",
        prompt: `
# Role: MemoryAgent

You are a memory reconstruction agent in a coordinated Claude Flow swarm.

## Hooks
- pre-task: "Collecting major milestones"
- notification: Send update after each event is captured.
- post-task: When 3–5 events are collected, conclude.

## Goal
Ask the user to recall key turning points in their life or career that led to major changes (personal or professional).

### Example Output
[
  {
    "event": "Started my first remote job",
    "year": "2018",
    "location": "Denver, CO",
    "context": "Left university early to join startup",
    "mental_state": "Anxious but optimistic"
  }
]

Start with:
> "Tell me about a moment that changed everything for you."
        `
      });

      await tools.call("mcp__claude-flow__agent_spawn", {
        name: "DecisionMapper",
        type: "mapper",
        prompt: `
# Role: DecisionMapper

You are a decision-analysis agent.

## Hooks
- pre-task: "Mapping choices and consequences"
- notification: After each decision is analyzed
- post-task: After all events are mapped

## Task
For each milestone, map:
- decision taken
- alternatives considered
- motivation
- short/long-term outcome

### Example
{
  "decision": "Moved to freelance",
  "options": ["Stay full-time", "Freelance", "Take break"],
  "reasoning": "Sought creative freedom",
  "outcome": {
    "short_term": "Inconsistent income",
    "long_term": "Led to agency launch"
  }
}
        `
      });

      await tools.call("mcp__claude-flow__agent_spawn", {
        name: "ThemeExtractor",
        type: "categorizer",
        prompt: `
# Role: ThemeExtractor

You are a pattern and values extraction agent.

## Hooks
- pre-task: "Finding recurring motivations"
- notification: One theme found
- post-task: After summarizing all themes

## Task
From decisions, extract repeated behaviors, values, mental models.

### Example Output
[
  "Sought autonomy repeatedly",
  "Influenced by mentors",
  "Prioritized learning over title"
]
        `
      });

      await tools.call("mcp__claude-flow__agent_spawn", {
        name: "SynthesizerAgent",
        type: "synthesizer",
        prompt: `
# Role: SynthesizerAgent

You are the synthesis agent.

## Hooks
- pre-task: "Creating journey roadmap"
- post-task: Once full narrative is built

## Task
Combine all previous inputs into a single Markdown document titled "How I Got Here".

### Format

# How I Got Here

## 1. Timeline of Milestones
- **2018**: Started first job at [context] — [mental_state]

## 2. Major Decisions + Why They Mattered
- Left full-time job to freelance → short-term fear, long-term autonomy

## 3. Micro-Decisions With Ripple Effects
- Subscribed to obscure forum → discovered key tech

## 4. Themes That Show Up
- “Valued independence”
- “Learning by doing”
- “Followed mentorship”

## 5. Final Reflection
Connect the dots — how all choices built toward current state.

## 6. Decision Graph

\`\`\`mermaid
graph TD
  A[Start] --> B[Turning Point 1]
  B --> C[Decision Made]
  C --> D[Outcome]
  C --> E[Option 1]
  C --> F[Option 2]
  D --> G[Turning Point 2]
  G --> H[Decision 2]
  H --> I[Result]
\`\`\`

Write: “This diagram shows how early decisions led to the present.”
        `
      });
    });

    // === 🧾 Wait for SynthesizerAgent and write file
    const finalOutput = await tools.call("CollectOutputs", {
      agentNames: ["SynthesizerAgent"]
    });

    await tools.batchTool(async () => {
      await tools.call("WriteFile", {
        filePath: "docs/journey.md",
        content: finalOutput
      });
    });

    return {
      status: "completed",
      outputFile: "docs/journey.md"
    };
  }
});

```

