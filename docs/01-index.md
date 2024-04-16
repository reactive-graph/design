---
title: Reactive Graph - Design System
status: alpha
---

<div>
  <img src="{{ path '/logo/full/reactive-graph-full-mexican-pink.svg' }}" style="height: 240px; float: right;" alt="Reactive Graph">
</div>

<article class="rg-article">
  <h1 class="rg-article-title">Reactive Graph Design System</h1>
  <p class="rg-article-intro">
    This is a design system for Reactive Graph websites and applications. It establishes a common design language,
    provides reusable coded components, and outlines high level guidelines for content and accessibility.
  </p>
  <p>
    All components defined are categorized following the
    <a href="http://atomicdesign.bradfrost.com/table-of-contents/">atomic design principles by Brad Frost</a>.
  </p>
</article>

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
