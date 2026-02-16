import { createEffect, createSignal, For, Show, onMount } from 'solid-js';
import * as markdown from 'binding';
import type { ParserOptions } from 'binding';
import JsonViewer from './json-viewer';
import HtmlViewer from './html-viewer';

const DEFAULT_INPUT = `---
title: Markdown Parser Playground
author: Demo User
date: 2024-01-15
tags: [markdown, parser, demo]
draft: false
---

# Markdown Parser Playground

Try editing this markdown text to see the AST output!

## Features

- **GitHub Flavored Markdown** (GFM)
- **Obsidian Flavored Markdown** (OFM)
- CJK text support
- Smart punctuation
- And more...

\`\`\`javascript
console.log('Hello, World!');
\`\`\`

| Feature | Supported |
|---------|-----------|
| Tables  | ✓         |
| Lists   | ✓         |

#tag [[wikilink]]
`;

interface OptionConfig {
  key: keyof ParserOptions;
  label: string;
  description?: string;
}

const OPTIONS: OptionConfig[] = [
  { key: 'github_flavored', label: 'GitHub Flavored', description: 'Enable GFM extensions' },
  { key: 'gfm_extended_autolink', label: 'GFM Autolink', description: 'Extended autolink support' },
  { key: 'obsidian_flavored', label: 'Obsidian Flavored', description: 'Enable OFM extensions' },
  { key: 'mdx_component', label: 'MDX Component', description: 'Support MDX components' },
  { key: 'cjk_autocorrect', label: 'CJK Autocorrect', description: 'Auto-correct CJK spacing' },
  { key: 'smart_punctuation', label: 'Smart Punctuation', description: 'Convert quotes and dashes' },
  { key: 'normalize_chinese_punctuation', label: 'Normalize Chinese Punct', description: 'Normalize Chinese punctuation' },
  { key: 'cjk_friendly_delimiters', label: 'CJK Friendly Delimiters', description: 'CJK-friendly emphasis delimiters' },
];

type ViewMode = 'ast' | 'html' | 'preview' | 'frontmatter';

