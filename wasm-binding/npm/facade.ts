import type {
  AstData,
  AstNode,
  Document as WasmDocument,
  FrontmatterOrNull,
  HeadingMatch,
  LinkMatch,
  ParserOptions,
  SemanticTarget,
  Tags,
} from "./markdown_binding";

const NO_NODE = 0xffffffff;

type WasmBinding = Pick<
  typeof import("./markdown_binding"),
  | "parse"
  | "parse_with_options"
  | "parse_selected"
  | "query_semantic_targets"
  | "query_semantic_targets_with_options"
  | "version"
>;

function utf8Width(codePoint: number): number {
  if (codePoint <= 0x7f) return 1;
  if (codePoint <= 0x7ff) return 2;
  if (codePoint <= 0xffff) return 3;
  return 4;
}

function locations(source: string, starts: Uint32Array, ends: Uint32Array) {
  const targets: [number, number, boolean][] = [];
  for (let index = 0; index < starts.length; index += 1) {
    targets.push([starts[index], index, true], [ends[index], index, false]);
  }
  targets.sort((left, right) => left[0] - right[0]);

  const result: { line: number; column: number }[] = new Array(targets.length);
  let byteIndex = 0;
  let sourceIndex = 0;
  let line = 1;
  let column = 1;
  for (const [target, nodeIndex, isStart] of targets) {
    while (byteIndex < target && sourceIndex < source.length) {
      const codePoint = source.codePointAt(sourceIndex)!;
      const width = utf8Width(codePoint);
      if (byteIndex + width > target) break;
      byteIndex += width;
      sourceIndex += codePoint > 0xffff ? 2 : 1;
      if (codePoint === 10) {
        line += 1;
        column = 1;
      } else {
        column += 1;
      }
    }
    result[nodeIndex * 2 + (isStart ? 0 : 1)] = { line, column };
  }
  return result;
}

function materializeTree(data: AstData, source: string): AstNode {
  if (data.abi_version !== 1) {
    throw new Error(`unsupported AST transport ABI: ${data.abi_version}`);
  }

  const payloads: Array<{ id: string | null; content?: unknown }> = JSON.parse(
    data.payloads_json,
  );
  if (payloads.length !== data.node_count) {
    throw new Error("invalid AST transport: payload count does not match node count");
  }

  const positions = locations(source, data.start, data.end);
  const nodes: Array<Record<string, unknown>> = new Array(data.node_count);
  for (let index = 0; index < data.node_count; index += 1) {
    const payload = payloads[index];
    const node: Record<string, unknown> = {
      kind: data.kind_names[data.kind[index]],
      start: positions[index * 2],
      end: positions[index * 2 + 1],
      children: [],
    };
    if (payload.id !== null) node.id = payload.id;
    if (Object.hasOwn(payload, "content")) node.content = payload.content;
    nodes[index] = node;
  }
  for (let index = 0; index < data.node_count; index += 1) {
    let child = data.first_child[index];
    while (child !== NO_NODE) {
      (nodes[index].children as unknown[]).push(nodes[child]);
      child = data.next_sibling[child];
    }
  }
  return nodes[data.root] as unknown as AstNode;
}

export class Document {
  #inner: WasmDocument;
  #source: string;
  #tree: AstNode | undefined;
  #disposed = false;

  constructor(inner: WasmDocument, source: string) {
    this.#inner = inner;
    this.#source = source;
  }

  #requireLive(): void {
    if (this.#disposed) throw new Error("document has been disposed");
  }

  get tree(): AstNode {
    this.#requireLive();
    if (this.#tree === undefined) {
      this.#tree = materializeTree(this.#inner.astData(), this.#source);
    }
    return this.#tree;
  }

  get tags(): Tags {
    this.#requireLive();
    return this.#inner.tags;
  }

  get frontmatter(): FrontmatterOrNull {
    this.#requireLive();
    return this.#inner.frontmatter;
  }

  get totalNodes(): number {
    this.#requireLive();
    return this.#inner.total_nodes;
  }

  toHtml(): string {
    this.#requireLive();
    return this.#inner.to_html();
  }

  queryHeadings(): HeadingMatch[] {
    this.#requireLive();
    return this.#inner.query_headings();
  }

  queryLinks(): LinkMatch[] {
    this.#requireLive();
    return this.#inner.query_links();
  }

  continueParse(): void {
    this.#requireLive();
    this.#inner.continue_parse();
    this.#tree = undefined;
  }

  dispose(): void {
    if (!this.#disposed) {
      this.#inner.free();
      this.#disposed = true;
      this.#tree = undefined;
    }
  }
}

export function createApi(binding: WasmBinding) {
  return {
    Document,
    parse(source: string): Document {
      return new Document(binding.parse(source), source);
    },
    parseWithOptions(source: string, options: ParserOptions): Document {
      return new Document(binding.parse_with_options(source, options), source);
    },
    parseSelected(source: string, nodeIds: Uint32Array): Document {
      return new Document(binding.parse_selected(source, nodeIds), source);
    },
    querySemanticTargets(source: string): SemanticTarget[] {
      return binding.query_semantic_targets(source);
    },
    querySemanticTargetsWithOptions(
      source: string,
      options: ParserOptions,
    ): SemanticTarget[] {
      return binding.query_semantic_targets_with_options(source, options);
    },
    version: binding.version,
  };
}
