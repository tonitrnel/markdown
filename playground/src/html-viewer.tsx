import { createEffect, createSignal } from 'solid-js';
import { codeToHtml } from 'shiki';

interface HtmlViewerProps {
  html: string;
}

function HtmlViewer(props: HtmlViewerProps) {
  const [highlightedHtml, setHighlightedHtml] = createSignal('');

  // Format HTML with indentation
  const formatHtml = (html: string): string => {
    let formatted = '';
    let indent = 0;
    const tab = '  ';

    // Split by tags while preserving them
    const parts = html.split(/(<[^>]+>)/g).filter((part) => part.trim());

    parts.forEach((part) => {
      if (part.startsWith('</')) {
        // Closing tag - decrease indent before adding
        indent = Math.max(0, indent - 1);
        formatted += tab.repeat(indent) + part + '\n';
      } else if (part.startsWith('<')) {
        // Opening tag or self-closing tag
        const isSelfClosing =
          part.endsWith('/>') ||
          /^<(area|base|br|col|embed|hr|img|input|link|meta|param|source|track|wbr)/.test(
            part,
          );

        formatted += tab.repeat(indent) + part + '\n';

        if (!isSelfClosing) {
          indent++;
        }
      } else {
        // Text content
        const trimmed = part.trim();
        if (trimmed) {
          formatted += tab.repeat(indent) + trimmed + '\n';
        }
      }
    });

    return formatted.trimEnd();
  };

  createEffect(async () => {
    const formatted = formatHtml(props.html);

    try {
      const highlighted = await codeToHtml(formatted, {
        lang: 'html',
        theme: 'github-dark',
      });
      setHighlightedHtml(highlighted);
    } catch (error) {
      console.error('Shiki highlighting error:', error);
      setHighlightedHtml(`<pre><code>${formatted}</code></pre>`);
    }
  });

  return <div class="html-viewer-container" innerHTML={highlightedHtml()} />;
}

export default HtmlViewer;