function App() {
  const [inputSignal, setInputSignal] = createSignal(DEFAULT_INPUT);
  const [astSignal, setAstSignal] = createSignal<any>(null);
  const [htmlSignal, setHtmlSignal] = createSignal<string>('');
  const [frontmatterSignal, setFrontmatterSignal] = createSignal<any>(null);
  const [elapsedSignal, setElapsedSignal] = createSignal<number>();
  const [showOptions, setShowOptions] = createSignal(false);
  const [viewMode, setViewMode] = createSignal<ViewMode>('ast');
  const [options, setOptions] = createSignal<ParserOptions>({
    github_flavored: true,
    obsidian_flavored: true,
    cjk_autocorrect: true,
  });

  let inputTextarea: HTMLTextAreaElement | undefined;

  const toggleOption = (key: keyof ParserOptions) => {
    setOptions((prev) => ({ ...prev, [key]: !prev[key] }));
  };

  createEffect(() => {
    const input = inputSignal();
    const opts = options();
    const now = performance.now();
    const document = markdown.parse_with_options(input, opts);
    setElapsedSignal(Math.ceil((performance.now() - now) * 100) / 100);
    setAstSignal(document.tree);
    setHtmlSignal(document.to_html());
    const fm = document.frontmatter;
    // Convert Map to plain object for easier display
    if (fm instanceof Map) {
      const obj: Record<string, any> = {};
      fm.forEach((value: any, key: string) => {
        obj[key] = value;
      });
      setFrontmatterSignal(obj);
    } else {
      setFrontmatterSignal(fm);
    }
  });

  const handleNodeClick = (start?: any, end?: any) => {
    if (!inputTextarea || !start || !end) return;

    const input = inputSignal();
    const lines = input.split('\n');
    
    // Calculate character positions from line/column
    let startPos = 0;
    for (let i = 0; i < start.line - 1; i++) {
      startPos += lines[i].length + 1; // +1 for newline
    }
    startPos += start.column - 1;

    let endPos = 0;
    for (let i = 0; i < end.line - 1; i++) {
      endPos += lines[i].length + 1;
    }
    endPos += end.column - 1;

    // Select the text range
    inputTextarea.focus();
    inputTextarea.setSelectionRange(startPos, endPos);
    
    // Calculate scroll position to center the selection
    const textareaRect = inputTextarea.getBoundingClientRect();
    const lineHeight = parseInt(getComputedStyle(inputTextarea).lineHeight) || 20;
    const targetScrollTop = (start.line - 1) * lineHeight;
    const viewportHeight = inputTextarea.clientHeight;
    
    // Center the selection in the viewport
    const centeredScrollTop = targetScrollTop - (viewportHeight / 2) + (lineHeight / 2);
    
    // Smooth scroll to the position
    inputTextarea.scrollTo({
      top: Math.max(0, centeredScrollTop),
      behavior: 'smooth'
    });
  };

  return (
    <main class="flex h-full w-full flex-col bg-gray-50">
      {/* Header */}
      <header class="border-b border-gray-200 bg-white px-6 py-4 shadow-sm">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-2xl font-bold text-gray-900">Markdown Parser Playground</h1>
            <p class="text-sm text-gray-500">Test and explore markdown parsing with various options</p>
          </div>
          <button
            onClick={() => setShowOptions(!showOptions())}
            class="rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50"
          >
            {showOptions() ? 'Hide' : 'Show'} Options
          </button>
        </div>
      </header>

      {/* Options Panel */}
      <Show when={showOptions()}>
        <div class="border-b border-gray-200 bg-white px-6 py-4">
          <h2 class="mb-3 text-sm font-semibold text-gray-700">Parser Options</h2>
          <div class="grid grid-cols-2 gap-3 md:grid-cols-4">
            <For each={OPTIONS}>
              {(option) => (
                <label class="flex cursor-pointer items-start gap-2">
                  <input
                    type="checkbox"
                    checked={!!options()[option.key]}
                    onChange={() => toggleOption(option.key)}
                    class="mt-0.5 h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-2 focus:ring-blue-500"
                  />
                  <div class="flex-1">
                    <div class="text-sm font-medium text-gray-700">{option.label}</div>
                    {option.description && (
                      <div class="text-xs text-gray-500">{option.description}</div>
                    )}
                  </div>
                </label>
              )}
            </For>
          </div>
        </div>
      </Show>

      {/* Main Content */}
      <div class="flex flex-1 gap-4 overflow-hidden p-6">
        {/* Input Panel */}
        <div class="flex flex-1 flex-col">
          <div class="mb-2 flex items-center justify-between">
            <label for="input" class="text-sm font-semibold text-gray-700">
              Input
            </label>
            <button
              onClick={() => setInputSignal('')}
              class="rounded px-2 py-1 text-xs text-gray-500 hover:bg-gray-100"
            >
              Clear
            </button>
          </div>
          <textarea
            ref={inputTextarea}
            id="input"
            value={inputSignal()}
            on:input={(evt) => setInputSignal(evt.currentTarget.value)}
            class="flex-1 resize-none rounded-lg border border-gray-300 bg-white p-4 font-mono text-sm outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-200"
            placeholder="Enter markdown text here..."
          ></textarea>
        </div>

        {/* Output Panel */}
        <div class="flex flex-1 flex-col">
          <div class="mb-2 flex items-center justify-between">
            <div class="flex items-center gap-3">
              <span class="text-sm font-semibold text-gray-700">Output</span>
              <span class="rounded-full bg-blue-100 px-2.5 py-0.5 text-xs font-semibold text-blue-800">
                {elapsedSignal()}ms
              </span>
            </div>
            <button
              onClick={() => {
                const content = viewMode() === 'ast' ? JSON.stringify(astSignal(), null, 2) : htmlSignal();
                navigator.clipboard.writeText(content);
              }}
              class="rounded px-2 py-1 text-xs text-gray-500 hover:bg-gray-100"
            >
              Copy
            </button>
          </div>

          {/* Tabs */}
          <div class="mb-3 flex gap-1">
            <button
              onClick={() => setViewMode('ast')}
              class={`tab-button ${viewMode() === 'ast' ? 'tab-button-active' : ''}`}
            >
              <svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
              </svg>
              AST
            </button>
            <button
              onClick={() => setViewMode('frontmatter')}
              class={`tab-button ${viewMode() === 'frontmatter' ? 'tab-button-active' : ''}`}
            >
              <svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
              Frontmatter
            </button>
            <button
              onClick={() => setViewMode('html')}
              class={`tab-button ${viewMode() === 'html' ? 'tab-button-active' : ''}`}
            >
              <svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
              </svg>
              HTML
            </button>
            <button
              onClick={() => setViewMode('preview')}
              class={`tab-button ${viewMode() === 'preview' ? 'tab-button-active' : ''}`}
            >
              <svg class="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
              </svg>
              Preview
            </button>
          </div>

          <Show when={viewMode() === 'ast'}>
            <div class="json-viewer-container flex-1 overflow-auto rounded-lg border border-gray-300 bg-gray-900 p-4">
              <Show when={astSignal()}>
                <JsonViewer data={astSignal()} onNodeClick={handleNodeClick} />
              </Show>
            </div>
          </Show>

          <Show when={viewMode() === 'frontmatter'}>
            <div class="json-viewer-container flex-1 overflow-auto rounded-lg border border-gray-300 bg-gray-900 p-4">
              <Show
                when={frontmatterSignal() && typeof frontmatterSignal() === 'object' && Object.keys(frontmatterSignal()).length > 0}
                fallback={
                  <div class="flex h-full items-center justify-center text-gray-500">
                    <div class="text-center">
                      <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                      </svg>
                      <p class="mt-2 text-sm">No frontmatter found</p>
                      <p class="mt-1 text-xs text-gray-600">Add YAML frontmatter at the top of your markdown</p>
                    </div>
                  </div>
                }
              >
                <JsonViewer data={frontmatterSignal()} />
              </Show>
            </div>
          </Show>

          <Show when={viewMode() === 'html'}>
            <div class="flex-1 overflow-auto rounded-lg border border-gray-300 bg-gray-900">
              <HtmlViewer html={htmlSignal()} />
            </div>
          </Show>

          <Show when={viewMode() === 'preview'}>
            <div
              class="markdown-preview flex-1 overflow-auto rounded-lg border border-gray-300 bg-white p-6"
              innerHTML={htmlSignal()}
            ></div>
          </Show>
        </div>
      </div>
    </main>
  );
}

export default App;
