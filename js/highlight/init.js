document.addEventListener('DOMContentLoaded', (event) => {
  document.querySelectorAll('pre.code-block').forEach((block) => {
    hljs.addPlugin(new CopyButtonPlugin({
      autohide: false, // Always show the copy button
    }));
    hljs.highlightElement(block);
  });
});
