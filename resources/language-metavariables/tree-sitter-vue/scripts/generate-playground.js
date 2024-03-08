const generatePlayground = require("../tree-sitter/script/generate-playground");

generatePlayground("docs", {
  name: "Vue",
  example: `
<template>
  <p>
    Hello, <a :[key]="url">{{ name }}</a>!
  </p>
</template>

<script>
module.exports = {
  data: function () {
    return {
      name: 'World',
      key: 'href',
      url: 'https://example.com/'
    }
  }
}
</script>

<style scoped>
p {
  font-size: 2em;
  text-align: center;
}
</style>
`.trim()
});
