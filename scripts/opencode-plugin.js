// OpenCode plugin: auto-format ASCII diagrams with ascii-fmt
//
// Install:
//   - Ensure 'ascii-fmt' is on PATH
//   - Drop this file into one of:
//       - ~/.config/opencode/plugins/ascii-fmt.js (global)
//       - .opencode/plugins/ascii-fmt.js (project)
//
// Behavior:
//   - For Markdown: formats fenced blocks with languages: ascii, diagram, ascii-diagram
//   - For other text files: formats whole file only if it looks like an ASCII diagram
//
// References:
//   - https://opencode.ai/docs/plugins/
//   - https://opencode.ai/docs/formatters/

const path = require('path');

const FENCED_LANGS = new Set(['ascii', 'diagram', 'ascii-diagram']);

// Never auto-format source code files. These often contain box-drawing
// characters inside string literals (tests/fixtures) and formatting the whole
// file would corrupt the code.
const SKIP_EXTS = new Set([
  '.rs',
  '.js',
  '.mjs',
  '.cjs',
  '.ts',
  '.tsx',
  '.jsx',
  '.json',
  '.toml',
  '.yaml',
  '.yml',
  '.lock',
  '.py',
  '.go',
  '.java',
  '.kt',
  '.swift',
  '.c',
  '.h',
  '.cc',
  '.cpp',
  '.hpp',
  '.cs',
  '.rb',
  '.php',
]);

function shouldSkipWholeFileFormatting(filePath) {
  const ext = path.extname(filePath).toLowerCase();
  if (ext === '.md') return false;
  return SKIP_EXTS.has(ext);
}

function looksLikeDiagram(text) {
  // Conservative heuristics to avoid touching normal prose/code.
  // Trigger on common diagram glyphs or repeated box/table patterns.
  if (/[\u2500-\u257F]/.test(text)) return true; // box-drawing block
  if (/(^|\n)\s*\+[-=]{2,}\+/.test(text)) return true; // +---+
  if (/(^|\n)\s*\|[^\n]*\|\s*(\n|$)/.test(text) && /\+[-=]{2,}\+/.test(text)) return true;
  if (/\b->\b|\b<-\b|\b=>\b|\b<=\b/.test(text) && /\n/.test(text)) return true;
  return false;
}

function parseFences(lines) {
  const blocks = [];
  let i = 0;

  while (i < lines.length) {
    const line = lines[i];
    const m = line.match(/^```([^\s`]*)\s*$/);
    if (!m) {
      i += 1;
      continue;
    }

    const lang = (m[1] || '').trim();
    const startFenceLine = i;
    const contentStart = i + 1;
    i += 1;

    while (i < lines.length && !lines[i].startsWith('```')) {
      i += 1;
    }

    if (i >= lines.length) break;

    const endFenceLine = i;
    const contentEnd = endFenceLine;
    blocks.push({ lang, startFenceLine, contentStart, contentEnd, endFenceLine });
    i += 1;
  }

  return blocks;
}

async function runAsciiFmtOnString(input) {
  const proc = Bun.spawn(['ascii-fmt'], {
    stdin: 'pipe',
    stdout: 'pipe',
    stderr: 'pipe',
  });

  proc.stdin.write(input);
  proc.stdin.end();

  const stdout = await new Response(proc.stdout).text();
  const stderr = await new Response(proc.stderr).text();
  const code = await proc.exited;

  if (code !== 0) {
    const msg = stderr.trim() ? `ascii-fmt failed: ${stderr.trim()}` : 'ascii-fmt failed';
    throw new Error(msg);
  }

  return stdout;
}

async function runAsciiFmtInPlace(filePath) {
  const proc = Bun.spawn(['ascii-fmt', filePath, '-o', filePath], {
    stdout: 'pipe',
    stderr: 'pipe',
  });

  const stderr = await new Response(proc.stderr).text();
  const code = await proc.exited;

  if (code !== 0) {
    const msg = stderr.trim() ? `ascii-fmt failed: ${stderr.trim()}` : 'ascii-fmt failed';
    throw new Error(msg);
  }
}

export const AsciiFmtPlugin = async ({ client, worktree }) => {
  // Prevent infinite loops when formatting updates the same file.
  const lastHashByFile = new Map();

  return {
    event: async ({ event }) => {
      if (!event || event.type !== 'file.edited') return;

      const rel = event.properties && event.properties.file;
      if (typeof rel !== 'string' || !rel.length) return;

      // Worktree is the git root path. Event file is repo-relative.
      const filePath = path.isAbsolute(rel) ? rel : path.join(worktree, rel);

      // Skip opencode internals.
      if (filePath.includes(`${path.sep}.opencode${path.sep}`)) return;

      // Never auto-format whole source files.
      if (shouldSkipWholeFileFormatting(filePath)) return;

      let text;
      try {
        text = await Bun.file(filePath).text();
      } catch {
        return;
      }

      const hash = Bun.hash(text);
      if (lastHashByFile.get(filePath) === hash) return;

      try {
        if (filePath.endsWith('.md')) {
          const lines = text.split(/\n/);
          const blocks = parseFences(lines);
          const targets = blocks.filter((b) => FENCED_LANGS.has(b.lang));
          if (targets.length === 0) return;

          for (const b of targets) {
            const original = lines.slice(b.contentStart, b.contentEnd).join('\n');
            const formatted = (await runAsciiFmtOnString(original)).replace(/\n$/, '');
            const newLines = formatted.length ? formatted.split(/\n/) : [''];
            lines.splice(b.contentStart, b.contentEnd - b.contentStart, ...newLines);

            // Adjust subsequent blocks due to line count changes.
            const delta = newLines.length - (b.contentEnd - b.contentStart);
            if (delta !== 0) {
              for (const other of targets) {
                if (other.startFenceLine > b.startFenceLine) {
                  other.startFenceLine += delta;
                  other.contentStart += delta;
                  other.contentEnd += delta;
                  other.endFenceLine += delta;
                }
              }
            }
          }

          const nextText = lines.join('\n');
          if (nextText !== text) {
            await Bun.write(filePath, nextText);
            lastHashByFile.set(filePath, Bun.hash(nextText));
          }
        } else {
          if (!looksLikeDiagram(text)) return;
          await runAsciiFmtInPlace(filePath);
          const next = await Bun.file(filePath).text();
          lastHashByFile.set(filePath, Bun.hash(next));
        }
      } catch (err) {
        await client.app.log({
          service: 'ascii-fmt',
          level: 'warn',
          message: `ascii-fmt integration failed for ${rel}`,
          extra: { error: String(err && err.message ? err.message : err) },
        });
      }
    },
  };
};
