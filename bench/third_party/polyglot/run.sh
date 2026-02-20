#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
BENCH_DIR="$ROOT_DIR/bench/third_party/polyglot"
GO_BIN="${GO_BIN:-/usr/local/go/bin/go}"
GO_CACHE_DIR="$BENCH_DIR/.gocache"
GO_MOD_CACHE_DIR="$GO_CACHE_DIR/mod"
CMARK_SRC_DIR="$BENCH_DIR/.third_party/cmark-master"
CMARK_BUILD_DIR="$CMARK_SRC_DIR/build"
CMARK_BIN="$BENCH_DIR/c/cmark_bench"
CMARK_URL="${CMARK_URL:-https://github.com/commonmark/cmark/archive/refs/heads/master.tar.gz}"
MARKDOWN_IT_DIR="$ROOT_DIR/bench/data/markdown-it"
MARKDOWN_IT_CORPUS="$ROOT_DIR/bench/data/markdown-it-corpus.md"

mkdir -p "$GO_CACHE_DIR" "$GO_MOD_CACHE_DIR"

build_markdown_it_corpus() {
  if [[ ! -d "$MARKDOWN_IT_DIR" ]]; then
    return 1
  fi
  : > "$MARKDOWN_IT_CORPUS"
  while IFS= read -r file; do
    printf "\n\n<!-- SOURCE: %s -->\n\n" "$file" >> "$MARKDOWN_IT_CORPUS"
    cat "$file" >> "$MARKDOWN_IT_CORPUS"
  done < <(find "$MARKDOWN_IT_DIR" -maxdepth 1 -type f \( -name "*.md" -o -name "*.txt" \) | sort)
  return 0
}

DATASET_PATHS=("$BENCH_DIR/data/data.md")
DATASET_LABELS=("default_data")
if build_markdown_it_corpus && [[ -s "$MARKDOWN_IT_CORPUS" ]]; then
  DATASET_PATHS+=("$MARKDOWN_IT_CORPUS")
  DATASET_LABELS+=("markdown_it_corpus")
fi

echo "name,ms_per_op"

for i in "${!DATASET_PATHS[@]}"; do
  dataset_path="${DATASET_PATHS[$i]}"
  dataset_label="${DATASET_LABELS[$i]}"
  (
    cd "$ROOT_DIR"
    POLYGLOT_DATA_FILE="$dataset_path" POLYGLOT_DATA_LABEL="$dataset_label" cargo run -p polyglot-bench --release --quiet
  )
done

if [[ -x "$GO_BIN" ]]; then
  (
    cd "$BENCH_DIR/go"
    if ! GOCACHE="$GO_CACHE_DIR/build" GOMODCACHE="$GO_MOD_CACHE_DIR" "$GO_BIN" mod tidy >/dev/null 2>&1; then
      echo "[skip] goldmark: dependency resolution failed (network/proxy/cache)." >&2
      for dataset_label in "${DATASET_LABELS[@]}"; do
        echo "go_goldmark_parse_only__${dataset_label},NA"
        echo "go_goldmark_parse_and_html__${dataset_label},NA"
      done
      exit 0
    fi
    for i in "${!DATASET_PATHS[@]}"; do
      dataset_path="${DATASET_PATHS[$i]}"
      dataset_label="${DATASET_LABELS[$i]}"
      if ! POLYGLOT_DATA_FILE="$dataset_path" POLYGLOT_DATA_LABEL="$dataset_label" GOCACHE="$GO_CACHE_DIR/build" GOMODCACHE="$GO_MOD_CACHE_DIR" "$GO_BIN" run .; then
        echo "[skip] goldmark: benchmark run failed (${dataset_label})." >&2
        echo "go_goldmark_parse_only__${dataset_label},NA"
        echo "go_goldmark_parse_and_html__${dataset_label},NA"
      fi
    done
  )
else
  echo "[skip] goldmark: go binary not found at $GO_BIN." >&2
  for dataset_label in "${DATASET_LABELS[@]}"; do
    echo "go_goldmark_parse_only__${dataset_label},NA"
    echo "go_goldmark_parse_and_html__${dataset_label},NA"
  done
fi

