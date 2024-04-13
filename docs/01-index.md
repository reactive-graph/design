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

{{#componentList}}
<a href="{{path '/components/preview/{{ this.handle }}' }}">{{ this.title }}</a>
{{/componentList}}


This is your index page. You can edit its contents at `docs/01-index.md`

## Test
{{ _self.title }}

## TODO

{{#each items}}
* {{ this }}
  {{/each}}

## Button component

The button component can be included within other components like this:

```
\{{> @button }}
```

This template for this component looks like this:

```
{{view '@button'}}
```

and it therefore expects a set of data to render it that is in the following format:

```
{{context '@button'}}
```

test 2
