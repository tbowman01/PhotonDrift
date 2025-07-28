use this command to resume development using claude-flow hive-mind

# sample workflow
npx claude-flow@alpha swarm create "Build complete e-commerce platform with user management, product catalog, and frontend" --agents 5 --strategy development --output ./output/ecommerce --monitor

# how do customize the claude.md workflow for your project

# most ucase cases are as simple as 
npx claude-flow@alpha hive-mind init    

# The Initialize Claude Flow v2.0.0 (creates CLAUDE.md & .claude/commands) - CLAUDE.md is the workflow that needs to be customized per project. 
# how? do more research and ask the model... 

npx claude-flow@alpha  hive-mind spawn "review and continue development in the next phase" --auto-spawn --claude --monitor --ui

uvx claude-monitor --plan max5 --timezone America/New_York