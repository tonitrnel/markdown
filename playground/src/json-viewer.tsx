import { createSignal, For, Show, createMemo } from 'solid-js';

interface JsonViewerProps {
  data: any;
  name?: string;
  depth?: number;
  isLast?: boolean;
  onNodeClick?: (start?: any, end?: any) => void;
  arrayIndex?: number;
}

function JsonViewer(props: JsonViewerProps) {
  const [collapsed, setCollapsed] = createSignal(props.depth ? props.depth > 1 : false);
  const depth = props.depth || 0;

  const isObject = (val: any) => val !== null && typeof val === 'object' && !Array.isArray(val);
  const isArray = (val: any) => Array.isArray(val);
  const isPrimitive = (val: any) => !isObject(val) && !isArray(val);

  const getValueClass = (val: any) => {
    if (val === null) return 'json-null';
    if (val === undefined) return 'json-undefined';
    if (typeof val === 'string') return 'json-string';
    if (typeof val === 'number') return 'json-number';
    if (typeof val === 'boolean') return 'json-boolean';
    return '';
  };

  const formatValue = (val: any) => {
    if (val === null) return 'null';
    if (val === undefined) return 'undefined';
    if (typeof val === 'string') return `"${val}"`;
    return String(val);
  };

  const getPreview = (val: any) => {
    if (isArray(val)) {
      return val.length === 0 ? '' : `${val.length} items`;
    }
    if (isObject(val)) {
      const keys = Object.keys(val);
      return keys.length === 0 ? '' : `${keys.length} keys`;
    }
    return '';
  };

  // Get label for array items (show kind if available)
  const getArrayItemLabel = () => {
    if (props.arrayIndex !== undefined && isObject(props.data) && props.data.kind) {
      return props.data.kind;
    }
    return undefined;
  };

  const entries = createMemo(() => {
    if (isObject(props.data)) {
      return Object.entries(props.data);
    }
    if (isArray(props.data)) {
      return props.data.map((item: any, idx: number) => [idx, item]);
    }
    return [];
  });

  const handleClick = (e: MouseEvent) => {
    e.stopPropagation();
    if (props.onNodeClick && isObject(props.data)) {
      props.onNodeClick(props.data.start, props.data.end);
    }
  };

  const displayName = props.name || getArrayItemLabel();

  return (
    <>
      <Show when={isObject(props.data) || isArray(props.data)}>
        <div class="json-line" onClick={handleClick}>
          <button
            class="json-toggle"
            onClick={(e) => {
              e.stopPropagation();
              setCollapsed(!collapsed());
            }}
            title={collapsed() ? 'Expand' : 'Collapse'}
          >
            <svg
              class={`json-arrow ${collapsed() ? 'json-arrow-collapsed' : 'json-arrow-expanded'}`}
              width="12"
              height="12"
              viewBox="0 0 12 12"
            >
              <path d="M4 2 L8 6 L4 10" fill="none" stroke="currentColor" stroke-width="1.5" />
            </svg>
          </button>
          <Show when={displayName !== undefined}>
            <Show
              when={props.arrayIndex !== undefined}
              fallback={
                <>
                  <span class="json-key">"{displayName}"</span>
                  <span class="json-colon">: </span>
                </>
              }
            >
              <span class="json-array-label">{displayName}</span>
              <span class="json-colon"> </span>
            </Show>
          </Show>
          <span class="json-bracket">{isArray(props.data) ? '[' : '{'}</span>
          <Show when={collapsed()}>
            <span class="json-preview"> {getPreview(props.data)} </span>
            <span class="json-bracket">{isArray(props.data) ? ']' : '}'}</span>
          </Show>
          <Show when={!props.isLast && collapsed()}>
            <span class="json-comma">,</span>
          </Show>
        </div>

        <Show when={!collapsed()}>
          <div class="json-children">
            <For each={entries()}>
              {([key, value], index) => (
                <JsonViewer
                  name={isArray(props.data) ? undefined : String(key)}
                  data={value}
                  depth={depth + 1}
                  isLast={index() === entries().length - 1}
                  onNodeClick={props.onNodeClick}
                  arrayIndex={isArray(props.data) ? Number(key) : undefined}
                />
              )}
            </For>
          </div>

          <div class="json-line json-closing-bracket">
            <span class="json-spacer"></span>
            <span class="json-bracket">{isArray(props.data) ? ']' : '}'}</span>
            <Show when={!props.isLast}>
              <span class="json-comma">,</span>
            </Show>
          </div>
        </Show>
      </Show>

      <Show when={isPrimitive(props.data)}>
        <div class="json-line">
          <span class="json-spacer"></span>
          <Show when={props.name !== undefined}>
            <span class="json-key">"{props.name}"</span>
            <span class="json-colon">: </span>
          </Show>
          <span class={getValueClass(props.data)}>{formatValue(props.data)}</span>
          <Show when={!props.isLast}>
            <span class="json-comma">,</span>
          </Show>
        </div>
      </Show>
    </>
  );
}

export default JsonViewer;
