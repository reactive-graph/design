'use strict';

const path = require('path');
const fractal = module.exports = require('@frctl/fractal').create();

fractal.set('project.title', 'Reactive Graph - Design System');
fractal.set('project.version', '0.1.0');

fractal.components.set('path', path.join(__dirname, 'components'));
fractal.docs.set('path', path.join(__dirname, 'docs'));
fractal.web.set('static.path', path.join(__dirname, 'public'));
