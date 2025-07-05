import { createEffect, createMemo, createSignal } from 'solid-js';
import * as markdown from 'binding';

function App() {
  const [inputSignal, setInputSignal] = createSignal('');
  const [astSignal, setAstSignal] = createSignal<string>('');
  const [elapsedSignal, setElapsedSignal] = createSignal<number>();
  createEffect(() => {
    const input = inputSignal();
    const now = performance.now();
    const document = markdown.parse(input);
    setElapsedSignal(Math.ceil((performance.now() - now) * 100) / 100);
    setAstSignal(JSON.stringify(document.tree, null, 2));
  });
  return (
    <main class="flex h-full w-full gap-2 bg-gray-50 p-10">
      <div class="w-50% h-full flex-1 flex-col gap-1">
        <label for="input" class="font-bold">
          Input
        </label>
        <textarea
          id="input"
          value={inputSignal()}
          on:input={(evt) => setInputSignal(evt.currentTarget.value)}
          class="h-full w-full resize-none border border-gray-200 bg-white p-2 outline-0 focus:border-gray-400"
        ></textarea>
      </div>
      <div class="w-50% h-full flex-1 flex-col gap-1">
        <div>
          <span class="font-bold">Output</span>
          <span class="ml-1 text-sm text-gray-400">{elapsedSignal()}ms</span>
        </div>
        <textarea readonly class="h-full w-full  border border-gray-200 p-2">
          {astSignal()}
        </textarea>
      </div>
    </main>
  );
}

export default App;
