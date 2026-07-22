package main

import (
	"bytes"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"time"

	"github.com/yuin/goldmark"
	"github.com/yuin/goldmark/text"
)

func bench(name string, iterations int, f func()) {
	for i := 0; i < 20; i++ {
		f()
	}

	start := time.Now()
	for i := 0; i < iterations; i++ {
		f()
	}
	elapsed := time.Since(start)
	msPerOp := float64(elapsed.Nanoseconds()) / 1e6 / float64(iterations)
	fmt.Printf("%s,%.6f\n", name, msPerOp)
}

func main() {
	label := os.Getenv("POLYGLOT_DATA_LABEL")
	if label == "" {
		label = "default_data"
	}

	dataPath := os.Getenv("POLYGLOT_DATA_FILE")
	if dataPath == "" {
		dataPath = filepath.Join("..", "data", "data.md")
	}

	raw, err := os.ReadFile(dataPath)
	if err != nil {
		panic(err)
	}

	iterations := 500
	if s := os.Getenv("POLYGLOT_ITERS"); s != "" {
		if v, err := strconv.Atoi(s); err == nil && v > 0 {
			iterations = v
		}
	}

	md := goldmark.New()
	p := md.Parser()

	bench("go_goldmark_parse_only__"+label, iterations, func() {
		doc := p.Parse(text.NewReader(raw))
		_ = doc
	})

	bench("go_goldmark_parse_and_html__"+label, iterations, func() {
		var buf bytes.Buffer
		if err := md.Convert(raw, &buf); err != nil {
			panic(err)
		}
	})
}
