---
title: Instance System
status: alpha
---

## Graphical Representation

We can use colors to represent the three fundamental types of the instance system:

| Type              | Color                                                           | Tag                                                                                                                   | Token                                                                                                                                     |
|-------------------|-----------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------|
| Entity Instance   | <code class="token rg-entity-instance">--celestial-blue</code>  | <span class="tag rg-entity-instance">Entity Instance</span><br> `<span class="tag rg-component">Component</span>`     | <span class="token rg-entity-instance">Entity Instance</span><br> `<span class="token rg-entity-instance">Entity Instance</span>`         |
| Relation Instance | <code class="token rg-relation-instance">--mexican-pink</code>  | <span class="tag rg-relation-instance">Relation Instance</span><br> `<span class="tag rg-component">Component</span>` | <span class="token rg-relation-instance">Relation Instance</span><br> `<span class="token rg-relation-instance">Relation Instance</span>` |

For example, to represent that the User 'Hanack' (which is an entity instance) belongs to (which is a relation instance) the
organization Inexor (which is also an entity instance), we would use the colors like so:

<code style="color: var(--text-1); font-size: var(--font-size-5);">
  <span class="tag rg-entity-instance">Hanack</span>---<span class="tag rg-relation-instance">belongs to</span>--&gt;<span class="tag rg-entity-instance">Inexor</span>
</code>

### Instance System Colors vs. Type System Colors

The colors don't infer with the colors of the type system. It's possible to use all of these colors:

<div style="display: grid; grid-template-columns: auto auto auto 1fr;">
  <code class="tag string">
    <code class="tag rg-entity-type">User</code>
    <br>
    <code class="tag rg-entity-instance">Hanack</code>  
  </code>
  <code class="tag string">
    <code class="tag rg-relation-type">---belongs to--&gt;</code>
    <br>
    <code class="tag rg-relation-instance">---belongs to--&gt;</code>
  </code>
  <code class="tag string">
    <code class="tag rg-entity-type">Organization</code>
    <br>
    <code class="tag rg-entity-instance">Inexor</code>
  </code>
</div>
