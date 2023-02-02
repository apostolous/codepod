# CodePods Server

CodePods Server is the core of the application. It acts as the strategizer and orchestrator for containers.

It contains and serves:
- all configs for work environments (known as `configs`)
- all configs for how to orchestrate containers (known as `strategies`)
- a REST API to work with all of these things.

And most importantly, it *orchestrates your Pods*.