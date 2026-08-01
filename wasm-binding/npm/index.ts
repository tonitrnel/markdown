import * as binding from "./markdown_binding";
import { createApi } from "./facade";

const api = createApi(binding);

export const Document = api.Document;
export const parse = api.parse;
export const parseWithOptions = api.parseWithOptions;
export const parseSelected = api.parseSelected;
export const querySemanticTargets = api.querySemanticTargets;
export const querySemanticTargetsWithOptions = api.querySemanticTargetsWithOptions;
export const version = api.version;

export type {
  AstNode,
  Frontmatter,
  FrontmatterOrNull,
  HeadingMatch,
  LinkMatch,
  Location,
  Node,
  ParserOptions,
  Reference,
  SemanticTarget,
  Tags,
  YamlValue,
} from "./markdown_binding";
