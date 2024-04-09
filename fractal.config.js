'use strict';

const path = require('path');
const fractal = module.exports = require('@frctl/fractal').create();

fractal.set('project.title', 'Reactive Graph - Design System');
fractal.set('project.version', '0.1.0');

fractal.components.set('path', path.join(__dirname, 'components'));
fractal.components.set('ext', '.hbs');
fractal.components.set('default.status', 'wip');
fractal.components.set('default.preview', '@preview');

fractal.docs.set('path', path.join(__dirname, 'docs'));
fractal.docs.set('ext', '.md');
fractal.docs.set('default.status', 'draft');
const hbs = require('@frctl/handlebars')({
  helpers: {
    componentList: function() {
      let ret = "<ul>";
      const options = Array.from(arguments).pop();
      for (let component of fractal.components.flatten()) {
        ret = ret + "<li>" + options.fn(component.toJSON()) + "</li>";
      }
      return ret + "</ul>";
    }
  }
});
fractal.docs.engine(hbs);

fractal.web.set('static.path', path.join(__dirname, 'public'));
fractal.web.set('builder.dest', path.join(__dirname, 'build'));
