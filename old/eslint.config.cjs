const { defineConfig } = require("eslint-define-config");
const js = require('@eslint/js');
const eslintPluginVue = require('eslint-plugin-vue');

module.exports = defineConfig([
  js.configs.recommended,
  {
    files: ["**/*.vue", "**/*.ts", "**/*.js"],
    ignores: ["node_modules/**", "**/.nuxt/**", ".nuxt/dev/index.mjs"],

    languageOptions: {
      parser: require('vue-eslint-parser'),
      parserOptions: {
        ecmaVersion: 2022,
        sourceType: 'module',
        parser: '@typescript-eslint/parser',
      },
    },
    plugins: {
      vue: eslintPluginVue,
    },
    rules: {
      ...eslintPluginVue.configs['vue3-recommended'].rules,
      // Custom rules for these files can be added here
    },
  },
]);