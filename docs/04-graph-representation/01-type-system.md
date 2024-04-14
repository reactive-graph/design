---
title: Type System
status: alpha
---

## Graphical Representation

We can use colors to represent the three fundamental types of the type system:

| Type          | Color                                                        | Tag                                                                                                                   | Token                                                                                                                     |
|---------------|--------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------|
| Component     | <code class="token rg-component">--chartreuse</code>         | <span class="tag rg-component">Component</span><br> `<span class="tag rg-component">Component</span>`                 | <span class="token rg-component">Component</span><br> `<span class="token rg-component">Component</span>`                 |
| Entity Type   | <code class="token rg-entity-type">--selective-yellow</code> | <span class="tag rg-entity-type">Entity Type</span><br> `<span class="tag rg-entity-type">Entity Type</span>`         | <span class="token rg-entity-type">Entity Type</span><br> `<span class="token rg-entity-type">Entity Type</span>`         |
| Relation Type | <code class="token rg-relation-type">--malachite</code>      | <span class="tag rg-relation-type">Relation Type</span><br> `<span class="tag rg-relation-type">Relation Type</span>` | <span class="token rg-relation-type">Relation Type</span><br> `<span class="token rg-relation-type">Relation Type</span>` |

For example, to represent that a User (which is a entity type) belongs to (which is a relation type) an organization
(which is also a entity type), we would use the colors like so:

<code style="color: var(--text-1); font-size: var(--font-size-5);">
  <span class="tag rg-entity-type">User</span>---<span class="tag rg-relation-type">belongs to</span>--&gt;<span class="tag rg-entity-type">Organization</span>
</code>
