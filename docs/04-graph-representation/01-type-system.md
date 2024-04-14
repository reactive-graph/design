---
title: Type System
status: alpha
---

## Graphical Representation

We can use colors to represent the three fundamental types of the type system:

| Type          | Color                                                                                                    |
|---------------|----------------------------------------------------------------------------------------------------------|
| Component     | <code style="color: var(--text-1--dark); background: var(--chartreuse);">--chartreuse</code>             |
| Entity Type   | <code style="color: var(--text-1--dark); background: var(--selective-yellow);">--selective-yellow</code> |
| Relation Type | <code style="color: var(--text-1--dark); background: var(--malachite);">--malachite</code>               |

For example, to represent that a User (which is a entity type) belongs to (which is a relation type) an organization
(which is also a entity type), we would use the colors like so:

<code style="color: var(--text-1);">
  <span style="color: var(--text-1--dark); background: var(--selective-yellow);">User</span>  
  ---<span style="color: var(--text-1--dark); background: var(--malachite);">belongs to</span>--&gt;
  <span style="color: var(--text-1--dark); background: var(--selective-yellow);">Organization</span>
</code>
