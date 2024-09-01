const { defineConfig } = require("eslint-define-config");

module.exports = defineConfig([
  {
    files: ["*.vue", "*.ts", "*.js"],
    rules: {
      // Custom rules for these files can be added here
    },
  },
]);