build_local_cmark() {
  if [[ ! -f "$CMARK_SRC_DIR/CMakeLists.txt" ]]; then
    mkdir -p "$BENCH_DIR/.third_party"
    if command -v curl >/dev/null 2>&1; then
      curl -fsSL "$CMARK_URL" 2>/dev/null | tar -xz -C "$BENCH_DIR/.third_party" 2>/dev/null
    elif command -v wget >/dev/null 2>&1; then
      wget -qO- "$CMARK_URL" 2>/dev/null | tar -xz -C "$BENCH_DIR/.third_party" 2>/dev/null
    else
      return 1
    fi
  fi

  if ! command -v cmake >/dev/null 2>&1; then
    return 1
  fi

  if [[ -f "$CMARK_BUILD_DIR/CMakeCache.txt" ]] &&
    ! grep -q "CMAKE_HOME_DIRECTORY:INTERNAL=$CMARK_SRC_DIR" "$CMARK_BUILD_DIR/CMakeCache.txt"; then
    rm -rf "$CMARK_BUILD_DIR"
  fi

  cmake -S "$CMARK_SRC_DIR" -B "$CMARK_BUILD_DIR" -DCMAKE_BUILD_TYPE=Release >/dev/null
  cmake --build "$CMARK_BUILD_DIR" --target cmark --config Release >/dev/null
  return 0
}

run_cmark_local() {
  local static_lib="$CMARK_BUILD_DIR/src/libcmark.a"
  local dylib="$CMARK_BUILD_DIR/src/libcmark.dylib"
  local so_lib="$CMARK_BUILD_DIR/src/libcmark.so"

  (
    cd "$BENCH_DIR/c"
    if [[ -f "$static_lib" ]]; then
      cc cmark_bench.c -O3 -I"$CMARK_SRC_DIR/src" -I"$CMARK_BUILD_DIR/src" "$static_lib" -o "$CMARK_BIN"
      for i in "${!DATASET_PATHS[@]}"; do
        dataset_path="${DATASET_PATHS[$i]}"
        dataset_label="${DATASET_LABELS[$i]}"
        POLYGLOT_DATA_FILE="$dataset_path" POLYGLOT_DATA_LABEL="$dataset_label" "$CMARK_BIN"
      done
    elif [[ -f "$dylib" ]]; then
      cc cmark_bench.c -O3 -I"$CMARK_SRC_DIR/src" -I"$CMARK_BUILD_DIR/src" -L"$CMARK_BUILD_DIR/src" -lcmark -o "$CMARK_BIN"
      for i in "${!DATASET_PATHS[@]}"; do
        dataset_path="${DATASET_PATHS[$i]}"
        dataset_label="${DATASET_LABELS[$i]}"
        POLYGLOT_DATA_FILE="$dataset_path" POLYGLOT_DATA_LABEL="$dataset_label" DYLD_LIBRARY_PATH="$CMARK_BUILD_DIR/src:${DYLD_LIBRARY_PATH:-}" "$CMARK_BIN"
      done
    elif [[ -f "$so_lib" ]]; then
      cc cmark_bench.c -O3 -I"$CMARK_SRC_DIR/src" -I"$CMARK_BUILD_DIR/src" -L"$CMARK_BUILD_DIR/src" -lcmark -o "$CMARK_BIN"
      for i in "${!DATASET_PATHS[@]}"; do
        dataset_path="${DATASET_PATHS[$i]}"
        dataset_label="${DATASET_LABELS[$i]}"
        POLYGLOT_DATA_FILE="$dataset_path" POLYGLOT_DATA_LABEL="$dataset_label" LD_LIBRARY_PATH="$CMARK_BUILD_DIR/src:${LD_LIBRARY_PATH:-}" "$CMARK_BIN"
      done
    else
      return 1
    fi
  )
}

if build_local_cmark && run_cmark_local; then
  :
else
  echo "[skip] cmark: local build failed (network/cmake/toolchain)." >&2
  for dataset_label in "${DATASET_LABELS[@]}"; do
    echo "c_cmark_parse_only__${dataset_label},NA"
    echo "c_cmark_parse_and_html__${dataset_label},NA"
  done
fi
