---
title: Reactive Graph - Design System
status: alpha
context:
  items:
    - Finish the docs
    - Write tests
    - Make the tea
---

## Component preview links:

<img src="{{ path '/logo/full/reactive-graph-full-mexican-pink.svg' }}" style="height: 240px;" alt="Reactive Graph">


This is your index page. You can edit its contents at `docs/01-index.md`

## Card component

The card component can be included within other components like this:

```
\{{> @card }}
```

This template for this component looks like this:

```
{{view '@card'}}
```

and it therefore expects a set of data to render it that is in the following format:

```
{{context '@card'}}
```
