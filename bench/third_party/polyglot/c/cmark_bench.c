#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <cmark.h>

static double now_ms(void) {
  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);
  return (double)ts.tv_sec * 1000.0 + (double)ts.tv_nsec / 1000000.0;
}

static char *read_file(const char *path, size_t *out_len) {
  FILE *fp = fopen(path, "rb");
  char *buf;
  size_t len;

  if (!fp) return NULL;
  fseek(fp, 0, SEEK_END);
  len = (size_t)ftell(fp);
  fseek(fp, 0, SEEK_SET);

  buf = (char *)malloc(len + 1);
  if (!buf) {
    fclose(fp);
    return NULL;
  }
  if (fread(buf, 1, len, fp) != len) {
    fclose(fp);
    free(buf);
    return NULL;
  }
  fclose(fp);
  buf[len] = '\0';
  *out_len = len;
  return buf;
}

int main(void) {
  size_t len = 0;
  const char *iters_env = getenv("POLYGLOT_ITERS");
  const char *label = getenv("POLYGLOT_DATA_LABEL");
  const char *data_path = getenv("POLYGLOT_DATA_FILE");
  int iters = 500;
  double start, end, ms_per_op;
  if (!label || !*label) {
    label = "default_data";
  }
  if (!data_path || !*data_path) {
    data_path = "../data/data.md";
  }
  char *text = read_file(data_path, &len);

  if (!text) {
    fprintf(stderr, "failed to read %s\n", data_path);
    return 1;
  }
  if (iters_env) {
    int parsed = atoi(iters_env);
    if (parsed > 0) iters = parsed;
  }

  for (int i = 0; i < 20; i++) {
    cmark_node *doc = cmark_parse_document(text, len, CMARK_OPT_DEFAULT);
    cmark_node_free(doc);
  }
  start = now_ms();
  for (int i = 0; i < iters; i++) {
    cmark_node *doc = cmark_parse_document(text, len, CMARK_OPT_DEFAULT);
    cmark_node_free(doc);
  }
  end = now_ms();
  ms_per_op = (end - start) / (double)iters;
  printf("c_cmark_parse_only__%s,%.6f\n", label, ms_per_op);

  for (int i = 0; i < 20; i++) {
    cmark_node *doc = cmark_parse_document(text, len, CMARK_OPT_DEFAULT);
    char *html = cmark_render_html(doc, CMARK_OPT_DEFAULT);
    free(html);
    cmark_node_free(doc);
  }
  start = now_ms();
  for (int i = 0; i < iters; i++) {
    cmark_node *doc = cmark_parse_document(text, len, CMARK_OPT_DEFAULT);
    char *html = cmark_render_html(doc, CMARK_OPT_DEFAULT);
    free(html);
    cmark_node_free(doc);
  }
  end = now_ms();
  ms_per_op = (end - start) / (double)iters;
  printf("c_cmark_parse_and_html__%s,%.6f\n", label, ms_per_op);

  free(text);
  return 0;
}
