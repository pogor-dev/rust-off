import js from '@eslint/js';
import tseslint from 'typescript-eslint';
import stylistic from '@stylistic/eslint-plugin'
import solid from 'eslint-plugin-solid';
import * as tsParser from '@typescript-eslint/parser'
import sonarjs from "eslint-plugin-sonarjs"

export default tseslint.config(
  js.configs.recommended,
  tseslint.configs.strictTypeChecked,
  stylistic.configs['recommended-flat'],
  solid.configs['flat/typescript'],
  sonarjs.configs.recommended,
  // TypeScript rules
  {
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
        project: './tsconfig.json',
      },
    },
  },
  // ESLint JS rules
  {
    plugins: {
      '@eslint': js
    },
    rules: {
      'sort-imports': ['error', {
        'ignoreCase': false,
        'ignoreDeclarationSort': false,
        'ignoreMemberSort': false,
        'memberSyntaxSortOrder': ['none', 'all', 'multiple', 'single'],
        'allowSeparatedGroups': false
      }]
    }
  },
  //Stylistic rules
  {
    plugins: {
      '@stylistic': stylistic
    },
    rules: {
      '@stylistic/jsx-one-expression-per-line': ['error', { 'allow': 'single-line' }],
    },
  },
)