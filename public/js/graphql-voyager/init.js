// Render <Voyager /> into the body.
document.addEventListener('DOMContentLoaded', (_event) => {
  document.querySelectorAll('div.graphql-voyager-wrapper').forEach(async (element) => {
    const endpoint = element.getAttribute('data-endpoint');
    const schema = element.getAttribute('data-schema');
    if (!endpoint && !schema) {
      element.innerHTML = 'Missing GraphQL endpoint URL or schema URL. Use either <code>data-endpoint="url"</code> <code>data-schema="url"</code>';
      return;
    }
    var introspection;
    if (schema != null) {
      console.log(`Loading schema from ${schema}`);
      const response = await fetch(schema);
      let sdl = await response.text();
      introspection = {
        "data": introspectionFromSchema(buildSchema(sdl))
      };

    } else if (endpoint != null) {
      console.log(`Loading schema by introspection of GraphQL endpoint ${endpoint}`);
      const {voyagerIntrospectionQuery: query} = GraphQLVoyager;

      const response = await fetch(
        endpoint,
        {
          method: 'post',
          headers: {
            Accept: 'application/json',
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({query}),
        },
      );
      introspection = await response.json();
    }
    const hideDocs = element.getAttribute('data-hide-docs') === "true";
    const hideSettings = element.getAttribute('data-hide-settings') === "true";
    GraphQLVoyager.renderVoyager(element, {
      introspection,
      hideDocs,
      hideSettings,
      hideVoyagerLogo: true,
    });
  });
});
