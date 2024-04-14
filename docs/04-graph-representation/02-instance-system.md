---
title: Instance System
status: alpha
---

## Graphical Representation

We can use colors to represent the three fundamental types of the instance system:

| Type              | Color                                                                                                |
|-------------------|------------------------------------------------------------------------------------------------------|
| Entity Instance   | <code style="color: var(--text-1--dark); background: var(--celestial-blue);">--celestial-blue</code> |
| Relation Instance | <code style="color: var(--text-1--light); background: var(--mexican-pink);">--mexican-pink</code>    |

For example, to represent that the User 'Hanack' (which is an entity instance) belongs to (which is a relation instance) the
organization Inexor (which is also an entity instance), we would use the colors like so:

<code style="color: var(--text-1);">
  <span style="color: var(--text-1--dark); background: var(--celestial-blue);">Hanack</span>  
  ---<span style="color: var(--text-1--light); background: var(--mexican-pink);">belongs to</span>--&gt;
  <span style="color: var(--text-1--dark); background: var(--celestial-blue);">Inexor</span>
</code>

### Instance System Colors vs. Type System Colors

The colors don't infer with the colors of the type system. It's possible to use all of these colors:

<div style="display: grid; grid-template-columns: 1fr 1fr 1fr;">
  <code>
    <div style="color: var(--text-1--dark); background: var(--selective-yellow);">User</div>
    <div style="color: var(--text-1--dark); background: var(--celestial-blue);">Hanack</div>  
  </code>
  <code>
    <div style="color: var(--text-1--dark); background: var(--malachite);">---belongs to--&gt;</div>
    <div style="color: var(--text-1--light); background: var(--mexican-pink);">---belongs to--&gt;</div>
  </code>
  <code>
    <div style="color: var(--text-1--dark); background: var(--selective-yellow);">Organization</div>
    <div style="color: var(--text-1--dark); background: var(--celestial-blue);">Inexor</div>
  </code>
</div>
