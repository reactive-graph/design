---
title: Reactive Graph - Design System
status: alpha
---

<div>
  <img src="{{ path '/logo/configurable/reactive-graph.svg' }}" style="height: 240px; float: right;" alt="Reactive Graph">
</div>

<article class="rg-article">
  <h1 class="rg-article-title">About the design system</h1>
  <p class="rg-article-intro">
    This is a design system for Reactive Graph websites and applications. It establishes a common design language,
    provides reusable coded components, and outlines high level guidelines for content and accessibility.
  </p>
</article>

### Atomic Design

Atomic design is a methodology composed of five distinct stages working together to create interface design systems in a more deliberate and hierarchical
manner.

All components defined are categorized following the
<a href="http://atomicdesign.bradfrost.com/table-of-contents/">atomic design principles by Brad Frost</a>.

### Web Components

Web Components is a suite of different technologies allowing you to create reusable custom elements — with their
functionality encapsulated away from the rest of your code — and utilize them in your web apps.

<strong>Interoperability:</strong> The design system is reusable and work across different JavaScript frameworks or
vanilla JavaScript projects.

<strong>Different Tech Stacks:</strong> The design system need to be shared across projects with diverse tech
stacks.

<strong>Micro Frontends:</strong> The design system serves as a neutral interface between different parts of a
micro frontend architecture.

### Design System Fundamentals

The fundamentals define the <a href="/docs/fundamentals/palette">color palette</a>,
the <a href="/docs/fundamentals/fonts">fonts</a>, the <a href="/docs/fundamentals/logo">logo</a>,
the <a href="/docs/fundamentals/icons">icon sets</a>, the sets of <a href="/docs/fundamentals/pattern">patterns</a>
and <a href="/docs/fundamentals/gradients">gradients</a>.</p>

### Graph Representation

The section about the graph representation defines how the
<a href="/docs/graph-representation/type-system">type system</a> of the graph can be visualized. This means
how look components, entity types and relation types. Furthermore, it also defines the appearance of
<a href="/docs/graph-representation/instance-system">instances</a> (entity instances and relation instances).

### Flow Node Representation

The section about the flow node representation defines how the control flows can be visualized. This includes
the visualization of <a href="/docs/flow-node-representation/flow-node">flow nodes</a> and their property sockets
and the <a href="/docs/flow-node-representation/connector">connectors</a> which enables the data flow.
