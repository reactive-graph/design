---
title: Reactive Graph - Design System
status: alpha
---

This is a design system for Reactive Graph websites and applications. It establishes a common design language, provides reusable coded components, and outlines high level guidelines for content and accessibility.

{{ render '@intro' }}

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
